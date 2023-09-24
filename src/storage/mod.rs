use crate::storage::katra::Katra;

pub mod bucket;
pub mod katra;
pub mod storage;

// StorageMan = Storage Manager

pub struct StorageMan {
    katras: Vec<Katra>,
}

impl StorageMan {
    pub fn new() -> Self {
        Self {
            katras: Vec::new(),
        }
    }

    pub fn insert(&mut self, item: Katra) {
        self.katras.push(item);
    }
}