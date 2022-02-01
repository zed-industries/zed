mod char_bag;

use gpui::executor;
use std::{
    borrow::Cow,
    cmp::{self, Ordering},
    path::Path,
    sync::atomic::{self, AtomicBool},
    sync::Arc,
};

pub use char_bag::CharBag;

const BASE_DISTANCE_PENALTY: f64 = 0.6;
const ADDITIONAL_DISTANCE_PENALTY: f64 = 0.05;
const MIN_DISTANCE_PENALTY: f64 = 0.2;

pub struct Matcher<'a> {
    query: &'a [char],
    lowercase_query: &'a [char],
    query_char_bag: CharBag,
    smart_case: bool,
    max_results: usize,
    min_score: f64,
    match_positions: Vec<usize>,
    last_positions: Vec<usize>,
    score_matrix: Vec<Option<f64>>,
    best_position_matrix: Vec<usize>,
}

trait Match: Ord {
    fn score(&self) -> f64;
    fn set_positions(&mut self, positions: Vec<usize>);
}

trait MatchCandidate {
    fn has_chars(&self, bag: CharBag) -> bool;
    fn to_string<'a>(&'a self) -> Cow<'a, str>;
}

#[derive(Clone, Debug)]
pub struct PathMatchCandidate<'a> {
    pub path: &'a Arc<Path>,
    pub char_bag: CharBag,
}

#[derive(Clone, Debug)]
pub struct PathMatch {
    pub score: f64,
    pub positions: Vec<usize>,
    pub worktree_id: usize,
    pub path: Arc<Path>,
    pub path_prefix: Arc<str>,
}

#[derive(Clone, Debug)]
pub struct StringMatchCandidate {
    pub id: usize,
    pub string: String,
    pub char_bag: CharBag,
}

pub trait PathMatchCandidateSet<'a>: Send + Sync {
    type Candidates: Iterator<Item = PathMatchCandidate<'a>>;
    fn id(&self) -> usize;
    fn len(&self) -> usize;
    fn prefix(&self) -> Arc<str>;
    fn candidates(&'a self, start: usize) -> Self::Candidates;
}

impl Match for PathMatch {
    fn score(&self) -> f64 {
        self.score
    }

    fn set_positions(&mut self, positions: Vec<usize>) {
        self.positions = positions;
    }
}

impl Match for StringMatch {
    fn score(&self) -> f64 {
        self.score
    }

    fn set_positions(&mut self, positions: Vec<usize>) {
        self.positions = positions;
    }
}

impl<'a> MatchCandidate for PathMatchCandidate<'a> {
    fn has_chars(&self, bag: CharBag) -> bool {
        self.char_bag.is_superset(bag)
    }

    fn to_string(&self) -> Cow<'a, str> {
        self.path.to_string_lossy()
    }
}

impl StringMatchCandidate {
    pub fn new(id: usize, string: String) -> Self {
        Self {
            id,
            char_bag: CharBag::from(string.as_str()),
            string,
        }
    }
}

impl<'a> MatchCandidate for &'a StringMatchCandidate {
    fn has_chars(&self, bag: CharBag) -> bool {
        self.char_bag.is_superset(bag)
    }

    fn to_string(&self) -> Cow<'a, str> {
        self.string.as_str().into()
    }
}

#[derive(Clone, Debug)]
pub struct StringMatch {
    pub candidate_id: usize,
    pub score: f64,
    pub positions: Vec<usize>,
    pub string: String,
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

impl PartialEq for PathMatch {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for PathMatch {}

impl PartialOrd for PathMatch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PathMatch {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score
            .partial_cmp(&other.score)
            .unwrap_or(Ordering::Equal)
            .then_with(|| self.worktree_id.cmp(&other.worktree_id))
            .then_with(|| Arc::as_ptr(&self.path).cmp(&Arc::as_ptr(&other.path)))
    }
}

pub async fn match_strings(
    candidates: &[StringMatchCandidate],
    query: &str,
    smart_case: bool,
    max_results: usize,
    cancel_flag: &AtomicBool,
    background: Arc<executor::Background>,
) -> Vec<StringMatch> {
    if candidates.is_empty() {
        return Default::default();
    }

    let lowercase_query = query.to_lowercase().chars().collect::<Vec<_>>();
    let query = query.chars().collect::<Vec<_>>();

    let lowercase_query = &lowercase_query;
    let query = &query;
    let query_char_bag = CharBag::from(&lowercase_query[..]);

    let num_cpus = background.num_cpus().min(candidates.len());
    let segment_size = (candidates.len() + num_cpus - 1) / num_cpus;
    let mut segment_results = (0..num_cpus)
        .map(|_| Vec::with_capacity(max_results))
        .collect::<Vec<_>>();

    background
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
                        max_results,
                    );
                    matcher.match_strings(
                        &candidates[segment_start..segment_end],
                        results,
                        cancel_flag,
                    );
                });
            }
        })
        .await;

    let mut results = Vec::new();
    for segment_result in segment_results {
        if results.is_empty() {
            results = segment_result;
        } else {
            util::extend_sorted(&mut results, segment_result, max_results, |a, b| b.cmp(&a));
        }
    }
    results
}

