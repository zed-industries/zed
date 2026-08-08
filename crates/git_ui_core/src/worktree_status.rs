use anyhow::{Context as _, Result};
use askpass::AskPassDelegate;
use collections::HashMap;
use fs::Fs;
use git::{
    repository::{CommitOptions, DiffStatType, GitRepository, RepoPath},
    status::{DiffStat, FileStatus},
};
use gpui::{App, AppContext as _, Entity, SharedString, Task};
use project::ProjectEnvironment;
use std::{path::PathBuf, sync::Arc};
use task::Shell;
use util::rel_path::RelPath;

/// One uncommitted change in a git worktree.
///
/// Deliberately smaller than the git panel's status entry: this describes a
/// worktree that is not open in the project, so there is no `Repository`
/// entity behind it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorktreeStatusEntry {
    pub repo_path: RepoPath,
    pub status: FileStatus,
    /// Lines added and removed against the worktree's HEAD.
    ///
    /// `None` for untracked files, which `git diff` does not report on.
    pub diff_stat: Option<DiffStat>,
}

/// A git worktree that is not open in the project.
///
/// The project only creates `Repository` entities for folders it has open, so
/// the other worktrees have to be driven through the git backend directly.
/// This is that handle: it owns the backend and the shell environment resolved
/// for the worktree's directory, which git needs for hooks, credential helpers
/// and commit signing.
#[derive(Clone)]
pub struct WorktreeRepository {
    backend: Arc<dyn GitRepository>,
    environment: Arc<HashMap<String, String>>,
    worktree_abs_path: PathBuf,
}

impl WorktreeRepository {
    pub fn open(
        fs: Arc<dyn Fs>,
        project_environment: Entity<ProjectEnvironment>,
        worktree_abs_path: PathBuf,
        cx: &mut App,
    ) -> Task<Result<Self>> {
        let environment = project_environment.update(cx, |project_environment, cx| {
            project_environment.local_directory_environment(
                &Shell::System,
                worktree_abs_path.as_path().into(),
                cx,
            )
        });
        cx.background_spawn(async move {
            let environment = Arc::new(environment.await.unwrap_or_else(|| {
                log::error!("no directory environment for worktree {worktree_abs_path:?}");
                HashMap::default()
            }));
            let dot_git_abs_path = worktree_abs_path.join(".git");
            // Mirrors `LocalRepositoryState::new`: a development build has no
            // bundled git binary, so fall back to the one on `PATH`.
            let system_git_binary_path = which::which("git").ok();
            let backend = fs
                .open_repo(&dot_git_abs_path, system_git_binary_path.as_deref())
                .with_context(|| format!("opening git worktree at {dot_git_abs_path:?}"))?;
            Ok(Self {
                backend,
                environment,
                worktree_abs_path,
            })
        })
    }

    pub fn worktree_abs_path(&self) -> &PathBuf {
        &self.worktree_abs_path
    }

    /// The worktree's uncommitted changes, with line counts against its HEAD.
    pub fn status(&self, cx: &App) -> Task<Result<Vec<WorktreeStatusEntry>>> {
        let backend = self.backend.clone();
        // An empty prefix list matches nothing, so ask for the worktree root.
        let root = RepoPath::from_rel_path(RelPath::empty());
        let status = backend.status(std::slice::from_ref(&root));
        let diff_stat = backend.diff_stat(DiffStatType::HeadToWorktree, std::slice::from_ref(&root));
        cx.background_spawn(async move {
            let status = status.await?;
            // Line counts are a nicety; a failure here should not cost the
            // user the file list.
            let diff_stats: HashMap<RepoPath, DiffStat> = match diff_stat.await {
                Ok(diff_stat) => diff_stat.entries.iter().cloned().collect(),
                Err(error) => {
                    log::warn!("failed to load worktree diff stats: {error:#}");
                    HashMap::default()
                }
            };
            Ok(status
                .entries
                .iter()
                .filter(|(_, status)| status.has_changes())
                .map(|(repo_path, status)| WorktreeStatusEntry {
                    repo_path: repo_path.clone(),
                    status: *status,
                    diff_stat: diff_stats.get(repo_path).copied(),
                })
                .collect())
        })
    }

    pub fn stage(&self, paths: Vec<RepoPath>, cx: &App) -> Task<Result<()>> {
        let backend = self.backend.clone();
        let environment = self.environment.clone();
        cx.background_spawn(async move { backend.stage_paths(paths, environment).await })
    }

