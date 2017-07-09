use ::*;

pub struct Monster {
    pub coord : Coord,
    pub symbol : String,
    pub move_timer : Timer,
}

impl Monster {
    pub fn new(coord : Coord, mut rng : &mut Rng) -> Self {
        let monster_symbols = vec![
            "·", // U-00b7
            "☥", // U-2625
            "☨", // U-2628
            "♄", // U-2744
            "❚", // U-275a
            "⟟", // U-27df
            "⟠", // U-27e0
        ];
        Self {
            coord : coord,
            symbol : sample(&mut rng, monster_symbols, 1)[0].to_string(),
            move_timer : Timer::from_millis(sample(&mut rng, 300..1500, 1)[0]),
        }
    }
    pub fn update(&mut self, delta : Duration) {
        self.move_timer.update(delta);
    }
    pub fn try_travel(&mut self,
                      target : Coord,
                      dirty_coords : &mut Vec<Coord>) {
        if !self.move_timer.ready {
            return;
        }
        self.move_timer.reset();
        let to_coord = self.coord.to(target);
        dirty_coords.push(self.coord);
        self.coord = to_coord;
    }
}
