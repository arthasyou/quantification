use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::{PartialSchema, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Risk {
    pub symbol: String,

    #[serde(rename = "positionSide")]
    pub position_side: String, // 持仓方向

    #[serde(rename = "positionAmt")]
    #[schema(schema_with = String::schema)]
    pub position_amt: Decimal, // 头寸数量，正数为多，负数为空

    #[serde(rename = "entryPrice")]
    #[schema(schema_with = String::schema)]
    pub entry_price: Decimal, // 开仓均价

    #[serde(rename = "breakEvenPrice")]
    #[schema(schema_with = String::schema)]
    pub break_even_price: Decimal, // 盈亏平衡价

    #[serde(rename = "markPrice")]
    #[schema(schema_with = String::schema)]
    pub mark_price: Decimal, // 当前标记价格

    #[serde(rename = "unRealizedProfit")]
    #[schema(schema_with = String::schema)]
    pub unrealized_profit: Decimal, // 持仓未实现盈亏

    #[serde(rename = "liquidationPrice")]
    #[schema(schema_with = String::schema)]
    pub liquidation_price: Decimal, // 强平价格

    #[serde(rename = "isolatedMargin")]
    #[schema(schema_with = String::schema)]
    pub isolated_margin: Decimal, // 逐仓保证金

    #[schema(schema_with = String::schema)]
    pub notional: Decimal, // 头寸名义价值

    #[serde(rename = "marginAsset")]
    pub margin_asset: String, // 保证金资产类型

    #[serde(rename = "isolatedWallet")]
    #[schema(schema_with = String::schema)]
    pub isolated_wallet: Decimal, // 逐仓钱包余额

    #[serde(rename = "initialMargin")]
    #[schema(schema_with = String::schema)]
    pub initial_margin: Decimal, // 初始保证金

    #[serde(rename = "maintMargin")]
    #[schema(schema_with = String::schema)]
    pub maint_margin: Decimal, // 维持保证金

    #[serde(rename = "positionInitialMargin")]
    #[schema(schema_with = String::schema)]
    pub position_initial_margin: Decimal, // 持仓初始保证金

    #[serde(rename = "openOrderInitialMargin")]
    #[schema(schema_with = String::schema)]
    pub open_order_initial_margin: Decimal, // 开单初始保证金

    pub adl: i32, // ADL

    #[serde(rename = "bidNotional")]
    #[schema(schema_with = String::schema)]
    pub bid_notional: Decimal, // 买单名义价值

    #[serde(rename = "askNotional")]
    #[schema(schema_with = String::schema)]
    pub ask_notional: Decimal, // 卖单名义价值

    #[serde(rename = "updateTime")]
    pub update_time: i64, // 更新时间
}

#[derive(Deserialize, Serialize)]
pub struct TradeRecord {
    buyer: bool,        // 是否是买方
    commission: String, // 手续费
    #[serde(rename = "commissionAsset")]
    commission_asset: String, // 手续费计价单位
    id: u64,            // 交易ID
    maker: bool,        // 是否是挂单方
    #[serde(rename = "orderId")]
    order_id: u64, // 订单编号
    price: String,      // 成交价
    qty: String,        // 成交量
    #[serde(rename = "quoteQty")]
    quote_qty: String, // 成交额
    #[serde(rename = "realizedPnl")]
    realized_pnl: String, // 实现盈亏
    side: String,       // 买卖方向
    #[serde(rename = "positionSide")]
    position_side: String, // 持仓方向
    symbol: String,     // 交易对
    time: u64,          // 时间
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BiannceOrder {
    #[serde(rename = "orderId")]
    pub order_id: u64,
    #[serde(rename = "avgPrice")]
    pub avg_price: String,
    #[serde(rename = "executedQty")]
    pub executed_qty: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActiveOrder {
    #[serde(rename = "orderId")]
    pub order_id: u64,
    #[serde(rename = "origQty")]
    orig_qty: String,
    price: String,
    #[serde(rename = "reduceOnly")]
    reduce_only: bool,
    side: String,
    #[serde(rename = "positionSide")]
    position_side: String,
    status: String,
    #[serde(rename = "stopPrice")]
    stop_price: String,
    symbol: String,
    #[serde(rename = "timeInForce")]
    time_in_force: String,
    #[serde(rename = "type")]
    order_type: String,
    #[serde(rename = "origType")]
    orig_type: String,
    #[serde(rename = "updateTime")]
    update_time: i64,
    #[serde(rename = "workingType")]
    working_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Leverage {
    pub leverage: u32,
    #[serde(rename = "maxNotionalValue")]
    pub max_notional_value: String,
    pub symbol: String,
}

#[derive(Deserialize)]
pub struct ExchangeInfo {
    pub symbols: Vec<SymbolInfo>,
}

#[derive(Deserialize, Debug)]
pub struct SymbolInfo {
    pub symbol: String,
    #[serde(rename = "quantityPrecision")]
    pub quantity_precision: u8,
}
