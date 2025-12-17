use std::{
    sync::{Arc, OnceLock},
};

use gpui::AppContext;
use search::SearchOptions;
use ui::IconName;

use crate::types::QuickMatchKind;
use crate::types::QuickMatchBuilder;
use anyhow::{Context as AnyhowContext, Result};
use fuzzy::StringMatchCandidate;
use git2::Sort;
use log::debug;

use crate::core::{
    ListPresentation, MatchBatcher, QuickSearchSource, SearchContext, SearchSink, SearchUiContext,
    SortPolicy, SourceId, SourceSpec, SourceSpecCore, SourceSpecUi,
};

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

    let branch: Option<Arc<str>> = match repo.head() {
        Ok(head) => head
            .shorthand()
            .map(|s| s.to_string())
            .and_then(|name| {
                let name = name.trim();
                (!name.is_empty() && name != "HEAD").then(|| Arc::<str>::from(name.to_string()))
            }),
        Err(err) => {
            debug!("quick_search: failed to read git HEAD: {:?}", err);
            None
        }
    };

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
            core: SourceSpecCore {
                supported_options: SearchOptions::empty(),
                min_query_len: 1,
                sort_policy: SortPolicy::StreamOrder,
            },
            ui: SourceSpecUi {
                title: Arc::from("Commits"),
                icon: IconName::GitBranchAlt,
                placeholder: Arc::from("Search commits..."),
                list_presentation: ListPresentation::Flat,
                use_diff_preview: true,
            },
        })
    }
}

impl QuickSearchSource for CommitsSource {
    fn spec(&self) -> &'static SourceSpec {
        Self::spec_static()
    }

    fn start_search(
        &self,
        ctx: SearchContext,
        sink: SearchSink,
        cx: &mut SearchUiContext<'_>,
    ) {
        let repos = ctx
            .project()
            .read(cx)
            .git_store()
            .read(cx)
            .repositories()
            .values()
            .cloned()
            .collect::<Vec<_>>();

        let query = ctx.query().as_ref().to_string();
        let repos = repos
            .into_iter()
            .map(|repo| {
                let repo_workdir = repo.read(cx).work_directory_abs_path.clone();
                (repo_workdir, repo)
            })
            .collect::<Vec<_>>();

        if repos.is_empty() {
            let message = "No Git repositories found in this project.".to_string();
            cx.spawn(move |_, app: &mut gpui::AsyncApp| {
                let mut app = app.clone();
                async move {
                    sink.record_error(message, &mut app);
                }
            })
            .detach();
            return;
        }

        let executor = ctx.background_executor().clone();
        let source_id = self.spec().id.0.clone();
        let cancel_flag = ctx.cancel_flag();
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
                                .unwrap_or_else(|err| {
                                    debug!(
                                        "quick_search: failed to read head commit from git store: {:?}",
                                        err
                                    );
                                    None
                                });
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

                let notice = used_fallback.then_some(
                    "Some repositories are remote; showing branch-tip commits (full history unavailable)."
                        .to_string(),
                );
                sink.set_query_notice(notice, &mut app);

                if commits.is_empty() {
                    sink.record_error("No commits found.".to_string(), &mut app);
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

                if cancel_flag.load(std::sync::atomic::Ordering::Relaxed) || sink.is_cancelled() {
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

                let mut batcher = MatchBatcher::new();
                for m in matches {
                    let Some(commit) = commits.get(m.candidate_id) else {
                        continue;
                    };

                    let sha_short: Arc<str> =
                        Arc::from(commit.sha.get(..8).unwrap_or(&commit.sha).to_string());
                    let subject = commit.subject.clone();
                    let author = commit.author_name.clone();

                    batcher.push(
                        QuickMatchBuilder::new(
                            source_id.clone(),
                            QuickMatchKind::GitCommit {
                            repo_workdir: commit.repo_workdir.clone(),
                            sha: commit.sha.clone(),
                            branch: commit.branch.clone(),
                            commit_timestamp: commit.commit_timestamp,
                        },
                        )
                        .file_name(sha_short)
                        .location_label(Some(author))
                        .snippet(Some(subject))
                        .first_line_snippet(None)
                        .build(),
                        &sink,
                        &mut app,
                    );
                }

                if !cancel_flag.load(std::sync::atomic::Ordering::Relaxed) && !sink.is_cancelled() {
                    batcher.finish(&sink, &mut app);
                }
            }
        })
        .detach();
    }

}
