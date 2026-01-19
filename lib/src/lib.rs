//! ASCII art generation library

#![no_std]

extern crate alloc;
extern crate core;

pub mod charset;
pub mod colorizer;
pub mod error;
pub mod generator;

pub mod prelude {
    use super::*;
    pub use charset::*;
    pub use colorizer::*;
    pub use error::*;
    pub use generator::*;
}
