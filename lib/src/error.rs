use thiserror::Error;
use image::error::ImageError;

pub type AsciiResult<T> = Result<T, AsciiError>;

#[derive(Debug, Error)]
pub enum AsciiError {
    #[error("image error: {0}")]
    Image(ImageError),

    #[error("invalid dimensions")]
    InvalidDimensions,

    #[error("unsupported format")]
    UnsupportedFormat,
}
