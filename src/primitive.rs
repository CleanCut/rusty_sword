use self::Direction::*;

pub const WORLD_COLS : usize = 60;
pub const WORLD_ROWS : usize = 30;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn char_to_direction(ch : char) -> Option<Direction> {
    match ch {
        'w'|',' => Some(Up),
        's'|'o' => Some(Down),
        'a'     => Some(Left),
        'd'|'e' => Some(Right),
        _       => None
    }
}

#[derive(Clone, Copy)]
pub struct Coord {
    pub col : u16,
    pub row : u16,
}

impl Coord {
    pub fn get_to_the(&self, direction : Direction) -> Coord {
        match direction {
            Up    => Coord { col: self.col, row: self.row-1 },
            Down  => Coord { col: self.col, row: self.row+1 },
            Left  => Coord { col: self.col-1, row: self.row },
            Right => Coord { col: self.col+1, row: self.row },
        }
    }
}

pub fn clamp(x : u16, min : u16, max : u16) -> u16 {
    if x < min { min } else if x > max { max } else { x }
}
