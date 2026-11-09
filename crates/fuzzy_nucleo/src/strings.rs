use std::{
    borrow::Borrow,
    cmp::Ordering,
    iter,
    ops::Range,
    sync::atomic::{self, AtomicBool},
};

use gpui::{BackgroundExecutor, SharedString};
use nucleo::Utf32Str;

use crate::{
    Cancelled, Case, LengthPenalty, MaxResults, Query, case_penalty, count_case_mismatches,
    is_cancelled,
    matcher::{self, LENGTH_PENALTY},
    positions_from_sorted,
};
use fuzzy::CharBag;

#[derive(Clone, Debug)]
pub struct StringMatchCandidate {
    pub id: usize,
    pub string: SharedString,
    char_bag: CharBag,
}

impl StringMatchCandidate {
    pub fn new(id: usize, string: impl Into<SharedString>) -> Self {
        Self::from_shared(id, string.into())
    }

    pub fn from_shared(id: usize, string: SharedString) -> Self {
        let char_bag = CharBag::from(string.as_ref());
        Self {
            id,
            string,
            char_bag,
        }
    }
}

#[derive(Clone, Debug)]
pub struct StringMatch {
    pub candidate_id: usize,
    pub score: f64,
    pub positions: Vec<usize>,
    pub string: SharedString,
}

impl StringMatch {
    pub fn ranges(&self) -> impl '_ + Iterator<Item = Range<usize>> {
        let mut positions = self.positions.iter().peekable();
        iter::from_fn(move || {
            let start = *positions.next()?;
            let char_len = self.char_len_at_index(start)?;
            let mut end = start + char_len;
            while let Some(next_start) = positions.peek() {
                if end == **next_start {
                    let Some(char_len) = self.char_len_at_index(end) else {
                        break;
                    };
                    end += char_len;
                    positions.next();
                } else {
                    break;
                }
            }
            Some(start..end)
        })
    }

    fn char_len_at_index(&self, ix: usize) -> Option<usize> {
        self.string
            .get(ix..)
            .and_then(|slice| slice.chars().next().map(|c| c.len_utf8()))
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
            .total_cmp(&other.score)
            .then_with(|| self.candidate_id.cmp(&other.candidate_id))
    }
}

/// Options for [`match_strings`] and [`match_strings_async`].
#[derive(Copy, Clone, Debug)]
pub struct StringMatchOptions<'a> {
    /// Defaults to all matches.
    pub max_results: MaxResults,
    /// Return nothing if this gets set while matching. Empty queries ignore it.
    pub cancel_flag: Option<&'a AtomicBool>,
    /// Defaults to [`Case::Smart`].
    pub case: Case,
    /// Defaults to [`LengthPenalty::On`].
    pub length_penalty: LengthPenalty,
}

impl Default for StringMatchOptions<'_> {
    fn default() -> Self {
        Self {
            max_results: MaxResults::All,
            cancel_flag: None,
            case: Case::Smart,
            length_penalty: LengthPenalty::On,
        }
    }
}

pub async fn match_strings_async<T>(
    candidates: &[T],
    query: &str,
    options: StringMatchOptions<'_>,
    executor: BackgroundExecutor,
) -> Vec<StringMatch>
where
    T: Borrow<StringMatchCandidate> + Sync,
{
    let StringMatchOptions {
        max_results,
        cancel_flag,
        case,
        length_penalty,
    } = options;
    if candidates.is_empty() {
        return Vec::new();
    }
    let max_results = max_results.resolve(candidates.len());

    let Some(query) = Query::build(query, case) else {
        return empty_query_results(candidates, max_results);
    };

    let num_cpus = executor.num_cpus().min(candidates.len());
    let base_size = candidates.len() / num_cpus;
    let remainder = candidates.len() % num_cpus;
    let segment_capacity = max_results.min(candidates.len().div_ceil(num_cpus));
    let mut segment_results = (0..num_cpus)
        .map(|_| Vec::with_capacity(segment_capacity))
        .collect::<Vec<_>>();

    let config = nucleo::Config::DEFAULT;
    let mut matchers = matcher::get_matchers(num_cpus, config);

    executor
        .scoped(|scope| {
            for (segment_idx, (results, matcher)) in segment_results
                .iter_mut()
                .zip(matchers.iter_mut())
                .enumerate()
            {
                let query = &query;
                scope.spawn(async move {
                    let segment_start = segment_idx * base_size + segment_idx.min(remainder);
                    let segment_end =
                        (segment_idx + 1) * base_size + (segment_idx + 1).min(remainder);

                    match_string_helper(
                        &candidates[segment_start..segment_end],
                        query,
                        matcher,
                        length_penalty,
                        results,
                        cancel_flag,
                    )
                    .ok();
                });
            }
        })
        .await;

    matcher::return_matchers(matchers);

    if is_cancelled(cancel_flag, atomic::Ordering::Acquire) {
        return Vec::new();
    }

    let mut results = segment_results.concat();
    gpui_util::truncate_to_bottom_n_sorted_by(&mut results, max_results, &|a, b| b.cmp(a));
    results
}

