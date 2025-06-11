use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct Store {
    data: Arc<RwLock<HashMap<String, String>>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn set(&self, key: String, value: String) {
        let mut lock = self.data.write().unwrap();
        lock.insert(key, value);
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        let lock = self.data.read().unwrap();
        lock.get(key).cloned()
    }
}

pub enum Value {
    String(String),
    Vec(Vec<u8>),
    Integer(i64),
    Bool(bool),
    // .. add more
}