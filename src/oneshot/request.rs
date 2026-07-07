use serde::Serialize;

use crate::{Auth, ClientConfig, Error, TranslateRequest};

pub(crate) const MAX_FREE_TEXT_LENGTH: usize = 1500;

#[derive(Serialize)]
struct OneshotRequest<'a> {
    text: &'a [String],
    target_lang: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_lang: Option<&'a str>,
    usage_type: &'static str,
    app_information: AppInformation<'a>,
}

#[derive(Serialize)]
struct AppInformation<'a> {
    os: &'a str,
    os_version: String,
    app_version: &'a str,
    app_build: &'a str,
    instance_id: &'a str,
}

pub(crate) fn build_body(
    request: &TranslateRequest,
    config: &ClientConfig,
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

    let body = OneshotRequest {
        text: &request.text,
        target_lang: request.target.wire(),
        source_lang: request.source.wire(),
        usage_type: "Translate",
        app_information: AppInformation {
            os: &config.fingerprint.os,
            os_version: config.fingerprint.os_version(),
            app_version: &config.fingerprint.extension_version,
            app_build: &config.fingerprint.app_build,
            instance_id,
        },
    };

    serde_json::to_vec(&body).map_err(Error::from)
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::*;
    use crate::{SourceLang, TargetLang, TranslateRequest};

    #[test]
    fn serializes_auto_source_without_source_lang() {
        let request = TranslateRequest::builder()
            .text("Hello")
            .unwrap()
            .source(SourceLang::Auto)
            .target(TargetLang::parse("ZH-HANS").unwrap())
            .build()
            .unwrap();

        let body = build_body(
            &request,
            &ClientConfig::default(),
            &Auth::Anonymous,
            "instance",
        )
        .unwrap();
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

        let body = build_body(
            &request,
            &ClientConfig::default(),
            &Auth::Anonymous,
            "instance",
        )
        .unwrap();
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
            build_body(
                &request,
                &ClientConfig::default(),
                &Auth::Anonymous,
                "instance"
            ),
            Err(Error::TextTooLong { .. })
        ));
    }
}
