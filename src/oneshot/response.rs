use reqwest::StatusCode;
use serde::Deserialize;

use crate::{Error, TargetLang, TranslateResponse, Translation};

#[derive(Deserialize)]
struct OneshotResponse {
    translations: Vec<OneshotTranslation>,
}

#[derive(Deserialize)]
struct OneshotTranslation {
    detected_source_language: String,
    text: String,
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
    use super::*;

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
