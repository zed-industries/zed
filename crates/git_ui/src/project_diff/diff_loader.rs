//! Task which updates the project diff multibuffer without putting too much
//! pressure on the frontend executor. It prioritizes loading the area around the user

use collections::HashSet;
use db::smol::stream::StreamExt;
use futures::channel::mpsc;
use gpui::{AppContext, AsyncWindowContext, Entity, Task, WeakEntity};
use project::git_store::StatusEntry;
use ui::{App, Window};
use util::ResultExt;

use crate::{git_panel::GitStatusEntry, project_diff::ProjectDiff};

enum Update {
    Position(usize),
    NewFile(StatusEntry),
    ListChanged,
    // should not need to handle re-ordering (sorting) here.
    // something to handle scroll? or should that live in the project diff?
}

struct LoaderHandle {
    task: Task<Option<()>>,
    sender: mpsc::UnboundedSender<Update>,
}

impl LoaderHandle {
    pub fn update_file_list(&self) {
        let _ = self
            .sender
            .unbounded_send(Update::ListChanged)
            .log_err();

    }
    pub fn update_pos(&self, pos: usize) {
        let _ = self
            .sender
            .unbounded_send(Update::Position((pos)))
            .log_err();
    }
}

pub fn start_loader(project_diff: Entity<ProjectDiff>, window: &Window, cx: &App) -> LoaderHandle {
    let (tx, rx) = mpsc::unbounded();

    let task = window.spawn(cx, async move |cx| {
        load(rx, project_diff.downgrade(), cx).await
    });
    LoaderHandle { task, sender: tx }
}

enum DiffEntry {
    Loading(GitStatusEntry),
    Loaded(GitStatusEntry),
    Queued(GitStatusEntry),
}

impl DiffEntry {
    fn queued(&self) -> bool {
        matches!(self, DiffEntry::Queued(_))
    }
}

