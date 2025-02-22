use crate::{
    error::Result,
    models::biance_model::{ExchangeInfo, Leverage},
};
use reqwest::Method;

pub async fn change_leverage(
    symbol: &str,  // 交易对符号，例如 "BTCUSDT"
    leverage: u32, // 杠杆倍数，范围 1 到 125
) -> Result<Leverage> {
    let endpoint = format!("{}/fapi/v1/leverage", super::BASE_URL);

    // 获取当前时间戳
    let timestamp = super::create_timestamp();

    // 构建请求参数
    let query_string = format!(
        "symbol={}&leverage={}&timestamp={}",
        symbol, leverage, timestamp
    );

    // 生成签名
    let signature = super::create_signature(&super::API_SECRET, &query_string);
    let url = format!("{}?{}&signature={}", endpoint, query_string, signature);

    // 使用 post_request 发送请求
    let response = super::request::<Leverage>(&url, Method::POST, &super::API_KEY).await?;

    Ok(response)
}

pub async fn get_quantity_precision() -> Result<ExchangeInfo> {
    let endpoint = format!("{}/fapi/v1/exchangeInfo", super::BASE_URL);
    let response = super::request::<ExchangeInfo>(&endpoint, Method::GET, &super::API_KEY).await?;
    Ok(response)
}
