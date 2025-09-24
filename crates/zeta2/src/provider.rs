use std::{
    cmp,
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::Context as _;
use arrayvec::ArrayVec;
use client::{Client, UserStore};
use edit_prediction::{DataCollectionState, Direction, EditPredictionProvider};
use gpui::{App, Entity, EntityId, Task, prelude::*};
use language::{BufferSnapshot, ToPoint as _};
use project::Project;
use util::ResultExt as _;

use crate::{Zeta, prediction::EditPrediction};

pub struct ZetaEditPredictionProvider {
    zeta: Entity<Zeta>,
    current_prediction: Option<CurrentEditPrediction>,
    next_pending_prediction_id: usize,
    pending_predictions: ArrayVec<PendingPrediction, 2>,
    last_request_timestamp: Instant,
}

impl ZetaEditPredictionProvider {
    pub const THROTTLE_TIMEOUT: Duration = Duration::from_millis(300);

    pub fn new(
        project: Option<&Entity<Project>>,
        client: &Arc<Client>,
        user_store: &Entity<UserStore>,
        cx: &mut App,
    ) -> Self {
        let zeta = Zeta::global(client, user_store, cx);
        if let Some(project) = project {
            zeta.update(cx, |zeta, cx| {
                zeta.register_project(project, cx);
            });
        }

        Self {
            zeta,
            current_prediction: None,
            next_pending_prediction_id: 0,
            pending_predictions: ArrayVec::new(),
            last_request_timestamp: Instant::now(),
        }
    }
}

#[derive(Clone)]
struct CurrentEditPrediction {
    buffer_id: EntityId,
    prediction: EditPrediction,
}

impl CurrentEditPrediction {
    fn should_replace_prediction(&self, old_prediction: &Self, snapshot: &BufferSnapshot) -> bool {
        if self.buffer_id != old_prediction.buffer_id {
            return true;
        }

        let Some(old_edits) = old_prediction.prediction.interpolate(snapshot) else {
            return true;
        };
        let Some(new_edits) = self.prediction.interpolate(snapshot) else {
            return false;
        };

        if old_edits.len() == 1 && new_edits.len() == 1 {
            let (old_range, old_text) = &old_edits[0];
            let (new_range, new_text) = &new_edits[0];
            new_range == old_range && new_text.starts_with(old_text)
        } else {
            true
        }
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
        project: Option<Entity<project::Project>>,
        buffer: Entity<language::Buffer>,
        cursor_position: language::Anchor,
        _debounce: bool,
        cx: &mut Context<Self>,
    ) {
        let Some(project) = project else {
            return;
        };

        if self
            .zeta
            .read(cx)
            .user_store
            .read_with(cx, |user_store, _cx| {
                user_store.account_too_young() || user_store.has_overdue_invoices()
            })
        {
            return;
        }

        if let Some(current_prediction) = self.current_prediction.as_ref() {
            let snapshot = buffer.read(cx).snapshot();
            if current_prediction
                .prediction
                .interpolate(&snapshot)
                .is_some()
            {
                return;
            }
        }

        let pending_prediction_id = self.next_pending_prediction_id;
        self.next_pending_prediction_id += 1;
        let last_request_timestamp = self.last_request_timestamp;

        let task = cx.spawn(async move |this, cx| {
            if let Some(timeout) = (last_request_timestamp + Self::THROTTLE_TIMEOUT)
                .checked_duration_since(Instant::now())
            {
                cx.background_executor().timer(timeout).await;
            }

            let prediction_request = this.update(cx, |this, cx| {
                this.last_request_timestamp = Instant::now();
                this.zeta.update(cx, |zeta, cx| {
                    zeta.request_prediction(&project, &buffer, cursor_position, cx)
                })
            });

            let prediction = match prediction_request {
                Ok(prediction_request) => {
                    let prediction_request = prediction_request.await;
                    prediction_request.map(|c| {
                        c.map(|prediction| CurrentEditPrediction {
                            buffer_id: buffer.entity_id(),
                            prediction,
                        })
                    })
                }
                Err(error) => Err(error),
            };

            this.update(cx, |this, cx| {
                if this.pending_predictions[0].id == pending_prediction_id {
                    this.pending_predictions.remove(0);
                } else {
                    this.pending_predictions.clear();
                }

                let Some(new_prediction) = prediction
                    .context("edit prediction failed")
                    .log_err()
                    .flatten()
                else {
                    cx.notify();
                    return;
                };

                if let Some(old_prediction) = this.current_prediction.as_ref() {
                    let snapshot = buffer.read(cx).snapshot();
                    if new_prediction.should_replace_prediction(old_prediction, &snapshot) {
                        this.current_prediction = Some(new_prediction);
                    }
                } else {
                    this.current_prediction = Some(new_prediction);
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

    fn accept(&mut self, _cx: &mut Context<Self>) {
        // TODO [zeta2] report accept
        self.current_prediction.take();
        self.pending_predictions.clear();
    }

    fn discard(&mut self, _cx: &mut Context<Self>) {
        self.pending_predictions.clear();
        self.current_prediction.take();
    }

    fn suggest(
        &mut self,
        buffer: &Entity<language::Buffer>,
        cursor_position: language::Anchor,
        cx: &mut Context<Self>,
    ) -> Option<edit_prediction::EditPrediction> {
        let CurrentEditPrediction {
            buffer_id,
            prediction,
            ..
        } = self.current_prediction.as_mut()?;

        // Invalidate previous prediction if it was generated for a different buffer.
        if *buffer_id != buffer.entity_id() {
            self.current_prediction.take();
            return None;
        }

        let buffer = buffer.read(cx);
        let Some(edits) = prediction.interpolate(&buffer.snapshot()) else {
            self.current_prediction.take();
            return None;
        };

        let cursor_row = cursor_position.to_point(buffer).row;
        let (closest_edit_ix, (closest_edit_range, _)) =
            edits.iter().enumerate().min_by_key(|(_, (range, _))| {
                let distance_from_start = cursor_row.abs_diff(range.start.to_point(buffer).row);
                let distance_from_end = cursor_row.abs_diff(range.end.to_point(buffer).row);
                cmp::min(distance_from_start, distance_from_end)
            })?;

        let mut edit_start_ix = closest_edit_ix;
        for (range, _) in edits[..edit_start_ix].iter().rev() {
            let distance_from_closest_edit =
                closest_edit_range.start.to_point(buffer).row - range.end.to_point(buffer).row;
            if distance_from_closest_edit <= 1 {
                edit_start_ix -= 1;
            } else {
                break;
            }
        }

        let mut edit_end_ix = closest_edit_ix + 1;
        for (range, _) in &edits[edit_end_ix..] {
            let distance_from_closest_edit =
                range.start.to_point(buffer).row - closest_edit_range.end.to_point(buffer).row;
            if distance_from_closest_edit <= 1 {
                edit_end_ix += 1;
            } else {
                break;
            }
        }

        Some(edit_prediction::EditPrediction {
            id: Some(prediction.id.to_string().into()),
            edits: edits[edit_start_ix..edit_end_ix].to_vec(),
            edit_preview: Some(prediction.edit_preview.clone()),
        })
    }
}
