#[derive(Clone, Copy)]
pub struct Coord {
    pub col : u16,
    pub row : u16,
}

impl Coord {
    pub fn at(col : u16, row : u16) -> Self {
        Self {
            col: col,
            row: row,
        }
    }
}


pub fn clamp(x : u16, min : u16, max : u16) -> u16 {
    if x < min { min } else if x > max { max } else { x }
}
