use ::settings::Settings;
use git::repository::GitRepository;
use git::status::FileStatus;
use git_panel_settings::GitPanelSettings;
use gpui::{AppContext, Hsla, Model};
use project::{Project, WorktreeId};
use std::sync::Arc;
use ui::{Color, Icon, IconName, IntoElement};
use worktree::RepositoryEntry;

pub mod git_panel;
mod git_panel_settings;

pub fn init(cx: &mut AppContext) {
    GitPanelSettings::register(cx);
}

pub fn first_worktree_repository(
    project: &Model<Project>,
    worktree_id: WorktreeId,
    cx: &mut AppContext,
) -> Option<(RepositoryEntry, Arc<dyn GitRepository>)> {
    project
        .read(cx)
        .worktree_for_id(worktree_id, cx)
        .and_then(|worktree| {
            let snapshot = worktree.read(cx).snapshot();
            let repo = snapshot.repositories().iter().next()?.clone();
            let git_repo = worktree
                .read(cx)
                .as_local()?
                .get_local_repo(&repo)?
                .repo()
                .clone();
            Some((repo, git_repo))
        })
}

pub fn first_repository_in_project(
    project: &Model<Project>,
    cx: &mut AppContext,
) -> Option<(WorktreeId, RepositoryEntry, Arc<dyn GitRepository>)> {
    project.read(cx).worktrees(cx).next().and_then(|worktree| {
        let snapshot = worktree.read(cx).snapshot();
        let repo = snapshot.repositories().iter().next()?.clone();
        let git_repo = worktree
            .read(cx)
            .as_local()?
            .get_local_repo(&repo)?
            .repo()
            .clone();
        Some((snapshot.id(), repo, git_repo))
    })
}

const ADDED_COLOR: Hsla = Hsla {
    h: 142. / 360.,
    s: 0.68,
    l: 0.45,
    a: 1.0,
};
const MODIFIED_COLOR: Hsla = Hsla {
    h: 48. / 360.,
    s: 0.76,
    l: 0.47,
    a: 1.0,
};
const REMOVED_COLOR: Hsla = Hsla {
    h: 355. / 360.,
    s: 0.65,
    l: 0.65,
    a: 1.0,
};

// TODO: Add updated status colors to theme
pub fn git_status_icon(status: FileStatus) -> impl IntoElement {
    let (icon_name, color) = if status.is_conflicted() {
        (IconName::Warning, REMOVED_COLOR)
    } else if status.is_deleted() {
        (IconName::SquareMinus, REMOVED_COLOR)
    } else if status.is_modified() {
        (IconName::SquareDot, MODIFIED_COLOR)
    } else {
        (IconName::SquarePlus, ADDED_COLOR)
    };
    Icon::new(icon_name).color(Color::Custom(color))
}
