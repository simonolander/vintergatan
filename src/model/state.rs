use crate::model::board::Board;
use crate::model::board_error::BoardError;
use crate::model::objective::Objective;
use crate::model::universe::Universe;

const GENERATE_SOLVED: bool = false;

pub struct State {
    pub universe: Universe,
    pub board: Board,
    pub objective: Objective,
    pub error: Option<BoardError>,
}

impl State {
    pub fn generate(size: usize) -> State {
        let universe = Universe::generate(size, size);
        let objective = Objective::generate(&universe);
        let mut board = Board::new(size, size);
        let error = Option::default();

        if GENERATE_SOLVED {
            for border in universe.get_galaxies().iter().flat_map(|g| g.get_borders()) {
                board.add_wall(border.p1(), border.p2());
            }
        }

        State {
            universe,
            board,
            objective,
            error,
        }
    }
}
