use std::fmt::{Display, Formatter};
use rand::Rng;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct Position {
    row: i32,
    column: i32,
}

impl Position {
    pub fn new(row: i32, column: i32) -> Position {
        Position { row, column }
    }

    pub fn random(width: usize, height: usize) -> Position {
        let mut rng = rand::thread_rng();
        let row = rng.gen_range(0..height) as i32;
        let column = rng.gen_range(0..width) as i32;
        Position { row, column }
    }

    pub fn up(&self) -> Position {
        Position {
            row: self.row - 1,
            ..*self
        }
    }

    pub fn right(&self) -> Position {
        Position {
            column: self.column + 1,
            ..*self
        }
    }

    pub fn down(&self) -> Position {
        Position {
            row: self.row + 1,
            ..*self
        }
    }

    pub fn left(&self) -> Position {
        Position {
            column: self.column - 1,
            ..*self
        }
    }

    pub fn adjacent(&self) -> Vec<Position> {
        vec![self.up(), self.right(), self.down(), self.left()]
    }

    pub fn is_adjacent_to(&self, other: &Position) -> bool {
        let delta_row = self.row.abs_diff(other.row);
        let delta_column = self.column.abs_diff(other.column);
        delta_row + delta_column == 1
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.column)
    }
}