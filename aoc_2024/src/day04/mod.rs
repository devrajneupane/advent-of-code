pub fn part_one(input: &str) -> usize {
    let grid = Grid::new(input);
    let mut res = 0;
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if grid.vec[row][col] == 'X' {
                res += grid
                    .search_word(row, col)
                    .iter()
                    .filter(|word| *word == &['X', 'M', 'A', 'S'])
                    .count();
            }
        }
    }
    res
}

pub fn part_two(input: &str) -> usize {
    let grid = Grid::new(input);
    let mut res = 0;
    for row in 1..grid.rows() - 2 {
        for col in 1..grid.cols() - 2 {
            if grid.vec[row][col] == 'A' {
                let a = [
                    grid.vec[row - 1][col - 1],
                    grid.vec[row][col],
                    grid.vec[row + 1][col + 1],
                ];
                let b = [
                    grid.vec[row - 1][col + 1],
                    grid.vec[row][col],
                    grid.vec[row + 1][col - 1],
                ];
                res += if [a, b]
                    .iter()
                    .all(|&word| word == ['M', 'A', 'S'] || word == ['S', 'A', 'M'])
                {
                    1
                } else {
                    0
                }
            }
        }
    }
    res
}

struct Grid {
    vec: Vec<Vec<char>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        Self {
            vec: input.lines().map(|line| line.chars().collect()).collect(),
        }
    }

    fn rows(&self) -> usize {
        self.vec.len()
    }

    fn cols(&self) -> usize {
        self.vec[0].len()
    }

    fn search_word(&self, row: usize, col: usize) -> Vec<Vec<char>> {
        let mut res = Vec::new();

        if row as isize - 3 >= 0 {
            // North
            res.push(
                (row - 3..=row)
                    .rev()
                    .map(|row_| self.vec[row_][col])
                    .collect(),
            );
        }
        if row + 3 < self.rows() {
            // South
            res.push((row..=row + 3).map(|row_| self.vec[row_][col]).collect());
        }

        if col + 3 < self.cols() {
            // West
            res.push((col..=col + 3).map(|col_| self.vec[row][col_]).collect());
        }

        if col as isize - 3 >= 0 {
            // East
            res.push(
                (col - 3..=col)
                    .rev()
                    .map(|col_| self.vec[row][col_])
                    .collect(),
            );
        }

        if row as isize - 3 >= 0 && col + 3 < self.cols() {
            // NorthWest
            res.push(
                (row - 3..=row)
                    .rev()
                    .zip(col..=col + 3)
                    .map(|(row_, col_)| self.vec[row_][col_])
                    .collect(),
            );
        }

        if row + 3 < self.rows() && col + 3 < self.cols() {
            // SouthWest
            res.push(
                (row..=row + 3)
                    .zip(col..=col + 3)
                    .map(|(row_, col_)| self.vec[row_][col_])
                    .collect(),
            );
        }

        if row + 3 < self.rows() && col as isize - 3 >= 0 {
            // SouthEast
            res.push(
                (row..=row + 3)
                    .zip((col - 3..=col).rev())
                    .map(|(row_, col_)| self.vec[row_][col_])
                    .collect(),
            );
        }

        if row as isize - 3 >= 0 && col as isize - 3 >= 0 {
            // NortEast
            res.push(
                (row - 3..=row)
                    .rev()
                    .zip((col - 3..=col).rev())
                    .map(|(row_, col_)| self.vec[row_][col_])
                    .collect(),
            );
        }

        res
    }
}

crate::aoctest!(18, 9);
