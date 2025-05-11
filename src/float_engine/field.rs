#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Field {
    Empty,
    White,
    Black,
}

impl Field {
    pub fn char(&self) -> &str {
        return match self {
            Field::Empty => "_",
            Field::White => "O",
            Field::Black => "X",
        };
    }

    pub fn to_bool(self) -> bool {
        debug_assert!(self != Field::Empty);
        return match self {
            Field::White => true,
            Field::Black => false,
            _ => false,
        };
    }
}
