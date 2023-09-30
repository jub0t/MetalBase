use std::collections::HashMap;

use crate::storage::storage::Storage;

pub mod bucket;
pub mod storage;
pub mod katra;

// StorageMan = Storage Manager

pub struct StorageMan {
    storages: HashMap<String, Storage>,
}

impl StorageMan {
    pub fn new() -> Self {
        Self {
            storages: HashMap::new(),
        }
    }

    pub fn create_storage(&mut self, name: &str) {
        self.storages.insert(name.to_string(), Storage::new());
    }

    pub fn get_storage(&self, name: &str) -> Option<&Storage> {
        return self.storages.get(name);
    }
}