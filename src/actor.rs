use std::time::Duration;

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
}

impl Player {
    pub fn new(coord : Coord) -> Self {
        Self {
            coord: coord,
            facing: Right,
            sword_coord : coord.get_to_the(Right),
            symbol : String::from("ℎ") // U-210e
        }
    }
}

pub struct Monster {
    pub coord : Coord,
    pub symbol : String,
}
