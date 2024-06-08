use std::cmp::{max, min};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

use js_sys::Math;
use petgraph::algo::connected_components;
use petgraph::graphmap::UnGraphMap;
use petgraph::visit::{Dfs, IntoEdges, Walker};
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
#[derive(Clone)]
pub struct Galaxy {
    positions: HashSet<Position>,
}

impl Galaxy {
    fn new(positions: impl IntoIterator<Item=Position>) -> Galaxy {
        Galaxy { positions: positions.into_iter().collect() }
    }

    fn center(&self) -> Position {
        #[derive(Default)]
        struct MinMax {
            min_row: i32,
            max_row: i32,
            min_column: i32,
            max_column: i32,
        }
        let option_min_max = self.positions.iter().fold(Option::<MinMax>::default(), |acc, p| match acc {
            None => Some(MinMax {
                min_row: p.row,
                max_row: p.row,
                min_column: p.column,
                max_column: p.column,
            }),
            Some(min_max) => Some(MinMax {
                min_row: min(p.row, min_max.min_row),
                max_row: max(p.row, min_max.min_row),
                min_column: min(p.column, min_max.min_column),
                max_column: max(p.column, min_max.min_column),
            })
        });

        if let Some(min_max) = option_min_max {
            let center_half_row = min_max.min_row + min_max.max_row;
            let center_half_column = min_max.min_column + min_max.max_column;
            Position::new(center_half_row, center_half_column)
        } else {
            Position::new(0, 0)
        }
    }

    fn mirror_position(&self, p: &Position) -> Position {
        let center = self.center();
        let mirrored_row = center.row - p.row;
        let mirrored_column = center.column - p.column;
        Position::new(mirrored_row, mirrored_column)
    }

    fn contains_position(&self, p: &Position) -> bool {
        self.positions.contains(p)
    }

    fn is_symmetric(&self) -> bool {
        self.positions.iter().all(|p| self.contains_position(&self.mirror_position(p)))
    }

    fn is_connected(&self) -> bool {
        if self.positions.is_empty() {
            return true
        }
        let mut graph = UnGraphMap::new();
        for p in self.positions.iter() {
            graph.add_node(*p);
            for adjacent in p.adjacent() {
                if self.contains_position(&adjacent) {
                    graph.add_edge(adjacent, *p, ());
                }
            }
        }
        connected_components(&graph) == 1
    }

    fn with_position(&self, p: &Position) -> Galaxy {
        let mut g = self.clone();
        g.positions.insert(*p);
        g
    }

    fn without_position(&self, p: &Position) -> Galaxy {
        let mut g = self.clone();
        g.positions.remove(p);
        g
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Universe {
    width: usize,
    height: usize,
    graph: UnGraphMap<Position, ()>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 10;
        let height = 10;
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
            let mut un = universe.clone();
            if (un.generate_step()) {
                universe = un;
            }
        }
        universe
    }

    pub fn generate_step(&mut self) -> bool {
        let p1 = self.random_position();
        let p1_galaxy = self.get_galaxy(&p1);
        let p2_option = random_element(self.adjacent_non_neighbours(&p1));
        if p2_option.is_none() {
            return false
        }
        let p2 = p2_option.unwrap();
        let p2_galaxy = self.get_galaxy(&p2);
        let p2_galaxy_without_p2 = p2_galaxy.without_position(&p2);
        if !p2_galaxy_without_p2.is_connected() {
            return false
        }
        let p1_galaxy_with_p2 = p1_galaxy.with_position(&p2);
        if !p1_galaxy_with_p2.is_symmetric() {
            let p3 = p1_galaxy.mirror_position(&p2);
            let p3_galaxy = self.get_galaxy(&p3);
            if !p3_galaxy.without_position(&p3).is_connected() {
                return false;
            }
            let p4 = p3_galaxy.mirror_position(&p3);
            self.remove_all_neighbours(&p4);
            self.make_neighbours(&p1, &p3);
        }
        if !p2_galaxy_without_p2.is_symmetric() {
            let p5 = p2_galaxy.mirror_position(&p2);
            self.remove_all_neighbours(&p5);
        }
        self.make_neighbours(&p1, &p2);
        true
    }

    pub fn remove_all_neighbours(&mut self, p: &Position) {
        for adjacent_position in self.adjacent_positions(p) {
            self.graph.remove_edge(*p, adjacent_position);
        }
    }

    /**
     * Joins p2 into the galaxy of p1, removing any previous edges from p2,
     * and adding edges to all neighbouring positions in the galaxy of p1.
     * Returns whether p1 and p2 were successfully made neighbours.
     */
    pub fn make_neighbours(&mut self, p1: &Position, p2: &Position) -> bool {
        if !p1.is_adjacent_to(p2) {
            return false;
        }
        if self.are_neighbours(p1, p2) {
            return false;
        }
        let p1_galaxy = self.get_galaxy(p1);
        for p2_adjacent in self.adjacent_positions(p2) {
            if p1_galaxy.contains_position(&p2_adjacent) {
                self.graph.add_edge(*p2, p2_adjacent, ());
            } else {
                self.graph.remove_edge(*p2, p2_adjacent);
            }
        }
        true
    }

    pub fn random_position(&self) -> Position {
        Position::random(self.width, self.height)
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

    pub fn get_galaxy(&self, p: &Position) -> Galaxy {
        let search = Dfs::new(&self.graph, *p);
        Galaxy::new(search.iter(&self.graph))
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