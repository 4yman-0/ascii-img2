use thiserror::Error;

#[derive(Debug, Error)]
pub enum AsciiError {
    #[error("image processing error: {0}")]
    Image(String),

    #[error("invalid dimensions")]
    InvalidDimensions,

    #[error("unsupported format")]
    UnsupportedFormat,
}
