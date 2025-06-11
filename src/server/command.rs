use crate::server::store::Store;

pub fn main() {
    println!("hello world");
}

pub enum Command {
    Ping,
    Set { key: String, value: String },
    Get { key: String },
}

impl Command {
    pub fn parse(input: &str) -> Result<Self, String> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        match parts.as_slice() {
            ["PING"] => Ok(Command::Ping),
            ["SET", key, value] => Ok(Command::Set {
                key: key.to_string(),
                value: value.to_string(),
            }),
            ["GET", key] => Ok(Command::Get {
                key: key.to_string(),
            }),
            _ => Err(format!("invalid command: {}", input)),
        }
    }

    pub async fn execute(&self, store: &Store) -> String {
        match self {
            Command::Ping => "PONG".to_string(),
            Command::Set { key, value } => {
                store.set(key.clone(), value.clone()).await;
                "OK".to_string()
            }
            Command::Get { key } => store.get(key).await.unwrap_or_else(|| "(nil)".to_owned()),
        }
    }
}
