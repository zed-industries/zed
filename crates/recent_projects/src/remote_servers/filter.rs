//! Fuzzy filtering for the remote-projects modal.
//!
//! At construction time we build [`FilterData`]: a flat list of
//! [`StringMatchCandidate`]s with one candidate per project for servers that
//! have projects, and one candidate per server (over the host name) for
//! project-less servers and SSH-config-only entries. Each candidate is tagged
//! with [`CandidateMeta`] recording which server (and project, if any) it
//! represents. This snapshot is reused for every keystroke; we only rebuild
//! it when the set of servers changes.
//!
//! On each query, `fuzzy_nucleo::match_strings[_async]` returns matches sorted
//! by score. [`build_filter_results`] regroups those matches by server (a
//! server with N projects can contribute up to N matches) and folds each
//! bucket into a single [`FilteredServer`] carrying highlight positions and
//! the best score across its candidates.
//!
//! Projects inside a [`FilteredServer`] are intentionally ordered by fuzzy
//! score, not by their position in the source list — when a query matches
//! only the host name, the server's projects come back ranked by how well
//! each one also matched.
//!
//! The caller stores the resulting `Vec<FilteredServer>` in
//! [`DefaultState::filtered_servers`] and renders by looking up each
//! `server_index` / `project_index` against the unchanged source list.

use std::sync::atomic::{self, AtomicBool};

use fuzzy_nucleo::{StringMatch, StringMatchCandidate};
use gpui::BackgroundExecutor;

use super::RemoteEntry;

#[derive(Debug)]
pub(super) struct FilterData {
    pub(super) candidates: Vec<StringMatchCandidate>,
    pub(super) meta: Vec<CandidateMeta>,
    pub(super) server_count: usize,
}

#[derive(Debug)]
pub(super) struct CandidateMeta {
    pub(super) server_index: usize,
    pub(super) project_index: Option<usize>,
    pub(super) host_byte_len: usize,
}

#[derive(Clone, Debug)]
pub(super) struct FilteredServer {
    pub(super) server_index: usize,
    pub(super) host_positions: Vec<usize>,
    pub(super) project_matches: Vec<FilteredProject>,
    pub(super) score: f64,
}

#[derive(Clone, Debug)]
pub(super) struct FilteredProject {
    pub(super) project_index: usize,
    pub(super) path_positions: Vec<usize>,
}

impl FilterData {
    pub(super) fn build(servers: &[RemoteEntry]) -> Self {
        let mut candidates = Vec::new();
        let mut meta = Vec::new();
        for (server_index, server) in servers.iter().enumerate() {
            let host = server.host_name();
            let host_byte_len = host.len();
            match server {
                RemoteEntry::Project { projects, .. } if !projects.is_empty() => {
                    for (project_index, entry) in projects.iter().enumerate() {
                        let combined = format!("{host} {}", entry.project.paths.join(", "));
                        meta.push(CandidateMeta {
                            server_index,
                            project_index: Some(project_index),
                            host_byte_len,
                        });
                        candidates.push(StringMatchCandidate::new(candidates.len(), combined));
                    }
                }
                RemoteEntry::Project { .. } | RemoteEntry::SshConfig { .. } => {
                    meta.push(CandidateMeta {
                        server_index,
                        project_index: None,
                        host_byte_len,
                    });
                    candidates.push(StringMatchCandidate::new(candidates.len(), host));
                }
            }
        }
        Self {
            candidates,
            meta,
            server_count: servers.len(),
        }
    }
}

pub(super) fn build_filter_results(
    matches: Vec<StringMatch>,
    filter_data: &FilterData,
) -> Vec<FilteredServer> {
    group_matches_by_server(matches, filter_data)
        .into_iter()
        .enumerate()
        .filter_map(|(server_index, group)| {
            (!group.is_empty()).then(|| build_server_result(server_index, group))
        })
        .collect()
}

fn group_matches_by_server(
    matches: Vec<StringMatch>,
    filter_data: &FilterData,
) -> Vec<Vec<(StringMatch, &CandidateMeta)>> {
    let mut buckets: Vec<Vec<_>> = (0..filter_data.server_count).map(|_| Vec::new()).collect();
    for m in matches {
        let meta = &filter_data.meta[m.candidate_id];
        buckets[meta.server_index].push((m, meta));
    }
    buckets
}

