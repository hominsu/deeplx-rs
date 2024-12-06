use crate::data::{
    CommonJobParams, DeepLXTranslationResult, Job, Lang, Params, PostData, Sentence,
    SplitTextResponse, TranslationResponse,
};
use crate::utils::{get_i_count, get_random_number, get_timestamp, is_rich_text};
use reqwest::header::{
    HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, AUTHORIZATION, CACHE_CONTROL, CONTENT_TYPE,
    COOKIE, DNT, ORIGIN, PRAGMA, REFERER, USER_AGENT,
};
use reqwest::{Client, Proxy};
use std::error::Error;
use std::io::Read;
use std::sync::Arc;

/// The main entry point for interacting with the DeepL translation service.
///
/// `DeepLX` provides methods to create a translation client and perform translation
/// requests. You can optionally specify a proxy, choose source and target languages, and
/// handle text with or without HTML/XML tags.
#[derive(Clone)]
pub struct DeepLX {
    client: Arc<Client>,
    base_url: &'static str,
    headers: HeaderMap,
}

impl DeepLX {
    /// Constructs a new `DeepLX` instance.
    ///
    /// # Parameters
    ///
    /// * `proxy` - An optional proxy URL, e.g. `"http://127.0.0.1:8080"`. If `None`, no proxy is used.
    ///
    /// # Panics
    ///
    /// This method will panic if the provided proxy string is invalid.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use deeplx::DeepLX;
    ///
    /// let translator = DeepLX::new(None);
    /// let translator_with_proxy = DeepLX::new(Some("http://localhost:8080"));
    /// ```
    pub fn new(proxy: Option<&str>) -> Self {
        let builder = Client::builder();
        let client = match proxy {
            Some(p) => builder.proxy(Proxy::all(p).unwrap()),
            None => builder,
        }
        .build()
        .unwrap();

        Self {
            client: Arc::new(client),
            base_url: "https://www2.deepl.com/jsonrpc",
            headers: headers(),
        }
    }

    async fn make_request(
        &self,
        post_data: &PostData<'_>,
        method: &str,
        deepl_session: Option<&str>,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let full_url = format!(
            "{}?client=chrome-extension,1.28.0&method={}",
            self.base_url, method
        );

        let mut headers = self.headers.clone();
        if let Some(session) = deepl_session {
            headers.insert(COOKIE, session.parse().unwrap());
        }

        let mut data = serde_json::to_string(&post_data)?;

        let replacement = if (post_data.id + 5) % 29 == 0 || (post_data.id + 3) % 13 == 0 {
            r#""method" : ""#
        } else {
            r#""method": ""#
        };
        data = data.replacen(r#""method":""#, replacement, 1);

        let resp = self
            .client
            .post(&full_url)
            .headers(headers)
            .body(data)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(format!("Request failed with status: {}", resp.status()).into());
        }

        let bytes = if resp
            .headers()
            .get("Content-Encoding")
            .map_or(false, |val| val.eq(&HeaderValue::from_static("br")))
        {
            let full = resp.bytes().await?;
            let mut reader = brotli::Decompressor::new(full.as_ref(), 4096);
            let mut vec = Vec::new();
            reader.read_to_end(&mut vec)?;
            vec
        } else {
            let full = resp.bytes().await?;
            full.to_vec()
        };

        Ok(bytes)
    }

    async fn split_text(
        &self,
        text: &str,
        tag_handling: bool,
        deepl_session: Option<&str>,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let post_data = PostData {
            json_rpc: "2.0",
            method: "LMT_split_text",
            id: get_random_number(),
            params: Params {
                common_job_params: CommonJobParams {
                    mode: "translate",
                    ..Default::default()
                },
                lang: Lang {
                    lang_user_selected: Option::from("auto"),
                    ..Default::default()
                },
                texts: Option::from(vec![text]),
                text_type: Some(if tag_handling || is_rich_text(text) {
                    "richtext"
                } else {
                    "plaintext"
                }),
                ..Default::default()
            },
        };
        self.make_request(&post_data, "LMT_split_text", deepl_session)
            .await
    }

