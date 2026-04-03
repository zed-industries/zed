//! The sidebar groups threads by a canonical path list.
//!
//! Threads have a path list associated with them, but this is the absolute path
//! of whatever worktrees they were associated with. In the sidebar, we want to
//! group all threads by their main worktree, and then we add a worktree chip to
//! the sidebar entry when that thread is in another worktree.
//!
//! This module is provides the functions and structures necessary to do this
//! lookup and mapping.

use collections::{HashMap, HashSet, vecmap::VecMap};
use gpui::{App, Entity};
use project::ProjectGroupKey;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use workspace::{MultiWorkspace, PathList, Workspace};

#[derive(Default)]
pub struct ProjectGroup {
    pub workspaces: Vec<Entity<Workspace>>,
    /// Root paths of all open workspaces in this group. Used to skip
    /// redundant thread-store queries for linked worktrees that already
    /// have an open workspace.
    covered_paths: HashSet<Arc<Path>>,
}

impl ProjectGroup {
    fn add_workspace(&mut self, workspace: &Entity<Workspace>, cx: &App) {
        if !self.workspaces.contains(workspace) {
            self.workspaces.push(workspace.clone());
        }
        for path in workspace.read(cx).root_paths(cx) {
            self.covered_paths.insert(path);
        }
    }

    pub fn first_workspace(&self) -> &Entity<Workspace> {
        self.workspaces
            .first()
            .expect("groups always have at least one workspace")
    }

    pub fn main_workspace(&self, cx: &App) -> &Entity<Workspace> {
        self.workspaces
            .iter()
            .find(|ws| {
                !crate::root_repository_snapshots(ws, cx)
                    .any(|snapshot| snapshot.is_linked_worktree())
            })
            .unwrap_or_else(|| self.first_workspace())
    }
}

pub struct ProjectGroupBuilder {
    /// Maps git repositories' work_directory_abs_path to their original_repo_abs_path
    directory_mappings: HashMap<PathBuf, PathBuf>,
    project_groups: VecMap<ProjectGroupKey, ProjectGroup>,
}

impl ProjectGroupBuilder {
    fn new() -> Self {
        Self {
            directory_mappings: HashMap::default(),
            project_groups: VecMap::new(),
        }
    }

    pub fn from_multiworkspace(mw: &MultiWorkspace, cx: &App) -> Self {
        let mut builder = Self::new();
        // First pass: collect all directory mappings from every workspace
        // so we know how to canonicalize any path (including linked
        // worktree paths discovered by the main repo's workspace).
        for workspace in mw.workspaces() {
            builder.add_workspace_mappings(workspace.read(cx), cx);
        }

        // Second pass: group each workspace using canonical paths derived
        // from the full set of mappings.
        for workspace in mw.workspaces() {
            let group_name = workspace.read(cx).project_group_key(cx);
            builder
                .project_group_entry(&group_name)
                .add_workspace(workspace, cx);
        }
        builder
    }

    fn project_group_entry(&mut self, name: &ProjectGroupKey) -> &mut ProjectGroup {
        self.project_groups.entry_ref(name).or_insert_default()
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

    pub fn canonicalize_path<'a>(&'a self, path: &'a Path) -> &'a Path {
        self.directory_mappings
            .get(path)
            .map(AsRef::as_ref)
            .unwrap_or(path)
    }

    /// Whether the given group should load threads for a linked worktree
    /// at `worktree_path`. Returns `false` if the worktree already has an
    /// open workspace in the group (its threads are loaded via the
    /// workspace loop) or if the worktree's canonical path list doesn't
    /// match `group_path_list`.
    pub fn group_owns_worktree(
        &self,
        group: &ProjectGroup,
        group_path_list: &PathList,
        worktree_path: &Path,
    ) -> bool {
        if group.covered_paths.contains(worktree_path) {
            return false;
        }
        let canonical = self.canonicalize_path_list(&PathList::new(&[worktree_path]));
        canonical == *group_path_list
    }

    /// Canonicalizes every path in a [`PathList`] using the builder's
    /// directory mappings.
    fn canonicalize_path_list(&self, path_list: &PathList) -> PathList {
        let paths: Vec<_> = path_list
            .paths()
            .iter()
            .map(|p| self.canonicalize_path(p).to_path_buf())
            .collect();
        PathList::new(&paths)
    }

    pub fn groups(&self) -> impl Iterator<Item = (&ProjectGroupKey, &ProjectGroup)> {
        self.project_groups.iter()
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
            let mut canonicalizer = ProjectGroupBuilder::new();
            for workspace in mw.workspaces() {
                canonicalizer.add_workspace_mappings(workspace.read(cx), cx);
            }

            // The main repo path should canonicalize to itself.
            assert_eq!(
                canonicalizer.canonicalize_path(Path::new("/project")),
                Path::new("/project"),
            );

            // An unknown path returns None.
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

        // Open the worktree checkout as its own project.
        let project = project::Project::test(fs.clone(), ["/wt/feature-a".as_ref()], cx).await;
        project
            .update(cx, |project, cx| project.git_scans_complete(cx))
            .await;

        let (multi_workspace, cx) = cx.add_window_view(|window, cx| {
            workspace::MultiWorkspace::test_new(project.clone(), window, cx)
        });

        multi_workspace.read_with(cx, |mw, cx| {
            let mut canonicalizer = ProjectGroupBuilder::new();
            for workspace in mw.workspaces() {
                canonicalizer.add_workspace_mappings(workspace.read(cx), cx);
            }

            // The worktree checkout path should canonicalize to the main repo.
            assert_eq!(
                canonicalizer.canonicalize_path(Path::new("/wt/feature-a")),
                Path::new("/project"),
            );
        });
    }
}
