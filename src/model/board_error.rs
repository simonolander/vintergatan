use std::collections::HashSet;
use crate::model::border::Border;
use crate::model::position::Position;

#[derive(Debug, Default)]
pub struct BoardError {
    pub dangling_segments: HashSet<Border>,
    pub incorrect_galaxy_sizes: HashSet<Position>,
    pub centerless_cells: HashSet<Position>,
    pub asymmetric_centers: HashSet<Position>,
}