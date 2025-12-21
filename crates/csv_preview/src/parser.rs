use crate::{CsvPreviewView, table_like_content::TableLikeContent};
use editor::{Editor, EditorEvent};
use gpui::{AppContext, Context, Entity, ListAlignment, ListState, Subscription, Task};
use std::time::{Duration, Instant};
use ui::px;

pub(crate) const REPARSE_DEBOUNCE: Duration = Duration::from_millis(200);

pub(crate) struct EditorState {
    pub editor: Entity<Editor>,
    pub _subscription: Subscription,
}

impl CsvPreviewView {
    pub(crate) fn set_editor(&mut self, editor: Entity<Editor>, cx: &mut Context<Self>) {
        if let Some(active) = &self.active_editor
            && active.editor == editor
        {
            return;
        }

        let subscription = cx.subscribe(&editor, |this, _editor, event: &EditorEvent, cx| {
            match event {
                EditorEvent::Edited { .. }
                // | EditorEvent::DirtyChanged
                | EditorEvent::Reparsed(_) // Change of the file from preview cell editor
                | EditorEvent::ExcerptsEdited{ .. } => {
                    println!("Event which triggered reparsing: {event:?}");
                    this.parse_csv_from_active_editor(true, cx);
                }
                _ => {
                    println!("Other event: {event:?}");
                }
            };
        });

        self.active_editor = Some(EditorState {
            editor,
            _subscription: subscription,
        });

        self.parse_csv_from_active_editor(false, cx);
    }

    pub(crate) fn parse_csv_from_active_editor(
        &mut self,
        wait_for_debounce: bool,
        cx: &mut Context<Self>,
    ) {
        if let Some(state) = &self.active_editor {
            self.parsing_task =
                Some(self.parse_csv_in_background(wait_for_debounce, state.editor.clone(), cx));
        }
    }

    fn parse_csv_in_background(
        &mut self,
        wait_for_debounce: bool,
        editor: Entity<Editor>,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<()>> {
        cx.spawn(async move |view, cx| {
            if wait_for_debounce {
                cx.background_executor().timer(REPARSE_DEBOUNCE).await;
            }

            let instant = Instant::now();
            let buffer_snapshot = view.update(cx, |_, cx| {
                editor
                    .read(cx)
                    .buffer()
                    .read(cx)
                    .as_singleton()
                    .map(|b| b.read(cx).text_snapshot())
            })?;

            let Some(buffer_snapshot) = buffer_snapshot else {
                return Ok(());
            };

            let parsing_task =
                cx.background_spawn(async move { TableLikeContent::from_buffer(buffer_snapshot) });

            let parsed_csv = parsing_task.await;

            let parse_duration = instant.elapsed();
            log::debug!("Parsed CSV in {}ms", parse_duration.as_millis());
            view.update(cx, move |view, cx| {
                view.list_state = ListState::new(parsed_csv.rows.len(), ListAlignment::Top, px(1.));
                view.contents = parsed_csv;
                view.performance_metrics.last_parse_took = Some(parse_duration);
                view.update_ordered_indices();
                cx.notify();
            })
        })
    }
}
