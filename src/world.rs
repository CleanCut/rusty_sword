use actor::*;
use floor::*;
use primitive::*;

use std::time::Duration;

pub struct World {
    pub floors : Vec<Floor>,
    pub actors : Vec<Box<Actor + Send>>,
    pub dirty_coords : Vec<Coord>,
    pub player : Box<Actor + Send>,
    pub messages : Vec<String>,
}

impl World {
    pub fn show_message(&mut self, msg : String) {
        self.messages.insert(0, msg);
        if self.messages.len() > 4 {
            self.messages.pop();
        }
    }
}

// Timer
pub struct Timer {
    time : Duration,
    time_left : Duration,
    pub ready : bool,
}

impl Timer {
    pub fn new(time : Duration) -> Self {
        Self {
            time : time,
            time_left : time,
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
