use ascii_img2::{Preprocessor as _, *};
use clap::{Parser, ValueEnum};
use image::{GenericImageView as _, open};

const CHARACTER_ASPECT_RATIO: f32 = 2.2;

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
            (Some(width), None) => (width, (width as f32 / CHARACTER_ASPECT_RATIO) as u32),
            (None, Some(height)) => ((height as f32 * CHARACTER_ASPECT_RATIO) as u32, height),
            (None, None) => image.dimensions(),
        };

        match cli.preprocessor.unwrap_or_default() {
            PreprocessorEnum::Basic => BasicPreprocessor { dimensions }.process(&image),
            PreprocessorEnum::Null => NullPreprocessor.process(&image),
        }
    };

    let charset = LinearCharset::new(
        cli.charset
            .map(|v| v.chars().collect::<Vec<char>>())
            .unwrap_or_else(|| vec![' ', ';', '#']),
    );

    let colorizer: Box<dyn Colorizer> = match cli.colorizer.unwrap_or_default() {
    	ColorizerEnum::Null => Box::new(NullColorizer),
    	ColorizerEnum::AnsiRgb => Box::new(AnsiRgbColorizer),
    	ColorizerEnum::Ansi256 => Box::new(Ansi256Colorizer),
    };

    let grid = match cli.generator.unwrap_or_default() {
        GeneratorEnum::Charset => CharsetGenerator.generate(
        	&process.into(),
        	&charset,
        	colorizer.as_ref()
        )?,
        GeneratorEnum::HalfBlock => HalfBlockGenerator.generate(
        	&process.into(),
        	&charset,
        	colorizer.as_ref()
        )?,
    };

    for line in grid {
        println!("{line}");
    }

    Ok(())
}
