#[cfg(test)]
mod tests {
    use deeplx::translate::DeepLX;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref ENDPOINT: DeepLX = DeepLX::new(None);
    }

    #[tokio::test]
    async fn test_translate() {
        let res = ENDPOINT
            .translate("auto", "zh", "Hello, world!", None, None)
            .await
            .unwrap();
        assert_eq!(res.code, 200);
        assert_eq!(res.data, "你好，世界");
    }
}
