use super::field::Field;

#[derive(Clone, Debug)]
pub struct FloatState {
    pub board: (u64, u64),
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
        };
    }

    pub fn is_valid(&self) -> bool {
        return self.board.0 & self.board.1 == 0;
    }

    pub fn is_empty(&self, coord: u64) -> bool {
        return (self.board.0 | self.board.1) & (1 << coord) == 0;
    }

    pub fn is_column_with_space(&self, coord: u64) -> bool {
        debug_assert!(coord < 16);
        return self.is_empty(coord + 48)
            || self.is_empty(coord + 32)
            || self.is_empty(coord + 16)
            || self.is_empty(coord);
    }

    pub fn get_on_index(&self, coord: u64) -> Field {
        debug_assert!(self.is_valid());
        debug_assert!(coord < 64);
        let bitmask = 1u64 << coord;
        if (self.board.0 & bitmask) > 0 {
            return Field::White;
        } else if (self.board.1 & bitmask) > 0 {
            return Field::Black;
        }
        return Field::Empty;
    }

    pub fn get_current_player(&self) -> bool {
        return (self.board.0.count_ones() + self.board.1.count_ones()) % 2 == 0;
    }

    fn new_from(old: &Self, coord: u64) -> Self {
        debug_assert!(old.get_on_index(coord) == Field::Empty);

        let mut new_board = old.board.clone();
        if old.get_current_player() {
            new_board.0 = new_board.0 | (1 << coord);
        } else {
            new_board.1 = new_board.1 | (1 << coord);
        }
        return Self { board: new_board };
    }

    pub fn make_move(&self, coord: u64) -> Result<Self, ()> {
        debug_assert!(coord < 16);
        for z in 0..4u64 {
            let new_coord = coord + (z * 16);
            if self.is_empty(new_coord) {
                return Ok(Self::new_from(self, new_coord));
            }
        }
        return Err(());
    }
}
