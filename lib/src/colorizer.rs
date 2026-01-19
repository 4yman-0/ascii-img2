use image::{Pixel, Rgb};

pub trait Colorizer<T: Pixel> {
    fn fg(&self, pixel: &T) -> String;
    fn bg(&self, pixel: &T) -> String;
}

pub struct NullColorizer;

impl<T: Pixel> Colorizer<T> for NullColorizer {
    fn fg(&self, _pixel: &T) -> String {
        "".to_string()
    }
    fn bg(&self, _pixel: &T) -> String {
        "".to_string()
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

        // Convert RGB components to 0–5 range
        let r = (r as u16 * 5 / 255) as u8;
        let g = (g as u16 * 5 / 255) as u8;
        let b = (b as u16 * 5 / 255) as u8;

		assert!(r <= 5);
		assert!(g <= 5);
		assert!(b <= 5);

        // 16 is the start of the 6×6×6 color cube
        16 + (36 * r) + (6 * g) + b
    }

    pub fn fg_256(r: u8, g: u8, b: u8) -> String {
        format!("\x1b[38;5;{}m", rgb_to_256(r, g, b))
    }

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
