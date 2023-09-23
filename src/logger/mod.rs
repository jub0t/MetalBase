// Logger Error Code(s)
#[derive(Debug, Clone)]
pub enum LEC {
    Database,
    Storage,
    Proxy,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Logger {}

impl Logger {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn log(code: LEC, message: &str) {
        println!("[{:#?}]: {}", code, message);
    }
}