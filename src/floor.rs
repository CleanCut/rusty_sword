use crate::coord::Coord;

pub struct Floor {
    pub rows: usize,
    pub cols: usize,
    pub tiles: Vec<Vec<Option<char>>>,
}

impl Floor {
    pub fn new(cols: usize, rows: usize) -> Self {
        // Tiles to use
        let horizontal = Some('─'); // U-2500
        let vertical = Some('│'); // U-2502
        let top_left = Some('┌'); // U-250c
        let top_right = Some('┐'); // U-2510
        let bottom_left = Some('└'); // U-2514
        let bottom_right = Some('┘'); // U-2518
        let blank = None;

        // Row-major
        let mut tiles = Vec::<Vec<Option<char>>>::with_capacity(rows);
        for _ in 0..rows {
            tiles.push(Vec::<Option<char>>::with_capacity(cols));
        }
        // First row is all wall
        tiles[0].push(top_left);
        for _ in 1..cols - 1 {
            tiles[0].push(horizontal);
        }
        tiles[0].push(top_right);
        // Middle rows are vertical sides and blank middles
        for row in 1..rows - 1 {
            tiles[row].push(vertical);
            for _ in 1..cols - 1 {
                tiles[row].push(blank);
            }
            tiles[row].push(vertical);
        }
        // Bottom row is all wall
        tiles[rows - 1].push(bottom_left);
        for _ in 1..cols - 1 {
            tiles[rows - 1].push(horizontal);
        }
        tiles[rows - 1].push(bottom_right);

        // Create the floor
        Self { rows, cols, tiles }
    }
    pub fn get_symbol(&self, coord: Coord) -> String {
        if let Some(symbol) = self.tiles[coord.row as usize][coord.col as usize] {
            symbol.to_string()
        } else {
            " ".to_string()
        }
    }
    pub fn is_wall(&self, coord: Coord) -> bool {
        self.tiles[coord.row as usize][coord.col as usize].is_some()
    }
}
