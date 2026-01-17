use ascii_img2::{Preprocessor as _, *};
use clap::{Parser, ValueEnum};
use image::{GenericImageView as _, open};

const CHARACTER_ASPECT_RATIO: f32 = 2.2;

#[derive(Clone, Default, ValueEnum)]
enum Generator {
    #[default]
    Luminance,
    AnsiRgb,
    Ansi256,
    HalfBlock,
}

#[derive(Clone, Default, ValueEnum)]
enum Preprocessor {
    #[default]
    Basic,
    Null,
}

#[derive(Parser)]
struct Cli {
    pub path: std::path::PathBuf,

    #[arg(short, long)]
    pub generator: Option<Generator>,

    #[arg(short, long)]
    pub preprocessor: Option<Preprocessor>,

    #[arg(short, long)]
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
            (Some(width), None) => (width, (width as f32 / CHARACTER_ASPECT_RATIO) as u32),
            (None, Some(height)) => ((height as f32 * CHARACTER_ASPECT_RATIO) as u32, height),
            (None, None) => image.dimensions(),
        };

        match cli.preprocessor.unwrap_or_default() {
            Preprocessor::Basic => BasicPreprocessor { dimensions }.process(&image),
            Preprocessor::Null => NullPreprocessor.process(&image),
        }
    };

    let charset = LinearCharset::new(
        cli.charset
            .map(|v| v.chars().collect::<Vec<char>>())
            .unwrap_or_else(|| vec![' ', ';', '#']),
    );

    let grid = match cli.generator.unwrap_or_default() {
        Generator::Luminance => LuminanceGenerator.generate(&process.into(), &charset)?,
        Generator::AnsiRgb => AnsiRgbGenerator.generate(&process.into(), &charset)?,
        Generator::Ansi256 => Ansi256Generator.generate(&process.into(), &charset)?,
        Generator::HalfBlock => HalfBlockGenerator.generate(&process.into(), &charset)?,
    };

    for line in grid {
        println!("{line}");
    }

    Ok(())
}
