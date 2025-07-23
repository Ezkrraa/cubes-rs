use crate::bit_engine::bit_state::FloatState;

pub trait CubesAlgorithm {
    fn pick_move(&self, _: FloatState) -> u64;
}