    pub fn unstage(&self, paths: Vec<RepoPath>, cx: &App) -> Task<Result<()>> {
        let backend = self.backend.clone();
        let environment = self.environment.clone();
        cx.background_spawn(async move { backend.unstage_paths(paths, environment).await })
    }

    /// Commits whatever is staged in the worktree.
    ///
    /// Author identity is left to git config, as `git commit` on the command
    /// line would.
    pub fn commit(
        &self,
        message: SharedString,
        askpass: AskPassDelegate,
        cx: &App,
    ) -> Task<Result<()>> {
        let backend = self.backend.clone();
        let environment = self.environment.clone();
        cx.background_spawn(async move {
            backend
                .commit(
                    message,
                    None,
                    CommitOptions {
                        amend: false,
                        signoff: false,
                        allow_empty: false,
                        no_verify: false,
                    },
                    askpass,
                    environment,
                )
                .await
        })
    }

    /// The file's contents in the worktree's HEAD, for use as a diff base.
    pub fn load_committed_text(&self, repo_path: RepoPath, cx: &App) -> Task<Option<String>> {
        let backend = self.backend.clone();
        cx.background_spawn(async move { backend.load_committed_text(repo_path).await })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use git::status::{StatusCode, TrackedStatus};
    use gpui::TestAppContext;
    use project::{Project, project_settings::ProjectSettings};
    use serde_json::json;
    use settings::{Settings as _, SettingsStore};
    use std::path::Path;
    use util::{path, rel_path::rel_path};
    use worktree::WorktreeSettings;

    fn repo_path(path: &str) -> RepoPath {
        RepoPath::from_rel_path(rel_path(path))
    }

    async fn open(
        fs: &Arc<FakeFs>,
        project: &Entity<Project>,
        worktree_abs_path: &str,
        cx: &mut TestAppContext,
    ) -> WorktreeRepository {
        let environment = project.read_with(cx, |project, _| project.environment().clone());
        cx.update(|cx| {
            WorktreeRepository::open(
                fs.clone(),
                environment,
                PathBuf::from(worktree_abs_path),
                cx,
            )
        })
        .await
        .unwrap()
    }

    async fn setup(cx: &mut TestAppContext) -> (Arc<FakeFs>, Entity<Project>) {
        zlog::init_test();
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            ProjectSettings::register(cx);
            WorktreeSettings::register(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "main.rs": "fn main() {}",
            }),
        )
        .await;
        fs.add_linked_worktree_for_repo(
            Path::new(path!("/project/.git")),
            false,
            git::repository::Worktree {
                path: PathBuf::from(path!("/worktrees/feature-a")),
                ref_name: Some("refs/heads/feature-a".into()),
                sha: "aaa".into(),
                is_main: false,
                is_bare: false,
            },
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        (fs, project)
    }

    #[gpui::test]
    async fn test_status_reports_changes_in_a_linked_worktree(cx: &mut TestAppContext) {
        let (fs, project) = setup(cx).await;
        fs.insert_file(
            path!("/worktrees/feature-a/main.rs"),
            b"fn main() { println!(\"hi\"); }".to_vec(),
        )
        .await;
        fs.insert_file(path!("/worktrees/feature-a/extra.rs"), b"// new".to_vec())
            .await;
        fs.set_head_and_index_for_repo(
            Path::new(path!("/worktrees/feature-a/.git")),
            &[("main.rs", "fn main() {}".into())],
        );

        let repository = open(&fs, &project, path!("/worktrees/feature-a"), cx).await;
        let entries = cx
            .update(|cx| repository.status(cx))
            .await
            .unwrap()
            .into_iter()
            .map(|entry| (entry.repo_path, entry.status))
            .collect::<Vec<_>>();

        assert_eq!(
            entries,
            vec![
                (repo_path("extra.rs"), FileStatus::Untracked),
                (
                    repo_path("main.rs"),
                    FileStatus::Tracked(TrackedStatus {
                        index_status: StatusCode::Unmodified,
                        worktree_status: StatusCode::Modified,
                    })
                ),
            ]
        );
    }

