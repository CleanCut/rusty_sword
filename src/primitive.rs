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
    pub fn new(col : u16, row : u16) -> Self {
        Self {
            col : col.into(),
            row : row.into(),
        }
    }
    pub fn to_the(&self, direction : Direction) -> Coord {
        match direction {
            Up    => Coord::new(self.col, self.row-1),
            Down  => Coord::new(self.col, self.row+1),
            Left  => Coord::new(self.col-1, self.row),
            Right => Coord::new(self.col+1, self.row),
        }
    }
}

