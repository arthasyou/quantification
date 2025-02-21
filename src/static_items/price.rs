use super::symbol::get_symbols;
use crate::error::{Error, Result};
use serde::Deserialize;
use std::{
    collections::HashMap,
    sync::{Arc, LazyLock},
};
use tokio::sync::Mutex;

#[derive(Debug, Deserialize, Clone)]
pub struct Price {
    pub buy: String,
    pub sell: String,
}

impl Price {
    pub fn new(book: (String, String)) -> Self {
        Self {
            buy: book.0,
            sell: book.1,
        }
    }
}

static PRICE: LazyLock<Arc<HashMap<String, Mutex<(String, String)>>>> = LazyLock::new(init_price);

fn init_price() -> Arc<HashMap<String, Mutex<(String, String)>>> {
    let symbols = get_symbols();
    // let r = get_quantity_precision(symbols).await.unwrap();
    let map = symbols
        .iter()
        .map(|symbol| {
            (
                symbol.clone(),
                Mutex::new(("0".to_string(), "0".to_string())),
            )
        })
        .collect::<HashMap<_, _>>();

    Arc::new(map)
}
fn get_price() -> Arc<HashMap<String, Mutex<(String, String)>>> {
    PRICE.clone()
}

// pub async fn get_all_price() -> HashMap<String, Price> {
//     let mut result = HashMap::new();
//     for (symbol, mutex) in get_price().iter() {
//         let book = mutex.lock().await;
//         result.insert(symbol.clone(), Price::new(book.clone()));
//     }
//     result
// }

pub async fn get_symbol_price(symbol: &str) -> Result<Price> {
    if let Some(mutex) = get_price().get(symbol) {
        let book = mutex.lock().await;
        Ok(Price::new(book.clone()))
    } else {
        Err(Error::ErrorMessage(format!("Symbol {} not found", symbol)))
    }
}

pub async fn update_symbol_price(symbol: &str, book_price: (String, String)) {
    if let Some(mutex) = get_price().get(symbol) {
        let mut book = mutex.lock().await;
        *book = book_price;
    } else {
        eprintln!("failed symbol: {:?}", symbol);
    }
}
