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

    pub fn elapsed_secs64(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }

    pub fn elapsed_fmt(&self) -> String {
        let raw = self.elapsed_secs64();

        if raw < 0.001 {
            format!("{:.3}ns", raw * 1_000_000.0)
        } else if raw < 0.01 {
            format!("{:.3}μs", raw * 1_000_000.0)
        } else if raw < 1. {
            format!("{:.3}ms", raw * 1_000.0)
        } else if raw < 60. {
            format!("{:.2}s", raw)
        } else {
            format!("{:.2}m", raw / 60.0)
        }
    }
}
