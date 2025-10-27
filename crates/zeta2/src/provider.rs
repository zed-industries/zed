use std::{
    cmp,
    sync::Arc,
    time::{Duration, Instant},
};

use arrayvec::ArrayVec;
use client::{Client, UserStore};
use edit_prediction::{DataCollectionState, Direction, EditPredictionProvider};
use gpui::{App, Entity, Task, prelude::*};
use language::ToPoint as _;
use project::Project;
use util::ResultExt as _;

use crate::{
    BufferEditPrediction, ContextMode, Zeta,
    related_excerpts::{RelatedExcerpt, find_related_excerpts},
};

pub struct ZetaEditPredictionProvider {
    zeta: Entity<Zeta>,
    next_pending_prediction_id: usize,
    pending_predictions: ArrayVec<PendingPrediction, 2>,
    last_request_timestamp: Instant,
    project: Entity<Project>,
    context: Option<Vec<RelatedExcerpt>>,
    refresh_context_task: Option<Task<Option<()>>>,
    refresh_context_debounce_task: Option<Task<Option<()>>>,
    refresh_context_timestamp: Option<Instant>,
}

impl ZetaEditPredictionProvider {
    pub const THROTTLE_TIMEOUT: Duration = Duration::from_millis(300);
    pub const CONTEXT_RETRIEVAL_DEBOUNCE_DURATION: Duration = Duration::from_secs(3);
    pub const CONTEXT_RETRIEVAL_IDLE_DURATION: Duration = Duration::from_secs(10);

    pub fn new(
        project: Entity<Project>,
        client: &Arc<Client>,
        user_store: &Entity<UserStore>,
        cx: &mut App,
    ) -> Self {
        let zeta = Zeta::global(client, user_store, cx);
        zeta.update(cx, |zeta, cx| {
            zeta.register_project(&project, cx);
        });

        Self {
            zeta,
            next_pending_prediction_id: 0,
            pending_predictions: ArrayVec::new(),
            last_request_timestamp: Instant::now(),
            project: project,
            refresh_context_task: None,
            refresh_context_debounce_task: None,
            refresh_context_timestamp: None,
            context: None,
        }
    }

    // Refresh the related excerpts when the user just beguns editing after
    // an idle period, and after they pause editing.
    fn refresh_context_if_needed(
        &mut self,
        buffer: &Entity<language::Buffer>,
        cursor_position: language::Anchor,
        cx: &mut Context<'_, ZetaEditPredictionProvider>,
    ) {
        if !matches!(
            &self.zeta.read(cx).options().context,
            ContextMode::Llm { .. }
        ) {
            return;
        }

        let now = Instant::now();
        let was_idle = self.refresh_context_timestamp.map_or(true, |timestamp| {
            now - timestamp > Self::CONTEXT_RETRIEVAL_IDLE_DURATION
        });
        self.refresh_context_timestamp = Some(now);
        self.refresh_context_debounce_task = Some(cx.spawn({
            let buffer = buffer.clone();
            async move |this, cx| {
                if was_idle {
                    log::debug!("refetching edit prediction context after idle");
                } else {
                    cx.background_executor()
                        .timer(Self::CONTEXT_RETRIEVAL_DEBOUNCE_DURATION)
                        .await;
                    log::debug!("refetching edit prediction context after pause");
                }
                this.update(cx, |this, cx| {
                    this.refresh_context(buffer, cursor_position, cx);
                })
                .ok()
            }
        }));
    }

    // Refresh the related excerpts asynchronously. Ensure the task runs to completion,
    // and avoid spawning more than one concurrent task.
    fn refresh_context(
        &mut self,
        buffer: Entity<language::Buffer>,
        cursor_position: language::Anchor,
        cx: &mut Context<Self>,
    ) {
        self.refresh_context_task
            .get_or_insert(cx.spawn(async move |this, cx| {
                let related_excerpts = this
                    .update(cx, |this, cx| {
                        let zeta = this.zeta.read(cx);
                        let ContextMode::Llm(options) = &zeta.options().context else {
                            return Task::ready(anyhow::Ok(vec![]));
                        };

                        find_related_excerpts(
                            buffer.clone(),
                            cursor_position,
                            &this.project,
                            zeta.history_for_project(&this.project),
                            options,
                            cx,
                        )
                    })
                    .ok()?
                    .await
                    .log_err()
                    .unwrap_or_default();
                this.update(cx, |this, _cx| {
                    this.context = Some(related_excerpts);
                    this.refresh_context_task.take();
                })
                .ok()
            }));
    }
}

struct PendingPrediction {
    id: usize,
    _task: Task<()>,
}

