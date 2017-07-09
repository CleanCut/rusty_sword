use ::*;

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
        self.ready = false;
        self.time_left = self.time;
    }

    pub fn update(&mut self, delta : Duration) {
        if self.ready {
            return;
        }
        if let Some(result) = self.time_left.checked_sub(delta) {
            self.time_left = result;
        } else {
            self.ready = true;
        }
    }
}
