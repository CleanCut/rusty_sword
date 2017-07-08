use ::*;

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
   pub score : u64,
}

impl Player {
    pub fn new(coord : Coord) -> Self {
        Self {
            coord : coord,
            facing : Right,
            sword_coord : coord.to_the(Right),
            symbol : String::from("ℎ"), // U-210e
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
            move_timer : Timer::from_millis(sample(&mut rng, 300..1500, 1)[0]),
        }
    }
    //pub fn spawn(&mut Rng, cols : u16, rows : u16, monsters : &mut Vec<Monster>) {


    //}
    pub fn update(&mut self, delta : Duration) {
        self.move_timer.update(delta);
    }
    pub fn try_travel(&mut self,
                      target : Coord,
                      floor : &Floor,
                      dirty_coords : &mut Vec<Coord>) {
        if !self.move_timer.ready {
            return;
        }
        self.move_timer.reset();
        let to_coord = self.coord.to(target);
        // Can't move through walls
        if floor.is_wall(&to_coord) {
            return;
        }
        dirty_coords.push(self.coord);
        self.coord = to_coord;
    }
}
