use crate::server::{biz::translate::TranslateRepo as BizTranslateRepo, pkgs::Error};

use deeplx::{DeepLX, DeepLXTranslationResult};
use std::{future::Future, pin::Pin, sync::Arc};

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
        tag_handling: Option<&'a str>,
        dl_session: Option<&'a str>,
    ) -> Pin<Box<dyn Future<Output = Result<DeepLXTranslationResult, Error>> + Send + 'a>> {
        Box::pin(async move {
            match self
                .translator
                .translate(source_lang, target_lang, text, tag_handling, dl_session)
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
