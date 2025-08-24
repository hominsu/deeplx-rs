//! `deeplx` is a Rust package for unlimited DeepL translation.
//!
//! # Overview
//!
//! `deeplx` provides a convenient interface for interacting with DeepL.
//!
//! By default, `deeplx` includes proxy support. This can be controlled by the `proxy` feature.
//! If you wish to disable proxy support, you can opt out of the default features in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! deeplx = { version = "2", default-features = false }
//! ```
//!
//! When the `proxy` feature is enabled (disabled on wasm32), you can specify a proxy when creating a new [`DeepLX`] instance:
//! ```no_run
//! use deeplx::{Config, DeepLX};
//!
//! #[cfg(not(target_arch = "wasm32"))]
//! let translator = DeepLX::new(Config {
//!     proxy: Some("http://pro.xy".to_string()),
//!     ..Default::default()
//! });
//! ```
//!
//! The core structure of this library is [`DeepLX`], through which you can:
//! - Create a new translation client instance using [`DeepLX::new`].
//! - Perform text translations with [`DeepLX::translate`], automatically detecting the source language if needed, and retrieving both a primary translation and multiple alternative translations.
//!
//! In addition, the library defines several data structures for representing requests and responses from DeepL’s API, ensuring consistency with [OwO-Network/DeepLX](https://github.com/OwO-Network/DeepLX):
//! - [`DeepLXTranslationResult`]: Represents the translation result including status code, source and target languages, translated text, and any alternative translations.
//!
//! By maintaining consistency with the [OwO-Network/DeepLX](https://github.com/OwO-Network/DeepLX),
//! this project aims to provide a familiar and straightforward experience for users looking to integrate DeepL translations into their Rust applications.
//!
//! # Example
//!
//! ```no_run
//! use deeplx::{Config, DeepLX};
//!
//! async fn run() {
//!     let translator = DeepLX::new(Config {   // If proxy enabled, with proxy
//!         #[cfg(not(target_arch = "wasm32"))]
//!         proxy: Some("http://pro.xy".to_string()),
//!         ..Default::default()
//!     });
//!     // let translator = DeepLX::new(Config::default());    // Otherwise
//!     match translator
//!         .translate("auto", "zh", "Hello, world!", None)
//!         .await
//!     {
//!         Ok(res) => println!("Translated: {}", res.data),
//!         Err(e) => eprintln!("{}", e),
//!     }
//! }
//!
//! #[cfg(not(target_arch = "wasm32"))]
//! #[tokio::main]
//! async fn main() {
//!    run().await;
//! }
//!
//! #[cfg(target_arch = "wasm32")]
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() {
//!    run().await;
//! }
//! ```
#![cfg_attr(docsrs, feature(doc_cfg))]

mod skeleton;

pub use skeleton::{
    data::DeepLXTranslationResult,
    error::Error,
    translate::{Config, DeepLX},
};
