use std::ptr::NonNull;

use crate::myers::slice::FileSlice;
use crate::util::{common_postfix, common_prefix};

const SNAKE_CNT: u32 = 20;
const K_HEUR: u32 = 4;

pub struct MiddleSnakeSearch<const BACK: bool> {
    kvec: NonNull<i32>,
    kmin: i32,
    kmax: i32,
    dmin: i32,
    dmax: i32,
}

impl<const BACK: bool> MiddleSnakeSearch<BACK> {
    /// # Safety
    /// `data` must be valid for reads and writes between `-file2.len() - 1` and `file1.len() + 1`
    pub unsafe fn new(data: NonNull<i32>, file1: &FileSlice, file2: &FileSlice) -> Self {
        let dmin = -(file2.len() as i32);
        let dmax = file1.len() as i32;
        let kmid = if BACK { dmin + dmax } else { 0 };
        let mut res = Self {
            kvec: data,
            kmin: kmid,
            kmax: kmid,
            dmin,
            dmax,
        };
        let init = if BACK { file1.len() as i32 } else { 0 };
        res.write_xpos_at_diagonal(kmid, init);
        res
    }

    pub fn contains(&self, k: i32) -> bool {
        (self.kmin..=self.kmax).contains(&k)
    }

    pub fn bounds_check(&self, k: i32) {
        debug_assert!((self.dmin - 1..=self.dmax + 1).contains(&k));
    }

    fn write_xpos_at_diagonal(&mut self, k: i32, token_idx1: i32) {
        self.bounds_check(k);
        unsafe { self.kvec.as_ptr().offset(k as isize).write(token_idx1) }
    }

    pub fn x_pos_at_diagonal(&self, diagonal: i32) -> i32 {
        self.bounds_check(diagonal);
        unsafe { self.kvec.as_ptr().offset(diagonal as isize).read() }
    }

    pub fn pos_at_diagonal(&self, diagonal: i32) -> (i32, i32) {
        self.bounds_check(diagonal);
        let token_idx1 = unsafe { self.kvec.as_ptr().offset(diagonal as isize).read() };
        let token_idx2 = token_idx1 - diagonal;
        (token_idx1, token_idx2)
    }

    /// We need to extend the diagonal "domain" by one. If the next
    /// values exits the box boundaries we need to change it in the
    /// opposite direction because (max - min) must be a power of
    /// two.
    ///
    /// Also we initialize the external K value to -1 so that we can
    /// avoid extra conditions in the check inside the core loop.
    pub fn next_d(&mut self) {
        let init_val = if BACK {
            // value should always be larger then bounds
            i32::MAX
        } else {
            // value should always be smaller then bounds
            i32::MIN
        };

        if self.kmin > self.dmin {
            self.kmin -= 1;
            self.write_xpos_at_diagonal(self.kmin - 1, init_val);
        } else {
            self.kmin += 1;
        }

        if self.kmax < self.dmax {
            self.kmax += 1;
            self.write_xpos_at_diagonal(self.kmax + 1, init_val);
        } else {
            self.kmax -= 1;
        }
    }

    pub fn run(
        &mut self,
        file1: &FileSlice,
        file2: &FileSlice,
        mut f: impl FnMut(i32, i32) -> bool,
    ) -> Option<SearchResult> {
        let mut res = None;
        let mut k = self.kmax;
        while k >= self.kmin {
            let mut token_idx1 = if BACK {
                if self.x_pos_at_diagonal(k - 1) < self.x_pos_at_diagonal(k + 1) {
                    self.x_pos_at_diagonal(k - 1)
                } else {
                    self.x_pos_at_diagonal(k + 1) - 1
                }
            } else if self.x_pos_at_diagonal(k - 1) >= self.x_pos_at_diagonal(k + 1) {
                self.x_pos_at_diagonal(k - 1) + 1
            } else {
                self.x_pos_at_diagonal(k + 1)
            };

            let mut token_idx2 = token_idx1 - k;
            let off = if BACK {
                if token_idx1 > 0 && token_idx2 > 0 {
                    let tokens1 = &file1.tokens[..token_idx1 as usize];
                    let tokens2 = &file2.tokens[..token_idx2 as usize];
                    common_postfix(tokens1, tokens2)
                } else {
                    0
                }
            } else if token_idx1 < file1.len() as i32 && token_idx2 < file2.len() as i32 {
                let tokens1 = &file1.tokens[token_idx1 as usize..];
                let tokens2 = &file2.tokens[token_idx2 as usize..];
                common_prefix(tokens1, tokens2)
            } else {
                0
            };

            if off > SNAKE_CNT {
                res = Some(SearchResult::Snake)
            }

            if BACK {
                token_idx1 -= off as i32;
                token_idx2 -= off as i32;
            } else {
                token_idx1 += off as i32;
                token_idx2 += off as i32;
            }
            self.write_xpos_at_diagonal(k, token_idx1);

            if f(k, token_idx1) {
                return Some(SearchResult::Found {
                    token_idx1,
                    token_idx2,
                });
            }

            k -= 2;
        }

        res
    }

