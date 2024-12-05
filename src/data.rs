use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct Lang<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_lang_computed: Option<&'a str>,
    pub target_lang: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang_user_selected: Option<&'a str>,
}

impl Default for Lang<'_> {
    fn default() -> Self {
        Lang {
            source_lang_computed: None,
            target_lang: "",
            lang_user_selected: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct CommonJobParams<'a> {
    pub mode: &'a str,
    #[serde(rename = "regionalVariant", skip_serializing_if = "Option::is_none")]
    pub regional_variant: Option<&'a str>,
}

impl Default for CommonJobParams<'_> {
    fn default() -> Self {
        CommonJobParams {
            mode: "",
            regional_variant: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct Sentence<'a> {
    pub prefix: &'a str,
    pub text: &'a str,
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct Job<'a> {
    pub kind: &'a str,
    pub preferred_num_beams: i32,
    pub raw_en_context_before: Vec<&'a str>,
    pub raw_en_context_after: Vec<&'a str>,
    pub sentences: Vec<Sentence<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
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

impl Default for Params<'_> {
    fn default() -> Self {
        Params {
            common_job_params: CommonJobParams::default(),
            lang: Lang::default(),
            texts: None,
            text_type: None,
            jobs: None,
            priority: None,
            timestamp: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
pub(crate) struct SplitTextResponseResultLang<'a> {
    pub detected: &'a str,
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
    #[serde(borrow)]
    pub lang: SplitTextResponseResultLang<'a>,
    #[serde(borrow)]
    pub texts: Vec<SplitTextResponseResultText<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct SplitTextResponse<'a> {
    #[serde(rename = "jsonrpc")]
    pub json_rpc: &'a str,
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
    pub result: TranslationResponseResult<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeepLXTranslationResult {
    pub code: i32,
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub data: String,
    pub alternatives: Vec<String>,
    pub source_lang: String,
    pub target_lang: String,
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
