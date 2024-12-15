use crate::model::galaxy::Galaxy;
use crate::model::position::Position;
use petgraph::graphmap::UnGraphMap;
use petgraph::visit::{Dfs, Walker};
use rand::prelude::SliceRandom;
use rand::rngs::StdRng;
use rand::{random, Rng, SeedableRng};
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct Universe {
    width: usize,
    height: usize,
    graph: UnGraphMap<Position, ()>,
}

impl Universe {
    pub fn new(width: usize, height: usize) -> Universe {
        let mut graph: UnGraphMap<Position, ()> = UnGraphMap::new();
        for row in 0..height {
            for column in 0..width {
                let position = Position::from((row, column));
                graph.add_node(position);
            }
        }
        Universe {
            width,
            height,
            graph,
        }
    }

    pub fn generate(width: usize, height: usize) -> Universe {
        let mut universe = Universe::new(width, height);
        let iterations = width * height * 10;
        let branches = 5;
        let seed: u64 = random();
        println!("Seed: {}", seed);
        let mut rng = StdRng::seed_from_u64(16930399551136634216);
        for _iteration in 0..iterations {
            let mut next_universes: Vec<Universe> =
                (0..branches).map(|_| universe.clone()).collect();
            for next_universe in next_universes.iter_mut() {
                next_universe.generate_step(&mut rng);
            }
            universe = next_universes
                .into_iter()
                .min_by_key(|universe| universe.get_score())
                .unwrap_or(universe);
        }
        universe
    }

    fn generate_step(&mut self, rng: &mut impl Rng) -> bool {
        // First we pick a random position in the universe
        let p1 = self.random_position(rng);
        let g1 = self.get_galaxy(&p1);

        // Then we pick one of the adjacent positions that is not already a neighbour. If there isn't one, we abort
        let p2_option = self.adjacent_non_neighbours(&p1).choose(rng).cloned();
        if p2_option.is_none() {
            return false;
        }
        let p2 = p2_option.unwrap();

        // If removing p2 from g2 makes the galaxy disconnected, abort
        let g2 = self.get_galaxy(&p2);
        let g2_without_p2 = g2.without_position(&p2);
        if !g2_without_p2.is_connected() {
            return false;
        }
        let p5 = g2.mirror_position(&p2);
        if !g2_without_p2.is_symmetric() && !g2_without_p2.without_position(&p5).is_connected() {
            return false;
        }

        let g1_with_p2 = g1.with_position(&p2);
        // If g1 is not symmetric, we need to do some additional work to make it so
        if !g1_with_p2.is_symmetric() {
            let p3 = g1.mirror_position(&p2);
            // If p3 is outside the universe, abort
            if self.is_outside(&p3) {
                return false;
            }
            let g3 = self.get_galaxy(&p3);
            let g3_without_p3 = g3.without_position(&p3);
            if !g3_without_p3.is_connected() {
                return false;
            }
            if !g3_without_p3.is_symmetric() {
                let p4 = g3.mirror_position(&p3);
                if !g3_without_p3.without_position(&p4).is_connected() {
                    return false;
                }
                assert!(
                    g3.contains_position(&p4),
                    "assertion failed: galaxy of {} should contain {}:\n{}",
                    p3,
                    p4,
                    self
                );
                self.remove_all_neighbours(&p4);
            }
            self.make_neighbours(&p1, &p3);
        }
        if !g2_without_p2.is_symmetric() {
            let p5 = g2.mirror_position(&p2);
            self.remove_all_neighbours(&p5);
        }
        self.make_neighbours(&p1, &p2);
        for p in self.get_positions() {
            let g = self.get_galaxy(&p);
            assert!(
                g.is_valid(),
                "assertion failed: galaxy of {} is invalid:\n{}",
                p,
                self
            );
        }
        true
    }

    /// Returns a list of galaxies in this universe, in no particular order
    pub fn get_galaxies(&self) -> Vec<Galaxy> {
        let mut galaxies: Vec<Galaxy> = Vec::new();
        let mut remaining_positions: BTreeSet<Position> = self.graph.nodes().collect();
        while let Some(position) = remaining_positions.first() {
            let galaxy = self.get_galaxy(position);
            for p in galaxy.get_positions() {
                remaining_positions.remove(p);
            }
            galaxies.push(galaxy);
        }
        galaxies
    }

