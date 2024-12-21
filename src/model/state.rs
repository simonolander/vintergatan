use crate::model::board::Board;
use crate::model::board_error::BoardError;
use crate::model::objective::Objective;
use crate::model::universe::Universe;
use std::cell::BorrowError;

pub struct State {
    pub universe: Universe,
    pub board: Board,
    pub objective: Objective,
    pub error: BoardError,
}

impl State {
    pub fn generate(size: usize) -> State {
        let universe = Universe::generate(size, size);
        let objective = Objective::generate(&universe);
        let board = Board::new(size, size);
        let error = BoardError::default();

        State {
            universe,
            board,
            objective,
            error,
        }
    }
}
