use actor::*;
use floor::*;
use primitive::*;
use timer::*;

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

    pub fn update(&mut self, delta : Duration) {
       self.player.update(delta); 
    }
}

