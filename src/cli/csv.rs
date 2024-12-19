use clap::Parser;
use std::{fmt, str::FromStr};

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = super::check_input, help = "Input csv file")]
    pub input: String,
    #[arg(short, long, help = "Output file")]
    pub output: Option<String>,
    #[arg(short = 'f', long, default_value = "json", help = "Output format")]
    pub format: OutputFormat,
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
