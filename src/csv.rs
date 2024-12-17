use serde_json::Value;

use crate::OutputFormat;

pub fn process_csv(input: &str, output: &str, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = csv::Reader::from_path(input)?;
    let headers = reader.headers()?.clone();
    let mut ret = Vec::with_capacity(128);
    for result in reader.records() {
        let record = result?;
        let value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    std::fs::write(output, content)?;

    Ok(())
}
