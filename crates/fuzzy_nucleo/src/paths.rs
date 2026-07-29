use gpui::BackgroundExecutor;
use std::{
    borrow::Cow,
    cmp::Ordering,
    sync::{
        Arc,
        atomic::{self, AtomicBool},
    },
};
use util::{paths::PathStyle, rel_path::RelPath};

use unicode_segmentation::UnicodeSegmentation;

use fuzzy::CharBag;

use crate::matcher::{self, LENGTH_PENALTY};
use crate::{
    Cancelled, Case, Query, case_penalty, count_case_mismatches, normalize_nfc,
    pattern_grapheme_atoms, positions_from_sorted, utf32_str,
};

struct PathQuery {
    query: Query,
    canonical_atoms: Vec<Vec<Option<Vec<char>>>>,
    normalize_candidates: bool,
}

impl PathQuery {
    fn build(query: &str, case: Case) -> Option<Self> {
        let canonical_query = normalize_nfc(query);
        let query = Query::build(&canonical_query, case)?;
        let canonical_atoms = pattern_grapheme_atoms(&canonical_query)
            .into_iter()
            .map(|atom| {
                atom.into_iter()
                    .map(|grapheme| {
                        (!grapheme.is_ascii())
                            .then(|| grapheme.chars().map(nucleo::chars::to_lower_case).collect())
                    })
                    .collect()
            })
            .collect();
        Some(Self {
            normalize_candidates: !canonical_query.is_ascii(),
            query,
            canonical_atoms,
        })
    }

    fn matches_canonical_graphemes(&self, candidate: &str, matched_indices: &[u32]) -> bool {
        if self.canonical_atoms.iter().flatten().all(Option::is_none) {
            return true;
        }
        if self.canonical_atoms.iter().map(Vec::len).sum::<usize>() != matched_indices.len() {
            return false;
        }

        let candidate_graphemes = candidate.graphemes(true).collect::<Vec<_>>();
        let mut matched_start = 0;
        for atom in &self.canonical_atoms {
            let matched_end = matched_start + atom.len();
            if !matches_canonical_graphemes(
                atom,
                &candidate_graphemes,
                &matched_indices[matched_start..matched_end],
            ) {
                return false;
            }
            matched_start = matched_end;
        }
        true
    }
}

fn matches_canonical_graphemes(
    expected_graphemes: &[Option<Vec<char>>],
    candidate_graphemes: &[&str],
    matched_indices: &[u32],
) -> bool {
    expected_graphemes
        .iter()
        .zip(matched_indices)
        .all(|(expected, &matched_index)| {
            let Some(expected) = expected else {
                return true;
            };
            candidate_graphemes
                .get(matched_index as usize)
                .is_some_and(|actual| {
                    actual
                        .chars()
                        .map(nucleo::chars::to_lower_case)
                        .eq(expected.iter().copied())
                })
        })
}

#[derive(Clone, Debug)]
pub struct PathMatchCandidate<'a> {
    pub is_dir: bool,
    pub path: &'a RelPath,
    pub char_bag: CharBag,
}

