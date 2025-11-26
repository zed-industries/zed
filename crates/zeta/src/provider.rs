use std::{cmp, sync::Arc, time::Duration};

use client::{Client, UserStore};
use edit_prediction::{DataCollectionState, Direction, EditPredictionProvider};
use gpui::{App, Entity, prelude::*};
use language::ToPoint as _;
use project::Project;

use crate::{BufferEditPrediction, Zeta, ZetaEditPredictionModel};

pub struct ZetaEditPredictionProvider {
    zeta: Entity<Zeta>,
    project: Entity<Project>,
}

impl ZetaEditPredictionProvider {
    pub const THROTTLE_TIMEOUT: Duration = Duration::from_millis(300);

    pub fn new(
        project: Entity<Project>,
        client: &Arc<Client>,
        user_store: &Entity<UserStore>,
        cx: &mut Context<Self>,
    ) -> Self {
        let zeta = Zeta::global(client, user_store, cx);
        zeta.update(cx, |zeta, cx| {
            zeta.register_project(&project, cx);
        });

        cx.observe(&zeta, |_this, _zeta, cx| {
            cx.notify();
        })
        .detach();

        Self {
            project: project,
            zeta,
        }
    }
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
        cx: &App,
    ) -> bool {
        let zeta = self.zeta.read(cx);
        if zeta.edit_prediction_model == ZetaEditPredictionModel::Sweep {
            zeta.sweep_ai.api_token.is_some()
        } else {
            true
        }
    }

    fn is_refreshing(&self, cx: &App) -> bool {
        self.zeta.read(cx).is_refreshing(&self.project)
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

        self.zeta.update(cx, |zeta, cx| {
            zeta.refresh_context_if_needed(&self.project, &buffer, cursor_position, cx);
            zeta.refresh_prediction_from_buffer(self.project.clone(), buffer, cursor_position, cx)
        });
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
    }

    fn discard(&mut self, cx: &mut Context<Self>) {
        self.zeta.update(cx, |zeta, cx| {
            zeta.discard_current_prediction(&self.project, cx);
        });
    }

    fn did_show(&mut self, cx: &mut Context<Self>) {
        self.zeta.update(cx, |zeta, cx| {
            zeta.did_show_current_prediction(&self.project, cx);
        });
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
            self.zeta.update(cx, |zeta, cx| {
                zeta.discard_current_prediction(&self.project, cx);
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