pub fn match_strings<T>(
    candidates: &[T],
    query: &str,
    options: StringMatchOptions<'_>,
) -> Vec<StringMatch>
where
    T: Borrow<StringMatchCandidate>,
{
    let StringMatchOptions {
        max_results,
        cancel_flag,
        case,
        length_penalty,
    } = options;
    if candidates.is_empty() {
        return Vec::new();
    }
    let max_results = max_results.resolve(candidates.len());

    let Some(query) = Query::build(query, case) else {
        return empty_query_results(candidates, max_results);
    };

    let config = nucleo::Config::DEFAULT;
    let mut matcher = matcher::get_matcher(config);
    let mut results = Vec::with_capacity(max_results);

    let outcome = match_string_helper(
        candidates,
        &query,
        &mut matcher,
        length_penalty,
        &mut results,
        cancel_flag,
    );

    matcher::return_matcher(matcher);
    if outcome.is_err() || is_cancelled(cancel_flag, atomic::Ordering::Acquire) {
        return Vec::new();
    }
    gpui_util::truncate_to_bottom_n_sorted_by(&mut results, max_results, &|a, b| b.cmp(a));
    results
}

fn empty_query_results<T: Borrow<StringMatchCandidate>>(
    candidates: &[T],
    max_results: usize,
) -> Vec<StringMatch> {
    candidates
        .iter()
        .take(max_results)
        .map(|candidate| {
            let borrowed = candidate.borrow();
            StringMatch {
                candidate_id: borrowed.id,
                score: 0.,
                positions: Vec::new(),
                string: borrowed.string.clone(),
            }
        })
        .collect()
}

fn match_string_helper<T>(
    candidates: &[T],
    query: &Query,
    matcher: &mut nucleo::Matcher,
    length_penalty: LengthPenalty,
    results: &mut Vec<StringMatch>,
    cancel_flag: Option<&AtomicBool>,
) -> Result<(), Cancelled>
where
    T: Borrow<StringMatchCandidate>,
{
    let mut buf = Vec::new();
    let mut matched_chars: Vec<u32> = Vec::new();
    let mut candidate_chars: Vec<char> = Vec::new();

    for candidate in candidates {
        buf.clear();
        matched_chars.clear();
        if is_cancelled(cancel_flag, atomic::Ordering::Relaxed) {
            return Err(Cancelled);
        }

        let borrowed = candidate.borrow();

        if !borrowed.char_bag.is_superset(query.char_bag) {
            continue;
        }

        let haystack: Utf32Str = Utf32Str::new(borrowed.string.as_ref(), &mut buf);

        let Some(score) = query.pattern.indices(haystack, matcher, &mut matched_chars) else {
            continue;
        };

        let case_mismatches = count_case_mismatches(
            query.query_chars.as_deref(),
            &matched_chars,
            borrowed.string.as_ref(),
            &mut candidate_chars,
        );

        matched_chars.sort_unstable();
        matched_chars.dedup();

        let positive = score as f64 * case_penalty(case_mismatches);
        let adjusted_score =
            positive - length_penalty_for(borrowed.string.as_ref(), length_penalty);
        let positions = positions_from_sorted(borrowed.string.as_ref(), &matched_chars);

        results.push(StringMatch {
            candidate_id: borrowed.id,
            score: adjusted_score,
            positions,
            string: borrowed.string.clone(),
        });
    }
    Ok(())
}

