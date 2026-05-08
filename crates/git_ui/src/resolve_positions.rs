use anyhow::{Context as _, Result};
use buffer_diff::{BufferDiff, BufferDiffSnapshot};
use editor::{
    Bias, Editor, EditorEvent, SelectionEffects,
    scroll::{Autoscroll, AutoscrollStrategy},
};
use futures::AsyncReadExt as _;
use git::repository::RepoPath;
use gpui::{App, AsyncWindowContext, Context, Entity, Subscription, Task, WeakEntity, Window};
use http_client::{AsyncBody, Method, Request};
use language::{Buffer, DiskState, ToOffset as _};
use project::git_store::Repository;
use std::{ops::Range, time::Duration};
use util::ResultExt as _;
use workspace::{
    ItemHandle, SplitDirection, Workspace, item::ItemEvent, notifications::NotifyTaskExt,
};

#[derive(Clone)]
pub struct ResolvePositionsContext {
    pub commit_editor: Entity<Editor>,
    pub commit_sha: String,
    pub repository: Entity<Repository>,
    pub workspace: WeakEntity<Workspace>,
}

pub struct ResolvePositionsController {
    context: ResolvePositionsContext,
    current_editor: Entity<Editor>,
    current_buffer: Entity<Buffer>,
    repo_path: RepoPath,
    target_commit: String,
    uncommitted_diff: Option<Entity<BufferDiff>>,
    debounce_task: Option<Task<()>>,
    _subscriptions: Vec<Subscription>,
}

#[derive(Clone, Copy)]
enum ResolveDirection {
    CommitToCurrent,
    CurrentToCommit,
}

#[derive(Clone)]
struct ResolveSelectionRequest {
    source_commit: String,
    target_commit: String,
    path: String,
    range: Range<usize>,
    target_editor: Entity<Editor>,
    target_buffer: Entity<Buffer>,
    uncommitted_diff: Option<Entity<BufferDiff>>,
    direction: ResolveDirection,
}

#[derive(serde::Serialize)]
struct DeltaResolvePositionsRequest {
    source_commit: String,
    target_commit: String,
    positions: Vec<DeltaPositionRequest>,
}

