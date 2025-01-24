use super::{
    data::{
        CommonJobParams, DeepLXTranslationResult, Job, Lang, Params, PostData, Sentence,
        SplitTextResponse, TranslationResponse,
    },
    utils::{get_i_count, get_random_number, get_timestamp, is_rich_text},
};

use std::{error::Error, sync::Arc};

#[cfg(feature = "impersonate")]
use rquest::{
    header::{
        HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, AUTHORIZATION, CACHE_CONTROL,
        CONTENT_TYPE, COOKIE, DNT, ORIGIN, PRAGMA, REFERER, USER_AGENT,
    },
    Client, Impersonate,
};

#[cfg(feature = "impersonate")]
#[cfg(feature = "proxy")]
use rquest::Proxy;

#[cfg(not(feature = "impersonate"))]
use reqwest::{
    header::{
        HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, AUTHORIZATION, CACHE_CONTROL,
        CONTENT_TYPE, COOKIE, DNT, ORIGIN, PRAGMA, REFERER, USER_AGENT,
    },
    Client,
};

#[cfg(not(feature = "impersonate"))]
#[cfg(feature = "proxy")]
use reqwest::Proxy;

/// Configuration settings for the `DeepLX` translation client.
///
/// # Examples
///
/// ## Using Default Configuration
///
/// ```no_run
/// use deeplx::{Config, DeepLX};
///
/// let translator = DeepLX::new(Config::default());
/// ```
///
/// ## Custom Base URL
///
/// ```no_run
/// use deeplx::{Config, DeepLX};
///
/// let translator = DeepLX::new(Config {
///     base_url: "https://custom.deepl.api/jsonrpc".to_string(),
///     ..Default::default()
/// });
/// ```
///
/// ## Configuring a Proxy (Requires `proxy` Feature)
///
/// ```no_run
/// use deeplx::{Config, DeepLX};
///
/// let translator = DeepLX::new(Config {
///     proxy: Some("http://pro.xy".to_string()),
///     ..Default::default()
/// });
/// ```
pub struct Config {
    pub base_url: String,
    #[cfg(feature = "proxy")]
    pub proxy: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_url: "https://www2.deepl.com/jsonrpc".to_string(),
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }
}

/// The main entry point for interacting with the DeepL translation service.
///
/// `DeepLX` provides methods to create a translation client and perform translation
/// requests. You can optionally specify a proxy (with default feature `proxy`),
/// choose source and target languages, and handle text with or without HTML/XML tags.
#[derive(Clone)]
pub struct DeepLX {
    base_url: String,
    client: Arc<Client>,
    headers: HeaderMap,
}

impl DeepLX {
    /// Constructs a new `DeepLX` instance.
    ///
    /// # Parameters
    ///
    /// * `Config` - Configuration settings for the translation client.
    ///
    /// # Panics
    ///
    /// This method will panic if the provided proxy string is invalid.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use deeplx::{Config, DeepLX};
    ///
    /// let translator = DeepLX::new(Config::default());
    /// let translator_with_proxy = DeepLX::new(Config {
    ///     proxy: Some("http://pro.xy".to_string()),
    ///     ..Default::default()
    /// });
    /// ```
    pub fn new(config: Config) -> Self {
        #[cfg(feature = "impersonate")]
        let builder = Client::builder().impersonate(Impersonate::Chrome131);

        #[cfg(not(feature = "impersonate"))]
        let builder = Client::builder();

        #[cfg(feature = "proxy")]
        let client = match config.proxy {
            Some(p) => builder.proxy(Proxy::all(p).unwrap()),
            None => builder,
        }
        .build()
        .unwrap();

        #[cfg(not(feature = "proxy"))]
        let client = builder.build().unwrap();

        Self {
            base_url: config.base_url,
            client: Arc::new(client),
            headers: headers(),
        }
    }

