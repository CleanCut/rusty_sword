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
    pub row: u16,
    pub col: u16,
}

impl Coord {
    pub fn new(row: u16, col: u16) -> Self {
        Self { row, col }
    }
    pub fn to_the(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self::new(self.row - 1, self.col),
            Direction::Down => Self::new(self.row + 1, self.col),
            Direction::Left => Self::new(self.row, self.col - 1),
            Direction::Right => Self::new(self.row, self.col + 1),
        }
    }
    pub fn to(&self, target: Self) -> Self {
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
