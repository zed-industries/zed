use std::collections::VecDeque;
use std::fmt::Debug;
use util::debug_panic;

use crate::{HashFrom, Occurrences};

#[derive(Debug)]
pub struct SlidingWindow<Id, T, S> {
    target: T,
    intersection: Occurrences<S>,
    regions: VecDeque<WeightedOverlapRegion<Id, S>>,
    numerator: u32,
    window_count: u32,
    jaccard_denominator_part: u32,
}

#[derive(Debug)]
struct WeightedOverlapRegion<Id, S> {
    #[cfg(debug_assertions)]
    id: Id,
    added_hashes: Vec<AddedHash<S>>,
    window_count_delta: u32,
}

#[derive(Debug)]
struct AddedHash<S> {
    hash: HashFrom<S>,
    target_count: u32,
}

impl<Id: Debug + PartialEq, T: AsRef<Occurrences<S>>, S> SlidingWindow<Id, T, S> {
    pub fn new(target: T) -> Self {
        Self::with_capacity(target, 0)
    }

    pub fn with_capacity(target: T, capacity: usize) -> Self {
        let jaccard_denominator_part = target.as_ref().len();
        Self {
            target,
            intersection: Occurrences::default().into(),
            regions: VecDeque::with_capacity(capacity),
            numerator: 0,
            window_count: 0,
            jaccard_denominator_part,
        }
    }

    pub fn clear_window(&mut self) {
        self.intersection.clear();
        self.regions.clear();
        self.numerator = 0;
        self.window_count = 0;
        self.jaccard_denominator_part = 0;
    }

    pub fn push_back(&mut self, id: Id, hashes: impl IntoIterator<Item = HashFrom<S>>) {
        let mut added_hashes = Vec::new();
        let mut window_count_delta = 0;
        for hash in hashes {
            window_count_delta += 1;
            let target_count = self.target.as_ref().get_count(hash);
            if target_count > 0 {
                added_hashes.push(AddedHash { hash, target_count });
                let window_hash_count = self.intersection.add_hash(hash);
                if window_hash_count <= target_count {
                    self.numerator += 1;
                } else {
                    self.jaccard_denominator_part += 1;
                }
            }
        }
        self.window_count += window_count_delta;
        self.regions.push_back(WeightedOverlapRegion {
            #[cfg(debug_assertions)]
            id,
            added_hashes,
            window_count_delta,
        });
    }

    pub fn pop_front(&mut self, id: Id) {
        let removed;
        #[cfg(debug_assertions)]
        {
            removed = self
                .regions
                .pop_front()
                .expect("No sliding window region to remove");
            debug_assert_eq!(removed.id, id);
        }

        #[cfg(not(debug_assertions))]
        {
            removed = self.regions.pop_front();
            let Some(removed) = removed else {
                return;
            };
        }

        for AddedHash { hash, target_count } in removed.added_hashes {
            let window_hash_count = self.intersection.remove_hash(hash);
            if window_hash_count < target_count {
                if let Some(numerator) = self.numerator.checked_sub(1) {
                    self.numerator = numerator;
                } else {
                    debug_panic!("bug: underflow in sliding window text similarity");
                }
            } else {
                if let Some(jaccard_denominator_part) = self.jaccard_denominator_part.checked_sub(1)
                {
                    self.jaccard_denominator_part = jaccard_denominator_part;
                } else {
                    debug_panic!("bug: underflow in sliding window text similarity");
                }
            }
        }

        if let Some(window_count) = self.window_count.checked_sub(removed.window_count_delta) {
            self.window_count = window_count;
        } else {
            debug_panic!("bug: underflow in sliding window text similarity");
        }
    }

    pub fn weighted_overlap_coefficient(&self) -> f32 {
        let denominator = self.target.as_ref().len().min(self.window_count);
        if denominator == 0 {
            0.0
        } else {
            self.numerator as f32 / denominator as f32
        }
    }

    pub fn weighted_jaccard_similarity(&self) -> f32 {
        let mut denominator = self.jaccard_denominator_part;
        if let Some(other_denominator_part) = self.window_count.checked_sub(self.intersection.len())
        {
            denominator += other_denominator_part;
        } else {
            debug_panic!("bug: underflow in sliding window text similarity");
        }
        self.numerator as f32 / denominator as f32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{IdentifierParts, Occurrences, WeightedSimilarity};

    #[test]
    fn test_sliding_window() {
        let target = Occurrences::new(IdentifierParts::within_string("a b c d"));
        let mut checked_window = CheckedSlidingWindow::new(target);

        checked_window.push_back("a");
        checked_window.pop_front();

        checked_window.push_back("a b");
        checked_window.push_back("a");
        checked_window.pop_front();
        checked_window.pop_front();

        checked_window.push_back("a b");
        checked_window.push_back("a b c");
        checked_window.pop_front();
        checked_window.push_back("a b c d");
        checked_window.pop_front();
        checked_window.pop_front();
    }

    #[derive(Debug)]
    struct CheckedSlidingWindow {
        inner: SlidingWindow<u32, Occurrences<IdentifierParts>, IdentifierParts>,
        text: String,
        first_line: u32,
        last_line: u32,
    }

    impl CheckedSlidingWindow {
        fn new(target: Occurrences<IdentifierParts>) -> Self {
            CheckedSlidingWindow {
                inner: SlidingWindow::new(target),
                text: String::new(),
                first_line: 0,
                last_line: 0,
            }
        }

        #[track_caller]
        fn push_back(&mut self, line: &str) {
            self.inner
                .push_back(self.last_line, IdentifierParts::within_string(line));
            self.text.push_str(line);
            self.text.push('\n');
            self.last_line += 1;
            self.check_after_mutation();
        }

        #[track_caller]
        fn pop_front(&mut self) {
            self.inner.pop_front(self.first_line);
            self.text.drain(0..self.text.find("\n").unwrap() + 1);
            self.first_line += 1;
            self.check_after_mutation();
        }

        #[track_caller]
        fn check_after_mutation(&self) {
            assert_eq!(
                self.inner.weighted_overlap_coefficient(),
                Occurrences::new(IdentifierParts::within_string(&self.text))
                    .weighted_overlap_coefficient(&self.inner.target),
                "weighted_overlap_coefficient"
            );
            assert_eq!(
                self.inner.weighted_jaccard_similarity(),
                Occurrences::new(IdentifierParts::within_string(&self.text))
                    .weighted_jaccard_similarity(&self.inner.target),
                "weighted_jaccard_similarity"
            );
        }
    }
}
