use ::*;

#[derive(Copy, Clone)]
pub struct Tile {
    pub wall : Option<char>,
}

impl Tile {
    pub fn new(wall : Option<char>) -> Self {
        if let Some(s) = wall {
            Self { wall: Some(s) }
        } else {
            Self { wall: None }
        }
    }
}

pub struct Floor {
    pub rows: usize,
    pub cols: usize,
    pub tiles: Vec<Vec<Tile>>
}

impl Floor {
    pub fn new(cols : usize, rows : usize) -> Self {
        // Tiles to use
        let horizontal   = Tile::new(Some('─')); // U-2500
        let vertical     = Tile::new(Some('│')); // U-2502
        let top_left     = Tile::new(Some('┌')); // U-250c
        let top_right    = Tile::new(Some('┐')); // U-2510
        let bottom_left  = Tile::new(Some('└')); // U-2514
        let bottom_right = Tile::new(Some('┘')); // U-2518
        let blank        = Tile::new(None);

        // Create tiles 2D Vec and populate with empty rows
        let mut tiles = Vec::<Vec<Tile>>::with_capacity(rows);
        for _ in 0..rows {
            tiles.push(Vec::<Tile>::with_capacity(rows));
        }
        // First row is all wall
        tiles[0].push(top_left);
        for _ in 1..cols-1 {
            tiles[0].push(horizontal);
        }
        tiles[0].push(top_right);
        // Middle rows are vertical sides and blank middles
        for row in 1..rows-1 {
            tiles[row].push(vertical);
            for _ in 1..cols-1 {
                tiles[row].push(blank);
            }
            tiles[row].push(vertical);
        }
        // Bottom row is also all wall
        tiles[rows-1].push(bottom_left);
        for _ in 1..cols-1 {
            tiles[rows-1].push(horizontal);
        }
        tiles[rows-1].push(bottom_right);

        // Create the floor
        Self {
            rows: rows,
            cols: cols,
            tiles: tiles,
        }
    }
    pub fn get_symbol(&self, coord : &Coord) -> String {
        if let Some(wall) = self.tiles[coord.row as usize][coord.col as usize].wall {
            wall.to_string()
        } else {
            " ".to_string()
        }
    }
    pub fn is_wall(&self, coord : &Coord) -> bool {
        self.tiles[coord.row as usize][coord.col as usize].wall.is_some()
    }
}
