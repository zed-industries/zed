use crate::{
    diff_multibuffer::{DiffMultibuffer, project_diff_path_key},
    review_state::{Fingerprint, ReviewScope, ReviewState, digest},
};
use anyhow::{Context as _, Result, anyhow};
use buffer_diff::BufferDiff;
use collections::HashSet;
use file_icons::FileIcons;
use futures::StreamExt as _;
use git::{repository::RepoPath, status::FileStatus};
use gpui::{
    App, AppContext as _, Context, Entity, Render, Subscription, Task, WeakEntity, uniform_list,
};
use language::{Buffer, BufferEvent};
use project::{
    Project,
    git_store::diff_buffer_list::{DiffBase, DiffBufferList},
};
use std::{collections::BTreeMap, path::PathBuf, time::Duration};
use ui::{Checkbox, ListItem, Tooltip, prelude::*};
use util::ResultExt as _;

struct ReviewEntry {
    status: FileStatus,
    buffer: Option<Entity<Buffer>>,
    diff: Option<Entity<BufferDiff>>,
    fingerprint: Option<Fingerprint>,
    validated_snapshot: Option<language::BufferSnapshot>,
    validated_base: Option<language::BufferSnapshot>,
    error: Option<String>,
    hash_generation: u64,
    changed: bool,
    _subscriptions: Vec<Subscription>,
    hash_task: Option<Task<()>>,
}

#[derive(Clone)]
enum Row {
    Folder {
        path: String,
        name: String,
        depth: usize,
        viewed: usize,
        total: usize,
    },
    File {
        path: RepoPath,
        depth: usize,
    },
}

#[derive(Default)]
struct Folder {
    folders: BTreeMap<String, Folder>,
    files: Vec<RepoPath>,
    viewed: usize,
    total: usize,
}

pub(crate) struct BranchReview {
    project: Entity<Project>,
    diff: WeakEntity<DiffMultibuffer>,
    list: Entity<DiffBufferList>,
    entries: BTreeMap<RepoPath, ReviewEntry>,
    rows: Vec<Row>,
    collapsed: HashSet<String>,
    selected: Option<RepoPath>,
    state: Option<Entity<ReviewState>>,
    scope: Option<ReviewScope>,
    state_subscription: Option<Subscription>,
    generation: u64,
    loading: bool,
    error: Option<String>,
    viewed: usize,
    total: usize,
    refresh_task: Option<Task<()>>,
    watched_root: Option<PathBuf>,
    watch_task: Option<Task<()>>,
    _subscription: Subscription,
}

impl BranchReview {
    pub fn new(
        project: Entity<Project>,
        diff: Entity<DiffMultibuffer>,
        cx: &mut Context<Self>,
    ) -> Self {
        let list = diff.read(cx).branch_diff().clone();
        let subscription = Subscription::join(
            cx.subscribe(&list, |this, _, _, cx| this.refresh(cx)),
            cx.observe(&list, |_, _, cx| cx.notify()),
        );
        let mut this = Self {
            project,
            diff: diff.downgrade(),
            list,
            entries: BTreeMap::new(),
            rows: Vec::new(),
            collapsed: HashSet::default(),
            selected: None,
            state: None,
            scope: None,
            state_subscription: None,
            generation: 0,
            loading: true,
            error: None,
            viewed: 0,
            total: 0,
            refresh_task: None,
            watched_root: None,
            watch_task: None,
            _subscription: subscription,
        };
        this.refresh(cx);
        this
    }

