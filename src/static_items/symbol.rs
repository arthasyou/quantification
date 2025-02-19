use std::sync::LazyLock;

static SYMBOLS: LazyLock<Vec<String>> = LazyLock::new(init_symbols);

fn init_symbols() -> Vec<String> {
    let base_symbols = vec![
        "ada", "crv", "doge", "dot", "hbar", "om", "xlm", "xrp", "sui", "wif", "render", "neiro",
        "pnut", "act", "ltc", "trx", "bnb", "wld", "fil",
    ];
    let symbols: Vec<String> = base_symbols.iter().map(|s| format!("{}usdt", s)).collect();
    symbols
}

pub fn get_symbols() -> &'static Vec<String> {
    &SYMBOLS
}