pub async fn match_paths<'a, Set: PathMatchCandidateSet<'a>>(
    candidate_sets: &'a [Set],
    query: &str,
    smart_case: bool,
    max_results: usize,
    cancel_flag: &AtomicBool,
    background: Arc<executor::Background>,
) -> Vec<PathMatch> {
    let path_count: usize = candidate_sets.iter().map(|s| s.len()).sum();
    if path_count == 0 {
        return Vec::new();
    }

    let lowercase_query = query.to_lowercase().chars().collect::<Vec<_>>();
    let query = query.chars().collect::<Vec<_>>();

    let lowercase_query = &lowercase_query;
    let query = &query;
    let query_char_bag = CharBag::from(&lowercase_query[..]);

    let num_cpus = background.num_cpus().min(path_count);
    let segment_size = (path_count + num_cpus - 1) / num_cpus;
    let mut segment_results = (0..num_cpus)
        .map(|_| Vec::with_capacity(max_results))
        .collect::<Vec<_>>();

    background
        .scoped(|scope| {
            for (segment_idx, results) in segment_results.iter_mut().enumerate() {
                scope.spawn(async move {
                    let segment_start = segment_idx * segment_size;
                    let segment_end = segment_start + segment_size;
                    let mut matcher = Matcher::new(
                        query,
                        lowercase_query,
                        query_char_bag,
                        smart_case,
                        max_results,
                    );

                    let mut tree_start = 0;
                    for candidate_set in candidate_sets {
                        let tree_end = tree_start + candidate_set.len();

                        if tree_start < segment_end && segment_start < tree_end {
                            let start = cmp::max(tree_start, segment_start) - tree_start;
                            let end = cmp::min(tree_end, segment_end) - tree_start;
                            let candidates = candidate_set.candidates(start).take(end - start);

                            matcher.match_paths(
                                candidate_set.id(),
                                candidate_set.prefix(),
                                candidates,
                                results,
                                &cancel_flag,
                            );
                        }
                        if tree_end >= segment_end {
                            break;
                        }
                        tree_start = tree_end;
                    }
                })
            }
        })
        .await;

    let mut results = Vec::new();
    for segment_result in segment_results {
        if results.is_empty() {
            results = segment_result;
        } else {
            util::extend_sorted(&mut results, segment_result, max_results, |a, b| b.cmp(&a));
        }
    }
    results
}

impl<'a> Matcher<'a> {
    pub fn new(
        query: &'a [char],
        lowercase_query: &'a [char],
        query_char_bag: CharBag,
        smart_case: bool,
        max_results: usize,
    ) -> Self {
        Self {
            query,
            lowercase_query,
            query_char_bag,
            min_score: 0.0,
            last_positions: vec![0; query.len()],
            match_positions: vec![0; query.len()],
            score_matrix: Vec::new(),
            best_position_matrix: Vec::new(),
            smart_case,
            max_results,
        }
    }

    pub fn match_strings(
        &mut self,
        candidates: &[StringMatchCandidate],
        results: &mut Vec<StringMatch>,
        cancel_flag: &AtomicBool,
    ) {
        self.match_internal(
            &[],
            &[],
            candidates.iter(),
            results,
            cancel_flag,
            |candidate, score| StringMatch {
                candidate_id: candidate.id,
                score,
                positions: Vec::new(),
                string: candidate.string.to_string(),
            },
        )
    }

