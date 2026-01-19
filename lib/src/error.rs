use image::error::ImageError;
use thiserror::Error;

pub type AsciiResult<T> = Result<T, AsciiError>;

/// Error thrown by an `AsciiGenerator`
#[derive(Debug, Error)]
pub enum AsciiError {
    #[error("image error: {0}")]
    Image(ImageError),
    // etc
}
