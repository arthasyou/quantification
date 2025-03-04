use crate::{
    error::{Error, Result},
    utils::create_position_order,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt,
    str::FromStr,
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

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "long" => Ok(Direction::Long),
            "short" => Ok(Direction::Short),
            _ => Err(format!("Invalid direction: {}", s)),
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
    pub highest_price: f64, // 记录历史最高价格，用于动态调整止损点和判断利润情况（做多时）
    pub lowest_price: f64, // 记录历史最低价格，用于动态调整止损点和判断利润情况（做空时）
    pub direction: Direction, // 交易方向，标识是做多还是做空
    pub quantity: String,
    pub leverage: f64,
    pub strategies: Vec<Strategy>,
    pub is_closed: bool,
    pub api_key: String,
    pub api_secret: String,
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
        mut strategies: Vec<Strategy>,
        api_key: String,
        api_secret: String,
    ) -> Self {
        let stop_loss = calculate_stop_price(&direction, entry_price, leverage, stop_loss_percent);

        let stop_order_id = order_id;

        strategies.push(Strategy {
            max: 1.1,
            adjustment: 0.1,
        });

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
            strategies,
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

        let adjustement = get_adjustment(actual_price_change_percentage, &mut self.strategies);

        println!("adjustement: {}", adjustement);

        if adjustement == 0.0 {
            return self.stop_loss;
        }

        if is_long {
            if actual_price_change_percentage >= 1.09 {
                self.highest_price * (1.0 - adjustement / self.leverage)
            } else {
                self.entry_price * (1.0 + adjustement / self.leverage)
            }
        } else {
            if actual_price_change_percentage >= 1.09 {
                self.lowest_price * (1.0 + adjustement / self.leverage)
            } else {
                self.entry_price * (1.0 - adjustement / self.leverage)
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

fn get_adjustment(percentage: f64, strategies: &mut Vec<Strategy>) -> f64 {
    let strategy = strategies.iter().find(|adj| percentage >= adj.max);
    let r = strategy.map_or_else(|| 0.0, |adj| adj.adjustment);
    match strategy {
        Some(_s) => {
            strategies.retain(|adj| percentage < adj.max);
        }
        None => {}
    }
    r
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

    async fn remove_user_symbol_direction_position(
        &self,
        symbol: &str,
        user_id: &str,
        direction: &Direction,
    ) {
        if let Some(mutex_vec) = self.keys.get(symbol.to_lowercase().as_str()) {
            let mut vec = mutex_vec.lock().await;
            vec.retain(|t| {
                if t.user_id == user_id && t.direction == *direction {
                    false
                } else {
                    true
                }
            });
        }
    }

    pub async fn get_user_symbol_direction_positions(
        &self,
        symbol: &str,
        deriction: &Direction,
        user_id: &str,
    ) -> Option<Position> {
        if let Some(mutex_vec) = self.keys.get(symbol) {
            let vec = mutex_vec.lock().await;
            let t = vec.clone();
            let r = t
                .into_iter()
                .find(|t| t.direction == *deriction && t.user_id == user_id);
            r
        } else {
            None
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

pub async fn get_user_symbol_direction_positions(
    symbol: &str,
    direction: &Direction,
    user_id: &str,
) -> Option<Position> {
    get_position_manager()
        .get_user_symbol_direction_positions(symbol, direction, user_id)
        .await
}

pub async fn remove_user_symbol_direction_position(
    symbol: &str,
    user_id: &str,
    direction: &Direction,
) {
    get_position_manager()
        .remove_user_symbol_direction_position(symbol, user_id, direction)
        .await;
}

#[cfg(test)]
mod tests {
    use crate::static_items::position::{Direction, Position};

    use super::*;
    // use std::f64::EPSILON;
    const EPSILON: f64 = 1e-5;

    #[test]
    fn test_calculate_new_stop_loss_long() {
        let strategies = vec![
            Strategy {
                max: 0.1,
                adjustment: 0.02,
            },
            Strategy {
                max: 0.2,
                adjustment: 0.04,
            },
            Strategy {
                max: 0.3,
                adjustment: 0.09,
            },
            Strategy {
                max: 0.4,
                adjustment: 0.16,
            },
            Strategy {
                max: 0.5,
                adjustment: 0.25,
            },
            Strategy {
                max: 0.6,
                adjustment: 0.36,
            },
            Strategy {
                max: 0.7,
                adjustment: 0.49,
            },
            Strategy {
                max: 0.8,
                adjustment: 0.64,
            },
            Strategy {
                max: 0.9,
                adjustment: 0.81,
            },
            Strategy {
                max: 1.0,
                adjustment: 0.90,
            },
            Strategy {
                max: 1.1,
                adjustment: 0.1,
            },
        ];
        let mut trade = Position {
            user_id: "".to_string(),
            entry_price: 4.5,
            highest_price: 5.0,
            lowest_price: 4.0,
            leverage: 10.0,
            stop_loss: 4.0,
            order_id: 1,
            stop_order: 1,
            symbol: "Filusdt".to_string(),
            direction: Direction::Long,
            quantity: "1.0".to_string(),
            strategies: strategies.clone(),
            is_closed: false,
            api_key: "".to_string(),
            api_secret: "".to_string(),
        };

        let test_cases = vec![
            (0.009, 4.0, "No change for profit < 10%"),
            (0.01, 4.509, "Profit 10%"),
            (0.02, 4.518, "Profit 20%"),
            (0.03, 4.5405, "Profit 30%"),
            (0.04, 4.572, "Profit 40%"),
            (0.05, 4.6125, "Profit 50%"),
            (0.06, 4.662, "Profit 60%"),
            (0.07, 4.7205, "Profit 70%"),
            (0.08, 4.788, "Profit 80%"),
            (0.091, 4.8645, "Profit 90%"),
            (0.1, 4.905, "Profit 100%"),
            (0.12, 4.95, "Profit 120%"),
        ];

        for (profit, expected, description) in test_cases {
            let result = trade.calculate_new_stop_loss(profit, true);
            // let EPSILON = 1e-9;
            assert!(
                (result - expected).abs() <= EPSILON,
                "{}: Expected {:.4}, got {:.4}",
                description,
                expected,
                result
            );
        }
    }

    #[test]
    fn test_calculate_new_stop_loss_short() {
        let strategies = vec![
            Strategy {
                max: 0.1,
                adjustment: 0.02,
            },
            Strategy {
                max: 0.2,
                adjustment: 0.04,
            },
            Strategy {
                max: 0.3,
                adjustment: 0.09,
            },
            Strategy {
                max: 0.4,
                adjustment: 0.16,
            },
            Strategy {
                max: 0.5,
                adjustment: 0.25,
            },
            Strategy {
                max: 0.6,
                adjustment: 0.36,
            },
            Strategy {
                max: 0.7,
                adjustment: 0.49,
            },
            Strategy {
                max: 0.8,
                adjustment: 0.64,
            },
            Strategy {
                max: 0.9,
                adjustment: 0.81,
            },
            Strategy {
                max: 1.0,
                adjustment: 0.90,
            },
            Strategy {
                max: 1.1,
                adjustment: 0.1,
            },
        ];
        let mut trade = Position {
            user_id: "".to_string(),
            entry_price: 4.5,
            highest_price: 5.0,
            lowest_price: 4.0,
            leverage: 10.0,
            stop_loss: 5.0,
            order_id: 1,
            stop_order: 1,
            symbol: "Filusdt".to_string(),
            direction: Direction::Short,
            quantity: "1.0".to_string(),
            strategies,
            is_closed: false,
            api_key: "".to_string(),
            api_secret: "".to_string(),
        };

        let test_cases = vec![
            (0.009, 5.0, "No change for profit < 10%"),
            (0.011, 4.491, "Profit 10%"),
            (0.021, 4.482, "Profit 20%"),
            (0.031, 4.4595, "Profit 30%"),
            (0.041, 4.428, "Profit 40%"),
            (0.051, 4.3875, "Profit 50%"),
            (0.061, 4.338, "Profit 60%"),
            (0.071, 4.2795, "Profit 70%"),
            (0.081, 4.212, "Profit 80%"),
            (0.091, 4.1355, "Profit 90%"),
            (0.10, 4.095, "Profit 100%"),
            (0.12, 4.04, "Profit 120%"),
        ];

        for (profit, expected, description) in test_cases {
            let result = trade.calculate_new_stop_loss(profit, false);
            assert!(
                (result - expected).abs() < EPSILON,
                "{}: Expected {:.4}, got {:.4}",
                description,
                expected,
                result
            );
        }
    }
}
