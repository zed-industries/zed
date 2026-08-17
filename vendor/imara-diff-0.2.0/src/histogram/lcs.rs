use crate::histogram::{Histogram, MAX_CHAIN_LEN};
use crate::intern::Token;

pub(super) fn find_lcs(
    before: &[Token],
    after: &[Token],
    histogram: &mut Histogram,
) -> Option<Lcs> {
    let mut search = LcsSearch {
        lcs: Lcs::default(),
        min_occurrences: MAX_CHAIN_LEN + 1,
        found_cs: false,
    };
    search.run(before, after, histogram);
    if search.success() {
        Some(search.lcs)
    } else {
        None
    }
}

#[derive(Default, Debug)]
pub struct Lcs {
    pub before_start: u32,
    pub after_start: u32,
    pub len: u32,
}

pub struct LcsSearch {
    lcs: Lcs,
    min_occurrences: u32,
    found_cs: bool,
}

impl LcsSearch {
    fn run(&mut self, before: &[Token], after: &[Token], histogram: &mut Histogram) {
        let mut pos = 0;
        while let Some(&token) = after.get(pos as usize) {
            if histogram.num_token_occurrences(token) != 0 {
                self.found_cs = true;
                if histogram.num_token_occurrences(token) <= self.min_occurrences {
                    pos = self.update_lcs(pos, token, histogram, before, after);
                    continue;
                }
            }

            pos += 1;
        }

        histogram.clear();
    }

    fn success(&mut self) -> bool {
        !self.found_cs || self.min_occurrences <= MAX_CHAIN_LEN
    }

    fn update_lcs(
        &mut self,
        after_pos: u32,
        token: Token,
        histogram: &Histogram,
        before: &[Token],
        after: &[Token],
    ) -> u32 {
        let mut next_token_idx2 = after_pos + 1;
        let mut occurrences_iter = histogram.token_occurrences(token).iter().copied();
        let mut token_idx1 = occurrences_iter.next().unwrap();

        'occurrences_iter: loop {
            let mut occurrences = histogram.num_token_occurrences(token);
            let mut start1 = token_idx1;
            let mut start2 = after_pos;
            loop {
                if start1 == 0 || start2 == 0 {
                    break;
                }
                let token1 = before.get(start1 as usize - 1);
                let token2 = after.get(start2 as usize - 1);
                if matches!((token1, token2), (Some(token1), Some(token2)) if token1 == token2) {
                    start1 -= 1;
                    start2 -= 1;
                    let new_occurrences = histogram.num_token_occurrences(before[start1 as usize]);
                    occurrences = occurrences.min(new_occurrences);
                } else {
                    break;
                }
            }

            let mut end1 = token_idx1 + 1;
            let mut end2 = after_pos + 1;
            loop {
                let token1 = before.get(end1 as usize);
                let token2 = after.get(end2 as usize);
                if matches!((token1, token2), (Some(token1), Some(token2)) if token1 == token2) {
                    let new_occurrences = histogram.num_token_occurrences(before[end1 as usize]);
                    occurrences = occurrences.min(new_occurrences);
                    end1 += 1;
                    end2 += 1;
                } else {
                    break;
                }
            }

            if next_token_idx2 < end2 {
                next_token_idx2 = end2;
            }

            let len = end2 - start2;
            debug_assert_eq!(len, end1 - start1);
            if self.lcs.len < len || self.min_occurrences > occurrences {
                self.min_occurrences = occurrences;
                self.lcs = Lcs {
                    before_start: start1,
                    after_start: start2,
                    len,
                };
            }

            loop {
                if let Some(next_token_idx) = occurrences_iter.next() {
                    if next_token_idx > end2 {
                        token_idx1 = next_token_idx;
                        break;
                    }
                } else {
                    break 'occurrences_iter;
                }
            }
        }

        next_token_idx2
    }
}