    #[gpui::test]
    async fn test_line_counts_are_reported_for_tracked_files_only(cx: &mut TestAppContext) {
        let (fs, project) = setup(cx).await;
        fs.insert_file(
            path!("/worktrees/feature-a/main.rs"),
            b"one\ntwo\nthree\n".to_vec(),
        )
        .await;
        fs.insert_file(path!("/worktrees/feature-a/extra.rs"), b"// new\n".to_vec())
            .await;
        fs.set_head_and_index_for_repo(
            Path::new(path!("/worktrees/feature-a/.git")),
            &[("main.rs", "one\n".into())],
        );

        let repository = open(&fs, &project, path!("/worktrees/feature-a"), cx).await;
        let entries = cx.update(|cx| repository.status(cx)).await.unwrap();
        let diff_stat_for = |path: &str| {
            entries
                .iter()
                .find(|entry| entry.repo_path == repo_path(path))
                .unwrap_or_else(|| panic!("expected {path} to be changed"))
                .diff_stat
        };

        // The exact counts come from git; the fake backend approximates a diff
        // as a whole-file replacement, so only assert that counts arrived.
        let main = diff_stat_for("main.rs").expect("a tracked change should carry line counts");
        assert!(main.added > 0);

        // `git diff` says nothing about files it has never seen.
        assert_eq!(
            diff_stat_for("extra.rs"),
            None,
            "untracked files have no line counts"
        );
    }

    #[gpui::test]
    async fn test_each_linked_worktree_reports_its_own_changes(cx: &mut TestAppContext) {
        let (fs, project) = setup(cx).await;
        fs.add_linked_worktree_for_repo(
            Path::new(path!("/project/.git")),
            false,
            git::repository::Worktree {
                path: PathBuf::from(path!("/worktrees/feature-b")),
                ref_name: Some("refs/heads/feature-b".into()),
                sha: "aaa".into(),
                is_main: false,
                is_bare: false,
            },
        )
        .await;

        fs.insert_file(path!("/worktrees/feature-a/only-in-a.rs"), b"// a".to_vec())
            .await;
        fs.insert_file(path!("/worktrees/feature-b/only-in-b.rs"), b"// b".to_vec())
            .await;
        fs.set_head_and_index_for_repo(Path::new(path!("/worktrees/feature-a/.git")), &[]);
        fs.set_head_and_index_for_repo(Path::new(path!("/worktrees/feature-b/.git")), &[]);

        for (name, expected) in [("feature-a", "only-in-a.rs"), ("feature-b", "only-in-b.rs")] {
            let repository = open(
                &fs,
                &project,
                &format!("{}/{name}", path!("/worktrees")),
                cx,
            )
            .await;
            let paths = cx
                .update(|cx| repository.status(cx))
                .await
                .unwrap()
                .into_iter()
                .map(|entry| entry.repo_path.as_unix_str().to_owned())
                .collect::<Vec<_>>();
            assert_eq!(paths, vec![expected.to_owned()]);
        }
    }

    #[gpui::test]
    async fn test_opening_a_missing_worktree_errors(cx: &mut TestAppContext) {
        let (fs, project) = setup(cx).await;
        let environment = project.read_with(cx, |project, _| project.environment().clone());

        let result = cx
            .update(|cx| {
                WorktreeRepository::open(
                    fs.clone(),
                    environment,
                    PathBuf::from(path!("/worktrees/never-existed")),
                    cx,
                )
            })
            .await;

        assert!(result.is_err());
    }

    #[gpui::test]
    async fn test_staging_and_unstaging_a_file(cx: &mut TestAppContext) {
        let (fs, project) = setup(cx).await;
        fs.insert_file(
            path!("/worktrees/feature-a/main.rs"),
            b"fn main() { changed(); }".to_vec(),
        )
        .await;
        fs.set_head_and_index_for_repo(
            Path::new(path!("/worktrees/feature-a/.git")),
            &[("main.rs", "fn main() {}".into())],
        );
        let repository = open(&fs, &project, path!("/worktrees/feature-a"), cx).await;

        let staging_of = async |repository: &WorktreeRepository, cx: &mut TestAppContext| {
            cx.update(|cx| repository.status(cx))
                .await
                .unwrap()
                .into_iter()
                .find(|entry| entry.repo_path == repo_path("main.rs"))
                .map(|entry| entry.status.staging())
        };

        assert_eq!(
            staging_of(&repository, cx).await,
            Some(git::status::StageStatus::Unstaged)
        );

        cx.update(|cx| repository.stage(vec![repo_path("main.rs")], cx))
            .await
            .unwrap();
        assert_eq!(
            staging_of(&repository, cx).await,
            Some(git::status::StageStatus::Staged)
        );

        cx.update(|cx| repository.unstage(vec![repo_path("main.rs")], cx))
            .await
            .unwrap();
        assert_eq!(
            staging_of(&repository, cx).await,
            Some(git::status::StageStatus::Unstaged)
        );
    }
}
