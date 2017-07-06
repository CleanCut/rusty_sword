use actor::*;
use floor::*;
use primitive::*;
use timer::*;

use std::time::Duration;

pub struct World {
    pub floor : Floor,
    pub dirty_coords : Vec<Coord>,
    pub messages : Vec<String>,
    pub actors : Vec<Weak<Actor + Send>>,
    pub player : Arc<Player>,
    pub monsters : Vec<Arc<Monster>>,
}

impl World {
    pub fn new() -> Self {
    }
    pub fn show_message(&mut self, msg : String) {
        self.messages.insert(0, msg);
        if self.messages.len() > 4 {
            self.messages.pop();
        }
    }

    pub fn update(&mut self, delta : Duration) {
       self.player.update(delta); 
    }
}

