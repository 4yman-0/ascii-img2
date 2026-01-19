use image::{DynamicImage, GenericImageView as _};

pub trait Preprocessor {
    fn process(&self, image: &DynamicImage) -> DynamicImage;
}

/// Basic grayscale + resize preprocessor
pub struct BasicPreprocessor {
    pub dimensions: (u32, u32),
}

impl Preprocessor for BasicPreprocessor {
    fn process(&self, image: &DynamicImage) -> DynamicImage {
        if self.dimensions == image.dimensions() {
            image.clone()
        } else {
            image.resize_exact(
                self.dimensions.0,
                self.dimensions.1,
                image::imageops::FilterType::Nearest,
            )
        }
    }
}

/// Null preprocessor
pub struct NullPreprocessor;

impl Preprocessor for NullPreprocessor {
    fn process(&self, image: &DynamicImage) -> DynamicImage {
        image.clone()
    }
}
