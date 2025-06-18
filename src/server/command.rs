use crate::server::store::{Store, Value};

pub fn main() {
    println!("hello world");
}

pub enum Command {
    Ping,
    Set {
        key: String,
        value: Value,
        exp_time_in_millis: Option<u128>,
    },
    ByteSet {
        key: String,
        value: Value,
        exp_time_in_millis: Option<u128>,
    },
    ISet {
        key: String,
        value: Value,
        exp_time_in_millis: Option<u128>,
    },
    BSet {
        key: String,
        value: Value,
        exp_time_in_millis: Option<u128>,
    },
    Get {
        key: String,
    },
    Del {
        key: String,
    },
    Expire {
        key: String,
        millis: u64,
    },
    Ttl {
        key: String,
    },
}

impl Command {
    pub fn parse(input: &str) -> Result<Self, String> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        match parts.as_slice() {
            ["PING"] => Ok(Command::Ping),
            ["SET", key, value] => Ok(Command::Set {
                key: key.to_string(),
                value: Value::String(value.to_string()),
                exp_time_in_millis: None,
            }),
            ["SET", key, value, "EX", exp_time] => {
                let exp_time_in_millis = exp_time
                    .parse::<u128>()
                    .map_err(|_| format!("invalid expiration time: {}", exp_time))?;
                Ok(Command::Set {
                    key: key.to_string(),
                    value: Value::String(value.to_string()),
                    exp_time_in_millis: Some(exp_time_in_millis),
                })
            }
            ["BYTESET", key, value] => Ok(Command::ByteSet {
                key: key.to_string(),
                value: Value::Bytes(value.as_bytes().to_vec()),
                exp_time_in_millis: None,
            }),
            ["BYTESET", key, value, "EX", exp_time] => {
                let exp_time_in_millis = exp_time
                    .parse::<u128>()
                    .map_err(|_| format!("invalid expiration time: {}", exp_time))?;
                Ok(Command::ByteSet {
                    key: key.to_string(),
                    value: Value::Bytes(value.as_bytes().to_vec()),
                    exp_time_in_millis: Some(exp_time_in_millis),
                })
            }
            ["ISET", key, value] => Ok(Command::ISet {
                key: key.to_string(),
                value: Value::Integer(
                    value
                        .parse()
                        .map_err(|_| format!("invalid integer value: {}", value))?,
                ),
                exp_time_in_millis: None,
            }),
            ["BSET", key, value] => Ok(Command::BSet {
                key: key.to_string(),
                value: Value::Bool(
                    value
                        .parse()
                        .map_err(|_| format!("invalid boolean value: {}", value))?,
                ),
                exp_time_in_millis: None,
            }),
            ["BSET", key, value, "EX", exp_time] => {
                let exp_time_in_millis = exp_time
                    .parse::<u128>()
                    .map_err(|_| format!("invalid expiration time: {}", exp_time))?;
                Ok(Command::BSet {
                    key: key.to_string(),
                    value: Value::Bool(
                        value
                            .parse()
                            .map_err(|_| format!("invalid boolean value: {}", value))?,
                    ),
                    exp_time_in_millis: Some(exp_time_in_millis),
                })
            }
            ["DEL", key] => Ok(Command::Del {
                key: key.to_string(),
            }),
            ["GET", key] => Ok(Command::Get {
                key: key.to_string(),
            }),
            ["EXPIRE", key, seconds] => {
                let millis: u64 = seconds
                    .parse()
                    .map_err(|_| format!("invalid seconds: {}", seconds))?;
                Ok(Command::Expire {
                    key: key.to_string(),
                    millis,
                })
            }
            ["TTL", key] => Ok(Command::Ttl {
                key: key.to_string(),
            }),
            _ => Err(format!("invalid command: {}", input)),
        }
    }

    pub async fn execute(&self, store: &Store) -> String {
        match self {
            Command::Ping => "PONG DEEZ NUTS".to_string(),
            Command::Set {
                key,
                value,
                exp_time_in_millis,
            } => {
                store
                    .set(
                        key.clone(),
                        value.clone(),
                        exp_time_in_millis.map(|exp| exp + store.current_time_millis()),
                    )
                    .await;
                "OK".to_string()
            }
            Command::ByteSet {
                key,
                value,
                exp_time_in_millis,
            } => {
                store
                    .set(
                        key.clone(),
                        value.clone(),
                        exp_time_in_millis.map(|exp| exp + store.current_time_millis()),
                    )
                    .await;
                "OK".to_string()
            }
            Command::ISet {
                key,
                value,
                exp_time_in_millis,
            } => {
                store
                    .set(
                        key.clone(),
                        value.clone(),
                        exp_time_in_millis.map(|exp| exp + store.current_time_millis()),
                    )
                    .await;
                "OK".to_string()
            }
            Command::BSet {
                key,
                value,
                exp_time_in_millis,
            } => {
                store
                    .set(
                        key.clone(),
                        value.clone(),
                        exp_time_in_millis.map(|exp| exp + store.current_time_millis()),
                    )
                    .await;
                "OK".to_string()
            }
            Command::Get { key } => {
                let value = store.get(key).await;
                match value {
                    Some(v) => match v {
                        Value::String(s) => s,
                        Value::Bytes(b) => String::from_utf8_lossy(&b).to_string(),
                        Value::Integer(i) => i.to_string(),
                        Value::Bool(b) => b.to_string(),
                    },
                    None => "nil".to_string(),
                }
            }
            Command::Del { key } => {
                let value = store.delete(key).await;
                if value.is_none() {
                    return "nil".to_string();
                }
                "OK".to_string()
            }
            Command::Expire { key, millis } => {
                let expiration_time = store.current_time_millis() + (*millis as u128);
                let is_sucess = store.set_expiration(key, expiration_time).await;
                if is_sucess.ok().unwrap() == true {
                    "Ok".to_string()
                } else {
                    "nil".to_string()
                }
            }
            Command::Ttl { key } => {
                if let Some(ttl) = store.get_time_to_live(key) {
                    ttl.to_string()
                } else {
                    "nil".to_string()
                }
            }
        }
    }
}