    fn refresh(&mut self, cx: &mut Context<Self>) {
        self.generation += 1;
        let generation = self.generation;
        self.loading = true;
        self.error = None;
        if !self.scope_matches(cx) {
            self.state = None;
            self.scope = None;
            self.state_subscription = None;
        }
        if !self.project.read(cx).is_local() {
            self.error = Some("Branch Review currently supports local repositories".into());
            self.loading = false;
            cx.notify();
            return;
        }
        let Some(repository) = self.list.read(cx).repo().cloned() else {
            self.error = Some("No repository selected".into());
            self.loading = false;
            cx.notify();
            return;
        };
        let DiffBase::Merge { base_ref } = self.list.read(cx).diff_base().clone() else {
            return;
        };
        let snapshot = repository.read(cx).snapshot();
        let fs = self.project.read(cx).fs().clone();
        let root = snapshot.work_directory_abs_path.to_path_buf();
        if self.watched_root.as_ref() != Some(&root) {
            self.watched_root = Some(root.clone());
            let fs = fs.clone();
            self.watch_task = Some(cx.spawn(async move |this, cx| {
                let (mut events, _watcher) = fs.watch(&root, Duration::from_millis(100)).await;
                while let Some(events) = events.next().await {
                    if this
                        .update(cx, |this, cx| {
                            // Permissions can change without a buffer edit or a Git
                            // status change (e.g. M remains M). Validate only affected
                            // review entries, never rescan the repository here.
                            let affected: Vec<_> = this
                                .entries
                                .keys()
                                .filter(|path| {
                                    let absolute = root.join(path.as_std_path());
                                    events.iter().any(|event| absolute.starts_with(&event.path))
                                })
                                .cloned()
                                .collect();
                            for path in affected {
                                if let Some(entry) = this.entries.get_mut(&path) {
                                    entry.validated_snapshot = None;
                                }
                                this.reconcile(&path, cx);
                            }
                        })
                        .is_err()
                    {
                        break;
                    }
                }
            }));
        }
        let buffers = self.list.update(cx, |list, cx| list.load_buffers(cx));
        let paths: HashSet<_> = buffers
            .iter()
            .map(|buffer| buffer.repo_path.clone())
            .collect();
        self.entries.retain(|path, _| paths.contains(path));
        for buffer in &buffers {
            self.entries
                .entry(buffer.repo_path.clone())
                .or_insert_with(|| ReviewEntry {
                    status: buffer.file_status,
                    buffer: None,
                    diff: None,
                    fingerprint: None,
                    validated_snapshot: None,
                    validated_base: None,
                    error: None,
                    hash_generation: 0,
                    changed: true,
                    _subscriptions: Vec::new(),
                    hash_task: None,
                });
        }
        self.rebuild_rows(cx);
        self.refresh_task = Some(cx.spawn(async move |this, cx| {
            let result: Result<()> = async {
                let branch = snapshot
                    .branch
                    .as_ref()
                    .context("Check out a branch to save review progress")?;
                let reflog_path = snapshot
                    .common_dir_abs_path
                    .join("logs")
                    .join(branch.ref_name.as_ref());
                let reflog = fs.load(&reflog_path).await.context(
                    "Branch Review needs the branch reflog to distinguish recreated branches",
                )?;
                let birth = reflog
                    .lines()
                    .next()
                    .context("The branch reflog is empty")?;
                let scope = ReviewScope {
                    repository: snapshot.common_dir_abs_path.to_path_buf(),
                    worktree: snapshot.work_directory_abs_path.to_path_buf(),
                    branch: branch.ref_name.to_string(),
                    branch_generation: digest(&[birth.as_bytes()]),
                    base_ref: base_ref.to_string(),
                };
                this.update(cx, |this, cx| -> Result<()> {
                    if this.generation != generation {
                        return Ok(());
                    }
                    let state = ReviewState::for_scope(&scope, cx)?;
                    this.state_subscription =
                        Some(cx.observe(&state, |this, _, cx| this.rebuild_rows(cx)));
                    this.state = Some(state);
                    this.scope = Some(scope);
                    Ok(())
                })??;
                for buffer in buffers {
                    let loaded = buffer.load.await;
                    let path = buffer.repo_path;
                    this.update(cx, |this, cx| {
                        if this.generation != generation {
                            return;
                        }
                        match loaded {
                            Ok(loaded) => {
                                let subscription = cx.subscribe(&loaded.main_buffer, {
                                    let path = path.clone();
                                    move |this, _, event, cx| {
                                        if matches!(
                                            event,
                                            BufferEvent::Edited { .. }
                                                | BufferEvent::FileHandleChanged
                                                | BufferEvent::Reloaded
                                                | BufferEvent::Saved
                                        ) {
                                            this.reconcile(&path, cx);
                                        }
                                    }
                                });
                                let diff_subscription = cx.subscribe(&loaded.diff, {
                                    let path = path.clone();
                                    move |this, _, _, cx| this.reconcile(&path, cx)
                                });
                                if let Some(entry) = this.entries.get_mut(&path) {
                                    entry.status = buffer.file_status;
                                    entry.buffer = Some(loaded.main_buffer);
                                    entry.diff = Some(loaded.diff);
                                    entry._subscriptions = vec![subscription, diff_subscription];
                                    entry.error = None;
                                }
                                this.reconcile(&path, cx);
                            }
                            Err(error) => {
                                if let Some(entry) = this.entries.get_mut(&path) {
                                    entry.fingerprint = None;
                                    entry.error =
                                        Some(format!("Cannot review this entry: {error:#}"));
                                }
                            }
                        }
                    })?;
                }
                Ok(())
            }
            .await;
            this.update(cx, |this, cx| {
                if this.generation != generation {
                    return;
                }
                this.loading = false;
                this.error = result.err().map(|error| format!("{error:#}"));
                this.rebuild_rows(cx);
            })
            .log_err();
        }));
        cx.notify();
    }

