use std::fmt::{Display, Formatter};

use js_sys::Math;
use petgraph::graphmap::UnGraphMap;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
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
        let row = random_i32(0, height as i32);
        let column = random_i32(0, width as i32);
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
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.column)
    }
}

impl From<(usize, usize)> for Position {
    fn from((row, column): (usize, usize)) -> Self {
        Position::new(row as i32, column as i32)
    }
}

fn random_bool() -> bool {
    Math::random() < 0.5
}

fn random_f64(lower_bound: f64, upper_bound: f64) -> f64 {
    (Math::random() * (upper_bound - lower_bound)) + lower_bound
}

fn random_i32(lower_bound: i32, upper_bound: i32) -> i32 {
    random_f64(lower_bound as f64, upper_bound as f64) as i32
}

fn random_usize(lower_bound: usize, upper_bound: usize) -> usize {
    random_f64(lower_bound as f64, upper_bound as f64) as usize
}

fn random_element<T: Clone>(v: Vec<T>) -> Option<T> {
    v.get(random_usize(0, v.len())).cloned()
}

#[wasm_bindgen]
pub struct Universe {
    width: usize,
    height: usize,
    graph: UnGraphMap<Position, ()>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 40;
        let height = 40;
        let mut graph: UnGraphMap<Position, ()> = UnGraphMap::new();
        for row in 0..height {
            for column in 0..width {
                graph.add_node(Position::from((row, column)));
            }
        }
        Universe {
            width,
            height,
            graph,
        }
    }

    pub fn generate() -> Universe {
        let mut universe = Universe::new();
        for _ in 0..(universe.width * universe.height) {
            universe.generate_step();
        }
        universe
    }

    pub fn generate_step(&mut self) -> bool {
        let p1 = Position::random(self.width, self.height);
        console_log!("p1: {}", p1);
        if let Some(p2) = random_element(self.adjacent_non_neighbours(&p1)) {
            self.graph.add_edge(p1, p2, ());
            console_log!("p2: {}", p2);
            true
        } else {
            false
        }
    }

    pub fn adjacent_positions(&self, p: &Position) -> Vec<Position> {
        vec![p.left(), p.up(), p.right(), p.down()]
            .iter()
            .copied()
            .filter(|&adjacent_position| self.graph.contains_node(adjacent_position))
            .collect()
    }

    pub fn adjacent_non_neighbours(&self, p: &Position) -> Vec<Position> {
        self.adjacent_positions(p).iter().copied().filter(|adjacent_position| !self.are_neighbours(p, adjacent_position)).collect()
    }

    pub fn are_neighbours(&self, p1: &Position, p2: &Position) -> bool {
        self.graph.contains_edge(*p1, *p2)
    }

    pub fn is_outside(&self, p: &Position) -> bool {
        self.graph.contains_node(*p)
    }

    pub fn is_inside(&self, p: &Position) -> bool {
        !self.is_outside(p)
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..=self.height {
            for column in 0..=self.width {
                let bottom_right = Position::from((row, column));
                let bottom_left = bottom_right.left();
                let top_left = bottom_left.up();
                let top_right = bottom_right.up();

                let bar_top = row != 0 && !self.are_neighbours(&top_left, &top_right);
                let bar_right = column != self.width && !self.are_neighbours(&top_right, &bottom_right);
                let bar_bottom = row != self.height && !self.are_neighbours(&bottom_left, &bottom_right);
                let bar_left = column != 0 && !self.are_neighbours(&top_left, &bottom_left);
                match (bar_top, bar_right, bar_bottom, bar_left) {
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