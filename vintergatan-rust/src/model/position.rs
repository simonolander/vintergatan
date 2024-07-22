use std::fmt::{Display, Formatter};

use rand::Rng;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct Position {
    pub row: i32,
    pub column: i32,
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

    /// Returns true iff other is directly above, below, to the left, or to the right of this position.
    pub fn is_adjacent_to(&self, other: &Position) -> bool {
        let delta_row = self.row.abs_diff(other.row);
        let delta_column = self.column.abs_diff(other.column);
        delta_row.checked_add(delta_column).map(|it| it == 1).unwrap_or(false)
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

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use proptest::prelude::*;

    use crate::model::position::Position;

    fn prop_assert_eq_vec_orderless<T: Eq + Debug>(left: Vec<T>, right: Vec<T>) -> Result<(), TestCaseError> {
        prop_assert_eq!(left.len(), right.len(), "assertion failed: `(left.len() == right.len())`\n  left: `{:?}`,\n right: `{:?}`", left, right);
        for item in &left {
            let left_count = left.iter().filter(|&it| it == item).count();
            let right_count = right.iter().filter(|&it| it == item).count();
            prop_assert_eq!(left_count, right_count, "assertion failed: `(left.count({:?}) == right.count({:?}))`\n  left: `{:?}`,\n right: `{:?}`", item, item, left, right);
        }
        Ok(())
    }

    proptest! {
        #[test]
        fn test_fmt(row: i32, col: i32) {
            let p = Position::new(row, col);
            let expected = format!("({}, {})", row, col);
            prop_assert_eq!(expected, p.to_string());
        }

        #[test]
        fn test_up(row: i32, col: i32) {
            prop_assume!(row != i32::MIN);
            let p = Position::new(row, col);
            let expected = Position::new(row - 1, col);
            prop_assert_eq!(expected, p.up());
        }

        #[test]
        fn test_right(row: i32, col: i32) {
            prop_assume!(col != i32::MAX);
            let p = Position::new(row, col);
            let expected = Position::new(row, col + 1);
            prop_assert_eq!(expected, p.right());
        }

        #[test]
        fn test_down(row: i32, col: i32) {
            prop_assume!(row != i32::MAX);
            let p = Position::new(row, col);
            let expected = Position::new(row + 1, col);
            prop_assert_eq!(expected, p.down());
        }

        #[test]
        fn test_left(row: i32, col: i32) {
            prop_assume!(col != i32::MIN);
            let p = Position::new(row, col);
            let expected = Position::new(row, col - 1);
            prop_assert_eq!(expected, p.left());
        }

        #[test]
        fn test_random(width in 1..i32::MAX, height in 1..i32::MAX) {
            let p = Position::random(width as usize, height as usize);
            prop_assert!(p.column >= 0);
            prop_assert!(p.column < width);
            prop_assert!(p.row >= 0);
            prop_assert!(p.row < height);
        }

        #[test]
        fn test_adjacent(row: i32, col: i32) {
            prop_assume!(row != i32::MIN && row != i32::MAX);
            prop_assume!(col != i32::MIN && col != i32::MAX);
            let p = Position::new(row, col);
            let adjacent = p.adjacent();
            prop_assert_eq_vec_orderless(adjacent, vec![p.left(), p.up(), p.down(), p.right()])?;
        }

        #[test]
        fn test_is_adjacent_to_should_not_be_adjacent_to_self(row: i32, col: i32) {
            let p = Position::new(row, col);
            prop_assert!(!p.is_adjacent_to(&p));
        }

        #[test]
        fn test_is_adjacent_to_should_be_adjacent_to_left_up_down_right(row: i32, col: i32) {
            prop_assume!(row != i32::MIN && row != i32::MAX);
            prop_assume!(col != i32::MIN && col != i32::MAX);
            let p = Position::new(row, col);
            prop_assert!(p.is_adjacent_to(&p.left()));
            prop_assert!(p.is_adjacent_to(&p.up()));
            prop_assert!(p.is_adjacent_to(&p.down()));
            prop_assert!(p.is_adjacent_to(&p.right()));
        }

        #[test]
        fn test_is_adjacent_to_should_not_be_adjacent_to_anything_but_left_up_down_right(r1: i32, c1: i32, r2: i32, c2: i32) {
            prop_assume!(r1 != i32::MIN && r1 != i32::MAX);
            prop_assume!(c1 != i32::MIN && c1 != i32::MAX);
            prop_assume!(r2 != i32::MIN && r2 != i32::MAX);
            prop_assume!(c2 != i32::MIN && c2 != i32::MAX);
            let p1 = Position::new(r1, c1);
            let p2 = Position::new(r2, c2);
            prop_assume!(p2 != p1.left());
            prop_assume!(p2 != p1.up());
            prop_assume!(p2 != p1.right());
            prop_assume!(p2 != p1.down());
            prop_assert!(!p1.is_adjacent_to(&p2));
        }

        #[test]
        fn test_is_adjacent_is_symmetric(r1: i32, c1: i32, r2: i32, c2: i32) {
            let p1 = Position::new(r1, c1);
            let p2 = Position::new(r2, c2);
            prop_assert_eq!(p1.is_adjacent_to(&p2), p2.is_adjacent_to(&p1));
        }

        #[test]
        fn test_from_usize_usize(row: i32, col: i32) {
            let tuple = (row as usize, col as usize);
            prop_assert_eq!(Position::from(tuple), Position::new(row, col));
        }
    }
}
