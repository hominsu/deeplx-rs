use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use dlx::{Auth, Client, SourceLang, TargetLang, TranslateRequest, TranslateResponse};

use crate::server::{biz::translate::TranslateRepo as BizTranslateRepo, pkgs::Error};

pub struct TranslateRepo {
    client: Arc<Client>,
}

impl TranslateRepo {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }
}

impl BizTranslateRepo for TranslateRepo {
    fn translate<'a>(
        &'a self,
        text: &'a str,
        source_lang: &'a str,
        target_lang: &'a str,
        auth: Auth,
    ) -> Pin<Box<dyn Future<Output = Result<TranslateResponse, Error>> + Send + 'a>> {
        Box::pin(async move {
            let request = TranslateRequest::builder()
                .text(text)?
                .source(SourceLang::parse(source_lang)?)
                .target(TargetLang::parse(target_lang)?)
                .build()?;

            self.client
                .translate_with_auth(&request, &auth)
                .await
                .map_err(|e| {
                    tracing::error!(%e);
                    Error::Dlx(e)
                })
        })
    }
}
