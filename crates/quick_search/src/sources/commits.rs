use std::{
    ops::Range,
    sync::{Arc, OnceLock, atomic::AtomicBool},
};

use gpui::{App, AppContext, Context, WeakEntity};
use search::SearchOptions;
use text::Anchor as TextAnchor;
use ui::IconName;

use crate::PickerHandle;
use crate::QuickSearchDelegate;
use crate::preview::{PreviewKey, PreviewRequest};
use crate::types::{QuickMatch, QuickMatchKind};
use anyhow::{Context as AnyhowContext, Result};
use fuzzy::StringMatchCandidate;
use git2::Sort;
use log::debug;

use super::{
    ConfirmOutcome, GitCommitPreviewMeta, ListPresentation, PreviewPanelUi, QuickSearchSource,
    SortPolicy, SourceId, SourceSpec,
};

fn resolve_git_remote_for_workdir(
    repo_workdir: &Arc<std::path::Path>,
    project: &gpui::Entity<project::Project>,
    cx: &mut gpui::App,
) -> Option<::git::GitRemote> {
    let git_store = project.read(cx).git_store().read(cx);
    let repo = git_store
        .repositories()
        .values()
        .find(|repo| repo.read(cx).work_directory_abs_path.as_ref() == repo_workdir.as_ref())?;

    let snapshot = repo.read(cx).snapshot();
    let remote_url = snapshot
        .remote_upstream_url
        .as_ref()
        .or(snapshot.remote_origin_url.as_ref())?;

    let provider_registry = ::git::GitHostingProviderRegistry::default_global(cx);
    let (host, parsed) = ::git::parse_git_remote_url(provider_registry, remote_url)?;
    Some(::git::GitRemote {
        host,
        owner: parsed.owner.into(),
        repo: parsed.repo.into(),
    })
}

pub static COMMITS_SOURCE: CommitsSource = CommitsSource;

pub struct CommitsSource;

#[derive(Clone)]
pub struct GitCommitEntry {
    pub repo_workdir: Arc<std::path::Path>,
    pub sha: Arc<str>,
    pub subject: Arc<str>,
    pub commit_timestamp: i64,
    pub author_name: Arc<str>,
    pub branch: Option<Arc<str>>,
}

pub fn list_commits_local(
    repo_workdir: Arc<std::path::Path>,
    limit: usize,
) -> Result<Vec<GitCommitEntry>> {
    let repo = git2::Repository::open(repo_workdir.as_ref()).context("opening git repository")?;

    let branch: Option<Arc<str>> = repo
        .head()
        .ok()
        .and_then(|head| head.shorthand().map(|s| s.to_string()))
        .and_then(|name| {
            let name = name.trim();
            (!name.is_empty() && name != "HEAD").then(|| Arc::<str>::from(name.to_string()))
        });

    let mut revwalk = repo.revwalk().context("creating git revwalk")?;
    revwalk.push_head().context("pushing HEAD to revwalk")?;

    revwalk
        .set_sorting(Sort::TIME)
        .context("setting revwalk sorting")?;

    let mut commits = Vec::new();
    for oid in revwalk.take(limit) {
        let oid = match oid {
            Ok(oid) => oid,
            Err(_) => continue,
        };
        let commit = match repo.find_commit(oid) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let sha: Arc<str> = Arc::from(oid.to_string());
        let subject: Arc<str> = Arc::from(commit.summary().unwrap_or("").trim().to_string());
        let commit_timestamp = commit.time().seconds();
        let author_name: Arc<str> = Arc::from(
            commit
                .author()
                .name()
                .unwrap_or("unknown")
                .trim()
                .to_string(),
        );

        commits.push(GitCommitEntry {
            repo_workdir: repo_workdir.clone(),
            sha,
            subject,
            commit_timestamp,
            author_name,
            branch: branch.clone(),
        });
    }

    Ok(commits)
}

impl CommitsSource {
    fn spec_static() -> &'static SourceSpec {
        static SPEC: OnceLock<SourceSpec> = OnceLock::new();
        SPEC.get_or_init(|| SourceSpec {
            id: SourceId(Arc::from("commits")),
            title: Arc::from("Commits"),
            icon: IconName::GitBranchAlt,
            placeholder: Arc::from("Search commits..."),
            supported_options: SearchOptions::empty(),
            min_query_len: 1,
            list_presentation: ListPresentation::Flat,
            use_diff_preview: true,
            sort_policy: SortPolicy::StreamOrder,
        })
    }
}

