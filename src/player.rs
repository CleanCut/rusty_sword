use ::*;

pub fn sword_symbol(direction : &Direction) -> String {
    match *direction {
        Up    => "⤉".to_string(), // U-2909
        Down  => "⤈".to_string(), // U-2908
        Left  => "↢".to_string(), // tilde
        Right => "↣".to_string(), // tilde
    }
}

// PLAYER
pub struct Player {
   pub coord : Coord,
   pub facing : Direction,
   pub sword_coord : Coord,
   pub symbol : String,
   pub dirty : bool,
   pub score : u64,
}

impl Player {
    pub fn new(coord : Coord) -> Self {
        Self {
            coord : coord,
            facing : Right,
            sword_coord : coord.to_the(Right),
            symbol : String::from("☥"), // U-2625
            dirty : true,
            score : 0,
        }
    }
    pub fn travel(&mut self,
                  direction : Direction,
                  floor : &Floor,
                  dirty_coords : &mut Vec<Coord>) -> bool {
        let mut moved = false;
        // Do I change direction?
        if self.facing != direction {
            self.dirty = true;
            dirty_coords.push(self.sword_coord);
            self.facing = direction;
        } else {
            // Can I move?
            let to_coord = self.coord.to_the(self.facing);
            if !floor.is_wall(&to_coord) {
                self.dirty = true;
                moved = true;
                dirty_coords.push(self.coord);
                dirty_coords.push(self.sword_coord);
                self.coord = to_coord;
            }
        }
        // Now, where is the sword?
        self.sword_coord = self.coord.to_the(self.facing);
        return moved;
    }
}