    fn extract_jobs<'a>(&self, split_result: &'a SplitTextResponse) -> Vec<Job<'a>> {
        let chunks = &split_result.result.texts[0].chunks;
        chunks
            .iter()
            .enumerate()
            .map(|(idx, chunk)| {
                let sentence = &chunk.sentences[0];
                let context_before = if idx > 0 {
                    vec![chunks[idx - 1].sentences[0].text.as_ref()]
                } else {
                    vec![]
                };
                let context_after = if idx < chunks.len() - 1 {
                    vec![chunks[idx + 1].sentences[0].text.as_ref()]
                } else {
                    vec![]
                };

                Job {
                    kind: "default",
                    preferred_num_beams: 4,
                    raw_en_context_before: context_before,
                    raw_en_context_after: context_after,
                    sentences: vec![Sentence {
                        prefix: sentence.prefix.clone(),
                        text: sentence.text.clone(),
                        id: Some((idx + 1) as i32),
                    }],
                }
            })
            .collect()
    }

    /// Translates the given text from a source language to a target language.
    ///
    /// This method automatically handles splitting the text into translation jobs,
    /// detecting the source language (if set to "auto"), and returning the translated text.
    ///
    /// # Parameters
    ///
    /// * `source_lang` - The source language code, e.g. `"en"`. Use `"auto"` to let the system detect the language.
    /// * `target_lang` - The target language code, e.g. `"zh"` or `"EN-GB"` for a regional variant.
    /// * `text` - The text to translate. Cannot be empty.
    /// * `tag_handling` - An optional parameter specifying the handling of tags, e.g. `"html"`, `"xml"`, or `None`.
    /// * `deepl_session` - An optional session string. If `None`, the "Free" method is used; otherwise "Pro".
    ///
    /// # Returns
    ///
    /// On success, returns a `DeepLXTranslationResult` containing the translated text and alternatives.
    /// On failure, returns an `Err` containing the underlying error.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use deeplx::DeepLX;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let translator = DeepLX::new(None);
    ///     match translator
    ///         .translate("auto", "zh", "Hello, world!", None, None)
    ///         .await {
    ///         Ok(res) => println!("Translated: {}", res.data),
    ///         Err(e) => eprintln!("Error: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn translate(
        &self,
        source_lang: &str,
        target_lang: &str,
        text: &str,
        tag_handling: Option<&str>,
        deepl_session: Option<&str>,
    ) -> Result<DeepLXTranslationResult, Box<dyn Error>> {
        if text.is_empty() {
            return Ok(DeepLXTranslationResult {
                code: 404,
                message: Some("No text to translate".to_string()),
                ..Default::default()
            });
        }

        let bytes = self
            .split_text(
                text,
                matches!(tag_handling, Some("html" | "xml")),
                deepl_session,
            )
            .await?;
        let split_result: SplitTextResponse = serde_json::from_slice(&bytes)?;
        let jobs = self.extract_jobs(&split_result);

        let source_lang_detached = match source_lang {
            "auto" | "" => {
                let code = whatlang::detect_lang(text).unwrap().code();
                isolang::Language::from_639_3(code)
                    .unwrap()
                    .to_639_1()
                    .unwrap()
                    .to_uppercase()
            }
            _ => source_lang.to_uppercase(),
        };

        let target_lang_parts = target_lang.split('-').collect::<Vec<&str>>();
        let (target_lang_code, has_regional_variant) = if target_lang_parts.len() > 1 {
            (target_lang_parts[0].to_uppercase(), true)
        } else {
            (target_lang.to_uppercase(), false)
        };

        let id = get_random_number();
        let post_data = PostData {
            json_rpc: "2.0",
            method: "LMT_handle_jobs",
            id,
            params: Params {
                common_job_params: CommonJobParams {
                    mode: "translate",
                    regional_variant: has_regional_variant.then_some(target_lang),
                },
                lang: Lang {
                    source_lang_computed: Some(source_lang_detached.as_str()),
                    target_lang: target_lang_code.as_str(),
                    ..Default::default()
                },
                jobs: Some(jobs),
                priority: Some(1),
                timestamp: Some(get_timestamp(get_i_count(text))),
                ..Default::default()
            },
        };

        let bytes = &self
            .make_request(&post_data, "LMT_handle_jobs", deepl_session)
            .await?;
        let resp: TranslationResponse = serde_json::from_slice(bytes)?;

        let mut alternatives = Vec::new();
        let mut translated_text = String::new();
        let translations = resp.result.translations;
        if !translations.is_empty() {
            let num_beams = translations[0].beams.len();
            for idx in 0..num_beams {
                let mut alt_text = String::new();
                for translation in &translations {
                    let beams = &translation.beams;
                    if idx < beams.len() {
                        alt_text.push_str(&beams[idx].sentences[0].text);
                    }
                }
                if !alt_text.is_empty() {
                    alternatives.push(alt_text);
                }
            }
            for translation in &translations {
                translated_text.push_str(&translation.beams[0].sentences[0].text);
                translated_text.push(' ');
            }
        }
        translated_text = translated_text.trim().to_string();

        if translated_text.is_empty() {
            return Ok(DeepLXTranslationResult {
                code: 503,
                message: Some("Translation failed".to_string()),
                ..Default::default()
            });
        };

        Ok(DeepLXTranslationResult {
            code: 200,
            id,
            data: translated_text,
            alternatives,
            source_lang: source_lang_detached,
            target_lang: target_lang.to_string(),
            method: if deepl_session.is_none() {
                "Free"
            } else {
                "Pro"
            }
            .to_string(),
            ..Default::default()
        })
    }
}

fn headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("en-US,en;q=0.9,zh-CN;q=0.8,zh-TW;q=0.7,zh-HK;q=0.6,zh;q=0.5"),
    );
    headers.insert(AUTHORIZATION, HeaderValue::from_static("None"));
    headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(DNT, HeaderValue::from_static("1"));
    headers.insert(
        ORIGIN,
        HeaderValue::from_static("chrome-extension://cofdbpoegempjloogbagkncekinflcnj"),
    );
    headers.insert(PRAGMA, HeaderValue::from_static("no-cache"));
    headers.insert("Priority", HeaderValue::from_static("u=1, i"));
    headers.insert(REFERER, HeaderValue::from_static("https://www.deepl.com/"));
    headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("empty"));
    headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("cors"));
    headers.insert("Sec-Fetch-Site", HeaderValue::from_static("none"));
    headers.insert("Sec-GPC", HeaderValue::from_static("1"));
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("DeepLBrowserExtension/1.28.0 Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36"),
    );
    headers
}
