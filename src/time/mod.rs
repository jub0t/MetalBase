use std::time::Instant;

pub struct Time {
    start: Instant,
}

impl Time {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    pub fn elapsed_secs64(&self) -> f64 {
        let elapsed = self.start.elapsed();
        elapsed.as_secs() as f64 + f64::from(elapsed.subsec_nanos()) / 1_000_000_000.0
    }

    pub fn elapsed_fmt(&self) -> String {
        let raw = self.elapsed_secs64();

        if raw < 0.001 {
            format!("{:.2}μs", raw * 1_000_000.0)
        } else if raw < 1.0 {
            format!("{:.2}ms", raw * 1_000.0)
        } else if raw < 60.0 {
            format!("{:.2}s", raw)
        } else {
            format!("{:.2}m", raw / 60.0)
        }
    }
}
