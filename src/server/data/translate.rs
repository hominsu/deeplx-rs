use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use deeplx::{DeepLX, DeepLXTranslationResult};

use crate::server::{biz::translate::TranslateRepo as BizTranslateRepo, pkgs::Error};

pub struct TranslateRepo {
    translator: Arc<DeepLX>,
}

impl TranslateRepo {
    pub fn new(translator: Arc<DeepLX>) -> Self {
        Self { translator }
    }
}

impl BizTranslateRepo for TranslateRepo {
    fn translate<'a>(
        &'a self,
        text: &'a str,
        source_lang: &'a str,
        target_lang: &'a str,
        dl_session: Option<&'a str>,
    ) -> Pin<Box<dyn Future<Output = Result<DeepLXTranslationResult, Error>> + Send + 'a>> {
        Box::pin(async move {
            match self
                .translator
                .translate(source_lang, target_lang, text, dl_session)
                .await
            {
                Ok(res) => Ok(res),
                Err(e) => {
                    tracing::error!(%e);
                    Err(Error::InternalServer)
                }
            }
        })
    }
}
