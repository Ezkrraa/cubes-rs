mod bit_engine;
use bit_engine::{bit_state::FloatState, field::Field};
mod ai;
mod ui;
use ai::{cubes_algorithm::CubesAlgorithm, minimax::MiniMax};
use crossterm::execute;
use crossterm::terminal;
use std::io;
use std::time::Instant;

fn main() {
    if execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen).is_err() {
        println!("Failed to open alternate screen.")
    }
    println!("Bye :)");
    run_game_singleplayer();
    execute!(io::stdout(), crossterm::terminal::LeaveAlternateScreen).unwrap();
}

fn run_game_singleplayer() {
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
    execute!(io::stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
    match board.winner().unwrap() {
        Field::Empty => println!("It was a draw"),
        Field::White => println!("O won:"),
        Field::Black => println!("X won:"),
    }
    board.print();
    println!("Press any key to continue.");
    FloatState::block();
}

// fn run_game_