impl<'a> PathMatchCandidate<'a> {
    /// Build a candidate whose prefilter bag covers both the worktree prefix and the path.
    /// Pass `None` when matching against paths that have no worktree prefix.
    pub fn new(path: &'a RelPath, is_dir: bool, path_prefix: Option<&RelPath>) -> Self {
        let mut char_bag = CharBag::default();
        if let Some(prefix) = path_prefix
            && !prefix.is_empty()
        {
            char_bag.extend(prefix.as_unix_str().chars().map(|c| c.to_ascii_lowercase()));
        }
        char_bag.extend(path.as_unix_str().chars().map(|c| c.to_ascii_lowercase()));
        Self {
            is_dir,
            path,
            char_bag,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PathMatch {
    pub score: f64,
    pub positions: Vec<usize>,
    pub worktree_id: usize,
    pub path: Arc<RelPath>,
    pub path_prefix: Arc<RelPath>,
    pub is_dir: bool,
    /// Number of steps removed from a shared parent with the relative path
    /// Used to order closer paths first in the search list
    pub distance_to_relative_ancestor: usize,
}

pub trait PathMatchCandidateSet<'a>: Send + Sync {
    type Candidates: Iterator<Item = PathMatchCandidate<'a>>;
    fn id(&self) -> usize;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn root_is_file(&self) -> bool;
    fn prefix(&self) -> Arc<RelPath>;
    fn candidates(&'a self, start: usize) -> Self::Candidates;
    fn path_style(&self) -> PathStyle;
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
            .total_cmp(&other.score)
            .then_with(|| self.worktree_id.cmp(&other.worktree_id))
            .then_with(|| {
                other
                    .distance_to_relative_ancestor
                    .cmp(&self.distance_to_relative_ancestor)
            })
            .then_with(|| self.path.cmp(&other.path))
    }
}

pub(crate) fn distance_between_paths(path: &RelPath, relative_to: &RelPath) -> usize {
    let mut path_components = path.components();
    let mut relative_components = relative_to.components();

    while path_components
        .next()
        .zip(relative_components.next())
        .map(|(path_component, relative_component)| path_component == relative_component)
        .unwrap_or_default()
    {}
    path_components.count() + relative_components.count() + 1
}

#[inline]
fn get_filename_match_bonus(
    candidate_buf: &str,
    query: &PathQuery,
    matcher: &mut nucleo::Matcher,
) -> f64 {
    let Some(filename) = std::path::Path::new(candidate_buf)
        .file_name()
        .and_then(|f| f.to_str())
        .filter(|f| !f.is_empty())
    else {
        return 0.0;
    };
    let mut buf = Vec::new();
    let haystack = utf32_str(filename, &mut buf);
    if !query.normalize_candidates {
        let score: u32 = query
            .query
            .pattern
            .atoms
            .iter()
            .filter_map(|atom| atom.score(haystack, matcher))
            .map(u32::from)
            .sum();
        return score as f64 / filename.len().max(1) as f64;
    }

    let candidate_graphemes = filename.graphemes(true).collect::<Vec<_>>();
    let mut matched_indices = Vec::new();
    let score: u32 = query
        .query
        .pattern
        .atoms
        .iter()
        .zip(&query.canonical_atoms)
        .filter_map(|(atom, canonical_graphemes)| {
            matched_indices.clear();
            let score = atom.indices(haystack, matcher, &mut matched_indices)?;
            matches_canonical_graphemes(canonical_graphemes, &candidate_graphemes, &matched_indices)
                .then_some(score)
        })
        .map(|s| s as u32)
        .sum();

    score as f64 / filename.len().max(1) as f64
}

fn path_match_helper<'a>(
    matcher: &mut nucleo::Matcher,
    path_query: &PathQuery,
    candidates: impl Iterator<Item = PathMatchCandidate<'a>>,
    results: &mut Vec<PathMatch>,
    worktree_id: usize,
    path_prefix: &Arc<RelPath>,
    root_is_file: bool,
    relative_to: &Option<Arc<RelPath>>,
    cancel_flag: &AtomicBool,
) -> Result<(), Cancelled> {
    let mut candidate_buf = if !path_prefix.is_empty() && !root_is_file {
        let mut s = path_prefix.as_unix_str().to_owned();
        s.push('/');
        s
    } else {
        String::new()
    };
    let path_prefix_len = candidate_buf.len();
    let mut buf = Vec::new();
    let mut matched_chars: Vec<u32> = Vec::new();
    let mut candidate_chars: Vec<char> = Vec::new();
    for candidate in candidates {
        buf.clear();
        matched_chars.clear();
        if cancel_flag.load(atomic::Ordering::Relaxed) {
            return Err(Cancelled);
        }

        if !candidate.char_bag.is_superset(path_query.query.char_bag) {
            continue;
        }

        candidate_buf.truncate(path_prefix_len);
        if root_is_file {
            candidate_buf.push_str(path_prefix.as_unix_str());
        } else {
            candidate_buf.push_str(candidate.path.as_unix_str());
        }

        let canonical_candidate = path_query
            .normalize_candidates
            .then(|| normalize_nfc(&candidate_buf));
        let match_candidate = canonical_candidate
            .as_deref()
            .unwrap_or(candidate_buf.as_str());
        let haystack = utf32_str(match_candidate, &mut buf);

        let Some(score) = path_query
            .query
            .pattern
            .indices(haystack, matcher, &mut matched_chars)
        else {
            continue;
        };

        if path_query.normalize_candidates
            && !path_query.matches_canonical_graphemes(match_candidate, &matched_chars)
        {
            continue;
        }

        let case_mismatches = count_case_mismatches(
            path_query.query.query_chars.as_deref(),
            &matched_chars,
            match_candidate,
            &mut candidate_chars,
        );

        matched_chars.sort_unstable();
        matched_chars.dedup();

        let length_penalty = match_candidate.len() as f64 * LENGTH_PENALTY;
        let filename_bonus = get_filename_match_bonus(match_candidate, path_query, matcher);
        let positive = (score as f64 + filename_bonus) * case_penalty(case_mismatches);
        let adjusted_score = positive - length_penalty;
        let positions = positions_from_sorted(&candidate_buf, &matched_chars);

        results.push(PathMatch {
            score: adjusted_score,
            positions,
            worktree_id,
            path: if root_is_file {
                Arc::clone(path_prefix)
            } else {
                candidate.path.into()
            },
            path_prefix: if root_is_file {
                RelPath::empty_arc()
            } else {
                Arc::clone(path_prefix)
            },
            is_dir: candidate.is_dir,
            distance_to_relative_ancestor: relative_to.as_ref().map_or(usize::MAX, |relative_to| {
                distance_between_paths(candidate.path, relative_to.as_ref())
            }),
        });
    }
    Ok(())
}

pub fn match_fixed_path_set(
    candidates: Vec<PathMatchCandidate>,
    worktree_id: usize,
    worktree_root_name: Option<Arc<RelPath>>,
    query: &str,
    case: Case,
    max_results: usize,
    path_style: PathStyle,
) -> Vec<PathMatch> {
    let query = if path_style.is_windows() {
        Cow::Owned(query.replace('\\', "/"))
    } else {
        Cow::Borrowed(query)
    };
    let Some(query) = PathQuery::build(&query, case) else {
        return Vec::new();
    };

    let mut config = nucleo::Config::DEFAULT;
    config.set_match_paths();
    let mut matcher = matcher::get_matcher(config);

    let root_is_file = worktree_root_name.is_some() && candidates.iter().all(|c| c.path.is_empty());

    let path_prefix = worktree_root_name.unwrap_or_else(|| RelPath::empty_arc());

    let mut results = Vec::new();

    path_match_helper(
        &mut matcher,
        &query,
        candidates.into_iter(),
        &mut results,
        worktree_id,
        &path_prefix,
        root_is_file,
        &None,
        &AtomicBool::new(false),
    )
    .ok();
    util::truncate_to_bottom_n_sorted_by(&mut results, max_results, &|a, b| b.cmp(a));
    matcher::return_matcher(matcher);
    results
}

pub async fn match_path_sets<'a, Set: PathMatchCandidateSet<'a>>(
    candidate_sets: &'a [Set],
    query: &str,
    relative_to: &Option<Arc<RelPath>>,
    case: Case,
    max_results: usize,
    cancel_flag: &AtomicBool,
    executor: BackgroundExecutor,
) -> Vec<PathMatch> {
    let path_count: usize = candidate_sets.iter().map(|s| s.len()).sum();
    if path_count == 0 {
        return Vec::new();
    }

    let path_style = candidate_sets[0].path_style();

    let query = if path_style.is_windows() {
        query.replace('\\', "/")
    } else {
        query.to_owned()
    };

    let Some(query) = PathQuery::build(&query, case) else {
        return Vec::new();
    };

    let num_cpus = executor.num_cpus().min(path_count);
    let segment_size = path_count.div_ceil(num_cpus);
    let mut segment_results = (0..num_cpus)
        .map(|_| Vec::with_capacity(max_results))
        .collect::<Vec<_>>();
    let mut config = nucleo::Config::DEFAULT;
    config.set_match_paths();
    let mut matchers = matcher::get_matchers(num_cpus, config);
    executor
        .scoped(|scope| {
            for (segment_idx, (results, matcher)) in segment_results
                .iter_mut()
                .zip(matchers.iter_mut())
                .enumerate()
            {
                let query = &query;
                let relative_to = relative_to.clone();
                scope.spawn(async move {
                    let segment_start = segment_idx * segment_size;
                    let segment_end = segment_start + segment_size;

                    let mut tree_start = 0;
                    for candidate_set in candidate_sets {
                        let tree_end = tree_start + candidate_set.len();

                        if tree_start < segment_end && segment_start < tree_end {
                            let start = tree_start.max(segment_start) - tree_start;
                            let end = tree_end.min(segment_end) - tree_start;
                            let candidates = candidate_set.candidates(start).take(end - start);

                            if path_match_helper(
                                matcher,
                                query,
                                candidates,
                                results,
                                candidate_set.id(),
                                &candidate_set.prefix(),
                                candidate_set.root_is_file(),
                                &relative_to,
                                cancel_flag,
                            )
                            .is_err()
                            {
                                break;
                            }
                        }

                        if tree_end >= segment_end {
                            break;
                        }
                        tree_start = tree_end;
                    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::BackgroundExecutor;
    use std::sync::atomic::AtomicUsize;
    use util::rel_path::RelPathBuf;

    fn match_path(path: &str, query: &str) -> Vec<PathMatch> {
        let path = RelPath::new_test(path);
        match_fixed_path_set(
            vec![PathMatchCandidate::new(path.as_ref(), false, None)],
            0,
            None,
            query,
            Case::Ignore,
            10,
            PathStyle::Unix,
        )
    }

    #[test]
    fn matches_nfc_query_against_nfd_path() {
        let path = RelPath::new_test("gro\u{0308}ssen.md");
        let matches = match_fixed_path_set(
            vec![PathMatchCandidate::new(path.as_ref(), false, None)],
            0,
            None,
            "grö",
            Case::Ignore,
            10,
            PathStyle::Unix,
        );

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].path.as_ref(), path.as_ref());
        assert_eq!(matches[0].positions, vec![0, 1, 2, 3]);
    }

    #[test]
    fn matches_nfd_query_against_nfc_path() {
        let matches = match_path("grössen.md", "gro\u{0308}");

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].positions, vec![0, 1, 2]);
    }

    #[test]
    fn matches_nfd_query_against_nfd_path() {
        let matches = match_path("gro\u{0308}ssen.md", "gro\u{0308}");

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].positions, vec![0, 1, 2, 3]);
    }

    #[test]
    fn matches_hangul_in_both_normalization_directions() {
        let decomposed = "\u{1112}\u{1161}\u{11ab}\u{1100}\u{1173}\u{11af}.md";
        let nfc_to_nfd = match_path(decomposed, "한글");
        let nfd_to_nfc = match_path(
            "한글.md",
            "\u{1112}\u{1161}\u{11ab}\u{1100}\u{1173}\u{11af}",
        );

        assert_eq!(nfc_to_nfd.len(), 1);
        assert_eq!(nfc_to_nfd[0].positions, vec![0, 3, 6, 9, 12, 15]);
        assert_eq!(nfd_to_nfc.len(), 1);
        assert_eq!(nfd_to_nfc[0].positions, vec![0, 3]);
    }

    #[test]
    fn matches_canonically_reordered_marks_without_precomposition() {
        let matches = match_path("q\u{315}\u{300}.md", "q\u{300}\u{315}");

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].positions, vec![0, 1, 3]);
    }

    #[test]
    fn rejects_non_equivalent_combining_marks() {
        assert!(match_path("q\u{323}.md", "q\u{307}").is_empty());
    }

    #[test]
    fn rejects_compatibility_only_equivalence() {
        assert!(match_path("1-file.md", "①").is_empty());
    }

    #[test]
    fn maps_matches_after_decomposed_graphemes_to_original_bytes() {
        let matches = match_path("a\u{308}bc.md", "c");

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].positions, vec![4]);
    }

    #[test]
    fn gives_canonically_equivalent_paths_equal_scores() {
        let nfc = match_path("grössen.md", "grö");
        let nfd = match_path("gro\u{308}ssen.md", "grö");

        assert_eq!(nfc.len(), 1);
        assert_eq!(nfd.len(), 1);
        assert_eq!(nfc[0].score, nfd[0].score);
    }

    #[test]
    fn retains_original_path_and_prefix() {
        let path = RelPath::new_test("src/gro\u{308}ssen.md");
        let prefix = Arc::<RelPath>::from(RelPath::new_test("wo\u{308}rktree").as_ref());
        let matches = match_fixed_path_set(
            vec![PathMatchCandidate::new(
                path.as_ref(),
                false,
                Some(prefix.as_ref()),
            )],
            7,
            Some(Arc::clone(&prefix)),
            "wörk grö",
            Case::Ignore,
            10,
            PathStyle::Unix,
        );

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].path.as_ref(), path.as_ref());
        assert_eq!(matches[0].path_prefix.as_ref(), prefix.as_ref());
    }

    #[test]
    fn matches_single_file_worktree_root() {
        let empty_path = RelPath::empty();
        let root = Arc::<RelPath>::from(RelPath::new_test("gro\u{308}ssen.md").as_ref());
        let matches = match_fixed_path_set(
            vec![PathMatchCandidate::new(
                empty_path,
                false,
                Some(root.as_ref()),
            )],
            0,
            Some(Arc::clone(&root)),
            "grö",
            Case::Ignore,
            10,
            PathStyle::Unix,
        );

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].path.as_ref(), root.as_ref());
        assert!(matches[0].path_prefix.is_empty());
        assert_eq!(matches[0].positions, vec![0, 1, 2, 3]);
    }

    #[test]
    fn supports_multi_atom_unicode_queries() {
        let matches = match_path("src/gro\u{308}ssen.md", "src grö");

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].positions, vec![0, 1, 2, 4, 5, 6, 7]);
    }

    #[test]
    fn retains_ascii_matching_positions_and_order() {
        let first = RelPath::new_test("src/bandana.rs");
        let second = RelPath::new_test("src/banana.rs");
        let matches = match_fixed_path_set(
            vec![
                PathMatchCandidate::new(first.as_ref(), false, None),
                PathMatchCandidate::new(second.as_ref(), false, None),
            ],
            0,
            None,
            "bna",
            Case::Ignore,
            10,
            PathStyle::Unix,
        );

        assert_eq!(matches[0].path.as_ref(), second.as_ref());
        assert_eq!(matches[0].positions, vec![4, 6, 7]);
        assert_eq!(matches[1].path.as_ref(), first.as_ref());
        assert_eq!(matches[1].positions, vec![4, 9, 10]);
    }

    #[test]
    fn preserves_empty_whitespace_and_max_result_behavior() {
        let path = RelPath::new_test("alpha.md");
        let candidate = || PathMatchCandidate::new(path.as_ref(), false, None);

        assert!(
            match_fixed_path_set(
                vec![candidate()],
                0,
                None,
                "",
                Case::Ignore,
                10,
                PathStyle::Unix,
            )
            .is_empty()
        );
        assert!(
            match_fixed_path_set(
                vec![candidate()],
                0,
                None,
                " \t ",
                Case::Ignore,
                10,
                PathStyle::Unix,
            )
            .is_empty()
        );
        assert!(
            match_fixed_path_set(
                vec![candidate()],
                0,
                None,
                "a",
                Case::Ignore,
                0,
                PathStyle::Unix,
            )
            .is_empty()
        );
    }

    #[test]
    fn matches_canonical_paths_with_unix_and_windows_separators() {
        let path = RelPath::new_test("gro\u{308}ssen.md");
        let prefix = Arc::<RelPath>::from(RelPath::new_test("ro\u{308}ot").as_ref());
        for (path_style, query) in [
            (PathStyle::Unix, "röot/grö"),
            (PathStyle::Windows, "röot\\grö"),
        ] {
            let matches = match_fixed_path_set(
                vec![PathMatchCandidate::new(
                    path.as_ref(),
                    false,
                    Some(prefix.as_ref()),
                )],
                0,
                Some(Arc::clone(&prefix)),
                query,
                Case::Ignore,
                10,
                path_style,
            );

            assert_eq!(matches.len(), 1, "path style: {path_style:?}");
            assert_eq!(matches[0].path.as_ref(), path.as_ref());
            assert_eq!(matches[0].path_prefix.as_ref(), prefix.as_ref());
        }
    }

    #[test]
    fn applies_smart_case_after_multi_scalar_grapheme() {
        let exact_path = RelPath::new_test("q\u{307}/Beta.md");
        let mismatch_path = RelPath::new_test("q\u{307}/beta.md");
        let matches = match_fixed_path_set(
            vec![
                PathMatchCandidate::new(exact_path.as_ref(), false, None),
                PathMatchCandidate::new(mismatch_path.as_ref(), false, None),
            ],
            0,
            None,
            "q\u{307} B",
            Case::Smart,
            10,
            PathStyle::Unix,
        );

        let exact_score = matches
            .iter()
            .find(|path_match| path_match.path.as_ref() == exact_path.as_ref())
            .expect("exact-case path should match")
            .score;
        let mismatch_score = matches
            .iter()
            .find(|path_match| path_match.path.as_ref() == mismatch_path.as_ref())
            .expect("mismatch-case path should match")
            .score;
        assert!(exact_score > mismatch_score);
    }

    #[test]
    fn matches_large_mixed_normalization_corpus() {
        let path_strings = (0..10_000)
            .map(|index| match index % 3 {
                0 => format!("ascii/{index}/plain.md"),
                1 => format!("nfc/{index}/grössen.md"),
                _ => format!("nfd/{index}/gro\u{308}ssen.md"),
            })
            .collect::<Vec<_>>();
        let paths = path_strings
            .iter()
            .map(|path| {
                RelPathBuf::try_from(path.as_str()).expect("generated path should be valid")
            })
            .collect::<Vec<_>>();
        let candidates = paths
            .iter()
            .map(|path| PathMatchCandidate::new(path.as_ref(), false, None))
            .collect();

        let matches = match_fixed_path_set(
            candidates,
            0,
            None,
            "grö",
            Case::Ignore,
            100,
            PathStyle::Unix,
        );

        assert_eq!(matches.len(), 100);
        assert!(matches.iter().all(|path_match| {
            let path = path_match.path.as_unix_str();
            path.contains("grössen") || path.contains("gro\u{308}ssen")
        }));
    }

    struct CancelOnYieldSet {
        paths: Vec<RelPathBuf>,
        cancel_flag: Arc<AtomicBool>,
        yielded: Arc<AtomicUsize>,
    }

    struct CancelOnYieldCandidates<'a> {
        paths: std::slice::Iter<'a, RelPathBuf>,
        cancel_flag: Arc<AtomicBool>,
        yielded: Arc<AtomicUsize>,
    }

    impl<'a> Iterator for CancelOnYieldCandidates<'a> {
        type Item = PathMatchCandidate<'a>;

        fn next(&mut self) -> Option<Self::Item> {
            let path = self.paths.next()?;
            self.yielded.fetch_add(1, atomic::Ordering::Relaxed);
            self.cancel_flag.store(true, atomic::Ordering::Relaxed);
            Some(PathMatchCandidate::new(path.as_ref(), false, None))
        }
    }

    impl<'a> PathMatchCandidateSet<'a> for CancelOnYieldSet {
        type Candidates = CancelOnYieldCandidates<'a>;

        fn id(&self) -> usize {
            0
        }

        fn len(&self) -> usize {
            self.paths.len()
        }

        fn root_is_file(&self) -> bool {
            false
        }

        fn prefix(&self) -> Arc<RelPath> {
            RelPath::empty_arc()
        }

        fn candidates(&'a self, start: usize) -> Self::Candidates {
            CancelOnYieldCandidates {
                paths: self.paths[start..].iter(),
                cancel_flag: Arc::clone(&self.cancel_flag),
                yielded: Arc::clone(&self.yielded),
            }
        }

        fn path_style(&self) -> PathStyle {
            PathStyle::Unix
        }
    }

    #[gpui::test]
    async fn checks_cancellation_before_processing_each_candidate(executor: BackgroundExecutor) {
        let cancel_flag = Arc::new(AtomicBool::new(false));
        let yielded = Arc::new(AtomicUsize::new(0));
        let set = CancelOnYieldSet {
            paths: vec![RelPathBuf::try_from("alpha.md").expect("valid test path")],
            cancel_flag: Arc::clone(&cancel_flag),
            yielded: Arc::clone(&yielded),
        };

        let matches = match_path_sets(
            &[set],
            "a",
            &None,
            Case::Ignore,
            10,
            cancel_flag.as_ref(),
            executor,
        )
        .await;

        assert!(matches.is_empty());
        assert_eq!(yielded.load(atomic::Ordering::Relaxed), 1);
    }
}
