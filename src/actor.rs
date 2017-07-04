use primitive::*;

use std::time::Duration;

pub trait Actor {
    fn name(&self) -> &str;
    fn symbol(&self) -> &str;
    fn coord(&self) -> &Coord;
    fn set_coord(&mut self, coord : &Coord);
    fn update(&mut self, delta : Duration);
}


// PLAYER
pub struct Player {
   pub _coord : Coord,
}

impl Actor for Player {
    fn name(&self) -> &str { "Rusty Sword!" }
    fn symbol(&self) -> &str { "†" } // U-2020
    fn coord(&self) -> &Coord { &self._coord }
    fn set_coord(&mut self, coord : &Coord) {
        self._coord = *coord;
    }
    fn update(&mut self, delta : Duration) {
        // XXX
    }
}

// MONSTER
pub struct Monster {
   pub _coord : Coord,
}

impl Actor for Monster {
    fn name(&self) -> &str { "Rusty Sword!" }
    fn symbol(&self) -> &str { "X" } // U-2020
    fn coord(&self) -> &Coord { &self._coord }
    fn set_coord(&mut self, coord : &Coord) {
        self._coord.col = coord.col;
        self._coord.row = coord.row;
    }
    fn update(&mut self, delta : Duration) {}
}



