use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("generation failed")]
    GenerationFailed,
    #[error("fetch failed")]
    FetchFailed,
}