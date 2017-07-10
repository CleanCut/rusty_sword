pub use self::Direction::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn byte_to_direction(byte : u8) -> Option<Direction> {
    match byte {
        b'w'|b',' => Some(Up),
        b's'|b'o' => Some(Down),
        b'a'      => Some(Left),
        b'd'|b'e' => Some(Right),
        _         => None
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Coord {
    pub col : u16,
    pub row : u16,
}

impl Coord {
    pub fn new(col : u16, row : u16) -> Self {
        Self {
            col : col,
            row : row,
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
    pub fn to(&self, target : Coord) -> Coord {
        // Are we already there?
        if *self == target {
            return *self;
        }
        // Move on the axis that's furthest away
        let col_diff = self.col as i32 - target.col as i32;
        let row_diff = self.row as i32 - target.row as i32;
        if col_diff.abs() > row_diff.abs() {
            if col_diff < 0 {
                return self.to_the(Right);
            } else {
                return self.to_the(Left);
            }
        } else {
            if row_diff < 0 {
                return self.to_the(Down);
            } else {
                return self.to_the(Up);
            }
        }
    }
}
