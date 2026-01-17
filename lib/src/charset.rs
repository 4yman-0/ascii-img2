pub trait Charset: Send + Sync {
    fn map(&self, luminance: f32) -> char;
}

pub struct LinearCharset {
    chars: Vec<char>,
}

impl LinearCharset {
    pub fn new(chars: impl Into<Vec<char>>) -> Self {
        Self {
            chars: chars.into(),
        }
    }
}

impl Charset for LinearCharset {
    fn map(&self, luminance: f32) -> char {
        let index = (luminance.clamp(0.0, 1.0) * (self.chars.len() - 1) as f32).round() as usize;
        self.chars[index]
    }
}
