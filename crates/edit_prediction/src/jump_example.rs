use std::{
    ops::Range,
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::{Context as _, Result};
pub use cloud_api_types::JumpExampleTrigger;
use cloud_api_types::{
    JumpExampleRecentFile, JumpExampleSpec, SubmitJumpExampleBody, SubmitJumpExampleResponse,
};
use gpui::{AppContext as _, AsyncApp, Context, Entity, Task, TaskExt as _, WeakEntity};
use language::{BufferSnapshot, File, Point};
use project::Project;
use release_channel::AppVersion;

use text::ToPoint as _;

use crate::{EditPredictionStore, ProjectState, StoredEvent, example_spec::RecentFile, zeta};

pub const JUMP_EXAMPLE_FUTURE_EVENT_COUNT: usize = 2;
pub const JUMP_EXAMPLE_TTL: Duration = Duration::from_secs(60 * 2);
pub const JUMP_EXAMPLE_NAVIGATION_COUNT: usize = 20;

pub struct PendingJumpExampleCapture {
    trigger: JumpExampleTrigger,
    file: Arc<dyn File>,
    edit_history: Vec<Arc<zeta_prompt::Event>>,
    recently_opened_files: Vec<RecentFile>,
    recently_viewed_files: Vec<RecentFile>,
    worktree_root_name: String,
    started_at: Instant,
    uncommitted_diff: String,
    pub future_events: Vec<Arc<zeta_prompt::Event>>,
    pub navigation_history: Vec<RecentFile>,
    diagnostics: Vec<zeta_prompt::ActiveBufferDiagnostic>,
    repository_url: Option<String>,
    revision: Option<String>,
}

pub fn start_jump_example_capture(
    project: Entity<Project>,
    snapshot: BufferSnapshot,
    position: language::Anchor,
    trigger: JumpExampleTrigger,
    stored_events: Vec<StoredEvent>,
    diagnostic_search_range: Range<Point>,
    cx: &mut Context<EditPredictionStore>,
) {
    cx.spawn(async move |ep_store, cx| {
        let Some(file) = snapshot.file().cloned() else {
            return anyhow::Ok(());
        };
        let Some(ep_store) = ep_store.upgrade() else {
            return anyhow::Ok(());
        };
        let (repository, worktree_id, worktree_info) = project.read_with(cx, |project, cx| {
            let repository = project.active_repository(cx);
            let worktree_id = file.worktree_id(cx);
            let worktree = project.worktree_for_id(worktree_id, cx);
            let worktree_info = worktree.map(|worktree| {
                (
                    worktree.read_with(cx, |worktree, _| {
                        worktree.root_name().as_unix_str().to_string()
                    }),
                    worktree,
                )
            });
            (repository, worktree_id, worktree_info)
        });
        let Some((worktree_root_name, worktree)) = worktree_info else {
            return Ok(());
        };

        let diagnostics = zeta::active_buffer_diagnostics(
            &snapshot,
            diagnostic_search_range.clone(),
            position.to_point(&snapshot).row,
            100,
        );

        let (uncommitted_diff, edit_history_events) = if repository.is_some() {
            let uncommitted_diffs = ep_store
                .update(cx, |ep_store, cx| {
                    // todo! this just cx.spawns. Better api
                    ep_store.uncommitted_diffs_for_events(
                        project.clone(),
                        worktree_id,
                        stored_events.clone(),
                        cx,
                    )
                })
                .await
                .context("failed to get uncommitted diffs for events")?;
            // todo! this calls background_spawn, should just return DTO
            // todo! why does this return events?
            crate::capture_example::uncommitted_diff_for_events(
                project.clone(),
                worktree_id,
                worktree_root_name.clone(),
                stored_events,
                uncommitted_diffs,
                cx,
            )
            .await?
        } else {
            (String::new(), stored_events.clone())
        };

        let edit_history = edit_history_events
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
        let now = cx.background_executor().now();
        ep_store.update(cx, |ep_store, cx| {
            let (recently_opened_files, recently_viewed_files) =
                ep_store.recent_paths_for_project(&project, cx);
            let project_state = ep_store.get_or_init_project(&project, cx);
            project_state
                .pending_jump_example_captures
                .push(PendingJumpExampleCapture {
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
                    started_at: now,
                    future_events: Vec::new(),
                    navigation_history: Vec::new(),
                });
            drain_completed_jump_example_captures(project_state, cx);
        });
        Ok(())
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
            trigger,
            file,
            edit_history,
            recently_opened_files,
            recently_viewed_files,
            worktree_root_name,
            started_at: _,
            uncommitted_diff,
            future_events,
            navigation_history,
            diagnostics,
            repository_url,
            revision,
        } = capture;
        let future_edit_history = render_jump_example_events(&future_events, &worktree_root_name);

        let cursor_path = file.path().as_std_path().into();
        let example = JumpExampleSpec {
            capture_id: uuid::Uuid::new_v4(),
            trigger,
            repository_url,
            revision,
            uncommitted_diff,
            recently_opened_files: jump_example_recent_files(recently_opened_files),
            recently_viewed_files: jump_example_recent_files(recently_viewed_files),
            cursor_path,
            // todo! cursor excerpt like in zeta prompt input
            cursor_position: String::new(),
            edit_history,
            diagnostics,
            future_edit_history,
            navigation_history: jump_example_recent_files(navigation_history),
        };
        let body = SubmitJumpExampleBody { example };
        let json_bytes = serde_json::to_vec(&body)?;
        let compressed = zstd::encode_all(&json_bytes[..], 3)?;
        let url = client
            .http_client()
            .build_zed_llm_url("/predict_edits/jump_example", &[])?;
        EditPredictionStore::send_api_request::<SubmitJumpExampleResponse>(
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
