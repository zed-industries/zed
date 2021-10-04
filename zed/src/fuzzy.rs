use gpui::executor;
use std::{
    cmp,
    sync::{atomic::AtomicBool, Arc},
};
use util;
use worktree::{EntryKind, Snapshot};

pub use fuzzy::*;

pub async fn match_strings(
    candidates: &[StringMatchCandidate],
    query: &str,
    smart_case: bool,
    max_results: usize,
    cancel_flag: &AtomicBool,
    background: Arc<executor::Background>,
) -> Vec<StringMatch> {
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
                    let segment_start = segment_idx * segment_size;
                    let segment_end = segment_start + segment_size;
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

pub async fn match_paths(
    snapshots: &[Snapshot],
    query: &str,
    include_ignored: bool,
    smart_case: bool,
    max_results: usize,
    cancel_flag: &AtomicBool,
    background: Arc<executor::Background>,
) -> Vec<PathMatch> {
    let path_count: usize = if include_ignored {
        snapshots.iter().map(Snapshot::file_count).sum()
    } else {
        snapshots.iter().map(Snapshot::visible_file_count).sum()
    };
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
                    for snapshot in snapshots {
                        let tree_end = if include_ignored {
                            tree_start + snapshot.file_count()
                        } else {
                            tree_start + snapshot.visible_file_count()
                        };

                        if tree_start < segment_end && segment_start < tree_end {
                            let path_prefix: Arc<str> =
                                if snapshot.root_entry().map_or(false, |e| e.is_file()) {
                                    snapshot.root_name().into()
                                } else if snapshots.len() > 1 {
                                    format!("{}/", snapshot.root_name()).into()
                                } else {
                                    "".into()
                                };

                            let start = cmp::max(tree_start, segment_start) - tree_start;
                            let end = cmp::min(tree_end, segment_end) - tree_start;
                            let paths = snapshot
                                .files(include_ignored, start)
                                .take(end - start)
                                .map(|entry| {
                                    if let EntryKind::File(char_bag) = entry.kind {
                                        PathMatchCandidate {
                                            path: &entry.path,
                                            char_bag,
                                        }
                                    } else {
                                        unreachable!()
                                    }
                                });

                            matcher.match_paths(
                                snapshot.id(),
                                path_prefix,
                                paths,
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
