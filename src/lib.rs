//! Rust client for DeepL's Chrome extension oneshot translation endpoint.
//!
//! The crate exposes a typed client API. HTTP compatibility responses are kept
//! in the optional server layer, not in the library core.
//!
//! ```no_run
//! use std::time::Duration;
//!
//! use deeplx::{Auth, Client, SourceLang, TargetLang, TranslateRequest};
//!
//! async fn run() -> Result<(), deeplx::Error> {
//!     let client = Client::builder()
//!         .auth(Auth::Anonymous)
//!         .timeout(Duration::from_secs(20))
//!         .build()?;
//!
//!     let response = client
//!         .translate(
//!             TranslateRequest::builder()
//!                 .text("Hello, world!")?
//!                 .source(SourceLang::Auto)
//!                 .target(TargetLang::parse("ZH-HANS")?)
//!                 .build()?,
//!         )
//!         .await?;
//!
//!     println!("{}", response.translations[0].text);
//!     Ok(())
//! }
//! ```
#![cfg_attr(docsrs, feature(doc_cfg))]

mod client;
mod config;
mod error;
mod lang;
mod oneshot;
mod translate;
mod transport;

pub use client::Client;
pub use config::{Auth, ClientConfig, Endpoint, Fingerprint, WarmupMode};
pub use error::Error;
pub use lang::{LanguageCode, SourceLang, TargetLang};
pub use translate::{TranslateRequest, TranslateResponse, Translation};
