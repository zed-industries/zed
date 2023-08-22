use std::{
    cmp,
    fmt::{self, Debug},
};

use collections::BinaryHeap;

struct Matrix {
    cells: Vec<isize>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn new() -> Self {
        Self {
            cells: Vec::new(),
            rows: 0,
            cols: 0,
        }
    }

    fn resize(&mut self, rows: usize, cols: usize) {
        self.cells.resize(rows * cols, 0);
        self.rows = rows;
        self.cols = cols;
    }

    fn get(&self, row: usize, col: usize) -> isize {
        if row >= self.rows {
            panic!("row out of bounds")
        }

        if col >= self.cols {
            panic!("col out of bounds")
        }
        self.cells[col * self.rows + row]
    }

    fn set(&mut self, row: usize, col: usize, value: isize) {
        if row >= self.rows {
            panic!("row out of bounds")
        }

        if col >= self.cols {
            panic!("col out of bounds")
        }

        self.cells[col * self.rows + row] = value;
    }
}

impl Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{:5}", self.get(i, j))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
enum Hunk {
    Insert(char),
    Remove(char),
    Keep(char),
}

struct Diff {
    old: String,
    new: String,
    scores: Matrix,
    last_diff_row: usize,
}

impl Diff {
    fn new(old: String) -> Self {
        let mut scores = Matrix::new();
        scores.resize(old.len() + 1, 1);
        for i in 0..=old.len() {
            scores.set(i, 0, -(i as isize));
        }
        dbg!(&scores);
        Self {
            old,
            new: String::new(),
            scores,
            last_diff_row: 0,
        }
    }

    fn push_new(&mut self, text: &str) -> Vec<Hunk> {
        let last_diff_column = self.new.len();
        self.new.push_str(text);
        self.scores.resize(self.old.len() + 1, self.new.len() + 1);

        for j in last_diff_column + 1..=self.new.len() {
            self.scores.set(0, j, -(j as isize));
            for i in 1..=self.old.len() {
                let insertion_score = self.scores.get(i, j - 1) - 1;
                let deletion_score = self.scores.get(i - 1, j) - 10;
                let equality_score = if self.old.as_bytes()[i - 1] == self.new.as_bytes()[j - 1] {
                    self.scores.get(i - 1, j - 1) + 5
                } else {
                    self.scores.get(i - 1, j - 1) - 20
                };
                let score = insertion_score.max(deletion_score).max(equality_score);
                self.scores.set(i, j, score);
            }
        }

        let mut max_score = isize::MIN;
        let mut best_row = self.last_diff_row;
        for i in self.last_diff_row..=self.old.len() {
            let score = self.scores.get(i, self.new.len());
            if score > max_score {
                max_score = score;
                best_row = i;
            }
        }

        let mut hunks = Vec::new();
        let mut i = best_row;
        let mut j = self.new.len();
        while (i, j) != (self.last_diff_row, last_diff_column) {
            let insertion_score = if j > last_diff_column {
                Some((i, j - 1))
            } else {
                None
            };
            let deletion_score = if i > self.last_diff_row {
                Some((i - 1, j))
            } else {
                None
            };
            let equality_score = if i > self.last_diff_row && j > last_diff_column {
                Some((i - 1, j - 1))
            } else {
                None
            };

            let (prev_i, prev_j) = [insertion_score, deletion_score, equality_score]
                .iter()
                .max_by_key(|cell| cell.map(|(i, j)| self.scores.get(i, j)))
                .unwrap()
                .unwrap();

            if prev_i == i && prev_j == j - 1 {
                hunks.push(Hunk::Insert(self.new.chars().skip(j - 1).next().unwrap()));
            } else if prev_i == i - 1 && prev_j == j {
                hunks.push(Hunk::Remove(self.old.chars().skip(i - 1).next().unwrap()));
            } else {
                hunks.push(Hunk::Keep(self.old.chars().skip(i - 1).next().unwrap()));
            }

            i = prev_i;
            j = prev_j;
        }
        self.last_diff_row = best_row;
        hunks.reverse();
        hunks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff() {
        let mut diff = Diff::new("hello world".to_string());
        dbg!(diff.push_new("hello"));
        dbg!(diff.push_new(" ciaone"));
        dbg!(diff.push_new(" world"));
    }
}
