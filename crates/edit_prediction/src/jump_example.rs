use std::{
    ops::Range,
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::{Context as _, Result};
pub use cloud_api_types::JumpExampleTrigger;
use cloud_api_types::{
    JumpExampleRecentFile, SubmitEditPredictionJumpExampleBody,
    SubmitEditPredictionJumpExampleResponse,
};
use futures::future::Shared;
use gpui::{AppContext as _, AsyncApp, Context, Entity, Task, TaskExt as _, WeakEntity};
use language::{BufferSnapshot, File, Point};
use project::{Project, WorktreeId};
use release_channel::AppVersion;

use text::ToPoint as _;
use util::rel_path::RelPath;

use crate::{
    EditPredictionStore, ProjectState, StoredEvent,
    data_collection::{
        UncommittedDiffResult, compute_cursor_excerpt, compute_uncommitted_diff,
        estimate_uncommitted_diff_byte_size, format_cursor_excerpt,
    },
    example_spec::RecentFile,
    zeta,
};

pub const JUMP_EXAMPLE_MAX_PENDING_CAPTURE_COUNT: usize = 10;
pub const JUMP_EXAMPLE_FUTURE_EVENT_COUNT: usize = 2;
pub const JUMP_EXAMPLE_TTL: Duration = Duration::from_secs(60 * 2);
pub const JUMP_EXAMPLE_NAVIGATION_COUNT: usize = 20;
pub const JUMP_EXAMPLE_MAX_UNCOMMITTED_DIFF_SIZE: usize = 64 * 1024;

pub struct PendingJumpExampleCapture {
    key: PendingJumpExampleCaptureKey,
    trigger: JumpExampleTrigger,
    file: Arc<dyn File>,
    edit_history: Vec<Arc<zeta_prompt::Event>>,
    recently_opened_files: Vec<RecentFile>,
    recently_viewed_files: Vec<RecentFile>,
    worktree_root_name: String,
    cursor_position: String,
    started_at: Instant,
    uncommitted_diff: Option<String>,
    pub future_events: Vec<Arc<zeta_prompt::Event>>,
    pub navigation_history: Vec<RecentFile>,
    diagnostics: Vec<zeta_prompt::ActiveBufferDiagnostic>,
    repository_url: Option<String>,
    revision: Option<String>,
    can_collect_data: bool,
    is_in_open_source_repo: bool,
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct PendingJumpExampleCaptureKey {
    worktree_id: WorktreeId,
    file_path: Arc<RelPath>,
    row_bucket: u32,
}

pub fn try_start_jump_example_capture(
    project_state: &ProjectState,
    uncommitted_diffs: Shared<Task<UncommittedDiffResult>>,
    project: Entity<Project>,
    snapshot: BufferSnapshot,
    position: language::Anchor,
    trigger: JumpExampleTrigger,
    stored_events: Vec<StoredEvent>,
    diagnostic_search_range: Range<Point>,
    can_collect_data: bool,
    is_in_open_source_repo: bool,
    cx: &mut Context<EditPredictionStore>,
) {
    let Some(file) = snapshot.file().cloned() else {
        return;
    };

    let example_key = PendingJumpExampleCaptureKey {
        worktree_id: file.worktree_id(cx),
        file_path: file.path().clone(),
        row_bucket: position.to_point(&snapshot).row / 10,
    };
    let should_capture_example = project_state.pending_jump_example_captures.len()
        < JUMP_EXAMPLE_MAX_PENDING_CAPTURE_COUNT
        && !project_state
            .starting_jump_example_captures
            .contains(&example_key)
        && !project_state
            .pending_jump_example_captures
            .iter()
            .any(|capture| &capture.key == &example_key);

    if !should_capture_example {
        return;
    }

    let _project = project.clone();
    let _example_key = example_key.clone();
    let task = cx.spawn(async move |ep_store, cx| {
        let project = _project;
        let example_key = _example_key;
        let Some(ep_store) = ep_store.upgrade() else {
            return anyhow::Ok(());
        };
        ep_store.update(cx, |ep_store, cx| {
            let project_state = ep_store.get_or_init_project(&project, cx);
            project_state
                .starting_jump_example_captures
                .push(example_key.clone());
        });

        let (repository, worktree) = project.read_with(cx, |project, cx| {
            let repository = project.active_repository(cx);
            let worktree_id = file.worktree_id(cx);
            let worktree = project.worktree_for_id(worktree_id, cx);
            (repository, worktree)
        });
        let Some(worktree) = worktree else {
            return Ok(());
        };

        let diagnostics = zeta::active_buffer_diagnostics(
            &snapshot,
            diagnostic_search_range.clone(),
            position.to_point(&snapshot).row,
            100,
        );

        let uncommitted_diff = 'uncommitted_diff: {
            if repository.is_none() {
                break 'uncommitted_diff None;
            }
            let uncommitted_diff_snapshot = uncommitted_diffs
                .await
                .map_err(|error| anyhow::anyhow!("{error:?}"))
                .context("failed to capture uncommitted diff")?;
            let estimated_byte_size =
                estimate_uncommitted_diff_byte_size(&uncommitted_diff_snapshot);
            if estimated_byte_size > JUMP_EXAMPLE_MAX_UNCOMMITTED_DIFF_SIZE {
                break 'uncommitted_diff None;
            }

            let uncommitted_diff = cx
                .background_executor()
                .spawn(async move { compute_uncommitted_diff(uncommitted_diff_snapshot) })
                .await;
            if uncommitted_diff.len() > JUMP_EXAMPLE_MAX_UNCOMMITTED_DIFF_SIZE {
                break 'uncommitted_diff None;
            }
            Some(uncommitted_diff)
        };

        let edit_history = stored_events
            .iter()
            .map(|e| e.event.clone())
            .collect::<Vec<_>>();
        let (repository_url, revision) = if let Some(repository) = &repository {
            repository.read_with(cx, |repository, _| {
                let snapshot = repository.snapshot();
                (
                    snapshot
                        .remote_origin_url
                        .clone()
                        .or_else(|| snapshot.remote_upstream_url.clone()),
                    snapshot
                        .head_commit
                        .as_ref()
                        .map(|commit| commit.sha.to_string()),
                )
            })
        } else {
            (None, None)
        };
        let line_comment_prefix = snapshot
            .language()
            .and_then(|language| language.config().line_comments.first())
            .map(|prefix| prefix.to_string())
            .unwrap_or_default();
        let (cursor_excerpt, cursor_offset_in_excerpt, _) = cx
            .background_executor()
            .spawn(async move { compute_cursor_excerpt(&snapshot, position) })
            .await;
        let cursor_position = format_cursor_excerpt(
            &cursor_excerpt,
            cursor_offset_in_excerpt,
            &line_comment_prefix,
        );
        let now = cx.background_executor().now();
        ep_store.update(cx, |ep_store, cx| {
            let recently_opened_files = ep_store.recently_opened_files_for_project(&project);
            let recently_viewed_files = ep_store.recently_viewed_files_for_project(&project);
            let project_state = ep_store.get_or_init_project(&project, cx);
            project_state
                .pending_jump_example_captures
                .push(PendingJumpExampleCapture {
                    key: example_key,
                    trigger,
                    file,
                    uncommitted_diff,
                    edit_history,
                    recently_opened_files,
                    recently_viewed_files,
                    repository_url,
                    revision,
                    diagnostics,
                    worktree_root_name: worktree.read(cx).root_name_str().to_owned(),
                    cursor_position,
                    started_at: now,
                    future_events: Vec::new(),
                    navigation_history: Vec::new(),
                    is_in_open_source_repo,
                    can_collect_data,
                });
            drain_completed_jump_example_captures(project_state, cx);
        });
        Ok(())
    });
    cx.spawn(async move |ep_store, cx| {
        let result = task.await;
        ep_store
            .update(cx, |ep_store, cx| {
                ep_store
                    .get_or_init_project(&project, cx)
                    .starting_jump_example_captures
                    .retain(|key| key != &example_key);
            })
            .ok();
        result
    })
    .detach_and_log_err(cx);
}

pub fn drain_completed_jump_example_captures(
    project_state: &mut ProjectState,
    cx: &mut Context<EditPredictionStore>,
) {
    let now = cx.background_executor().now();

    let mut capture_index = 0;
    while capture_index < project_state.pending_jump_example_captures.len() {
        let capture = &project_state.pending_jump_example_captures[capture_index];
        let finished = capture.future_events.len() >= JUMP_EXAMPLE_FUTURE_EVENT_COUNT
            || now.saturating_duration_since(capture.started_at) >= JUMP_EXAMPLE_TTL;
        if !finished {
            capture_index += 1;
            continue;
        }

        let capture = project_state
            .pending_jump_example_captures
            .remove(capture_index);
        cx.spawn(async move |this, cx| {
            let result = submit_jump_example_capture_task(this, capture, cx).await;
            if let Err(error) = result {
                log::error!("failed to submit jump opportunity capture: {error:?}");
            }
        })
        .detach();
    }
}

fn submit_jump_example_capture_task(
    this: WeakEntity<EditPredictionStore>,
    capture: PendingJumpExampleCapture,
    cx: &mut AsyncApp,
) -> Task<Result<()>> {
    let Some((organization_id, client, llm_token, app_version)) = this
        .update(cx, |this, cx| {
            (
                this.user_store
                    .read(cx)
                    .current_organization()
                    .map(|organization| organization.id.clone()),
                this.client.clone(),
                this.llm_token.clone(),
                AppVersion::global(cx),
            )
        })
        .ok()
    else {
        return Task::ready(Ok(()));
    };
    cx.background_spawn(async move {
        let PendingJumpExampleCapture {
            key: _,
            trigger,
            file,
            edit_history,
            recently_opened_files,
            recently_viewed_files,
            worktree_root_name,
            cursor_position,
            started_at: _,
            uncommitted_diff,
            future_events,
            navigation_history,
            diagnostics,
            repository_url,
            revision,
            is_in_open_source_repo,
            can_collect_data,
        } = capture;
        let future_edit_history = render_jump_example_events(&future_events, &worktree_root_name);

        let cursor_path = file.path().as_std_path().into();
        let example = SubmitEditPredictionJumpExampleBody {
            request_id: uuid::Uuid::new_v4(),
            trigger,
            repository_url,
            revision,
            uncommitted_diff,
            recently_opened_files: jump_example_recent_files(recently_opened_files),
            recently_viewed_files: jump_example_recent_files(recently_viewed_files),
            cursor_path,
            cursor_position,
            edit_history,
            diagnostics,
            future_edit_history,
            navigation_history: jump_example_recent_files(navigation_history),
            is_in_open_source_repo,
            can_collect_data,
        };
        let json_bytes = serde_json::to_vec(&example)?;
        let compressed = zstd::encode_all(&json_bytes[..], 3)?;
        let url = client
            .http_client()
            .build_zed_llm_url("/predict_edits/jump_example", &[])?;
        EditPredictionStore::send_api_request::<SubmitEditPredictionJumpExampleResponse>(
            |builder| {
                Ok(builder
                    .uri(url.as_ref())
                    .header("Content-Encoding", "zstd")
                    .body(compressed.clone().into())?)
            },
            client,
            llm_token,
            organization_id,
            app_version,
        )
        .await?;
        Ok(())
    })
}

fn jump_example_recent_files(files: Vec<RecentFile>) -> Vec<JumpExampleRecentFile> {
    files
        .into_iter()
        .map(|file| JumpExampleRecentFile {
            path: file.path,
            cursor_position: file.cursor_position,
        })
        .collect()
}

fn render_jump_example_events(events: &[Arc<zeta_prompt::Event>], root_name: &str) -> String {
    let mut edit_history = String::new();
    for event in events {
        crate::capture_example::write_event_with_relative_paths(
            &mut edit_history,
            event,
            root_name,
        );
        if !edit_history.ends_with('\n') {
            edit_history.push('\n');
        }
    }
    edit_history
}
