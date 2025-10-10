use std::collections::VecDeque;
use std::fmt::Debug;
use util::debug_panic;

use crate::{HashFrom, Occurrences};

#[derive(Debug)]
pub struct SlidingWindow<Id, T, S> {
    target: T,
    window: Occurrences<S>,
    regions: VecDeque<WeightedOverlapRegion<Id, S>>,
    numerator: u32,
    window_count: u32,
    jaccard_denominator_part: u32,
}

#[derive(Debug)]
pub struct WeightedOverlapRegion<Id, S> {
    #[cfg(debug_assertions)]
    id: Id,
    added_hashes: Vec<HashFrom<S>>,
    numerator_delta: u32,
    window_count_delta: u32,
    jaccard_denominator_delta: u32,
}

impl<Id: Debug + PartialEq, T: AsRef<Occurrences<S>>, S> SlidingWindow<Id, T, S> {
    pub fn new(target: T) -> Self {
        Self::with_capacity(target, 0)
    }

    pub fn with_capacity(target: T, capacity: usize) -> Self {
        let jaccard_denominator_part = target.as_ref().len();
        Self {
            target,
            window: Occurrences::default().into(),
            regions: VecDeque::with_capacity(capacity),
            numerator: 0,
            window_count: 0,
            jaccard_denominator_part,
        }
    }

    pub fn push_back(&mut self, id: Id, hashes: impl IntoIterator<Item = HashFrom<S>>) {
        let mut added_hashes = Vec::new();
        let mut numerator_delta = 0;
        let mut jaccard_denominator_delta = 0;
        let mut window_count_delta = 0;
        for hash in hashes {
            window_count_delta += 1;
            let target_count = self.target.as_ref().get_count(hash);
            if target_count > 0 {
                added_hashes.push(hash);
                let window_count = self.window.add_hash(hash);
                if window_count <= target_count {
                    numerator_delta += 1;
                } else {
                    jaccard_denominator_delta += 1;
                }
            }
        }
        self.numerator += numerator_delta;
        self.window_count += window_count_delta;
        self.jaccard_denominator_part += jaccard_denominator_delta;
        self.regions.push_back(WeightedOverlapRegion {
            #[cfg(debug_assertions)]
            id,
            added_hashes,
            numerator_delta,
            window_count_delta,
            jaccard_denominator_delta,
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

        for hash in removed.added_hashes {
            self.window.remove_hash(hash);
        }

        if let Some(numerator) = self.numerator.checked_sub(removed.numerator_delta)
            && let Some(window_count) = self.window_count.checked_sub(removed.window_count_delta)
            && let Some(jaccard_denominator_part) = self
                .jaccard_denominator_part
                .checked_sub(removed.jaccard_denominator_delta)
        {
            self.numerator = numerator;
            self.window_count = window_count;
            self.jaccard_denominator_part = jaccard_denominator_part;
        } else {
            debug_panic!("bug: underflow in sliding window text similarity");
        }
    }

    pub fn weighted_overlap_coefficient(&self) -> f32 {
        let denominator = self.target.as_ref().len().min(self.window_count);
        self.numerator as f32 / denominator as f32
    }

    pub fn weighted_jaccard_similarity(&self) -> f32 {
        let mut denominator = self.jaccard_denominator_part;
        if let Some(other_denominator_part) = self.window_count.checked_sub(self.window.len()) {
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

        checked_window.push_back("a b");
        dbg!(&checked_window);
        checked_window.push_back("c d e");
        dbg!(&checked_window);
        checked_window.push_back("a c");
        dbg!(&checked_window);
        checked_window.pop_front();
        dbg!(&checked_window);
        checked_window.push_back("a");
        dbg!(&checked_window);
        checked_window.pop_front();
        checked_window.push_back("g");
        checked_window.pop_front();
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
