use std::{fmt, str::FromStr};

use clap::Parser;

// rcli csv -i ./assets/juventus.csv -o .output.json --headers
#[derive(Debug, Parser)]
#[command(name="rcli", author, version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub command: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(about = "Show csv or convert csv to other formats")]
    Csv(CsvOpts),
    #[command(about = "Generate a custom password")]
    GenPass(GenPassOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = check_input)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(long, value_parser = OutputFormat::from_str, default_value_t = OutputFormat::Json)]
    pub format: OutputFormat,
    #[arg(long)]
    pub headers: bool,
}

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    /// Length of the generated password
    #[arg(short, long, default_value_t = 12)]
    pub length: u8,
    /// Include lowercase letters (a-z)
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,
    /// Include uppercase letters (A-Z)
    #[arg(long, default_value_t = true)]
    pub uppercase: bool,
    /// Include numbers (0-9)
    #[arg(long, default_value_t = true)]
    pub numbers: bool,
    /// Include special characters (!@#$%^&*()_+-=[]{}|;:,.<>?)
    #[arg(long, default_value_t = true)]
    pub special: bool,
}

#[derive(Debug, Copy, Clone)]
pub enum OutputFormat {
    Json,
    Yaml,
}

impl From<OutputFormat> for &str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(format!("Not supported format: {}", s)),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str((*self).into())
    }
}

fn check_input(s: &str) -> Result<String, &'static str> {
    if std::path::Path::new(s).exists() {
        Ok(s.to_string())
    } else {
        Err("Input file not exists")
    }
}
