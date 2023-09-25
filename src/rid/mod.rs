use uuid::Uuid;

pub struct RanID {}

impl RanID {
    pub fn new() -> String {
        return Uuid::new_v4().to_string();
    }
}