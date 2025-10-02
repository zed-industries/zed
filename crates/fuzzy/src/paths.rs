use gpui::BackgroundExecutor;
use std::{
    cmp::{self, Ordering},
    sync::{
        Arc,
        atomic::{self, AtomicBool},
    },
};
use util::{paths::PathStyle, rel_path::RelPath};

use crate::{
    CharBag,
    matcher::{MatchCandidate, Matcher},
};

#[derive(Clone, Debug)]
pub struct PathMatchCandidate<'a> {
    pub is_dir: bool,
    pub path: &'a RelPath,
    pub char_bag: CharBag,
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

impl<'a> MatchCandidate for PathMatchCandidate<'a> {
    fn has_chars(&self, bag: CharBag) -> bool {
        self.char_bag.is_superset(bag)
    }

    fn candidate_chars(&self) -> impl Iterator<Item = char> {
        self.path.as_unix_str().chars()
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
            .then_with(|| {
                other
                    .distance_to_relative_ancestor
                    .cmp(&self.distance_to_relative_ancestor)
            })
            .then_with(|| self.path.cmp(&other.path))
    }
}

pub fn match_fixed_path_set(
    candidates: Vec<PathMatchCandidate>,
    worktree_id: usize,
    query: &str,
    smart_case: bool,
    max_results: usize,
) -> Vec<PathMatch> {
    let lowercase_query = query.to_lowercase().chars().collect::<Vec<_>>();
    let query = query.chars().collect::<Vec<_>>();
    let query_char_bag = CharBag::from(&lowercase_query[..]);

    let mut matcher = Matcher::new(&query, &lowercase_query, query_char_bag, smart_case, true);

    let mut results = Vec::new();
    matcher.match_candidates(
        &[],
        &[],
        candidates.into_iter(),
        &mut results,
        &AtomicBool::new(false),
        |candidate, score, positions| PathMatch {
            score,
            worktree_id,
            positions: positions.clone(),
            is_dir: candidate.is_dir,
            path: candidate.path.into(),
            path_prefix: RelPath::empty().into(),
            distance_to_relative_ancestor: usize::MAX,
        },
    );
    util::truncate_to_bottom_n_sorted_by(&mut results, max_results, &|a, b| b.cmp(a));
    results
}

pub async fn match_path_sets<'a, Set: PathMatchCandidateSet<'a>>(
    candidate_sets: &'a [Set],
    query: &str,
    relative_to: &Option<Arc<RelPath>>,
    smart_case: bool,
    max_results: usize,
    cancel_flag: &AtomicBool,
    executor: BackgroundExecutor,
) -> Vec<PathMatch> {
    let path_count: usize = candidate_sets.iter().map(|s| s.len()).sum();
    if path_count == 0 {
        return Vec::new();
    }

    let path_style = candidate_sets[0].path_style();

    let query = query
        .chars()
        .map(|char| {
            if path_style.is_windows() && char == '\\' {
                '/'
            } else {
                char
            }
        })
        .collect::<Vec<_>>();

    let lowercase_query = query
        .iter()
        .map(|query| query.to_ascii_lowercase())
        .collect::<Vec<_>>();

    let query = &query;
    let lowercase_query = &lowercase_query;
    let query_char_bag = CharBag::from_iter(lowercase_query.iter().copied());

    let num_cpus = executor.num_cpus().min(path_count);
    let segment_size = path_count.div_ceil(num_cpus);
    let mut segment_results = (0..num_cpus)
        .map(|_| Vec::with_capacity(max_results))
        .collect::<Vec<_>>();

    executor
        .scoped(|scope| {
            for (segment_idx, results) in segment_results.iter_mut().enumerate() {
                scope.spawn(async move {
                    let segment_start = segment_idx * segment_size;
                    let segment_end = segment_start + segment_size;
                    let mut matcher =
                        Matcher::new(query, lowercase_query, query_char_bag, smart_case, true);

                    let mut tree_start = 0;
                    for candidate_set in candidate_sets {
                        if cancel_flag.load(atomic::Ordering::Acquire) {
                            break;
                        }

                        let tree_end = tree_start + candidate_set.len();

                        if tree_start < segment_end && segment_start < tree_end {
                            let start = cmp::max(tree_start, segment_start) - tree_start;
                            let end = cmp::min(tree_end, segment_end) - tree_start;
                            let candidates = candidate_set.candidates(start).take(end - start);

                            let worktree_id = candidate_set.id();
                            let mut prefix = candidate_set
                                .prefix()
                                .as_unix_str()
                                .chars()
                                .collect::<Vec<_>>();
                            if !candidate_set.root_is_file() && !prefix.is_empty() {
                                prefix.push('/');
                            }
                            let lowercase_prefix = prefix
                                .iter()
                                .map(|c| c.to_ascii_lowercase())
                                .collect::<Vec<_>>();
                            matcher.match_candidates(
                                &prefix,
                                &lowercase_prefix,
                                candidates,
                                results,
                                cancel_flag,
                                |candidate, score, positions| PathMatch {
                                    score,
                                    worktree_id,
                                    positions: positions.clone(),
                                    path: Arc::from(candidate.path),
                                    is_dir: candidate.is_dir,
                                    path_prefix: candidate_set.prefix(),
                                    distance_to_relative_ancestor: relative_to.as_ref().map_or(
                                        usize::MAX,
                                        |relative_to| {
                                            distance_between_paths(
                                                candidate.path,
                                                relative_to.as_ref(),
                                            )
                                        },
                                    ),
                                },
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

    if cancel_flag.load(atomic::Ordering::Acquire) {
        return Vec::new();
    }

    let mut results = segment_results.concat();
    util::truncate_to_bottom_n_sorted_by(&mut results, max_results, &|a, b| b.cmp(a));
    results
}

/// Compute the distance from a given path to some other path
/// If there is no shared path, returns usize::MAX
fn distance_between_paths(path: &RelPath, relative_to: &RelPath) -> usize {
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

#[cfg(test)]
mod tests {
    use util::rel_path::RelPath;

    use super::distance_between_paths;

    #[test]
    fn test_distance_between_paths_empty() {
        distance_between_paths(RelPath::empty(), RelPath::empty());
    }
}
