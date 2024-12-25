use crate::model::board_error::BoardError;
use crate::model::border::Border;
use crate::model::galaxy::Galaxy;
use crate::model::objective::Objective;
use crate::model::position::{CenterPlacement, Position};
use crate::model::universe::Universe;
use itertools::Itertools;
use petgraph::graphmap::UnGraphMap;
use petgraph::visit::{FilterEdge, Visitable};
use petgraph::Direction;
use std::collections::{BTreeSet, HashMap, HashSet};

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

    pub fn is_active(&self, border: &Border) -> bool {
        self.is_wall(border.p1(), border.p2())
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

    /// Toggles the wall between [p1] and [p2], returns true if there's a wall after the toggle
    pub fn toggle_wall(&mut self, p1: Position, p2: Position) -> bool {
        if self.is_wall(p1, p2) {
            self.remove_wall(p1, p2);
            false
        } else {
            self.add_wall(p1, p2);
            true
        }
    }

    pub fn get_borders(&self) -> impl Iterator<Item = Border> + use<'_> {
        self.graph.all_edges().map(|(p1, p2, _)| (p1, p2).into())
    }

    fn get_universe(&self) -> Universe {
        let mut universe = Universe::new(self.width, self.height);
        for p1 in self.graph.nodes() {
            for p2 in universe.adjacent_non_neighbours(&p1) {
                if !self.is_wall(p1, p2) {
                    universe.make_neighbours(&p1, &p2)
                }
            }
        }
        universe
    }

    pub fn compute_error(&self, objective: &Objective) -> BoardError {
        let dangling_segments = self.get_dangling_borders().collect();

        let universe = self.get_universe();
        let galaxies = universe.get_galaxies();
        let galaxy_by_center: HashMap<Position, &Galaxy> = galaxies
            .iter()
            .map(|galaxy| (galaxy.center(), galaxy))
            .collect();
        let galaxy_by_position: HashMap<Position, &Galaxy> = galaxies
            .iter()
            .flat_map(|galaxy| galaxy.get_positions().copied().map(move |p| (p, galaxy)))
            .collect();

        let cut_centers: HashSet<Position> = objective
            .centers
            .iter()
            .map(|gc| gc.position)
            .filter(|center| self.is_center_cut(center))
            .collect();

        let asymmetric_centers = Default::default();
        let incorrect_galaxy_sizes = Default::default();
        let centerless_cells = Default::default();

        BoardError {
            dangling_segments,
            incorrect_galaxy_sizes,
            centerless_cells,
            cut_centers,
            asymmetric_centers,
        }
    }

    fn is_center_cut(&self, center: &Position) -> bool {
        match center.get_center_placement() {
            CenterPlacement::Center(_) => false,
            CenterPlacement::VerticalBorder(border) => self.is_active(&border),
            CenterPlacement::HorizontalBorder(border) => self.is_active(&border),
            CenterPlacement::Intersection(rect) => {
                let top_left = Position::new(rect.min_row, rect.min_column);
                let top_right = Position::new(rect.min_row, rect.max_column);
                let bottom_left = Position::new(rect.max_row, rect.min_column);
                let bottom_right = Position::new(rect.max_row, rect.max_column);
                self.is_wall(top_left, top_right)
                    || self.is_wall(top_right, bottom_right)
                    || self.is_wall(bottom_right, bottom_left)
                    || self.is_wall(top_left, bottom_left)
            }
        }
    }

    fn get_dangling_borders(&self) -> impl Iterator<Item = Border> + use<'_> {
        self.get_borders().filter(|border| self.is_dangling(border))
    }

    fn is_dangling(&self, border: &Border) -> bool {
        let p1 = border.p1();
        let p2 = border.p2();
        if border.is_vertical() {
            // Check that the border connects to something above
            if p1.row != 0 {
                let p1_up = p1.up();
                let p2_up = p2.up();
                if !self.is_wall(p1, p1_up)
                    && !self.is_wall(p1_up, p2_up)
                    && !self.is_wall(p2_up, p2)
                {
                    return true;
                }
            }

            // Check that the border connects to something below
            if p1.row != (self.height - 1) as i32 {
                let p1_down = p1.down();
                let p2_down = p2.down();
                if !self.is_wall(p1, p1_down)
                    && !self.is_wall(p1_down, p2_down)
                    && !self.is_wall(p2_down, p2)
                {
                    return true;
                }
            }
        } else {
            // The border is horizontal

            // Check that the border connects to something to the left
            if p1.column != 0 {
                let p1_left = p1.left();
                let p2_left = p2.left();
                if !self.is_wall(p1, p1_left)
                    && !self.is_wall(p1_left, p2_left)
                    && !self.is_wall(p2_left, p2)
                {
                    return true;
                }
            }

            // Check that the border connects to something below
            if p1.column != (self.width - 1) as i32 {
                let p1_right = p1.right();
                let p2_right = p2.right();
                if !self.is_wall(p1, p1_right)
                    && !self.is_wall(p1_right, p2_right)
                    && !self.is_wall(p2_right, p2)
                {
                    return true;
                }
            }
        }

        false
    }
}
