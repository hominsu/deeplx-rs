use std::sync::LazyLock;

use deeplx::{Auth, Client, SourceLang, TargetLang, TranslateRequest};

static TRANSLATOR: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .auth(Auth::Anonymous)
        .build()
        .expect("failed to build translator")
});

async fn translate(source_lang: &str, target_lang: &str, text: &str) {
    let request = TranslateRequest::builder()
        .text(text)
        .unwrap()
        .source(SourceLang::parse(source_lang).unwrap())
        .target(TargetLang::parse(target_lang).unwrap())
        .build()
        .unwrap();

    match TRANSLATOR.translate(&request).await {
        Ok(res) => {
            assert!(!res.translations.is_empty());
            println!("{res:?}")
        }
        Err(e) => eprintln!("{e:?}"),
    }
}

#[tokio::test]
async fn test_translate() {
    translate("auto", "zh", "Hello, world!").await;
}

#[tokio::test]
async fn test_translate_confident() {
    translate("en", "zh", "Hello, world!").await;
}

#[tokio::test]
async fn test_translate_new_line() {
    translate("auto", "zh", "Hello\nworld!").await;
}
