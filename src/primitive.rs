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



