use std::sync::LazyLock;

static SYMBOLS: LazyLock<Vec<String>> = LazyLock::new(init_symbols);

fn init_symbols() -> Vec<String> {
    let base_symbols = vec![
        "btc", "eth", "xrp", "sol", "bnb", "kaito", "ltc", "doge", "bera", "sui", "ada", "trx",
        "link", "apt", "avax", "om", "fil", "xlm", "cake", "tao", "ton", "dot", "uni", "aave",
        "wld", "etc", "hbar",
    ];
    // let base_symbols = vec!["xlm"];

    let symbols: Vec<String> = base_symbols.iter().map(|s| format!("{}usdt", s)).collect();
    symbols
}

pub fn get_symbols() -> &'static Vec<String> {
    &SYMBOLS
}
