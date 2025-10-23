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
    /// Guarenteed to be sorted
    pub positions: Vec<usize>,
    pub worktree_id: usize,
    pub path: Arc<RelPath>,
    pub path_prefix: Arc<RelPath>,
    pub is_dir: bool,
    /// Number of steps removed from a shared parent with the relative path
    /// Used to order closer paths first in the search list
    pub distance_to_relative_ancestor: usize,
}

// This has only one implementation. It's here to invert dependencies so fuzzy
// does not need to depend on project. Though we also use it to make testing easier.
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
        println!(
            "{:?}: {}, {:?} {}",
            self.path, self.score, other.path, other.score
        );
        dbg!(self.score, other.score);
        self.score
            .total_cmp(&other.score)
            .reverse()
            .then_with(|| self.worktree_id.cmp(&other.worktree_id))
            .then_with(|| {
                other
                    .distance_to_relative_ancestor
                    .cmp(&self.distance_to_relative_ancestor)
            })
            // see shorter_over_lexicographical test for an example of why we want this
            .then_with(|| {
                self.path
                    .as_unix_str()
                    .chars()
                    .count()
                    .cmp(&other.path.as_unix_str().chars().count())
            })
            .then_with(|| self.path.cmp(&other.path))
    }
}

pub fn match_fixed_path_set(
    candidates: Vec<PathMatchCandidate>,
    worktree_id: usize,
    worktree_root_name: Option<Arc<RelPath>>,
    query: &str,
    smart_case: bool,
    max_results: usize,
    path_style: PathStyle,
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

    let path_prefix = worktree_root_name.unwrap_or(RelPath::empty().into());
    let path_prefix_len = path_prefix.display(path_style).len();
    let mut candidate_buf = dbg!(path_prefix.display(path_style).to_string());
    let mut results = Vec::with_capacity(candidates.len());
    for c in candidates {
        let mut indices = Vec::new();
        let mut buf = Vec::new();
        candidate_buf.truncate(path_prefix_len);
        candidate_buf.push_str(c.path.as_unix_str());
        if let Some(score) = pattern.indices(
            nucleo::Utf32Str::new(&candidate_buf, &mut buf),
            &mut matcher,
            &mut indices,
        ) {
            results.push(PathMatch {
                score: score as f64,
                worktree_id,
                positions: indices.into_iter().map(|n| n as usize).collect(),
                is_dir: c.is_dir,
                path: c.path.into(),
                path_prefix: Arc::clone(&path_prefix),
                distance_to_relative_ancestor: usize::MAX,
            })
        };
    }
    matcher::return_matcher(matcher);
    util::truncate_to_bottom_n_sorted(&mut results, max_results);
    for r in &mut results {
        r.positions.sort();
    }
    results
}

pub fn path_match_helper<'a>(
    matcher: &mut nucleo::Matcher,
    pattern: &Pattern,
    candidates: impl Iterator<Item = PathMatchCandidate<'a>>,
    worktree_id: usize,
    relative_to: &Option<Arc<RelPath>>,
    path_style: PathStyle,
    results: &mut Vec<PathMatch>,
) {
    let path_prefix = relative_to.clone().unwrap_or(RelPath::empty().into());
    let path_prefix_len = path_prefix.display(path_style).len();
    let mut candidate_buf = dbg!(path_prefix.display(path_style).to_string());
    for c in candidates {
        let mut indices = Vec::new();
        let mut buf = Vec::new();
        candidate_buf.truncate(path_prefix_len);
        candidate_buf.push_str(c.path.as_unix_str());
        if let Some(score) = pattern.indices(
            nucleo::Utf32Str::new(&candidate_buf, &mut buf),
            matcher,
            &mut indices,
        ) {

            results.push(PathMatch {
                score: score as f64,
                worktree_id,
                positions: indices.into_iter().map(|n| n as usize).collect(),
                is_dir: c.is_dir,
                path: c.path.into(),
                path_prefix: Arc::clone(&path_prefix),
                distance_to_relative_ancestor: relative_to
                    .as_ref()
                    .map_or(usize::MAX, |relative_to| {
                        distance_between_paths(c.path, relative_to.as_ref())
                    }),
            })
        };
    }
}

