#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
