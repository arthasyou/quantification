use std::{
    collections::HashMap,
    sync::{Arc, LazyLock},
};
use tokio::sync::Mutex;

use super::symbol::get_symbols;

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

pub fn get_price() -> Arc<HashMap<String, Mutex<(String, String)>>> {
    PRICE.clone()
}
