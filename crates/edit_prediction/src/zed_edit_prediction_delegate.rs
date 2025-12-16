use std::{cmp, sync::Arc};

use client::{Client, UserStore};
use cloud_llm_client::EditPredictionRejectReason;
use edit_prediction_types::{DataCollectionState, Direction, EditPredictionDelegate};
use gpui::{App, Entity, prelude::*};
use language::{Buffer, ToPoint as _};
use project::Project;

use crate::{BufferEditPrediction, EditPredictionModel, EditPredictionStore};

pub struct ZedEditPredictionDelegate {
    store: Entity<EditPredictionStore>,
    project: Entity<Project>,
    singleton_buffer: Option<Entity<Buffer>>,
}

impl ZedEditPredictionDelegate {
    pub fn new(
        project: Entity<Project>,
        singleton_buffer: Option<Entity<Buffer>>,
        client: &Arc<Client>,
        user_store: &Entity<UserStore>,
        cx: &mut Context<Self>,
    ) -> Self {
        let store = EditPredictionStore::global(client, user_store, cx);
        store.update(cx, |store, cx| {
            store.register_project(&project, cx);
        });

        cx.observe(&store, |_this, _ep_store, cx| {
            cx.notify();
        })
        .detach();

        Self {
            project: project,
            store: store,
            singleton_buffer,
        }
    }
}

impl EditPredictionDelegate for ZedEditPredictionDelegate {
    fn name() -> &'static str {
        "zed-predict"
    }

    fn display_name() -> &'static str {
        "Zed's Edit Predictions"
    }

    fn show_predictions_in_menu() -> bool {
        true
    }

    fn show_tab_accept_marker() -> bool {
        true
    }

    fn data_collection_state(&self, cx: &App) -> DataCollectionState {
        if let Some(buffer) = &self.singleton_buffer
            && let Some(file) = buffer.read(cx).file()
        {
            let is_project_open_source =
                self.store
                    .read(cx)
                    .is_file_open_source(&self.project, file, cx);
            if self.store.read(cx).data_collection_choice.is_enabled() {
                DataCollectionState::Enabled {
                    is_project_open_source,
                }
            } else {
                DataCollectionState::Disabled {
                    is_project_open_source,
                }
            }
        } else {
            return DataCollectionState::Disabled {
                is_project_open_source: false,
            };
        }
    }

    fn toggle_data_collection(&mut self, cx: &mut App) {
        self.store.update(cx, |store, cx| {
            store.toggle_data_collection_choice(cx);
        });
    }

    fn usage(&self, cx: &App) -> Option<client::EditPredictionUsage> {
        self.store.read(cx).usage(cx)
    }

    fn is_enabled(
        &self,
        _buffer: &Entity<language::Buffer>,
        _cursor_position: language::Anchor,
        cx: &App,
    ) -> bool {
        let store = self.store.read(cx);
        if store.edit_prediction_model == EditPredictionModel::Sweep {
            store.has_sweep_api_token(cx)
        } else {
            true
        }
    }

    fn is_refreshing(&self, cx: &App) -> bool {
        self.store.read(cx).is_refreshing(&self.project)
    }

    fn refresh(
        &mut self,
        buffer: Entity<language::Buffer>,
        cursor_position: language::Anchor,
        _debounce: bool,
        cx: &mut Context<Self>,
    ) {
        let store = self.store.read(cx);

        if store.user_store.read_with(cx, |user_store, _cx| {
            user_store.account_too_young() || user_store.has_overdue_invoices()
        }) {
            return;
        }

        self.store.update(cx, |store, cx| {
            if let Some(current) =
                store.prediction_at(&buffer, Some(cursor_position), &self.project, cx)
                && let BufferEditPrediction::Local { prediction } = current
                && prediction.interpolate(buffer.read(cx)).is_some()
            {
                return;
            }

            store.refresh_context(&self.project, &buffer, cursor_position, cx);
            store.refresh_prediction_from_buffer(self.project.clone(), buffer, cursor_position, cx)
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
        self.store.update(cx, |store, cx| {
            store.accept_current_prediction(&self.project, cx);
        });
    }

    fn discard(&mut self, cx: &mut Context<Self>) {
        self.store.update(cx, |store, _cx| {
            store.reject_current_prediction(EditPredictionRejectReason::Discarded, &self.project);
        });
    }

    fn did_show(&mut self, cx: &mut Context<Self>) {
        self.store.update(cx, |store, cx| {
            store.did_show_current_prediction(&self.project, cx);
        });
    }

    fn suggest(
        &mut self,
        buffer: &Entity<language::Buffer>,
        cursor_position: language::Anchor,
        cx: &mut Context<Self>,
    ) -> Option<edit_prediction_types::EditPrediction> {
        self.store.update(cx, |store, cx| {
            let prediction =
                store.prediction_at(buffer, Some(cursor_position), &self.project, cx)?;

            let prediction = match prediction {
                BufferEditPrediction::Local { prediction } => prediction,
                BufferEditPrediction::Jump { prediction } => {
                    return Some(edit_prediction_types::EditPrediction::Jump {
                        id: Some(prediction.id.to_string().into()),
                        snapshot: prediction.snapshot.clone(),
                        target: prediction.edits.first().unwrap().0.start,
                    });
                }
            };

            let buffer = buffer.read(cx);
            let snapshot = buffer.snapshot();

            let Some(edits) = prediction.interpolate(&snapshot) else {
                store.reject_current_prediction(
                    EditPredictionRejectReason::InterpolatedEmpty,
                    &self.project,
                );
                return None;
            };

            let cursor_row = cursor_position.to_point(&snapshot).row;
            let (closest_edit_ix, (closest_edit_range, _)) =
                edits.iter().enumerate().min_by_key(|(_, (range, _))| {
                    let distance_from_start =
                        cursor_row.abs_diff(range.start.to_point(&snapshot).row);
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
                let distance_from_closest_edit = range.start.to_point(buffer).row
                    - closest_edit_range.end.to_point(&snapshot).row;
                if distance_from_closest_edit <= 1 {
                    edit_end_ix += 1;
                } else {
                    break;
                }
            }

            Some(edit_prediction_types::EditPrediction::Local {
                id: Some(prediction.id.to_string().into()),
                edits: edits[edit_start_ix..edit_end_ix].to_vec(),
                edit_preview: Some(prediction.edit_preview.clone()),
            })
        })
    }
}
