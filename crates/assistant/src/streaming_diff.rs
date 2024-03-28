use collections::HashMap;
use ordered_float::OrderedFloat;
use std::{
    cmp,
    fmt::{self, Debug},
    ops::Range,
};

struct Matrix {
    cells: Vec<f64>,
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
        self.cells.resize(rows * cols, 0.);
        self.rows = rows;
        self.cols = cols;
    }

    fn get(&self, row: usize, col: usize) -> f64 {
        if row >= self.rows {
            panic!("row out of bounds")
        }

        if col >= self.cols {
            panic!("col out of bounds")
        }
        self.cells[col * self.rows + row]
    }

    fn set(&mut self, row: usize, col: usize, value: f64) {
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

pub struct StreamingDiff {
    old: Vec<char>,
    new: Vec<char>,
    scores: Matrix,
    old_text_ix: usize,
    new_text_ix: usize,
    equal_runs: HashMap<(usize, usize), u32>,
}

impl StreamingDiff {
    const INSERTION_SCORE: f64 = -1.;
    const DELETION_SCORE: f64 = -20.;
    const EQUALITY_BASE: f64 = 1.8;
    const MAX_EQUALITY_EXPONENT: i32 = 16;

    pub fn new(old: String) -> Self {
        let old = old.chars().collect::<Vec<_>>();
        let mut scores = Matrix::new();
        scores.resize(old.len() + 1, 1);
        for i in 0..=old.len() {
            scores.set(i, 0, i as f64 * Self::DELETION_SCORE);
        }
        Self {
            old,
            new: Vec::new(),
            scores,
            old_text_ix: 0,
            new_text_ix: 0,
            equal_runs: Default::default(),
        }
    }

    pub fn push_new(&mut self, text: &str) -> Vec<Hunk> {
        self.new.extend(text.chars());
        self.scores.resize(self.old.len() + 1, self.new.len() + 1);

        for j in self.new_text_ix + 1..=self.new.len() {
            self.scores.set(0, j, j as f64 * Self::INSERTION_SCORE);
            for i in 1..=self.old.len() {
                let insertion_score = self.scores.get(i, j - 1) + Self::INSERTION_SCORE;
                let deletion_score = self.scores.get(i - 1, j) + Self::DELETION_SCORE;
                let equality_score = if self.old[i - 1] == self.new[j - 1] {
                    let mut equal_run = self.equal_runs.get(&(i - 1, j - 1)).copied().unwrap_or(0);
                    equal_run += 1;
                    self.equal_runs.insert((i, j), equal_run);

                    let exponent = cmp::min(equal_run as i32 / 4, Self::MAX_EQUALITY_EXPONENT);
                    self.scores.get(i - 1, j - 1) + Self::EQUALITY_BASE.powi(exponent)
                } else {
                    f64::NEG_INFINITY
                };

                let score = insertion_score.max(deletion_score).max(equality_score);
                self.scores.set(i, j, score);
            }
        }

        let mut max_score = f64::NEG_INFINITY;
        let mut next_old_text_ix = self.old_text_ix;
        let next_new_text_ix = self.new.len();
        for i in self.old_text_ix..=self.old.len() {
            let score = self.scores.get(i, next_new_text_ix);
            if score > max_score {
                max_score = score;
                next_old_text_ix = i;
            }
        }

        let hunks = self.backtrack(next_old_text_ix, next_new_text_ix);
        self.old_text_ix = next_old_text_ix;
        self.new_text_ix = next_new_text_ix;
        hunks
    }

    fn backtrack(&self, old_text_ix: usize, new_text_ix: usize) -> Vec<Hunk> {
        let mut pending_insert: Option<Range<usize>> = None;
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
                if self.old[i - 1] == self.new[j - 1] {
                    Some((i - 1, j - 1))
                } else {
                    None
                }
            } else {
                None
            };

            let (prev_i, prev_j) = [insertion_score, deletion_score, equality_score]
                .iter()
                .max_by_key(|cell| cell.map(|(i, j)| OrderedFloat(self.scores.get(i, j))))
                .unwrap()
                .unwrap();

            if prev_i == i && prev_j == j - 1 {
                if let Some(pending_insert) = pending_insert.as_mut() {
                    pending_insert.start = prev_j;
                } else {
                    pending_insert = Some(prev_j..j);
                }
            } else {
                if let Some(range) = pending_insert.take() {
                    hunks.push(Hunk::Insert {
                        text: self.new[range].iter().collect(),
                    });
                }

                let char_len = self.old[i - 1].len_utf8();
                if prev_i == i - 1 && prev_j == j {
                    if let Some(Hunk::Remove { len }) = hunks.last_mut() {
                        *len += char_len;
                    } else {
                        hunks.push(Hunk::Remove { len: char_len })
                    }
                } else if let Some(Hunk::Keep { len }) = hunks.last_mut() {
                    *len += char_len;
                } else {
                    hunks.push(Hunk::Keep { len: char_len })
                }
            }

            i = prev_i;
            j = prev_j;
        }

        if let Some(range) = pending_insert.take() {
            hunks.push(Hunk::Insert {
                text: self.new[range].iter().collect(),
            });
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
    use std::env;

    use super::*;
    use rand::prelude::*;

    #[gpui::test(iterations = 100)]
    fn test_random_diffs(mut rng: StdRng) {
        let old_text_len = env::var("OLD_TEXT_LEN")
            .map(|i| i.parse().expect("invalid `OLD_TEXT_LEN` variable"))
            .unwrap_or(10);
        let new_text_len = env::var("NEW_TEXT_LEN")
            .map(|i| i.parse().expect("invalid `NEW_TEXT_LEN` variable"))
            .unwrap_or(10);

        let old = util::RandomCharIter::new(&mut rng)
            .take(old_text_len)
            .collect::<String>();
        log::info!("old text: {:?}", old);

        let mut diff = StreamingDiff::new(old.clone());
        let mut hunks = Vec::new();
        let mut new_len = 0;
        let mut new = String::new();
        while new_len < new_text_len {
            let new_chunk_len = rng.gen_range(1..=new_text_len - new_len);
            let new_chunk = util::RandomCharIter::new(&mut rng)
                .take(new_len)
                .collect::<String>();
            log::info!("new chunk: {:?}", new_chunk);
            new_len += new_chunk_len;
            new.push_str(&new_chunk);
            let new_hunks = diff.push_new(&new_chunk);
            log::info!("hunks: {:?}", new_hunks);
            hunks.extend(new_hunks);
        }
        let final_hunks = diff.finish();
        log::info!("final hunks: {:?}", final_hunks);
        hunks.extend(final_hunks);

        log::info!("new text: {:?}", new);
        let mut old_ix = 0;
        let mut new_ix = 0;
        let mut patched = String::new();
        for hunk in hunks {
            match hunk {
                Hunk::Keep { len } => {
                    assert_eq!(&old[old_ix..old_ix + len], &new[new_ix..new_ix + len]);
                    patched.push_str(&old[old_ix..old_ix + len]);
                    old_ix += len;
                    new_ix += len;
                }
                Hunk::Remove { len } => {
                    old_ix += len;
                }
                Hunk::Insert { text } => {
                    assert_eq!(text, &new[new_ix..new_ix + text.len()]);
                    patched.push_str(&text);
                    new_ix += text.len();
                }
            }
        }
        assert_eq!(patched, new);
    }
}