impl QuickSearchSource for CommitsSource {
    fn spec(&self) -> &'static SourceSpec {
        Self::spec_static()
    }

    fn start_search(
        &self,
        delegate: &mut QuickSearchDelegate,
        query: String,
        generation: usize,
        cancel_flag: Arc<AtomicBool>,
        picker: WeakEntity<PickerHandle>,
        cx: &mut Context<PickerHandle>,
    ) {
        let repos = delegate
            .project
            .read(cx)
            .git_store()
            .read(cx)
            .repositories()
            .values()
            .cloned()
            .collect::<Vec<_>>();

        let repos = repos
            .into_iter()
            .map(|repo| {
                let repo_workdir = repo.read(cx).work_directory_abs_path.clone();
                (repo_workdir, repo)
            })
            .collect::<Vec<_>>();

        if repos.is_empty() {
            delegate.is_streaming = false;
            delegate.total_results = 0;
            delegate.query_error = Some("No Git repositories found in this project.".to_string());
            delegate.match_list.clear();
            delegate.stream_finished = true;
            cx.notify();
            return;
        }

        let executor = cx.background_executor().clone();
        let source_id = self.spec().id.0.clone();
        cx.spawn(move |_, app: &mut gpui::AsyncApp| {
            let mut app = app.clone();
            async move {
                if cancel_flag.load(std::sync::atomic::Ordering::Relaxed) {
                    return;
                }

                let mut commits = Vec::new();
                let mut used_fallback = false;
                for (repo_workdir, repo_entity) in repos {
                    if cancel_flag.load(std::sync::atomic::Ordering::Relaxed) {
                        return;
                    }

                    let local_workdir = repo_workdir.clone();
                    let local_task =
                        executor.spawn(async move { list_commits_local(local_workdir, 500) });
                    match local_task.await {
                        Ok(mut entries) => commits.append(&mut entries),
                        Err(err) => {
                            debug!(
                                "quick_search: local commit listing failed (falling back): {:?}",
                                err
                            );
                            used_fallback = true;

                            let branches_rx = app.update_entity(&repo_entity, |repo, _| repo.branches());
                            let branches_rx = match branches_rx {
                                Ok(rx) => rx,
                                Err(_) => continue,
                            };

                            let branches = match branches_rx.await {
                                Ok(Ok(branches)) => branches,
                                Ok(Err(_)) | Err(_) => Vec::new(),
                            };

                            let mut seen = std::collections::HashSet::<String>::new();
                            for b in branches {
                                let branch_name: Arc<str> = Arc::from(b.name().to_string());
                                let Some(summary) = b.most_recent_commit else {
                                    continue;
                                };
                                let sha = summary.sha.to_string();
                                if !seen.insert(sha.clone()) {
                                    continue;
                                }
                                commits.push(GitCommitEntry {
                                    repo_workdir: repo_workdir.clone(),
                                    sha: Arc::<str>::from(sha),
                                    subject: Arc::<str>::from(summary.subject.to_string()),
                                    commit_timestamp: summary.commit_timestamp,
                                    author_name: Arc::<str>::from(summary.author_name.to_string()),
                                    branch: Some(branch_name.clone()),
                                });
                            }

                            let head_commit = app
                                .update_entity(&repo_entity, |repo, _| {
                                    repo.snapshot().head_commit
                                })
                                .ok()
                                .flatten();
                            if let Some(head) = head_commit {
                                let sha = head.sha.to_string();
                                let subject = head
                                    .message
                                    .lines()
                                    .next()
                                    .unwrap_or("")
                                    .trim()
                                    .to_string();
                                if !sha.is_empty() {
                                    commits.push(GitCommitEntry {
                                        repo_workdir: repo_workdir.clone(),
                                        sha: Arc::<str>::from(sha),
                                        subject: Arc::<str>::from(subject),
                                        commit_timestamp: head.commit_timestamp,
                                        author_name: Arc::<str>::from(head.author_name.to_string()),
                                        branch: None,
                                    });
                                }
                            }
                        }
                    }
                }

                if let Some(picker_entity) = picker.upgrade() {
                    let notice = used_fallback.then_some(
                        "Some repositories are remote; showing branch-tip commits (full history unavailable).".to_string(),
                    );
                    if let Err(err) = app.update_entity(&picker_entity, |picker, cx| {
                        if picker.delegate.search_engine.generation() != generation {
                            return;
                        }
                        picker.delegate.query_notice = notice.clone();
                        cx.notify();
                    }) {
                        debug!("quick_search: failed to set git fallback notice: {:?}", err);
                    }
                }

                if commits.is_empty() {
                    crate::record_error(
                        picker,
                        generation,
                        "No commits found.".to_string(),
                        &mut app,
                    );
                    return;
                }

                let candidates = commits
                    .iter()
                    .enumerate()
                    .map(|(id, c)| {
                        let s = format!("{} {} {}", c.sha, c.subject, c.author_name);
                        StringMatchCandidate::new(id, &s)
                    })
                    .collect::<Vec<_>>();

                let mut matches = fuzzy::match_strings(
                    candidates.as_slice(),
                    &query,
                    true,
                    true,
                    1_000,
                    &cancel_flag,
                    executor,
                )
                .await;

                if cancel_flag.load(std::sync::atomic::Ordering::Relaxed) {
                    return;
                }

                matches.sort_by(|a, b| {
                    b.score
                        .partial_cmp(&a.score)
                        .unwrap_or(std::cmp::Ordering::Equal)
                        .then_with(|| {
                            let at = commits
                                .get(a.candidate_id)
                                .map(|c| c.commit_timestamp)
                                .unwrap_or(0);
                            let bt = commits
                                .get(b.candidate_id)
                                .map(|c| c.commit_timestamp)
                                .unwrap_or(0);
                            bt.cmp(&at)
                        })
                });

                let mut batch = Vec::with_capacity(matches.len());
                for m in matches {
                    let Some(commit) = commits.get(m.candidate_id) else {
                        continue;
                    };

                    let sha_short: Arc<str> =
                        Arc::from(commit.sha.get(..8).unwrap_or(&commit.sha).to_string());
                    let subject = commit.subject.clone();
                    let author = commit.author_name.clone();

                    let path_label: Arc<str> = Arc::from("");
                    let display_path: Arc<str> = Arc::from("");
                    let path_segments: Arc<[Arc<str>]> = Arc::from([]);

                    batch.push(QuickMatch {
                        id: 0,
                        key: crate::types::MatchKey(0),
                        source_id: source_id.clone(),
                        group: None,
                        path_label,
                        display_path,
                        display_path_positions: None,
                        path_segments,
                        file_name: sha_short,
                        file_name_positions: None,
                        location_label: Some(author),
                        snippet: Some(subject),
                        first_line_snippet: None,
                        snippet_match_positions: None,
                        snippet_syntax_highlights: None,
                        blame: None,
                        kind: QuickMatchKind::GitCommit {
                            repo_workdir: commit.repo_workdir.clone(),
                            sha: commit.sha.clone(),
                            branch: commit.branch.clone(),
                            commit_timestamp: commit.commit_timestamp,
                        },
                    });
                }

                if !cancel_flag.load(std::sync::atomic::Ordering::Relaxed) {
                    crate::flush_batch(picker.clone(), generation, &mut batch, &mut app);
                    crate::finish_stream(picker, generation, &mut app);
                }
            }
        })
        .detach();
    }

    fn preview_request_for_match(
        &self,
        selected: &QuickMatch,
        _weak_ranges: Vec<Range<TextAnchor>>,
        _use_diff_preview: bool,
        query: &str,
    ) -> PreviewRequest {
        let key = PreviewKey(selected.id);
        match &selected.kind {
            QuickMatchKind::GitCommit {
                repo_workdir, sha, ..
            } => PreviewRequest::GitCommit {
                key,
                repo_workdir: repo_workdir.clone(),
                sha: sha.clone(),
                query: Arc::<str>::from(query.to_string()),
            },
            _ => PreviewRequest::Empty,
        }
    }

    fn confirm_outcome_for_match(&self, selected: &QuickMatch, _cx: &App) -> ConfirmOutcome {
        match &selected.kind {
            QuickMatchKind::GitCommit {
                repo_workdir, sha, ..
            } => ConfirmOutcome::OpenGitCommit {
                repo_workdir: repo_workdir.clone(),
                sha: sha.clone(),
            },
            _ => ConfirmOutcome::Dismiss,
        }
    }

    fn preview_panel_ui_for_match(
        &self,
        selected: &QuickMatch,
        project: &gpui::Entity<project::Project>,
        cx: &mut gpui::App,
    ) -> PreviewPanelUi {
        match &selected.kind {
            QuickMatchKind::GitCommit {
                repo_workdir,
                sha,
                commit_timestamp,
                ..
            } => PreviewPanelUi::GitCommit {
                meta: {
                    let remote = resolve_git_remote_for_workdir(repo_workdir, project, cx);
                    let github_url = remote.as_ref().map(|remote| {
                        Arc::<str>::from(format!(
                            "{}/{}/{}/commit/{}",
                            remote.host.base_url(),
                            remote.owner,
                            remote.repo,
                            sha,
                        ))
                    });
                    GitCommitPreviewMeta {
                        sha: sha.clone(),
                        subject: selected
                            .snippet
                            .clone()
                            .unwrap_or_else(|| Arc::<str>::from("")),
                        author: selected
                            .location_label
                            .clone()
                            .unwrap_or_else(|| Arc::<str>::from("")),
                        commit_timestamp: *commit_timestamp,
                        repo_label: selected.path_label.clone(),
                        remote,
                        github_url,
                    }
                },
            },
            _ => PreviewPanelUi::Standard {
                path_text: selected.display_path.clone(),
                highlights: selected
                    .display_path_positions
                    .as_deref()
                    .map(|positions| positions.to_vec())
                    .unwrap_or_default(),
            },
        }
    }
}
