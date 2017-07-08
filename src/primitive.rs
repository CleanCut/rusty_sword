use self::Direction::*;

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

pub fn clamp(x : u16, min : u16, max : u16) -> u16 {
    if x < min { min } else if x > max { max } else { x }
}
