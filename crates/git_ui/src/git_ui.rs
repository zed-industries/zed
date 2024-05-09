use std::{
    path::{Path, PathBuf},
    time::{self, Instant, SystemTime},
};

use gpui::{AppContext, ViewContext};
use serde::Deserialize;
use workspace::Workspace;

#[derive(Deserialize)]
struct OpenRecentlyChanged {
    count: usize,
}

gpui::impl_actions!(git, [OpenRecentlyChanged]);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(
        |workspace: &mut Workspace, cx: &mut ViewContext<Workspace>| {
            workspace.register_action(open_recently_changed);
        },
    )
    .detach();
}

fn open_recently_changed(
    workspace: &mut Workspace,
    action: &OpenRecentlyChanged,
    cx: &mut ViewContext<Workspace>,
) {
    let worktrees = workspace.project().read(cx).worktrees().collect::<Vec<_>>();

    let mut repos = Vec::new();
    for worktree in worktrees {
        if let Some(local_worktree) = worktree.read(cx).as_local() {
            if let Some(local_repo) = local_worktree
                .repository_for_path(Path::new(""))
                .and_then(|repo| local_worktree.get_local_repo(&repo))
            {
                repos.push(local_repo.repo().clone());
            }
        }
    }

    if !repos.is_empty() {
        let recent_paths = cx.background_executor().spawn(async move {
            let mut paths = Vec::new();
            for repo in repos {
                paths.extend(repo.lock().recently_changed_paths(100))?;
            }
            anyhow::Ok(paths)
        });

        cx.spawn(|this, cx| async move {
            let recent_paths =



        })

    }
}