    pub fn match_paths<'c: 'a>(
        &mut self,
        tree_id: usize,
        path_prefix: Arc<str>,
        path_entries: impl Iterator<Item = PathMatchCandidate<'c>>,
        results: &mut Vec<PathMatch>,
        cancel_flag: &AtomicBool,
    ) {
        let prefix = path_prefix.chars().collect::<Vec<_>>();
        let lowercase_prefix = prefix
            .iter()
            .map(|c| c.to_ascii_lowercase())
            .collect::<Vec<_>>();
        self.match_internal(
            &prefix,
            &lowercase_prefix,
            path_entries,
            results,
            cancel_flag,
            |candidate, score| PathMatch {
                score,
                worktree_id: tree_id,
                positions: Vec::new(),
                path: candidate.path.clone(),
                path_prefix: path_prefix.clone(),
            },
        )
    }

    fn match_internal<C: MatchCandidate, R, F>(
        &mut self,
        prefix: &[char],
        lowercase_prefix: &[char],
        candidates: impl Iterator<Item = C>,
        results: &mut Vec<R>,
        cancel_flag: &AtomicBool,
        build_match: F,
    ) where
        R: Match,
        F: Fn(&C, f64) -> R,
    {
        let mut candidate_chars = Vec::new();
        let mut lowercase_candidate_chars = Vec::new();

        for candidate in candidates {
            if !candidate.has_chars(self.query_char_bag) {
                continue;
            }

            if cancel_flag.load(atomic::Ordering::Relaxed) {
                break;
            }

            candidate_chars.clear();
            lowercase_candidate_chars.clear();
            for c in candidate.to_string().chars() {
                candidate_chars.push(c);
                lowercase_candidate_chars.push(c.to_ascii_lowercase());
            }

            if !self.find_last_positions(&lowercase_prefix, &lowercase_candidate_chars) {
                continue;
            }

            let matrix_len = self.query.len() * (prefix.len() + candidate_chars.len());
            self.score_matrix.clear();
            self.score_matrix.resize(matrix_len, None);
            self.best_position_matrix.clear();
            self.best_position_matrix.resize(matrix_len, 0);

            let score = self.score_match(
                &candidate_chars,
                &lowercase_candidate_chars,
                &prefix,
                &lowercase_prefix,
            );

            if score > 0.0 {
                let mut mat = build_match(&candidate, score);
                if let Err(i) = results.binary_search_by(|m| mat.cmp(&m)) {
                    if results.len() < self.max_results {
                        mat.set_positions(self.match_positions.clone());
                        results.insert(i, mat);
                    } else if i < results.len() {
                        results.pop();
                        mat.set_positions(self.match_positions.clone());
                        results.insert(i, mat);
                    }
                    if results.len() == self.max_results {
                        self.min_score = results.last().unwrap().score();
                    }
                }
            }
        }
    }

    fn find_last_positions(
        &mut self,
        lowercase_prefix: &[char],
        lowercase_candidate: &[char],
    ) -> bool {
        let mut lowercase_prefix = lowercase_prefix.iter();
        let mut lowercase_candidate = lowercase_candidate.iter();
        for (i, char) in self.lowercase_query.iter().enumerate().rev() {
            if let Some(j) = lowercase_candidate.rposition(|c| c == char) {
                self.last_positions[i] = j + lowercase_prefix.len();
            } else if let Some(j) = lowercase_prefix.rposition(|c| c == char) {
                self.last_positions[i] = j;
            } else {
                return false;
            }
        }
        true
    }

    fn score_match(
        &mut self,
        path: &[char],
        path_cased: &[char],
        prefix: &[char],
        lowercase_prefix: &[char],
    ) -> f64 {
        let score = self.recursive_score_match(
            path,
            path_cased,
            prefix,
            lowercase_prefix,
            0,
            0,
            self.query.len() as f64,
        ) * self.query.len() as f64;

        if score <= 0.0 {
            return 0.0;
        }

        let path_len = prefix.len() + path.len();
        let mut cur_start = 0;
        let mut byte_ix = 0;
        let mut char_ix = 0;
        for i in 0..self.query.len() {
            let match_char_ix = self.best_position_matrix[i * path_len + cur_start];
            while char_ix < match_char_ix {
                let ch = prefix
                    .get(char_ix)
                    .or_else(|| path.get(char_ix - prefix.len()))
                    .unwrap();
                byte_ix += ch.len_utf8();
                char_ix += 1;
            }
            cur_start = match_char_ix + 1;
            self.match_positions[i] = byte_ix;
        }

        score
    }

