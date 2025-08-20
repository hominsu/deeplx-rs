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
            .translate("auto", "zh", "Hello, world!", None)
            .await
        {
            Ok(res) => {
                assert_eq!(res.code, 200);
                println!("{res:?}")
            }
            Err(e) => eprintln!("{e:?}"),
        }
    }

    #[tokio::test]
    async fn test_translate_confident() {
        match TRANSLATOR
            .translate("en", "zh", "Hello, world!", None)
            .await
        {
            Ok(res) => {
                assert_eq!(res.code, 200);
                println!("{res:?}")
            }
            Err(e) => eprintln!("{e:?}"),
        }
    }

    #[tokio::test]
    async fn test_translate_new_line() {
        match TRANSLATOR
            .translate("auto", "zh", "Hello\nworld!", None)
            .await
        {
            Ok(res) => {
                assert_eq!(res.code, 200);
                println!("{res:?}")
            }
            Err(e) => eprintln!("{e:?}"),
        }
    }
}
