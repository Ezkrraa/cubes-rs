use super::{board_state::BoardState, field::Field};

impl BoardState {
    pub fn winner(&self) -> Option<Field> {
        // line wins
        for i in 0..4 {
            for j in 0..4 {
                if Self::check_line((
                    self.board[i][j][0],
                    self.board[i][j][1],
                    self.board[i][j][2],
                    self.board[i][j][3],
                )) != Field::Empty
                {
                    // vertical (z-axis) win
                    return Some(self.board[i][j][0]);
                } else if Self::check_line((
                    self.board[i][0][j],
                    self.board[i][1][j],
                    self.board[i][2][j],
                    self.board[i][3][j],
                )) != Field::Empty
                {
                    // horizontal (y-axis) win
                    return Some(self.board[i][0][j]);
                } else if Self::check_line((
                    self.board[0][i][j],
                    self.board[1][i][j],
                    self.board[2][i][j],
                    self.board[3][i][j],
                )) != Field::Empty
                {
                    // horizontal (x-axis) win
                    return Some(self.board[0][i][j]);
                }
            }
        }

        // diagonal wins
        for i in 0..4 {
            if Self::check_line((
                self.board[i][0][0],
                self.board[i][1][1],
                self.board[i][2][2],
                self.board[i][3][3],
            )) != Field::Empty
            {
                // XY
                return Some(self.board[i][0][0]);
            } else if Self::check_line((
                self.board[i][0][3],
                self.board[i][1][2],
                self.board[i][2][1],
                self.board[i][3][0],
            )) != Field::Empty
            {
                // reverse XY
                return Some(self.board[i][0][0]);
            } else if Self::check_line((
                self.board[0][i][0],
                self.board[1][i][1],
                self.board[2][i][2],
                self.board[3][i][3],
            )) != Field::Empty
            {
                // XZ
                return Some(self.board[i][0][0]);
            } else if Self::check_line((
                self.board[0][i][3],
                self.board[1][i][2],
                self.board[2][i][1],
                self.board[3][i][0],
            )) != Field::Empty
            {
                // reverse XZ
                return Some(self.board[0][i][0]);
            } else if Self::check_line((
                self.board[0][0][i],
                self.board[1][1][i],
                self.board[2][2][i],
                self.board[3][3][i],
            )) != Field::Empty
            {
                // YZ
                return Some(self.board[i][0][0]);
            } else if Self::check_line((
                self.board[0][3][i],
                self.board[1][2][i],
                self.board[2][1][i],
                self.board[3][0][i],
            )) != Field::Empty
            {
                // reverse YZ
                return Some(self.board[0][3][i]);
            }
        }

        if Self::check_line((
            self.board[0][0][0],
            self.board[1][1][1],
            self.board[2][2][2],
            self.board[3][3][3],
        )) != Field::Empty
        {
            // main 3D diagonal
            return Some(self.board[0][0][0]);
        } else if Self::check_line((
            self.board[0][0][3],
            self.board[1][1][2],
            self.board[2][2][1],
            self.board[3][3][0],
        )) != Field::Empty
        {
            // main reverse 3D diagona
            return Some(self.board[0][0][3]);
        } else if Self::check_line((
            self.board[0][3][0],
            self.board[1][2][1],
            self.board[2][1][2],
            self.board[3][0][3],
        )) != Field::Empty
        {
            // reverse 3D diagonal 1
            return Some(self.board[0][3][0]);
        } else if Self::check_line((
            self.board[0][3][3],
            self.board[1][2][2],
            self.board[2][1][1],
            self.board[3][0][0],
        )) != Field::Empty
        {
            // reverse 3D diagonal 2
            return Some(self.board[0][3][3]);
        }

        let mut empty_found = false;
        for z in 0..4 {
            for y in 0..4 {
                for x in 0..4 {
                    if self.board[z][y][x] == Field::Empty {
                        empty_found = true;
                    }
                }
            }
        }
        if empty_found {
            return None;
        } else {
            return Some(Field::Empty);
        }
    }

