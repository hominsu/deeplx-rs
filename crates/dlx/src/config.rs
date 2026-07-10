use std::time::Duration;

use reqwest::Url;
use secrecy::SecretString;

use super::Error;

#[derive(Clone, Debug)]
pub enum Auth {
    Anonymous,
    Bearer(SecretString),
}

#[derive(Clone, Debug)]
pub enum WarmupMode {
    Disabled,
    Blocking,
    Background,
}

#[derive(Clone, Debug)]
pub enum Endpoint {
    OfficialOneshot,
    Custom { free_url: Url, pro_url: Option<Url> },
}

impl Endpoint {
    pub(crate) fn url_for_auth(&self, auth: &Auth) -> Result<Url, Error> {
        match (self, auth) {
            (Self::OfficialOneshot, Auth::Anonymous) => {
                Url::parse("https://oneshot-free.www.deepl.com/v1/translate")
                    .map_err(|_| Error::InvalidUrl)
            }
            (Self::OfficialOneshot, Auth::Bearer(_)) => {
                Url::parse("https://oneshot-pro.www.deepl.com/v1/translate")
                    .map_err(|_| Error::InvalidUrl)
            }
            (Self::Custom { free_url, .. }, Auth::Anonymous) => Ok(free_url.clone()),
            (Self::Custom { pro_url, .. }, Auth::Bearer(_)) => {
                pro_url.clone().ok_or(Error::MissingProEndpoint)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub endpoint: Endpoint,
    pub auth: Auth,
    pub timeout: Duration,
    pub proxy: Option<Url>,
    pub warmup: WarmupMode,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            endpoint: Endpoint::OfficialOneshot,
            auth: Auth::Anonymous,
            timeout: Duration::from_secs(20),
            proxy: None,
            warmup: WarmupMode::Disabled,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn official_endpoint_selects_free_and_pro_urls() {
        assert_eq!(
            Endpoint::OfficialOneshot
                .url_for_auth(&Auth::Anonymous)
                .unwrap()
                .as_str(),
            "https://oneshot-free.www.deepl.com/v1/translate"
        );
        assert_eq!(
            Endpoint::OfficialOneshot
                .url_for_auth(&Auth::Bearer(SecretString::from("token".to_string())))
                .unwrap()
                .as_str(),
            "https://oneshot-pro.www.deepl.com/v1/translate"
        );
    }

    #[test]
    fn custom_endpoint_requires_pro_url_for_bearer_auth() {
        let endpoint = Endpoint::Custom {
            free_url: Url::parse("http://example.test/free").unwrap(),
            pro_url: None,
        };

        assert!(matches!(
            endpoint.url_for_auth(&Auth::Bearer(SecretString::from("token".to_string()))),
            Err(Error::MissingProEndpoint)
        ));
    }
}
