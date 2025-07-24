use std::io;

use colored::Colorize;

use crate::FloatState;

use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{Clear, ClearType},
};

#[derive(PartialEq, Eq, Copy, Clone)]
enum DirectionInput {
    Up,
    Down,
    Left,
    Right,
    End,
    Stop,
}

impl FloatState {
    pub fn print(&self) {
        for z in 0..4u64 {
            for y in 0..4u64 {
                for x in 0..4u64 {
                    print!("{} ", self.get_on_index(z * 16 + y * 4 + x).char())
                }
                print!("\n")
            }
            print!("\n")
        }
    }

    fn print_highlight(&self, coord: u64) {
        for z in 0..4u64 {
            for y in 0..4u64 {
                for x in 0..4u64 {
                    if coord == (z * 16 + y * 4 + x) {
                        print!(
                            "{} ",
                            self.get_on_index(z * 16 + y * 4 + x)
                                .char()
                                .on_white()
                                .black()
                        )
                    } else {
                        print!("{} ", self.get_on_index(z * 16 + y * 4 + x).char())
                    }
                }
                print!("\r\n")
            }
            print!("\r\n")
        }
    }

    // returns a two-dimensional coordinate (from above, X and Y)
    pub fn pick_coord(&self) -> Result<u64, ()> {
        // XYZ coordinate
        let mut selection: u64 = 0;
        loop {
            execute!(io::stdout(), Clear(ClearType::All)).unwrap();
            println!(
                "\r\n\r\n\r\n\r\n\r\n\r\nSelection: {} ({}, {}, {})\r",
                selection,
                selection % 16 % 4,
                selection % 16 / 4,
                selection / 16
            );
            self.print_highlight(selection);
            let direction = Self::get_direction();
            if direction == DirectionInput::Stop {
                return Err(());
            } else if direction == DirectionInput::End && self.is_column_with_space(selection) {
                return Ok(selection);
            } else {
                selection = Self::move_coordinate(selection, direction);
            }
        }
    }

    pub fn block() {
        loop {
            match crossterm::event::read().unwrap() {
                event => {
                    if event.is_key_press() {
                        return;
                    }
                }
            }
        }
    }

    fn move_coordinate(coordinate: u64, direction: DirectionInput) -> u64 {
        let mut coord = coordinate;
        let (x, y, z) = (coord % 4, coord % 16 / 4, coord / 16);
        if direction == DirectionInput::Up {
            if y > 0 || z > 0 {
                coord -= 4
            }
        } else if direction == DirectionInput::Down {
            if y < 3 || z < 3 {
                coord += 4;
            }
        } else if direction == DirectionInput::Left {
            if x > 0 {
                coord -= 1;
            }
        } else if direction == DirectionInput::Right {
            if x < 3 {
                coord += 1;
            }
        }
        return coord;
    }

    fn get_direction() -> DirectionInput {
        // if this fails I can't recover much anyways
        crossterm::terminal::enable_raw_mode().unwrap();
	//TODO: disable on leaving
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
                            KeyCode::Char('w') => return DirectionInput::Up,
                            KeyCode::Char('k') => return DirectionInput::Up,

                            KeyCode::Down => return DirectionInput::Down,
                            KeyCode::Char('s') => return DirectionInput::Down,
                            KeyCode::Char('j') => return DirectionInput::Down,

                            KeyCode::Left => return DirectionInput::Left,
                            KeyCode::Char('a') => return DirectionInput::Left,
                            KeyCode::Char('h') => return DirectionInput::Left,

                            KeyCode::Right => return DirectionInput::Right,
                            KeyCode::Char('d') => return DirectionInput::Right,
                            KeyCode::Char('l') => return DirectionInput::Right,

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
