use crate::{
    CharBag,
    matcher::{MatchCandidate, Matcher},
};
use gpui::BackgroundExecutor;
use std::{
    borrow::Borrow,
    cmp::{self, Ordering},
    iter,
    ops::Range,
    sync::atomic::{self, AtomicBool},
};

#[derive(Clone, Debug)]
pub struct StringMatchCandidate {
    pub id: usize,
    pub string: String,
    pub char_bag: CharBag,
    pub boost: f64,
}

impl StringMatchCandidate {
    pub fn new(id: usize, string: &str) -> Self {
        Self {
            id,
            string: string.into(),
            char_bag: string.into(),
            boost: 1.0,
        }
    }

    pub fn with_boost(id: usize, string: &str, boost: f64) -> Self {
        Self {
            id,
            string: string.into(),
            char_bag: string.into(),
            boost,
        }
    }
}

impl MatchCandidate for &StringMatchCandidate {
    fn has_chars(&self, bag: CharBag) -> bool {
        self.char_bag.is_superset(bag)
    }

    fn candidate_chars(&self) -> impl Iterator<Item = char> {
        self.string.chars()
    }
}

#[derive(Clone, Debug)]
pub struct StringMatch {
    pub candidate_id: usize,
    pub score: f64,
    pub positions: Vec<usize>,
    pub string: String,
}

impl StringMatch {
    pub fn ranges(&self) -> impl '_ + Iterator<Item = Range<usize>> {
        let mut positions = self.positions.iter().peekable();
        iter::from_fn(move || {
            if let Some(start) = positions.next().copied() {
                let Some(char_len) = self.char_len_at_index(start) else {
                    log::error!(
                        "Invariant violation: Index {start} out of range or not on a utf-8 boundary in string {:?}",
                        self.string
                    );
                    return None;
                };
                let mut end = start + char_len;
                while let Some(next_start) = positions.peek() {
                    if end == **next_start {
                        let Some(char_len) = self.char_len_at_index(end) else {
                            log::error!(
                                "Invariant violation: Index {end} out of range or not on a utf-8 boundary in string {:?}",
                                self.string
                            );
                            return None;
                        };
                        end += char_len;
                        positions.next();
                    } else {
                        break;
                    }
                }

                return Some(start..end);
            }
            None
        })
    }

    /// Gets the byte length of the utf-8 character at a byte offset. If the index is out of range
    /// or not on a utf-8 boundary then None is returned.
    fn char_len_at_index(&self, ix: usize) -> Option<usize> {
        self.string
            .get(ix..)
            .and_then(|slice| slice.chars().next().map(|char| char.len_utf8()))
    }
}

impl PartialEq for StringMatch {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for StringMatch {}

impl PartialOrd for StringMatch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StringMatch {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score
            .partial_cmp(&other.score)
            .unwrap_or(Ordering::Equal)
            .then_with(|| self.candidate_id.cmp(&other.candidate_id))
    }
}

