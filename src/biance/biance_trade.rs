use crate::{error::Result, models::biance_model::Risk};
use reqwest::Method;

pub async fn get_biance_risk(key: &str, secret: &str) -> Result<Vec<Risk>> {
    let endpoint = format!("{}/fapi/v3/positionRisk", super::BASE_URL);

    // 获取当前时间戳
    let timestamp = super::create_timestamp();

    // 准备查询字符串并生成签名
    let query_string = format!("timestamp={}", timestamp);
    let signature = super::create_signature(secret, &query_string);

    // 完整请求 URL，包含签名
    let url = format!("{}?{}&signature={}", endpoint, query_string, signature);

    // 调用 get_request 发起请求并解析为 AccountInfo
    super::request(&url, Method::GET, key).await
}
