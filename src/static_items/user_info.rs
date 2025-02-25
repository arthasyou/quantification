use std::{
    collections::HashMap,
    sync::{Arc, LazyLock},
};

use rust_decimal::Decimal;
use tokio::sync::Mutex;

static KEY: LazyLock<Arc<UserInfoManager>> = LazyLock::new(UserInfoManager::new);

#[derive(Debug, Clone)]
pub struct UserInfo {
    pub id: String,
    pub agent_id: String,
    pub balance: Decimal,
}

impl UserInfo {
    pub fn new(id: String, agent_id: String, balance: Decimal) -> Self {
        UserInfo {
            id,
            agent_id,
            balance,
        }
    }
}

pub struct UserInfoManager {
    keys: Mutex<HashMap<String, UserInfo>>, // 锁住 HashMap，确保多线程安全
}

impl UserInfoManager {
    // 创建新的 KeyManager，初始化 keys
    pub fn new() -> Arc<Self> {
        let map = HashMap::new(); // 初始化为空的 HashMap
        Arc::new(UserInfoManager {
            keys: Mutex::new(map),
        })
    }

    pub async fn insert_user_info(&self, info: UserInfo) {
        let mut map = self.keys.lock().await;
        map.insert(info.id.clone(), info);
    }

    pub async fn delete_user_info(&self, id: &str) {
        let mut map = self.keys.lock().await;
        map.remove(id);
    }

    pub async fn get_user_info(&self, id: &str) -> Option<UserInfo> {
        let map = self.keys.lock().await;
        let key = map.get(id).cloned();
        key
    }
}

fn get_key_manager() -> Arc<UserInfoManager> {
    KEY.clone()
}

pub async fn insert_user_info(key: UserInfo) {
    get_key_manager().insert_user_info(key).await;
}

pub async fn delete_user_info(user_id: &str) {
    get_key_manager().delete_user_info(user_id).await;
}

pub async fn get_agent_id(user_id: &str) -> Option<String> {
    let info = get_key_manager().get_user_info(user_id).await?;
    Some(info.agent_id)
}

pub async fn get_user_balance(user_id: &str) -> Option<Decimal> {
    let info = get_key_manager().get_user_info(user_id).await?;
    Some(info.balance)
}

pub async fn add_user_balance(user_id: &str, amount: Decimal) -> Option<Decimal> {
    let mut info = get_key_manager().get_user_info(user_id).await?;
    info.balance += amount;
    Some(info.balance)
}

pub async fn sub_user_balance(user_id: &str, amount: Decimal) -> Option<Decimal> {
    let mut info = get_key_manager().get_user_info(user_id).await?;
    info.balance -= amount;
    Some(info.balance)
}
