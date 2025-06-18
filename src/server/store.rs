use dashmap::DashMap;
use log::info;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Bytes(Vec<u8>),
    Integer(i64),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub struct ValueEntry {
    pub value: Value,
    expiration: Option<u128>,
}

#[derive(Clone)]
pub struct Store {
    data: Arc<DashMap<Arc<str>, ValueEntry>>,
    expiration_map: Arc<RwLock<BTreeMap<u128, Vec<Arc<str>>>>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
            expiration_map: Arc::new(RwLock::new(BTreeMap::new())),
            //todo! start a background task to clean up expired entries
        }
    }
    pub fn current_time_millis(&self) -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }

    async fn update_expiration(&self, expiration: Option<u128>, key: Arc<str>) {
        if let Some(expiration) = expiration {
            info!("Setting expiration for key: {} at {}", key, expiration);
            let mut expiration_map = self.expiration_map.write().await;
            expiration_map
                .entry(expiration)
                .or_insert(Vec::new())
                .push(key.clone());
        };
    }

    pub async fn set(&self, key: impl Into<Arc<str>>, value: Value, expiration_ms: Option<u128>) {
        let key: Arc<str> = key.into();
        let expiration = expiration_ms.map(|ms| self.current_time_millis() + ms);

        match self.data.get_mut(&key) {
            Some(mut entry) => {
                entry.value = value;
                entry.expiration = expiration;
            }
            None => {
                self.data
                    .insert(key.clone(), ValueEntry { value, expiration });
            }
        }
        self.update_expiration(expiration, key).await;
    }

    async fn remove_from_expiration_map(&self, expiration: u128, key: &str) {
        let mut expiration_map = self.expiration_map.write().await;
        if let Some(keys) = expiration_map.get_mut(&expiration) {
            keys.retain(|k| k.as_ref() != key);
            // If no keys are left for this expiration, remove the entry
            // to prevent memory leaks.
            if keys.is_empty() {
                expiration_map.remove(&expiration);
            }
        }
    }

    pub async fn get(&self, key: &str) -> Option<Value> {
        let entry = self.data.get(key)?;
        if let Some(expiration) = entry.expiration {
            let now_as_millis = Instant::now().elapsed().as_millis();
            if expiration <= now_as_millis {
                self.remove_from_expiration_map(expiration, key).await;
                self.data.remove(key);
                info!("Key {} has expired and has been removed", key);
                return None;
            }
        }
        Some(entry.value.clone())
    }

    pub async fn delete(&self, key: &str) -> Option<Value> {
        let entry_val = self.data.remove(key);
        match entry_val {
            Some(entry) => {
                let exp_key = entry.1.expiration;
                if let Some(exp_key) = exp_key {
                    let mut exp_map = self.expiration_map.write().await;
                    if let Some(keys) = exp_map.get_mut(&exp_key) {
                        keys.retain(|k| k.as_ref() != key);
                        if keys.is_empty() {
                            exp_map.remove(&exp_key);
                        }
                    }
                }
                Some(entry.1.value)
            }
            None => todo!(),
        }
    }

    pub async fn set_expiration(&self, key: &str, expiration_ms: u128) -> Result<bool, String> {
        let key: Arc<str> = Arc::from(key);
        if let Some(value) = self.data.get_mut(&key) {
            if let Some(curr_exp_ms) = value.expiration {
                // check the curr_exp_ms present in expiration_map and then remove the key from that map
                let mut expiration_map = self.expiration_map.write().await;
                if let Some(keys) = expiration_map.get_mut(&curr_exp_ms) {
                    keys.retain(|k| k.as_ref() != key.as_ref());
                    if keys.is_empty() {
                        expiration_map.remove(&curr_exp_ms);
                    }
                }
                if let Some(keys) = expiration_map.get_mut(&expiration_ms) {
                    keys.push(Arc::clone(&key));
                }
            }
            Ok(true)
        } else {
            let mut exp_map = self.expiration_map.write().await;
            exp_map.insert(expiration_ms, vec![Arc::clone(&key)]);
            Ok(true)
        }
    }

    pub fn get_time_to_live(&self, key: &str) -> Option<u128> {
        let entry = self.data.get(key);
        if let Some(entry) = entry {
            entry.expiration.clone()
        } else {
            None
        }
    }
}
