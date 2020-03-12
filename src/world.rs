use crate::coord::Coord;
use crate::floor::Floor;
use crate::monster::Monster;
use crate::player::Player;

pub struct World {
    pub floor: Floor,
    pub player: Player,
    pub dirty_coords: Vec<Coord>,
    pub monsters: Vec<Monster>,
}

impl World {
    pub fn new() -> Self {
        Self {
            floor: Floor::new(60, 30),
            player: Player::new(Coord::new(30, 15)),
            dirty_coords: Vec::<Coord>::new(),
            monsters: Vec::<Monster>::new(),
        }
    }
}
