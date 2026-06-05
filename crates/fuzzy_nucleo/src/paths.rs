use gpui::BackgroundExecutor;
use std::{
    cmp::Ordering,
    sync::{
        Arc,
        atomic::{self, AtomicBool},
    },
};
use util::{paths::PathStyle, rel_path::RelPath};

use nucleo::Utf32Str;
use nucleo::pattern::Pattern;

use fuzzy::CharBag;

use crate::matcher::{self, LENGTH_PENALTY};
use crate::{Cancelled, Case, Query, case_penalty, count_case_mismatches, positions_from_sorted};

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
    pattern: &Pattern,
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
    let haystack = Utf32Str::new(filename, &mut buf);
    let score: u32 = pattern
        .atoms
        .iter()
        .filter_map(|atom| atom.score(haystack, matcher))
        .map(|s| s as u32)
        .sum();

    score as f64 / filename.len().max(1) as f64
}

fn path_match_helper<'a>(
    matcher: &mut nucleo::Matcher,
    query: &Query,
    candidates: impl Iterator<Item = PathMatchCandidate<'a>>,
    results: &mut Vec<PathMatch>,
    worktree_id: usize,
    path_prefix: &Arc<RelPath>,
    root_is_file: bool,
    relative_to: &Option<Arc<RelPath>>,
    path_style: PathStyle,
    cancel_flag: &AtomicBool,
) -> Result<(), Cancelled> {
    let mut candidate_buf = if !path_prefix.is_empty() && !root_is_file {
        let mut s = path_prefix.display(path_style).to_string();
        s.push_str(path_style.primary_separator());
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

        if !candidate.char_bag.is_superset(query.char_bag) {
            continue;
        }

        candidate_buf.truncate(path_prefix_len);
        if root_is_file {
            candidate_buf.push_str(path_prefix.as_unix_str());
        } else {
            candidate_buf.push_str(candidate.path.as_unix_str());
        }

        let haystack = Utf32Str::new(&candidate_buf, &mut buf);

        let Some(score) = query.pattern.indices(haystack, matcher, &mut matched_chars) else {
            continue;
        };

        let case_mismatches = count_case_mismatches(
            query.query_chars.as_deref(),
            &matched_chars,
            &candidate_buf,
            &mut candidate_chars,
        );

        matched_chars.sort_unstable();
        matched_chars.dedup();

        let length_penalty = candidate_buf.len() as f64 * LENGTH_PENALTY;
        let filename_bonus = get_filename_match_bonus(&candidate_buf, &query.pattern, matcher);
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
                RelPath::empty().into()
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
    let Some(query) = Query::build(query, case) else {
        return Vec::new();
    };

    let mut config = nucleo::Config::DEFAULT;
    config.set_match_paths();
    let mut matcher = matcher::get_matcher(config);

    let root_is_file = worktree_root_name.is_some() && candidates.iter().all(|c| c.path.is_empty());

    let path_prefix = worktree_root_name.unwrap_or_else(|| RelPath::empty().into());

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
        path_style,
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

    let Some(query) = Query::build(&query, case) else {
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
                                path_style,
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
