use crate::{AsciiError, Charset, Colorizer};
use image::{GenericImageView, Primitive, Rgb, RgbImage};

pub trait AsciiGenerator<T: GenericImageView> {
    fn generate(&self, image: &T, charset: &dyn Charset, colorizer: &dyn Colorizer) -> Result<Vec<String>, AsciiError>;
}

/// ASCII generator using luminance
#[derive(Clone)]
pub struct CharsetGenerator;

impl CharsetGenerator {
    #[inline]
    fn luminance<T: Primitive + Into<f32>>(rgb: &Rgb<T>) -> f32 {
        // Average luminance
        (rgb[0].into() + rgb[1].into() + rgb[2].into()) / T::DEFAULT_MAX_VALUE.into()
    }
}

impl AsciiGenerator<RgbImage> for CharsetGenerator {
    fn generate(
        &self,
        image: &RgbImage,
        charset: &dyn Charset,
        colorizer: &dyn Colorizer,
    ) -> Result<Vec<String>, AsciiError> {
        let (w, h) = image.dimensions();
        let (w, h) = (w as usize, h as usize);
        let mut result: Vec<String> = Vec::with_capacity(h);

        for y in 0..h {
            result.push(
                image
                    .pixels()
                    .skip(y * w)
                    .take(w)
                    .flat_map(|pixel| {
                        let lum = Self::luminance(pixel);
                        let char = charset.map(lum);
                        colorizer.fg(pixel)
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
        colorizer: &dyn Colorizer,
    ) -> Result<Vec<String>, AsciiError> {
        let (w, h) = image.dimensions();
        let (w, h) = (w as usize, h as usize);

        let mut result: Vec<String> = Vec::with_capacity(h);

        for y in (0..h).step_by(2) {
            let top_iter = image.pixels().skip(y * w).take(w);

            let bottom_iter = if y + 1 < h {
                image.pixels().skip((y + 1) * w).take(w)
            } else {
                // duplicate top row if height is odd
                image.pixels().skip(y * w).take(w)
            };

            result.push(
                top_iter
                    .zip(bottom_iter)
                    .flat_map(|(top, bottom)| {
                        let fg = colorizer.fg(top);
                        let bg = colorizer.bg(bottom);

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
