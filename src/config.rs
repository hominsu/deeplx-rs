use std::time::Duration;

use reqwest::Url;
use secrecy::SecretString;

use crate::Error;

#[derive(Clone, Debug)]
pub enum Auth {
    Anonymous,
    Bearer(SecretString),
}

#[derive(Clone, Debug)]
pub enum Endpoint {
    OfficialOneshot,
    Custom { free_url: Url, pro_url: Option<Url> },
}

#[derive(Clone, Debug)]
pub enum WarmupMode {
    Disabled,
    Blocking,
    Background,
}

#[derive(Clone, Debug)]
pub struct Fingerprint {
    pub chrome_major: u16,
    pub extension_version: String,
    pub extension_id: String,
    pub os: String,
    pub app_build: String,
}

#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub endpoint: Endpoint,
    pub auth: Auth,
    pub timeout: Duration,
    pub proxy: Option<Url>,
    pub warmup: WarmupMode,
    pub fingerprint: Fingerprint,
}

impl Default for Fingerprint {
    fn default() -> Self {
        Self {
            chrome_major: 120,
            extension_version: "1.86.0".to_string(),
            extension_id: "cofdbpoegempjloogbagkncekinflcnj".to_string(),
            os: "brex_macOS".to_string(),
            app_build: "chrome_web_store".to_string(),
        }
    }
}

impl Fingerprint {
    pub fn user_agent(&self) -> String {
        format!(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{major}.0.0.0 Safari/537.36",
            major = self.chrome_major
        )
    }

    pub(crate) fn os_version(&self) -> String {
        format!("brex_chrome_{}.0.0.0", self.chrome_major)
    }

    pub(crate) fn sec_ch_ua(&self) -> String {
        format!(
            "\"Not_A Brand\";v=\"8\", \"Chromium\";v=\"{major}\", \"Google Chrome\";v=\"{major}\"",
            major = self.chrome_major
        )
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            endpoint: Endpoint::OfficialOneshot,
            auth: Auth::Anonymous,
            timeout: Duration::from_secs(20),
            proxy: None,
            warmup: WarmupMode::Disabled,
            fingerprint: Fingerprint::default(),
        }
    }
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
