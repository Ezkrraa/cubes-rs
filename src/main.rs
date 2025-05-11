mod float_engine;
use float_engine::{field::Field, float_state::FloatState};
mod ai;
mod ui;
use ai::{cubes_algorithm::CubesAlgorithm, minimax::MiniMax};
use std::time::Instant;

// optimized make_move
//

fn main() {
    let mut board: FloatState = FloatState::blank();
    let minimax = MiniMax {};
    while board.winner().is_none() {
        let picked_move = board.pick_coord();
        if picked_move.is_err() {
            println!("Bye");
            return;
        }
        let result = board.make_move(picked_move.unwrap());
        if result.is_err() {
            println!("Error making move on board: {:?}", board)
        } else {
            board = result.unwrap();
        }
        if !board.winner().is_none() {
            break;
        }

        println!("{:?}", board.get_legal_moves());
        let before = Instant::now();
        let picked_move = minimax.pick_move(board.clone());
        let result = board.make_move(picked_move);
        println!("{:.2?}", before.elapsed());
        if result.is_err() {
            println!("Error making move")
        } else {
            board = result.unwrap();
        }
    }
    match board.winner().unwrap() {
        Field::Empty => println!("It was a draw"),
        Field::White => println!("O won:"),
        Field::Black => println!("X won:"),
    }
    board.print();
}
