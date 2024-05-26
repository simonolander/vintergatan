use std::collections::HashSet;
use std::fmt::{Display, Formatter};

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Universe {
    width: usize,
    height: usize,
    neighbours: HashSet<(usize, usize, usize, usize)>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 4;
        let height = 4;
        let mut neighbours = HashSet::new();
        neighbours.insert((0usize, 0usize, 0usize, 1usize));
        neighbours.insert((0usize, 0usize, 1usize, 0usize));
        neighbours.insert((1usize, 0usize, 1usize, 1usize));
        Universe {
            width,
            height,
            neighbours,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..=self.height {
            for column in 0..=self.width {
                let top = match (row, column) {
                    (0, _) => false,
                    (_, 0) => true,
                    (_, c) if c == self.width => true,
                    (r, c) => !self.neighbours.contains(&(r - 1, c - 1, r - 1, c))
                };
                let right = match (row, column) {
                    (_, c) if c == self.width => false,
                    (0, _) => true,
                    (r, _) if r == self.height => true,
                    (r, c) => !self.neighbours.contains(&(r - 1, c, r, c))
                };
                let bottom = match (row, column) {
                    (r, _) if r == self.height => false,
                    (_, 0) => true,
                    (_, c) if c == self.width => true,
                    (r, c) => !self.neighbours.contains(&(r, c - 1, r, c))
                };
                let left = match (row, column) {
                    (_, 0) => false,
                    (0, _) => true,
                    (r, _) if r == self.height => true,
                    (r, c) => !self.neighbours.contains(&(r - 1, c - 1, r, c - 1)),
                };
                match (top, right, bottom, left) {
                    (false, false, false, false) => write!(f, "  ")?,
                    (false, false, false, true) => write!(f, "╴ ")?,
                    (false, false, true, false) => write!(f, "╷ ")?,
                    (false, false, true, true) => write!(f, "┐ ")?,
                    (false, true, false, false) => write!(f, "╶─")?,
                    (false, true, false, true) => write!(f, "──")?,
                    (false, true, true, false) => write!(f, "┌─")?,
                    (false, true, true, true) => write!(f, "┬─")?,
                    (true, false, false, false) => write!(f, "╵ ")?,
                    (true, false, false, true) => write!(f, "┘ ")?,
                    (true, false, true, false) => write!(f, "│ ")?,
                    (true, false, true, true) => write!(f, "┤ ")?,
                    (true, true, false, false) => write!(f, "└─")?,
                    (true, true, false, true) => write!(f, "┴─")?,
                    (true, true, true, false) => write!(f, "├─")?,
                    (true, true, true, true) => write!(f, "┼─")?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}