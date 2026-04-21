use std::{
    borrow::Borrow,
    cmp::Ordering,
    iter,
    ops::Range,
    sync::atomic::{self, AtomicBool},
};

use gpui::{BackgroundExecutor, SharedString};
use nucleo::Utf32Str;
use nucleo::pattern::{Atom, AtomKind, CaseMatching, Normalization};

use crate::{
    Cancelled, Case, LengthPenalty,
    matcher::{self, LENGTH_PENALTY},
    positions_from_sorted,
};
use fuzzy::CharBag;

// String matching is always case-insensitive at the nucleo level — using
// `CaseMatching::Smart` there would reject queries whose capitalization
// doesn't match the candidate, breaking pickers like the command palette
// (`"Editor: Backspace"` against the action named `"editor: backspace"`).
// `Case::Smart` is still honored as a *scoring hint*: when the query
// contains uppercase, candidates whose matched characters disagree in case
// are downranked rather than dropped.
const SMART_CASE_PENALTY_PER_MISMATCH: f64 = 0.9;

struct Query {
    atoms: Vec<Atom>,
    source_words: Option<Vec<Vec<char>>>,
    char_bag: CharBag,
}

impl Query {
    fn build(query: &str, case: Case) -> Option<Self> {
        let mut atoms = Vec::new();
        let mut source_words = Vec::new();
        let wants_case_penalty = case.is_smart() && query.chars().any(|c| c.is_uppercase());

        for word in query.split_whitespace() {
            atoms.push(Atom::new(
                word,
                CaseMatching::Ignore,
                Normalization::Smart,
                AtomKind::Fuzzy,
                false,
            ));
            if wants_case_penalty {
                source_words.push(word.chars().collect());
            }
        }

        if atoms.is_empty() {
            return None;
        }

        Some(Query {
            atoms,
            source_words: wants_case_penalty.then_some(source_words),
            char_bag: CharBag::from(query),
        })
    }
}

#[derive(Clone, Debug)]
pub struct StringMatchCandidate {
    pub id: usize,
    pub string: SharedString,
    char_bag: CharBag,
}

