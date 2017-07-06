use primitive::*;
use timer::*;

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
   move_timer : Timer,
}

impl Player {
    pub fn new(col : u16, row : u16) -> Self {
        Player {
            _coord : Coord { col, row },
            move_timer : Timer::from_millis(300),
        }
    }
}

impl Actor for Player {
    fn name(&self) -> &str { "Rusty Sword!" }
    fn symbol(&self) -> &str { "†" } // U-2020
    fn coord(&self) -> &Coord { &self._coord }
    fn set_coord(&mut self, coord : &Coord) {
        self._coord = *coord;
    }
    fn update(&mut self, delta : Duration) {
        self.move_timer.update(delta);
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



