use reqwest::header::{
    ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderName,
    HeaderValue, ORIGIN, USER_AGENT,
};
use secrecy::ExposeSecret;

use crate::{Auth, ClientConfig, Error};

const SEC_CH_UA: HeaderName = HeaderName::from_static("sec-ch-ua");
const SEC_CH_UA_MOBILE: HeaderName = HeaderName::from_static("sec-ch-ua-mobile");
const SEC_CH_UA_PLATFORM: HeaderName = HeaderName::from_static("sec-ch-ua-platform");
const SEC_FETCH_DEST: HeaderName = HeaderName::from_static("sec-fetch-dest");
const SEC_FETCH_MODE: HeaderName = HeaderName::from_static("sec-fetch-mode");
const SEC_FETCH_SITE: HeaderName = HeaderName::from_static("sec-fetch-site");

pub(crate) fn headers(config: &ClientConfig, auth: &Auth) -> Result<HeaderMap, Error> {
    let mut headers = HeaderMap::new();

    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br"),
    );
    headers.insert(
        USER_AGENT,
        HeaderValue::from_str(config.fingerprint.user_agent().as_str())
            .map_err(|_| Error::InvalidHeader)?,
    );
    headers.insert(
        ORIGIN,
        HeaderValue::from_str(
            format!(
                "chrome-extension://{}",
                config.fingerprint.extension_id.as_str()
            )
            .as_str(),
        )
        .map_err(|_| Error::InvalidHeader)?,
    );
    headers.insert(
        SEC_CH_UA,
        HeaderValue::from_str(config.fingerprint.sec_ch_ua().as_str())
            .map_err(|_| Error::InvalidHeader)?,
    );
    headers.insert(SEC_CH_UA_MOBILE, HeaderValue::from_static("?0"));
    headers.insert(SEC_CH_UA_PLATFORM, HeaderValue::from_static("\"macOS\""));
    headers.insert(SEC_FETCH_SITE, HeaderValue::from_static("cross-site"));
    headers.insert(SEC_FETCH_MODE, HeaderValue::from_static("cors"));
    headers.insert(SEC_FETCH_DEST, HeaderValue::from_static("empty"));

    match auth {
        Auth::Anonymous => {
            headers.insert(AUTHORIZATION, HeaderValue::from_static("None"));
        }
        Auth::Bearer(token) => {
            let value = format!("Bearer {}", token.expose_secret());
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(value.as_str()).map_err(|_| Error::InvalidHeader)?,
            );
        }
    }

    Ok(headers)
}

#[cfg(test)]
mod tests {
    use secrecy::SecretString;

    use super::*;

    #[test]
    fn anonymous_auth_sets_authorization_none() {
        let headers = headers(&ClientConfig::default(), &Auth::Anonymous).unwrap();
        assert_eq!(headers[AUTHORIZATION], "None");
        assert!(headers.get(ORIGIN).is_some());
    }

    #[test]
    fn bearer_auth_sets_authorization_bearer() {
        let token = SecretString::from("token.value".to_string());
        let headers = headers(&ClientConfig::default(), &Auth::Bearer(token)).unwrap();
        assert_eq!(headers[AUTHORIZATION], "Bearer token.value");
    }
}
