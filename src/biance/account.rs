use crate::error::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountInfo {
    #[serde(rename = "makerCommission")]
    maker_commission: i32,
    #[serde(rename = "takerCommission")]
    taker_commission: i32,
    #[serde(rename = "buyerCommission")]
    buyer_commission: i32,
    #[serde(rename = "sellerCommission")]
    seller_commission: i32,
    #[serde(rename = "canTrade")]
    can_trade: bool,
    #[serde(rename = "canWithdraw")]
    can_withdraw: bool,
    #[serde(rename = "canDeposit")]
    can_deposit: bool,
    // 添加其他需要的字段
}

#[allow(dead_code)]
pub async fn get_account() -> Result<AccountInfo> {
    let endpoint = format!("{}/fapi/v3/balance", super::BASE_URL);

    // 获取当前时间戳
    let timestamp = super::create_timestamp();

    // 准备查询字符串并生成签名
    let query_string = format!("timestamp={}", timestamp);
    let signature = super::create_signature(&super::API_SECRET, &query_string);

    // 完整请求 URL，包含签名
    let url = format!("{}?{}&signature={}", endpoint, query_string, signature);

    // 调用 get_request 发起请求并解析为 AccountInfo
    super::request(&url, Method::GET, &super::API_KEY).await
}
