use actor::*;
use floor::*;
use primitive::*;

pub struct World {
    pub floor : Floor,
    pub dirty_coords : Vec<Coord>,
    pub messages : Vec<String>,
    pub actors : Vec<Weak<Actor + Send>>,
    pub player : Arc<Player>,
    pub monsters : Arc<Monster>,
}

impl World {
    pub fn show_message(&mut self, msg : String) {
        self.messages.insert(0, msg);
        if self.messages.len() > 4 {
            self.messages.pop();
        }
    }
}

