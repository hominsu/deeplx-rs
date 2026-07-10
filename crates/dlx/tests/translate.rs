use dlx::{Auth, Client, SourceLang, TargetLang, TranslateRequest};

async fn translate(source_lang: &str, target_lang: &str, text: &str) {
    let translator = Client::builder()
        .auth(Auth::Anonymous)
        .build()
        .expect("failed to build translator");
    let request = TranslateRequest::builder()
        .text(text)
        .unwrap()
        .source(SourceLang::parse(source_lang).unwrap())
        .target(TargetLang::parse(target_lang).unwrap())
        .build()
        .unwrap();

    let res = translator
        .translate(&request)
        .await
        .expect("translation request failed");
    assert!(!res.translations.is_empty());
    println!("{res:?}");
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
