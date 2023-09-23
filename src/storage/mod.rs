use std::collections::HashMap;

use uuid::Uuid;

use crate::storage::bucket::Bucket;

pub mod bucket;
pub mod katra;

pub type Buckets = HashMap<String, Bucket>;

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


