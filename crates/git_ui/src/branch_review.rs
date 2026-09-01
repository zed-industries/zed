use crate::{
    diff_multibuffer::{DiffMultibuffer, ViewedDeltaSide, project_diff_path_key},
    review_state::{Fingerprint, ReviewScope, ReviewState, SnapshotAvailability, digest},
};
use anyhow::{Context as _, Result, anyhow};
use buffer_diff::BufferDiff;
use collections::HashSet;
use editor::{Editor, EditorEvent};
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

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum ReviewFilter {
    #[default]
    All,
    Unviewed,
    Changed,
}

struct ReviewEntry {
    status: FileStatus,
    buffer: Option<Entity<Buffer>>,
    diff: Option<Entity<BufferDiff>>,
    fingerprint: Option<Fingerprint>,
    validated_snapshot: Option<language::BufferSnapshot>,
    validated_base: Option<language::BufferSnapshot>,
    validated_comparison: Option<(Option<String>, Option<String>)>,
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

pub(crate) enum ReviewEvent {
    OpenDiff,
    Reply(u64),
}

impl gpui::EventEmitter<ReviewEvent> for BranchReview {}

pub(crate) struct BranchReview {
    project: Entity<Project>,
    diff: WeakEntity<DiffMultibuffer>,
    list: Entity<DiffBufferList>,
    entries: BTreeMap<RepoPath, ReviewEntry>,
    comments: Vec<crate::github_review::PublishedComment>,
    comment_markdown: BTreeMap<u64, (String, Entity<markdown::Markdown>)>,
    expanded_threads: HashSet<u64>,
    comment_blocks: Vec<(
        Entity<editor::Editor>,
        Vec<editor::display_map::CustomBlockId>,
    )>,
    rows: Vec<Row>,
    collapsed: HashSet<String>,
    selected: Option<RepoPath>,
    search: Entity<Editor>,
    query: String,
    filter: ReviewFilter,
    changed_since_viewed: usize,
    matching: usize,
    scroll_handle: gpui::UniformListScrollHandle,
    _search_subscription: Subscription,
    state: Option<Entity<ReviewState>>,
    scope: Option<ReviewScope>,
    storage_key: Option<String>,
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
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let search = cx.new(|cx| Editor::single_line(window, cx));
        search.update(cx, |search, cx| {
            search.set_placeholder_text("Filter files…", window, cx)
        });
        let search_subscription = cx.subscribe(&search, |this, search, event, cx| {
            if matches!(event, EditorEvent::Edited { .. }) {
                this.query = search.read(cx).text(cx).to_lowercase();
                this.rebuild_rows(cx);
            }
        });
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
            comments: Vec::new(),
            comment_markdown: BTreeMap::new(),
            expanded_threads: HashSet::default(),
            comment_blocks: Vec::new(),
            rows: Vec::new(),
            collapsed: HashSet::default(),
            selected: None,
            search,
            query: String::new(),
            filter: ReviewFilter::All,
            changed_since_viewed: 0,
            matching: 0,
            scroll_handle: gpui::UniformListScrollHandle::new(),
            _search_subscription: search_subscription,
            state: None,
            scope: None,
            storage_key: None,
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

    pub fn comparison_for_path(
        &self,
        path: &str,
        cx: &App,
    ) -> Option<(Option<String>, Option<String>)> {
        if !self.scope_matches(cx) {
            return None;
        }
        let entry = self
            .entries
            .iter()
            .find_map(|(entry_path, entry)| (entry_path.as_unix_str() == path).then_some(entry))?;
        if entry.error.is_some() {
            return None;
        }
        let buffer = entry.buffer.as_ref()?.read(cx);
        let diff = entry.diff.as_ref()?.read(cx);
        let deleted = !buffer.is_dirty()
            && buffer.file().is_some_and(|file| {
                matches!(
                    file.disk_state(),
                    language::DiskState::Deleted
                        | language::DiskState::Historic { was_deleted: true }
                )
            });
        Some(((!deleted).then(|| buffer.text()), diff.base_text_string(cx)))
    }

    pub fn path_for_buffer(&self, buffer_id: language::BufferId, cx: &App) -> Option<RepoPath> {
        self.entries.iter().find_map(|(path, entry)| {
            (entry
                .buffer
                .as_ref()
                .is_some_and(|buffer| buffer.read(cx).remote_id() == buffer_id)
                || entry
                    .diff
                    .as_ref()
                    .is_some_and(|diff| diff.read(cx).base_text(cx).remote_id() == buffer_id))
            .then(|| path.clone())
        })
    }

