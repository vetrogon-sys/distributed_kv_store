use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::{Arc, Mutex};

pub struct KeyValueStore {
    store: Arc<Mutex<HashMap<String, String>>>,
}

impl KeyValueStore {
    pub fn new() -> KeyValueStore {
        KeyValueStore {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn put(&self, key: String, value: String) {
        let mut store = self.store.lock().unwrap();
        store.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        let store = self.store.lock().unwrap();
        store.get(&key).cloned()
    }

    pub fn delete(&self, key: String) {
        let mut store = self.store.lock().unwrap();
        store.remove(&key);
    }

}

pub struct  DistributedKeyValueStore {
    servers: Vec<KeyValueStore>,
}

impl DistributedKeyValueStore {

    pub fn new(servers: Vec<KeyValueStore>) -> DistributedKeyValueStore {
        DistributedKeyValueStore { servers }
    }

    fn get_server_for_key(&self, key: &String) -> Option<&KeyValueStore> {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let idx = hasher.finish() % (u64::from(self.servers.len()));
        self.servers.get(idx as usize)
    }

    pub fn put(&self, key: String, value: String) {
        let server = self.get_server_for_key(&key);
        server.unwrap().put(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        let server = self.get_server_for_key(&key);
        server.unwrap().get(key)
    }

    pub fn delete(&self, key: String) {
        let server = self.get_server_for_key(&key);
        server.unwrap().delete(key);
    }

}