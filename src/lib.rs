use std::fmt::{Display, Formatter};
use std::ops::AddAssign;

use wasm_bindgen::prelude::*;

use crate::Cell::{Alive, Dead};

mod utils;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, vintergatan!");
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl AddAssign<Cell> for u8 {
    fn add_assign(&mut self, cell: Cell) {
        *self = self.wrapping_add(cell as u8)
    }
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn get_column(&self, index: usize) -> u32 {
        index as u32 % self.width
    }

    fn get_row(&self, index: usize) -> u32 {
        index as u32 / self.width
    }

    fn live_neighbour_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1] {
            for delta_column in [self.width - 1, 0, 1] {
                if delta_row == 0 && delta_column == 0 {
                    continue;
                }
                let neighbour_row = (row + delta_row) % self.height;
                let neighbour_column = (column + delta_column) % self.width;
                let index = self.get_index(neighbour_row, neighbour_column);
                count += self.cells[index];
            }
        }
        count
    }

    fn live_neighbour_count_by_index(&self, index: usize) -> u8 {
        self.live_neighbour_count(self.get_row(index), self.get_column(index))
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;
        let cells = (0..width * height).map(|i| {
            if i % 2 == 0 || i % 7 == 0 {
                Alive
            } else { Dead }
        }).collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn tick(&mut self) {
        let mut next = Vec::new();

        for (index, cell) in self.cells.iter().enumerate() {
            let next_cell = match (cell, self.live_neighbour_count_by_index(index)) {
                (Alive, 2) => Alive,
                (_, 3) => Alive,
                _ => Dead,
            };
            next.push(next_cell);
        }

        self.cells = next;
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for cell in line {
                let symbol = match cell {
                    Dead => '◻',
                    Alive => '◼',
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}