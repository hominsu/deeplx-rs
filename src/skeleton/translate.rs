use super::data::{DeepLXTranslationResult, Lang, Params, PostData, TextItem, TranslationResponse};
use super::utils::{get_i_count, get_random_number, get_timestamp};

use std::error::Error;

#[cfg(feature = "impersonate")]
use rquest::{
    header::{
        HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, AUTHORIZATION, CACHE_CONTROL,
        CONTENT_TYPE, COOKIE, DNT, ORIGIN, PRAGMA, REFERER, USER_AGENT,
    },
    Client, Impersonate, Proxy,
};

#[cfg(not(feature = "impersonate"))]
use reqwest::{
    header::{
        HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, AUTHORIZATION, CACHE_CONTROL,
        CONTENT_TYPE, COOKIE, DNT, ORIGIN, PRAGMA, REFERER, USER_AGENT,
    },
    Client, Proxy,
};

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
    pub proxy: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_url: "https://www2.deepl.com/jsonrpc".to_string(),
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
    proxy: Option<String>,
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
        Self {
            base_url: config.base_url,
            proxy: config.proxy,
            headers: headers(),
        }
    }

    async fn make_request(
        &self,
        post_data: &PostData<'_>,
        deepl_session: Option<&str>,
    ) -> Result<bytes::Bytes, Box<dyn Error>> {
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

        #[cfg(feature = "impersonate")]
        let builder = Client::builder().impersonate(Impersonate::Chrome131);

        #[cfg(not(feature = "impersonate"))]
        let builder = Client::builder();

        let builder = match &self.proxy {
            Some(p) => builder.proxy(Proxy::all(p.clone()).unwrap()),
            None => builder,
        };

        let resp = builder
            .build()?
            .post(&self.base_url)
            .headers(headers)
            .body(data)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(format!("Request failed with status: {}", resp.status()).into());
        }

        Ok(resp.bytes().await?)
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
        _: Option<&str>,
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

        // determine source language
        let source_lang_detached = match source_lang {
            "auto" | "" => {
                let iso_639_3 = whatlang::detect_lang(text)
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
        let (target_lang_code, _) = if target_lang_parts.len() > 1 {
            (target_lang_parts[0].to_uppercase(), true)
        } else {
            (target_lang.to_uppercase(), false)
        };

        // prepare the JSON-RPC request
        let id = get_random_number();
        let i_count = get_i_count(text);
        let timestamp = get_timestamp(i_count);

        let post_data = PostData {
            json_rpc: "2.0",
            method: "LMT_handle_texts",
            id,
            params: Params {
                splitting: "newlines",
                lang: Lang {
                    source_lang_user_selected: source_lang,
                    target_lang: target_lang_code.as_str(),
                    ..Default::default()
                },
                texts: vec![TextItem {
                    text,
                    request_alternatives: 3,
                }],
                timestamp,
            },
        };

        // send request and parse response
        let bytes = &self.make_request(&post_data, deepl_session).await?;
        let resp: TranslationResponse = serde_json::from_slice(bytes)?;

        let texts = resp.result.texts;
        if texts.is_empty() {
            return Ok(DeepLXTranslationResult {
                code: 503,
                message: Some("Translation failed".to_string()),
                ..Default::default()
            });
        }

        let main_translation = texts[0].text.clone();

        // collect alternatives
        // let num_beams = translations[0].beams.len();
        let mut alternatives = Vec::new();
        for t in &texts {
            let alt_text = t.text.clone();
            if !alt_text.is_empty() {
                alternatives.push(alt_text);
            }
        }

        Ok(DeepLXTranslationResult {
            code: 200,
            id,
            data: main_translation,
            alternatives,
            source_lang: if resp.result.lang.is_empty() {
                source_lang_detached
            } else {
                resp.result.lang
            },
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
