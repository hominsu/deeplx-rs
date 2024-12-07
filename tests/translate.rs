#[cfg(test)]
mod tests {
    use deeplx::{Config, DeepLX};
    use lazy_static::lazy_static;

    lazy_static! {
        static ref TRANSLATOR: DeepLX = DeepLX::new(Config::default());
    }

    #[tokio::test]
    async fn test_translate() {
        match TRANSLATOR
            .translate("auto", "zh", "Hello, world!", None, None)
            .await
        {
            Ok(res) => {
                assert_eq!(res.code, 200);
                println!("{}", res.data)
            }
            Err(e) => eprintln!("{}", e),
        }
    }
}
