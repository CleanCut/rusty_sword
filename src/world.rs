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
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            floor: Floor::new(rows, cols),
            player: Player::new(Coord::new((rows / 2) as u16, (cols / 2) as u16)),
            dirty_coords: Vec::<Coord>::new(),
            monsters: Vec::<Monster>::new(),
        }
    }
}
