use core::f32;
use std::cmp::Ordering;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::engine::{board_state::BoardState, field::Field};

use super::cubes_algorithm::CubesAlgorithm;

const STANDARD_DEPTH: i32 = 7;

pub struct MiniMax {}

impl MiniMax {
    fn evaluate(
        state: &BoardState,
        depth: i32,
        mut alpha: f32,
        beta: f32,
        player: bool,
        move_as: bool,
    ) -> f32 {
        assert!(depth > -1);
        let winner = state.winner();
        if winner.is_some() {
            if winner.unwrap() != Field::Empty {
                let score: f32;
                if player == winner.unwrap().to_bool() {
                    score = (10_000 + depth * 100) as f32;
                } else {
                    score = (-10_000 - depth * 100) as f32;
                }
                return score;
            }
            return 0.0;
        }
        let moves: Vec<(usize, usize)> = state.get_legal_moves();
        if depth <= 0 || moves.len() == 0 {
            return Self::simple_eval(state, Field::from_bool(player));
        }

        if move_as {
            let mut value = f32::NEG_INFINITY;
            for evaluate_move in moves {
                let new_state = state.make_move(evaluate_move).unwrap();
                value = value.max(Self::evaluate(
                    &new_state,
                    depth - 1,
                    alpha,
                    beta,
                    player,
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
                    player,
                    !move_as,
                ));
                if value < beta {
                    break;
                }
                alpha = alpha.max(value);
            }
            return value;
        }
    }

    fn simple_eval(state: &BoardState, player: Field) -> f32 {
        let good_points = state.count_winnable_lines(player);
        let bad_points = state.count_winnable_lines(player.get_opposite());
        if good_points + bad_points == 0 {
            return 0.0;
        }
        let score: f32 = f32::from((good_points - bad_points) / (good_points + bad_points));
        return score as f32;
    }
}

impl CubesAlgorithm for MiniMax {
    fn pick_move(&self, state: BoardState) -> (usize, usize) {
        let moves: Vec<(usize, usize)> = state.get_legal_moves();
        assert!(moves.len() > 0);
        if moves.len() == 1 {
            return moves[0];
        }
        let max_var = moves
            .par_iter()
            .map(|legal_move| {
                let bool_current_player = state.current_player().to_bool();
                let score = Self::evaluate(
                    &state.clone(),
                    STANDARD_DEPTH,
                    f32::NEG_INFINITY,
                    f32::INFINITY,
                    bool_current_player,
                    bool_current_player,
                );
                return (legal_move, score);
            })
            .max_by(|item_a, item_b| {
                if item_a.0 > item_b.0 {
                    Ordering::Greater
                } else if (item_a.0 < item_b.0) {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            })
            .unwrap();
        return *(max_var.0);
    }
}
