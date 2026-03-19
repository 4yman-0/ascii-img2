use crate::prelude::{AsciiResult, Charset, Colorizer};
use image::{GenericImageView, Primitive, Rgb, RgbImage};

mod image_lines_ext;
use image_lines_ext::LinesTrait as _;

pub trait AsciiGenerator<T: GenericImageView> {
    fn generate<W: core::fmt::Write>(
        &self,
        writer: &mut W,
        image: &T,
        charset: &dyn Charset,
        colorizer: &dyn Colorizer<T::Pixel>,
    ) -> AsciiResult<()>;
}

/// An ASCII generator that uses the `Charset` provided to it
/// ```rust
/// use ascii_img2::prelude::*;
/// let image = image::RgbImage::new(10, 10);
/// let charset = LinearCharset::new(vec![' ', ';', '&']);
/// let colorizer = NullColorizer;
/// let mut output = String::new();
/// CharsetGenerator.generate(
///     &mut output,
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
    fn generate<W: core::fmt::Write>(
        &self,
        writer: &mut W,
        image: &RgbImage,
        charset: &dyn Charset,
        colorizer: &dyn Colorizer<Rgb<u8>>,
    ) -> AsciiResult<()> {
    	for line in image.lines() {
			for pixel in line {
			    let lum = Self::luminance(&pixel);
			    let character = charset.map(lum);
				writeln!(
					writer,
			    	"{}{character}",
			    	colorizer
			    	    .fg(&pixel),
			    )?;
			}
		}
		Ok(())
    }
}

/// An ASCII generator that uses Unicode half blocks.
/// This generator must be used with a colorizer other than `NullColorizer`.
/// ```rust
/// use ascii_img2::prelude::*;
/// let image = image::RgbImage::new(10, 10);
/// let charset = LinearCharset::new(vec![' ', ';', '&']);
/// let colorizer = AnsiRgbColorizer;
/// let mut output = String::new();
/// HalfBlockGenerator.generate(
///     &mut output,
///     &image,
///     &charset,
///     &colorizer,
/// );
/// ```
#[derive(Clone)]
pub struct HalfBlockGenerator;

impl AsciiGenerator<RgbImage> for HalfBlockGenerator {
    fn generate<W: core::fmt::Write>(
        &self,
        writer: &mut W,
        image: &RgbImage,
        _charset: &dyn Charset,
        colorizer: &dyn Colorizer<Rgb<u8>>,
    ) -> AsciiResult<()> {
    	let mut lines = image.lines();
		
        while let (Some(top_line), Some(bottom_line)) = (lines.next(), lines.next()) {
            for (top, bottom) in top_line.zip(bottom_line) {
				let bg = colorizer.bg(&top);
				let fg = colorizer.fg(&bottom);

				writeln!(writer, "{}{}▄", fg, bg)?;
			}
        }

        Ok(())
    }
}
