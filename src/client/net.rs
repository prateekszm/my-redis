use crate::client::cli::{Cli, Command};
use anyhow::Result;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
pub async fn run(cli: Cli) -> Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6379").await?;
    let command = match cli.command {
        Command::Ping => "PING".to_string(),
        Command::Set { key, value } => format!("SET {} {}", key, value),
        Command::Get { key } => format!("GET {}", key),
    };
    println!("{:#?}", command);
    stream
        .write_all(format!("{}\r\n", command).as_bytes())
        .await?;
    let mut reader = BufReader::new(stream);
    let mut response = String::new();
    reader.read_line(&mut response).await?;

    println!("{}", response.trim());
    Ok(())
}
