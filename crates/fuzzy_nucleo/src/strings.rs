use std::{
    borrow::Borrow,
    cmp::Ordering,
    iter,
    ops::Range,
    sync::atomic::{self, AtomicBool},
};

use gpui::BackgroundExecutor;
use nucleo::Utf32Str;

use crate::paths::make_atoms;
use crate::{
    Cancelled,
    matcher::{self, LENGTH_PENALTY},
};
use fuzzy::CharBag;

#[derive(Clone, Debug)]
pub struct StringMatchCandidate {
    pub id: usize,
    pub string: String,
    char_bag: CharBag,
}

impl StringMatchCandidate {
    pub fn new(id: usize, string: &str) -> Self {
        Self {
            id,
            char_bag: CharBag::from(string),
            string: string.into(),
        }
    }

    pub fn from_string(id: usize, string: String) -> Self {
        Self {
            id,
            char_bag: CharBag::from(string.as_str()),
            string,
        }
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
        return Vec::new();
    }

    if query.is_empty() {
        return empty_query_results(candidates, max_results);
    }

    let atoms = make_atoms(query, smart_case);
    let query_bag = CharBag::from(query);

    let num_cpus = executor.num_cpus().min(candidates.len());
    let segment_size = candidates.len().div_ceil(num_cpus);
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
                let atoms = &atoms;
                scope.spawn(async move {
                    let segment_start = segment_idx * segment_size;
                    let segment_end = (segment_start + segment_size).min(candidates.len());

                    match_string_candidates(
                        &candidates[segment_start..segment_end],
                        query_bag,
                        atoms,
                        matcher,
                        penalize_length,
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
    smart_case: bool,
    penalize_length: bool,
    max_results: usize,
) -> Vec<StringMatch>
where
    T: Borrow<StringMatchCandidate>,
{
    if candidates.is_empty() || max_results == 0 {
        return Vec::new();
    }

    if query.is_empty() {
        return empty_query_results(candidates, max_results);
    }

    let atoms = make_atoms(query, smart_case);
    let query_bag = CharBag::from(query);
    let config = nucleo::Config::DEFAULT;
    let mut matcher = matcher::get_matcher(config);
    let mut results = Vec::with_capacity(max_results.min(candidates.len()));

    match_string_candidates(
        candidates,
        query_bag,
        &atoms,
        &mut matcher,
        penalize_length,
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
        .map(|candidate| StringMatch {
            candidate_id: candidate.borrow().id,
            score: 0.,
            positions: Default::default(),
            string: candidate.borrow().string.clone(),
        })
        .collect()
}

fn match_string_candidates<T>(
    candidates: &[T],
    query_bag: CharBag,
    atoms: &[nucleo::pattern::Atom],
    matcher: &mut nucleo::Matcher,
    penalize_length: bool,
    results: &mut Vec<StringMatch>,
    cancel_flag: &AtomicBool,
) -> Result<(), Cancelled>
where
    T: Borrow<StringMatchCandidate>,
{
    let mut buf = Vec::new();
    let mut matched_chars: Vec<u32> = Vec::new();
    let mut atom_matched_chars = Vec::new();

    for candidate in candidates {
        buf.clear();
        matched_chars.clear();
        if cancel_flag.load(atomic::Ordering::Relaxed) {
            return Err(Cancelled);
        }

        let borrowed = candidate.borrow();

        if !borrowed.char_bag.is_superset(query_bag) {
            continue;
        }

        let haystack: Utf32Str = Utf32Str::new(&borrowed.string, &mut buf);

        let mut total_score: u32 = 0;
        let mut all_matched = true;

        for atom in atoms {
            atom_matched_chars.clear();
            if let Some(score) = atom.indices(haystack, matcher, &mut atom_matched_chars) {
                total_score = total_score.saturating_add(score as u32);
                matched_chars.extend_from_slice(&atom_matched_chars);
            } else {
                all_matched = false;
                break;
            }
        }

        if all_matched && !atoms.is_empty() {
            matched_chars.sort_unstable();
            matched_chars.dedup();

            let length_penalty = if penalize_length {
                borrowed.string.len() as f64 * LENGTH_PENALTY
            } else {
                0.0
            };
            let adjusted_score = total_score as f64 - length_penalty;
            let positions: Vec<usize> = borrowed
                .string
                .char_indices()
                .enumerate()
                .filter_map(|(char_offset, (byte_offset, _))| {
                    matched_chars
                        .binary_search(&(char_offset as u32))
                        .is_ok()
                        .then_some(byte_offset)
                })
                .collect();

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
        let results = match_strings_async(&cs, "hel", false, false, 10, &cancel, executor).await;
        let matched: Vec<&str> = results.iter().map(|m| m.string.as_str()).collect();
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
        let results = match_strings_async(&cs, "src parser", false, false, 10, &cancel, executor).await;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].string, "src/lib/parser.rs");
    }

    #[gpui::test]
    async fn test_empty_query_returns_all(executor: BackgroundExecutor) {
        let cs = candidates(&["alpha", "beta", "gamma"]);
        let cancel = AtomicBool::new(false);
        let results = match_strings_async(&cs, "", false, false, 10, &cancel, executor).await;
        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|m| m.score == 0.0));
    }

    #[gpui::test]
    async fn test_empty_candidates(executor: BackgroundExecutor) {
        let cs: Vec<StringMatchCandidate> = vec![];
        let cancel = AtomicBool::new(false);
        let results = match_strings_async(&cs, "query", false, false, 10, &cancel, executor).await;
        assert!(results.is_empty());
    }

    #[gpui::test]
    async fn test_cancellation(executor: BackgroundExecutor) {
        let cs = candidates(&["hello", "world"]);
        let cancel = AtomicBool::new(true);
        let results = match_strings_async(&cs, "hel", false, false, 10, &cancel, executor).await;
        assert!(results.is_empty());
    }

    #[gpui::test]
    async fn test_max_results_limit(executor: BackgroundExecutor) {
        let cs = candidates(&["ab", "abc", "abcd", "abcde"]);
        let cancel = AtomicBool::new(false);
        let results = match_strings_async(&cs, "ab", false, false, 2, &cancel, executor).await;
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
        let results =
            match_strings_async(&cs, "fuzzy", false, false, 10, &cancel, executor.clone()).await;

        // Exact/shorter matches should score higher than substrings buried in long names.
        let ordered = matches!(
            (
                results[0].string.as_str(),
                results[1].string.as_str(),
                results[2].string.as_str()
            ),
            (
                "fuzzy",
                "a_fuzzy_thing",
                "some_very_long_variable_name_fuzzy"
            )
        );
        assert!(ordered, "matches are not in the proper order.");

        // penalize length should widen the gap between results.
        let results_penalty = match_strings_async(&cs, "fuzzy", false, true, 10, &cancel, executor).await;
        let greater = results[2].score > results_penalty[2].score;

        assert!(
            greater,
            "penalize length not resulting in long candidates having worse scores."
        )
    }

    #[gpui::test]
    async fn test_utf8_positions(executor: BackgroundExecutor) {
        let cs = candidates(&["café"]);
        let cancel = AtomicBool::new(false);
        let results = match_strings_async(&cs, "caf", false, false, 10, &cancel, executor).await;
        assert_eq!(results.len(), 1);
        let m = &results[0];
        assert_eq!(m.positions, vec![0, 1, 2]);

        for &pos in &m.positions {
            assert!(
                m.string.is_char_boundary(pos),
                "position {pos} is not a char boundary in {:?}",
                m.string,
            );
        }
    }

    #[gpui::test]
    async fn test_smart_case(executor: BackgroundExecutor) {
        let cs = candidates(&["FooBar", "foobar", "FOOBAR"]);
        let cancel = AtomicBool::new(false);

        let case_insensitive =
            match_strings_async(&cs, "foobar", false, false, 10, &cancel, executor.clone()).await;
        assert_eq!(case_insensitive.len(), 3);

        let smart = match_strings_async(&cs, "FooBar", true, false, 10, &cancel, executor).await;
        assert!(smart.iter().any(|m| m.string == "FooBar"));
        let foobar_score = smart.iter().find(|m| m.string == "FooBar").map(|m| m.score);
        let lower_score = smart.iter().find(|m| m.string == "foobar").map(|m| m.score);
        if let (Some(exact), Some(lower)) = (foobar_score, lower_score) {
            assert!(
                exact >= lower,
                "exact case match should score >= case-insensitive"
            );
        }
    }

    #[gpui::test]
    async fn test_char_bag_prefilter(executor: BackgroundExecutor) {
        let cs = candidates(&["abcdef", "abc", "def", "aabbcc"]);
        let cancel = AtomicBool::new(false);
        let results = match_strings_async(&cs, "abc", false, false, 10, &cancel, executor).await;
        let matched: Vec<&str> = results.iter().map(|m| m.string.as_str()).collect();
        assert!(matched.contains(&"abcdef"));
        assert!(matched.contains(&"abc"));
        assert!(matched.contains(&"aabbcc"));
        assert!(!matched.contains(&"def"));
    }

    #[test]
    fn test_sync_basic_match() {
        let cs = candidates(&["hello", "world", "help"]);
        let results = match_strings(&cs, "hel", false, false, 10);
        let matched: Vec<&str> = results.iter().map(|m| m.string.as_str()).collect();
        assert!(matched.contains(&"hello"));
        assert!(matched.contains(&"help"));
        assert!(!matched.contains(&"world"));
    }

    #[test]
    fn test_sync_empty_query_returns_all() {
        let cs = candidates(&["alpha", "beta", "gamma"]);
        let results = match_strings(&cs, "", false, false, 10);
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_sync_max_results() {
        let cs = candidates(&["ab", "abc", "abcd", "abcde"]);
        let results = match_strings(&cs, "ab", false, false, 2);
        assert_eq!(results.len(), 2);
    }

    #[gpui::test]
    async fn test_empty_query_respects_max_results(executor: BackgroundExecutor) {
        let cs = candidates(&["alpha", "beta", "gamma", "delta"]);
        let cancel = AtomicBool::new(false);
        let results = match_strings_async(&cs, "", false, false, 2, &cancel, executor).await;
        assert_eq!(results.len(), 2);
    }
}
