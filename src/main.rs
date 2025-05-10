mod engine;
use engine::{board_state, field::Field};
mod ai;
mod ui;
use ai::{cubes_algorithm::CubesAlgorithm, minimax::MiniMax};

fn main() {
    let mut board: board_state::BoardState = board_state::BoardState::blank();
    let minimax = MiniMax {};
    while board.winner().is_none() {
        let picked_move = board.pick_coord();
        if picked_move.is_err() {
            println!("Bye");
            return;
        }
        let result = board.make_move(picked_move.unwrap());
        if result.is_err() {
            println!("Error making move")
        } else {
            board = result.unwrap();
        }
        if !board.winner().is_none() {
            break;
        }

        let picked_move = minimax.pick_move(board.clone());
        let result = board.make_move(picked_move);
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