#[derive(serde::Serialize)]
struct DeltaPositionRequest {
    path: String,
    offset: usize,
    bias: DeltaAnchorBias,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
enum DeltaAnchorBias {
    Left,
    Right,
}

#[derive(serde::Deserialize)]
struct DeltaResolvePositionsResponse {
    positions: Option<Vec<DeltaResolvedPosition>>,
}

#[derive(serde::Deserialize)]
struct DeltaResolvedPosition {
    offset: usize,
}

impl ResolvePositionsController {
    pub fn start<T: 'static>(
        context: ResolvePositionsContext,
        window: &mut Window,
        cx: &mut Context<T>,
        controller_slot: fn(&mut T) -> &mut Option<Self>,
    ) {
        let Some(file) = context
            .commit_editor
            .read(cx)
            .active_buffer(cx)
            .and_then(|buffer| buffer.read(cx).file().cloned())
        else {
            return;
        };
        if !matches!(
            file.disk_state(),
            DiskState::Historic { was_deleted: false }
        ) {
            return;
        }

        let rel_path = file.path().clone();
        let worktree_id = file.worktree_id(cx);
        let repo_path = RepoPath::from_rel_path(&rel_path);
        let project_path = context
            .repository
            .read(cx)
            .repo_path_to_project_path(&repo_path, cx)
            .unwrap_or(project::ProjectPath {
                worktree_id,
                path: rel_path,
            });

        let Some(workspace) = context.workspace.upgrade() else {
            return;
        };
        if workspace
            .read(cx)
            .project()
            .read(cx)
            .entry_for_path(&project_path, cx)
            .is_none()
        {
            return;
        }

        let Some(target_commit) = context
            .repository
            .read(cx)
            .snapshot()
            .head_commit
            .as_ref()
            .map(|commit| commit.sha.to_string())
        else {
            return;
        };

        let open_task = workspace.update(cx, |workspace, cx| {
            let active_pane_id = workspace.active_pane().entity_id();
            let target_pane = workspace
                .panes()
                .iter()
                .find(|pane| pane.entity_id() != active_pane_id)
                .map(|pane| pane.downgrade());
            if let Some(target_pane) = target_pane {
                workspace.open_path_preview(
                    project_path,
                    Some(target_pane),
                    true,
                    false,
                    true,
                    window,
                    cx,
                )
            } else {
                workspace.split_path_preview(
                    project_path,
                    false,
                    Some(SplitDirection::vertical(cx)),
                    window,
                    cx,
                )
            }
        });
        let parent = cx.weak_entity();
        let workspace_weak = context.workspace.clone();
        window
            .spawn(cx, async move |cx| {
                let current_item = open_task.await?;
                let current_editor = cx
                    .update(|_, cx| current_item.act_as::<Editor>(cx))?
                    .context("opened item is not an editor")?;
                let current_buffer = cx
                    .update(|_, cx| current_editor.read(cx).active_buffer(cx))?
                    .context("opened editor has no active buffer")?;
                let uncommitted_diff = cx
                    .update(|_, cx| {
                        context.workspace.upgrade().map(|workspace| {
                            let project = workspace.read(cx).project().clone();
                            let git_store = project.read(cx).git_store().clone();
                            git_store.update(cx, |git_store, cx| {
                                git_store.open_uncommitted_diff(current_buffer.clone(), cx)
                            })
                        })
                    })?
                    .map(|task| async { task.await.log_err() });
                let uncommitted_diff = match uncommitted_diff {
                    Some(task) => task.await,
                    None => None,
                };

                parent.update_in(cx, |parent, window, cx| {
                    let controller = Self::new(
                        context,
                        current_editor,
                        current_buffer,
                        repo_path,
                        target_commit,
                        uncommitted_diff,
                        current_item,
                        controller_slot,
                        window,
                        cx,
                    );
                    *controller_slot(parent) = Some(controller);
                    if let Some(controller) = controller_slot(parent).as_mut() {
                        controller.schedule_resolve_selection(
                            ResolveDirection::CommitToCurrent,
                            controller_slot,
                            window,
                            cx,
                        );
                    }
                    cx.notify();
                })?;
                anyhow::Ok(())
            })
            .detach_and_notify_err(workspace_weak, window, cx);
    }

