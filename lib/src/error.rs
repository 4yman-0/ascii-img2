use core::fmt::Error as FmtError;
use image::error::ImageError;
use thiserror::Error;

pub type AsciiResult<T> = Result<T, AsciiError>;

/// Error thrown by an `AsciiGenerator`
#[derive(Debug, Error)]
pub enum AsciiError {
    #[error("image error: {0}")]
    Image(ImageError),

	#[error("formatting error: {0}")]
	Fmt(FmtError),
}

impl From<ImageError> for AsciiError {
	fn from(from: ImageError) -> Self {
		Self::Image(from)
	}
}

impl From<FmtError> for AsciiError {
	fn from(from: FmtError) -> Self {
		Self::Fmt(from)
	}
}
