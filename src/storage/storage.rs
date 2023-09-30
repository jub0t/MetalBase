use std::collections::HashMap;

use crate::rid::RanID;
use crate::storage::bucket::{Bucket, Buckets};

pub struct Storage {
    id: String,
    buckets: Buckets,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            id: RanID::new(),
            buckets: HashMap::new(),
        }
    }

    pub fn create_bucket(&mut self, name: &str) {
        self.buckets.insert(name.to_string(), Bucket::new());
    }
}