    #[allow(clippy::too_many_arguments)]
    fn new<T: 'static>(
        context: ResolvePositionsContext,
        current_editor: Entity<Editor>,
        current_buffer: Entity<Buffer>,
        repo_path: RepoPath,
        target_commit: String,
        uncommitted_diff: Option<Entity<BufferDiff>>,
        current_item: Box<dyn ItemHandle>,
        controller_slot: fn(&mut T) -> &mut Option<Self>,
        window: &mut Window,
        cx: &mut Context<T>,
    ) -> Self {
        let mut subscriptions = Vec::new();
        subscriptions.push(cx.subscribe_in(
            &context.commit_editor,
            window,
            move |parent, editor, event, window, cx| {
                if matches!(event, EditorEvent::SelectionsChanged { local: true })
                    && let Some(controller) = controller_slot(parent).as_mut()
                {
                    if editor.read(cx).is_focused(window) {
                        controller.schedule_resolve_selection(
                            ResolveDirection::CommitToCurrent,
                            controller_slot,
                            window,
                            cx,
                        );
                    }
                }
            },
        ));
        subscriptions.push(cx.subscribe_in(
            &current_editor,
            window,
            move |parent, editor, event, window, cx| {
                if matches!(event, EditorEvent::SelectionsChanged { local: true })
                    && let Some(controller) = controller_slot(parent).as_mut()
                {
                    if editor.read(cx).is_focused(window) {
                        controller.schedule_resolve_selection(
                            ResolveDirection::CurrentToCommit,
                            controller_slot,
                            window,
                            cx,
                        );
                    }
                }
            },
        ));
        let parent = cx.weak_entity();
        subscriptions.push(current_item.subscribe_to_item_events(
            window,
            cx,
            Box::new(move |event, _, cx| {
                if matches!(event, ItemEvent::CloseItem) {
                    parent
                        .update(cx, |parent, cx| {
                            *controller_slot(parent) = None;
                            cx.notify();
                        })
                        .log_err();
                }
            }),
        ));

        Self {
            context,
            current_editor,
            current_buffer,
            repo_path,
            target_commit,
            uncommitted_diff,
            debounce_task: None,
            _subscriptions: subscriptions,
        }
    }

    fn schedule_resolve_selection<T: 'static>(
        &mut self,
        direction: ResolveDirection,
        controller_slot: fn(&mut T) -> &mut Option<Self>,
        window: &mut Window,
        cx: &mut Context<T>,
    ) {
        let Some(request) = self.resolve_selection_request(direction, cx) else {
            return;
        };
        let Some(repository_url) = self.delta_repository_url(cx) else {
            return;
        };
        let parent: WeakEntity<T> = cx.weak_entity();
        self.debounce_task = Some(window.spawn(cx, async move |cx| {
            cx.background_executor()
                .timer(Duration::from_millis(100))
                .await;
            match resolve_positions_via_delta(repository_url, request.clone(), cx).await {
                Ok(range) => {
                    parent
                        .update_in(cx, |parent, window, cx| {
                            if let Some(controller) = controller_slot(parent).as_mut() {
                                controller.apply_resolved_selection(request, range, window, cx);
                            }
                        })
                        .log_err();
                }
                Err(error) => log::warn!("failed to resolve selection via DeltaDB: {error:#}"),
            }
        }));
    }

    fn resolve_selection_request(
        &self,
        direction: ResolveDirection,
        cx: &App,
    ) -> Option<ResolveSelectionRequest> {
        let path = self.repo_path.as_unix_str().to_string();
        let source_commit = match direction {
            ResolveDirection::CommitToCurrent => self.context.commit_sha.clone(),
            ResolveDirection::CurrentToCommit => self.target_commit.clone(),
        };
        let target_commit = match direction {
            ResolveDirection::CommitToCurrent => self.target_commit.clone(),
            ResolveDirection::CurrentToCommit => self.context.commit_sha.clone(),
        };
        let source_editor = match direction {
            ResolveDirection::CommitToCurrent => self.context.commit_editor.clone(),
            ResolveDirection::CurrentToCommit => self.current_editor.clone(),
        };
        let target_editor = match direction {
            ResolveDirection::CommitToCurrent => self.current_editor.clone(),
            ResolveDirection::CurrentToCommit => self.context.commit_editor.clone(),
        };
        let target_buffer = match direction {
            ResolveDirection::CommitToCurrent => self.current_buffer.clone(),
            ResolveDirection::CurrentToCommit => self
                .context
                .commit_editor
                .read(cx)
                .active_buffer(cx)
                .filter(|buffer| {
                    buffer
                        .read(cx)
                        .file()
                        .is_some_and(|file| **file.path() == *self.repo_path)
                })?,
        };
        let mut range = editor_selection_buffer_range(&source_editor, &self.repo_path, cx)?;
        if matches!(direction, ResolveDirection::CurrentToCommit) {
            range = adjust_working_copy_range_to_head(
                range,
                &self.current_buffer,
                self.uncommitted_diff.as_ref(),
                cx,
            );
        }

        Some(ResolveSelectionRequest {
            source_commit,
            target_commit,
            path,
            range,
            target_editor,
            target_buffer,
            uncommitted_diff: self.uncommitted_diff.clone(),
            direction,
        })
    }

    fn delta_repository_url<T>(&self, cx: &Context<T>) -> Option<String> {
        let snapshot = self.context.repository.read(cx).snapshot();
        let remote_url = snapshot
            .remote_upstream_url
            .as_ref()
            .or(snapshot.remote_origin_url.as_ref())?;
        Some(format!(
            "http://localhost:9292/repository3/{}",
            percent_encode(remote_url)
        ))
    }

    fn apply_resolved_selection<T>(
        &mut self,
        request: ResolveSelectionRequest,
        mut range: Range<usize>,
        window: &mut Window,
        cx: &mut Context<T>,
    ) {
        if matches!(request.direction, ResolveDirection::CommitToCurrent) {
            range = adjust_head_range_to_working_copy(
                range,
                &request.target_buffer,
                request.uncommitted_diff.as_ref(),
                cx,
            );
        }

        let snapshot = request.target_buffer.read(cx).snapshot();
        let range = snapshot.anchor_after(snapshot.clip_offset(range.start, Bias::Right))
            ..snapshot.anchor_before(snapshot.clip_offset(range.end, Bias::Left));

        request.target_editor.update(cx, |editor, cx| {
            if let Some(anchor_range) = editor
                .buffer()
                .read(cx)
                .snapshot(cx)
                .buffer_anchor_range_to_anchor_range(range)
            {
                editor.change_selections(
                    SelectionEffects::scroll(Autoscroll::Strategy(
                        AutoscrollStrategy::Center,
                        None,
                    )),
                    window,
                    cx,
                    |selections| selections.select_anchor_ranges([anchor_range]),
                );
            }
        });
    }
}

