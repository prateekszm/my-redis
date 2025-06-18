pub mod cli;
pub mod net;

use crate::client::cli::Cli;
use anyhow::Result;
use clap::Parser;

pub async fn run() -> Result<()> {
    let args = Cli::parse();
    net::run(args).await
}
