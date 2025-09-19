use std::{ops::Range, sync::Arc};

use gpui::{App, Entity, EntityId, Task, prelude::*};

use edit_prediction::{DataCollectionState, Direction, EditPrediction, EditPredictionProvider};
use language::{Anchor, ToPoint};

pub struct Zeta2EditPredictionProvider {
    current: Option<CurrentEditPrediction>,
    pending: Option<Task<()>>,
}

impl Zeta2EditPredictionProvider {
    pub fn new() -> Self {
        Self {
            current: None,
            pending: None,
        }
    }
}

#[derive(Clone)]
struct CurrentEditPrediction {
    buffer_id: EntityId,
    prediction: EditPrediction,
}

impl EditPredictionProvider for Zeta2EditPredictionProvider {
    fn name() -> &'static str {
        // TODO [zeta2]
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

    fn usage(&self, _cx: &App) -> Option<client::EditPredictionUsage> {
        // TODO [zeta2]
        None
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
        self.pending.is_some()
    }

    fn refresh(
        &mut self,
        _project: Option<Entity<project::Project>>,
        buffer: Entity<language::Buffer>,
        cursor_position: language::Anchor,
        _debounce: bool,
        cx: &mut Context<Self>,
    ) {
        // TODO [zeta2] check account
        // TODO [zeta2] actually request completion / interpolate

        let snapshot = buffer.read(cx).snapshot();
        let point = cursor_position.to_point(&snapshot);
        let end_anchor = snapshot.anchor_before(language::Point::new(
            point.row,
            snapshot.line_len(point.row),
        ));

        let edits: Arc<[(Range<Anchor>, String)]> =
            vec![(cursor_position..end_anchor, "👻".to_string())].into();
        let edits_preview_task = buffer.read(cx).preview_edits(edits.clone(), cx);

        // TODO [zeta2] throttle
        // TODO [zeta2] keep 2 requests
        self.pending = Some(cx.spawn(async move |this, cx| {
            let edits_preview = edits_preview_task.await;

            this.update(cx, |this, cx| {
                this.current = Some(CurrentEditPrediction {
                    buffer_id: buffer.entity_id(),
                    prediction: EditPrediction {
                        // TODO! [zeta2] request id?
                        id: None,
                        edits: edits.to_vec(),
                        edit_preview: Some(edits_preview),
                    },
                });
                this.pending.take();
                cx.notify();
            })
            .ok();
        }));
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
        self.current.take();
        self.pending.take();
    }

    fn discard(&mut self, _cx: &mut Context<Self>) {
        self.current.take();
        self.pending.take();
    }

    fn suggest(
        &mut self,
        buffer: &Entity<language::Buffer>,
        _cursor_position: language::Anchor,
        _cx: &mut Context<Self>,
    ) -> Option<EditPrediction> {
        let current_prediction = self.current.take()?;

        if current_prediction.buffer_id != buffer.entity_id() {
            return None;
        }

        // TODO [zeta2] interpolate

        Some(current_prediction.prediction)
    }
}
