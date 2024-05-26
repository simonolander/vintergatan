use std::collections::HashSet;
use std::fmt::{Display, Formatter};

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Universe {
    width: usize,
    height: usize,
    neighbours: HashSet<(usize, usize)>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        Universe {
            width: 16,
            height: 16,
            neighbours: HashSet::new(),
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "┌")?;
        for _ in 0..self.width {
            write!(f, "┬")?;
        }
        write!(f, "┐\n")?;
        for _ in 0..self.height {
            write!(f, "├")?;
            for _ in 0..self.width {
                write!(f, "┼")?;
            }
            write!(f, "┤\n")?;
        }
        write!(f, "└")?;
        for _ in 0..self.width {
            write!(f, "┴")?;
        }
        write!(f, "┘\n")?;
        Ok(())
    }
}