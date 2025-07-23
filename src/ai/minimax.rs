use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::bit_engine::{bit_state::FloatState, field::Field};

use super::cubes_algorithm::CubesAlgorithm;

const STANDARD_DEPTH: i32 = 5;
// static mut TOTAL_PASSES: u64 = 0;

pub struct MiniMax {}

impl MiniMax {
    fn evaluate(
        state: &FloatState,
        depth: i32,
        mut alpha: f32,
        mut beta: f32,
        eval_for: bool,
        move_as: bool,
    ) -> f32 {
        debug_assert!(depth > -1);
        debug_assert!(state.is_valid());
        let winner = state.winner();
        // unsafe { TOTAL_PASSES += 1 };
        if winner.is_some() {
            if winner.unwrap() != Field::Empty {
                let score: f32;
                if eval_for == winner.unwrap().to_bool() {
                    // println!("Good");
                    score = 10_000f32 + depth as f32 * 100f32;
                } else {
                    score = -10_000f32 - depth as f32 * 100f32;
                }
                return score;
            }
            return 0.0;
        }
        if depth <= 0 {
            return Self::simple_eval(state, eval_for);
        }
        let moves: ([u64; 16], usize) = state.get_legal_moves();
        if moves.1 == 0 {
            return Self::simple_eval(state, eval_for);
        }

        if move_as {
            let mut value = f32::NEG_INFINITY;
            for i in 0..moves.1 {
                let new_state = state.make_move(moves.0[i]).unwrap();
                value = value.max(Self::evaluate(
                    &new_state,
                    depth - 1,
                    alpha,
                    beta,
                    eval_for,
                    !move_as,
                ));
                if value > beta {
                    break;
                }
                alpha = alpha.max(value);
            }
            return value;
        } else {
            let mut value = f32::INFINITY;
            for i in 0..moves.1 {
                let new_state = state.make_move(moves.0[i]).unwrap();
                value = value.min(Self::evaluate(
                    &new_state,
                    depth - 1,
                    alpha,
                    beta,
                    eval_for,
                    !move_as,
                ));
                if value < alpha {
                    break;
                }
                beta = beta.min(value);
            }
            return value;
        }
    }

    fn simple_eval(state: &FloatState, player: bool) -> f32 {
        let good_points = state.count_winnable_lines(player);
        let bad_points = state.count_winnable_lines(!player);
        if good_points + bad_points == 0.0 {
            return 0.0;
        }
        let score = (good_points - bad_points) / (good_points + bad_points);
        return score;
    }
}

impl CubesAlgorithm for MiniMax {
    //#[allow(static_mut_refs)]
    fn pick_move(&self, state: FloatState) -> u64 {
        let moves: ([u64; 16], usize) = state.get_legal_moves();
        debug_assert!(moves.1 > 0);
        if moves.1 == 1 {
            return moves.0[0];
        }
        let bool_current_player = state.get_current_player();
        let evals: Vec<(&u64, f32)> = moves.0[0..moves.1]
            .par_iter()
            .map(|legal_move| {
                let score = Self::evaluate(
                    &state.make_move(*legal_move).unwrap(),
                    STANDARD_DEPTH,
                    f32::NEG_INFINITY,
                    f32::INFINITY,
                    bool_current_player,
                    bool_current_player,
                );
                // println!("Evaluated {:?}", (legal_move, score));
                return (legal_move, score);
            })
            .collect();
        // println!("{:?}", evals);
        // unsafe {
        //     println!("Evaluated {} branches", TOTAL_PASSES);
        //     TOTAL_PASSES = 0;
        // };
        let mut highest = evals[0];
        for eval in evals {
            if eval.1 > highest.1 {
                highest = eval
            }
        }
        // println!("Picked {:?} with value {:?}", highest.0, highest.1);
        return *highest.0;
    }
}
