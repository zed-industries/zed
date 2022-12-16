use std::{
    borrow::Cow,
    cmp::{self, Ordering},
    path::Path,
    sync::{atomic::AtomicBool, Arc},
};

use gpui::executor;

use crate::{
    matcher::{Match, MatchCandidate, Matcher},
    CharBag,
};

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

pub trait PathMatchCandidateSet<'a>: Send + Sync {
    type Candidates: Iterator<Item = PathMatchCandidate<'a>>;
    fn id(&self) -> usize;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
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

impl<'a> MatchCandidate for PathMatchCandidate<'a> {
    fn has_chars(&self, bag: CharBag) -> bool {
        self.char_bag.is_superset(bag)
    }

    fn to_string(&self) -> Cow<'a, str> {
        self.path.to_string_lossy()
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
            .then_with(|| self.path.cmp(&other.path))
    }
}

pub async fn match_path_sets<'a, Set: PathMatchCandidateSet<'a>>(
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

                            let worktree_id = candidate_set.id();
                            let prefix = candidate_set.prefix().chars().collect::<Vec<_>>();
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
                                |candidate, score| PathMatch {
                                    score,
                                    worktree_id,
                                    positions: Vec::new(),
                                    path: candidate.path.clone(),
                                    path_prefix: candidate_set.prefix(),
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

    let mut results = Vec::new();
    for segment_result in segment_results {
        if results.is_empty() {
            results = segment_result;
        } else {
            util::extend_sorted(&mut results, segment_result, max_results, |a, b| b.cmp(a));
        }
    }
    results
}
