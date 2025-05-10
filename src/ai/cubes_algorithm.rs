use super::super::board_state::BoardState;

pub trait CubesAlgorithm {
    fn pick_move(&self, _: BoardState) -> (usize, usize);
}
