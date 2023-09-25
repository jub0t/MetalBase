use std::collections::HashMap;

use crate::rid::RanID;
use crate::storage::bucket::Buckets;

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
}


