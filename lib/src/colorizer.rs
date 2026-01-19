//! Color generation module

use alloc::string::String;
use image::{Pixel, Rgb};

/// Converts a pixel to a string representation (like ANSI's 24-bit and 8-bit color encoding)
pub trait Colorizer<T: Pixel> {
    /// Converts a pixel to a string that controls the color of the character being printed
    fn fg(&self, pixel: &T) -> String;

    /// Converts a pixel to a string that controls the color of the character's background
    fn bg(&self, pixel: &T) -> String;
}

/// Returns an empty string no matter what pixel is provided
/// ```
/// use ascii_img2::prelude::*;
/// let color = image::Rgb::from([255, 0, 255]);
/// assert_eq!(NullColorizer.fg(&color), "");
/// ```
pub struct NullColorizer;

impl<T: Pixel> Colorizer<T> for NullColorizer {
    fn fg(&self, _pixel: &T) -> String {
        String::new()
    }
    fn bg(&self, _pixel: &T) -> String {
        String::new()
    }
}

/// ANSI color encoding helpers
mod ansi {
    use alloc::{format, string::String};
    //pub const RESET: &str = "\x1b[0m";

    #[inline]
    #[must_use]
    pub fn fg_rgb(r: u8, g: u8, b: u8) -> String {
        format!("\x1b[38;2;{r};{g};{b}m")
    }

    #[inline]
    #[must_use]
    pub fn bg_rgb(r: u8, g: u8, b: u8) -> String {
        format!("\x1b[48;2;{r};{g};{b}m")
    }

    #[must_use]
    const fn rgb_to_256(r: u8, g: u8, b: u8) -> u8 {
        // Check for grayscale first
        if r == g && g == b {
            if r > 253 {
                return 255;
            }
            // convert grayscale to 0-23 range
            let grayscale = (r as u16 * 24 / 255) as u8;
            assert!(grayscale <= 24);
            return 232 + grayscale;
        }

        // Convert RGB to 0–5 range
        let r = (r as u16 * 5 / 255) as u8;
        let g = (g as u16 * 5 / 255) as u8;
        let b = (b as u16 * 5 / 255) as u8;

        assert!(r <= 5);
        assert!(g <= 5);
        assert!(b <= 5);

        // 16 is the start of the 6×6×6 color cube
        16 + (36 * r) + (6 * g) + b
    }

    #[inline]
    #[must_use]
    pub fn fg_256(r: u8, g: u8, b: u8) -> String {
        format!("\x1b[38;5;{}m", rgb_to_256(r, g, b))
    }

    #[inline]
    #[must_use]
    pub fn bg_256(r: u8, g: u8, b: u8) -> String {
        format!("\x1b[48;5;{}m", rgb_to_256(r, g, b))
    }
}

pub struct AnsiRgbColorizer;

impl Colorizer<Rgb<u8>> for AnsiRgbColorizer {
    fn fg(&self, pixel: &Rgb<u8>) -> String {
        ansi::fg_rgb(pixel[0], pixel[1], pixel[2])
    }
    fn bg(&self, pixel: &Rgb<u8>) -> String {
        ansi::bg_rgb(pixel[0], pixel[1], pixel[2])
    }
}

pub struct Ansi256Colorizer;

impl Colorizer<Rgb<u8>> for Ansi256Colorizer {
    fn fg(&self, pixel: &Rgb<u8>) -> String {
        ansi::fg_256(pixel[0], pixel[1], pixel[2])
    }
    fn bg(&self, pixel: &Rgb<u8>) -> String {
        ansi::bg_256(pixel[0], pixel[1], pixel[2])
    }
}
