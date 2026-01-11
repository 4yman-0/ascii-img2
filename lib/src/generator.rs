use crate::{AsciiError, Charset};
use image::{DynamicImage, GrayImage, RgbImage, Rgb, Primitive};

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
		let mut result: Vec<String> = Vec::with_capacity(h as usize);
		
		for y in 0..=h {
			result.push(image.pixels()
				.skip(w as usize * y as usize)
				.take(w as usize).map(|pixel| {
				charset.map(pixel[0] as f32 / 255.0)
			}).collect::<String>());
		}

		Ok(result)
    }
}

/// ANSI escape helpers
pub mod ansi {
    pub const RESET: &str = "\x1b[0m";

    #[inline]
    pub fn fg_rgb(r: u8, g: u8, b: u8) -> String {
        format!("\x1b[38;2;{};{};{}m", r, g, b)
    }

    #[inline]
    pub fn bg_rgb(r: u8, g: u8, b: u8) -> String {
        format!("\x1b[48;2;{};{};{}m", r, g, b)
    }
}

/// ANSI ASCII generator using truecolor foreground RGB.
#[derive(Clone)]
pub struct AnsiRgbGenerator;

impl AnsiRgbGenerator {
	#[inline]
	fn luminance<T: Primitive + Into<f32>>(rgb: &Rgb<T>) -> f32 {
	    // ITU-R BT.709 luminance
	    (rgb[0].into() + rgb[1].into() + rgb[2].into())
	        / T::DEFAULT_MAX_VALUE.into()
	}
}

impl AsciiGenerator<RgbImage> for AnsiRgbGenerator {
    fn generate(
        &self,
        image: &RgbImage,
        charset: &dyn Charset,
    ) -> Result<Vec<String>, AsciiError> {
		let (w, h) = image.dimensions();
		let mut result: Vec<String> = Vec::with_capacity(h as usize);

    	for y in 0..h {
    		result.push(image.pixels()
    			.skip(y as usize * w as usize)
    			.take(w as usize)
    			.flat_map(|pixel| {
				let lum = Self::luminance(pixel);
				let char = charset.map(lum);
				ansi::fg_rgb(pixel[0], pixel[1], pixel[2]).chars().chain([char]).collect::<Vec<char>>()
			}).collect());
		}

		Ok(result)
    }
}
