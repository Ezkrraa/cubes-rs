use super::{field::Field, float_state::FloatState};

const WINNING_LINES: [u64; 76] = FloatState::generate_lines();

impl FloatState {
    pub fn winner(&self) -> Option<Field> {
        if self.board.0.count_ones() + self.board.1.count_ones() < 7 {
            return None;
        }
        for &line in WINNING_LINES.iter() {
            if self.board.0 & line == line {
                return Some(Field::White);
            } else if self.board.1 & line == line {
                return Some(Field::Black);
            }
        }
        if !(self.board.0 | self.board.1) == 0 {
            return Some(Field::Empty);
        }
        return None;
    }

    pub fn get_legal_moves(&self) -> ([u64; 16], usize) {
        let mut out = [0u64; 16];
        let mut num = 0;
        for i in 0..16 {
            if self.is_column_with_space(i) {
                out[num] = i;
                num += 1;
            }
        }
        return (out, num);
    }

    pub fn count_winnable_lines(&self, count_for: bool) -> f32 {
        let mut total_score = 0f32;
        for &line in WINNING_LINES.iter() {
            let white_score = (self.board.0 & line).count_ones();
            let black_score = (self.board.1 & line).count_ones();
            if count_for {
                if black_score == 0 {
                    total_score += Self::line_value(white_score)
                }
            } else {
                if white_score == 0 {
                    total_score += Self::line_value(black_score)
                }
            }
        }
        return total_score;
    }

    fn line_value(score: u32) -> f32 {
        return match score {
            0 => 0f32,
            1 => 1.0,
            2 => 3.0,
            3 => 15.0,
            _ => 0.0,
        };
    }

    const fn generate_lines() -> [u64; 76] {
        let mut lines = [0u64; 76];
        let mut idx = 0;

        let mut z = 0;
        while z < 4 {
            let mut y = 0;
            while y < 4 {
                // Rows (X axis)
                let mut line = 0u64;
                let mut x = 0;
                while x < 4 {
                    line |= 1u64 << (z * 16 + y * 4 + x);
                    x += 1;
                }
                lines[idx] = line;
                idx += 1;
                y += 1;
            }

            let mut x = 0;
            while x < 4 {
                // Columns (Y axis)
                let mut line = 0u64;
                let mut y = 0;
                while y < 4 {
                    line |= 1u64 << (z * 16 + y * 4 + x);
                    y += 1;
                }
                lines[idx] = line;
                idx += 1;
                x += 1;
            }

            // Diagonals in XY plane
            let mut diag1 = 0u64;
            let mut diag2 = 0u64;
            let mut i = 0;
            while i < 4 {
                diag1 |= 1u64 << (z * 16 + i * 4 + i);
                diag2 |= 1u64 << (z * 16 + i * 4 + (3 - i));
                i += 1;
            }
            lines[idx] = diag1;
            idx += 1;
            lines[idx] = diag2;
            idx += 1;

            z += 1;
        }

        // Columns across Z (pillars)
        let mut y = 0;
        while y < 4 {
            let mut x = 0;
            while x < 4 {
                let mut line = 0u64;
                let mut z = 0;
                while z < 4 {
                    line |= 1u64 << (z * 16 + y * 4 + x);
                    z += 1;
                }
                lines[idx] = line;
                idx += 1;
                x += 1;
            }
            y += 1;
        }

        // Diagonals in YZ
        let mut x = 0;
        while x < 4 {
            let mut diag1 = 0u64;
            let mut diag2 = 0u64;
            let mut i = 0;
            while i < 4 {
                diag1 |= 1u64 << (i * 16 + i * 4 + x);
                diag2 |= 1u64 << (i * 16 + (3 - i) * 4 + x);
                i += 1;
            }
            lines[idx] = diag1;
            idx += 1;
            lines[idx] = diag2;
            idx += 1;
            x += 1;
        }

        // Diagonals in XZ
        let mut y = 0;
        while y < 4 {
            let mut diag1 = 0u64;
            let mut diag2 = 0u64;
            let mut i = 0;
            while i < 4 {
                diag1 |= 1u64 << (i * 16 + y * 4 + i);
                diag2 |= 1u64 << (i * 16 + y * 4 + (3 - i));
                i += 1;
            }
            lines[idx] = diag1;
            idx += 1;
            lines[idx] = diag2;
            idx += 1;
            y += 1;
        }

        // 4 space diagonals (true 3D diagonals)
        let mut diag1 = 0u64;
        let mut diag2 = 0u64;
        let mut diag3 = 0u64;
        let mut diag4 = 0u64;
        let mut i = 0;
        while i < 4 {
            diag1 |= 1u64 << (i * 16 + i * 4 + i);
            diag2 |= 1u64 << (i * 16 + i * 4 + (3 - i));
            diag3 |= 1u64 << (i * 16 + (3 - i) * 4 + i);
            diag4 |= 1u64 << (i * 16 + (3 - i) * 4 + (3 - i));
            i += 1;
        }
        lines[idx] = diag1;
        idx += 1;
        lines[idx] = diag2;
        idx += 1;
        lines[idx] = diag3;
        idx += 1;
        lines[idx] = diag4;

        lines
    }
}
