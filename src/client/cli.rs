use clap::Parser;

#[derive(Parser)]
#[command(name = "redis-cli")]
#[command(version = "0.1")]
#[command(about = "Basic Redis client", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Parser)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Command {
    Ping,
    Set { key: String, value: String },
    Get { key: String },
}
