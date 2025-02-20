use std::{
    collections::HashMap,
    sync::{Arc, LazyLock},
};

use serde::Deserialize;
use tokio::sync::Mutex;

use crate::biance::leverage::get_quantity_precision;

use super::symbol;

#[derive(Debug, Clone, Deserialize)]
pub struct Percision {
    pub value: u8,
}

static PERCISION: LazyLock<Arc<PercisionsManager>> = LazyLock::new(PercisionsManager::new);

pub struct PercisionsManager {
    keys: Mutex<HashMap<String, Percision>>,
}

impl PercisionsManager {
    // 创建新的 KeyManager，初始化 keys
    pub fn new() -> Arc<Self> {
        let map = HashMap::new(); // 初始化为空的 HashMa
        Arc::new(PercisionsManager {
            keys: Mutex::new(map),
        })
    }

    async fn init_percisions(&self) {
        let symbols = symbol::get_symbols();
        let response = get_quantity_precision().await.unwrap();
        for symbol in symbols {
            let symbol_uppercase = symbol.to_uppercase();
            if let Some(symbol_info) = response
                .symbols
                .iter()
                .find(|s| s.symbol == *symbol_uppercase)
            {
                self.keys.lock().await.insert(
                    symbol.to_string(),
                    Percision {
                        value: symbol_info.quantityPrecision,
                    },
                );
            }
        }
    }

    async fn get_symbol_percision(&self, symbol: &str) -> Option<u8> {
        let map = self.keys.lock().await;
        let key = map.get(symbol).cloned();
        match key {
            Some(key) => Some(key.value),
            None => None,
        }
    }
}

fn get_percisions_manager() -> Arc<PercisionsManager> {
    PERCISION.clone()
}

pub async fn init_percisions() {
    get_percisions_manager().init_percisions().await;
}

pub async fn get_symbol_percision(symbol: &str) -> Option<u8> {
    get_percisions_manager().get_symbol_percision(symbol).await
}
