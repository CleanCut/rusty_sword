extern crate rand;
//use rand::Rng;
use self::rand::{Rng, sample};

use floor::*;
use primitive::*;
use timer::*;

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
            coord : coord,
            facing : Right,
            sword_coord : coord.to_the(Right),
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
        let to_coord = self.coord.to_the(self.facing);
        if !floor.is_wall(&to_coord) {
            self.dirty = true;
            dirty_coords.push(self.coord);
            dirty_coords.push(self.sword_coord);
            self.coord = to_coord;
        }
        // Now, where is the sword?
        self.sword_coord = self.coord.to_the(self.facing);
    }
}

pub struct Monster {
    pub coord : Coord,
    pub symbol : String,
    pub move_timer : Timer,
}

impl Monster {
    pub fn new(coord : Coord, mut rng : &mut Rng) -> Self {
        let monster_symbols = vec![
            "⟟", // U-27df
            "⟠", // U-27e0
            "♄", // U-2744
            "☥", // U-2625
            "❚", // U-275a
            "☨", // U-2628
            "·", // U-00b7
        ];
        Self {
            coord : coord,
            symbol : sample(&mut rng, monster_symbols, 1)[0].to_string(),
            move_timer : Timer::from_millis(sample(&mut rng, 500..1500, 1)[0]),
        }
    }
}