fn editor_selection_buffer_range(
    editor: &Entity<Editor>,
    repo_path: &RepoPath,
    cx: &App,
) -> Option<Range<usize>> {
    let multibuffer_snapshot = editor.read(cx).buffer().read(cx).snapshot(cx);
    let selection = editor.read(cx).selections.newest_anchor();
    let (start, start_buffer) = multibuffer_snapshot.anchor_to_buffer_anchor(selection.start)?;
    let (end, end_buffer) = multibuffer_snapshot.anchor_to_buffer_anchor(selection.end)?;
    if start_buffer.remote_id() != end_buffer.remote_id() {
        return None;
    }
    let buffer = editor.read(cx).active_buffer(cx)?;
    let buffer = buffer.read(cx);
    if buffer.remote_id() != start_buffer.remote_id() {
        return None;
    }
    if !buffer
        .file()
        .is_some_and(|file| **file.path() == **repo_path)
    {
        return None;
    }
    let start = start.to_offset(&start_buffer);
    let end = end.to_offset(&start_buffer);
    Some(start.min(end)..start.max(end))
}

fn adjust_head_range_to_working_copy(
    range: Range<usize>,
    buffer: &Entity<Buffer>,
    uncommitted_diff: Option<&Entity<BufferDiff>>,
    cx: &App,
) -> Range<usize> {
    let Some(uncommitted_diff) = uncommitted_diff else {
        return range;
    };
    let diff_snapshot = uncommitted_diff.read(cx).snapshot(cx);
    let buffer_snapshot = buffer.read(cx).snapshot();
    adjust_base_offset_to_buffer(range.start, &diff_snapshot, &buffer_snapshot)
        ..adjust_base_offset_to_buffer(range.end, &diff_snapshot, &buffer_snapshot)
}

fn adjust_working_copy_range_to_head(
    range: Range<usize>,
    buffer: &Entity<Buffer>,
    uncommitted_diff: Option<&Entity<BufferDiff>>,
    cx: &App,
) -> Range<usize> {
    let Some(uncommitted_diff) = uncommitted_diff else {
        return range;
    };
    let diff_snapshot = uncommitted_diff.read(cx).snapshot(cx);
    let buffer_snapshot = buffer.read(cx).snapshot();
    adjust_buffer_offset_to_base(range.start, &diff_snapshot, &buffer_snapshot)
        ..adjust_buffer_offset_to_base(range.end, &diff_snapshot, &buffer_snapshot)
}

