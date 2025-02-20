use crate::{
    error::{Error, Result},
    utils::create_position_order,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, LazyLock},
};
use tokio::sync::Mutex;
use utoipa::ToSchema;

use super::{strategy::Strategy, symbol::get_symbols};

#[derive(Debug, PartialEq, Deserialize, Serialize, ToSchema, Clone)]
pub enum Direction {
    Long,  // 做多
    Short, // 做空
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Long => write!(f, "Long"),
            Direction::Short => write!(f, "Short"),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Position {
    pub order_id: u64,
    pub user_id: String,
    pub stop_order: u64,
    pub symbol: String, // 货币或资产符号，表示此交易涉及的交易品种，如 "EUR/USD" 或 "AAPL"
    pub entry_price: f64, // 入场价格，交易开始时的初始价格
    pub stop_loss: f64, // 止损点位，如果当前价格达到该值，交易将自动平仓以限制损失
    highest_price: f64, // 记录历史最高价格，用于动态调整止损点和判断利润情况（做多时）
    lowest_price: f64,  // 记录历史最低价格，用于动态调整止损点和判断利润情况（做空时）
    pub direction: Direction, // 交易方向，标识是做多还是做空
    pub quantity: String,
    pub leverage: f64,
    pub strategy: Vec<Strategy>,
    pub is_closed: bool,
    api_key: String,
    api_secret: String,
}

impl Position {
    // 创建一个新的交易，自动设置止损为-5%（即95%）
    pub async fn new(
        order_id: u64,
        user_id: String,
        symbol: String,
        entry_price: f64,
        direction: Direction,
        quantity: String,
        leverage: f64,
        stop_loss_percent: f64,
        strategy: Vec<Strategy>,
        api_key: String,
        api_secret: String,
    ) -> Self {
        let stop_loss = calculate_stop_price(&direction, entry_price, leverage, stop_loss_percent);

        let stop_order_id = order_id;

        Self {
            user_id,
            order_id,
            stop_order: stop_order_id,
            symbol,
            entry_price,
            stop_loss,
            highest_price: entry_price, // 做多时初始为入场价
            lowest_price: entry_price,  // 做空时初始为入场价
            direction,
            quantity,
            leverage,
            strategy,
            is_closed: false,
            api_key,
            api_secret,
        }
    }

    // 更新价格并调整历史最高或最低价和止损
    pub async fn update_price(&mut self, book_price: (String, String)) {
        let price = match self.direction {
            Direction::Long => {
                let price = book_price.1;
                let price_f64: f64 = price.parse().unwrap();
                if price_f64 > self.highest_price {
                    self.highest_price = price_f64;
                    let profit_percentage =
                        (self.highest_price - self.entry_price) / self.entry_price;
                    self.update_stop_loss(profit_percentage, true).await;
                }
                price
            }
            Direction::Short => {
                let price = book_price.0;
                let price_f64: f64 = price.parse().unwrap();
                if price_f64 < self.lowest_price {
                    self.lowest_price = price_f64;
                    let profit_percentage =
                        (self.entry_price - self.lowest_price) / self.entry_price;
                    self.update_stop_loss(profit_percentage, false).await;
                }
                price
            }
        };
        self.check_exit_conditions(&price).await;
    }

    async fn update_stop_loss(&mut self, profit_percentage: f64, is_long: bool) {
        let new_stop_price = self.calculate_new_stop_loss(profit_percentage, is_long);

        if new_stop_price != self.stop_loss {
            self.stop_loss = new_stop_price;
        }
    }

    fn calculate_new_stop_loss(&mut self, profit_percentage: f64, is_long: bool) -> f64 {
        let actual_price_change_percentage = profit_percentage * self.leverage;

        let adjustment = get_strategy(actual_price_change_percentage, &mut self.strategy);

        if adjustment == 0.0 {
            return self.stop_loss;
        }

        if is_long {
            if actual_price_change_percentage >= 1.09 {
                self.highest_price * (1.0 - adjustment / self.leverage)
            } else {
                self.entry_price * (1.0 + adjustment / self.leverage)
            }
        } else {
            if actual_price_change_percentage >= 1.09 {
                self.lowest_price * (1.0 + adjustment / self.leverage)
            } else {
                self.entry_price * (1.0 - adjustment / self.leverage)
            }
        }
    }