async fn load(
    rx: mpsc::UnboundedReceiver<Update>,
    project_diff: WeakEntity<ProjectDiff>,
    cx: &mut AsyncWindowContext,
) -> Option<()> {
    // let initial_entries = cx.read_entity(&cx.entity(), |project_diff, cx| project_diff.first_n_entries(cx, 100));
    // let loading = to_load.drain(..100).map(|| refresh_one)
    let mut existing = Vec::new();

    loop {
        let update = rx.next().await?;
        match update {
            Update::Position(pos) => {
                if existing.get(pos).is_some_and(|diff| diff.queued()) {
                    todo!("append to future unordered, also load in the bit
                    around (maybe with a short sleep ahead so we get some sense
                    of 'priority'")
                }
                // drop whatever is loading so we get to the new bit earlier
            }
            Update::NewFile(status_entry) => todo!(),
            Update::ListChanged => {
                let (added, removed) = project_diff
                    .upgrade()?
                    .read_with(cx, |diff, cx| diff_current_list(&existing, diff, cx))
                    .ok()?;
            }
        }

        // wait for Update OR Load done
        // -> Immediately spawn update
        // OR
        // -> spawn next group
    }
}

// could be new list
fn diff_current_list(
    existing_entries: &[GitStatusEntry],
    project_diff: &ProjectDiff,
    cx: &App,
) -> (Vec<(usize, GitStatusEntry)>, Vec<usize>) {
    let Some(new_entries) = project_diff.entries(cx) else {
        return (Vec::new(), Vec::new());
    };

    let existing_entries = existing_entries.iter().enumerate();
    for entry in new_entries {
        let Some((idx, existing)) = existing_entries.next() else {
            todo!();
        };

        if existing == entry {
        }



    }

    // let initial_entries = cx.read_entity(&cx.entity(), |project_diff, cx| project_diff.first_n_entries(cx, 100));
    // let loading = to_load.drain(..100).map(|| refresh_one)
}

// // remove anything not part of the diff in the multibuffer
// fn remove_anything_not_being_loaded() {
//     this.update(cx, |this, cx| {
//         multibuffer.update(cx, |multibuffer, cx| {
//             for path in previous_paths {
//                 this.buffer_diff_subscriptions.remove(&path.path);
//                 multibuffer.remove_excerpts_for_path(path, cx);
//             }
//         });
//     })?;
// }

pub async fn refresh_group(
    this: WeakEntity<ProjectDiff>,
    cached_status: Vec<StatusEntry>,
    cx: &mut AsyncWindowContext,
) -> anyhow::Result<()> {
    dbg!("refreshing all");
    use project::git_store::branch_diff::BranchDiff;
    let Some(this) = this.upgrade() else {
        return Ok(());
    };
    let multibuffer = cx.read_entity(&this, |this, _| this.multibuffer.clone())?;
    let branch_diff = cx.read_entity(&this, |pd, _| pd.branch_diff.clone())?;

    let Some(repo) = cx.read_entity(&branch_diff, |bd, _| bd.repo.clone())? else {
        return Ok(());
    };
    let project = cx.read_entity(&branch_diff, |bd, _| bd.project.clone())?;

    let mut previous_paths =
        cx.read_entity(&multibuffer, |mb, _| mb.paths().collect::<HashSet<_>>())?;

    // Idea: on click in git panel prioritize task for that file in some way ...
    //       could have a hashmap of futures here
    // - needs to prioritize *some* background tasks over others
    // -
    let mut tasks = FuturesUnordered::new();
    let mut seen = HashSet::default();
    for entry in cached_status {
        seen.insert(entry.repo_path.clone());
        let tree_diff_status = cx.read_entity(&branch_diff, |branch_diff, _| {
            branch_diff
                .tree_diff
                .as_ref()
                .and_then(|t| t.entries.get(&entry.repo_path))
                .cloned()
        })?;

        let Some(status) = cx.read_entity(&branch_diff, |bd, _| {
            bd.merge_statuses(Some(entry.status), tree_diff_status.as_ref())
        })?
        else {
            continue;
        };
        if !status.has_changes() {
            continue;
        }

        let Some(project_path) = cx.read_entity(&repo, |repo, cx| {
            repo.repo_path_to_project_path(&entry.repo_path, cx)
        })?
        else {
            continue;
        };

        let sort_prefix = cx.read_entity(&repo, |repo, cx| {
            sort_prefix(repo, &entry.repo_path, entry.status, cx)
        })?;

        let path_key = PathKey::with_sort_prefix(sort_prefix, entry.repo_path.into_arc());
        previous_paths.remove(&path_key);

        let repo = repo.clone();
        let project = project.downgrade();
        let task = cx.spawn(async move |cx| {
            let res = BranchDiff::load_buffer(
                tree_diff_status,
                project_path,
                repo,
                project,
                &mut cx.to_app(),
            )
            .await;
            (res, path_key, entry.status)
        });

        tasks.push(task)
    }

    // remove anything not part of the diff in the multibuffer
    this.update(cx, |this, cx| {
        multibuffer.update(cx, |multibuffer, cx| {
            for path in previous_paths {
                this.buffer_diff_subscriptions.remove(&path.path);
                multibuffer.remove_excerpts_for_path(path, cx);
            }
        });
    })?;

    // add the new buffers as they are parsed
    let mut last_notify = Instant::now();
    while let Some((res, path_key, file_status)) = tasks.next().await {
        if let Some((buffer, diff)) = res.log_err() {
            cx.update(|window, cx| {
                this.update(cx, |this, cx| {
                    this.register_buffer(path_key, file_status, buffer, diff, window, cx)
                });
            })?;
        }

        if last_notify.elapsed().as_millis() > 100 {
            cx.update_entity(&this, |_, cx| cx.notify())?;
            last_notify = Instant::now();
        }
    }

    Ok(())
}

pub(crate) fn sort_or_collapse_changed() {
    todo!()
}
