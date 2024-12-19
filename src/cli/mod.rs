use clap::Parser;

pub mod base64;
pub mod csv;
pub mod gen_pass;

#[derive(Debug, Parser)]
#[command(name="rcli", author, version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub command: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(about = "Show csv or convert csv to other formats")]
    Csv(csv::CsvOpts),
    #[command(about = "Generate a custom password")]
    GenPass(gen_pass::GenPassOpts),
    #[command(subcommand)]
    Base64(base64::Base64SubCommand),
}

pub fn check_input(s: &str) -> Result<String, &'static str> {
    if s == "-" || std::path::Path::new(s).exists() {
        Ok(s.to_string())
    } else {
        Err("Input file not exists")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_input() {
        assert_eq!(check_input("-"), Ok("-".to_string()));
        assert_eq!(check_input("*"), Err("Input file not exists"));
        assert_eq!(check_input("cargo.toml"), Ok("cargo.toml".to_string()));
        assert_eq!(
            check_input("nonexistent_file.txt"),
            Err("Input file not exists")
        );
    }
}