    pub fn set_comments(
        &mut self,
        comments: Vec<crate::github_review::PublishedComment>,
        cx: &mut Context<Self>,
    ) {
        self.comment_markdown
            .retain(|id, _| comments.iter().any(|published| published.comment.id == *id));
        for published in &comments {
            let body = published.comment.body.clone().unwrap_or_default();
            let cached = self
                .comment_markdown
                .entry(published.comment.id)
                .or_insert_with(|| {
                    (
                        body.clone(),
                        crate::review_markdown::new(
                            &body,
                            self.project.read(cx).languages().clone(),
                            cx,
                        ),
                    )
                });
            if cached.0 != body {
                cached.1.update(cx, |markdown, cx| {
                    markdown.replace(crate::review_markdown::source(&body), cx)
                });
                cached.0 = body;
            }
        }
        self.comments = comments;
        let review = cx.weak_entity();
        cx.defer(move |cx| {
            review
                .update(cx, |review, cx| review.update_comment_blocks(cx))
                .log_err();
        });
    }

    fn update_comment_blocks(&mut self, cx: &mut Context<Self>) {
        use editor::display_map::{BlockPlacement, BlockProperties, BlockStyle};
        for (editor, ids) in self.comment_blocks.drain(..) {
            editor.update(cx, |editor, cx| {
                editor.remove_blocks(ids.into_iter().collect(), None, cx)
            });
        }
        if self.comments.is_empty() || !self.scope_matches(cx) {
            return;
        }
        let Some(diff) = self.diff.upgrade() else {
            return;
        };
        let split = diff.read(cx).editor().clone();
        let right = split.read(cx).rhs_editor().clone();
        let left = split.read(cx).lhs_editor().cloned();
        let mut rendered_threads = HashSet::default();
        for published in &self.comments {
            let comment = &published.comment;
            let root = comment.in_reply_to_id.unwrap_or(comment.id);
            if rendered_threads.contains(&root)
                || comment
                    .thread
                    .as_ref()
                    .is_some_and(|thread| thread.is_outdated)
            {
                continue;
            }
            let (Some(path), Some(line), Some(side)) = (&comment.path, comment.line, comment.side)
            else {
                continue;
            };
            let Some(entry) = self.entries.iter().find_map(|(entry_path, entry)| {
                (entry_path.as_unix_str() == path).then_some(entry)
            }) else {
                continue;
            };
            let (Some(buffer), Some(diff)) = (&entry.buffer, &entry.diff) else {
                continue;
            };
            let current = buffer.read(cx).snapshot();
            let base = diff.read(cx).base_text(cx);
            if current.text() != published.current.clone().unwrap_or_default()
                || base.text() != published.base.clone().unwrap_or_default()
            {
                continue;
            }
            let (editor, target) = match side {
                crate::github_review::DiffSide::Right => (&right, &current),
                crate::github_review::DiffSide::Left => {
                    let Some(left) = &left else {
                        continue;
                    };
                    (left, &base)
                }
            };
            if line == 0 || line > target.max_point().row + 1 {
                continue;
            }
            let anchor = editor
                .read(cx)
                .buffer()
                .read(cx)
                .snapshot(cx)
                .anchor_in_excerpt(target.anchor_before(language::Point::new(line - 1, 0)));
            let Some(anchor) = anchor else {
                continue;
            };
            rendered_threads.insert(root);
            let expanded = self.expanded_threads.contains(&root);
            let bodies: Vec<_> = self
                .comments
                .iter()
                .filter(|published| {
                    published
                        .comment
                        .in_reply_to_id
                        .unwrap_or(published.comment.id)
                        == root
                })
                .filter_map(|published| {
                    self.comment_markdown
                        .get(&published.comment.id)
                        .map(|(_, markdown)| {
                            (published.comment.user.login.clone(), markdown.clone())
                        })
                })
                .collect();
            let review = cx.weak_entity();
            let buffer = buffer.clone();
            let diff = diff.clone();
            let comment = comment.clone();
            let ids =
                editor.update(cx, |editor, cx| {
                    editor.insert_blocks(
                        [BlockProperties {
                            placement: BlockPlacement::Below(anchor),
                            height: None,
                            style: BlockStyle::Flex,
                            priority: 0,
                            render: std::sync::Arc::new(move |cx| {
                                if buffer.read(cx.app).version() != *current.version()
                                    || diff.read(cx.app).base_text(cx.app).version()
                                        != base.version()
                                {
                                    return div().into_any_element();
                                }
                                let review = review.clone();
                                let id = comment.in_reply_to_id.unwrap_or(comment.id);
                                let expand_review = review.clone();
                                v_flex()
                                    .w_full()
                                    .min_w_0()
                                    .gap_1()
                                    .px_2()
                                    .py_1()
                                    .bg(cx.app.theme().colors().panel_background)
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .child(
                                                Label::new(format!(
                                                    "GitHub thread · {}",
                                                    comment
                                                        .thread
                                                        .as_ref()
                                                        .map(|thread| if thread.is_resolved {
                                                            "Resolved"
                                                        } else {
                                                            "Unresolved"
                                                        })
                                                        .unwrap_or("State unavailable")
                                                ))
                                                .size(LabelSize::XSmall)
                                                .color(Color::Muted),
                                            )
                                            .child(
                                                Button::new(
                                                    ("expand-inline-thread", root as usize),
                                                    if expanded { "Collapse" } else { "Expand" },
                                                )
                                                .on_click(move |_, _, cx| {
                                                    expand_review
                                                        .update(cx, |review, cx| {
                                                            if !review
                                                                .expanded_threads
                                                                .remove(&root)
                                                            {
                                                                review
                                                                    .expanded_threads
                                                                    .insert(root);
                                                            }
                                                            review.update_comment_blocks(cx);
                                                            cx.notify();
                                                        })
                                                        .log_err();
                                                }),
                                            )
                                            .child(
                                                Button::new(
                                                    ("reply-inline", root as usize),
                                                    "Discussion / Reply",
                                                )
                                                .on_click(move |_, _, cx| {
                                                    review
                                                        .update(cx, |_, cx| {
                                                            cx.emit(ReviewEvent::Reply(id))
                                                        })
                                                        .log_err();
                                                }),
                                            ),
                                    )
                                    .child(
                                        v_flex()
                                            .min_w_0()
                                            .gap_1()
                                            .when(!expanded, |view| {
                                                view.max_h(px(160.)).overflow_hidden()
                                            })
                                            .children(bodies.iter().map(|(author, markdown)| {
                                                v_flex()
                                                    .min_w_0()
                                                    .child(
                                                        Label::new(author.clone())
                                                            .size(LabelSize::XSmall)
                                                            .color(Color::Muted),
                                                    )
                                                    .child(crate::review_markdown::render(
                                                        markdown.clone(),
                                                        cx.window,
                                                        cx.app,
                                                    ))
                                            })),
                                    )
                                    .into_any_element()
                            }),
                        }],
                        None,
                        cx,
                    )
                });
            self.comment_blocks.push((editor.clone(), ids));
        }
    }

