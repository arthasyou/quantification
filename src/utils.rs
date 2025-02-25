use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::biance::biance_trade::get_biance_risk;

use crate::biance::order::{get_biance_active_order, get_biance_finished_order};
use crate::database::fee_db::db_create_fee;
use crate::error::{Error, Result};
use crate::models::biance_model::{BiannceOrder, Risk, TradeRecord};
use crate::models::fee_model::CreateFeeRequest;
use crate::static_items::user_info::get_agent_id;
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

    let leverage_f64 = trade_request.leverage as f64;
    // 计算可买数量
    let quantity = trade_request.margin * leverage_f64 / market_price;
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
        Ok(order) => get_biance_active_order(symbol, order.order_id, key, secret).await,
        Err(e) => Err(Error::ErrorMessage(format!("Order failed: {}", e))),
    }
}

pub async fn get_symbol_direction_quantity(
    symbol: &str,
    position_side: &str,
    key: &str,
    secret: &str,
) -> Result<String> {
    let risks = get_biance_risk(key, secret)
        .await
        .map_err(|_e| Error::ErrorMessage("Symbol not found".to_string()))?;

    let filtered_risks: Vec<Risk> = risks
        .into_iter()
        .filter(|risk| risk.symbol == symbol.to_uppercase() && risk.position_side == position_side)
        .collect();
    if filtered_risks.is_empty() {
        return Err(Error::ErrorMessage("Symbol not found".to_string()));
    }
    let amt = filtered_risks[0].position_amt;
    let quantity = amt.abs().to_string();
    Ok(quantity)
}

pub async fn close_position_order(
    user_id: &str,
    symbol: &str,
    side: &str,
    position_side: &str,
    key: &str,
    secret: &str,
) -> Result<Vec<TradeRecord>> {
    let quantity = get_symbol_direction_quantity(symbol, position_side, key, secret).await?;
    let order_response =
        create_position_order(symbol, side, position_side, &quantity, key, secret).await;
    match order_response {
        Ok(order) => match get_biance_finished_order(symbol, order.order_id, key, secret).await {
            Ok(active_order) => {
                // 计算佣金
                let commission = calculate_total_commission(&active_order);
                let agent_id = get_agent_id(user_id).await.unwrap_or("".to_string());
                let input = CreateFeeRequest {
                    user_id: user_id.to_string(),
                    agent_id,
                    amount: commission.to_f64().unwrap_or(0.0),
                };
                db_create_fee(input).await?;
                Ok(active_order)
            }
            Err(e) => Err(Error::ErrorMessage(format!(
                "Failed to get active order: {}",
                e
            ))),
        },
        Err(e) => Err(Error::ErrorMessage(format!("Order failed: {}", e))),
    }
}

// [TradeRecord { buyer: false, commission: "0.00507780", commission_asset: "USDT", id: 808126806, maker: false, order_id: 31186926487, price: "3.276", qty: "3.1", quote_qty: "10.1556", realized_pnl: "0", side: "SELL", position_side: "LONG", symbol: "FILUSDT", time: 1740391156270 }]

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

fn calculate_commission(amount_str: &str) -> Decimal {
    // 尝试将字符串解析为 Decimal 类型
    let amount: Decimal = amount_str.parse().unwrap_or(Decimal::ZERO);

    // 佣金计算：千分之5
    let commission = amount.abs() * Decimal::new(5, 3); // 5千分之一
    commission
}

pub fn calculate_total_commission(trade_records: &Vec<TradeRecord>) -> Decimal {
    trade_records
        .iter()
        .map(|trade_record| calculate_commission(&trade_record.realized_pnl))
        .sum()
}
