use clap::Parser;
use rcli::{process_csv, Opts};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.command {
        rcli::SubCommand::Csv(opts) => {
            let output = if let Some(ref output) = opts.output {
                output
            } else {
                &format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?
        }
    }
    Ok(())
}