    async fn make_request(
        &self,
        post_data: &PostData<'_>,
        method: &str,
        deepl_session: Option<&str>,
    ) -> Result<bytes::Bytes, Box<dyn Error>> {
        let full_url = format!(
            "{}?client=chrome-extension,1.28.0&method={}",
            self.base_url, method
        );

        let mut headers = self.headers.clone();
        if let Some(session) = deepl_session {
            headers.insert(COOKIE, session.parse().unwrap());
        }

        let data = serde_json::to_string(&post_data)?;

        let use_colon_spacing = ((post_data.id + 5) % 29 == 0) || ((post_data.id + 3) % 13 == 0);
        let replacement = if use_colon_spacing {
            r#""method" : ""#
        } else {
            r#""method": ""#
        };

        let data = data.replacen(r#""method":""#, replacement, 1);

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

        Ok(resp.bytes().await?)
    }

    async fn split_text(
        &self,
        text: &str,
        tag_handling: bool,
        deepl_session: Option<&str>,
    ) -> Result<SplitTextResponse, Box<dyn Error>> {
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

        let resp = self
            .make_request(&post_data, "LMT_split_text", deepl_session)
            .await?;

        Ok(serde_json::from_slice(&resp)?)
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
    /// use deeplx::{Config, DeepLX};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let translator = DeepLX::new(Config::default());
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
        // return if there's nothing to translate
        if text.is_empty() {
            return Ok(DeepLXTranslationResult {
                code: 404,
                message: Some("No text to translate".to_string()),
                ..Default::default()
            });
        }

        let text_parts = text.split("\n").collect::<Vec<&str>>();

        let mut translated_parts: Vec<String> = Vec::new();
        let mut all_alternatives: Vec<Vec<String>> = Vec::new();

        for part in text_parts {
            if part.trim().is_empty() {
                translated_parts.push(String::new());
                all_alternatives.push(vec![String::new()]);
                continue;
            }

            // split text
            let is_tag = matches!(tag_handling, Some("html" | "xml"));
            let split_result = self.split_text(part, is_tag, deepl_session).await?;

            // build jobs for translation
            let jobs = self.extract_jobs(&split_result);

            // determine source language
            let source_lang_detached = match source_lang {
                "auto" | "" => {
                    let iso_639_3 = whatlang::detect_lang(part)
                        .ok_or("Failed to detect language")?
                        .code();

                    isolang::Language::from_639_3(iso_639_3)
                        .and_then(|lang| lang.to_639_1())
                        .map(|iso_639_1| iso_639_1.to_uppercase())
                        .ok_or("Could not map detected language to ISO 639-1")?
                }
                _ => source_lang.to_uppercase(),
            };

            // check target language
            let target_lang_parts = target_lang.split('-').collect::<Vec<&str>>();
            let (target_lang_code, has_regional_variant) = if target_lang_parts.len() > 1 {
                (target_lang_parts[0].to_uppercase(), true)
            } else {
                (target_lang.to_uppercase(), false)
            };

            // prepare the JSON-RPC request
            let post_data = PostData {
                json_rpc: "2.0",
                method: "LMT_handle_jobs",
                id: get_random_number(),
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
                    timestamp: Some(get_timestamp(get_i_count(part))),
                    ..Default::default()
                },
            };

            // send request and parse response
            let resp = &self
                .make_request(&post_data, "LMT_handle_jobs", deepl_session)
                .await?;
            let resp: TranslationResponse = serde_json::from_slice(resp)?;

            let translations = resp.result.translations;
            if translations.is_empty() {
                return Ok(DeepLXTranslationResult {
                    code: 503,
                    message: Some("Translation failed".to_string()),
                    ..Default::default()
                });
            }

            // collect alternatives
            let num_beams = translations[0].beams.len();
            let mut part_alternatives = Vec::new();
            for i in 1..num_beams {
                let alt_text = translations
                    .iter()
                    .filter_map(|tr| tr.beams.get(i))
                    .map(|beam| beam.sentences[0].text.as_str())
                    .collect::<Vec<&str>>()
                    .join(" ")
                    .trim()
                    .to_string();

                if !alt_text.is_empty() {
                    part_alternatives.push(alt_text);
                }
            }

            // best translation
            let part_translation = translations
                .iter()
                .filter_map(|tr| tr.beams.first())
                .map(|beam| beam.sentences[0].text.as_str())
                .collect::<Vec<&str>>()
                .join(" ")
                .trim()
                .to_string();

            if part_translation.is_empty() {
                return Ok(DeepLXTranslationResult {
                    code: 503,
                    message: Some("Translation failed".to_string()),
                    ..Default::default()
                });
            };

            all_alternatives.push(part_alternatives);
            translated_parts.push(part_translation);
        }

        // combine translations and alternatives
        let translated_text = translated_parts.join("\n");

        let max_alts = all_alternatives
            .iter()
            .map(|alts| alts.len())
            .max()
            .unwrap_or(0);

        let mut combined_alternatives = Vec::with_capacity(max_alts);
        for i in 0..max_alts {
            let alt = all_alternatives
                .iter()
                .zip(&translated_parts)
                .map(|(alts, part)| match alts.get(i) {
                    Some(alt) => alt.as_str(),
                    None if part.is_empty() => "",
                    _ => part.as_str(),
                })
                .collect::<Vec<&str>>()
                .join("\n");

            combined_alternatives.push(alt);
        }

        Ok(DeepLXTranslationResult {
            code: 200,
            id: get_random_number(),
            data: translated_text,
            alternatives: combined_alternatives,
            source_lang: source_lang.to_string(),
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
