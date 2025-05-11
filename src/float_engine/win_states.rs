use super::{field::Field, float_state::FloatState};

impl FloatState {
    pub fn winner(&self) -> Option<Field> {
        if self.history.len() < 7 {
            return None;
        }
        // line wins
        for i in 0..4 {
            for j in 0..4 {
                if self.check_line((
                    i * 16 + j * 4 + 0,
                    i * 16 + j * 4 + 1,
                    i * 16 + j * 4 + 2,
                    i * 16 + j * 4 + 3,
                )) != Field::Empty
                {
                    // vertical (z-axis) win
                    return Some(self.get_on_index(i * 16 + j * 4 + 0));
                } else if self.check_line((
                    i * 16 + 0 * 4 + j,
                    i * 16 + 1 * 4 + j,
                    i * 16 + 2 * 4 + j,
                    i * 16 + 3 * 4 + j,
                )) != Field::Empty
                {
                    // horizontal (y-axis) win
                    return Some(self.get_on_index(i * 16 + 0 * 4 + j));
                } else if self.check_line((
                    0 * 16 + i * 4 + j,
                    1 * 16 + i * 4 + j,
                    2 * 16 + i * 4 + j,
                    3 * 16 + i * 4 + j,
                )) != Field::Empty
                {
                    // horizontal (x-axis) win
                    return Some(self.get_on_index(0 * 16 + i * 4 + j));
                }
            }
        }

        // diagonal wins
        for i in 0..4 {
            if self.check_line((
                i * 16 + 0 * 4 + 0,
                i * 16 + 1 * 4 + 1,
                i * 16 + 2 * 4 + 2,
                i * 16 + 3 * 4 + 3,
            )) != Field::Empty
            {
                // XY
                return Some(self.get_on_index(i * 16 + 0 * 4 + 0));
            } else if self.check_line((
                i * 16 + 0 * 4 + 3,
                i * 16 + 1 * 4 + 2,
                i * 16 + 2 * 4 + 1,
                i * 16 + 3 * 4 + 0,
            )) != Field::Empty
            {
                // reverse XY
                return Some(self.get_on_index(i * 16 + 0 * 4 + 0));
            } else if self.check_line((
                0 * 16 + i * 4 + 0,
                1 * 16 + i * 4 + 1,
                2 * 16 + i * 4 + 2,
                3 * 16 + i * 4 + 3,
            )) != Field::Empty
            {
                // XZ
                return Some(self.get_on_index(i * 16 + 0 * 4 + 0));
            } else if self.check_line((
                0 * 16 + i * 4 + 3,
                1 * 16 + i * 4 + 2,
                2 * 16 + i * 4 + 1,
                3 * 16 + i * 4 + 0,
            )) != Field::Empty
            {
                // reverse XZ
                return Some(self.get_on_index(0 * 16 + i * 4 + 0));
            } else if self.check_line((
                0 * 16 + 0 * 4 + i,
                1 * 16 + 1 * 4 + i,
                2 * 16 + 2 * 4 + i,
                3 * 16 + 3 * 4 + i,
            )) != Field::Empty
            {
                // YZ
                return Some(self.get_on_index(i * 16 + 0 * 4 + 0));
            } else if self.check_line((
                0 * 16 + 3 * 4 + i,
                1 * 16 + 2 * 4 + i,
                2 * 16 + 1 * 4 + i,
                3 * 16 + 0 * 4 + i,
            )) != Field::Empty
            {
                // reverse YZ
                return Some(self.get_on_index(0 * 16 + 3 * 4 + i));
            }
        }

        if self.check_line((
            0 * 16 + 0 * 4 + 0,
            1 * 16 + 1 * 4 + 1,
            2 * 16 + 2 * 4 + 2,
            3 * 16 + 3 * 4 + 3,
        )) != Field::Empty
        {
            // main 3D diagonal
            return Some(self.get_on_index(0 * 16 + 0 * 4 + 0));
        } else if self.check_line((
            0 * 16 + 0 * 4 + 3,
            1 * 16 + 1 * 4 + 2,
            2 * 16 + 2 * 4 + 1,
            3 * 16 + 3 * 4 + 0,
        )) != Field::Empty
        {
            // main reverse 3D diagona
            return Some(self.get_on_index(0 * 16 + 0 * 4 + 3));
        } else if self.check_line((
            0 * 16 + 3 * 4 + 0,
            1 * 16 + 2 * 4 + 1,
            2 * 16 + 1 * 4 + 2,
            3 * 16 + 0 * 4 + 3,
        )) != Field::Empty
        {
            // reverse 3D diagonal 1
            return Some(self.get_on_index(0 * 16 + 3 * 4 + 0));
        } else if self.check_line((
            0 * 16 + 3 * 4 + 3,
            1 * 16 + 2 * 4 + 2,
            2 * 16 + 1 * 4 + 1,
            3 * 16 + 0 * 4 + 0,
        )) != Field::Empty
        {
            // reverse 3D diagonal 2
            return Some(self.get_on_index(0 * 16 + 3 * 4 + 3));
        }

        if !(self.board.0 | self.board.1) == 0 {
            // all fields were filled
            return Some(Field::Empty);
        }
        return None;
    }

