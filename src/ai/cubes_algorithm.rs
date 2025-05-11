use crate::float_engine::float_state::FloatState;

pub trait CubesAlgorithm {
    fn pick_move(&self, _: FloatState) -> u64;
}
