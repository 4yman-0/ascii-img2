use crate::{AsciiError, Charset};
use image::{DynamicImage, GrayImage, Primitive, Rgb, RgbImage};

pub trait AsciiGenerator<T: From<DynamicImage>> {
    fn generate(&self, image: &T, charset: &dyn Charset) -> Result<Vec<String>, AsciiError>;
}

/// ASCII generator using luminance
#[derive(Clone)]
pub struct LuminanceGenerator;

impl AsciiGenerator<GrayImage> for LuminanceGenerator {
    fn generate(
        &self,
        image: &GrayImage,
        charset: &dyn Charset,
    ) -> Result<Vec<String>, AsciiError> {
        let (w, h) = image.dimensions();
        let (w, h) = (w as usize, h as usize);
        let mut result: Vec<String> = Vec::with_capacity(h);

        for y in 0..=h {
            result.push(
                image
                    .pixels()
                    .skip(w * y)
                    .take(w)
                    .map(|pixel| charset.map(f32::from(pixel[0]) / u8::MAX as f32))
                    .collect::<String>(),
            );
        }

        Ok(result)
    }
}

/// ANSI escape helpers
pub mod ansi {
    pub const RESET: &str = "\x1b[0m";

    #[inline]
    pub fn fg_rgb(r: u8, g: u8, b: u8) -> String {
        format!("\x1b[38;2;{r};{g};{b}m")
    }

    #[inline]
    pub fn bg_rgb(r: u8, g: u8, b: u8) -> String {
        format!("\x1b[48;2;{r};{g};{b}m")
    }

    #[inline]
    fn rgb_to_256(r: u8, g: u8, b: u8) -> u8 {
        // Check for grayscale first
        if r == g && g == b {
            if r < 8 {
                return 16;
            }
            if r > 248 {
                return 231;
            }
            // 24 grayscale colors from 232 to 255
            assert!(((r as u16 - 9) / 10) < 24);
            return 231 + ((r as u16 - 8) / 10) as u8;
        }

        // Convert RGB components to 0–5 range
        let r = (r as u16 * 5 / 255) as u8;
        let g = (g as u16 * 5 / 255) as u8;
        let b = (b as u16 * 5 / 255) as u8;

        // 16 is the start of the 6×6×6 color cube
        16 + 36 * r + 6 * g + b
    }

    pub fn fg_256(r: u8, g: u8, b: u8) -> String {
        format!("\x1b[38;5;{}m", rgb_to_256(r, g, b))
    }

    pub fn bg_256(r: u8, g: u8, b: u8) -> String {
        format!("\x1b[48;5;{}m", rgb_to_256(r, g, b))
    }
}

/// ANSI ASCII generator using truecolor foreground RGB.
#[derive(Clone)]
pub struct AnsiRgbGenerator;

impl AnsiRgbGenerator {
    #[inline]
    fn luminance<T: Primitive + Into<f32>>(rgb: &Rgb<T>) -> f32 {
        // Average luminance
        (rgb[0].into() + rgb[1].into() + rgb[2].into()) / T::DEFAULT_MAX_VALUE.into()
    }
}

impl AsciiGenerator<RgbImage> for AnsiRgbGenerator {
    fn generate(&self, image: &RgbImage, charset: &dyn Charset) -> Result<Vec<String>, AsciiError> {
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
                        ansi::fg_rgb(pixel[0], pixel[1], pixel[2])
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

#[derive(Clone)]
pub struct Ansi256Generator;

impl Ansi256Generator {
    #[inline]
    fn luminance<T: Primitive + Into<f32>>(rgb: &Rgb<T>) -> f32 {
        // Average luminance
        (rgb[0].into() + rgb[1].into() + rgb[2].into()) / T::DEFAULT_MAX_VALUE.into()
    }
}

impl AsciiGenerator<RgbImage> for Ansi256Generator {
    fn generate(&self, image: &RgbImage, charset: &dyn Charset) -> Result<Vec<String>, AsciiError> {
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
                        ansi::fg_256(pixel[0], pixel[1], pixel[2])
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
                        let fg = ansi::fg_rgb(top[0], top[1], top[2]);
                        let bg = ansi::bg_rgb(bottom[0], bottom[1], bottom[2]);

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
