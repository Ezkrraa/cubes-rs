use colored::Colorize;

use crate::board_state::BoardState;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

#[derive(PartialEq, Eq, Copy, Clone)]
enum DirectionInput {
    Up,
    Down,
    Left,
    Right,
    End,
    Stop,
}

impl BoardState {
    pub fn print(&self) {
        for z in 0..4 {
            for y in 0..4 {
                for x in 0..4 {
                    print!("{} ", self.board[z][y][x].char())
                }
                print!("\n")
            }
            print!("\n")
        }
    }

    fn print_highlight(&self, coord: (usize, usize, usize)) {
        for z in 0..4 {
            for y in 0..4 {
                for x in 0..4 {
                    if coord == (x, y, z) {
                        print!("{} ", self.board[z][y][x].char().on_white().black())
                    } else {
                        print!("{} ", self.board[z][y][x].char())
                    }
                }
                print!("\n")
            }
            print!("\n")
        }
    }

    // returns a two-dimensional coordinate (from above, X and Y)
    pub fn pick_coord(&self) -> Result<(usize, usize), ()> {
        // XYZ coordinate
        let mut selection: (usize, usize, usize) = (0, 0, 0);
        loop {
            println!(
                "\n\n\n\n\n\nSelection: {}, {}, {}",
                selection.0, selection.1, selection.2
            );
            self.print_highlight(selection);
            let direction = Self::get_direction();
            if direction == DirectionInput::Stop {
                return Err(());
            } else if direction == DirectionInput::End {
                return Ok((selection.0, selection.1));
            } else {
                selection = Self::move_coordinate(selection, direction);
            }
        }
    }

    fn move_coordinate(
        coordinate: (usize, usize, usize),
        direction: DirectionInput,
    ) -> (usize, usize, usize) {
        let mut coord = coordinate;
        if direction == DirectionInput::Up {
            if coord.1 > 0 {
                coord.1 -= 1;
            } else if coord.2 > 0 {
                coord.2 -= 1;
                coord.1 = 3;
            }
        } else if direction == DirectionInput::Down {
            if coord.1 < 3 {
                coord.1 += 1;
            } else if coord.2 < 3 {
                coord.2 += 1;
                coord.1 = 0;
            }
        } else if direction == DirectionInput::Left {
            if coord.0 > 0 {
                coord.0 -= 1;
            }
        } else if direction == DirectionInput::Right {
            if coord.0 < 3 {
                coord.0 += 1;
            }
        }
        return coord;
    }

    fn get_direction() -> DirectionInput {
        // if this fails I can't recover much anyways
        crossterm::terminal::enable_raw_mode().unwrap();

        loop {
            match crossterm::event::read() {
                Ok(event) => {
                    if !event.is_key_press() {
                        continue;
                    }
                    match event {
                        Event::Key(KeyEvent {
                            code: KeyCode::Char('c'),
                            kind: _,
                            modifiers: KeyModifiers::CONTROL,
                            state: _,
                        }) => return DirectionInput::Stop,
                        Event::Key(key_event) => match key_event.code {
                            KeyCode::Up => return DirectionInput::Up,
                            KeyCode::Down => return DirectionInput::Down,
                            KeyCode::Left => return DirectionInput::Left,
                            KeyCode::Right => return DirectionInput::Right,

                            KeyCode::Esc => return DirectionInput::Stop,
                            KeyCode::Backspace => return DirectionInput::Stop,

                            KeyCode::End => return DirectionInput::End,
                            KeyCode::Enter => return DirectionInput::End,
                            _ => continue,
                        },
                        _ => continue,
                    }
                }
                Err(_) => panic!("Event reading error"),
            }
        }
    }
}