    fn reconcile(&mut self, path: &RepoPath, cx: &mut Context<Self>) {
        let Some(entry) = self.entries.get_mut(path) else {
            return;
        };
        let (Some(buffer), Some(diff)) = (&entry.buffer, &entry.diff) else {
            return;
        };
        let snapshot = buffer.read(cx).snapshot();
        let base = diff.read(cx).base_text(cx);
        let base_exists = diff.read(cx).base_text_exists();
        let base_mode = self.list.read(cx).base_mode_for_path(path);
        let renamed_from = self
            .list
            .read(cx)
            .renamed_from(path)
            .map(|path| path.to_string());
        let Some(repository) = self.list.read(cx).repo() else {
            return;
        };
        let abs_path = repository
            .read(cx)
            .work_directory_abs_path
            .join(path.as_std_path());
        let fs = self.project.read(cx).fs().clone();
        let generation = self.generation;
        entry.hash_generation += 1;
        let hash_generation = entry.hash_generation;

        if !buffer.read(cx).is_dirty()
            && buffer.read(cx).file().is_some_and(|file| {
                file.disk_state().mtime().is_some()
                    && file.disk_state().mtime() != buffer.read(cx).saved_mtime()
            })
        {
            self.rebuild_rows(cx);
            return;
        }
        let expected_snapshot = snapshot.clone();
        let expected_base = base.clone();
        let path = path.clone();
        entry.hash_task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(50))
                .await;
            let result = async {
                let metadata = fs.metadata(&abs_path).await?;
                if metadata.is_some_and(|metadata| {
                    metadata.is_symlink || metadata.is_dir || metadata.is_fifo
                }) {
                    return Err(anyhow!(
                        "Symlinks, submodules, and special files are not yet reviewable"
                    ));
                }
                if base_mode.is_some_and(|mode| mode != 0 && mode != 0o100644 && mode != 0o100755) {
                    return Err(anyhow!(
                        "Historical symlinks and submodules are not yet reviewable"
                    ));
                }
                let current_exists = metadata.is_some();
                let current_mode = if let Some(metadata) = metadata {
                    if metadata.is_executable {
                        0o100755
                    } else {
                        0o100644
                    }
                } else {
                    0
                };
                let base_mode = base_mode.unwrap_or(if base_exists { current_mode } else { 0 });
                let path_string = path.to_string();
                cx.background_spawn(async move {
                    let current = if snapshot.line_ending() == language::LineEnding::Windows {
                        snapshot.text().replace('\n', "\r\n")
                    } else {
                        snapshot.text()
                    };
                    let base = if base.line_ending() == language::LineEnding::Windows {
                        base.text().replace('\n', "\r\n")
                    } else {
                        base.text()
                    };
                    if current.contains('\0') || base.contains('\0') {
                        return Err(anyhow!("Binary files are not yet reviewable"));
                    }

                    let changed = base_exists != current_exists
                        || base != current
                        || base_mode != current_mode;
                    let fingerprint = Fingerprint::new(
                        &path_string,
                        base_exists.then_some(base.as_bytes()),
                        current_exists.then_some(current.as_bytes()),
                        base_mode,
                        current_mode,
                    )
                    .with_rename(renamed_from.as_deref());
                    Ok((fingerprint, changed))
                })
                .await
            }
            .await;
            this.update(cx, |this, cx| {
                if this.generation != generation {
                    return;
                }
                if let Some(entry) = this.entries.get_mut(&path) {
                    if entry.hash_generation != hash_generation {
                        return;
                    }
                    if entry.buffer.as_ref().is_none_or(|buffer| {
                        buffer.read(cx).snapshot().remote_id() != expected_snapshot.remote_id()
                            || buffer.read(cx).snapshot().version() != expected_snapshot.version()
                    }) || entry.diff.as_ref().is_none_or(|diff| {
                        diff.read(cx).base_text(cx).remote_id() != expected_base.remote_id()
                            || diff.read(cx).base_text(cx).version() != expected_base.version()
                    }) {
                        return;
                    }
                    match result {
                        Ok((fingerprint, changed)) => {
                            entry.fingerprint = Some(fingerprint);
                            entry.validated_snapshot = Some(expected_snapshot);
                            entry.validated_base = Some(expected_base);
                            entry.changed = changed;
                            entry.error = None;
                        }
                        Err(error) => {
                            entry.fingerprint = None;
                            entry.error = Some(format!("{error:#}"));
                        }
                    }
                }
                this.rebuild_rows(cx);
            })
            .log_err();
        }));
        self.rebuild_rows(cx);
    }

    fn scope_matches(&self, cx: &App) -> bool {
        let Some(scope) = &self.scope else {
            return false;
        };
        let Some(repository) = self.list.read(cx).repo() else {
            return false;
        };
        let repository = repository.read(cx);
        repository
            .branch
            .as_ref()
            .is_some_and(|branch| branch.ref_name.as_ref() == scope.branch)
            && repository.work_directory_abs_path.as_ref() == scope.worktree
            && matches!(self.list.read(cx).diff_base(), DiffBase::Merge { base_ref } if base_ref.as_ref() == scope.base_ref)
    }

    fn is_viewed(&self, path: &RepoPath, cx: &App) -> bool {
        self.scope_matches(cx)
            && self.error.is_none()
            && self.list.read(cx).tree_diff_error().is_none()
            && self
                .entries
                .get(path)
                .filter(|entry| {
                    entry
                        .validated_base
                        .as_ref()
                        .zip(entry.diff.as_ref())
                        .is_some_and(|(base, diff)| {
                            base.remote_id() == diff.read(cx).base_text(cx).remote_id()
                                && base.version() == diff.read(cx).base_text(cx).version()
                        })
                })
                .filter(|entry| {
                    entry
                        .validated_snapshot
                        .as_ref()
                        .zip(entry.buffer.as_ref())
                        .is_some_and(|(snapshot, buffer)| {
                            let buffer = buffer.read(cx);
                            buffer.snapshot().remote_id() == snapshot.remote_id()
                                && buffer.snapshot().version() == snapshot.version()
                                && buffer.line_ending() == snapshot.line_ending()
                                && buffer.file().map(|file| file.disk_state())
                                    == snapshot.file().map(|file| file.disk_state())
                        })
                })
                .and_then(|entry| entry.fingerprint.as_ref())
                .zip(self.state.as_ref())
                .is_some_and(|(fingerprint, state)| {
                    state.read(cx).is_viewed(&path.to_string(), fingerprint)
                })
    }

    fn toggle_viewed(&mut self, path: &RepoPath, cx: &mut Context<Self>) {
        if !self.scope_matches(cx)
            || self.loading
            || self.list.read(cx).is_tree_base_loading()
            || self.list.read(cx).tree_diff_error().is_some()
            || self.error.is_some()
        {
            return;
        }
        let Some(entry) = self.entries.get(path) else {
            return;
        };
        if entry
            .hash_task
            .as_ref()
            .is_some_and(|task| !task.is_ready())
        {
            return;
        }
        let Some(fingerprint) = entry.fingerprint.clone() else {
            return;
        };
        if entry
            .validated_snapshot
            .as_ref()
            .zip(entry.buffer.as_ref())
            .is_none_or(|(snapshot, buffer)| {
                let buffer = buffer.read(cx);
                buffer.snapshot().remote_id() != snapshot.remote_id()
                    || buffer.snapshot().version() != snapshot.version()
                    || buffer.line_ending() != snapshot.line_ending()
                    || buffer.file().map(|file| file.disk_state())
                        != snapshot.file().map(|file| file.disk_state())
            })
        {
            self.reconcile(path, cx);
            return;
        }
        if entry
            .validated_base
            .as_ref()
            .zip(entry.diff.as_ref())
            .is_none_or(|(base, diff)| {
                base.remote_id() != diff.read(cx).base_text(cx).remote_id()
                    || base.version() != diff.read(cx).base_text(cx).version()
            })
        {
            self.reconcile(path, cx);
            return;
        }
        let viewed = self.is_viewed(path, cx);
        if let Some(state) = self.state.as_ref() {
            state.update(cx, |state, cx| {
                state.set_viewed(path.to_string(), (!viewed).then_some(fingerprint), cx)
            });
        }
        self.rebuild_rows(cx);
    }

    fn rebuild_rows(&mut self, cx: &mut Context<Self>) {
        let mut root = Folder::default();
        for (path, entry) in &self.entries {
            if !entry.changed {
                continue;
            }
            let viewed = self.is_viewed(path, cx) as usize;
            root.viewed += viewed;
            root.total += 1;
            let path_string = path.to_string();
            let mut parts = path_string.split('/').peekable();
            let mut folder = &mut root;
            while let Some(part) = parts.next() {
                if parts.peek().is_none() {
                    folder.files.push(path.clone());
                } else {
                    folder = folder.folders.entry(part.to_owned()).or_default();
                    folder.viewed += viewed;
                    folder.total += 1;
                }
            }
        }
        self.total = root.total;
        self.viewed = root.viewed;
        self.rows.clear();
        flatten(&root, "", 0, &self.collapsed, &mut self.rows);
        cx.notify();
    }

    fn render_row(&self, index: usize, cx: &Context<Self>) -> AnyElement {
        match self.rows[index].clone() {
            Row::Folder {
                path,
                name,
                depth,
                viewed,
                total,
            } => {
                let expanded = !self.collapsed.contains(&path);
                let toggle_path = path.clone();
                ListItem::new(("review-folder", index))
                    .indent_level(depth + 1)
                    .toggle(expanded)
                    .always_show_disclosure_icon(true)
                    .on_toggle(cx.listener(move |this, _, _, cx| {
                        if !this.collapsed.remove(&toggle_path) {
                            this.collapsed.insert(toggle_path.clone());
                        }
                        this.rebuild_rows(cx);
                    }))
                    .start_slot(
                        Icon::new(IconName::Folder)
                            .color(Color::Muted)
                            .size(IconSize::Small),
                    )
                    .child(Label::new(name).size(LabelSize::Small))
                    .end_slot(
                        Label::new(format!("{viewed}/{total}"))
                            .size(LabelSize::XSmall)
                            .color(Color::Muted),
                    )
                    .on_click(cx.listener(move |this, _, _, cx| {
                        if !this.collapsed.remove(&path) {
                            this.collapsed.insert(path.clone());
                        }
                        this.rebuild_rows(cx);
                    }))
                    .into_any_element()
            }
            Row::File { path, depth } => {
                let Some(entry) = self.entries.get(&path) else {
                    return div().into_any_element();
                };
                let viewed = self.is_viewed(&path, cx);
                let disabled = !self.scope_matches(cx)
                    || self.list.read(cx).is_tree_base_loading()
                    || self.list.read(cx).tree_diff_error().is_some()
                    || self.loading
                    || self.error.is_some()
                    || entry.fingerprint.is_none()
                    || entry
                        .hash_task
                        .as_ref()
                        .is_some_and(|task| !task.is_ready())
                    || self
                        .state
                        .as_ref()
                        .is_none_or(|state| state.read(cx).error.is_some());
                let renamed_from = self.list.read(cx).renamed_from(&path);
                let (status, color) = if renamed_from.is_some() {
                    ("R", Color::VersionControlModified)
                } else if entry.status.is_deleted() {
                    ("D", Color::VersionControlDeleted)
                } else if entry.status.is_created() {
                    ("A", Color::VersionControlAdded)
                } else {
                    ("M", Color::VersionControlModified)
                };
                let checkbox_path = path.clone();
                let tooltip = entry.error.clone().unwrap_or_else(|| {
                    if viewed {
                        "Mark unviewed".into()
                    } else {
                        "Mark Viewed: approve this comparison".into()
                    }
                });
                let icon = FileIcons::get_icon(path.as_std_path(), cx)
                    .map(Icon::from_path)
                    .unwrap_or_else(|| Icon::new(IconName::File));
                let path_tooltip = renamed_from.map_or_else(
                    || path.to_string(),
                    |source| {
                        format!(
                            "{} (renamed from {}; review both entries)",
                            &*path, &**source
                        )
                    },
                );
                ListItem::new(("review-file", index))
                    .tooltip(Tooltip::text(path_tooltip))
                    .indent_level(depth + 1)
                    .toggle_state(self.selected.as_ref() == Some(&path))
                    .start_slot(
                        h_flex()
                            .gap_1()
                            .child(
                                Checkbox::new(("review-viewed", index), viewed.into())
                                    .disabled(disabled)
                                    .tooltip(Tooltip::text(tooltip))
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        cx.stop_propagation();
                                        this.toggle_viewed(&checkbox_path, cx)
                                    })),
                            )
                            .child(icon.size(IconSize::Small).color(Color::Muted)),
                    )
                    .child(
                        Label::new(
                            path.as_std_path()
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .into_owned(),
                        )
                        .size(LabelSize::Small)
                        .color(if viewed {
                            Color::Muted
                        } else {
                            Color::Default
                        }),
                    )
                    .end_slot(Label::new(status).size(LabelSize::XSmall).color(color))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.selected = Some(path.clone());
                        if let Some(entry) = this.entries.get(&path)
                            && let Some(repository) = this.list.read(cx).repo()
                        {
                            let key =
                                project_diff_path_key(repository.read(cx), &path, entry.status, cx);
                            this.diff
                                .update(cx, |diff, cx| diff.move_to_path(key, window, cx))
                                .log_err();
                        }
                        cx.notify();
                    }))
                    .into_any_element()
            }
        }
    }
}