/// Query should contain spaces if you want it to be matched out of order
/// for example: 'audio Cargo' matching 'audio/Cargo.toml'
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
                            for c in candidates {
                                if cancel_flag.load(std::sync::atomic::Ordering::Relaxed) {
                                    break 'outer;
                                }
                                let mut indices = Vec::new();
                                let mut buf = Vec::new();
                                if let Some(score) = pattern.indices(
                                    nucleo::Utf32Str::new(&c.path.as_unix_str(), &mut buf),
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
    util::truncate_to_bottom_n_sorted(&mut results, max_results);
    for r in &mut results {
        r.positions.sort();
    }

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
    use std::sync::{Arc, atomic::AtomicBool};

    use gpui::TestAppContext;
    use util::{paths::PathStyle, rel_path::RelPath};

    use crate::{CharBag, PathMatchCandidate, PathMatchCandidateSet};

    use super::distance_between_paths;

    #[test]
    fn test_distance_between_paths_empty() {
        distance_between_paths(RelPath::empty(), RelPath::empty());
    }

    struct TestCandidateSet<'a> {
        prefix: Arc<RelPath>,
        candidates: Vec<PathMatchCandidate<'a>>,
        path_style: PathStyle,
    }

    impl<'a> PathMatchCandidateSet<'a> for TestCandidateSet<'a> {
        type Candidates = std::vec::IntoIter<PathMatchCandidate<'a>>;

        fn id(&self) -> usize {
            0
        }
        fn len(&self) -> usize {
            self.candidates.len()
        }
        fn is_empty(&self) -> bool {
            self.candidates.is_empty()
        }
        fn root_is_file(&self) -> bool {
            true // TODO: swap this
        }
        fn prefix(&self) -> Arc<RelPath> {
            self.prefix.clone()
        }
        fn candidates(&self, start: usize) -> Self::Candidates {
            self.candidates[start..]
                .iter()
                .cloned()
                .collect::<Vec<_>>()
                .into_iter()
        }
        fn path_style(&self) -> PathStyle {
            self.path_style
        }
    }

    async fn path_matches(
        cx: &mut TestAppContext,
        candidates: &'static [&'static str],
        query: &'static str,
    ) -> Vec<String> {
        let set = TestCandidateSet {
            prefix: RelPath::unix("a/b").unwrap().into(),
            candidates: candidates
                .into_iter()
                .map(|s| PathMatchCandidate {
                    is_dir: false,
                    path: RelPath::unix(s).unwrap().into(),
                    char_bag: CharBag::from_iter(s.to_lowercase().chars()),
                })
                .collect(),
            path_style: PathStyle::Windows,
        };
        let candidate_sets = vec![set];

        let cancellation_flag = AtomicBool::new(false);
        let executor = cx.background_executor.clone();
        let matches = cx
            .foreground_executor
            .spawn(async move {
                super::match_path_sets(
                    candidate_sets.as_slice(),
                    query,
                    &None,
                    false,
                    100,
                    &cancellation_flag,
                    executor,
                )
                .await
            })
            .await;

        matches
            .iter()
            .map(|s| s.path.as_unix_str().to_string())
            .collect::<Vec<_>>()
    }

    #[gpui::test]
    async fn test_dir_paths(cx: &mut TestAppContext) {
        const CANDIDATES: &'static [&'static str] = &[
            "gpui_even_more/Cargo.toml",
            "gpui_more/Cargo.toml",
            "gpui/Cargo.toml",
        ];

        assert_eq!(
            path_matches(cx, CANDIDATES, "toml gpui").await,
            [
                "gpui/Cargo.toml",
                "gpui_more/Cargo.toml",
                "gpui_even_more/Cargo.toml",
            ]
        );

        assert_eq!(
            path_matches(cx, CANDIDATES, "gpui more").await,
            ["gpui_more/Cargo.toml", "gpui_even_more/Cargo.toml",]
        );
    }
    #[gpui::test]
    async fn test_more_dir_paths(cx: &mut TestAppContext) {
        const CANDIDATES: &'static [&'static str] = &[
            "crates/gpui_macros/Cargo.toml",
            "crates/gpui_tokio/Cargo.toml",
            "crates/gpui/Cargo.toml",
        ];

        assert_eq!(
            path_matches(cx, CANDIDATES, "toml gpui").await,
            [
                "crates/gpui/Cargo.toml",
                "crates/gpui_tokio/Cargo.toml",
                "crates/gpui_macros/Cargo.toml"
            ]
        );
    }

    #[gpui::test]
    async fn denoise(cx: &mut TestAppContext) {
        const CANDIDATES: &'static [&'static str] = &[
            "crates/debug_adapter_extension/Cargo.toml",
            "crates/debugger_tools/Cargo.toml",
            "crates/debugger_ui/Cargo.toml",
            "crates/deepseek/Cargo.toml",
            "crates/denoise/Cargo.toml",
        ];

        assert_eq!(
            path_matches(cx, CANDIDATES, "toml de").await,
            [
                "crates/denoise/Cargo.toml",
                "crates/deepseek/Cargo.toml",
                "crates/debugger_ui/Cargo.toml",
                "crates/debugger_tools/Cargo.toml",
                "crates/debug_adapter_extension/Cargo.toml",
            ]
        );
    }

    #[gpui::test]
    async fn test_path_matcher(cx: &mut TestAppContext) {
        const CANDIDATES: &'static [&'static str] = &[
            "blue", "red", "purple", "pink", "green", "yellow", "magenta", "orange", "ocean",
            "navy", "brown",
        ];
        assert_eq!(path_matches(cx, CANDIDATES, "bl").await, ["blue"]);
    }

    #[gpui::test]
    async fn shorter_over_lexicographical(cx: &mut TestAppContext) {
        const CANDIDATES: &'static [&'static str] = &["qr", "qqqqqqqqqqqq"];
        assert_eq!(
            path_matches(cx, CANDIDATES, "q").await,
            ["qr", "qqqqqqqqqqqq"]
        );
    }
    // TODO: add perf test on zed repo

    #[gpui::test]
    async fn prefer_single_word_match_to_multiple_fragments(cx: &mut TestAppContext) {
        const CANDIDATES: &'static [&'static str] = &[
            "crates/theme_importer/README.md",
            "extensions/test-extension/README.md",
            "extensions/slash-commands-example/README.md",
            "crates/livekit_api/vendored/protocol/README.md",
            "crates/assistant_tools/src/read_file_tool/description.md",
        ];
        assert_eq!(path_matches(cx, CANDIDATES, "read").await, CANDIDATES);
    }
}
