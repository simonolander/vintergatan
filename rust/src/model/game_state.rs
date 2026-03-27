use crate::model::board::Board;
use crate::model::board_error::BoardError;
use crate::model::border::Border;
use crate::model::history::{History, HistoryEntry};
use crate::model::objective::{GalaxyCenter, Objective};
use crate::model::position::Position;
use crate::model::solver::Solver;
use crate::model::universe::Universe;
use rand::prelude::IteratorRandom;
use serde::Serialize;
use std::collections::BTreeSet;
use ts_rs::TS;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsValue, UnwrapThrowExt};
use HistoryEntry::ToggleBorder;

const GENERATE_SOLVED: bool = false;

#[wasm_bindgen]
pub struct GameState {
    /// The universe as it was generated, used for providing hints
    #[wasm_bindgen(skip)]
    pub universe: Universe,
    /// The current board state
    #[wasm_bindgen(skip)]
    pub board: Board,
    /// The set of centers and walls that the player needs to solve for
    #[wasm_bindgen(skip)]
    pub objective: Objective,
    /// The errors of the current board state, None means that the player isn't done solving
    #[wasm_bindgen(skip)]
    pub error: Option<BoardError>,
    /// History of board states
    #[wasm_bindgen(skip)]
    pub history: History,
}

#[wasm_bindgen]
impl GameState {
    pub fn generate(size: usize) -> GameState {
        let universe = Universe::generate(size, size);
        let objective = Solver::generate_objective(&universe);
        let mut board = Board::new(size, size);
        for border in objective.get_active_borders() {
            board.add_wall(border.p1(), border.p2());
        }
        let error = None;
        let history = History::new();

        if GENERATE_SOLVED {
            for border in universe.get_galaxies().iter().flat_map(|g| g.get_borders()) {
                let p1 = border.p1();
                let p2 = border.p2();
                if board.contains(&p1) && board.contains(&p2) {
                    board.add_wall(p1, p2);
                }
            }
        }
        // let mut solver = Solver::new(size, size, &objective);
        // let solution = solver.solve().unwrap();
        // for border in solution.borders {
        //     if board.contains(&border.p1()) && board.contains(&border.p2()) {
        //         board.add_wall(border.p1(), border.p2());
        //     }
        // }

        GameState {
            universe,
            board,
            objective,
            error,
            history,
        }
    }

    pub fn get_view(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&StateView::from(self)).unwrap_throw()
    }

    pub fn toggle_border(&mut self, r1: i32, c1: i32, r2: i32, c2: i32) {
        let p1 = Position::new(r1, c1);
        let p2 = Position::new(r2, c2);
        let border = Border::new(p1, p2);
        self.board.toggle_wall(p1, p2);
        self.history.push(ToggleBorder(border));
        self.error = None;
    }

    pub fn check_solution(&mut self) {
        self.objective
            .borders
            .iter()
            .for_each(|(border, &active)| {
                if active {
                    self.board.add_border(*border)
                } else {
                    self.board.remove_border(border)
                }
            });
        self.error = self.board.compute_error(&self.objective).into();
    }

    pub fn undo(&mut self) {
        if let Some(entry) = self.history.undo() {
            match entry {
                ToggleBorder(border) => self.board.toggle_wall(border.p1(), border.p2()),
            };
            self.error = None;
        }
    }

    pub fn redo(&mut self) {
        if let Some(entry) = self.history.redo() {
            match entry {
                ToggleBorder(border) => self.board.toggle_wall(border.p1(), border.p2()),
            };
            self.error = None;
        }
    }

    pub fn take_hint(&mut self) {
        let correct_borders = self.universe.get_interior_borders();
        let active_borders: BTreeSet<Border> = self.board.get_interior_borders().collect();
        let mut rng = rand::thread_rng();

        if let Some(incorrect_active_border) = active_borders
            .iter()
            .filter(|b| !correct_borders.contains(b))
            .choose(&mut rng)
        {
            self.board
                .remove_wall(incorrect_active_border.p1(), incorrect_active_border.p2());
            self.objective
                .borders
                .insert(*incorrect_active_border, false);
            self.error = None;
        } else if let Some(incorrect_inactive_border) = correct_borders
            .iter()
            .filter(|b| !active_borders.contains(b))
            .choose(&mut rng)
        {
            self.board.add_wall(
                incorrect_inactive_border.p1(),
                incorrect_inactive_border.p2(),
            );
            self.objective
                .borders
                .insert(*incorrect_inactive_border, true);
            self.error = None;
        }
    }

    pub fn objective_to_string(&self) -> String {
        self.objective.to_string()
    }

    pub fn board_to_string(&self) -> String {
        self.board.to_string()
    }

    pub fn universe_to_string(&self) -> String {
        self.universe.to_string()
    }
}

/// The parts of the state necessary for rendering
#[derive(Serialize, TS)]
#[ts(export)]
pub struct StateView {
    pub vertical_borders: Vec<Vec<bool>>,
    pub horizontal_borders: Vec<Vec<bool>>,
    pub objective: ObjectiveView,
    pub error: Option<BoardError>,
    pub has_future: bool,
    pub has_past: bool,
    pub is_solved: bool,
}

impl From<&GameState> for StateView {
    fn from(state: &GameState) -> Self {
        StateView {
            vertical_borders: state.board.get_vertical_borders(),
            horizontal_borders: state.board.get_horizontal_borders(),
            objective: (&state.objective).into(),
            error: state.error.clone(),
            has_future: state.history.has_future(),
            has_past: state.history.has_past(),
            is_solved: state
                .error
                .as_ref()
                .map(|it| it.is_error_free())
                .unwrap_or(false),
        }
    }
}

#[derive(Serialize, TS)]
#[ts(export)]
pub struct ObjectiveView {
    pub centers: BTreeSet<GalaxyCenter>,
    pub active_borders: BTreeSet<Border>,
    pub inactive_borders: BTreeSet<Border>,
}

impl From<&Objective> for ObjectiveView {
    fn from(objective: &Objective) -> Self {
        let mut active_borders = BTreeSet::new();
        let mut inactive_borders = BTreeSet::new();
        for (&border, &active) in &objective.borders {
            if active {
                active_borders.insert(border);
            } else {
                inactive_borders.insert(border);
            }
        }
        ObjectiveView {
            centers: objective.centers.clone(),
            active_borders,
            inactive_borders,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::game_state::{GameState, StateView};

    #[test]
    fn should_generate_state() {
        let state = GameState::generate(10);
        StateView::from(&state);
    }
}
