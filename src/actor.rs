use primitive::*;
use timer::*;
use world::*;

use std::time::Duration;
use std::sync::*;

pub trait Actor {
    fn name(&self) -> &str;
    fn symbol(&self) -> &str;
    fn coord(&self) -> &Coord;
    fn set_coord(&mut self, coord : &Coord);
    fn update(&mut self, delta : Duration);
}


// PLAYER
pub struct Player {
   coord : Coord,
   move_timer : Timer,
   world : Arc<Mutex<World>>,
}

impl Player {
    fn new(coord : Coord, world : Arc<Mutex<World>>) -> Self {
        Self {
            coord: coord,
            world: world,
        }
    }
    pub fn handle_input(&mut self, keypress : &char, dirty_coord_tx : &mut mpsc::Sender<Coord>) {
        match *keypress {
            'd'|'e' => {
                dirty_coord_tx.send(self.coord.clone());
                //self._coord.col = clamp(self._coord.col + 1, 0, ;
            },
            _ => {},
        }
    }
}

impl Actor for Player {
    fn name(&self) -> &str { "Rusty Sword!" }
    fn symbol(&self) -> &str { "†" } // U-2020
    fn coord(&self) -> &Coord { &self.coord }
    fn set_coord(&mut self, coord : &Coord) {
        self.coord = *coord;
    }
    fn update(&mut self, delta : Duration) { 
        self.move_timer.update(delta);
    }
}

// MONSTER
pub struct Monster {
   coord : Coord,
   world : Arc<Mutex<World>>,
}

impl Monster {
    fn new(coord : Coord, world : Arc<Mutex<World>>) -> Self {
        Self {
            coord: coord,
            world: world,
        }
    }
}

impl Actor for Monster {
    fn name(&self) -> &str { "Rusty Sword!" }
    fn symbol(&self) -> &str { "X" } // U-2020
    fn coord(&self) -> &Coord { &self.coord }
    fn set_coord(&mut self, coord : &Coord) {
        self.coord = *coord;
    }
    fn update(&mut self, delta : Duration) { /* XXX */ }
}



