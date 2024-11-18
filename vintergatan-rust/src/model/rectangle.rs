use crate::model::position::Position;

#[derive(Eq, PartialEq, Default)]
pub struct Rectangle {
    pub min_row: i32,
    pub max_row: i32,
    pub min_column: i32,
    pub max_column: i32,
}

impl Rectangle {
    pub fn new(
        min_row: i32,
        max_row: i32,
        min_column: i32,
        max_column: i32,
    ) -> Rectangle {
        Rectangle {
            min_row,
            max_row,
            min_column,
            max_column,
        }
    }

    pub fn width(&self) -> i32 {
        self.max_column - self.min_column
    }

    pub fn height(&self) -> i32 {
        self.max_row - self.min_row
    }

    pub fn area(&self) -> i32 {
        self.width() * self.height()
    }

    pub fn positions(&self) -> Vec<Position> {
        (self.min_row..self.max_row).flat_map(
            |row| (self.min_column..self.max_column).map(
                move |column| Position::new(row, column)
            )
        ).collect()
    }
}