use crate::cli::check_input;
use clap::Parser;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "Sign a text")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signed text")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a key for signing")]
    GenerateKey(TextGenerateKeyOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = check_input, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = check_input, default_value = "-")]
    pub key: String,
    #[arg(long, value_parser= TextSignFormat::from_str, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = check_input, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = check_input, default_value = "-")]
    pub key: String,
    #[arg(long, value_parser = TextSignFormat::from_str, default_value = "blake3")]
    pub format: TextSignFormat,
    #[arg(short, long)]
    pub sig: String,
}

#[derive(Debug, Copy, Clone)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

#[derive(Debug, Parser)]
pub struct TextGenerateKeyOpts {
    #[arg(long, value_parser = TextSignFormat::from_str, default_value = "blake3")]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = check_path)]
    pub output: PathBuf,
}

fn check_path(s: &str) -> Result<PathBuf, &'static str> {
    let path = PathBuf::from(s);
    if path.exists() && path.is_dir() {
        Ok(path)
    } else {
        Err("Directory does not exist or is not a directory")
    }
}

impl FromStr for TextSignFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(Self::Blake3),
            "ed25519" => Ok(Self::Ed25519),
            _ => Err("Invalid text sign format"),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(f: TextSignFormat) -> Self {
        match f {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextSignFormat::Blake3 => {
                write!(f, "blake3")
            }
            TextSignFormat::Ed25519 => {
                write!(f, "ed25519")
            }
        }
    }
}
