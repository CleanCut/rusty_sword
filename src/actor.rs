use std::time::Duration;

use floor::*;
use primitive::*;
use primitive::Direction::*;

pub fn sword_symbol(direction : &Direction) -> String {
    match *direction {
        Up    => "↟".to_string(), // U-219f
        Down  => "↡".to_string(), // U-21a1
        Left  => "↞".to_string(), // U-219e
        Right => "↠".to_string(), // U-21a0
    }
}

// PLAYER
pub struct Player {
   pub coord : Coord,
   pub facing : Direction,
   pub sword_coord : Coord,
   pub symbol : String,
   pub dirty : bool,
}

impl Player {
    pub fn new(coord : Coord) -> Self {
        Self {
            coord: coord,
            facing: Right,
            sword_coord : coord.get_to_the(Right),
            symbol : String::from("ℎ"), // U-210e
            dirty : true,

        }
    }
    pub fn travel(&mut self, direction : Direction, floor : &Floor, dirty_coords : &mut Vec<Coord>) {
        // Do I change direction?
        if self.facing != direction {
            self.dirty = true;
            dirty_coords.push(self.sword_coord);
            self.facing = direction;
        }
        // Can I move?
        let to_coord = self.coord.get_to_the(self.facing);
        if !floor.is_wall(&to_coord) {
            self.dirty = true;
            dirty_coords.push(self.coord);
            dirty_coords.push(self.sword_coord);
            self.coord = to_coord;
        }
        // Now, where is the sword?
        self.sword_coord = self.coord.get_to_the(self.facing);
    }
}

pub struct Monster {
    pub coord : Coord,
    pub symbol : String,
}
