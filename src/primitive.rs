pub use self::Direction::*;

#[derive(Copy, Clone, PartialEq, Eq)]
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Coord {
    pub col : u16,
    pub row : u16,
}

impl Coord {
    pub fn to_the(&self, direction : Direction) -> Coord {
        match direction {
            Up    => Coord { col: self.col, row: self.row-1 },
            Down  => Coord { col: self.col, row: self.row+1 },
            Left  => Coord { col: self.col-1, row: self.row },
            Right => Coord { col: self.col+1, row: self.row },
        }
    }
}