    pub fn set_storage_key(&mut self, key: Option<String>, cx: &mut Context<Self>) {
        if self.storage_key != key {
            self.storage_key = key;
            self.state = None;
            self.refresh(cx);
        }
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
                    validated_comparison: None,
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
                    let state = if let Some(key) = &this.storage_key {
                        ReviewState::for_key(key.clone(), cx)?
                    } else {
                        ReviewState::for_scope(&scope, cx)?
                    };
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
                    Ok((
                        fingerprint,
                        changed,
                        base_exists.then_some(base),
                        current_exists.then_some(current),
                    ))
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
                        Ok((fingerprint, changed, base, current)) => {
                            if let Some(state) = &this.state {
                                state.update(cx, |state, cx| {
                                    state.enrich_approval(
                                        &path.to_string(),
                                        &fingerprint,
                                        base.as_deref(),
                                        current.as_deref(),
                                        cx,
                                    )
                                });
                            }
                            entry.fingerprint = Some(fingerprint);
                            entry.validated_snapshot = Some(expected_snapshot);
                            entry.validated_base = Some(expected_base);
                            entry.validated_comparison = Some((base, current));
                            entry.changed = changed;
                            entry.error = None;
                        }
                        Err(error) => {
                            entry.fingerprint = None;
                            entry.validated_comparison = None;
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

    fn validated_fingerprint(&self, path: &RepoPath, cx: &App) -> Option<&Fingerprint> {
        if !self.scope_matches(cx)
            || self.loading
            || self.error.is_some()
            || self.list.read(cx).is_tree_base_loading()
            || self.list.read(cx).tree_diff_error().is_some()
        {
            return None;
        }
        self.entries
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
    }

    fn is_viewed(&self, path: &RepoPath, cx: &App) -> bool {
        self.validated_fingerprint(path, cx)
            .zip(self.state.as_ref())
            .is_some_and(|(fingerprint, state)| {
                state.read(cx).is_viewed(&path.to_string(), fingerprint)
            })
    }

    fn change_reasons(&self, path: &RepoPath, cx: &App) -> Vec<&'static str> {
        self.validated_fingerprint(path, cx)
            .zip(self.state.as_ref())
            .map(|(fingerprint, state)| {
                state
                    .read(cx)
                    .change_reasons(&path.to_string(), fingerprint)
            })
            .unwrap_or_default()
    }

    fn matches_filter(&self, path: &RepoPath, cx: &App) -> bool {
        path.to_string().to_lowercase().contains(&self.query)
            && match self.filter {
                ReviewFilter::All => true,
                ReviewFilter::Unviewed => !self.is_viewed(path, cx),
                ReviewFilter::Changed => !self.change_reasons(path, cx).is_empty(),
            }
    }

    fn open_path(&mut self, path: RepoPath, window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(ReviewEvent::OpenDiff);
        self.selected = Some(path.clone());
        let viewed_delta_side = self
            .diff
            .read_with(cx, |diff, _| diff.viewed_delta_side())
            .ok()
            .flatten();
        if let Some(side) = viewed_delta_side {
            if self.activate_viewed_delta(path.clone(), side, window, cx) {
                cx.notify();
                return;
            }
            self.diff
                .update(cx, |diff, cx| diff.clear_viewed_delta(window, cx))
                .log_err();
        }
        if let Some(entry) = self.entries.get(&path)
            && let Some(repository) = self.list.read(cx).repo()
        {
            let key = project_diff_path_key(repository.read(cx), &path, entry.status, cx);
            self.diff
                .update(cx, |diff, cx| diff.move_to_path(key, window, cx))
                .log_err();
        }
        cx.notify();
    }

    pub(crate) fn is_viewed_delta(&self, cx: &App) -> bool {
        self.diff
            .read_with(cx, |diff, _| diff.viewed_delta_side().is_some())
            .unwrap_or(false)
    }

    fn approved_snapshot(&self, path: &RepoPath, cx: &App) -> Option<SnapshotAvailability> {
        self.state
            .as_ref()?
            .read(cx)
            .approved_snapshot(&path.to_string())
            .ok()
            .flatten()
    }

    fn activate_viewed_delta(
        &mut self,
        path: RepoPath,
        side: ViewedDeltaSide,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(SnapshotAvailability::Available { base, current }) =
            self.approved_snapshot(&path, cx)
        else {
            return false;
        };
        let Some((current_base, current_working)) = self
            .entries
            .get(&path)
            .and_then(|entry| entry.validated_comparison.clone())
        else {
            return false;
        };
        let (approved_text, current_exists) = match side {
            ViewedDeltaSide::WorkingFile => (current, current_working.is_some()),
            ViewedDeltaSide::Base => (base, current_base.is_some()),
        };
        self.diff
            .update(cx, |diff, cx| {
                diff.show_viewed_delta(path, side, approved_text, current_exists, window, cx)
            })
            .is_ok()
    }

    fn show_branch_comparison(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.diff
            .update(cx, |diff, cx| diff.clear_viewed_delta(window, cx))
            .log_err();
    }

    fn show_selected_viewed_delta(
        &mut self,
        side: ViewedDeltaSide,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(path) = self.selected.clone() else {
            return;
        };
        if !self.activate_viewed_delta(path, side, window, cx) {
            self.error = Some(
                "A durable Viewed snapshot is unavailable for this file. Mark it Viewed again to create one."
                    .into(),
            );
            cx.notify();
        }
    }

    pub fn navigate_unviewed(
        &mut self,
        backwards: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mut all_rows = Vec::new();
        let root = self.folder_tree(cx);
        flatten(&root, "", 0, &HashSet::default(), &mut all_rows);
        let paths: Vec<_> = all_rows
            .into_iter()
            .filter_map(|row| match row {
                Row::File { path, .. } => Some(path),
                _ => None,
            })
            .collect();
        let selected = self
            .selected
            .as_ref()
            .and_then(|selected| paths.iter().position(|path| path == selected));
        let count = paths.len();
        for offset in 1..=count {
            let index = match selected {
                Some(index) if backwards => (index + count - offset) % count,
                Some(index) => (index + offset) % count,
                None if backwards => count - offset,
                None => offset - 1,
            };
            let path = &paths[index];
            if self.matches_filter(path, cx) && !self.is_viewed(path, cx) {
                let path = path.clone();
                let mut parent = path.as_std_path().parent();
                while let Some(directory) = parent {
                    self.collapsed.remove(directory.to_string_lossy().as_ref());
                    parent = directory.parent();
                }
                self.rebuild_rows(cx);
                if let Some(index) = self.rows.iter().position(
                    |row| matches!(row, Row::File { path: candidate, .. } if candidate == &path),
                ) {
                    self.scroll_handle
                        .scroll_to_item(index, gpui::ScrollStrategy::Center);
                }
                self.open_path(path, window, cx);
                break;
            }
        }
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
        let comparison = entry
            .validated_comparison
            .as_ref()
            .map(|(base, current)| (base.as_deref(), current.as_deref()));
        if let Some(state) = self.state.as_ref() {
            state.update(cx, |state, cx| {
                state.set_viewed(
                    path.to_string(),
                    (!viewed).then_some(fingerprint),
                    (!viewed).then_some(comparison).flatten(),
                    cx,
                )
            });
        }
        self.rebuild_rows(cx);
    }

    fn toggle_viewed_from_ui(
        &mut self,
        path: &RepoPath,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let delta_side = self
            .diff
            .read_with(cx, |diff, _| diff.viewed_delta_side())
            .ok()
            .flatten();
        self.toggle_viewed(path, cx);
        let Some(delta_side) = delta_side else {
            return;
        };
        if matches!(
            self.approved_snapshot(path, cx),
            Some(SnapshotAvailability::Available { .. })
        ) {
            self.activate_viewed_delta(path.clone(), delta_side, window, cx);
        } else {
            self.show_branch_comparison(window, cx);
        }
    }

    fn rebuild_rows(&mut self, cx: &mut Context<Self>) {
        if !self.comments.is_empty() {
            let review = cx.weak_entity();
            cx.defer(move |cx| {
                review
                    .update(cx, |review, cx| review.update_comment_blocks(cx))
                    .log_err();
            });
        }
        let root = self.folder_tree(cx);
        self.total = root.total;
        self.viewed = root.viewed;
        self.changed_since_viewed = self
            .entries
            .iter()
            .filter(|(path, entry)| entry.changed && !self.change_reasons(path, cx).is_empty())
            .count();
        let mut filtered = root;
        self.filter_folders(&mut filtered, cx);
        self.rows.clear();
        let collapsed = if self.query.is_empty() && self.filter == ReviewFilter::All {
            self.collapsed.clone()
        } else {
            HashSet::default()
        };
        flatten(&filtered, "", 0, &collapsed, &mut self.rows);
        self.matching = self
            .entries
            .iter()
            .filter(|(path, entry)| entry.changed && self.matches_filter(path, cx))
            .count();
        cx.notify();
    }

    fn folder_tree(&self, cx: &App) -> Folder {
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
        root
    }

    fn filter_folders(&self, folder: &mut Folder, cx: &App) {
        folder.files.retain(|path| self.matches_filter(path, cx));
        folder.folders.retain(|_, child| {
            self.filter_folders(child, cx);
            !child.files.is_empty() || !child.folders.is_empty()
        });
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
                let reasons = self.change_reasons(&path, cx);
                let tooltip = entry.error.clone().unwrap_or_else(|| {
                    if viewed {
                        "Mark unviewed".into()
                    } else if !reasons.is_empty() {
                        format!("Changed since Viewed: {}", reasons.join(", "))
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
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        cx.stop_propagation();
                                        this.toggle_viewed_from_ui(&checkbox_path, window, cx)
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
                    .end_slot(
                        h_flex()
                            .gap_1()
                            .when(!reasons.is_empty(), |row| {
                                row.child(
                                    Icon::new(IconName::ArrowCircle)
                                        .size(IconSize::Small)
                                        .color(Color::Warning),
                                )
                            })
                            .child(Label::new(status).size(LabelSize::XSmall).color(color)),
                    )
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.open_path(path.clone(), window, cx)
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
        let validating = self.loading
            || self.list.read(cx).is_tree_base_loading()
            || self
                .entries
                .values()
                .any(|entry| entry.fingerprint.is_none() && entry.error.is_none());
        let selected_snapshot = self
            .selected
            .as_ref()
            .and_then(|path| self.approved_snapshot(path, cx));
        let selected_change_reasons = self
            .selected
            .as_ref()
            .map(|path| self.change_reasons(path, cx))
            .unwrap_or_default();
        let delta_side = self
            .diff
            .read_with(cx, |diff, _| diff.viewed_delta_side())
            .ok()
            .flatten();
        v_flex()
            .size_full()
            .min_h_0()
            .bg(cx.theme().colors().panel_background)
            .child(
                h_flex()
                    .px_2()
                    .py_1()
                    .justify_between()
                    .child(Label::new("Files").size(LabelSize::Small))
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
                div()
                    .mx_2()
                    .mb_1()
                    .px_1()
                    .border_1()
                    .border_color(cx.theme().colors().border)
                    .child(self.search.clone()),
            )
            .child(
                h_flex().px_2().gap_1().flex_wrap().children(
                    [
                        ("all-review-files", "All", ReviewFilter::All),
                        ("unviewed-review-files", "Unviewed", ReviewFilter::Unviewed),
                        (
                            "changed-review-files",
                            "Changed since Viewed",
                            ReviewFilter::Changed,
                        ),
                    ]
                    .into_iter()
                    .map(|(id, label, filter)| {
                        Button::new(id, label)
                            .toggle_state(self.filter == filter)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.filter = filter;
                                this.rebuild_rows(cx);
                            }))
                    }),
                ),
            )
            .child(
                h_flex()
                    .px_2()
                    .gap_1()
                    .child(Button::new("previous-unviewed", "Previous").on_click(
                        cx.listener(|this, _, window, cx| this.navigate_unviewed(true, window, cx)),
                    ))
                    .child(
                        Button::new("next-unviewed", "Next unviewed").on_click(cx.listener(
                            |this, _, window, cx| this.navigate_unviewed(false, window, cx),
                        )),
                    ),
            )
            .child(
                h_flex()
                    .px_2()
                    .pb_1()
                    .gap_1()
                    .child(
                        Label::new(format!(
                            "{}/{} Viewed · {} changed",
                            self.viewed, self.total, self.changed_since_viewed
                        ))
                        .size(LabelSize::XSmall)
                        .color(Color::Muted),
                    )
                    .when(validating || saving, |row| {
                        row.child(
                            Label::new(if saving { "Saving…" } else { "Validating…" })
                                .size(LabelSize::XSmall)
                                .color(Color::Muted),
                        )
                    }),
            )
            .child(
                div().px_2().pb_1().child(
                    Label::new(if delta_side.is_some() {
                        "Approved snapshot + current comparison"
                    } else {
                        "Merge base + working files"
                    })
                        .size(LabelSize::XSmall)
                        .color(Color::Muted),
                ),
            )
            .when_some(selected_snapshot, |view, snapshot| {
                let available = matches!(snapshot, SnapshotAvailability::Available { .. });
                view.child(
                    v_flex()
                        .px_2()
                        .pb_1()
                        .gap_1()
                        .child(
                            h_flex()
                                .gap_1()
                                .child(
                                    Button::new("branch-comparison", "Branch")
                                        .toggle_state(delta_side.is_none())
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.show_branch_comparison(window, cx)
                                        })),
                                )
                                .child(
                                    Button::new("since-viewed-comparison", "Since Viewed")
                                        .toggle_state(delta_side.is_some())
                                        .disabled(!available)
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.show_selected_viewed_delta(
                                                ViewedDeltaSide::WorkingFile,
                                                window,
                                                cx,
                                            )
                                        })),
                                ),
                        )
                        .when(delta_side.is_some() && available, |section| {
                            section.child(
                                h_flex()
                                    .gap_1()
                                    .child(
                                        Button::new("working-file-viewed-delta", "Working file")
                                            .toggle_state(
                                                delta_side == Some(ViewedDeltaSide::WorkingFile),
                                            )
                                            .on_click(cx.listener(|this, _, window, cx| {
                                                this.show_selected_viewed_delta(
                                                    ViewedDeltaSide::WorkingFile,
                                                    window,
                                                    cx,
                                                )
                                            })),
                                    )
                                    .child(
                                        Button::new("base-viewed-delta", "Base")
                                            .toggle_state(
                                                delta_side == Some(ViewedDeltaSide::Base),
                                            )
                                            .on_click(cx.listener(|this, _, window, cx| {
                                                this.show_selected_viewed_delta(
                                                    ViewedDeltaSide::Base,
                                                    window,
                                                    cx,
                                                )
                                            })),
                                    ),
                            )
                        })
                        .when(
                            delta_side.is_some() && !selected_change_reasons.is_empty(),
                            |section| {
                                section.child(
                                    Label::new(format!(
                                        "Comparison changes: {}",
                                        selected_change_reasons.join(", ")
                                    ))
                                    .size(LabelSize::XSmall)
                                    .color(Color::Muted),
                                )
                            },
                        )
                        .when(!available, |section| {
                            section.child(
                                Label::new(match snapshot {
                                    SnapshotAvailability::TooLarge => {
                                        "Since Viewed is unavailable because this file exceeds 2 MiB"
                                    }
                                    SnapshotAvailability::Legacy => {
                                        "Mark Viewed again to create a durable comparison snapshot"
                                    }
                                    SnapshotAvailability::Available { .. } => "",
                                })
                                .size(LabelSize::XSmall)
                                .color(Color::Muted),
                            )
                        }),
                )
            })
            .when(
                self.filter != ReviewFilter::All || !self.query.is_empty(),
                |view| {
                    view.child(
                        div().px_2().child(
                            Label::new(format!("{} matching files", self.matching))
                                .size(LabelSize::XSmall),
                        ),
                    )
                },
            )
            .when(
                self.matching == 0 && !validating && error.is_none(),
                |view| {
                    view.child(
                        div().p_2().child(
                            Label::new(if !self.query.is_empty() {
                                "No matching files"
                            } else if self.filter == ReviewFilter::Unviewed {
                                "No remaining unviewed files"
                            } else if self.filter == ReviewFilter::Changed {
                                "No files changed since Viewed"
                            } else {
                                "No changed files"
                            })
                            .size(LabelSize::Small),
                        ),
                    )
                },
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
                .track_scroll(&self.scroll_handle)
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
        review.update(cx, |review, cx| {
            let comment = serde_json::from_value(json!({"id":1, "body":"Native GitHub thread", "user":{"login":"reviewer"}, "path":"src/a.txt", "side":"RIGHT", "line":1})).unwrap();
            review.set_comments(vec![crate::github_review::PublishedComment { comment, current: Some("reviewed a".into()), base: Some("base a".into()) }], cx);
        });
        cx.run_until_parked();
        review.read_with(cx, |review, _| assert_eq!(review.comment_blocks.len(), 1));
        let editor = diff.read_with(cx, |diff, cx| diff.editor(cx).read(cx).rhs_editor().clone());
        editor.update_in(cx, |editor, window, cx| {
            cx.focus_self(window);
            editor.insert("edited ", window, cx);
        });
        cx.run_until_parked();
        review.read_with(cx, |review, _| {
            assert!(
                review.comment_blocks.is_empty(),
                "Local edits must remove uncertain inline thread positions"
            )
        });
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
        let expected_buffer_id = review.read_with(cx, |review, cx| {
            assert!(matches!(
                review.approved_snapshot(&a, cx),
                Some(SnapshotAvailability::Available {
                    base: Some(_),
                    current: Some(_),
                })
            ));
            review.entries[&a]
                .buffer
                .as_ref()
                .unwrap()
                .read(cx)
                .remote_id()
        });
        let unrelated_buffer_id = review.read_with(cx, |review, cx| {
            review.entries[&b]
                .buffer
                .as_ref()
                .unwrap()
                .read(cx)
                .remote_id()
        });
        let expected_base_buffer_id = review.read_with(cx, |review, cx| {
            review.entries[&a]
                .diff
                .as_ref()
                .unwrap()
                .read(cx)
                .base_text(cx)
                .remote_id()
        });
        review.update_in(cx, |review, window, cx| {
            review.open_path(a.clone(), window, cx);
            review.show_selected_viewed_delta(ViewedDeltaSide::WorkingFile, window, cx);
        });
        cx.run_until_parked();
        let delta_editor =
            diff.read_with(cx, |diff, cx| diff.editor(cx).read(cx).rhs_editor().clone());
        delta_editor.read_with(cx, |editor, cx| {
            let buffer_ids = editor
                .buffer()
                .read(cx)
                .snapshot(cx)
                .all_buffer_ids()
                .collect::<Vec<_>>();
            assert_eq!(buffer_ids, vec![expected_buffer_id]);
            assert!(!buffer_ids.contains(&unrelated_buffer_id));
        });
        delta_editor.update_in(cx, |editor, window, cx| {
            cx.focus_self(window);
            editor.insert("delta ", window, cx);
        });
        cx.executor().advance_clock(Duration::from_millis(500));
        cx.run_until_parked();
        assert!(
            String::from_utf8(fs.read_file_sync(path!("/project/src/a.txt")).unwrap())
                .unwrap()
                .contains("delta ")
        );
        review.update_in(cx, |review, window, cx| {
            review.show_selected_viewed_delta(ViewedDeltaSide::Base, window, cx);
            assert!(review.is_viewed_delta(cx));
        });
        cx.executor().advance_clock(Duration::from_millis(250));
        cx.run_until_parked();
        delta_editor.read_with(cx, |editor, cx| {
            let buffer_ids = editor
                .buffer()
                .read(cx)
                .snapshot(cx)
                .all_buffer_ids()
                .collect::<Vec<_>>();
            assert!(
                buffer_ids
                    .iter()
                    .all(|buffer_id| *buffer_id == expected_base_buffer_id)
            );
            assert!(!buffer_ids.contains(&unrelated_buffer_id));
        });
        review.update_in(cx, |review, window, cx| {
            review.show_branch_comparison(window, cx);
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
    async fn pr_progress_survives_new_checkout_branches_and_revalidates_only_changed_files(
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
        fs.insert_tree(path!("/project"), json!({".git":{"logs":{"refs":{"heads":{"review-old":"created old\n", "review-new":"created new\n"}}}}, "src":{"a.txt":"reviewed a", "b.txt":"reviewed b"}})).await;
        let git_dir = Path::new(path!("/project/.git"));
        fs.set_branch_name(git_dir, Some("review-old"));
        fs.set_head_and_index_for_repo(
            git_dir,
            &[
                ("src/a.txt", "reviewed a".into()),
                ("src/b.txt", "reviewed b".into()),
            ],
        );
        fs.set_merge_base_content_for_repo(
            git_dir,
            &[
                ("src/a.txt", "base a".into()),
                ("src/b.txt", "base b".into()),
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
        let review = diff.read_with(cx, |diff, _| diff.review.clone());
        let key = "github_review_v1:test_pr_across_revisions".to_string();
        review.update(cx, |review, cx| {
            review.set_storage_key(Some(key.clone()), cx)
        });
        for _ in 0..5 {
            cx.run_until_parked();
            cx.executor().advance_clock(Duration::from_millis(100));
        }
        cx.run_until_parked();
        let a = RepoPath::new("src/a.txt").unwrap();
        let b = RepoPath::new("src/b.txt").unwrap();
        let c = RepoPath::new("src/c.txt").unwrap();
        review.update(cx, |review, cx| {
            review.toggle_viewed(&a, cx);
            review.toggle_viewed(&b, cx);
        });
        cx.run_until_parked();
        review.read_with(cx, |review, cx| {
            assert!(review.is_viewed(&a, cx));
            assert!(review.is_viewed(&b, cx));
        });
        fs.insert_file(path!("/project/src/b.txt"), b"revised b".to_vec())
            .await;
        fs.insert_file(path!("/project/src/c.txt"), b"new c".to_vec())
            .await;
        fs.set_head_and_index_for_repo(
            git_dir,
            &[
                ("src/a.txt", "reviewed a".into()),
                ("src/b.txt", "revised b".into()),
                ("src/c.txt", "new c".into()),
            ],
        );
        fs.set_branch_name(git_dir, Some("review-new"));
        for _ in 0..5 {
            cx.run_until_parked();
            cx.executor().advance_clock(Duration::from_millis(100));
        }
        cx.run_until_parked();
        let recreated = cx
            .update(|window, cx| {
                BranchDiff::new_with_default_branch(project, workspace, window, cx)
            })
            .await
            .unwrap();
        let recreated_review = recreated.read_with(cx, |diff, _| diff.review.clone());
        recreated_review.update(cx, |review, cx| review.set_storage_key(Some(key), cx));
        for _ in 0..5 {
            cx.run_until_parked();
            cx.executor().advance_clock(Duration::from_millis(100));
        }
        cx.run_until_parked();
        recreated_review.read_with(cx, |review, cx| {
            assert!(review.is_viewed(&a, cx));
            assert!(!review.is_viewed(&b, cx));
            assert!(!review.is_viewed(&c, cx));
            assert_eq!((review.viewed, review.total), (1, 3));
            assert_eq!(
                review.approved_snapshot(&b, cx),
                Some(SnapshotAvailability::Available {
                    base: Some("base b".into()),
                    current: Some("reviewed b".into()),
                })
            );
        });
        recreated_review.update_in(cx, |review, window, cx| {
            assert_eq!(review.change_reasons(&b, cx), ["Content changed"]);
            assert!(review.change_reasons(&a, cx).is_empty());
            assert!(review.change_reasons(&c, cx).is_empty());
            assert_eq!(review.changed_since_viewed, 1);
            review.filter = ReviewFilter::Changed;
            review.rebuild_rows(cx);
            assert_eq!(review.matching, 1);
            assert!(
                review
                    .rows
                    .iter()
                    .any(|row| matches!(row, Row::File { path, .. } if path == &b))
            );
            review.filter = ReviewFilter::Unviewed;
            review.collapsed.insert("src".into());
            review.rebuild_rows(cx);
            assert_eq!(review.matching, 2);
            assert_eq!((review.viewed, review.total), (1, 3));
            review.navigate_unviewed(false, window, cx);
            assert_eq!(review.selected.as_ref(), Some(&b));
            review.toggle_viewed(&b, cx);
            assert_eq!(
                review.selected.as_ref(),
                Some(&b),
                "Checking a filtered file must not navigate"
            );
            assert_eq!(review.matching, 1);
            review.navigate_unviewed(false, window, cx);
            assert_eq!(review.selected.as_ref(), Some(&c));
            review.navigate_unviewed(true, window, cx);
            assert_eq!(review.selected.as_ref(), Some(&c));
            review.query = "B.TXT".to_lowercase();
            review.filter = ReviewFilter::All;
            review.rebuild_rows(cx);
            assert_eq!(review.matching, 1);
            assert_eq!((review.viewed, review.total), (2, 3));
            review.query = "missing".into();
            review.rebuild_rows(cx);
            assert!(review.rows.is_empty());
            review.query.clear();
        });
        recreated_review.update(cx, |review, cx| review.set_storage_key(None, cx));
        for _ in 0..5 {
            cx.run_until_parked();
            cx.executor().advance_clock(Duration::from_millis(100));
        }
        cx.run_until_parked();
        recreated_review.read_with(cx, |review, cx| {
            assert!(
                !review.is_viewed(&a, cx),
                "PR progress must not leak into local branch review"
            )
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
