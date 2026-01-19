use alloc::{vec::Vec, string::String};
use crate::prelude::{AsciiResult, Charset, Colorizer};
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

/// ASCII generator using luminance
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
        let mut result: Vec<String> = Vec::with_capacity(image.height() as _);

        for line in image.lines() {
            result.push(
                line.flat_map(|pixel| {
                    let lum = Self::luminance(&pixel);
                    let char = charset.map(lum);
                    colorizer
                        .fg(&pixel)
                        .chars()
                        .chain([char])
                        .collect::<Vec<char>>()
                })
                .collect(),
            );
        }

        Ok(result)
    }
}

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
