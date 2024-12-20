use crate::model::position::Position;
use std::cmp::{max, min};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub struct Border {
    p1: Position,
    p2: Position,
}

impl Border {
    pub fn new(p1: Position, p2: Position) -> Self {
        Self {
            p1: min(p1, p2),
            p2: max(p1, p2),
        }
    }

    pub fn p1(&self) -> Position {
        self.p1
    }

    pub fn p2(&self) -> Position {
        self.p2
    }
}