impl EditPredictionProvider for ZetaEditPredictionProvider {
    fn name() -> &'static str {
        "zed-predict2"
    }

    fn display_name() -> &'static str {
        "Zed's Edit Predictions 2"
    }

    fn show_completions_in_menu() -> bool {
        true
    }

    fn show_tab_accept_marker() -> bool {
        true
    }

    fn data_collection_state(&self, _cx: &App) -> DataCollectionState {
        // TODO [zeta2]
        DataCollectionState::Unsupported
    }

    fn toggle_data_collection(&mut self, _cx: &mut App) {
        // TODO [zeta2]
    }

    fn usage(&self, cx: &App) -> Option<client::EditPredictionUsage> {
        self.zeta.read(cx).usage(cx)
    }

    fn is_enabled(
        &self,
        _buffer: &Entity<language::Buffer>,
        _cursor_position: language::Anchor,
        _cx: &App,
    ) -> bool {
        true
    }

    fn is_refreshing(&self) -> bool {
        !self.pending_predictions.is_empty()
    }

    fn refresh(
        &mut self,
        buffer: Entity<language::Buffer>,
        cursor_position: language::Anchor,
        _debounce: bool,
        cx: &mut Context<Self>,
    ) {
        let zeta = self.zeta.read(cx);

        if zeta.user_store.read_with(cx, |user_store, _cx| {
            user_store.account_too_young() || user_store.has_overdue_invoices()
        }) {
            return;
        }

        if let Some(current) = zeta.current_prediction_for_buffer(&buffer, &self.project, cx)
            && let BufferEditPrediction::Local { prediction } = current
            && prediction.interpolate(buffer.read(cx)).is_some()
        {
            return;
        }

        self.refresh_context_if_needed(&buffer, cursor_position, cx);

        let pending_prediction_id = self.next_pending_prediction_id;
        self.next_pending_prediction_id += 1;
        let last_request_timestamp = self.last_request_timestamp;

        let project = self.project.clone();
        let task = cx.spawn(async move |this, cx| {
            if let Some(timeout) = (last_request_timestamp + Self::THROTTLE_TIMEOUT)
                .checked_duration_since(Instant::now())
            {
                cx.background_executor().timer(timeout).await;
            }

            let refresh_task = this.update(cx, |this, cx| {
                this.last_request_timestamp = Instant::now();
                this.zeta.update(cx, |zeta, cx| {
                    zeta.refresh_prediction(&project, &buffer, cursor_position, cx)
                })
            });

            if let Some(refresh_task) = refresh_task.ok() {
                refresh_task.await.log_err();
            }

            this.update(cx, |this, cx| {
                if this.pending_predictions[0].id == pending_prediction_id {
                    this.pending_predictions.remove(0);
                } else {
                    this.pending_predictions.clear();
                }

                cx.notify();
            })
            .ok();
        });

        // We always maintain at most two pending predictions. When we already
        // have two, we replace the newest one.
        if self.pending_predictions.len() <= 1 {
            self.pending_predictions.push(PendingPrediction {
                id: pending_prediction_id,
                _task: task,
            });
        } else if self.pending_predictions.len() == 2 {
            self.pending_predictions.pop();
            self.pending_predictions.push(PendingPrediction {
                id: pending_prediction_id,
                _task: task,
            });
        }

        cx.notify();
    }

    fn cycle(
        &mut self,
        _buffer: Entity<language::Buffer>,
        _cursor_position: language::Anchor,
        _direction: Direction,
        _cx: &mut Context<Self>,
    ) {
    }

    fn accept(&mut self, cx: &mut Context<Self>) {
        self.zeta.update(cx, |zeta, cx| {
            zeta.accept_current_prediction(&self.project, cx);
        });
        self.pending_predictions.clear();
    }

    fn discard(&mut self, cx: &mut Context<Self>) {
        self.zeta.update(cx, |zeta, _cx| {
            zeta.discard_current_prediction(&self.project);
        });
        self.pending_predictions.clear();
    }

    fn suggest(
        &mut self,
        buffer: &Entity<language::Buffer>,
        cursor_position: language::Anchor,
        cx: &mut Context<Self>,
    ) -> Option<edit_prediction::EditPrediction> {
        let prediction =
            self.zeta
                .read(cx)
                .current_prediction_for_buffer(buffer, &self.project, cx)?;

        let prediction = match prediction {
            BufferEditPrediction::Local { prediction } => prediction,
            BufferEditPrediction::Jump { prediction } => {
                return Some(edit_prediction::EditPrediction::Jump {
                    id: Some(prediction.id.to_string().into()),
                    snapshot: prediction.snapshot.clone(),
                    target: prediction.edits.first().unwrap().0.start,
                });
            }
        };

        let buffer = buffer.read(cx);
        let snapshot = buffer.snapshot();

        let Some(edits) = prediction.interpolate(&snapshot) else {
            self.zeta.update(cx, |zeta, _cx| {
                zeta.discard_current_prediction(&self.project);
            });
            return None;
        };

        let cursor_row = cursor_position.to_point(&snapshot).row;
        let (closest_edit_ix, (closest_edit_range, _)) =
            edits.iter().enumerate().min_by_key(|(_, (range, _))| {
                let distance_from_start = cursor_row.abs_diff(range.start.to_point(&snapshot).row);
                let distance_from_end = cursor_row.abs_diff(range.end.to_point(&snapshot).row);
                cmp::min(distance_from_start, distance_from_end)
            })?;

        let mut edit_start_ix = closest_edit_ix;
        for (range, _) in edits[..edit_start_ix].iter().rev() {
            let distance_from_closest_edit = closest_edit_range.start.to_point(&snapshot).row
                - range.end.to_point(&snapshot).row;
            if distance_from_closest_edit <= 1 {
                edit_start_ix -= 1;
            } else {
                break;
            }
        }

        let mut edit_end_ix = closest_edit_ix + 1;
        for (range, _) in &edits[edit_end_ix..] {
            let distance_from_closest_edit =
                range.start.to_point(buffer).row - closest_edit_range.end.to_point(&snapshot).row;
            if distance_from_closest_edit <= 1 {
                edit_end_ix += 1;
            } else {
                break;
            }
        }

        Some(edit_prediction::EditPrediction::Local {
            id: Some(prediction.id.to_string().into()),
            edits: edits[edit_start_ix..edit_end_ix].to_vec(),
            edit_preview: Some(prediction.edit_preview.clone()),
        })
    }
}