    pub fn best_position(&self, file1: &FileSlice, file2: &FileSlice) -> (isize, i32) {
        let mut best_distance: isize = if BACK { isize::MAX } else { -1 };
        let mut best_token_idx1 = if BACK { i32::MAX } else { -1 };
        let mut k = self.kmax;
        while k >= self.kmin {
            let mut token_idx1 = self.x_pos_at_diagonal(k);
            if BACK {
                token_idx1 = token_idx1.max(0);
            } else {
                token_idx1 = token_idx1.min(file1.len() as i32);
            }
            let mut token_idx2 = token_idx1 - k;
            if BACK {
                if token_idx2 < 0 {
                    token_idx1 = k;
                    token_idx2 = 0;
                }
            } else if token_idx2 > file2.len() as i32 {
                token_idx1 = file2.len() as i32 + k;
                token_idx2 = file2.len() as i32;
            }

            let distance = token_idx1 as isize + token_idx2 as isize;
            if BACK && distance < best_distance || !BACK && distance > best_distance {
                best_distance = distance;
                best_token_idx1 = token_idx1;
            }

            k -= 2;
        }
        (best_distance, best_token_idx1)
    }

    pub fn found_snake(&self, ec: u32, file1: &FileSlice, file2: &FileSlice) -> Option<(i32, i32)> {
        let mut best_score = 0;
        let mut best_token_idx1 = 0;
        let mut best_token_idx2 = 0;
        let mut k = self.kmax;
        while k >= self.kmin {
            let (token_idx1, token_idx2) = self.pos_at_diagonal(k);
            if BACK {
                if !(0..file1.len() as i32 - SNAKE_CNT as i32).contains(&token_idx1) {
                    k -= 2;
                    continue;
                }
                if !(0..file2.len() as i32 - SNAKE_CNT as i32).contains(&token_idx2) {
                    k -= 2;
                    continue;
                }
            } else {
                if !(SNAKE_CNT as i32..file1.len() as i32).contains(&token_idx1) {
                    k -= 2;
                    continue;
                }
                if !(SNAKE_CNT as i32..file2.len() as i32).contains(&token_idx2) {
                    k -= 2;
                    continue;
                }
            }

            let main_diagonal_distance = k.unsigned_abs() as usize;
            let distance = if BACK {
                (file1.len() - token_idx1 as u32) + (file2.len() - token_idx2 as u32)
            } else {
                token_idx1 as u32 + token_idx2 as u32
            };
            let score = distance as usize + main_diagonal_distance;
            if score > (K_HEUR * ec) as usize && score > best_score {
                let is_snake = if BACK {
                    file1.tokens[token_idx1 as usize..]
                        .iter()
                        .zip(&file2.tokens[token_idx2 as usize..])
                        .take(SNAKE_CNT as usize)
                        .all(|(token1, token2)| token1 == token2)
                } else {
                    file1.tokens[..token_idx1 as usize]
                        .iter()
                        .zip(&file2.tokens[..token_idx2 as usize])
                        .rev()
                        .take(SNAKE_CNT as usize)
                        .all(|(token1, token2)| token1 == token2)
                };
                if is_snake {
                    best_token_idx1 = token_idx1;
                    best_token_idx2 = token_idx2;
                    best_score = score
                }
            }

            k -= 2;
        }

        (best_score > 0).then_some((best_token_idx1, best_token_idx2))
    }
}

pub enum SearchResult {
    Snake,
    Found { token_idx1: i32, token_idx2: i32 },
}
