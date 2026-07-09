use super::{Auth, Error, TargetLang, TranslateRequest, TranslateResponse, Translation};
use crate::impersonated_chrome_version;

use reqwest::{
    StatusCode,
    header::{
        ACCEPT, ACCEPT_ENCODING, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, HeaderMap,
        HeaderName, HeaderValue, ORIGIN, USER_AGENT,
    },
};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};

pub(crate) const MAX_FREE_TEXT_LENGTH: usize = 1500;

const CHROME_EXTENSION_VERSION: &str = "1.86.0";
const CHROME_EXTENSION_ORIGIN: &str = "chrome-extension://cofdbpoegempjloogbagkncekinflcnj";
const CHROME_OS: &str = "brex_macOS";
const CHROME_OS_VERSION: &str = concat!("brex_chrome_", impersonated_chrome_version!(), ".0.0.0");
const CHROME_APP_BUILD: &str = "chrome_web_store";
const CHROME_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
const ACCEPT_LANGUAGE_VALUE: &str = "zh-CN,zh;q=0.9";
const SEC_CH_UA_VALUE: &str =
    "\"Not_A Brand\";v=\"8\", \"Chromium\";v=\"120\", \"Google Chrome\";v=\"120\"";

const ACCEPT_LANGUAGE_HEADER: HeaderName = HeaderName::from_static("accept-language");
const SEC_CH_UA: HeaderName = HeaderName::from_static("sec-ch-ua");
const SEC_CH_UA_MOBILE: HeaderName = HeaderName::from_static("sec-ch-ua-mobile");
const SEC_CH_UA_PLATFORM: HeaderName = HeaderName::from_static("sec-ch-ua-platform");
const SEC_FETCH_DEST: HeaderName = HeaderName::from_static("sec-fetch-dest");
const SEC_FETCH_MODE: HeaderName = HeaderName::from_static("sec-fetch-mode");
const SEC_FETCH_SITE: HeaderName = HeaderName::from_static("sec-fetch-site");

#[derive(Serialize)]
struct OneshotRequest<'a> {
    text: &'a [String],
    target_lang: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_lang: Option<&'a str>,
    usage_type: &'static str,
    app_information: AppInformation<'a>,
}

impl<'a> OneshotRequest<'a> {
    fn new(request: &'a TranslateRequest, instance_id: &'a str) -> Self {
        Self {
            text: &request.text,
            target_lang: request.target.wire(),
            source_lang: request.source.wire(),
            usage_type: "Translate",
            app_information: AppInformation::new(instance_id),
        }
    }
}

#[derive(Serialize)]
struct AppInformation<'a> {
    os: &'static str,
    os_version: &'static str,
    app_version: &'static str,
    app_build: &'static str,
    instance_id: &'a str,
}

impl<'a> AppInformation<'a> {
    fn new(instance_id: &'a str) -> Self {
        Self {
            os: CHROME_OS,
            os_version: CHROME_OS_VERSION,
            app_version: CHROME_EXTENSION_VERSION,
            app_build: CHROME_APP_BUILD,
            instance_id,
        }
    }
}

#[derive(Deserialize)]
struct OneshotResponse {
    translations: Vec<OneshotTranslation>,
}

#[derive(Deserialize)]
struct OneshotTranslation {
    detected_source_language: String,
    text: String,
}

pub(crate) fn post_headers(auth: &Auth, body_len: usize) -> Result<HeaderMap, Error> {
    let mut headers = HeaderMap::new();

    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

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

    headers.insert(SEC_CH_UA, HeaderValue::from_static(SEC_CH_UA_VALUE));
    headers.insert(ORIGIN, HeaderValue::from_static(CHROME_EXTENSION_ORIGIN));
    headers.insert(SEC_CH_UA_MOBILE, HeaderValue::from_static("?0"));
    headers.insert(SEC_CH_UA_PLATFORM, HeaderValue::from_static("\"macOS\""));
    headers.insert(USER_AGENT, HeaderValue::from_static(CHROME_USER_AGENT));
    headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
    headers.insert(SEC_FETCH_SITE, HeaderValue::from_static("cross-site"));
    headers.insert(SEC_FETCH_MODE, HeaderValue::from_static("cors"));
    headers.insert(
        CONTENT_LENGTH,
        HeaderValue::from_str(&body_len.to_string()).map_err(|_| Error::InvalidHeader)?,
    );
    headers.insert(SEC_FETCH_DEST, HeaderValue::from_static("empty"));
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br"),
    );
    headers.insert(
        ACCEPT_LANGUAGE_HEADER,
        HeaderValue::from_static(ACCEPT_LANGUAGE_VALUE),
    );

    Ok(headers)
}

pub(crate) fn build_body(
    request: &TranslateRequest,
    auth: &Auth,
    instance_id: &str,
) -> Result<Vec<u8>, Error> {
    if request.text.is_empty() || request.text.iter().any(String::is_empty) {
        return Err(Error::EmptyText);
    }

    if matches!(auth, Auth::Anonymous) {
        let actual = request.text_char_count();
        if actual > MAX_FREE_TEXT_LENGTH {
            return Err(Error::TextTooLong {
                actual,
                limit: MAX_FREE_TEXT_LENGTH,
            });
        }
    }

    let body = OneshotRequest::new(request, instance_id);

    serde_json::to_vec(&body).map_err(Error::from)
}

