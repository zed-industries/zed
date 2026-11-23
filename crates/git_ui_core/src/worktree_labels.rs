use std::path::{Path, PathBuf};

use gpui::{App, Entity, SharedString};
use ui::prelude::*;
use util::paths::PathStyle;
use workspace::Workspace;

#[derive(Clone)]
pub struct WorkspaceMenuWorktreeLabel {
    pub icon: Option<IconName>,
    pub primary_name: SharedString,
    pub secondary_name: Option<SharedString>,
}

impl WorkspaceMenuWorktreeLabel {
    pub fn text(&self) -> String {
        match &self.secondary_name {
            Some(secondary_name) => format!("{}/{secondary_name}", self.primary_name),
            None => self.primary_name.to_string(),
        }
    }

    pub fn render(&self) -> impl IntoElement {
        h_flex()
            .min_w_0()
            .gap_0p5()
            .when_some(self.icon, |this, icon| {
                this.child(Icon::new(icon).size(IconSize::XSmall).color(Color::Muted))
            })
            .child(Label::new(self.primary_name.clone()).truncate())
            .when_some(self.secondary_name.clone(), |this, secondary_name| {
                this.child(Label::new("/").alpha(0.5))
                    .child(Label::new(secondary_name).truncate())
            })
    }
}

pub fn workspace_menu_worktree_labels(
    workspace: &Entity<Workspace>,
    cx: &App,
) -> Vec<WorkspaceMenuWorktreeLabel> {
    worktree_menu_labels(root_worktree_names(workspace, cx))
}

fn worktree_menu_labels(
    root_worktree_names: Vec<RootWorktreeName>,
) -> Vec<WorkspaceMenuWorktreeLabel> {
    let show_folder_name = root_worktree_names.len() > 1;
    root_worktree_names
        .into_iter()
        .map(|root| match root.worktree_name {
            Some(worktree_name) if show_folder_name || root.in_checkout_subfolder => {
                WorkspaceMenuWorktreeLabel {
                    icon: Some(IconName::GitWorktree),
                    primary_name: root.folder_name,
                    secondary_name: Some(worktree_name),
                }
            }
            Some(worktree_name) => WorkspaceMenuWorktreeLabel {
                icon: Some(IconName::GitWorktree),
                primary_name: worktree_name,
                secondary_name: None,
            },
            None => WorkspaceMenuWorktreeLabel {
                icon: None,
                primary_name: root.folder_name,
                secondary_name: None,
            },
        })
        .collect()
}

struct RootWorktreeName {
    folder_name: SharedString,
    worktree_name: Option<SharedString>,
    in_checkout_subfolder: bool,
}

struct RootCheckout {
    path: PathBuf,
    common_dir: PathBuf,
    is_linked_worktree: bool,
    is_delta_checkout: bool,
}

fn root_worktree_names(workspace: &Entity<Workspace>, cx: &App) -> Vec<RootWorktreeName> {
    let root_paths = workspace.read(cx).root_paths(cx);
    let project = workspace.read(cx).project().clone();
    let (path_style, repository_snapshots, worktree_paths, git_worktree_paths) = {
        let project = project.read(cx);
        let path_style = project.path_style(cx);
        let repository_snapshots = project
            .repositories(cx)
            .values()
            .map(|repo| repo.read(cx).snapshot())
            .collect::<Vec<_>>();
        (
            path_style,
            repository_snapshots,
            project.worktree_paths(cx),
            project.git_worktree_paths(cx),
        )
    };

    root_paths
        .into_iter()
        .map(|root_path| {
            let is_delta_checkout = worktree_paths
                .ordered_pairs()
                .zip(git_worktree_paths.ordered_pairs())
                .any(|((main_path, folder_path), (git_main_path, _))| {
                    folder_path.as_path() == root_path.as_ref() && main_path != git_main_path
                });
            let checkout = repository_snapshots
                .iter()
                .filter(|snapshot| {
                    path_style.starts_with(&root_path, &snapshot.work_directory_abs_path)
                })
                .max_by_key(|snapshot| snapshot.work_directory_abs_path.as_os_str().len())
                .map(|snapshot| RootCheckout {
                    path: snapshot.work_directory_abs_path.to_path_buf(),
                    common_dir: snapshot.common_dir_abs_path.to_path_buf(),
                    is_linked_worktree: snapshot.is_linked_worktree(),
                    is_delta_checkout,
                });
            root_worktree_name(&root_path, checkout.as_ref(), path_style)
        })
        .collect()
}

fn root_worktree_name(
    root_path: &Path,
    checkout: Option<&RootCheckout>,
    path_style: PathStyle,
) -> RootWorktreeName {
    let folder_name = path_style
        .file_name(root_path)
        .map(|name| SharedString::from(name.to_string_lossy().into_owned()))
        .unwrap_or_default();
    let Some(checkout) =
        checkout.filter(|checkout| checkout.path == root_path || checkout.is_delta_checkout)
    else {
        return RootWorktreeName {
            folder_name,
            worktree_name: None,
            in_checkout_subfolder: false,
        };
    };
    let checkout_name = path_style
        .file_name(&checkout.path)
        .map(|name| SharedString::from(name.to_string_lossy().into_owned()))
        .unwrap_or_else(|| folder_name.clone());
    let worktree_name = if checkout.is_delta_checkout {
        project::delta_checkout_short_name(
            &project::DeltaClone {
                path: checkout.common_dir.clone(),
                is_linked_worktree: checkout.is_linked_worktree,
                checkout_path: Some(checkout.path.clone()),
            },
            &checkout.path,
            path_style,
        )
        .unwrap_or(checkout_name)
    } else if checkout.is_linked_worktree {
        project::linked_worktree_short_name(
            project::repo_identity_path(&checkout.common_dir, path_style),
            &checkout.path,
            path_style,
        )
        .unwrap_or(checkout_name)
    } else {
        "main".into()
    };
    RootWorktreeName {
        in_checkout_subfolder: checkout.path != root_path,
        folder_name,
        worktree_name: Some(worktree_name),
    }
}
