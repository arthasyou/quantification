use serde::Deserialize;

use crate::biance::order::get_biance_active_order;
use crate::error::{Error, Result};
use crate::models::biance_model::BiannceOrder;
use crate::{biance::order::create_biance_order, models::trade_model::CreatePositionRequest};

pub fn calculate_quantity(
    trade_request: &CreatePositionRequest,
    market_price: f64,
    precision: u8,
) -> String {
    // 确保市场价格有效，避免除以 0
    if market_price <= 0.0 {
        return "0.0".to_string();
    }

    // 计算可买数量
    let quantity = trade_request.margin * trade_request.leverage / market_price;

    // 动态格式化数量，保留指定的精度
    format!("{:.precision$}", quantity, precision = precision as usize)
}

pub async fn create_position_order(
    symbol: &str,
    side: &str,
    position_side: &str,
    quantity: &str,
    key: &str,
    secret: &str,
) -> Result<BiannceOrder> {
    let order_response = create_biance_order(
        symbol,
        side,
        position_side,
        "MARKET", // 假设使用市价单
        quantity, // 将数量格式化为字符串
        None,     // 市价单无需价格
        None,     // 此示例未设置止损价格
        key,
        secret,
    )
    .await;
    match order_response {
        Ok(order) => get_biance_active_order(symbol, order.orderId, key, secret).await,
        Err(e) => Err(Error::ErrorMessage(format!("Order failed: {}", e))),
    }
}

pub fn trim_trailing_zeros(input: &str) -> String {
    if input.contains('.') {
        // 如果包含小数点，去掉末尾的零
        input
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    } else {
        // 如果是整数，直接返回
        input.to_string()
    }
}

pub fn format_url(symbol: &str) -> String {
    // format!("wss://stream.binance.com:443/ws/{}@miniTicker", symbol)
    // format!("wss://stream.binance.com:443/ws/{}@trade", symbol)
    format!("wss://stream.binance.com:443/ws/{}@bookTicker", symbol)
}

#[derive(Deserialize, Debug, Clone)]
pub struct Book {
    pub a: String,
    pub b: String,
}

pub fn parse_trade_json(json_text: &str) -> Result<Book> {
    serde_json::from_str(json_text).map_err(Error::JsonError) // 使用 map_err 将 serde_json::Error 转换为 Error::JsonError
}
