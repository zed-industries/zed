use crate::histogram::lcs::find_lcs;
use crate::histogram::list_pool::{ListHandle, ListPool};
use crate::intern::Token;
use crate::myers;

mod lcs;
mod list_pool;

const MAX_CHAIN_LEN: u32 = 63;

struct Histogram {
    token_occurrences: Vec<ListHandle>,
    pool: ListPool,
}

pub fn diff(
    before: &[Token],
    after: &[Token],
    removed: &mut [bool],
    added: &mut [bool],
    num_tokens: u32,
) {
    let mut histogram = Histogram::new(num_tokens);
    histogram.run(before, after, removed, added);
}

impl Histogram {
    fn new(num_buckets: u32) -> Histogram {
        Histogram {
            token_occurrences: vec![ListHandle::default(); num_buckets as usize],
            pool: ListPool::new(2 * num_buckets),
        }
    }

    fn clear(&mut self) {
        self.pool.clear();
    }

    fn token_occurrences(&self, token: Token) -> &[u32] {
        self.token_occurrences[token.0 as usize].as_slice(&self.pool)
    }

    fn num_token_occurrences(&self, token: Token) -> u32 {
        self.token_occurrences[token.0 as usize].len(&self.pool)
    }

    fn populate(&mut self, file: &[Token]) {
        for (i, &token) in file.iter().enumerate() {
            self.token_occurrences[token.0 as usize].push(i as u32, &mut self.pool);
        }
    }

    fn run(
        &mut self,
        mut before: &[Token],
        mut after: &[Token],
        mut removed: &mut [bool],
        mut added: &mut [bool],
    ) {
        loop {
            if before.is_empty() {
                added.fill(true);
                return;
            } else if after.is_empty() {
                removed.fill(true);
                return;
            }

            self.populate(before);
            match find_lcs(before, after, self) {
                // no lcs was found, that means that file1 and file2 two have nothing in common
                Some(lcs) if lcs.len == 0 => {
                    added.fill(true);
                    removed.fill(true);
                    return;
                }
                Some(lcs) => {
                    self.run(
                        &before[..lcs.before_start as usize],
                        &after[..lcs.after_start as usize],
                        &mut removed[..lcs.before_start as usize],
                        &mut added[..lcs.after_start as usize],
                    );

                    // this is equivalent to (tail) recursion but implement as a loop for efficeny reasons
                    let before_end = lcs.before_start + lcs.len;
                    before = &before[before_end as usize..];
                    removed = &mut removed[before_end as usize..];

                    let after_end = lcs.after_start + lcs.len;
                    after = &after[after_end as usize..];
                    added = &mut added[after_end as usize..];
                }
                None => {
                    // we are diffing two extremely large repetitive files
                    // this is a worst case for histogram diff with O(N^2) performance
                    // fallback to myers to maintain linear time complxity
                    myers::diff(before, after, removed, added, false);
                    return;
                }
            }
        }
    }
}
