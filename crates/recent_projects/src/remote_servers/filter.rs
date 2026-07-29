//! Fuzzy filtering for the remote-projects modal.
//!
//! At construction time we build [`FilterData`]: a flat list of
//! [`StringMatchCandidate`]s with one candidate per project for servers that
//! have projects, and one candidate per server (over the host name) for
//! project-less servers and SSH-config-only entries. Each candidate matches
//! against the displayed host plus any search-only alias (the real SSH host
//! when a nickname hides it), so a server stays findable by either name. Each
//! candidate is tagged with [`CandidateMeta`] recording which server (and
//! project, if any) it represents. This snapshot is reused for every
//! keystroke; we only rebuild it when the set of servers changes.
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
    /// Byte length of the host text that is actually displayed (and thus
    /// highlightable) as the server's primary label. Host match positions at
    /// or beyond this are dropped — they fall inside the search-only alias.
    pub(super) display_host_byte_len: usize,
    /// Byte length of the full searchable host text (display host plus any
    /// alias), used to find where the project-path portion of the combined
    /// candidate string begins.
    pub(super) match_host_byte_len: usize,
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
            let display_host = server.display_host();
            let display_host_byte_len = display_host.len();
            let search_host = match server.host_alias() {
                Some(alias) => format!("{display_host} {alias}"),
                None => display_host.to_string(),
            };
            let match_host_byte_len = search_host.len();
            match server {
                RemoteEntry::Project { projects, .. } if !projects.is_empty() => {
                    for (project_index, entry) in projects.iter().enumerate() {
                        let combined = format!("{search_host} {}", entry.project.paths.join(", "));
                        meta.push(CandidateMeta {
                            server_index,
                            project_index: Some(project_index),
                            display_host_byte_len,
                            match_host_byte_len,
                        });
                        candidates.push(StringMatchCandidate::new(candidates.len(), combined));
                    }
                }
                RemoteEntry::Project { .. } | RemoteEntry::SshConfig { .. } => {
                    meta.push(CandidateMeta {
                        server_index,
                        project_index: None,
                        display_host_byte_len,
                        match_host_byte_len,
                    });
                    candidates.push(StringMatchCandidate::new(candidates.len(), search_host));
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
        let Some(meta) = filter_data.meta.get(m.candidate_id) else {
            continue;
        };
        let Some(bucket) = buckets.get_mut(meta.server_index) else {
            continue;
        };
        bucket.push((m, meta));
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
        // `FilterData::build` emits either one host-only candidate or one
        // candidate per project for a server, never a mix, so the `None` arm
        // assigning `host_positions` can't clobber positions accumulated by
        // the `Some` arm.
        match meta.project_index {
            None => {
                host_positions = m
                    .positions
                    .into_iter()
                    .filter(|&p| p < meta.display_host_byte_len)
                    .collect();
            }
            Some(project_index) => {
                // +1 accounts for the single-byte space separator in
                // format!("{search_host} {paths}") used by FilterData::build.
                // Positions inside the search-only host alias (between the
                // displayed host and the separator) are dropped from both
                // sides — they index into content that isn't shown anywhere.
                let host_prefix_len = meta.match_host_byte_len + 1;
                host_positions.extend(
                    m.positions
                        .iter()
                        .copied()
                        .filter(|&p| p < meta.display_host_byte_len),
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
    use settings::RemoteProject;
    use ui::SharedString;

    struct MockServer {
        host: &'static str,
        nickname: Option<&'static str>,
        project_paths: &'static [&'static str],
    }

    fn mock(host: &'static str, project_paths: &'static [&'static str]) -> MockServer {
        MockServer {
            host,
            nickname: None,
            project_paths,
        }
    }

    fn mock_with_nickname(
        host: &'static str,
        nickname: &'static str,
        project_paths: &'static [&'static str],
    ) -> MockServer {
        MockServer {
            host,
            nickname: Some(nickname),
            project_paths,
        }
    }

    fn build_entries(servers: &[MockServer]) -> Vec<RemoteEntry> {
        servers
            .iter()
            .map(|server| {
                if server.project_paths.is_empty() {
                    RemoteEntry::SshConfig {
                        host: SharedString::from(server.host),
                    }
                } else {
                    let projects = server
                        .project_paths
                        .iter()
                        .map(|path| ProjectEntry {
                            project: RemoteProject {
                                paths: vec![(*path).to_string()],
                            },
                        })
                        .collect();
                    let connection = Connection::Ssh(SshConnection {
                        host: server.host.to_string(),
                        nickname: server.nickname.map(str::to_string),
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
                        projects,
                        connection,
                        index: ServerIndex::Ssh(SshServerIndex(0)),
                    }
                }
            })
            .collect()
    }

    fn with_filter_data<R>(
        servers: &[MockServer],
        f: impl FnOnce(&[MockServer], &FilterData) -> R,
    ) -> R {
        let entries = build_entries(servers);
        let data = FilterData::build(&entries);
        f(servers, &data)
    }

    #[test]
    fn test_filter_host_only() {
        with_filter_data(&[mock("myhost", &[])], |_, data| {
            let results = run_sync(data, "myh");
            assert_eq!(results.len(), 1);
            assert_eq!(results[0].server_index, 0);
            assert!(!results[0].host_positions.is_empty());
        });
    }

    #[test]
    fn test_filter_no_match() {
        with_filter_data(&[mock("myhost", &["/home/project"])], |_, data| {
            let results = run_sync(data, "zzz");
            assert!(results.is_empty());
        });
    }

    #[test]
    fn test_filter_project_path_match() {
        with_filter_data(&[mock("myhost", &["/home/user/project"])], |_, data| {
            let results = run_sync(data, "project");
            assert_eq!(results.len(), 1);
            assert_eq!(results[0].project_matches.len(), 1);
            assert_eq!(results[0].project_matches[0].project_index, 0);
        });
    }

    #[test]
    fn test_filter_host_match_includes_all_projects() {
        with_filter_data(&[mock("myhost", &["/path/a", "/path/b"])], |_, data| {
            let results = run_sync(data, "myhost");
            assert_eq!(results.len(), 1);
            assert_eq!(results[0].project_matches.len(), 2);
        });
    }

    #[test]
    fn test_filter_excludes_non_matching_servers() {
        with_filter_data(
            &[mock("alpha", &["/path/a"]), mock("beta", &["/path/b"])],
            |_, data| {
                let results = run_sync(data, "alpha");
                assert_eq!(results.len(), 1);
                assert_eq!(results[0].server_index, 0);
            },
        );
    }

    #[test]
    fn test_position_mapping_splits_host_and_path() {
        with_filter_data(&[mock("dev", &["/src/app"])], |servers, data| {
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
    }

    #[test]
    fn test_position_mapping_host_only_server() {
        with_filter_data(&[mock("myhost", &[])], |servers, data| {
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
    }

    #[test]
    fn test_unicode_host_and_path_positions() {
        with_filter_data(&[mock("señor", &["/código/app"])], |servers, data| {
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
    }

    #[test]
    fn test_filter_data_build_from_real_entries() {
        with_filter_data(&[mock("alpha", &[]), mock("beta", &[])], |_, data| {
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
    }

    #[gpui::test]
    async fn test_run_async_returns_none_when_cancelled(cx: &mut gpui::TestAppContext) {
        let data = FilterData::build(&build_entries(&[mock("alpha", &[])]));
        let cancel = AtomicBool::new(true);
        let executor = cx.background_executor.clone();
        let result = run_async(&data, "alpha", &cancel, executor).await;
        assert!(
            result.is_none(),
            "cancel set before run should short-circuit"
        );
    }

    #[gpui::test]
    async fn test_run_async_returns_results_when_not_cancelled(cx: &mut gpui::TestAppContext) {
        let data = FilterData::build(&build_entries(&[mock("alpha", &["/home/project"])]));
        let cancel = AtomicBool::new(false);
        let executor = cx.background_executor.clone();
        let results = run_async(&data, "alpha", &cancel, executor)
            .await
            .expect("uncancelled run should return results");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].server_index, 0);
        assert!(!results[0].host_positions.is_empty());
    }

    #[test]
    fn test_filter_matches_nickname_and_host() {
        let servers = [mock_with_nickname("10.0.0.5", "prod", &["/srv/app"])];
        with_filter_data(&servers, |servers, data| {
            let nickname = servers[0].nickname.expect("server has a nickname");

            let by_nickname = run_sync(data, "prod");
            assert_eq!(by_nickname.len(), 1, "nickname should match");
            assert!(
                !by_nickname[0].host_positions.is_empty(),
                "matching the nickname should highlight it"
            );
            assert!(
                by_nickname[0]
                    .host_positions
                    .iter()
                    .all(|&p| p < nickname.len()),
                "host positions {:?} must stay within the displayed nickname {:?}",
                by_nickname[0].host_positions,
                nickname,
            );

            let by_host = run_sync(data, "10.0");
            assert_eq!(by_host.len(), 1, "real host should remain searchable");
            assert!(
                by_host[0].host_positions.is_empty(),
                "alias-only matches are searchable but not highlighted, got {:?}",
                by_host[0].host_positions,
            );
        });
    }

    #[test]
    fn test_projects_ordered_by_match_score() {
        with_filter_data(&[mock("srv", &["/a", "/b"])], |_, data| {
            // candidate 0 -> project 0, candidate 1 -> project 1; feed them
            // in descending-score order as `match_strings` would, then check
            // the regrouping keeps the higher-scored project first.
            let matches = vec![
                StringMatch {
                    candidate_id: 1,
                    score: 0.9,
                    positions: Vec::new(),
                    string: SharedString::default(),
                },
                StringMatch {
                    candidate_id: 0,
                    score: 0.5,
                    positions: Vec::new(),
                    string: SharedString::default(),
                },
            ];
            let results = build_filter_results(matches, data);
            assert_eq!(results.len(), 1);
            let project_indices: Vec<_> = results[0]
                .project_matches
                .iter()
                .map(|p| p.project_index)
                .collect();
            assert_eq!(project_indices, vec![1, 0]);
        });
    }
}