    /// Make p have no neighbours
    pub fn remove_all_neighbours(&mut self, p: &Position) {
        for adjacent_position in self.adjacent_positions(p) {
            self.graph.remove_edge(*p, adjacent_position);
        }
    }

    /// Metric of how "cool" is the universe is. Lower is better.
    pub fn get_score(&self) -> i64 {
        let mut score: i64 = 0;

        // Add points for long, straight, horizontal borders
        for row in 1..self.height as i32 {
            let mut current_length: i64 = 0;
            for col in 0..self.width as i32 {
                let up = Position::new(row - 1, col);
                let down = Position::new(row, col);
                if self.are_neighbours(&up, &down) {
                    score += current_length.pow(2);
                    current_length = 0;
                } else {
                    current_length += 1;
                }
            }
            score += current_length.pow(2);
        }

        // Add points for long, straight, vertical borders
        for col in 1..self.width as i32 {
            let mut current_length: i64 = 0;
            for row in 0..self.height as i32 {
                let left = Position::new(row, col - 1);
                let right = Position::new(row, col);
                if self.are_neighbours(&left, &right) {
                    score += current_length.pow(2);
                    current_length = 0;
                } else {
                    current_length += 1;
                }
            }
            score += current_length.pow(2);
        }

        // Add points for big rectangles
        for galaxy in self.get_galaxies() {
            for rect in galaxy.rectangles() {
                let area = rect.area() as i64;
                score += area.pow(2);
            }
        }

        score
    }

    pub fn add_galaxy(&mut self, galaxy: &Galaxy) {
        for p1 in galaxy.get_positions() {
            for p2 in &self.adjacent_positions(p1) {
                if galaxy.contains_position(p2) {
                    self.graph.add_edge(*p1, *p2, ());
                }
            }
        }
    }

    /**
     * Joins p2 into the galaxy of p1, removing any previous edges from p2,
     * and adding edges to all neighbouring positions in the galaxy of p1.
     * Returns whether p1 and p2 were successfully made neighbours.
     */
    pub fn make_neighbours(&mut self, p1: &Position, p2: &Position) {
        let g1 = self.get_galaxy(p1);
        for p2_adjacent in self.adjacent_positions(p2) {
            if g1.contains_position(&p2_adjacent) {
                self.graph.add_edge(*p2, p2_adjacent, ());
            } else {
                self.graph.remove_edge(*p2, p2_adjacent);
            }
        }
    }

    pub fn random_position(&self, rng: &mut impl Rng) -> Position {
        Position::random(self.width, self.height, rng)
    }

    pub fn adjacent_positions(&self, p: &Position) -> Vec<Position> {
        vec![p.left(), p.up(), p.right(), p.down()]
            .iter()
            .copied()
            .filter(|&adjacent_position| self.graph.contains_node(adjacent_position))
            .collect()
    }

    pub fn adjacent_non_neighbours(&self, p: &Position) -> Vec<Position> {
        self.adjacent_positions(p)
            .iter()
            .copied()
            .filter(|adjacent_position| !self.are_neighbours(p, adjacent_position))
            .collect()
    }

    pub fn are_neighbours(&self, p1: &Position, p2: &Position) -> bool {
        self.graph.contains_edge(*p1, *p2)
    }

    pub fn get_galaxy(&self, p: &Position) -> Galaxy {
        let search = Dfs::new(&self.graph, *p);
        Galaxy::from_positions(search.iter(&self.graph))
    }

    pub fn is_outside(&self, p: &Position) -> bool {
        !self.is_inside(p)
    }

    pub fn is_inside(&self, p: &Position) -> bool {
        self.graph.contains_node(*p)
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn get_positions(&self) -> impl Iterator<Item = Position> + '_ {
        (0..self.height)
            .flat_map(|row| (0..self.width).map(move |col| (row, col)))
            .map(|t| Position::from(t))
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
                let bar_right =
                    column != self.width && !self.are_neighbours(&top_right, &bottom_right);
                let bar_bottom =
                    row != self.height && !self.are_neighbours(&bottom_left, &bottom_right);
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
