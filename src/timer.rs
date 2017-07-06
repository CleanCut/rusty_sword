use std::time::Duration;

// Timer
pub struct Timer {
    time : Duration,
    time_left : Duration,
    pub ready : bool,
}

impl Timer {
    pub fn from_millis(milliseconds : u64) -> Self {
        let duration = Duration::from_millis(milliseconds);
        Self {
            time : duration,
            time_left : duration,
            ready : false,
        }
    }

    pub fn reset(&mut self) {
        self.time_left = self.time;
    }

    pub fn update(&mut self, delta : Duration) {
        if self.ready {
            return;
        }
        self.time_left = self.time_left - delta;
        if self.time_left < Duration::from_secs(0) {
            self.ready = true;
        }
    }
}
