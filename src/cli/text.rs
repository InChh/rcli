use crate::cli::check_input;
use crate::process::text::{generate_key, process_sign, process_verify, Key};
use crate::CmdExecutor;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
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

impl CmdExecutor for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let signed = process_sign(&self.input, &self.key, self.format)?;
        println!("Signature: {}", signed);
        Ok(())
    }
}

impl CmdExecutor for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let verified = process_verify(&self.input, &self.key, &self.sig, self.format)?;
        match verified {
            true => {
                println!("Signature verified");
            }
            false => {
                eprintln!("Signature not verified");
            }
        }
        Ok(())
    }
}

impl CmdExecutor for TextGenerateKeyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = generate_key(self.format)?;
        match key {
            Key::Symmetric { key } => {
                tokio::fs::write(self.output.join("blake3.key"), key).await?;
            }
            Key::Asymmetric { public, secret } => {
                tokio::fs::write(self.output.join("pk.pem"), public).await?;
                tokio::fs::write(self.output.join("sk.pem"), secret).await?;
            }
        }
        println!("Key generated to {:?}", self.output);
        Ok(())
    }
}
