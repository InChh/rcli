use clap::Parser;
use rcli::cli::base64::Base64SubCommand;
use rcli::cli::http::HttpSubCommand;
use rcli::cli::text::TextSubCommand;
use rcli::cli::{Opts, SubCommand};
use rcli::process::base64::{process_decode, process_encode};
use rcli::process::csv::process_csv;
use rcli::process::gen_pass::generate_password;
use rcli::process::http::http_serve;
use rcli::process::text::{generate_key, process_sign, process_verify};
use zxcvbn::zxcvbn;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
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
                process_encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?;
            }
        },
        SubCommand::Text(subcommand) => match subcommand {
            TextSubCommand::Sign(opts) => {
                let signed = process_sign(&opts.input, &opts.key, opts.format)?;
                println!("Signature: {}", signed);
            }
            TextSubCommand::Verify(opts) => {
                let verified = process_verify(&opts.input, &opts.key, &opts.sig, opts.format)?;
                match verified {
                    true => {
                        println!("Signature verified");
                    }
                    false => {
                        eprintln!("Signature not verified");
                    }
                }
            }
            TextSubCommand::GenerateKey(opts) => {
                generate_key(opts.format, opts.output)?;
            }
        },
        SubCommand::Http(subcommand) => match subcommand {
            HttpSubCommand::Serve(opts) => {
                http_serve(opts.dir, opts.port).await?;
            }
        },
    }
    Ok(())
}
