use crate::{Error, SourceLang, TargetLang};

#[derive(Clone, Debug)]
pub struct TranslateRequest {
    pub text: Vec<String>,
    pub source: SourceLang,
    pub target: TargetLang,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TranslateResponse {
    pub translations: Vec<Translation>,
    pub source_lang: Option<String>,
    pub target_lang: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Translation {
    pub detected_source_language: String,
    pub text: String,
}

#[derive(Debug, Default)]
pub struct TranslateRequestBuilder {
    text: Vec<String>,
    source: SourceLang,
    target: Option<TargetLang>,
}

impl Default for SourceLang {
    fn default() -> Self {
        Self::Auto
    }
}

impl TranslateRequest {
    pub fn builder() -> TranslateRequestBuilder {
        TranslateRequestBuilder::default()
    }

    pub(crate) fn text_char_count(&self) -> usize {
        self.text.iter().map(|item| item.chars().count()).sum()
    }
}

impl TranslateRequestBuilder {
    pub fn text(mut self, text: impl Into<String>) -> Result<Self, Error> {
        let text = text.into();
        if text.is_empty() {
            return Err(Error::EmptyText);
        }
        self.text = vec![text];
        Ok(self)
    }

    pub fn texts<I, S>(mut self, texts: I) -> Result<Self, Error>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let text = texts.into_iter().map(Into::into).collect::<Vec<_>>();
        if text.is_empty() || text.iter().any(String::is_empty) {
            return Err(Error::EmptyText);
        }
        self.text = text;
        Ok(self)
    }

    pub fn source(mut self, source: SourceLang) -> Self {
        self.source = source;
        self
    }

    pub fn target(mut self, target: TargetLang) -> Self {
        self.target = Some(target);
        self
    }

    pub fn build(self) -> Result<TranslateRequest, Error> {
        if self.text.is_empty() {
            return Err(Error::EmptyText);
        }

        Ok(TranslateRequest {
            text: self.text,
            source: self.source,
            target: self.target.ok_or(Error::MissingTargetLang)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_text() {
        assert!(matches!(
            TranslateRequest::builder().text(""),
            Err(Error::EmptyText)
        ));
        assert!(matches!(
            TranslateRequest::builder().texts(Vec::<String>::new()),
            Err(Error::EmptyText)
        ));
    }
}