    fn recursive_score_match(
        &mut self,
        path: &[char],
        path_cased: &[char],
        prefix: &[char],
        lowercase_prefix: &[char],
        query_idx: usize,
        path_idx: usize,
        cur_score: f64,
    ) -> f64 {
        if query_idx == self.query.len() {
            return 1.0;
        }

        let path_len = prefix.len() + path.len();

        if let Some(memoized) = self.score_matrix[query_idx * path_len + path_idx] {
            return memoized;
        }

        let mut score = 0.0;
        let mut best_position = 0;

        let query_char = self.lowercase_query[query_idx];
        let limit = self.last_positions[query_idx];

        let mut last_slash = 0;
        for j in path_idx..=limit {
            let path_char = if j < prefix.len() {
                lowercase_prefix[j]
            } else {
                path_cased[j - prefix.len()]
            };
            let is_path_sep = path_char == '/' || path_char == '\\';

            if query_idx == 0 && is_path_sep {
                last_slash = j;
            }

            if query_char == path_char || (is_path_sep && query_char == '_' || query_char == '\\') {
                let curr = if j < prefix.len() {
                    prefix[j]
                } else {
                    path[j - prefix.len()]
                };

                let mut char_score = 1.0;
                if j > path_idx {
                    let last = if j - 1 < prefix.len() {
                        prefix[j - 1]
                    } else {
                        path[j - 1 - prefix.len()]
                    };

                    if last == '/' {
                        char_score = 0.9;
                    } else if last == '-' || last == '_' || last == ' ' || last.is_numeric() {
                        char_score = 0.8;
                    } else if last.is_lowercase() && curr.is_uppercase() {
                        char_score = 0.8;
                    } else if last == '.' {
                        char_score = 0.7;
                    } else if query_idx == 0 {
                        char_score = BASE_DISTANCE_PENALTY;
                    } else {
                        char_score = MIN_DISTANCE_PENALTY.max(
                            BASE_DISTANCE_PENALTY
                                - (j - path_idx - 1) as f64 * ADDITIONAL_DISTANCE_PENALTY,
                        );
                    }
                }

                // Apply a severe penalty if the case doesn't match.
                // This will make the exact matches have higher score than the case-insensitive and the
                // path insensitive matches.
                if (self.smart_case || curr == '/') && self.query[query_idx] != curr {
                    char_score *= 0.001;
                }

                let mut multiplier = char_score;

                // Scale the score based on how deep within the path we found the match.
                if query_idx == 0 {
                    multiplier /= ((prefix.len() + path.len()) - last_slash) as f64;
                }

                let mut next_score = 1.0;
                if self.min_score > 0.0 {
                    next_score = cur_score * multiplier;
                    // Scores only decrease. If we can't pass the previous best, bail
                    if next_score < self.min_score {
                        // Ensure that score is non-zero so we use it in the memo table.
                        if score == 0.0 {
                            score = 1e-18;
                        }
                        continue;
                    }
                }

                let new_score = self.recursive_score_match(
                    path,
                    path_cased,
                    prefix,
                    lowercase_prefix,
                    query_idx + 1,
                    j + 1,
                    next_score,
                ) * multiplier;

                if new_score > score {
                    score = new_score;
                    best_position = j;
                    // Optimization: can't score better than 1.
                    if new_score == 1.0 {
                        break;
                    }
                }
            }
        }

        if best_position != 0 {
            self.best_position_matrix[query_idx * path_len + path_idx] = best_position;
        }

        self.score_matrix[query_idx * path_len + path_idx] = Some(score);
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_get_last_positions() {
        let mut query: &[char] = &['d', 'c'];
        let mut matcher = Matcher::new(query, query, query.into(), false, 10);
        let result = matcher.find_last_positions(&['a', 'b', 'c'], &['b', 'd', 'e', 'f']);
        assert_eq!(result, false);

        query = &['c', 'd'];
        let mut matcher = Matcher::new(query, query, query.into(), false, 10);
        let result = matcher.find_last_positions(&['a', 'b', 'c'], &['b', 'd', 'e', 'f']);
        assert_eq!(result, true);
        assert_eq!(matcher.last_positions, vec![2, 4]);

        query = &['z', '/', 'z', 'f'];
        let mut matcher = Matcher::new(query, query, query.into(), false, 10);
        let result = matcher.find_last_positions(&['z', 'e', 'd', '/'], &['z', 'e', 'd', '/', 'f']);
        assert_eq!(result, true);
        assert_eq!(matcher.last_positions, vec![0, 3, 4, 8]);
    }

    #[test]
    fn test_match_path_entries() {
        let paths = vec![
            "",
            "a",
            "ab",
            "abC",
            "abcd",
            "alphabravocharlie",
            "AlphaBravoCharlie",
            "thisisatestdir",
            "/////ThisIsATestDir",
            "/this/is/a/test/dir",
            "/test/tiatd",
        ];

        assert_eq!(
            match_query("abc", false, &paths),
            vec![
                ("abC", vec![0, 1, 2]),
                ("abcd", vec![0, 1, 2]),
                ("AlphaBravoCharlie", vec![0, 5, 10]),
                ("alphabravocharlie", vec![4, 5, 10]),
            ]
        );
        assert_eq!(
            match_query("t/i/a/t/d", false, &paths),
            vec![("/this/is/a/test/dir", vec![1, 5, 6, 8, 9, 10, 11, 15, 16]),]
        );

        assert_eq!(
            match_query("tiatd", false, &paths),
            vec![
                ("/test/tiatd", vec![6, 7, 8, 9, 10]),
                ("/this/is/a/test/dir", vec![1, 6, 9, 11, 16]),
                ("/////ThisIsATestDir", vec![5, 9, 11, 12, 16]),
                ("thisisatestdir", vec![0, 2, 6, 7, 11]),
            ]
        );
    }

    #[test]
    fn test_match_multibyte_path_entries() {
        let paths = vec!["aαbβ/cγdδ", "αβγδ/bcde", "c1️⃣2️⃣3️⃣/d4️⃣5️⃣6️⃣/e7️⃣8️⃣9️⃣/f", "/d/🆒/h"];
        assert_eq!("1️⃣".len(), 7);
        assert_eq!(
            match_query("bcd", false, &paths),
            vec![
                ("αβγδ/bcde", vec![9, 10, 11]),
                ("aαbβ/cγdδ", vec![3, 7, 10]),
            ]
        );
        assert_eq!(
            match_query("cde", false, &paths),
            vec![
                ("αβγδ/bcde", vec![10, 11, 12]),
                ("c1️⃣2️⃣3️⃣/d4️⃣5️⃣6️⃣/e7️⃣8️⃣9️⃣/f", vec![0, 23, 46]),
            ]
        );
    }

    fn match_query<'a>(
        query: &str,
        smart_case: bool,
        paths: &Vec<&'a str>,
    ) -> Vec<(&'a str, Vec<usize>)> {
        let lowercase_query = query.to_lowercase().chars().collect::<Vec<_>>();
        let query = query.chars().collect::<Vec<_>>();
        let query_chars = CharBag::from(&lowercase_query[..]);

        let path_arcs = paths
            .iter()
            .map(|path| Arc::from(PathBuf::from(path)))
            .collect::<Vec<_>>();
        let mut path_entries = Vec::new();
        for (i, path) in paths.iter().enumerate() {
            let lowercase_path = path.to_lowercase().chars().collect::<Vec<_>>();
            let char_bag = CharBag::from(lowercase_path.as_slice());
            path_entries.push(PathMatchCandidate {
                char_bag,
                path: path_arcs.get(i).unwrap(),
            });
        }

        let mut matcher = Matcher::new(&query, &lowercase_query, query_chars, smart_case, 100);

        let cancel_flag = AtomicBool::new(false);
        let mut results = Vec::new();
        matcher.match_paths(
            0,
            "".into(),
            path_entries.into_iter(),
            &mut results,
            &cancel_flag,
        );

        results
            .into_iter()
            .map(|result| {
                (
                    paths
                        .iter()
                        .copied()
                        .find(|p| result.path.as_ref() == Path::new(p))
                        .unwrap(),
                    result.positions,
                )
            })
            .collect()
    }
}
