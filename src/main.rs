use clap::Parser;
use rcli::cli::Opts;
use rcli::CmdExecutor;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts = Opts::parse();
    opts.command.execute().await?;
    Ok(())
}
