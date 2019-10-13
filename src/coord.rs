use crossterm::KeyEvent;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn key_to_direction(key: KeyEvent) -> Option<Direction> {
    match key {
        KeyEvent::Char('w') | KeyEvent::Up | KeyEvent::Char(',') => Some(Direction::Up),
        KeyEvent::Char('s') | KeyEvent::Down | KeyEvent::Char('o') => Some(Direction::Down),
        KeyEvent::Char('a') | KeyEvent::Left => Some(Direction::Left),
        KeyEvent::Char('d') | KeyEvent::Right | KeyEvent::Char('e') => Some(Direction::Right),
        _ => None,
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Coord {
    pub col: u16,
    pub row: u16,
}

impl Coord {
    pub fn new(col: u16, row: u16) -> Self {
        Self { col: col, row: row }
    }
    pub fn to_the(&self, direction: Direction) -> Coord {
        match direction {
            Direction::Up => Coord::new(self.col, self.row - 1),
            Direction::Down => Coord::new(self.col, self.row + 1),
            Direction::Left => Coord::new(self.col - 1, self.row),
            Direction::Right => Coord::new(self.col + 1, self.row),
        }
    }
    pub fn to(&self, target: Coord) -> Coord {
        // Are we already there?
        if *self == target {
            return *self;
        }
        // Move on the axis that's furthest away
        let col_diff = self.col as i32 - target.col as i32;
        let row_diff = self.row as i32 - target.row as i32;
        if col_diff.abs() > row_diff.abs() {
            if col_diff < 0 {
                self.to_the(Direction::Right)
            } else {
                self.to_the(Direction::Left)
            }
        } else {
            if row_diff < 0 {
                self.to_the(Direction::Down)
            } else {
                self.to_the(Direction::Up)
            }
        }
    }
}
