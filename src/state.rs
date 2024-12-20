use crate::model::board::Board;
use crate::model::universe::Universe;

#[derive(Debug, Clone, Default)]
pub enum State {
    #[default]
    Initial,
    Loading,
    Loaded(LoadedState),
}

impl State {
    pub fn loaded_state(&self) -> Option<&LoadedState> {
        if let State::Loaded(state) = self {
            Some(state)
        } else {
            None
        }
    }

    pub fn is_loaded(&self) -> bool {
        if let State::Loaded(_) = self {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone)]
pub struct LoadedState {
    pub universe: Universe,
    pub board: Board,
}

impl LoadedState {
    pub fn generate(size: usize) -> Self {
        let universe = Universe::generate(size, size);
        let board = Board::new(size, size);
        Self { universe, board }
    }
}