fn build_server_result(
    server_index: usize,
    group: Vec<(StringMatch, &CandidateMeta)>,
) -> FilteredServer {
    debug_assert!(!group.is_empty(), "empty groups are filtered out upstream");

    let mut host_positions = Vec::new();
    let mut project_matches = Vec::new();
    let mut score = f64::NEG_INFINITY;

    for (m, meta) in group {
        score = score.max(m.score);
        match meta.project_index {
            None => {
                host_positions = m.positions;
            }
            Some(project_index) => {
                // +1 accounts for the single-byte space separator in
                // format!("{host} {paths}") used by FilterData::build. A
                // position landing exactly on the separator (host_byte_len)
                // is dropped from both sides — it indexes into synthetic
                // content that isn't shown anywhere.
                let host_prefix_len = meta.host_byte_len + 1;
                host_positions.extend(
                    m.positions
                        .iter()
                        .copied()
                        .filter(|&p| p < meta.host_byte_len),
                );
                project_matches.push(FilteredProject {
                    project_index,
                    path_positions: m
                        .positions
                        .into_iter()
                        .filter_map(|p| p.checked_sub(host_prefix_len))
                        .collect(),
                });
            }
        }
    }

    host_positions.sort_unstable();
    host_positions.dedup();

    FilteredServer {
        server_index,
        host_positions,
        project_matches,
        score,
    }
}

pub(super) fn run_sync(data: &FilterData, query: &str) -> Vec<FilteredServer> {
    let case = fuzzy_nucleo::Case::smart_if_uppercase_in(query);
    let matches = fuzzy_nucleo::match_strings(
        &data.candidates,
        query,
        case,
        fuzzy_nucleo::LengthPenalty::Off,
        data.candidates.len(),
    );
    let mut results = build_filter_results(matches, data);
    results.sort_by(|a, b| b.score.total_cmp(&a.score));
    results
}

