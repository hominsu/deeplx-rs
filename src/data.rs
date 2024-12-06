use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct Lang<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_lang_computed: Option<&'a str>,
    pub target_lang: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang_user_selected: Option<&'a str>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct CommonJobParams<'a> {
    pub mode: &'a str,
    #[serde(rename = "regionalVariant", skip_serializing_if = "Option::is_none")]
    pub regional_variant: Option<&'a str>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct Sentence<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<Cow<'a, str>>,
    pub text: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct Job<'a> {
    pub kind: &'a str,
    pub preferred_num_beams: i32,
    pub raw_en_context_before: Vec<&'a str>,
    pub raw_en_context_after: Vec<&'a str>,
    pub sentences: Vec<Sentence<'a>>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct Params<'a> {
    #[serde(rename = "commonJobParams")]
    pub common_job_params: CommonJobParams<'a>,
    pub lang: Lang<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texts: Option<Vec<&'a str>>,
    #[serde(rename = "textType", skip_serializing_if = "Option::is_none")]
    pub text_type: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<Job<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct SplitTextResponseResultLang {
    pub detected: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct SplitTextResponseResultChunk<'a> {
    #[serde(borrow)]
    pub sentences: Vec<Sentence<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct SplitTextResponseResultText<'a> {
    #[serde(borrow)]
    pub chunks: Vec<SplitTextResponseResultChunk<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct SplitTextResponseResult<'a> {
    pub lang: SplitTextResponseResultLang,
    #[serde(borrow)]
    pub texts: Vec<SplitTextResponseResultText<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct SplitTextResponse<'a> {
    #[serde(rename = "jsonrpc")]
    pub json_rpc: String,
    pub id: i64,
    #[serde(borrow)]
    pub result: SplitTextResponseResult<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct TranslationResponseResultBeam<'a> {
    #[serde(borrow)]
    pub sentences: Vec<Sentence<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct TranslationResponseResultTranslation<'a> {
    #[serde(borrow)]
    pub beams: Vec<TranslationResponseResultBeam<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct TranslationResponseResult<'a> {
    #[serde(borrow)]
    pub translations: Vec<TranslationResponseResultTranslation<'a>>,
    pub source_lang: &'a str,
    pub target_lang: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct TranslationResponse<'a> {
    #[serde(rename = "jsonrpc")]
    pub json_rpc: &'a str,
    pub id: i64,
    #[serde(borrow)]
    pub result: TranslationResponseResult<'a>,
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
