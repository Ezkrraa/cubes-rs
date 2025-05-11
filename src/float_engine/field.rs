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

    pub fn to_bool(&self) -> bool {
        assert!(*self != Field::Empty);
        return match self {
            Field::White => true,
            Field::Black => false,
            _ => false,
        };
    }

    pub fn from_bool(boo: bool) -> Self {
        if boo {
            return Field::White;
        } else {
            return Field::Black;
        }
    }

    pub fn get_opposite(&self) -> Self {
        assert!(*self != Field::Empty);
        if *self == Self::White {
            return Self::Black;
        } else {
            return Self::White;
        }
    }
}
