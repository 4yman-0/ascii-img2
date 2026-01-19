use crate::prelude::{AsciiResult, Charset, Colorizer};
use alloc::{string::String, vec::Vec};
use core::iter::FromIterator;
use image::{GenericImageView, Primitive, Rgb, RgbImage};

mod image_lines_ext;
use image_lines_ext::LinesTrait as _;

pub trait AsciiGenerator<T: GenericImageView> {
    fn generate(
        &self,
        image: &T,
        charset: &dyn Charset,
        colorizer: &dyn Colorizer<T::Pixel>,
    ) -> AsciiResult<Vec<String>>;
}

/// An ASCII generator that uses the `Charset` provided to it
/// ```
/// use ascii_img2::prelude::*;
/// let image = image::RgbImage::new(10, 10);
/// let charset = LinearCharset::new(vec![' ', ';', '&']);
/// let colorizer = NullColorizer;
/// CharsetGenerator.generate(
///     &image,
///     &charset,
///     &colorizer,
/// );
/// ```
#[derive(Clone)]
pub struct CharsetGenerator;

impl CharsetGenerator {
    #[inline]
    fn luminance<T: Primitive + Into<f32>>(rgb: &Rgb<T>) -> f32 {
        // Average luminance
        (rgb[0].into() + rgb[1].into() + rgb[2].into()) / T::DEFAULT_MAX_VALUE.into() / 3.0
    }
}

impl AsciiGenerator<RgbImage> for CharsetGenerator {
    fn generate(
        &self,
        image: &RgbImage,
        charset: &dyn Charset,
        colorizer: &dyn Colorizer<Rgb<u8>>,
    ) -> AsciiResult<Vec<String>> {
        Ok(image
            .lines()
            .map(|line| {
                line.map(|pixel| {
                    let lum = Self::luminance(&pixel);
                    let char = charset.map(lum);
                    String::from_iter(colorizer.fg(&pixel).chars().chain([char]))
                })
                .collect::<String>()
            })
            .collect::<Vec<_>>())
    }
}

/// An ASCII generator that uses Unicode half block
/// This generator must be used with a colorizer other than `NullColorizer`
/// ```
/// use ascii_img2::prelude::*;
/// let image = image::RgbImage::new(10, 10);
/// let charset = LinearCharset::new(vec![' ', ';', '&']);
/// let colorizer = AnsiRgbColorizer;
/// HalfBlockGenerator.generate(
///     &image,
///     &charset,
///     &colorizer,
/// );
/// ```
pub struct HalfBlockGenerator;

impl AsciiGenerator<RgbImage> for HalfBlockGenerator {
    fn generate(
        &self,
        image: &RgbImage,
        _charset: &dyn Charset,
        colorizer: &dyn Colorizer<Rgb<u8>>,
    ) -> AsciiResult<Vec<String>> {
        let mut result: Vec<String> = Vec::with_capacity(image.height() as _);
        let mut lines = image.lines();
        
        while let (Some(top_iter), Some(bottom_iter)) = (lines.next(), lines.next()) {
            result.push(
                top_iter
                    .zip(bottom_iter)
                    .flat_map(|(top, bottom)| {
                        let fg = colorizer.fg(&top);
                        let bg = colorizer.bg(&bottom);

                        fg.chars()
                            .chain(bg.chars())
                            .chain(['▀'])
                            .collect::<Vec<char>>()
                    })
                    .chain("\x1b[0m".chars()) // reset at EOL
                    .collect(),
            );
        }

        Ok(result)
    }
}