pub(super) async fn run_async(
    data: &FilterData,
    query: &str,
    cancel: &AtomicBool,
    executor: BackgroundExecutor,
) -> Option<Vec<FilteredServer>> {
    let case = fuzzy_nucleo::Case::smart_if_uppercase_in(query);
    let matches = fuzzy_nucleo::match_strings_async(
        &data.candidates,
        query,
        case,
        fuzzy_nucleo::LengthPenalty::Off,
        data.candidates.len(),
        cancel,
        executor,
    )
    .await;
    if cancel.load(atomic::Ordering::Acquire) {
        return None;
    }
    let mut results = build_filter_results(matches, data);
    results.sort_by(|a, b| b.score.total_cmp(&a.score));
    Some(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::remote_connections::{Connection, SshConnection};
    use crate::remote_servers::{ProjectEntry, ServerIndex, SshServerIndex};
    use gpui::{App, ScrollHandle};
    use settings::RemoteProject;
    use ui::{NavigableEntry, SharedString};

    struct MockServer {
        host: &'static str,
        project_paths: &'static [&'static str],
    }

    fn mock(host: &'static str, project_paths: &'static [&'static str]) -> MockServer {
        MockServer {
            host,
            project_paths,
        }
    }

    fn build_entries(cx: &App, handle: &ScrollHandle, servers: &[MockServer]) -> Vec<RemoteEntry> {
        servers
            .iter()
            .map(|server| {
                if server.project_paths.is_empty() {
                    RemoteEntry::SshConfig {
                        open_folder: NavigableEntry::new(handle, cx),
                        host: SharedString::from(server.host),
                    }
                } else {
                    let projects = server
                        .project_paths
                        .iter()
                        .map(|path| ProjectEntry {
                            navigation: NavigableEntry::new(handle, cx),
                            project: RemoteProject {
                                paths: vec![(*path).to_string()],
                            },
                        })
                        .collect();
                    let connection = Connection::Ssh(SshConnection {
                        host: server.host.to_string(),
                        projects: server
                            .project_paths
                            .iter()
                            .map(|p| RemoteProject {
                                paths: vec![(*p).to_string()],
                            })
                            .collect(),
                        ..Default::default()
                    });
                    RemoteEntry::Project {
                        open_folder: NavigableEntry::new(handle, cx),
                        projects,
                        configure: NavigableEntry::new(handle, cx),
                        connection,
                        index: ServerIndex::Ssh(SshServerIndex(0)),
                    }
                }
            })
            .collect()
    }

    fn with_filter_data<R>(
        cx: &App,
        servers: &[MockServer],
        f: impl FnOnce(&[MockServer], &FilterData) -> R,
    ) -> R {
        let handle = ScrollHandle::new();
        let entries = build_entries(cx, &handle, servers);
        let data = FilterData::build(&entries);
        f(servers, &data)
    }

    #[gpui::test]
    async fn test_filter_host_only(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            with_filter_data(cx, &[mock("myhost", &[])], |_, data| {
                let results = run_sync(data, "myh");
                assert_eq!(results.len(), 1);
                assert_eq!(results[0].server_index, 0);
                assert!(!results[0].host_positions.is_empty());
            });
        });
    }

    #[gpui::test]
    async fn test_filter_no_match(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            with_filter_data(cx, &[mock("myhost", &["/home/project"])], |_, data| {
                let results = run_sync(data, "zzz");
                assert!(results.is_empty());
            });
        });
    }

    #[gpui::test]
    async fn test_filter_project_path_match(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            with_filter_data(cx, &[mock("myhost", &["/home/user/project"])], |_, data| {
                let results = run_sync(data, "project");
                assert_eq!(results.len(), 1);
                assert_eq!(results[0].project_matches.len(), 1);
                assert_eq!(results[0].project_matches[0].project_index, 0);
            });
        });
    }

    #[gpui::test]
    async fn test_filter_host_match_includes_all_projects(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            with_filter_data(cx, &[mock("myhost", &["/path/a", "/path/b"])], |_, data| {
                let results = run_sync(data, "myhost");
                assert_eq!(results.len(), 1);
                assert_eq!(results[0].project_matches.len(), 2);
            });
        });
    }

    #[gpui::test]
    async fn test_filter_excludes_non_matching_servers(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            with_filter_data(
                cx,
                &[mock("alpha", &["/path/a"]), mock("beta", &["/path/b"])],
                |_, data| {
                    let results = run_sync(data, "alpha");
                    assert_eq!(results.len(), 1);
                    assert_eq!(results[0].server_index, 0);
                },
            );
        });
    }

    #[gpui::test]
    async fn test_position_mapping_splits_host_and_path(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            with_filter_data(cx, &[mock("dev", &["/src/app"])], |servers, data| {
                let results = run_sync(data, "dev app");

                assert_eq!(results.len(), 1);
                let result = &results[0];
                let host = servers[result.server_index].host;
                let path = servers[result.server_index].project_paths[0];

                assert!(
                    result.host_positions.iter().all(|&p| p < host.len()),
                    "host positions {:?} must be within host {:?} (len {})",
                    result.host_positions,
                    host,
                    host.len(),
                );

                assert_eq!(result.project_matches.len(), 1);
                let proj = &result.project_matches[0];
                assert_eq!(proj.project_index, 0);
                assert!(
                    proj.path_positions.iter().all(|&p| p < path.len()),
                    "path positions {:?} must be within path {:?} (len {})",
                    proj.path_positions,
                    path,
                    path.len(),
                );

                assert!(
                    !result.host_positions.is_empty(),
                    "query 'dev' should match host 'dev'"
                );
                assert!(
                    !proj.path_positions.is_empty(),
                    "query 'app' should match path '/src/app'"
                );
            });
        });
    }

    #[gpui::test]
    async fn test_position_mapping_host_only_server(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            with_filter_data(cx, &[mock("myhost", &[])], |servers, data| {
                let results = run_sync(data, "myh");
                assert_eq!(results.len(), 1);
                let host = servers[0].host;
                assert!(
                    results[0].host_positions.iter().all(|&p| p < host.len()),
                    "host positions {:?} out of bounds for {:?}",
                    results[0].host_positions,
                    host,
                );
                assert!(results[0].project_matches.is_empty());
            });
        });
    }

    #[gpui::test]
    async fn test_unicode_host_and_path_positions(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            with_filter_data(cx, &[mock("señor", &["/código/app"])], |servers, data| {
                let results = run_sync(data, "señ app");
                assert_eq!(results.len(), 1);
                let result = &results[0];
                let host = servers[0].host;
                let path = servers[0].project_paths[0];

                assert!(
                    result
                        .host_positions
                        .iter()
                        .all(|&p| p < host.len() && host.is_char_boundary(p)),
                    "host positions {:?} must be valid char boundaries in {:?}",
                    result.host_positions,
                    host,
                );

                assert_eq!(result.project_matches.len(), 1);
                let proj = &result.project_matches[0];
                assert!(
                    proj.path_positions
                        .iter()
                        .all(|&p| p < path.len() && path.is_char_boundary(p)),
                    "path positions {:?} must be valid char boundaries in {:?}",
                    proj.path_positions,
                    path,
                );
            });
        });
    }

    #[gpui::test]
    async fn test_filter_data_build_from_real_entries(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            with_filter_data(cx, &[mock("alpha", &[]), mock("beta", &[])], |_, data| {
                assert_eq!(data.server_count, 2);
                assert_eq!(data.candidates.len(), 2);
                assert_eq!(data.candidates[0].string, "alpha");
                assert_eq!(data.candidates[1].string, "beta");

                let results = run_sync(data, "alp");
                assert_eq!(results.len(), 1);
                assert_eq!(results[0].server_index, 0);
                assert!(!results[0].host_positions.is_empty());

                let empty = run_sync(data, "zzz");
                assert!(empty.is_empty());
            });
        });
    }

    #[gpui::test]
    async fn test_run_async_returns_none_when_cancelled(cx: &mut gpui::TestAppContext) {
        let data = cx.update(|cx| {
            let handle = ScrollHandle::new();
            let entries = build_entries(cx, &handle, &[mock("alpha", &[])]);
            FilterData::build(&entries)
        });
        let cancel = AtomicBool::new(true);
        let executor = cx.background_executor.clone();
        let result = run_async(&data, "alpha", &cancel, executor).await;
        assert!(
            result.is_none(),
            "cancel set before run should short-circuit"
        );
    }
}
