use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct Lang<'a> {
    pub source_lang_user_selected: &'a str,
    pub target_lang: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang_user_selected: Option<&'a str>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct TextItem<'a> {
    pub text: &'a str,
    #[serde(rename = "requestAlternatives")]
    pub request_alternatives: i64,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct Params<'a> {
    pub splitting: &'a str,
    pub lang: Lang<'a>,
    pub texts: Vec<TextItem<'a>>,
    pub timestamp: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct PostData<'a> {
    #[serde(rename = "jsonrpc")]
    pub json_rpc: &'a str,
    pub method: &'a str,
    pub id: i64,
    pub params: Params<'a>,
}

/// A structure representing the final result of a translation.
///
/// This struct aggregates all necessary information after a translation request,
/// including the translated text, alternatives, source language, target language, and more.
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct DeepLXTranslationResult {
    /// The HTTP-like status code. `200` indicates success, other values indicate various errors or failure states.
    pub code: i32,
    /// The unique identifier of the request or response.
    pub id: i64,
    /// A descriptive message providing more context or details about the result, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The primary translated text result.
    pub data: String,
    /// A collection of alternative translations.
    pub alternatives: Vec<String>,
    /// The detected or used source language code.
    pub source_lang: String,
    /// The target language code used for the translation.
    pub target_lang: String,
    /// Indicates the method used, for example "Pro" or "Free".
    pub method: String,
}

impl Default for DeepLXTranslationResult {
    fn default() -> Self {
        DeepLXTranslationResult {
            code: 0,
            id: 0,
            message: None,
            data: "".to_string(),
            alternatives: vec![],
            source_lang: "".to_string(),
            target_lang: "".to_string(),
            method: "".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct Text {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct Texts {
    pub alternatives: Vec<Text>,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct TranslationResponseResult {
    pub texts: Vec<Texts>,
    pub lang: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct TranslationResponse {
    #[serde(rename = "jsonrpc")]
    pub json_rpc: String,
    pub id: i64,
    pub result: TranslationResponseResult,
}
