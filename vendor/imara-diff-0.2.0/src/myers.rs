use std::ptr::NonNull;

use crate::intern::Token;
use crate::myers::middle_snake::{MiddleSnakeSearch, SearchResult};
use crate::myers::slice::FileSlice;
use crate::util::sqrt;

mod middle_snake;
mod preprocess;
mod slice;

pub fn diff(
    before: &[Token],
    after: &[Token],
    removed: &mut [bool],
    added: &mut [bool],
    minimal: bool,
) {
    // Preprocess the files by removing parts of the file that are not contained in the other file at all.
    // This process remaps the token indices so we have to account for that during the rest of the diff
    let (before, after) = preprocess::preprocess(before, after, removed, added);

    // Perform the actual diff
    Myers::new(before.tokens.len(), after.tokens.len()).run(
        FileSlice::new(&before, removed),
        FileSlice::new(&after, added),
        minimal,
    );
}

const HEUR_MIN_COST: u32 = 256;
const MAX_COST_MIN: u32 = 256;

pub struct Myers {
    kvec: NonNull<[i32]>,
    kforward: NonNull<i32>,
    kbackward: NonNull<i32>,
    max_cost: u32,
}

impl Drop for Myers {
    fn drop(&mut self) {
        unsafe { drop(Box::from_raw(self.kvec.as_ptr())) }
    }
}

impl Myers {
    fn new(len1: usize, len2: usize) -> Self {
        let ndiags = len1 + len2 + 3;
        let kvec: *mut [i32] = Box::into_raw(vec![0; 2 * ndiags + 2].into_boxed_slice());
        let (kforward, kbackward) = unsafe {
            (
                NonNull::new_unchecked((kvec as *mut i32).add(len2 + 1)),
                NonNull::new_unchecked((kvec as *mut i32).add(ndiags + len2 + 1)),
            )
        };
        Self {
            kvec: unsafe { NonNull::new_unchecked(kvec) },
            kforward,
            kbackward,
            max_cost: sqrt(ndiags).max(MAX_COST_MIN),
        }
    }

    fn run<'f>(&mut self, mut file1: FileSlice<'f>, mut file2: FileSlice<'f>, mut need_min: bool) {
        loop {
            file1.strip_common(&mut file2);

            if file1.is_empty() {
                file2.mark_changed();
                return;
            } else if file2.is_empty() {
                file1.mark_changed();
                return;
            }

            let split = self.split(&file1, &file2, need_min);
            self.run(
                file1.borrow().slice(..split.token_idx1 as u32),
                file2.borrow().slice(..split.token_idx2 as u32),
                split.minimized_lo,
            );

            file1 = file1.slice(split.token_idx1 as u32..);
            file2 = file2.slice(split.token_idx2 as u32..);
            need_min = split.minimized_hi
        }
    }

    /// See "An O(ND) Difference Algorithm and its Variations", by Eugene Myers.
    /// Basically considers a "box" (off1, off2, lim1, lim2) and scan from both
    /// the forward diagonal starting from (off1, off2) and the backward diagonal
    /// starting from (lim1, lim2). If the K values on the same diagonal crosses
    /// returns the furthest point of reach. We might encounter expensive edge cases
    /// using this algorithm, so a little bit of heuristic is needed to cut the
    /// search and to return a suboptimal point.
    fn split(&mut self, file1: &FileSlice, file2: &FileSlice, need_min: bool) -> Split {
        let mut forward_search =
            unsafe { MiddleSnakeSearch::<false>::new(self.kforward, file1, file2) };
        let mut backwards_search =
            unsafe { MiddleSnakeSearch::<true>::new(self.kbackward, file1, file2) };
        let is_odd = (file2.len() - file2.len()) & 1 != 0;

        let mut ec = 0;

        while ec <= self.max_cost {
            let mut found_snake = false;
            forward_search.next_d();
            if is_odd {
                if let Some(res) = forward_search.run(file1, file2, |k, token_idx1| {
                    backwards_search.contains(k)
                        && backwards_search.x_pos_at_diagonal(k) <= token_idx1
                }) {
                    match res {
                        SearchResult::Snake => found_snake = true,
                        SearchResult::Found {
                            token_idx1,
                            token_idx2,
                        } => {
                            return Split {
                                token_idx1,
                                token_idx2,
                                minimized_lo: true,
                                minimized_hi: true,
                            };
                        }
                    }
                }
            } else {
                found_snake |= forward_search.run(file1, file2, |_, _| false).is_some()
            };

            backwards_search.next_d();
            if !is_odd {
                if let Some(res) = backwards_search.run(file1, file2, |k, token_idx1| {
                    forward_search.contains(k) && token_idx1 <= forward_search.x_pos_at_diagonal(k)
                }) {
                    match res {
                        SearchResult::Snake => found_snake = true,
                        SearchResult::Found {
                            token_idx1,
                            token_idx2,
                        } => {
                            return Split {
                                token_idx1,
                                token_idx2,
                                minimized_lo: true,
                                minimized_hi: true,
                            };
                        }
                    }
                }
            } else {
                found_snake |= backwards_search.run(file1, file2, |_, _| false).is_some()
            };

            if need_min {
                continue;
            }

            // If the edit cost is above the heuristic trigger and if
            // we got a good snake, we sample current diagonals to see
            // if some of them have reached an "interesting" path. Our
            // measure is a function of the distance from the diagonal
            // corner (i1 + i2) penalized with the distance from the
            // mid diagonal itself. If this value is above the current
            // edit cost times a magic factor (XDL_K_HEUR) we consider
            // it interesting.
            if found_snake && ec > HEUR_MIN_COST {
                if let Some((token_idx1, token_idx2)) = forward_search.found_snake(ec, file1, file2)
                {
                    return Split {
                        token_idx1,
                        token_idx2,
                        minimized_lo: true,
                        minimized_hi: false,
                    };
                }

                if let Some((token_idx1, token_idx2)) =
                    backwards_search.found_snake(ec, file1, file2)
                {
                    return Split {
                        token_idx1,
                        token_idx2,
                        minimized_lo: false,
                        minimized_hi: true,
                    };
                }
            }

            ec += 1;
        }

        let (distance_forward, token_idx1_forward) = forward_search.best_position(file1, file2);
        let (distance_backwards, token_idx1_backwards) =
            backwards_search.best_position(file1, file2);
        if distance_forward > file1.len() as isize + file2.len() as isize - distance_backwards {
            Split {
                token_idx1: token_idx1_forward,
                token_idx2: (distance_forward - token_idx1_forward as isize) as i32,
                minimized_lo: true,
                minimized_hi: false,
            }
        } else {
            Split {
                token_idx1: token_idx1_backwards,
                token_idx2: (distance_backwards - token_idx1_backwards as isize) as i32,
                minimized_lo: false,
                minimized_hi: true,
            }
        }
    }
}

