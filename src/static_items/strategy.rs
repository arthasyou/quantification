use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, LazyLock},
};
use tokio::sync::Mutex;
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema, Debug, Clone)]
pub struct Strategy {
    pub min: f64,
    pub max: Option<f64>,
    pub adjustment: f64,
}

#[derive(Deserialize, Serialize, ToSchema, Debug, Clone)]
pub struct StrategyConfig {
    pub s1: Vec<Strategy>,
    pub s2: Vec<Strategy>,
}

impl Default for StrategyConfig {
    fn default() -> Self {
        let strategies = vec![
            Strategy {
                min: 0.10,
                max: Some(0.19),
                adjustment: 0.02,
            },
            Strategy {
                min: 0.20,
                max: Some(0.29),
                adjustment: 0.04,
            },
            Strategy {
                min: 0.30,
                max: Some(0.39),
                adjustment: 0.09,
            },
            Strategy {
                min: 0.40,
                max: Some(0.49),
                adjustment: 0.16,
            },
            Strategy {
                min: 0.50,
                max: Some(0.59),
                adjustment: 0.25,
            },
            Strategy {
                min: 0.60,
                max: Some(0.69),
                adjustment: 0.36,
            },
            Strategy {
                min: 0.70,
                max: Some(0.79),
                adjustment: 0.49,
            },
            Strategy {
                min: 0.7999,
                max: Some(0.89),
                adjustment: 0.64,
            },
            Strategy {
                min: 0.8999,
                max: Some(1.0),
                adjustment: 0.81,
            },
            Strategy {
                min: 0.9999,
                max: Some(1.1),
                adjustment: 0.90,
            },
            Strategy {
                min: 1.1,
                max: None,
                adjustment: 0.1,
            },
        ];

        StrategyConfig {
            s1: strategies.clone(),
            s2: strategies,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserStrategy {
    pub user_id: String,
    pub cfg: StrategyConfig,
}

static STRATEGY: LazyLock<Arc<StrategyManager>> = LazyLock::new(StrategyManager::new);

pub struct StrategyManager {
    keys: Mutex<HashMap<String, UserStrategy>>, // 锁住 HashMap，确保多线程安全
}

impl StrategyManager {
    pub fn new() -> Arc<Self> {
        let map = HashMap::new(); // 初始化为空的 HashMap
        Arc::new(StrategyManager {
            keys: Mutex::new(map),
        })
    }

    pub async fn insert_strategy(&self, strategy: UserStrategy) {
        let mut map = self.keys.lock().await;
        map.insert(strategy.user_id.clone(), strategy);
    }

    pub async fn update_strategy(&self, user_id: &str, cfg: StrategyConfig) {
        let mut map = self.keys.lock().await;
        if let Some(strategy) = map.get_mut(user_id) {
            strategy.cfg = cfg;
        }
    }
    pub async fn delete_strategy(&self, user_id: &str) {
        let mut map = self.keys.lock().await;
        map.remove(user_id);
    }

    pub async fn get_strategy(&self, user_id: &str) -> Option<StrategyConfig> {
        let map = self.keys.lock().await;
        let key = map.get(user_id).cloned();
        key.map(|k| k.cfg)
    }
}

fn get_strategy_manager() -> Arc<StrategyManager> {
    STRATEGY.clone()
}

pub async fn insert_user_strategy(strategy: UserStrategy) {
    get_strategy_manager().insert_strategy(strategy).await;
}

pub async fn update_user_strategy(user_id: &str, cfg: StrategyConfig) {
    get_strategy_manager().update_strategy(user_id, cfg).await;
}

pub async fn delete_user_strategy(user_id: &str) {
    get_strategy_manager().delete_strategy(user_id).await;
}

pub async fn get_user_strategy(user_id: &str) -> Option<StrategyConfig> {
    get_strategy_manager().get_strategy(user_id).await
}

pub async fn get_user_spec_strategy(user_id: &str, spec_id: u8) -> Option<Vec<Strategy>> {
    let cfg = get_user_strategy(user_id).await;
    match cfg {
        Some(cfg) => {
            if spec_id == 1 {
                Some(cfg.s1)
            } else {
                Some(cfg.s2)
            }
        }
        None => None,
    }
}
