use deeplx::{Auth, Client, Endpoint, Error, SourceLang, TargetLang, TranslateRequest};
use reqwest::Url;
use secrecy::SecretString;

#[test]
fn builds_public_translate_request() {
    let request = TranslateRequest::builder()
        .text("Hello")
        .unwrap()
        .source(SourceLang::Auto)
        .target(TargetLang::parse("ZH-HANS").unwrap())
        .build()
        .unwrap();

    assert_eq!(request.text, vec!["Hello"]);
    assert_eq!(request.target.code(), "ZH-HANS");
}

#[tokio::test]
async fn translate_rejects_missing_custom_pro_endpoint_before_network() {
    let client = Client::builder()
        .endpoint(Endpoint::Custom {
            free_url: Url::parse("http://127.0.0.1:1/free").unwrap(),
            pro_url: None,
        })
        .build()
        .unwrap();
    let request = TranslateRequest::builder()
        .text("Hello")
        .unwrap()
        .source(SourceLang::parse("EN").unwrap())
        .target(TargetLang::parse("DE").unwrap())
        .build()
        .unwrap();

    let err = client
        .translate_with_auth(
            request,
            Auth::Bearer(SecretString::from("token.value".to_string())),
        )
        .await
        .unwrap_err();

    assert!(matches!(err, Error::MissingProEndpoint));
}
