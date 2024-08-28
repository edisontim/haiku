mod actors;
mod subcommands;
mod types;
mod utils;

use tracing_subscriber::EnvFilter;

use clap::Parser;

use crate::subcommands::{Args, Subcommands};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("haiku=trace"))
        .with_file(true)
        .with_line_number(true)
        .init();

    let args = Args::parse();

    match args.command {
        Subcommands::Run(run) => run.execute().await,
        Subcommands::Build(build) => build.execute(),
    }
}
