#[derive(thiserror::Error, Debug)]
#[error("lang detect error: {0}")]
pub struct LangDetectError(#[from] pub std::io::Error);

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    Request(#[from] reqwest::Error),

    #[error(transparent)]
    JSON(#[from] serde_json::Error),

    #[error(transparent)]
    LangDetect(#[from] LangDetectError),
}
