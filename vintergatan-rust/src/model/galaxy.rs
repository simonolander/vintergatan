use std::cmp::{max, min};
use std::collections::HashSet;

use petgraph::algo::connected_components;
use petgraph::graphmap::UnGraphMap;

use crate::model::position::Position;
use crate::model::rectangle::Rectangle;

#[derive(Clone)]
pub struct Galaxy {
    positions: HashSet<Position>,
}

/// A galaxy is a set of positions. A valid galaxy needs to satisfy the following conditions:
/// - It must not be empty
/// - It must be connected
/// - It must contain its center
/// - It must be rotationally symmetric
impl Galaxy {
    pub fn new(positions: impl IntoIterator<Item=Position>) -> Galaxy {
        Galaxy { positions: positions.into_iter().collect() }
    }

    /// Returns the center of this galaxy is half-steps.
    /// For example, the center of a galaxy containing [(0, 0)] is (0, 0),
    /// the center of a galaxy containing [(0, 0), (0, 1)] is (0, 1),
    /// and the center of a galaxy containing [(0, 1)] is (0, 2).
    ///
    /// If the galaxy is empty, (0, 0) is returned.
    pub fn center(&self) -> Position {
        let rect = self.bounding_rectangle();
        let center_half_row = rect.min_row + rect.max_row;
        let center_half_column = rect.min_column + rect.max_column;
        Position::new(center_half_row, center_half_column)
    }

    /// Returns the smallest rectangle that contains the galaxy.
    pub fn bounding_rectangle(&self) -> Rectangle {
        self.positions.iter().fold(
            Option::<Rectangle>::default(),
            |acc, p| match acc {
                None => Some(Rectangle {
                    min_row: p.row,
                    max_row: p.row,
                    min_column: p.column,
                    max_column: p.column,
                }),
                Some(rect) => Some(Rectangle::new(
                    min(p.row, rect.min_row),
                    max(p.row, rect.max_row),
                    min(p.column, rect.min_column),
                    max(p.column, rect.max_column),
                ))
            },
        ).unwrap_or(Rectangle::default())
    }

    pub fn mirror_position(&self, p: &Position) -> Position {
        let center = self.center();
        let mirrored_row = center.row - p.row;
        let mirrored_column = center.column - p.column;
        Position::new(mirrored_row, mirrored_column)
    }

    pub fn contains_position(&self, p: &Position) -> bool {
        self.positions.contains(p)
    }

    pub fn is_symmetric(&self) -> bool {
        self.positions.iter().all(|p| self.contains_position(&self.mirror_position(p)))
    }

    pub fn is_connected(&self) -> bool {
        if self.positions.is_empty() {
            return true;
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

    pub fn contains_center(&self) -> bool {
        let center = self.center();
        let rows = if center.row % 2 == 0 {
            vec![center.row / 2]
        } else {
            vec![center.row / 2, center.row / 2 + 1]
        };
        let columns = if center.column % 2 == 0 {
            vec![center.column / 2]
        } else {
            vec![center.column / 2, center.column / 2 + 1]
        };
        for &row in &rows {
            for &col in &columns {
                let p = Position::new(row, col);
                if !self.contains_position(&p) {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_valid(&self) -> bool {
        !self.positions.is_empty() && self.contains_center() && self.is_connected() && self.is_symmetric()
    }

    pub fn with_position(&self, p: &Position) -> Galaxy {
        let mut g = self.clone();
        g.positions.insert(*p);
        g
    }

    pub fn without_position(&self, p: &Position) -> Galaxy {
        let mut g = self.clone();
        g.positions.remove(p);
        g
    }

    ///
    pub fn rectangles(&self) -> Vec<Rectangle> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::model::galaxy::Galaxy;
    use crate::model::position::Position;

    fn galaxy(positions: &[(i32, i32)]) -> Galaxy {
        Galaxy::new(positions.iter().map(|&(row, col)| Position::new(row, col)))
    }

    #[test]
    fn test_center() {
        assert_eq!(Position::new(0, 0), galaxy(&[(0, 0)]).center());
        assert_eq!(Position::new(0, 1), galaxy(&[(0, 0), (0, 1)]).center());
        assert_eq!(Position::new(0, 2), galaxy(&[(0, 1)]).center());
        assert_eq!(Position::new(1, 0), galaxy(&[(0, 0), (1, 0)]).center());
        assert_eq!(Position::new(2, 0), galaxy(&[(1, 0)]).center());
        assert_eq!(Position::new(0, 2), galaxy(&[(0, 0), (0, 1), (0, 2)]).center());
        assert_eq!(Position::new(14, 6), galaxy(&[(6, 3), (7, 3), (8, 3)]).center());
        assert_eq!(Position::new(14, 7), galaxy(&[(6, 3), (7, 3), (7, 4), (8, 4)]).center());
        assert_eq!(Position::new(1, 1), galaxy(&[(0, 0), (0, 1), (1, 0), (1, 1)]).center());
    }

    #[test]
    fn test_mirror_position() {
        assert_eq!(Position::new(0, 0), galaxy(&[(0, 0)]).center());
        assert_eq!(Position::new(0, 1), galaxy(&[(0, 0), (0, 1)]).center());
        assert_eq!(Position::new(0, 2), galaxy(&[(0, 1)]).center());
        assert_eq!(Position::new(1, 0), galaxy(&[(0, 0), (1, 0)]).center());
        assert_eq!(Position::new(2, 0), galaxy(&[(1, 0)]).center());
        assert_eq!(Position::new(0, 2), galaxy(&[(0, 0), (0, 1), (0, 2)]).center());
        assert_eq!(Position::new(14, 6), galaxy(&[(6, 3), (7, 3), (8, 3)]).center());
        assert_eq!(Position::new(14, 7), galaxy(&[(6, 3), (7, 3), (7, 4), (8, 4)]).center());
        assert_eq!(Position::new(1, 1), galaxy(&[(0, 0), (0, 1), (1, 0), (1, 1)]).center());
    }
}