    // if a line is all one color, returns that color
    // otherwise, return Empty
    fn check_line(items: (Field, Field, Field, Field)) -> Field {
        if items.0 == items.1 && items.1 == items.2 && items.2 == items.3 {
            return items.0;
        } else {
            return Field::Empty;
        }
    }

    pub fn get_legal_moves(&self) -> Vec<(usize, usize)> {
        let mut out: Vec<(usize, usize)> = vec![];
        for x in 0..4 {
            for y in 0..4 {
                let mut is_empty = false;
                for z in 0..4 {
                    if self.board[z][y][x] == Field::Empty {
                        is_empty = true;
                        break;
                    }
                }
                if is_empty {
                    out.push((x, y));
                }
            }
        }
        return out;
    }

    fn count_line_score(count_for: Field, items: [Field; 4]) -> i16 {
        let mut count = 0;
        for item in items {
            if count_for == item {
                count += 1
            } else if item != Field::Empty {
                return 0;
            }
        }
        if count > 2 {
            return 3;
        } else if count == 1 {
            return 1;
        } else {
            return 0;
        }
    }

    pub fn count_winnable_lines(&self, count_for: Field) -> i16 {
        let mut total_score: i16 = 0;
        // line wins
        for i in 0..4 {
            for j in 0..4 {
                total_score += Self::count_line_score(
                    count_for,
                    [
                        self.board[i][j][0],
                        self.board[i][j][1],
                        self.board[i][j][2],
                        self.board[i][j][3],
                    ],
                );
                total_score += Self::count_line_score(
                    count_for,
                    [
                        self.board[i][0][j],
                        self.board[i][1][j],
                        self.board[i][2][j],
                        self.board[i][3][j],
                    ],
                );
                total_score += Self::count_line_score(
                    count_for,
                    [
                        self.board[0][i][j],
                        self.board[1][i][j],
                        self.board[2][i][j],
                        self.board[3][i][j],
                    ],
                );
            }

            // diagonal wins
            for i in 0..4 {
                total_score += Self::count_line_score(
                    count_for,
                    [
                        self.board[i][0][0],
                        self.board[i][1][1],
                        self.board[i][2][2],
                        self.board[i][3][3],
                    ],
                );
                total_score += Self::count_line_score(
                    count_for,
                    [
                        self.board[i][0][3],
                        self.board[i][1][2],
                        self.board[i][2][1],
                        self.board[i][3][0],
                    ],
                );
                total_score += Self::count_line_score(
                    count_for,
                    [
                        self.board[0][i][0],
                        self.board[1][i][1],
                        self.board[2][i][2],
                        self.board[3][i][3],
                    ],
                );
                total_score += Self::count_line_score(
                    count_for,
                    [
                        self.board[0][i][3],
                        self.board[1][i][2],
                        self.board[2][i][1],
                        self.board[3][i][0],
                    ],
                );
                total_score += Self::count_line_score(
                    count_for,
                    [
                        self.board[0][0][i],
                        self.board[1][1][i],
                        self.board[2][2][i],
                        self.board[3][3][i],
                    ],
                );
                total_score += Self::count_line_score(
                    count_for,
                    [
                        self.board[0][3][i],
                        self.board[1][2][i],
                        self.board[2][1][i],
                        self.board[3][0][i],
                    ],
                );
            }
            total_score += Self::count_line_score(
                count_for,
                [
                    self.board[0][0][0],
                    self.board[1][1][1],
                    self.board[2][2][2],
                    self.board[3][3][3],
                ],
            );
            total_score += Self::count_line_score(
                count_for,
                [
                    self.board[0][0][3],
                    self.board[1][1][2],
                    self.board[2][2][1],
                    self.board[3][3][0],
                ],
            );
            total_score += Self::count_line_score(
                count_for,
                [
                    self.board[0][3][0],
                    self.board[1][2][1],
                    self.board[2][1][2],
                    self.board[3][0][3],
                ],
            );
            total_score += Self::count_line_score(
                count_for,
                [
                    self.board[0][3][3],
                    self.board[1][2][2],
                    self.board[2][1][1],
                    self.board[3][0][0],
                ],
            );
        }
        return total_score;
    }
}
