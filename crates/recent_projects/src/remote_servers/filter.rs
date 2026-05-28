//! Fuzzy filtering for the remote-projects modal.
//!
//! At construction time we build [`FilterData`]: a flat list of
//! [`StringMatchCandidate`]s, one per project, or one per server when a server
//! has no projects,
//! each one is tagged with [`CandidateMeta`] recording which server/project the
//! candidate came from. This snapshot is reused for every keystroke. we only
//! rebuild it when the set of servers changes.
//!
//! On each query, `fuzzy_nucleo::match_strings[_async]` returns matches sorted
//! by score. [`build_filter_results`] regroups those matches by server (one
//! server can appear in multiple matches — once per project, plus once for
//! the host candidate) and folds each bucket into a single [`FilteredServer`]
//! carrying highlight positions and the best score across its candidates.
//!
//! The caller then unpacks each `FilteredServer` back into a UI `RemoteEntry`
//! enriched with highlight positions, producing
//! `DefaultState::filtered_servers`. `DefaultState::active_servers()` returns
//! that filtered list when present, else the unfiltered source list.

use std::sync::atomic::{self, AtomicBool};

use fuzzy_nucleo::{StringMatch, StringMatchCandidate};
use gpui::BackgroundExecutor;

use super::{ProjectEntry, RemoteEntry};

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

#[derive(Debug)]
pub(super) struct FilteredServer {
    pub(super) server_index: usize,
    pub(super) host_positions: Vec<usize>,
    pub(super) project_matches: Vec<FilteredProject>,
    pub(super) score: f64,
}

#[derive(Debug)]
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
                // format!("{host} {paths}") used by FilterData::build.
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

pub(super) fn apply(source: &[RemoteEntry], results: &[FilteredServer]) -> Vec<RemoteEntry> {
    results
        .iter()
        .filter_map(|result| {
            let entry = source.get(result.server_index)?;
            Some(match entry {
                RemoteEntry::Project {
                    open_folder,
                    projects,
                    configure,
                    connection,
                    index,
                    ..
                } => RemoteEntry::Project {
                    open_folder: open_folder.clone(),
                    configure: configure.clone(),
                    connection: connection.clone(),
                    index: *index,
                    host_positions: result.host_positions.clone(),
                    projects: result
                        .project_matches
                        .iter()
                        .filter_map(|pm| {
                            let project = projects.get(pm.project_index)?;
                            Some(ProjectEntry {
                                navigation: project.navigation.clone(),
                                project: project.project.clone(),
                                highlight_positions: pm.path_positions.clone(),
                            })
                        })
                        .collect(),
                },
                RemoteEntry::SshConfig {
                    open_folder, host, ..
                } => RemoteEntry::SshConfig {
                    open_folder: open_folder.clone(),
                    host: host.clone(),
                    host_positions: result.host_positions.clone(),
                },
            })
        })
        .collect()
}