#[inline]
fn length_penalty_for(s: &str, length_penalty: LengthPenalty) -> f64 {
    if length_penalty.is_on() {
        s.len() as f64 * LENGTH_PENALTY
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::BackgroundExecutor;

    fn options() -> StringMatchOptions<'static> {
        StringMatchOptions {
            max_results: MaxResults::All,
            cancel_flag: None,
            case: Case::Ignore,
            length_penalty: LengthPenalty::Off,
        }
    }

    fn candidates(strings: &[&str]) -> Vec<StringMatchCandidate> {
        strings
            .iter()
            .enumerate()
            .map(|(id, s)| StringMatchCandidate::new(id, *s))
            .collect()
    }

    #[gpui::test]
    async fn test_basic_match(executor: BackgroundExecutor) {
        let cs = candidates(&["hello", "world", "help"]);
        let results = match_strings_async(&cs, "hel", options(), executor).await;
        let matched: Vec<&str> = results.iter().map(|m| m.string.as_ref()).collect();
        assert!(matched.contains(&"hello"));
        assert!(matched.contains(&"help"));
        assert!(!matched.contains(&"world"));
    }

    #[gpui::test]
    async fn test_multi_word_query(executor: BackgroundExecutor) {
        let cs = candidates(&[
            "src/lib/parser.rs",
            "src/bin/main.rs",
            "tests/parser_test.rs",
        ]);
        let results = match_strings_async(&cs, "src parser", options(), executor).await;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].string, "src/lib/parser.rs");
    }

    #[gpui::test]
    async fn test_empty_query_returns_all(executor: BackgroundExecutor) {
        let cs = candidates(&["alpha", "beta", "gamma"]);
        let results = match_strings_async(&cs, "", options(), executor).await;
        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|m| m.score == 0.0));
    }

    #[gpui::test]
    async fn test_whitespace_only_query_returns_all(executor: BackgroundExecutor) {
        let cs = candidates(&["alpha", "beta", "gamma"]);
        let results = match_strings_async(&cs, "   \t\n", options(), executor).await;
        assert_eq!(results.len(), 3);
    }

    #[gpui::test]
    async fn test_empty_candidates(executor: BackgroundExecutor) {
        let cs: Vec<StringMatchCandidate> = vec![];
        let results = match_strings_async(&cs, "query", options(), executor).await;
        assert!(results.is_empty());
    }

    #[gpui::test]
    async fn test_cancellation(executor: BackgroundExecutor) {
        let cs = candidates(&["hello", "world"]);
        let cancel = AtomicBool::new(true);
        let results = match_strings_async(
            &cs,
            "hel",
            StringMatchOptions {
                cancel_flag: Some(&cancel),
                ..options()
            },
            executor,
        )
        .await;
        assert!(results.is_empty());
    }

    #[gpui::test]
    async fn test_max_results_limit(executor: BackgroundExecutor) {
        let cs = candidates(&["ab", "abc", "abcd", "abcde"]);
        let results = match_strings_async(
            &cs,
            "ab",
            StringMatchOptions {
                max_results: MaxResults::limit::<2>(),
                ..options()
            },
            executor,
        )
        .await;
        assert_eq!(results.len(), 2);
    }

    #[gpui::test]
    async fn test_scoring_order(executor: BackgroundExecutor) {
        let cs = candidates(&[
            "some_very_long_variable_name_fuzzy",
            "fuzzy",
            "a_fuzzy_thing",
        ]);
        let results = match_strings_async(&cs, "fuzzy", options(), executor.clone()).await;

        let ordered = matches!(
            (
                results[0].string.as_ref(),
                results[1].string.as_ref(),
                results[2].string.as_ref()
            ),
            (
                "fuzzy",
                "a_fuzzy_thing",
                "some_very_long_variable_name_fuzzy"
            )
        );
        assert!(ordered, "matches are not in the proper order.");

        let results_penalty = match_strings_async(
            &cs,
            "fuzzy",
            StringMatchOptions {
                length_penalty: LengthPenalty::On,
                ..options()
            },
            executor,
        )
        .await;
        let greater = results[2].score > results_penalty[2].score;
        assert!(greater, "penalize length not affecting long candidates");
    }

    #[gpui::test]
    async fn test_utf8_positions(executor: BackgroundExecutor) {
        let cs = candidates(&["café"]);
        let results = match_strings_async(&cs, "caf", options(), executor).await;
        assert_eq!(results.len(), 1);
        let m = &results[0];
        assert_eq!(m.positions, vec![0, 1, 2]);
        for &pos in &m.positions {
            assert!(m.string.is_char_boundary(pos));
        }
    }

    #[gpui::test]
    async fn test_smart_case(executor: BackgroundExecutor) {
        let cs = candidates(&["FooBar", "foobar", "FOOBAR"]);

        let case_insensitive =
            match_strings_async(&cs, "foobar", options(), executor.clone()).await;
        assert_eq!(case_insensitive.len(), 3);

        let smart = match_strings_async(
            &cs,
            "FooBar",
            StringMatchOptions {
                case: Case::Smart,
                ..options()
            },
            executor,
        )
        .await;
        assert!(smart.iter().any(|m| m.string == "FooBar"));
        let foobar_score = smart.iter().find(|m| m.string == "FooBar").map(|m| m.score);
        let lower_score = smart.iter().find(|m| m.string == "foobar").map(|m| m.score);
        if let (Some(exact), Some(lower)) = (foobar_score, lower_score) {
            assert!(exact >= lower);
        }
    }

    #[gpui::test]
    async fn test_smart_case_does_not_flip_order_when_length_penalty_on(
        executor: BackgroundExecutor,
    ) {
        // Regression for the sign bug: with a length penalty large enough to push
        // `total_score - length_penalty` negative, case mismatches used to make
        // scores *better* (less negative). Exact-case match must still rank first.
        let cs = candidates(&[
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaa_FooBar",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaa_foobar",
        ]);
        let results = match_strings_async(
            &cs,
            "FooBar",
            StringMatchOptions {
                case: Case::Smart,
                length_penalty: LengthPenalty::On,
                ..options()
            },
            executor,
        )
        .await;
        let exact = results
            .iter()
            .find(|m| m.string.as_ref() == "aaaaaaaaaaaaaaaaaaaaaaaaaaaa_FooBar")
            .map(|m| m.score)
            .expect("exact-case candidate should match");
        let mismatch = results
            .iter()
            .find(|m| m.string.as_ref() == "aaaaaaaaaaaaaaaaaaaaaaaaaaaa_foobar")
            .map(|m| m.score)
            .expect("mismatch-case candidate should match");
        assert!(
            exact >= mismatch,
            "exact-case score ({exact}) should be >= mismatch-case score ({mismatch})"
        );
    }

    #[gpui::test]
    async fn test_char_bag_prefilter(executor: BackgroundExecutor) {
        let cs = candidates(&["abcdef", "abc", "def", "aabbcc"]);
        let results = match_strings_async(&cs, "abc", options(), executor).await;
        let matched: Vec<&str> = results.iter().map(|m| m.string.as_ref()).collect();
        assert!(matched.contains(&"abcdef"));
        assert!(matched.contains(&"abc"));
        assert!(matched.contains(&"aabbcc"));
        assert!(!matched.contains(&"def"));
    }

    #[test]
    fn test_sync_basic_match() {
        let cs = candidates(&["hello", "world", "help"]);
        let results = match_strings(&cs, "hel", options());
        let matched: Vec<&str> = results.iter().map(|m| m.string.as_ref()).collect();
        assert!(matched.contains(&"hello"));
        assert!(matched.contains(&"help"));
        assert!(!matched.contains(&"world"));
    }

    #[test]
    fn test_sync_empty_query_returns_all() {
        let cs = candidates(&["alpha", "beta", "gamma"]);
        let results = match_strings(&cs, "", options());
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_sync_whitespace_only_query_returns_all() {
        let cs = candidates(&["alpha", "beta", "gamma"]);
        let results = match_strings(&cs, "  ", options());
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_sync_max_results() {
        let cs = candidates(&["ab", "abc", "abcd", "abcde"]);
        let results = match_strings(
            &cs,
            "ab",
            StringMatchOptions {
                max_results: MaxResults::limit::<2>(),
                ..options()
            },
        );
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_sync_cancellation() {
        let cs = candidates(&["hello", "world"]);
        let cancel = AtomicBool::new(true);
        let results = match_strings(
            &cs,
            "hel",
            StringMatchOptions {
                cancel_flag: Some(&cancel),
                ..options()
            },
        );
        assert!(results.is_empty());
    }

    #[test]
    fn test_sync_cancellation_during_last_candidate() {
        struct CancellingCandidate<'a> {
            candidate: StringMatchCandidate,
            cancel: &'a AtomicBool,
        }

        impl Borrow<StringMatchCandidate> for CancellingCandidate<'_> {
            fn borrow(&self) -> &StringMatchCandidate {
                self.cancel.store(true, atomic::Ordering::Release);
                &self.candidate
            }
        }

        let cancel = AtomicBool::new(false);
        let cs = [CancellingCandidate {
            candidate: StringMatchCandidate::new(0, "hello"),
            cancel: &cancel,
        }];
        let results = match_strings(
            &cs,
            "hel",
            StringMatchOptions {
                cancel_flag: Some(&cancel),
                ..options()
            },
        );
        assert!(results.is_empty());
    }

    #[gpui::test]
    async fn test_empty_query_respects_max_results(executor: BackgroundExecutor) {
        let cs = candidates(&["alpha", "beta", "gamma", "delta"]);
        let results = match_strings_async(
            &cs,
            "",
            StringMatchOptions {
                max_results: MaxResults::limit::<2>(),
                ..options()
            },
            executor,
        )
        .await;
        assert_eq!(results.len(), 2);
    }

    #[gpui::test]
    async fn test_multi_word_with_nonmatching_word(executor: BackgroundExecutor) {
        let cs = candidates(&["src/parser.rs", "src/main.rs"]);
        let results = match_strings_async(&cs, "src xyzzy", options(), executor).await;
        assert!(
            results.is_empty(),
            "no candidate contains 'xyzzy', so nothing should match"
        );
    }

    #[gpui::test]
    async fn test_segment_size_not_divisible_by_cpus(executor: BackgroundExecutor) {
        executor.set_num_cpus(4);
        let cs = candidates(&["alpha", "beta", "gamma", "delta", "epsilon"]);
        let results = match_strings_async(&cs, "a", options(), executor).await;
        let matched: Vec<&str> = results.iter().map(|m| m.string.as_ref()).collect();
        assert!(matched.contains(&"alpha"));
        assert!(matched.contains(&"gamma"));
        assert!(matched.contains(&"delta"));
    }

    #[gpui::test]
    async fn test_segment_size_with_many_cpus_few_candidates(executor: BackgroundExecutor) {
        executor.set_num_cpus(16);
        let cs = candidates(&["one", "two", "three"]);
        let results = match_strings_async(&cs, "o", options(), executor).await;
        let matched: Vec<&str> = results.iter().map(|m| m.string.as_ref()).collect();
        assert!(matched.contains(&"one"));
        assert!(matched.contains(&"two"));
    }

    #[gpui::test]
    async fn test_segment_size_single_candidate(executor: BackgroundExecutor) {
        executor.set_num_cpus(8);
        let cs = candidates(&["lonely"]);
        let results = match_strings_async(&cs, "lone", options(), executor).await;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].string.as_ref(), "lonely");
    }

    #[gpui::test]
    async fn test_segment_size_candidates_equal_cpus(executor: BackgroundExecutor) {
        executor.set_num_cpus(4);
        let cs = candidates(&["aaa", "bbb", "ccc", "ddd"]);
        let results = match_strings_async(&cs, "a", options(), executor).await;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].string.as_ref(), "aaa");
    }

    #[gpui::test]
    async fn test_segment_size_candidates_one_more_than_cpus(executor: BackgroundExecutor) {
        executor.set_num_cpus(3);
        let cs = candidates(&["ant", "ape", "dog", "axe"]);
        let results = match_strings_async(&cs, "a", options(), executor).await;
        let matched: Vec<&str> = results.iter().map(|m| m.string.as_ref()).collect();
        assert!(matched.contains(&"ant"));
        assert!(matched.contains(&"ape"));
        assert!(matched.contains(&"axe"));
        assert!(!matched.contains(&"dog"));
    }
}
