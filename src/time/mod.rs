use tokio::time::Instant;

pub struct Time {
    start: Instant,
}

impl Time {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    pub fn elapsed_raw(&self) -> u128 {
        self.start.elapsed().as_nanos()
    }

    pub fn elapsed_fmt(&self) -> String {
        let raw = self.elapsed_raw();
        println!("{}", raw);


        if raw < 1000 {
            format!("{}ns", raw)
        } else if raw < 1000000 {
            format!("{}μs", raw / 1000)
        } else if raw < 1000000000 {
            format!("{}ms", raw / 1000000)
        } else if raw < 1000000000000 {
            format!("{}s", raw / 1000000000)
        } else {
            format!("{}s", raw / 1000000000000)
        }
    }
}