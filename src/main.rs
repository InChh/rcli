use clap::Parser;
use rcli::cli::base64::Base64SubCommand;
use rcli::cli::{Opts, SubCommand};
use rcli::process::csv::process_csv;
use rcli::process::gen_pass::generate_password;

use zxcvbn::zxcvbn;

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.command {
        SubCommand::Csv(opts) => {
            let output = if let Some(ref output) = opts.output {
                output
            } else {
                &format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?
        }
        SubCommand::GenPass(opts) => {
            let password = generate_password(
                opts.length,
                opts.lowercase,
                opts.uppercase,
                opts.numbers,
                opts.special,
            )?;
            eprintln!("Generated password: {}", password);

            let estimate = zxcvbn(&password, &[]);
            eprintln!("Password strength: {}", estimate.score());
        }
        SubCommand::Base64(subcommand) => match subcommand {
            Base64SubCommand::Encode(opts) => {
                rcli::process::base64::process_encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                rcli::process::base64::process_decode(&opts.input, opts.format)?;
            }
        },
    }
    Ok(())
}
