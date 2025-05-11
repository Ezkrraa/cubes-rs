use core::f32;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::float_engine::{field::Field, float_state::FloatState};

use super::cubes_algorithm::CubesAlgorithm;

// unsorted:
// 3:     15204
// 3:     23997
// 4:     54171
// 4:     53916
// 5:    584016
// 5:    806971
// 5:   1091447
// 6:   3620401
// 6:   3612290
// 7:  21935552
// 7:  21945426
// 7:  28840632
// 8: 164691231

// sorted:
// 7: ...

const STANDARD_DEPTH: i32 = 5;
static mut total_passes: u64 = 0;

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
        assert!(depth > -1);
        assert!(state.is_valid());
        let winner = state.winner();
        unsafe { total_passes += 1 };
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
        // let mut moves: Vec<(u64, f32)> = state
        //     .get_legal_moves()
        //     .into_iter()
        //     .map(|legal_move| {
        //         (
        //             legal_move,
        //             Self::simple_eval(
        //                 &state.make_move(legal_move).unwrap(),
        //                 Field::from_bool(player),
        //             ),
        //         )
        //     })
        //     .collect();
        let moves: Vec<u64> = state.get_legal_moves();
        if depth <= 0 || moves.len() == 0 {
            return Self::simple_eval(state, eval_for);
        }

        // moves.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));

        if move_as {
            let mut value = f32::NEG_INFINITY;
            for evaluate_move in moves {
                let new_state = state.make_move(evaluate_move).unwrap();
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
            for evaluate_move in moves {
                let new_state = state.make_move(evaluate_move).unwrap();
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
    #[allow(static_mut_refs)]
    fn pick_move(&self, state: FloatState) -> u64 {
        let moves: Vec<u64> = state.get_legal_moves();
        assert!(moves.len() > 0);
        if moves.len() == 1 {
            return moves[0];
        }
        let bool_current_player = state.get_current_player();
        let evals: Vec<(&u64, f32)> = moves
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
                println!("Evaluated {:?}", (legal_move, score));
                return (legal_move, score);
            })
            .collect();
        println!("{:?}", evals);
        unsafe {
            println!("{}", total_passes.clone());
            total_passes = 0;
        };
        let mut highest = evals[0];
        for eval in evals {
            if eval.1 > highest.1 {
                highest = eval
            }
        }
        println!("Picked {:?} with value {:?}", *(highest.0), highest.1);
        return *(highest.0);
    }
}
