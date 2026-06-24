use crate::{
    EditPredictionStore, StoredEvent,
    data_collection::{compute_uncommitted_diff, uncommitted_diffs_for_events},
    zeta,
};
use anyhow::Result;
use gpui::{Context, Entity, Task};
use language::{Buffer, Point, ToPoint as _};
use project::Project;
use text::OffsetRangeExt as _;

const MAX_UNCOMMITTED_DIFF_SIZE: usize = 64 * 1024;

pub(crate) struct CapturedPredictionContext {
    pub(crate) repository_url: Option<String>,
    pub(crate) revision: Option<String>,
    pub(crate) uncommitted_diff: Option<String>,
    pub(crate) buffer_diagnostics: Vec<zeta_prompt::ActiveBufferDiagnostic>,
    pub(crate) editable_context: Vec<zeta_prompt::RelatedFile>,
}

pub(crate) fn capture_prediction_context(
    project: Entity<Project>,
    buffer: Entity<Buffer>,
    cursor_anchor: language::Anchor,
    stored_events: Vec<StoredEvent>,
    repository_url: Option<String>,
    revision: Option<String>,
    editable_context_task: Task<Result<Vec<zeta_prompt::RelatedFile>>>,
    cx: &mut Context<EditPredictionStore>,
) -> Option<Task<Result<CapturedPredictionContext>>> {
    let snapshot = buffer.read(cx).snapshot();
    let worktree_id = snapshot.file()?.worktree_id(cx);
    let uncommitted_diff_task =
        uncommitted_diffs_for_events(project, worktree_id, stored_events, cx);

    Some(cx.spawn(async move |_this, cx| {
        let uncommitted_diff_snapshot = match uncommitted_diff_task.await {
            Ok(snapshot) => Some(snapshot),
            Err(error) => {
                log::debug!("failed to capture uncommitted diff: {error:?}");
                None
            }
        };

        let uncommitted_diff = if let Some(uncommitted_diff_snapshot) = uncommitted_diff_snapshot {
            let estimated_uncommitted_diff_size = uncommitted_diff_snapshot
                .iter()
                .map(|(_, buffer_snapshot, diff_snapshot)| {
                    diff_snapshot
                        .hunks(buffer_snapshot)
                        .map(|hunk| {
                            hunk.diff_base_byte_range.len()
                                + hunk.range.to_offset(buffer_snapshot).len()
                        })
                        .sum::<usize>()
                })
                .sum::<usize>();

            if estimated_uncommitted_diff_size <= MAX_UNCOMMITTED_DIFF_SIZE {
                let uncommitted_diff = cx
                    .background_executor()
                    .spawn(async move { compute_uncommitted_diff(uncommitted_diff_snapshot) })
                    .await;
                (uncommitted_diff.len() <= MAX_UNCOMMITTED_DIFF_SIZE).then_some(uncommitted_diff)
            } else {
                None
            }
        } else {
            None
        };

        let buffer_diagnostics = zeta::active_buffer_diagnostics(
            &snapshot,
            Point::new(0, 0)..snapshot.max_point(),
            cursor_anchor.to_point(&snapshot).row,
            100,
        );
        let editable_context = match editable_context_task.await {
            Ok(editable_context) => editable_context,
            Err(error) => {
                log::debug!("failed to capture editable context: {error:?}");
                Vec::new()
            }
        };

        Ok(CapturedPredictionContext {
            repository_url,
            revision,
            uncommitted_diff,
            buffer_diagnostics,
            editable_context,
        })
    }))
}
