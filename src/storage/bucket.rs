use std::collections::HashMap;

use crate::rid::RanID;
use crate::storage::katra::{FileType, Katra};

pub type Buckets = HashMap<String, Bucket>;

pub struct Bucket {
    id: String,
    max_size: usize,
    drops: HashMap<String, Katra>,
    banned_types: Vec<FileType>,
}

impl Bucket {
    pub fn new() -> Self {
        return Self {
            id: RanID::new(),
            drops: HashMap::new(),
            banned_types: Vec::new(),
            max_size: 1024 * 1024 * 1024 * 1024, // One Gigabyte
        };
    }

    pub fn set_max_size(&mut self, max_size: usize) {
        self.max_size = max_size
    }

    pub fn insert(&mut self, file: Katra) {}
}