fn flatten(
    folder: &Folder,
    parent: &str,
    depth: usize,
    collapsed: &HashSet<String>,
    rows: &mut Vec<Row>,
) {
    for (name, child) in &folder.folders {
        let path = if parent.is_empty() {
            name.clone()
        } else {
            format!("{parent}/{name}")
        };
        rows.push(Row::Folder {
            path: path.clone(),
            name: name.clone(),
            depth,
            viewed: child.viewed,
            total: child.total,
        });
        if !collapsed.contains(&path) {
            flatten(child, &path, depth + 1, collapsed, rows);
        }
    }
    rows.extend(folder.files.iter().map(|path| Row::File {
        path: path.clone(),
        depth,
    }));
}

impl Render for BranchReview {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.state.as_ref().map(|state| state.read(cx));
        let error = self
            .error
            .clone()
            .or_else(|| state.and_then(|state| state.error.clone()))
            .or_else(|| self.list.read(cx).tree_diff_error().map(str::to_owned));
        let saving = state.is_some_and(|state| state.saving);
        v_flex()
            .h_full()
            .w(rems(20.))
            .flex_shrink_0()
            .border_r_1()
            .border_color(cx.theme().colors().border)
            .bg(cx.theme().colors().panel_background)
            .child(
                h_flex()
                    .px_2()
                    .py_1()
                    .justify_between()
                    .child(Label::new("Branch Review").size(LabelSize::Small))
                    .child(
                        IconButton::new("refresh-review", IconName::ArrowCircle)
                            .tooltip(Tooltip::text("Refresh comparison"))
                            .on_click(cx.listener(|this, _, _, cx| {
                                let base = this.list.read(cx).diff_base().clone();
                                this.list
                                    .update(cx, |list, cx| list.set_diff_base(base, cx));
                            })),
                    ),
            )
            .child(
                h_flex()
                    .px_2()
                    .pb_1()
                    .gap_1()
                    .child(
                        Label::new(format!("{}/{} Viewed", self.viewed, self.total))
                            .size(LabelSize::XSmall)
                            .color(Color::Muted),
                    )
                    .when(self.loading || saving, |row| {
                        row.child(
                            Label::new(if saving { "Saving…" } else { "Validating…" })
                                .size(LabelSize::XSmall)
                                .color(Color::Muted),
                        )
                    }),
            )
            .child(
                div().px_2().pb_1().child(
                    Label::new("Merge base + working files")
                        .size(LabelSize::XSmall)
                        .color(Color::Muted),
                ),
            )
            .when_some(error, |view, error| {
                view.child(
                    div().px_2().py_1().child(
                        Label::new(error)
                            .line_clamp(4)
                            .size(LabelSize::XSmall)
                            .color(Color::Error),
                    ),
                )
            })
            .child(
                uniform_list(
                    "branch-review-files",
                    self.rows.len(),
                    cx.processor(|this, range: std::ops::Range<usize>, _, cx| {
                        range
                            .map(|index| this.render_row(index, cx))
                            .collect::<Vec<_>>()
                    }),
                )
                .flex_1(),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::branch_diff::BranchDiff;
    use fs::Fs as _;
    use gpui::{TestAppContext, UpdateGlobal as _};
    use project::FakeFs;
    use serde_json::json;
    use settings::SettingsStore;
    use std::path::Path;
    use util::path;
    use workspace::MultiWorkspace;

    #[gpui::test]
    async fn native_diff_edits_autosave_and_invalidate_only_the_edited_file(
        cx: &mut TestAppContext,
    ) {
        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            crate::init(cx);
        });
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {"logs": {"refs": {"heads": {"feature": "branch created\n"}}}},
                "src": {"a.txt": "reviewed a", "b.txt": "reviewed b", "clean.txt": "unchanged"}
            }),
        )
        .await;
        let git_dir = Path::new(path!("/project/.git"));
        fs.set_branch_name(git_dir, Some("feature"));
        fs.set_head_and_index_for_repo(
            git_dir,
            &[
                ("src/a.txt", "reviewed a".into()),
                ("src/b.txt", "reviewed b".into()),
                ("src/clean.txt", "unchanged".into()),
            ],
        );
        fs.set_merge_base_content_for_repo(
            git_dir,
            &[
                ("src/a.txt", "base a".into()),
                ("src/b.txt", "base b".into()),
                ("src/clean.txt", "unchanged".into()),
            ],
        );
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |workspace, _| workspace.workspace().clone());
        let diff = cx
            .update(|window, cx| {
                BranchDiff::new_with_default_branch(project.clone(), workspace.clone(), window, cx)
            })
            .await
            .unwrap();
        cx.run_until_parked();
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.run_until_parked();
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_active_pane(Box::new(diff.clone()), None, true, window, cx);
            SettingsStore::update_global(cx, |settings, cx| {
                settings.update_user_settings(cx, |settings| {
                    settings.workspace.autosave = Some(settings::AutosaveSetting::AfterDelay {
                        milliseconds: 500.into(),
                    });
                });
            });
        });
        let review = diff.read_with(cx, |diff, _| diff.review.clone());
        let a = RepoPath::new("src/a.txt").unwrap();
        let b = RepoPath::new("src/b.txt").unwrap();
        review.update(cx, |review, cx| {
            review.toggle_viewed(&a, cx);
            review.toggle_viewed(&b, cx);
            assert_eq!(review.viewed, 2);
        });
        let editor = diff.read_with(cx, |diff, cx| diff.editor(cx).read(cx).rhs_editor().clone());
        editor.update_in(cx, |editor, window, cx| {
            cx.focus_self(window);
            editor.insert("edited ", window, cx);
        });
        cx.run_until_parked();
        diff.read_with(cx, |diff, cx| assert!(workspace::Item::is_dirty(diff, cx)));
        assert_eq!(
            fs.read_file_sync(path!("/project/src/a.txt")).unwrap(),
            b"reviewed a"
        );
        cx.executor().advance_clock(Duration::from_millis(250));
        cx.run_until_parked();
        review.read_with(cx, |review, cx| {
            assert!(!review.is_viewed(&a, cx));
            assert!(review.is_viewed(&b, cx));
        });
        cx.executor().advance_clock(Duration::from_millis(250));
        cx.run_until_parked();
        diff.read_with(cx, |diff, cx| assert!(!workspace::Item::is_dirty(diff, cx)));
        let saved =
            String::from_utf8(fs.read_file_sync(path!("/project/src/a.txt")).unwrap()).unwrap();
        assert!(saved.contains("edited "), "{saved:?}");
        assert_eq!(
            fs.read_file_sync(path!("/project/src/b.txt")).unwrap(),
            b"reviewed b"
        );
        review.read_with(cx, |review, cx| {
            assert!(!review.is_viewed(&a, cx));
            assert!(review.is_viewed(&b, cx));
        });
    }

    #[gpui::test]
    async fn explicit_review_survives_refresh_and_selectively_invalidates_live_edits(
        cx: &mut TestAppContext,
    ) {
        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            crate::init(cx);
        });
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {"logs": {"refs": {"heads": {"feature": "branch created\n"}}}},
                "src": {"a.txt": "reviewed a", "b.txt": "reviewed b", "clean.txt": "unchanged"}
            }),
        )
        .await;
        let git_dir = Path::new(path!("/project/.git"));
        fs.set_branch_name(git_dir, Some("feature"));
        fs.set_head_and_index_for_repo(
            git_dir,
            &[
                ("src/a.txt", "reviewed a".into()),
                ("src/b.txt", "reviewed b".into()),
                ("src/clean.txt", "unchanged".into()),
            ],
        );
        fs.set_merge_base_content_for_repo(
            git_dir,
            &[
                ("src/a.txt", "base a".into()),
                ("src/b.txt", "base b".into()),
                ("src/clean.txt", "unchanged".into()),
            ],
        );
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |workspace, _| workspace.workspace().clone());
        let diff = cx
            .update(|window, cx| {
                BranchDiff::new_with_default_branch(project.clone(), workspace.clone(), window, cx)
            })
            .await
            .unwrap();
        cx.run_until_parked();
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.run_until_parked();
        let review = diff.read_with(cx, |diff, _| diff.review.clone());
        let a = RepoPath::new("src/a.txt").unwrap();
        let b = RepoPath::new("src/b.txt").unwrap();
        review.update(cx, |review, cx| {
            assert!(review.error.is_none(), "{:?}", review.error);
            assert_eq!((review.viewed, review.total), (0, 2));
            review.selected = Some(a.clone());
            review.rebuild_rows(cx);
            assert!(!review.is_viewed(&a, cx));
            review.toggle_viewed(&a, cx);
            review.toggle_viewed(&b, cx);
            assert_eq!((review.viewed, review.total), (2, 2));
        });
        cx.run_until_parked();
        let buffer = review.read_with(cx, |review, _| {
            review.entries.get(&b).unwrap().buffer.clone().unwrap()
        });
        buffer.update(cx, |buffer, cx| {
            buffer.set_text("revised b", cx);
        });
        cx.run_until_parked();
        review.update(cx, |review, cx| {
            assert!(review.is_viewed(&a, cx));
            assert!(!review.is_viewed(&b, cx));
            // A click during hashing must not approve the newer comparison.
            review.toggle_viewed(&b, cx);
        });
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.run_until_parked();
        review.read_with(cx, |review, cx| {
            assert_eq!((review.viewed, review.total), (1, 2));
            assert!(!review.is_viewed(&b, cx));
        });
        buffer.update(cx, |buffer, cx| {
            buffer.set_text("reviewed b", cx);
        });
        cx.run_until_parked();
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.run_until_parked();
        review.update(cx, |review, cx| {
            assert!(review.is_viewed(&b, cx));
            review.collapsed.insert("src".into());
            review.rebuild_rows(cx);
            assert_eq!(review.rows.len(), 1);
            assert_eq!((review.viewed, review.total), (2, 2));
            review.refresh(cx);
        });
        cx.run_until_parked();
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.run_until_parked();
        review.read_with(cx, |review, cx| {
            assert!(review.is_viewed(&a, cx));
            assert!(review.is_viewed(&b, cx));
            assert_eq!(review.rows.len(), 1);
        });
        let recreated = cx
            .update(|window, cx| {
                BranchDiff::new_with_default_branch(project.clone(), workspace, window, cx)
            })
            .await
            .unwrap();
        cx.run_until_parked();
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.run_until_parked();
        recreated.read_with(cx, |diff, cx| {
            let review = diff.review.read(cx);
            assert!(review.is_viewed(&a, cx));
            assert!(review.is_viewed(&b, cx));
        });
        project
            .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))
            .await
            .unwrap();
        // Exercise filesystem events and the commit transition through the real
        // project/diff-buffer pipeline, not just fingerprint comparisons.
        fs.insert_file(path!("/project/src/b.txt"), b"external b".to_vec())
            .await;
        fs.insert_file(path!("/project/src/c.txt"), b"new c".to_vec())
            .await;
        for _ in 0..4 {
            cx.run_until_parked();
            cx.executor().advance_clock(Duration::from_millis(100));
        }
        cx.run_until_parked();
        let c = RepoPath::new("src/c.txt").unwrap();
        review.read_with(cx, |review, cx| {
            assert!(review.is_viewed(&a, cx));
            assert!(!review.is_viewed(&b, cx));
            assert!(!review.is_viewed(&c, cx));
            assert_eq!((review.viewed, review.total), (1, 3));
        });
        fs.set_head_and_index_for_repo(
            git_dir,
            &[
                ("src/a.txt", "reviewed a".into()),
                ("src/b.txt", "external b".into()),
                ("src/c.txt", "new c".into()),
                ("src/clean.txt", "unchanged".into()),
            ],
        );
        for _ in 0..4 {
            cx.run_until_parked();
            cx.executor().advance_clock(Duration::from_millis(100));
        }
        cx.run_until_parked();
        review.read_with(cx, |review, cx| {
            assert!(review.is_viewed(&a, cx));
            assert_eq!((review.viewed, review.total), (1, 3));
        });
        // Same branch name with a new reflog origin must not reuse approvals.
        fs.insert_file(
            path!("/project/.git/logs/refs/heads/feature"),
            b"recreated branch\n".to_vec(),
        )
        .await;
        review.update(cx, |review, cx| review.refresh(cx));
        for _ in 0..4 {
            cx.run_until_parked();
            cx.executor().advance_clock(Duration::from_millis(100));
        }
        cx.run_until_parked();
        review.read_with(cx, |review, cx| {
            assert!(!review.is_viewed(&a, cx));
            assert_eq!(review.viewed, 0);
        });
        let clean = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/project/src/clean.txt"), cx)
            })
            .await
            .unwrap();
        clean.update(cx, |buffer, cx| buffer.set_text("unsaved change", cx));
        for _ in 0..4 {
            cx.run_until_parked();
            cx.executor().advance_clock(Duration::from_millis(100));
        }
        cx.run_until_parked();
        review.read_with(cx, |review, cx| {
            let path = RepoPath::new("src/clean.txt").unwrap();
            assert!(review.entries.contains_key(&path));
            assert!(!review.is_viewed(&path, cx));
            assert_eq!(review.total, 4);
        });
        clean.update(cx, |buffer, cx| buffer.set_text("unchanged", cx));
        for _ in 0..4 {
            cx.run_until_parked();
            cx.executor().advance_clock(Duration::from_millis(100));
        }
        cx.run_until_parked();
        review.read_with(cx, |review, _| assert_eq!(review.total, 3));
        review.update(cx, |review, cx| review.toggle_viewed(&a, cx));
        let list = review.read_with(cx, |review, _| review.list.clone());
        list.update(cx, |list, cx| {
            list.set_diff_base(
                DiffBase::Merge {
                    base_ref: "release".into(),
                },
                cx,
            )
        });
        cx.run_until_parked();
        review.read_with(cx, |review, cx| assert!(!review.is_viewed(&a, cx)));
        // Switch back while the previous scope still has hashes in flight.
        list.update(cx, |list, cx| {
            list.set_diff_base(
                DiffBase::Merge {
                    base_ref: "origin/main".into(),
                },
                cx,
            )
        });
        for _ in 0..4 {
            cx.run_until_parked();
            cx.executor().advance_clock(Duration::from_millis(100));
        }
        cx.run_until_parked();
        review.read_with(cx, |review, cx| {
            assert_eq!(review.scope.as_ref().unwrap().base_ref, "origin/main");
            assert!(review.is_viewed(&a, cx));
        });
        // Git blob IDs are immutable. The generic fake helper reuses IDs by
        // position, so install a distinct blob explicitly for base advancement.
        fs.with_git_state(git_dir, true, |state| {
            let oid = git::Oid::from_bytes(&[99; 20]).unwrap();
            state.oids.insert(oid, b"changed base a".to_vec());
            state.merge_base_contents.insert(a.clone(), oid);
        })
        .unwrap();
        list.update(cx, |list, cx| {
            list.set_diff_base(
                DiffBase::Merge {
                    base_ref: "origin/main".into(),
                },
                cx,
            )
        });
        for _ in 0..4 {
            cx.run_until_parked();
            cx.executor().advance_clock(Duration::from_millis(100));
        }
        cx.run_until_parked();
        review.read_with(cx, |review, cx| assert!(!review.is_viewed(&a, cx)));
        let multibuffer = review.read_with(cx, |review, _| review.diff.upgrade().unwrap());
        cx.update(|window, cx| {
            multibuffer.update(cx, |diff, cx| diff.move_to_beginning(window, cx))
        });
        fs.rename(
            Path::new(path!("/project/src/a.txt")),
            Path::new(path!("/project/src/renamed_a.txt")),
            Default::default(),
        )
        .await
        .unwrap();
        for _ in 0..4 {
            cx.run_until_parked();
            cx.executor().advance_clock(Duration::from_millis(100));
        }
        cx.run_until_parked();
        review.read_with(cx, |review, cx| {
            let renamed = RepoPath::new("src/renamed_a.txt").unwrap();
            assert!(review.entries.contains_key(&renamed));
            assert!(!review.is_viewed(&renamed, cx));
        });
    }
}
