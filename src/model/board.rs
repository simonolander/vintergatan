use crate::model::position::Position;
use petgraph::graphmap::UnGraphMap;

#[derive(Clone, Debug)]
pub struct Board {
    width: usize,
    height: usize,
    graph: UnGraphMap<Position, ()>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Board {
            width,
            height,
            graph: Default::default(),
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.width
    }

    fn contains(&self, position: &Position) -> bool {
        position.row >= 0
            && position.row < self.height as i32
            && position.column >= 0
            && position.column < self.width as i32
    }

    /// Adds a wall between [p1] and [p2], returns true if the wall did not previously exist
    pub fn add_wall(&mut self, p1: Position, p2: Position) -> bool {
        debug_assert!(p1.is_adjacent_to(&p2));
        debug_assert!(self.contains(&p1));
        debug_assert!(self.contains(&p2));
        let result = self.graph.add_edge(p1, p2, ());
        result.is_none()
    }

    /// Removes the wall between [p1] and [p2], if it exists. Returns true if the wall existed
    pub fn remove_wall(&mut self, p1: Position, p2: Position) -> bool {
        debug_assert!(p1.is_adjacent_to(&p2));
        debug_assert!(self.contains(&p1));
        debug_assert!(self.contains(&p2));
        let result = self.graph.remove_edge(p1, p2);
        result.is_some()
    }

    /// Returns whether there is a wall between p1 and p2
    pub fn is_wall(&self, p1: Position, p2: Position) -> bool {
        debug_assert!(p1.is_adjacent_to(&p2));
        debug_assert!(self.contains(&p1));
        debug_assert!(self.contains(&p2));
        self.graph.contains_edge(p1, p2)
    }

    /// Toggles the wall between [p1] and [p2]
    pub fn toggle_wall(&mut self, p1: Position, p2: Position) {
        if self.is_wall(p1, p2) {
            self.remove_wall(p1, p2);
        } else {
            self.add_wall(p1, p2);
        }
    }

    pub fn get_walls(&self) -> Vec<(Position, Position)> {
        self.graph
            .all_edges()
            .map(|edge| (edge.0, edge.1))
            .collect()
    }
}
