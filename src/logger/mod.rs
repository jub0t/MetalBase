// Logger Error Code(s)
#[derive(Debug)]
pub enum LEC {
    Database,
    Storage,
    Proxy,
    Unknown,
}

pub struct Logger {}

impl Logger {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn log(code: LEC, message: &str) {
        println!("[{:#?}]: {}", code, message);
    }
}