use std::ops::Range;

use language::{Bias, Rope};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum SearchDirection {
    Up,
    Left,
    Diagonal,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SearchState {
    cost: u32,
    direction: SearchDirection,
}

impl SearchState {
    fn new(cost: u32, direction: SearchDirection) -> Self {
        Self { cost, direction }
    }
}

struct SearchMatrix {
    cols: usize,
    data: Vec<SearchState>,
}

impl SearchMatrix {
    fn new(rows: usize, cols: usize) -> Self {
        SearchMatrix {
            cols,
            data: vec![SearchState::new(0, SearchDirection::Diagonal); rows * cols],
        }
    }

    fn get(&self, row: usize, col: usize) -> SearchState {
        self.data[row * self.cols + col]
    }

    fn set(&mut self, row: usize, col: usize, cost: SearchState) {
        self.data[row * self.cols + col] = cost;
    }
}

pub fn search(haystack: &Rope, needle: &str) -> Range<usize> {
    const INSERTION_COST: u32 = 3;
    const DELETION_COST: u32 = 10;
    const WHITESPACE_INSERTION_COST: u32 = 1;
    const WHITESPACE_DELETION_COST: u32 = 1;

    let haystack_len = haystack.len();
    let needle_len = needle.len();
    let mut matrix = SearchMatrix::new(needle_len + 1, haystack_len + 1);
    let mut leading_deletion_cost = 0_u32;
    for (row, needle_byte) in needle.bytes().enumerate() {
        let deletion_cost = if needle_byte.is_ascii_whitespace() {
            WHITESPACE_DELETION_COST
        } else {
            DELETION_COST
        };

        leading_deletion_cost = leading_deletion_cost.saturating_add(deletion_cost);
        matrix.set(
            row + 1,
            0,
            SearchState::new(leading_deletion_cost, SearchDirection::Diagonal),
        );

        for (col, haystack_byte) in haystack
            .bytes_in_range(0..haystack.len())
            .flatten()
            .enumerate()
        {
            let insertion_cost = if haystack_byte.is_ascii_whitespace() {
                WHITESPACE_INSERTION_COST
            } else {
                INSERTION_COST
            };

            let up = SearchState::new(
                matrix.get(row, col + 1).cost.saturating_add(deletion_cost),
                SearchDirection::Up,
            );
            let left = SearchState::new(
                matrix.get(row + 1, col).cost.saturating_add(insertion_cost),
                SearchDirection::Left,
            );
            let diagonal = SearchState::new(
                if needle_byte == *haystack_byte {
                    matrix.get(row, col).cost
                } else {
                    matrix
                        .get(row, col)
                        .cost
                        .saturating_add(deletion_cost + insertion_cost)
                },
                SearchDirection::Diagonal,
            );
            matrix.set(row + 1, col + 1, up.min(left).min(diagonal));
        }
    }

    // Traceback to find the best match
    let mut best_haystack_end = haystack_len;
    let mut best_cost = u32::MAX;
    for col in 1..=haystack_len {
        let cost = matrix.get(needle_len, col).cost;
        if cost < best_cost {
            best_cost = cost;
            best_haystack_end = col;
        }
    }

    let mut query_ix = needle_len;
    let mut haystack_ix = best_haystack_end;
    while query_ix > 0 && haystack_ix > 0 {
        let current = matrix.get(query_ix, haystack_ix);
        match current.direction {
            SearchDirection::Diagonal => {
                query_ix -= 1;
                haystack_ix -= 1;
            }
            SearchDirection::Up => {
                query_ix -= 1;
            }
            SearchDirection::Left => {
                haystack_ix -= 1;
            }
        }
    }

    let mut start = haystack.offset_to_point(haystack.clip_offset(haystack_ix, Bias::Left));
    start.column = 0;
    let mut end = haystack.offset_to_point(haystack.clip_offset(best_haystack_end, Bias::Right));
    if end.column > 0 {
        end.column = haystack.line_len(end.row);
    }

    haystack.point_to_offset(start)..haystack.point_to_offset(end)
}
