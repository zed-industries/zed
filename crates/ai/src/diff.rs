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
pub enum Hunk {
    Insert { text: String },
    Remove { len: usize },
    Keep { len: usize },
}

pub struct Diff {
    old: String,
    new: String,
    scores: Matrix,
    old_text_ix: usize,
    new_text_ix: usize,
}

impl Diff {
    const INSERTION_SCORE: isize = -1;
    const DELETION_SCORE: isize = -4;
    const EQUALITY_SCORE: isize = 5;

    pub fn new(old: String) -> Self {
        let mut scores = Matrix::new();
        scores.resize(old.len() + 1, 1);
        for i in 0..=old.len() {
            scores.set(i, 0, i as isize * Self::DELETION_SCORE);
        }
        Self {
            old,
            new: String::new(),
            scores,
            old_text_ix: 0,
            new_text_ix: 0,
        }
    }

    pub fn push_new(&mut self, text: &str) -> Vec<Hunk> {
        self.new.push_str(text);
        self.scores.resize(self.old.len() + 1, self.new.len() + 1);

        for j in self.new_text_ix + 1..=self.new.len() {
            self.scores.set(0, j, j as isize * Self::INSERTION_SCORE);
            for i in 1..=self.old.len() {
                let insertion_score = self.scores.get(i, j - 1) + Self::INSERTION_SCORE;
                let deletion_score = self.scores.get(i - 1, j) + Self::DELETION_SCORE;
                let equality_score = if self.old.as_bytes()[i - 1] == self.new.as_bytes()[j - 1] {
                    self.scores.get(i - 1, j - 1) + Self::EQUALITY_SCORE
                } else {
                    isize::MIN
                };
                let score = insertion_score.max(deletion_score).max(equality_score);
                self.scores.set(i, j, score);
            }
        }

        let mut max_score = isize::MIN;
        let mut best_row = self.old_text_ix;
        let mut best_col = self.new_text_ix;
        for i in self.old_text_ix..=self.old.len() {
            for j in self.new_text_ix..=self.new.len() {
                let score = self.scores.get(i, j);
                if score > max_score {
                    max_score = score;
                    best_row = i;
                    best_col = j;
                }
            }
        }

        let hunks = self.backtrack(best_row, best_col);
        self.old_text_ix = best_row;
        self.new_text_ix = best_col;
        hunks
    }

    fn backtrack(&self, old_text_ix: usize, new_text_ix: usize) -> Vec<Hunk> {
        let mut hunks = Vec::new();
        let mut i = old_text_ix;
        let mut j = new_text_ix;
        while (i, j) != (self.old_text_ix, self.new_text_ix) {
            let insertion_score = if j > self.new_text_ix {
                Some((i, j - 1))
            } else {
                None
            };
            let deletion_score = if i > self.old_text_ix {
                Some((i - 1, j))
            } else {
                None
            };
            let equality_score = if i > self.old_text_ix && j > self.new_text_ix {
                if self.old.as_bytes()[i - 1] == self.new.as_bytes()[j - 1] {
                    Some((i - 1, j - 1))
                } else {
                    None
                }
            } else {
                None
            };

            let (prev_i, prev_j) = [insertion_score, deletion_score, equality_score]
                .iter()
                .max_by_key(|cell| cell.map(|(i, j)| self.scores.get(i, j)))
                .unwrap()
                .unwrap();

            if prev_i == i && prev_j == j - 1 {
                if let Some(Hunk::Insert { text }) = hunks.last_mut() {
                    text.insert_str(0, &self.new[prev_j..j]);
                } else {
                    hunks.push(Hunk::Insert {
                        text: self.new[prev_j..j].to_string(),
                    })
                }
            } else if prev_i == i - 1 && prev_j == j {
                if let Some(Hunk::Remove { len }) = hunks.last_mut() {
                    *len += 1;
                } else {
                    hunks.push(Hunk::Remove { len: 1 })
                }
            } else {
                if let Some(Hunk::Keep { len }) = hunks.last_mut() {
                    *len += 1;
                } else {
                    hunks.push(Hunk::Keep { len: 1 })
                }
            }

            i = prev_i;
            j = prev_j;
        }
        hunks.reverse();
        hunks
    }

    pub fn finish(self) -> Vec<Hunk> {
        self.backtrack(self.old.len(), self.new.len())
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
        // dbg!(diff.push_new(" world"));
        dbg!(diff.finish());
    }
}
