pub mod cli;
pub mod net;

use anyhow::Result;
use clap::Parser;
use crate::client::cli::Cli;

pub async fn run() -> Result<()> {
    let args = Cli::parse();
    net::run(args).await
}