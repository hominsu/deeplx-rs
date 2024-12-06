//! `deeplx` is a Rust package for unlimited DeepL translation.
//!
//! # Overview
//!
//! `deeplx` provides a convenient interface for interacting with DeepL.
//!
//! The core structure of this library is [`DeepLX`], through which you can:
//!
//! - Create a new translation client instance using [`DeepLX::new`], optionally with a proxy.
//! - Perform text translations with [`DeepLX::translate`], automatically detecting the source language if needed, and retrieving both a primary translation and multiple alternative translations.
//!
//! In addition, the library defines several data structures for representing requests and responses from DeepL’s API, ensuring consistency with [OwO-Network/DeepLX](https://github.com/OwO-Network/DeepLX):
//!
//! - [`DeepLXTranslationResult`]: Represents the translation result including status code, source and target languages, translated text, and any alternative translations.
//!
//! By maintaining consistency with the [OwO-Network/DeepLX](https://github.com/OwO-Network/DeepLX), this project aims to provide a familiar and straightforward experience for users looking to integrate DeepL translations into their Rust applications.
//!
//! # Example
//!
//! ```no_run
//! use deeplx::DeepLX;
//!
//! #[tokio::main]
//! async fn main() {
//!     let translator = DeepLX::new(None);
//!     match translator
//!         .translate("auto", "zh", "Hello, world!", None, None)
//!         .await
//!     {
//!         Ok(res) => println!("Translated: {}", res.data),
//!         Err(e) => eprintln!("{}", e),
//!     }
//! }
//! ```
#![cfg_attr(docsrs, feature(doc_cfg))]

mod data;
mod translate;
mod utils;

pub use data::DeepLXTranslationResult;
pub use translate::DeepLX;
