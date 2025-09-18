use gpui::BackgroundExecutor;
use nucleo::pattern::{AtomKind, CaseMatching, Normalization, Pattern};
use std::{
    cmp::{self, Ordering},
    sync::{
        Arc,
        atomic::{self, AtomicBool},
    },
};
use util::{paths::PathStyle, rel_path::RelPath};

use crate::{CharBag, matcher};

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
    let mut matcher = matcher::get_matcher(nucleo::Config::DEFAULT);
    let pattern = Pattern::new(
        query,
        if smart_case {
            CaseMatching::Smart
        } else {
            CaseMatching::Ignore
        },
        Normalization::Smart,
        AtomKind::Fuzzy,
    );

    let mut results = Vec::new();
    for c in candidates {
        let mut indices = Vec::new();
        let mut buf = Vec::new();
        if let Some(score) = pattern.indices(
            nucleo::Utf32Str::new(&c.path.as_str(), &mut buf),
            &mut matcher,
            &mut indices,
        ) {
            results.push(PathMatch {
                score: score as f64,
                worktree_id,
                positions: indices.into_iter().map(|n| n as usize).collect(),
                is_dir: c.is_dir,
                path: c.path.into(),
                path_prefix: RelPath::empty().into(),
                distance_to_relative_ancestor: usize::MAX,
            })
        };
    }
    matcher::return_matcher(matcher);
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

    let query = if path_style.is_windows() {
        query.replace('\\', "/")
    } else {
        query.to_owned()
    };

    let pattern = Pattern::new(
        &query,
        if smart_case {
            CaseMatching::Smart
        } else {
            CaseMatching::Ignore
        },
        Normalization::Smart,
        AtomKind::Fuzzy,
    );

    let num_cpus = executor.num_cpus().min(path_count);
    let segment_size = path_count.div_ceil(num_cpus);
    let mut segment_results = (0..num_cpus)
        .map(|_| Vec::with_capacity(max_results))
        .collect::<Vec<_>>();

    let mut matchers = matcher::get_matchers(num_cpus, nucleo::Config::DEFAULT);

    // This runs num_cpu parallel searches. Each search is going through all candidate sets
    // Each parallel search goes through one segment of the every candidate set. The segments are
    // not overlapping.

    executor
        .scoped(|scope| {
            for (segment_idx, (results, matcher)) in segment_results
                .iter_mut()
                .zip(matchers.iter_mut())
                .enumerate()
            {
                let relative_to = relative_to.clone();
                let pattern = pattern.clone();
                scope.spawn(async move {
                    let segment_start = segment_idx * segment_size;
                    let segment_end = segment_start + segment_size;

                    let mut tree_start = 0;
                    'outer: for candidate_set in candidate_sets {
                        let tree_end = tree_start + candidate_set.len();

                        if (segment_start..segment_end).contains(&tree_start) {
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
                            for c in candidates {
                                if cancel_flag.load(std::sync::atomic::Ordering::Relaxed) {
                                    break 'outer;
                                }
                                let mut indices = Vec::new();
                                let mut buf = Vec::new();
                                if let Some(score) = pattern.indices(
                                    nucleo::Utf32Str::new(&c.path.as_str(), &mut buf),
                                    matcher,
                                    &mut indices,
                                ) {
                                    results.push(PathMatch {
                                        score: score as f64,
                                        worktree_id,
                                        positions: indices
                                            .into_iter()
                                            .map(|n| n as usize)
                                            .collect(),
                                        path: Arc::from(c.path),
                                        is_dir: c.is_dir,
                                        path_prefix: candidate_set.prefix(),
                                        distance_to_relative_ancestor: relative_to.as_ref().map_or(
                                            usize::MAX,
                                            |relative_to| {
                                                distance_between_paths(c.path, relative_to.as_ref())
                                            },
                                        ),
                                    })
                                };
                            }
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

    matcher::return_matchers(matchers);

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