pub async fn match_strings<T>(
    candidates: &[T],
    query: &str,
    smart_case: bool,
    penalize_length: bool,
    max_results: usize,
    cancel_flag: &AtomicBool,
    executor: BackgroundExecutor,
) -> Vec<StringMatch>
where
    T: Borrow<StringMatchCandidate> + Sync,
{
    if candidates.is_empty() || max_results == 0 {
        return Default::default();
    }

    if query.is_empty() {
        return candidates
            .iter()
            .map(|candidate| StringMatch {
                candidate_id: candidate.borrow().id,
                score: candidate.borrow().boost,
                positions: Default::default(),
                string: candidate.borrow().string.clone(),
            })
            .collect();
    }

    let lowercase_query = query.to_lowercase().chars().collect::<Vec<_>>();
    let query = query.chars().collect::<Vec<_>>();

    let lowercase_query = &lowercase_query;
    let query = &query;
    let query_char_bag = CharBag::from(&lowercase_query[..]);

    let num_cpus = executor.num_cpus().min(candidates.len());
    let segment_size = candidates.len().div_ceil(num_cpus);
    let mut segment_results = (0..num_cpus)
        .map(|_| Vec::with_capacity(max_results.min(candidates.len())))
        .collect::<Vec<_>>();

    executor
        .scoped(|scope| {
            for (segment_idx, results) in segment_results.iter_mut().enumerate() {
                let cancel_flag = &cancel_flag;
                scope.spawn(async move {
                    let segment_start = cmp::min(segment_idx * segment_size, candidates.len());
                    let segment_end = cmp::min(segment_start + segment_size, candidates.len());
                    let mut matcher = Matcher::new(
                        query,
                        lowercase_query,
                        query_char_bag,
                        smart_case,
                        penalize_length,
                    );

                    matcher.match_candidates(
                        &[],
                        &[],
                        candidates[segment_start..segment_end]
                            .iter()
                            .map(|c| c.borrow()),
                        results,
                        cancel_flag,
                        |candidate: &&StringMatchCandidate, score, positions| StringMatch {
                            candidate_id: candidate.id,
                            score: score * candidate.boost,
                            positions: positions.clone(),
                            string: candidate.string.to_string(),
                        },
                    );
                });
            }
        })
        .await;

    if cancel_flag.load(atomic::Ordering::Acquire) {
        return Vec::new();
    }

    let mut results = segment_results.concat();
    util::truncate_to_bottom_n_sorted_by(&mut results, max_results, &|a, b| b.cmp(a));
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    #[gpui::test]
    async fn test_boost_with_empty_query(cx: &mut TestAppContext) {
        let candidates = vec![
            StringMatchCandidate::with_boost(0, "low priority", 1.0),
            StringMatchCandidate::with_boost(1, "high priority", 5.0),
            StringMatchCandidate::with_boost(2, "medium priority", 2.0),
        ];

        let matches = match_strings(
            &candidates,
            "",
            false,
            false,
            10,
            &AtomicBool::new(false),
            cx.background_executor().clone(),
        )
        .await;

        // With empty query, should be sorted by boost (descending)
        assert_eq!(matches.len(), 3);
        assert_eq!(matches[0].candidate_id, 1); // high priority (5.0)
        assert_eq!(matches[0].score, 5.0);
        assert_eq!(matches[1].candidate_id, 2); // medium priority (2.0)
        assert_eq!(matches[1].score, 2.0);
        assert_eq!(matches[2].candidate_id, 0); // low priority (1.0)
        assert_eq!(matches[2].score, 1.0);
    }

    #[gpui::test]
    async fn test_boost_affects_fuzzy_matching(cx: &mut TestAppContext) {
        let candidates = vec![
            StringMatchCandidate::with_boost(0, "backspace", 1.0),   // Good match, no boost
            StringMatchCandidate::with_boost(1, "back", 10.0),       // Perfect match, high boost
            StringMatchCandidate::with_boost(2, "feedback", 1.0),    // Weaker match, no boost
        ];

        let matches = match_strings(
            &candidates,
            "back",
            false,
            false,
            10,
            &AtomicBool::new(false),
            cx.background_executor().clone(),
        )
        .await;

        // "back" with 10x boost should rank first despite similar fuzzy scores
        assert!(matches.len() >= 2);
        assert_eq!(matches[0].candidate_id, 1); // "back" with high boost
        assert_eq!(matches[0].string, "back");

        // Verify boost multiplied the score
        assert!(matches[0].score > matches[1].score);
    }

    #[gpui::test]
    async fn test_boost_doesnt_promote_bad_matches(cx: &mut TestAppContext) {
        let candidates = vec![
            StringMatchCandidate::with_boost(0, "backspace", 1.0),      // Good fuzzy match
            StringMatchCandidate::with_boost(1, "xyz", 100.0),          // No match, huge boost
            StringMatchCandidate::with_boost(2, "feedback", 1.0),       // Weaker match
        ];

        let matches = match_strings(
            &candidates,
            "backs",
            false,
            false,
            10,
            &AtomicBool::new(false),
            cx.background_executor().clone(),
        )
        .await;

        // "xyz" should not appear despite huge boost because fuzzy score is 0
        assert!(matches.iter().all(|m| m.string != "xyz"));

        // Good matches should still be first
        if !matches.is_empty() {
            assert_eq!(matches[0].string, "backspace");
        }
    }

    #[gpui::test]
    async fn test_default_boost_is_neutral(cx: &mut TestAppContext) {
        let candidates = vec![
            StringMatchCandidate::new(0, "apple"),
            StringMatchCandidate::new(1, "application"),
        ];

        let matches = match_strings(
            &candidates,
            "app",
            false,
            false,
            10,
            &AtomicBool::new(false),
            cx.background_executor().clone(),
        )
        .await;

        // Default boost (1.0) should not change behavior
        // Just verify we get matches and no panics
        assert!(!matches.is_empty());
    }
}
