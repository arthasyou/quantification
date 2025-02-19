use std::{
    collections::HashMap,
    sync::{Arc, LazyLock},
};

use tokio::sync::Mutex;

static KEY: LazyLock<Arc<KeyManager>> = LazyLock::new(KeyManager::new);

#[derive(Debug, Clone)]
pub struct SecretKey {
    pub id: String,
    pub key: String,
    pub secret: String,
}

impl SecretKey {
    pub fn new(id: String, key: String, secret: String) -> Self {
        SecretKey { id, key, secret }
    }
}

pub struct KeyManager {
    keys: Mutex<HashMap<String, SecretKey>>, // 锁住 HashMap，确保多线程安全
}

impl KeyManager {
    // 创建新的 KeyManager，初始化 keys
    pub fn new() -> Arc<Self> {
        let map = HashMap::new(); // 初始化为空的 HashMap
        Arc::new(KeyManager {
            keys: Mutex::new(map),
        })
    }

    // 插入一个新密钥
    pub async fn insert_key(&self, key: SecretKey) {
        let mut map = self.keys.lock().await;
        map.insert(key.id.clone(), key);
    }

    // 删除一个密钥
    pub async fn delete_key(&self, key_id: &str) {
        let mut map = self.keys.lock().await;
        map.remove(key_id);
    }

    // 获取一个密钥
    pub async fn get_key(&self, key_id: &str) -> Option<SecretKey> {
        let map = self.keys.lock().await;
        let key = map.get(key_id).cloned();
        key
    }
}

fn get_key_manager() -> Arc<KeyManager> {
    KEY.clone()
}

pub async fn insert_secret_key(key: SecretKey) {
    get_key_manager().insert_key(key).await;
}

pub async fn get_secret_key(user_id: &str) -> Option<SecretKey> {
    get_key_manager().get_key(user_id).await
}

pub async fn delete_secret_key(user_id: &str) {
    get_key_manager().delete_key(user_id).await;
}