#[derive(Debug)]
struct Split {
    token_idx1: i32,
    token_idx2: i32,
    minimized_lo: bool,
    minimized_hi: bool,
}

// /// the mapping performed during preprocessing makes it impossible to directly call
// /// the `sink` during the diff itself. Instead `file.changed` is set to true for all
// /// tokens that are changed
// /// below these arrays are used to call the sink function
// fn process_changes_with_sink(
//     before: &PreprocessedFile,
//     after: &PreprocessedFile,
//     sink: &mut impl Sink,
// ) {
//     let before_end = before.changed.len() as u32 + before.offset;
//     let after_end = after.changed.len() as u32 + after.offset;

//     let mut before = before
//         .changed
//         .iter()
//         .enumerate()
//         .map(|(i, removed)| (i as u32 + before.offset, *removed));

//     let mut after = after
//         .changed
//         .iter()
//         .enumerate()
//         .map(|(i, inserted)| (i as u32 + after.offset, *inserted));

//     let mut next1 = before.next();
//     let mut next2 = after.next();

//     while let (Some((before_pos, removed)), Some((after_pos, inserted))) = (next1, next2) {
//         if !(removed | inserted) {
//             next1 = before.next();
//             next2 = after.next();
//             continue;
//         }

//         let mut hunk_before = before_pos..before_pos;
//         let mut hunk_after = after_pos..after_pos;
//         if removed {
//             let end = before.find(|(_, changed)| !changed);
//             next1 = end.map(|(end, _)| (end, false));
//             hunk_before.end = end.map_or(before_end, |(end, _)| end);
//         };

//         if inserted {
//             let end = after.find(|(_, changed)| !changed);
//             next2 = end.map(|(end, _)| (end, false));
//             hunk_after.end = end.map_or(after_end, |(end, _)| end);
//         }

//         sink.process_change(hunk_before, hunk_after);
//     }

//     if let Some((before_pos, _)) = next1 {
//         sink.process_change(before_pos..before_end, after_end..after_end);
//     } else if let Some((after_pos, _)) = next2 {
//         sink.process_change(before_end..before_end, after_pos..after_end);
//     }
// }
