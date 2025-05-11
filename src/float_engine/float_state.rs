use super::field::Field;

#[derive(Clone, Debug)]
pub struct FloatState {
    pub board: (u64, u64),
    pub history: Vec<u64>,
}

impl FloatState {
    pub fn blank() -> Self {
        return Self {
            board: (
                // indexes iterates over X, then Y, then Z. (Z is worth most)
                // O's set is shown first
                0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            ),
            history: vec![],
        };
    }

    pub fn is_valid(&self) -> bool {
        return self.board.0 & self.board.1 == 0;
    }

    pub fn is_empty(&self, coord: u64) -> bool {
        return (self.board.0 | self.board.1) & (1 << coord) == 0;
    }

    pub fn is_column_with_space(&self, coord: u64) -> bool {
        assert!(coord < 16);
        return self.is_empty(coord + 48)
            || self.is_empty(coord + 32)
            || self.is_empty(coord + 16)
            || self.is_empty(coord);
    }

    pub fn get_on_index(&self, coord: u64) -> Field {
        assert!(self.is_valid());
        assert!(coord < 64);
        let bitmask = 1u64 << coord;
        if (self.board.0 & bitmask) > 0 {
            return Field::White;
        } else if (self.board.1 & bitmask) > 0 {
            return Field::Black;
        }
        return Field::Empty;
    }

    pub fn get_current_player(&self) -> bool {
        return self.history.len() % 2 == 0;
    }

    fn new_from(old: &Self, coord: u64) -> Self {
        assert!(old.get_on_index(coord) == Field::Empty);

        let mut new_history = old.history.clone();
        new_history.push(coord);

        let mut new_board = old.board.clone();
        if old.get_current_player() {
            new_board.0 = new_board.0 | (1 << coord);
        } else {
            new_board.1 = new_board.1 | (1 << coord);
        }
        return Self {
            board: new_board,
            history: new_history,
        };
    }

    pub fn make_move(&self, coord: u64) -> Result<Self, ()> {
        assert!(coord < 16);
        for z in 0..4u64 {
            let new_coord = coord + (z * 16);
            if self.is_empty(new_coord) {
                return Ok(Self::new_from(self, new_coord));
            }
        }
        return Err(());
    }
}
