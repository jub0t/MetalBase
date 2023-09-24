use std::collections::HashMap;

use uuid::Uuid;

use crate::storage::bucket::Buckets;

pub struct Storage {
    id: String,
    buckets: Buckets,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            buckets: HashMap::new(),
        }
    }
}


