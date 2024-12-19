use crate::cli::base64::Base64Format;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use base64::{engine::general_purpose::STANDARD, Engine};
use std::io::Read;

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let content = match format {
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD.encode(&buf),
        Base64Format::Standard => STANDARD.encode(&buf),
    };

    println!("{}", content);

    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    let decoded = match format {
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD.decode(buf.trim())?,
        Base64Format::Standard => STANDARD.decode(buf.trim())?,
    };

    println!("{}", String::from_utf8(decoded)?);

    Ok(())
}

fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(input)?)
    };
    Ok(reader)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        process_encode(input, format).unwrap();
    }

    #[test]
    fn test_decode() {
        let input = "fixtures/b64.txt";
        let format = Base64Format::UrlSafe;
        process_decode(input, format).unwrap();
    }
}
