#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Value {
    // Basics
    String(String),
    Boolean(bool),

    // Numbers
    Int(i32),
    UInt(u32),
    Float(f32),

    None,
}

impl Default for Value {
    fn default() -> Self {
        return Self::None;
    }
}

impl Value {}