pub(crate) fn parse_response(
    status: StatusCode,
    body: &str,
    target: &TargetLang,
) -> Result<TranslateResponse, Error> {
    if status == StatusCode::TOO_MANY_REQUESTS {
        return Err(Error::TooManyRequests);
    }

    if !status.is_success() {
        return Err(Error::UpstreamStatus {
            status,
            body: body.to_string(),
        });
    }

    let response: OneshotResponse = serde_json::from_str(body)?;
    if response.translations.is_empty() {
        return Err(Error::MissingTranslation);
    }

    let translations = response
        .translations
        .into_iter()
        .map(|translation| Translation {
            detected_source_language: translation.detected_source_language,
            text: translation.text,
        })
        .collect::<Vec<_>>();

    if translations
        .iter()
        .all(|translation| translation.text.is_empty())
    {
        return Err(Error::MissingTranslation);
    }

    Ok(TranslateResponse {
        source_lang: translations
            .first()
            .map(|translation| translation.detected_source_language.clone())
            .filter(|lang| !lang.is_empty()),
        target_lang: target.code().to_string(),
        translations,
    })
}

#[cfg(test)]
mod tests {
    use secrecy::SecretString;
    use serde_json::Value;

    use super::*;
    use crate::{SourceLang, TargetLang};

    #[test]
    fn anonymous_auth_sets_authorization_none() {
        let headers = post_headers(&Auth::Anonymous, 300).unwrap();
        assert_eq!(headers[AUTHORIZATION], "None");
        assert_eq!(headers[ACCEPT_ENCODING], "gzip, deflate, br");
        assert!(headers.get(ORIGIN).is_some());
    }

    #[test]
    fn bearer_auth_sets_authorization_bearer() {
        let token = SecretString::from("token.value".to_string());
        let headers = post_headers(&Auth::Bearer(token), 300).unwrap();
        assert_eq!(headers[AUTHORIZATION], "Bearer token.value");
    }

    #[test]
    fn serializes_auto_source_without_source_lang() {
        let request = TranslateRequest::builder()
            .text("Hello")
            .unwrap()
            .source(SourceLang::Auto)
            .target(TargetLang::parse("ZH-HANS").unwrap())
            .build()
            .unwrap();

        let body = build_body(&request, &Auth::Anonymous, "instance").unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["text"], serde_json::json!(["Hello"]));
        assert_eq!(body["target_lang"], "zh-Hans");
        assert_eq!(body["usage_type"], "Translate");
        assert!(body.get("source_lang").is_none());
        assert_eq!(body["app_information"]["instance_id"], "instance");
    }

    #[test]
    fn serializes_known_source_lang() {
        let request = TranslateRequest::builder()
            .text("Hello")
            .unwrap()
            .source(SourceLang::parse("EN").unwrap())
            .target(TargetLang::parse("DE").unwrap())
            .build()
            .unwrap();

        let body = build_body(&request, &Auth::Anonymous, "instance").unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["source_lang"], "en");
    }

    #[test]
    fn rejects_anonymous_text_over_limit() {
        let request = TranslateRequest::builder()
            .text("a".repeat(MAX_FREE_TEXT_LENGTH + 1))
            .unwrap()
            .target(TargetLang::parse("DE").unwrap())
            .build()
            .unwrap();

        assert!(matches!(
            build_body(&request, &Auth::Anonymous, "instance"),
            Err(Error::TextTooLong { .. })
        ));
    }

    #[test]
    fn maps_429_to_too_many_requests() {
        assert!(matches!(
            parse_response(
                StatusCode::TOO_MANY_REQUESTS,
                "",
                &TargetLang::parse("DE").unwrap()
            ),
            Err(Error::TooManyRequests)
        ));
    }

    #[test]
    fn maps_non_success_to_upstream_status() {
        assert!(matches!(
            parse_response(
                StatusCode::SERVICE_UNAVAILABLE,
                "busy",
                &TargetLang::parse("DE").unwrap()
            ),
            Err(Error::UpstreamStatus { .. })
        ));
    }

    #[test]
    fn rejects_empty_translations() {
        assert!(matches!(
            parse_response(
                StatusCode::OK,
                r#"{"translations":[]}"#,
                &TargetLang::parse("DE").unwrap()
            ),
            Err(Error::MissingTranslation)
        ));
    }

    #[test]
    fn parses_translation_response() {
        let response = parse_response(
            StatusCode::OK,
            r#"{"translations":[{"detected_source_language":"EN","text":"Hallo"}]}"#,
            &TargetLang::parse("DE").unwrap(),
        )
        .unwrap();

        assert_eq!(response.source_lang.as_deref(), Some("EN"));
        assert_eq!(response.target_lang, "DE");
        assert_eq!(response.translations[0].text, "Hallo");
    }
}