    // 检查是否应平仓
    async fn check_exit_conditions(&mut self, price: &str) {
        // 如果交易已平仓，直接返回，不打印
        if self.is_closed {
            return;
        }
        let price_f64: f64 = price.parse().unwrap();

        if (self.direction == Direction::Long && price_f64 <= self.stop_loss)
            || (self.direction == Direction::Short && price_f64 >= self.stop_loss)
        {
            println!(
                "止损触发于 {}，交易对 {}， 方向{:?}, 开仓价格: {}, 关闭交易 ID {}。",
                price, self.symbol, self.direction, self.entry_price, self.order_id
            );
            let (side, position_side) = match self.direction {
                Direction::Long => ("SELL", "LONG"),
                Direction::Short => ("BUY", "SHORT"),
            };
            let _ = create_position_order(
                &self.symbol,
                side,
                position_side,
                &self.quantity,
                &self.api_key,
                &self.api_secret,
            )
            .await
            .map_err(|e| {
                eprintln!("Create position error: {:?}", e);
            });

            // 设置为已平仓状态
            self.is_closed = true;
        }
    }
}

pub fn calculate_stop_price(
    direction: &Direction,
    price: f64,
    leverage: f64,
    stop_loss_percent: f64,
) -> f64 {
    match direction {
        Direction::Long => price * (1.0 - stop_loss_percent / leverage), // 做多时根据杠杆倍数和调整参数设置止损
        Direction::Short => price * (1.0 + stop_loss_percent / leverage), // 做空时根据杠杆倍数和调整参数设置止损
    }
}

fn get_strategy(percentage: f64, strategy: &mut Vec<Strategy>) -> f64 {
    strategy.retain(|adj| percentage <= adj.max.unwrap_or(f64::INFINITY));

    strategy
        .iter()
        .find(|adj| percentage >= adj.min && adj.max.map_or(true, |max| percentage < max))
        .map_or_else(|| 0.0, |adj| adj.adjustment)
}

static POSITION: LazyLock<Arc<PositionManager>> = LazyLock::new(PositionManager::new);

pub struct PositionManager {
    keys: HashMap<String, Mutex<Vec<Position>>>,
}

impl PositionManager {
    pub fn new() -> Arc<Self> {
        let symbols = get_symbols();
        let map = symbols
            .iter()
            .map(|symbol| (symbol.clone(), Mutex::new(Vec::new())))
            .collect::<HashMap<_, _>>();
        Arc::new(PositionManager { keys: map })
    }

    async fn insert_position(&self, position: Position) -> Result<()> {
        if let Some(mutex_vec) = self.keys.get(&position.symbol) {
            let mut vec = mutex_vec.lock().await;
            vec.push(position);
            Ok(())
        } else {
            Err(Error::ErrorMessage("Symbol not found".to_string()))
        }
    }

    async fn clear_position(&self, symbol: &str) {
        if let Some(mutex_vec) = self.keys.get(symbol) {
            let mut vec = mutex_vec.lock().await;
            vec.retain(|t| if t.is_closed { false } else { true });
        }
    }

    async fn update_position_price(&self, symbol: &str, price: (String, String)) {
        if let Some(mutex_vec) = self.keys.get(symbol) {
            let mut vec = mutex_vec.lock().await;
            for t in vec.iter_mut() {
                t.update_price(price.clone()).await;
            }
        }
    }

    pub async fn get_symbol_positions(&self, symbol: &str) -> Vec<Position> {
        if let Some(mutex_vec) = self.keys.get(symbol) {
            let vec = mutex_vec.lock().await;
            vec.clone()
        } else {
            Vec::new()
        }
    }
}

fn get_position_manager() -> Arc<PositionManager> {
    POSITION.clone()
}

pub async fn inser_user_positon(position: Position) -> Result<()> {
    get_position_manager().insert_position(position).await
}

pub async fn clear_sombol_position(symbol: &str) {
    get_position_manager().clear_position(symbol).await;
}

pub async fn update_symbol_position_price(symbol: &str, price: (String, String)) {
    get_position_manager()
        .update_position_price(symbol, price)
        .await;
}

pub async fn get_symbol_positions(symbol: &str) -> Vec<Position> {
    get_position_manager().get_symbol_positions(symbol).await
}
