use std::collections::HashMap;

use crate::storage::katra::Katra;

pub struct Bucket {
    id: String,
    max_size: usize,
    drops: HashMap<String, Katra>,
}

impl Bucket {}