    fn check_line(&self, coords: (u64, u64, u64, u64)) -> Field {
        if self.get_on_index(coords.0) != Field::Empty
            && self.get_on_index(coords.0) == self.get_on_index(coords.1)
            && self.get_on_index(coords.1) == self.get_on_index(coords.2)
            && self.get_on_index(coords.2) == self.get_on_index(coords.3)
        {
            return self.get_on_index(coords.0);
        }
        return Field::Empty;
    }

    pub fn get_legal_moves(&self) -> Vec<u64> {
        let mut out: Vec<u64> = vec![];
        for i in 0..16u64 {
            if self.is_column_with_space(i) {
                out.push(i);
            }
        }
        return out;
    }

    fn count_line_score(&self, count_for: Field, coords: [u64; 4]) -> f32 {
        let mut count = 0;
        for index in coords {
            let item = self.get_on_index(index);
            if count_for == item {
                count += 1
            } else if item != Field::Empty {
                return 0.0;
            }
        }
        if count > 2 {
            return 30.0;
        } else if count == 2 {
            return 10.0;
        } else if count == 1 {
            return 5.0;
        } else {
            return 0.0;
        }
    }

    pub fn count_winnable_lines(&self, count_for: bool) -> f32 {
        let mut total_score = 0f32;
        let count_for_field = Field::from_bool(count_for);
        // lines
        for i in 0..4u64 {
            for j in 0..4u64 {
                let x_row = i + j * 4;
                let y_row = i + j * 4;
                let z_row = i + j * 4;
                total_score += self.count_line_score(
                    count_for_field,
                    [x_row + 0, x_row + 1, x_row + 2, x_row + 3],
                );
                total_score += self.count_line_score(
                    count_for_field,
                    [y_row + 0 * 4, y_row + 1 * 4, y_row + 2 * 4, y_row + 3 * 4],
                );
                total_score += self.count_line_score(
                    count_for_field,
                    [
                        z_row + 0 * 16,
                        z_row + 1 * 16,
                        z_row + 2 * 16,
                        z_row + 3 * 16,
                    ],
                );
            }
        }

        for i in 0..4u64 {
            // XY and reverse XY
            total_score += self.count_line_score(
                count_for_field,
                [
                    (i * 16) + (0 * 4) + (0),
                    (i * 16) + (1 * 4) + (1),
                    (i * 16) + (2 * 4) + (2),
                    (i * 16) + (3 * 4) + (3),
                ],
            );
            total_score += self.count_line_score(
                count_for_field,
                [
                    (i * 16) + (0 * 4) + (3),
                    (i * 16) + (1 * 4) + (2),
                    (i * 16) + (2 * 4) + (1),
                    (i * 16) + (3 * 4) + (0),
                ],
            );

            // YZ and reverse YZ
            total_score += self.count_line_score(
                count_for_field,
                [
                    (0 * 16) + (i * 4) + (0),
                    (1 * 16) + (i * 4) + (1),
                    (2 * 16) + (i * 4) + (2),
                    (3 * 16) + (i * 4) + (3),
                ],
            );
            total_score += self.count_line_score(
                count_for_field,
                [
                    (0 * 16) + (i * 4) + (3),
                    (1 * 16) + (i * 4) + (2),
                    (2 * 16) + (i * 4) + (1),
                    (3 * 16) + (i * 4) + (0),
                ],
            );
            // XZ and reverse XZ
            total_score += self.count_line_score(
                count_for_field,
                [
                    (0 * 16) + (0 * 4) + (i),
                    (1 * 16) + (1 * 4) + (i),
                    (2 * 16) + (2 * 4) + (i),
                    (3 * 16) + (3 * 4) + (i),
                ],
            );
            total_score += self.count_line_score(
                count_for_field,
                [
                    (3 * 16) + (0 * 4) + (i),
                    (2 * 16) + (1 * 4) + (i),
                    (1 * 16) + (2 * 4) + (i),
                    (0 * 16) + (3 * 4) + (i),
                ],
            );
        }

        // diagonals to opposite corners (that don't share any verteces or faces)
        total_score += self.count_line_score(
            count_for_field,
            [
                (0 * 16) + (0 * 4) + (0),
                (1 * 16) + (1 * 4) + (1),
                (2 * 16) + (2 * 4) + (2),
                (3 * 16) + (3 * 4) + (3),
            ],
        );
        total_score += self.count_line_score(
            count_for_field,
            [
                (0 * 16) + (0 * 4) + (3),
                (1 * 16) + (1 * 4) + (2),
                (2 * 16) + (2 * 4) + (1),
                (3 * 16) + (3 * 4) + (0),
            ],
        );
        total_score += self.count_line_score(
            count_for_field,
            [
                (0 * 16) + (3 * 4) + (0),
                (1 * 16) + (2 * 4) + (1),
                (2 * 16) + (1 * 4) + (2),
                (3 * 16) + (0 * 4) + (3),
            ],
        );
        total_score += self.count_line_score(
            count_for_field,
            [
                (0 * 16) + (3 * 4) + (3),
                (1 * 16) + (2 * 4) + (2),
                (2 * 16) + (1 * 4) + (1),
                (3 * 16) + (0 * 4) + (0),
            ],
        );

        return total_score;
    }
}
