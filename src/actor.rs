use std::time::Duration;

use primitive::*;
use primitive::Direction::*;

// PLAYER
pub struct Player {
   coord : Coord,
}

pub fn sword_symbol(direction : Direction) -> String{
    match direction {
        Up    => "↟".to_string(), // U-219f
        Down  => "↡".to_string(), // U-21a1
        Left  => "↞".to_string(), // U-219e
        Right => "↠".to_string(), // U-21a0
    }
}

impl Player {
    pub fn new(coord : Coord) -> Self {
        Self {
            coord: coord,
        }
    }
    pub fn name(&self) -> &str { "Rusty Sword!" }
    pub fn symbol(&self) -> &str { "ℎ" } // U-210e
    pub fn coord(&self) -> &Coord { &self.coord }
    pub fn set_coord(&mut self, coord : &Coord) {
        self.coord = *coord;
    }
    pub fn update(&mut self, delta : Duration) { }
}

// MONSTER
pub struct Monster {
   coord : Coord,
}

impl Monster {
    pub fn new(coord : Coord) -> Self {
        Self {
            coord: coord,
        }
    }
    pub fn name(&self) -> &str { "Rusty Sword!" }
    pub fn symbol(&self) -> &str { "X" } // U-2020
    pub fn coord(&self) -> &Coord { &self.coord }
    pub fn set_coord(&mut self, coord : &Coord) {
        self.coord = *coord;
    }
    pub fn update(&mut self, delta : Duration) { /* XXX */ }
}



