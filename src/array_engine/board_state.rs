use super::field::Field;

#[derive(Clone)]
pub struct BoardState {
    pub board: [[[Field; 4]; 4]; 4],
    history: Vec<(u8, u8)>,
}

impl BoardState {
    pub fn blank() -> Self {
        return Self {
            board: [
                [
                    [Field::Empty, Field::Empty, Field::Empty, Field::Empty],
                    [Field::Empty, Field::Empty, Field::Empty, Field::Empty],
                    [Field::Empty, Field::Empty, Field::Empty, Field::Empty],
                    [Field::Empty, Field::Empty, Field::Empty, Field::Empty],
                ],
                [
                    [Field::Empty, Field::Empty, Field::Empty, Field::Empty],
                    [Field::Empty, Field::Empty, Field::Empty, Field::Empty],
                    [Field::Empty, Field::Empty, Field::Empty, Field::Empty],
                    [Field::Empty, Field::Empty, Field::Empty, Field::Empty],
                ],
                [
                    [Field::Empty, Field::Empty, Field::Empty, Field::Empty],
                    [Field::Empty, Field::Empty, Field::Empty, Field::Empty],
                    [Field::Empty, Field::Empty, Field::Empty, Field::Empty],
                    [Field::Empty, Field::Empty, Field::Empty, Field::Empty],
                ],
                [
                    [Field::Empty, Field::Empty, Field::Empty, Field::Empty],
                    [Field::Empty, Field::Empty, Field::Empty, Field::Empty],
                    [Field::Empty, Field::Empty, Field::Empty, Field::Empty],
                    [Field::Empty, Field::Empty, Field::Empty, Field::Empty],
                ],
            ],
            history: vec![],
        };
    }

    fn new_from(old: &Self, coord: (usize, usize, usize)) -> Self {
        assert!(old.board[coord.2][coord.1][coord.0] == Field::Empty);

        let mut new_history: Vec<(u8, u8)> = old.history.clone();
        new_history.push((coord.0 as u8, coord.1 as u8));

        let mut new_board: [[[Field; 4]; 4]; 4] = old.board.clone();
        new_board[coord.2][coord.1][coord.0] = old.current_player();

        return Self {
            board: new_board,
            history: new_history,
        };
    }

    pub fn make_move(&self, coord: (usize, usize)) -> Result<Self, String> {
        let x = coord.0;
        let y = coord.1;
        for z in 0..4 {
            if self.board[z][y][x] == Field::Empty {
                return Ok(Self::new_from(self, (x, y, z)));
            }
        }
        return Err(format!(
            "Tried to move to ({}, {}) on this board: {:?}",
            coord.0, coord.1, self.board
        ));
    }

    #[allow(dead_code)]
    pub fn current_player(&self) -> Field {
        return if self.history.len() % 2 == 0 {
            Field::White
        } else {
            Field::Black
        };
    }
}
