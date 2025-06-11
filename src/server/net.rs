use crate::server::command::Command;
use crate::server::store::Store;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

pub async fn run() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    let store = Arc::new(Store::new());
    loop {
        let (socket, _) = listener.accept().await?;
        let store = Arc::clone(&store);
        tokio::spawn(async move {
            let (reader, mut writer) = socket.into_split();
            let mut reader = BufReader::new(reader);
            let mut lines = String::new();
            while let Ok(bytes) = reader.read_line(&mut lines).await {
                if bytes == 0 {
                    break;
                }
                match Command::parse(&lines) {
                    Ok(command) => {
                        let response = command.execute(&store).await;
                        writer.write_all(response.as_bytes()).await.unwrap();
                        writer.write_all(b"\r\n").await.unwrap();
                    }
                    Err(err) => {
                        writer
                            .write_all(format!("ERROR: {}\r\n", err).as_bytes())
                            .await
                            .unwrap();
                    }
                }
                lines.clear();
            }
        });
    }
}
