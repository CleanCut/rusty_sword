use actor::*;
use floor::*;
use primitive::*;

use std::sync::*;
use std::time::Duration;

pub struct World {
    pub floor : Floor,
    pub dirty_coords : Vec<Coord>,
    pub messages : Vec<String>,
    pub player : Player,
    pub monsters : Vec<Monster>,
}

impl World {
    pub fn new(cols : usize, rows : usize) -> Self {
        Self {
            floor: Floor::new("Dungeon Level 1", cols, rows),
            dirty_coords: Vec::<Coord>::new(),
            messages: vec!["Welcome to: Rusty Sword – Game of Infamy!".to_string()],
            player : Player::new(Coord {col: 1, row: 1}),
            monsters : Vec::<Monster>::new(),
        }
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

