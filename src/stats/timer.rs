use std::time::{Duration, Instant};

pub struct Timer {
    pub start: Instant,
    pub last_instant: Instant,
    pub delta: Duration,
    pub total: Duration,
    pub period: Duration,
    pub countdown: Duration,
    pub ready: bool,
}

impl Timer {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            start: now,
            last_instant: now,
            total: Duration::default(),
            delta: Duration::default(),
            period: Duration::from_millis(1000),
            countdown: Duration::default(),
            ready: true,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();

        self.total = now - self.start;
        self.delta = now - self.last_instant;
        self.last_instant = now;
        self.countdown = self.countdown.checked_sub(self.delta).unwrap_or_else(|| {
            self.ready = true;
            self.period
        })
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}