impl StringMatchCandidate {
    pub fn new(id: usize, string: impl ToString) -> Self {
        Self::from_shared(id, SharedString::new(string.to_string()))
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

pub async fn match_strings_async<T>(
    candidates: &[T],
    query: &str,
    case: Case,
    length_penalty: LengthPenalty,
    max_results: usize,
    cancel_flag: &AtomicBool,
    executor: BackgroundExecutor,
) -> Vec<StringMatch>
where
    T: Borrow<StringMatchCandidate> + Sync,
{
    if candidates.is_empty() || max_results == 0 {
        return Vec::new();
    }

    let Some(query) = Query::build(query, case) else {
        return empty_query_results(candidates, max_results);
    };

    let num_cpus = executor.num_cpus().min(candidates.len());
    let base_size = candidates.len() / num_cpus;
    let remainder = candidates.len() % num_cpus;
    let mut segment_results = (0..num_cpus)
        .map(|_| Vec::with_capacity(max_results.min(candidates.len())))
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

    if cancel_flag.load(atomic::Ordering::Acquire) {
        return Vec::new();
    }

    let mut results = segment_results.concat();
    util::truncate_to_bottom_n_sorted_by(&mut results, max_results, &|a, b| b.cmp(a));
    results
}

pub fn match_strings<T>(
    candidates: &[T],
    query: &str,
    case: Case,
    length_penalty: LengthPenalty,
    max_results: usize,
) -> Vec<StringMatch>
where
    T: Borrow<StringMatchCandidate>,
{
    if candidates.is_empty() || max_results == 0 {
        return Vec::new();
    }

    let Some(query) = Query::build(query, case) else {
        return empty_query_results(candidates, max_results);
    };

    let config = nucleo::Config::DEFAULT;
    let mut matcher = matcher::get_matcher(config);
    let mut results = Vec::with_capacity(max_results.min(candidates.len()));

    match_string_helper(
        candidates,
        &query,
        &mut matcher,
        length_penalty,
        &mut results,
        &AtomicBool::new(false),
    )
    .ok();

    matcher::return_matcher(matcher);
    util::truncate_to_bottom_n_sorted_by(&mut results, max_results, &|a, b| b.cmp(a));
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
    cancel_flag: &AtomicBool,
) -> Result<(), Cancelled>
where
    T: Borrow<StringMatchCandidate>,
{
    let mut buf = Vec::new();
    let mut matched_chars: Vec<u32> = Vec::new();
    let mut atom_matched_chars = Vec::new();
    let mut candidate_chars: Vec<char> = Vec::new();

    for candidate in candidates {
        buf.clear();
        matched_chars.clear();
        if cancel_flag.load(atomic::Ordering::Relaxed) {
            return Err(Cancelled);
        }

        let borrowed = candidate.borrow();

        if !borrowed.char_bag.is_superset(query.char_bag) {
            continue;
        }

        let haystack: Utf32Str = Utf32Str::new(&borrowed.string, &mut buf);

        if query.source_words.is_some() {
            candidate_chars.clear();
            candidate_chars.extend(borrowed.string.chars());
        }

        let mut total_score: u32 = 0;
        let mut case_mismatches: u32 = 0;
        let mut all_matched = true;

        for (atom_idx, atom) in query.atoms.iter().enumerate() {
            atom_matched_chars.clear();
            let Some(score) = atom.indices(haystack, matcher, &mut atom_matched_chars) else {
                all_matched = false;
                break;
            };
            total_score = total_score.saturating_add(score as u32);
            if let Some(source_words) = query.source_words.as_deref() {
                let query_chars = &source_words[atom_idx];
                if query_chars.len() == atom_matched_chars.len() {
                    for (&query_char, &pos) in query_chars.iter().zip(&atom_matched_chars) {
                        if let Some(&candidate_char) = candidate_chars.get(pos as usize)
                            && candidate_char != query_char
                            && candidate_char.eq_ignore_ascii_case(&query_char)
                        {
                            case_mismatches += 1;
                        }
                    }
                }
            }
            matched_chars.extend_from_slice(&atom_matched_chars);
        }

        if all_matched {
            matched_chars.sort_unstable();
            matched_chars.dedup();

            let positive = total_score as f64 * case_penalty(case_mismatches);
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
    }
    Ok(())
}

#[inline]
fn case_penalty(mismatches: u32) -> f64 {
    if mismatches == 0 {
        1.0
    } else {
        SMART_CASE_PENALTY_PER_MISMATCH.powi(mismatches as i32)
    }
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

    fn candidates(strings: &[&str]) -> Vec<StringMatchCandidate> {
        strings
            .iter()
            .enumerate()
            .map(|(id, s)| StringMatchCandidate::new(id, s))
            .collect()
    }

    #[gpui::test]
    async fn test_basic_match(executor: BackgroundExecutor) {
        let cs = candidates(&["hello", "world", "help"]);
        let cancel = AtomicBool::new(false);
        let results = match_strings_async(
            &cs,
            "hel",
            Case::Ignore,
            LengthPenalty::Off,
            10,
            &cancel,
            executor,
        )
        .await;
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
        let cancel = AtomicBool::new(false);
        let results = match_strings_async(
            &cs,
            "src parser",
            Case::Ignore,
            LengthPenalty::Off,
            10,
            &cancel,
            executor,
        )
        .await;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].string, "src/lib/parser.rs");
    }

    #[gpui::test]
    async fn test_empty_query_returns_all(executor: BackgroundExecutor) {
        let cs = candidates(&["alpha", "beta", "gamma"]);
        let cancel = AtomicBool::new(false);
        let results = match_strings_async(
            &cs,
            "",
            Case::Ignore,
            LengthPenalty::Off,
            10,
            &cancel,
            executor,
        )
        .await;
        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|m| m.score == 0.0));
    }

    #[gpui::test]
    async fn test_whitespace_only_query_returns_all(executor: BackgroundExecutor) {
        let cs = candidates(&["alpha", "beta", "gamma"]);
        let cancel = AtomicBool::new(false);
        let results = match_strings_async(
            &cs,
            "   \t\n",
            Case::Ignore,
            LengthPenalty::Off,
            10,
            &cancel,
            executor,
        )
        .await;
        assert_eq!(results.len(), 3);
    }

    #[gpui::test]
    async fn test_empty_candidates(executor: BackgroundExecutor) {
        let cs: Vec<StringMatchCandidate> = vec![];
        let cancel = AtomicBool::new(false);
        let results = match_strings_async(
            &cs,
            "query",
            Case::Ignore,
            LengthPenalty::Off,
            10,
            &cancel,
            executor,
        )
        .await;
        assert!(results.is_empty());
    }

    #[gpui::test]
    async fn test_cancellation(executor: BackgroundExecutor) {
        let cs = candidates(&["hello", "world"]);
        let cancel = AtomicBool::new(true);
        let results = match_strings_async(
            &cs,
            "hel",
            Case::Ignore,
            LengthPenalty::Off,
            10,
            &cancel,
            executor,
        )
        .await;
        assert!(results.is_empty());
    }

    #[gpui::test]
    async fn test_max_results_limit(executor: BackgroundExecutor) {
        let cs = candidates(&["ab", "abc", "abcd", "abcde"]);
        let cancel = AtomicBool::new(false);
        let results = match_strings_async(
            &cs,
            "ab",
            Case::Ignore,
            LengthPenalty::Off,
            2,
            &cancel,
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
        let cancel = AtomicBool::new(false);
        let results = match_strings_async(
            &cs,
            "fuzzy",
            Case::Ignore,
            LengthPenalty::Off,
            10,
            &cancel,
            executor.clone(),
        )
        .await;

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
            Case::Ignore,
            LengthPenalty::On,
            10,
            &cancel,
            executor,
        )
        .await;
        let greater = results[2].score > results_penalty[2].score;
        assert!(greater, "penalize length not affecting long candidates");
    }

    #[gpui::test]
    async fn test_utf8_positions(executor: BackgroundExecutor) {
        let cs = candidates(&["café"]);
        let cancel = AtomicBool::new(false);
        let results = match_strings_async(
            &cs,
            "caf",
            Case::Ignore,
            LengthPenalty::Off,
            10,
            &cancel,
            executor,
        )
        .await;
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
        let cancel = AtomicBool::new(false);

        let case_insensitive = match_strings_async(
            &cs,
            "foobar",
            Case::Ignore,
            LengthPenalty::Off,
            10,
            &cancel,
            executor.clone(),
        )
        .await;
        assert_eq!(case_insensitive.len(), 3);

        let smart = match_strings_async(
            &cs,
            "FooBar",
            Case::Smart,
            LengthPenalty::Off,
            10,
            &cancel,
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
        let cancel = AtomicBool::new(false);
        let results = match_strings_async(
            &cs,
            "FooBar",
            Case::Smart,
            LengthPenalty::On,
            10,
            &cancel,
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
        let cancel = AtomicBool::new(false);
        let results = match_strings_async(
            &cs,
            "abc",
            Case::Ignore,
            LengthPenalty::Off,
            10,
            &cancel,
            executor,
        )
        .await;
        let matched: Vec<&str> = results.iter().map(|m| m.string.as_ref()).collect();
        assert!(matched.contains(&"abcdef"));
        assert!(matched.contains(&"abc"));
        assert!(matched.contains(&"aabbcc"));
        assert!(!matched.contains(&"def"));
    }

    #[test]
    fn test_sync_basic_match() {
        let cs = candidates(&["hello", "world", "help"]);
        let results = match_strings(&cs, "hel", Case::Ignore, LengthPenalty::Off, 10);
        let matched: Vec<&str> = results.iter().map(|m| m.string.as_ref()).collect();
        assert!(matched.contains(&"hello"));
        assert!(matched.contains(&"help"));
        assert!(!matched.contains(&"world"));
    }

    #[test]
    fn test_sync_empty_query_returns_all() {
        let cs = candidates(&["alpha", "beta", "gamma"]);
        let results = match_strings(&cs, "", Case::Ignore, LengthPenalty::Off, 10);
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_sync_whitespace_only_query_returns_all() {
        let cs = candidates(&["alpha", "beta", "gamma"]);
        let results = match_strings(&cs, "  ", Case::Ignore, LengthPenalty::Off, 10);
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_sync_max_results() {
        let cs = candidates(&["ab", "abc", "abcd", "abcde"]);
        let results = match_strings(&cs, "ab", Case::Ignore, LengthPenalty::Off, 2);
        assert_eq!(results.len(), 2);
    }

    #[gpui::test]
    async fn test_empty_query_respects_max_results(executor: BackgroundExecutor) {
        let cs = candidates(&["alpha", "beta", "gamma", "delta"]);
        let cancel = AtomicBool::new(false);
        let results = match_strings_async(
            &cs,
            "",
            Case::Ignore,
            LengthPenalty::Off,
            2,
            &cancel,
            executor,
        )
        .await;
        assert_eq!(results.len(), 2);
    }

    #[gpui::test]
    async fn test_multi_word_with_nonmatching_word(executor: BackgroundExecutor) {
        let cs = candidates(&["src/parser.rs", "src/main.rs"]);
        let cancel = AtomicBool::new(false);
        let results = match_strings_async(
            &cs,
            "src xyzzy",
            Case::Ignore,
            LengthPenalty::Off,
            10,
            &cancel,
            executor,
        )
        .await;
        assert!(
            results.is_empty(),
            "no candidate contains 'xyzzy', so nothing should match"
        );
    }

    #[gpui::test]
    async fn test_segment_size_not_divisible_by_cpus(executor: BackgroundExecutor) {
        executor.set_num_cpus(4);
        let cs = candidates(&["alpha", "beta", "gamma", "delta", "epsilon"]);
        let cancel = AtomicBool::new(false);
        let results = match_strings_async(
            &cs,
            "a",
            Case::Ignore,
            LengthPenalty::Off,
            10,
            &cancel,
            executor,
        )
        .await;
        let matched: Vec<&str> = results.iter().map(|m| m.string.as_ref()).collect();
        assert!(matched.contains(&"alpha"));
        assert!(matched.contains(&"gamma"));
        assert!(matched.contains(&"delta"));
    }

    #[gpui::test]
    async fn test_segment_size_with_many_cpus_few_candidates(executor: BackgroundExecutor) {
        executor.set_num_cpus(16);
        let cs = candidates(&["one", "two", "three"]);
        let cancel = AtomicBool::new(false);
        let results = match_strings_async(
            &cs,
            "o",
            Case::Ignore,
            LengthPenalty::Off,
            10,
            &cancel,
            executor,
        )
        .await;
        let matched: Vec<&str> = results.iter().map(|m| m.string.as_ref()).collect();
        assert!(matched.contains(&"one"));
        assert!(matched.contains(&"two"));
    }

    #[gpui::test]
    async fn test_segment_size_single_candidate(executor: BackgroundExecutor) {
        executor.set_num_cpus(8);
        let cs = candidates(&["lonely"]);
        let cancel = AtomicBool::new(false);
        let results = match_strings_async(
            &cs,
            "lone",
            Case::Ignore,
            LengthPenalty::Off,
            10,
            &cancel,
            executor,
        )
        .await;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].string.as_ref(), "lonely");
    }

    #[gpui::test]
    async fn test_segment_size_candidates_equal_cpus(executor: BackgroundExecutor) {
        executor.set_num_cpus(4);
        let cs = candidates(&["aaa", "bbb", "ccc", "ddd"]);
        let cancel = AtomicBool::new(false);
        let results = match_strings_async(
            &cs,
            "a",
            Case::Ignore,
            LengthPenalty::Off,
            10,
            &cancel,
            executor,
        )
        .await;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].string.as_ref(), "aaa");
    }

    #[gpui::test]
    async fn test_segment_size_candidates_one_more_than_cpus(executor: BackgroundExecutor) {
        executor.set_num_cpus(3);
        let cs = candidates(&["ant", "ape", "dog", "axe"]);
        let cancel = AtomicBool::new(false);
        let results = match_strings_async(
            &cs,
            "a",
            Case::Ignore,
            LengthPenalty::Off,
            10,
            &cancel,
            executor,
        )
        .await;
        let matched: Vec<&str> = results.iter().map(|m| m.string.as_ref()).collect();
        assert!(matched.contains(&"ant"));
        assert!(matched.contains(&"ape"));
        assert!(matched.contains(&"axe"));
        assert!(!matched.contains(&"dog"));
    }
}
