use ascii_img2::prelude::*;
use clap::{Parser, ValueEnum};
use image::{open, GenericImageView as _};

mod preprocessor;
use preprocessor::{Preprocessor as _, *};

// TODO: this does not apply to all monospace fonts
const CHARACTER_ASPECT_RATIO: u32 = 2;

#[derive(Clone, Default, ValueEnum)]
enum GeneratorEnum {
    #[default]
    Charset,
    HalfBlock,
}

#[derive(Clone, Default, ValueEnum)]
enum ColorizerEnum {
    #[default]
    Null,
    AnsiRgb,
    Ansi256,
}

#[derive(Clone, Default, ValueEnum)]
enum PreprocessorEnum {
    #[default]
    Basic,
    Null,
}

#[derive(Parser)]
struct Cli {
    pub path: std::path::PathBuf,

    #[arg(short, long)]
    pub generator: Option<GeneratorEnum>,

    #[arg(short, long)]
    pub colorizer: Option<ColorizerEnum>,

    #[arg(short, long)]
    pub preprocessor: Option<PreprocessorEnum>,

    #[arg(long)]
    pub charset: Option<String>,

    #[arg(long)]
    pub width: Option<u32>,

    #[arg(long)]
    pub height: Option<u32>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let image = open(cli.path)?;

    let process = {
        let dimensions = match (cli.width, cli.height) {
            (Some(width), Some(height)) => (width, height),
            (Some(width), None) => (width, width / CHARACTER_ASPECT_RATIO),
            (None, Some(height)) => (height * CHARACTER_ASPECT_RATIO, height),
            (None, None) => image.dimensions(),
        };

        match cli.preprocessor.unwrap_or_default() {
            PreprocessorEnum::Basic => BasicPreprocessor { dimensions }.process(&image),
            PreprocessorEnum::Null => NullPreprocessor.process(&image),
        }
    };

    let charset = LinearCharset::new(
        cli.charset
            .map_or_else(|| vec![' ', ';', '#'], |v| v.chars().collect()),
    );

    let colorizer: Box<dyn Colorizer<image::Rgb<u8>>> = match cli.colorizer.unwrap_or_default() {
        ColorizerEnum::Null => Box::new(NullColorizer),
        ColorizerEnum::AnsiRgb => Box::new(AnsiRgbColorizer),
        ColorizerEnum::Ansi256 => Box::new(Ansi256Colorizer),
    };

    let grid = match cli.generator.unwrap_or_default() {
        GeneratorEnum::Charset => {
            CharsetGenerator.generate(&process.into(), &charset, colorizer.as_ref())?
        }
        GeneratorEnum::HalfBlock => {
            HalfBlockGenerator.generate(&process.into(), &charset, colorizer.as_ref())?
        }
    };

    for line in grid {
        println!("{line}");
    }

    Ok(())
}
