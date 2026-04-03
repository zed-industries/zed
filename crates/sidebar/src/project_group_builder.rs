//! Maps worktree paths to their canonical (main) git repository paths.
//!
//! Threads have a path list associated with them, but this is the absolute path
//! of whatever worktrees they were associated with. In the sidebar, we want to
//! group all threads by their main worktree, and then we add a worktree chip to
//! the sidebar entry when that thread is in another worktree.
//!
//! This module provides the canonicalization mapping needed to resolve linked
//! worktree paths back to their main repository path.

use collections::HashMap;
use gpui::{App, Entity};
use std::path::{Path, PathBuf};
use workspace::Workspace;

/// Maps git worktree paths to their main repository path.
///
/// This is used to determine whether a thread's `folder_paths` entry is a
/// linked worktree (canonical != original) so we can show worktree chips
/// in the sidebar.
pub struct WorktreeCanonicalizer {
    /// Maps git repositories' work_directory_abs_path to their original_repo_abs_path.
    directory_mappings: HashMap<PathBuf, PathBuf>,
}

impl WorktreeCanonicalizer {
    pub fn new() -> Self {
        Self {
            directory_mappings: HashMap::default(),
        }
    }

    /// Builds a canonicalizer from all workspaces, collecting directory
    /// mappings from their git repositories.
    pub fn from_workspaces(workspaces: &[Entity<Workspace>], cx: &App) -> Self {
        let mut canonicalizer = Self::new();
        for workspace in workspaces {
            canonicalizer.add_workspace_mappings(workspace.read(cx), cx);
        }
        canonicalizer
    }

    fn add_mapping(&mut self, work_directory: &Path, original_repo: &Path) {
        let old = self
            .directory_mappings
            .insert(PathBuf::from(work_directory), PathBuf::from(original_repo));
        if let Some(old) = old {
            debug_assert_eq!(
                &old, original_repo,
                "all worktrees should map to the same main worktree"
            );
        }
    }

    pub fn add_workspace_mappings(&mut self, workspace: &Workspace, cx: &App) {
        for repo in workspace.project().read(cx).repositories(cx).values() {
            let snapshot = repo.read(cx).snapshot();

            self.add_mapping(
                &snapshot.work_directory_abs_path,
                &snapshot.original_repo_abs_path,
            );

            for worktree in snapshot.linked_worktrees.iter() {
                self.add_mapping(&worktree.path, &snapshot.original_repo_abs_path);
            }
        }
    }

    /// Returns the canonical (main repository) path for the given path.
    /// If the path is not a known worktree, returns the path unchanged.
    pub fn canonicalize_path<'a>(&'a self, path: &'a Path) -> &'a Path {
        self.directory_mappings
            .get(path)
            .map(AsRef::as_ref)
            .unwrap_or(path)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use fs::FakeFs;
    use gpui::TestAppContext;
    use settings::SettingsStore;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
        });
    }

    async fn create_fs_with_main_and_worktree(cx: &mut TestAppContext) -> Arc<FakeFs> {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            serde_json::json!({
                ".git": {
                    "worktrees": {
                        "feature-a": {
                            "commondir": "../../",
                            "HEAD": "ref: refs/heads/feature-a",
                        },
                    },
                },
                "src": {},
            }),
        )
        .await;
        fs.insert_tree(
            "/wt/feature-a",
            serde_json::json!({
                ".git": "gitdir: /project/.git/worktrees/feature-a",
                "src": {},
            }),
        )
        .await;
        fs.add_linked_worktree_for_repo(
            std::path::Path::new("/project/.git"),
            false,
            git::repository::Worktree {
                path: std::path::PathBuf::from("/wt/feature-a"),
                ref_name: Some("refs/heads/feature-a".into()),
                sha: "abc".into(),
                is_main: false,
            },
        )
        .await;
        fs
    }

    #[gpui::test]
    async fn test_main_repo_maps_to_itself(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = create_fs_with_main_and_worktree(cx).await;
        cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

        let project = project::Project::test(fs.clone(), ["/project".as_ref()], cx).await;
        project
            .update(cx, |project, cx| project.git_scans_complete(cx))
            .await;

        let (multi_workspace, cx) = cx.add_window_view(|window, cx| {
            workspace::MultiWorkspace::test_new(project.clone(), window, cx)
        });

        multi_workspace.read_with(cx, |mw, cx| {
            let canonicalizer = WorktreeCanonicalizer::from_workspaces(mw.workspaces(), cx);

            // The main repo path should canonicalize to itself.
            assert_eq!(
                canonicalizer.canonicalize_path(Path::new("/project")),
                Path::new("/project"),
            );

            // An unknown path returns itself.
            assert_eq!(
                canonicalizer.canonicalize_path(Path::new("/something/else")),
                Path::new("/something/else"),
            );
        });
    }

    #[gpui::test]
    async fn test_worktree_checkout_canonicalizes_to_main_repo(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = create_fs_with_main_and_worktree(cx).await;
        cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

        let project = project::Project::test(fs.clone(), ["/wt/feature-a".as_ref()], cx).await;
        project
            .update(cx, |project, cx| project.git_scans_complete(cx))
            .await;

        let (multi_workspace, cx) = cx.add_window_view(|window, cx| {
            workspace::MultiWorkspace::test_new(project.clone(), window, cx)
        });

        multi_workspace.read_with(cx, |mw, cx| {
            let canonicalizer = WorktreeCanonicalizer::from_workspaces(mw.workspaces(), cx);

            // The worktree checkout path should canonicalize to the main repo.
            assert_eq!(
                canonicalizer.canonicalize_path(Path::new("/wt/feature-a")),
                Path::new("/project"),
            );
        });
    }
}