fn adjust_base_offset_to_buffer(
    offset: usize,
    diff_snapshot: &BufferDiffSnapshot,
    buffer_snapshot: &language::BufferSnapshot,
) -> usize {
    let mut offset_delta = 0isize;
    for hunk in diff_snapshot.hunks(buffer_snapshot) {
        let base_start = hunk.diff_base_byte_range.start;
        let base_end = hunk.diff_base_byte_range.end;
        let buffer_start = hunk.buffer_range.start.to_offset(buffer_snapshot);
        let buffer_end = hunk.buffer_range.end.to_offset(buffer_snapshot);
        if offset < base_start {
            return apply_offset_delta(offset, offset_delta);
        }
        if offset <= base_end {
            let buffer_extent = buffer_end.saturating_sub(buffer_start);
            return buffer_start + offset.saturating_sub(base_start).min(buffer_extent);
        }
        offset_delta += buffer_end as isize - base_end as isize;
    }
    apply_offset_delta(offset, offset_delta)
}

fn adjust_buffer_offset_to_base(
    offset: usize,
    diff_snapshot: &BufferDiffSnapshot,
    buffer_snapshot: &language::BufferSnapshot,
) -> usize {
    let mut offset_delta = 0isize;
    for hunk in diff_snapshot.hunks(buffer_snapshot) {
        let base_start = hunk.diff_base_byte_range.start;
        let base_end = hunk.diff_base_byte_range.end;
        let buffer_start = hunk.buffer_range.start.to_offset(buffer_snapshot);
        let buffer_end = hunk.buffer_range.end.to_offset(buffer_snapshot);
        if offset < buffer_start {
            return apply_offset_delta(offset, -offset_delta);
        }
        if offset <= buffer_end {
            let base_extent = base_end.saturating_sub(base_start);
            return base_start + offset.saturating_sub(buffer_start).min(base_extent);
        }
        offset_delta += buffer_end as isize - base_end as isize;
    }
    apply_offset_delta(offset, -offset_delta)
}

fn apply_offset_delta(offset: usize, delta: isize) -> usize {
    if delta.is_negative() {
        offset.saturating_sub(delta.unsigned_abs())
    } else {
        offset.saturating_add(delta as usize)
    }
}

async fn resolve_positions_via_delta(
    repository_url: String,
    request: ResolveSelectionRequest,
    cx: &mut AsyncWindowContext,
) -> Result<Range<usize>> {
    let http_client = cx.update(|_, cx| cx.http_client())?;
    let body = serde_json::to_string(&DeltaResolvePositionsRequest {
        source_commit: request.source_commit,
        target_commit: request.target_commit,
        positions: vec![
            DeltaPositionRequest {
                path: request.path.clone(),
                offset: request.range.start,
                bias: DeltaAnchorBias::Left,
            },
            DeltaPositionRequest {
                path: request.path,
                offset: request.range.end,
                bias: DeltaAnchorBias::Right,
            },
        ],
    })?;
    let http_request = Request::builder()
        .method(Method::POST)
        .uri(format!("{repository_url}/translate-positions"))
        .header("content-type", "application/json")
        .body(AsyncBody::from(body))?;
    let mut response = http_client.send(http_request).await?;
    if !response.status().is_success() {
        anyhow::bail!(
            "Delta resolve-positions failed with status {}",
            response.status()
        );
    }
    let mut response_body = String::new();
    response
        .body_mut()
        .read_to_string(&mut response_body)
        .await?;
    let response: DeltaResolvePositionsResponse = serde_json::from_str(&response_body)?;
    let positions = response.positions.context("commits are not related")?;
    let start = positions.first().context("missing resolved start")?.offset;
    let end = positions.get(1).context("missing resolved end")?.offset;
    Ok(start.min(end)..start.max(end))
}

fn percent_encode(input: &str) -> String {
    let mut encoded = String::new();
    for byte in input.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
            encoded.push(byte as char);
        } else {
            encoded.push_str(&format!("%{byte:02X}"));
        }
    }
    encoded
}
