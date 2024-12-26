use std::str::FromStr;

use crate::process::base64::{process_decode, process_encode};
use crate::CmdExecutor;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(about = "Encode input as base64")]
    Encode(Base64EncodeOpts),
    #[command(about = "Decode base64 input")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = super::check_input, default_value = "-", help = "Input string to encode"
    )]
    pub input: String,
    #[arg(long, value_parser = Base64Format::from_str, default_value = "standard", help = "Base64 format to use"
    )]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = super::check_input, default_value = "-", help = "Base64 string to decode"
    )]
    pub input: String,
    #[arg(long, value_parser = Base64Format::from_str, default_value = "standard", help = "Base64 format to use"
    )]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

impl From<Base64Format> for &str {
    fn from(format: Base64Format) -> &'static str {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Not supported format: {}", s)),
        }
    }
}

impl CmdExecutor for Base64SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            Base64SubCommand::Encode(opts) => {
                let encoded = process_encode(&opts.input, opts.format)?;
                println!("{}", encoded);
            }
            Base64SubCommand::Decode(opts) => {
                let decoded = process_decode(&opts.input, opts.format)?;
                println!("{}", String::from_utf8_lossy(&decoded));
            }
        }
        Ok(())
    }
}
