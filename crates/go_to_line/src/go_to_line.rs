use std::sync::Arc;

use editor::{
    display_map::ToDisplayPoint, scroll::autoscroll::Autoscroll, Editor, FILE_ROW_COLUMN_DELIMITER,
};
use gpui::{
    actions, elements::*, geometry::vector::Vector2F, AnyViewHandle, AppContext, Axis, Entity,
    View, ViewContext, ViewHandle,
};
use menu::{Cancel, Confirm};
use settings::Settings;
use text::{Bias, Point};
use workspace::{Modal, Workspace};

actions!(go_to_line, [Toggle]);

pub fn init(cx: &mut AppContext) {
    cx.add_action(GoToLine::toggle);
    cx.add_action(GoToLine::confirm);
    cx.add_action(GoToLine::cancel);
}

pub struct GoToLine {
    line_editor: ViewHandle<Editor>,
    active_editor: ViewHandle<Editor>,
    prev_scroll_position: Option<Vector2F>,
    cursor_point: Point,
    max_point: Point,
}

pub enum Event {
    Dismissed,
}

impl GoToLine {
    pub fn new(active_editor: ViewHandle<Editor>, cx: &mut ViewContext<Self>) -> Self {
        let line_editor = cx.add_view(|cx| {
            Editor::single_line(
                Some(Arc::new(|theme| theme.picker.input_editor.clone())),
                cx,
            )
        });
        cx.subscribe(&line_editor, Self::on_line_editor_event)
            .detach();

        let (scroll_position, cursor_point, max_point) = active_editor.update(cx, |editor, cx| {
            let scroll_position = editor.scroll_position(cx);
            let buffer = editor.buffer().read(cx).snapshot(cx);
            (
                Some(scroll_position),
                editor.selections.newest(cx).head(),
                buffer.max_point(),
            )
        });

        Self {
            line_editor,
            active_editor,
            prev_scroll_position: scroll_position,
            cursor_point,
            max_point,
        }
    }

    fn toggle(workspace: &mut Workspace, _: &Toggle, cx: &mut ViewContext<Workspace>) {
        if let Some(editor) = workspace
            .active_item(cx)
            .and_then(|active_item| active_item.downcast::<Editor>())
        {
            workspace.toggle_modal(cx, |_, cx| cx.add_view(|cx| GoToLine::new(editor, cx)));
        }
    }

    fn cancel(&mut self, _: &Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Dismissed);
    }

    fn confirm(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {
        self.prev_scroll_position.take();
        if let Some(point) = self.point_from_query(cx) {
            self.active_editor.update(cx, |active_editor, cx| {
                let snapshot = active_editor.snapshot(cx).display_snapshot;
                let point = snapshot.buffer_snapshot.clip_point(point, Bias::Left);
                active_editor.change_selections(Some(Autoscroll::center()), cx, |s| {
                    s.select_ranges([point..point])
                });
            });
        }

        cx.emit(Event::Dismissed);
    }

    fn on_line_editor_event(
        &mut self,
        _: ViewHandle<Editor>,
        event: &editor::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            editor::Event::Blurred => cx.emit(Event::Dismissed),
            editor::Event::BufferEdited { .. } => {
                if let Some(point) = self.point_from_query(cx) {
                    self.active_editor.update(cx, |active_editor, cx| {
                        let snapshot = active_editor.snapshot(cx).display_snapshot;
                        let point = snapshot.buffer_snapshot.clip_point(point, Bias::Left);
                        let display_point = point.to_display_point(&snapshot);
                        let row = display_point.row();
                        active_editor.highlight_rows(Some(row..row + 1));
                        active_editor.request_autoscroll(Autoscroll::center(), cx);
                    });
                    cx.notify();
                }
            }
            _ => {}
        }
    }

    fn point_from_query(&self, cx: &ViewContext<Self>) -> Option<Point> {
        let line_editor = self.line_editor.read(cx).text(cx);
        let mut components = line_editor
            .splitn(2, FILE_ROW_COLUMN_DELIMITER)
            .map(str::trim)
            .fuse();
        let row = components.next().and_then(|row| row.parse::<u32>().ok())?;
        let column = components.next().and_then(|col| col.parse::<u32>().ok());
        Some(Point::new(
            row.saturating_sub(1),
            column.unwrap_or(0).saturating_sub(1),
        ))
    }
}

impl Entity for GoToLine {
    type Event = Event;

    fn release(&mut self, cx: &mut AppContext) {
        let scroll_position = self.prev_scroll_position.take();
        cx.update_window(self.active_editor.window_id(), |cx| {
            self.active_editor.update(cx, |editor, cx| {
                editor.highlight_rows(None);
                if let Some(scroll_position) = scroll_position {
                    editor.set_scroll_position(scroll_position, cx);
                }
            })
        });
    }
}

impl View for GoToLine {
    fn ui_name() -> &'static str {
        "GoToLine"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let theme = &cx.global::<Settings>().theme.picker;

        let label = format!(
            "{}{FILE_ROW_COLUMN_DELIMITER}{} of {} lines",
            self.cursor_point.row + 1,
            self.cursor_point.column + 1,
            self.max_point.row + 1
        );

        Flex::new(Axis::Vertical)
            .with_child(
                ChildView::new(&self.line_editor, cx)
                    .contained()
                    .with_style(theme.input_editor.container),
            )
            .with_child(
                Label::new(label, theme.no_matches.label.clone())
                    .contained()
                    .with_style(theme.no_matches.container),
            )
            .contained()
            .with_style(theme.container)
            .constrained()
            .with_max_width(500.0)
            .into_any_named("go to line")
    }

    fn focus_in(&mut self, _: AnyViewHandle, cx: &mut ViewContext<Self>) {
        cx.focus(&self.line_editor);
    }
}

impl Modal for GoToLine {
    fn dismiss_on_event(event: &Self::Event) -> bool {
        matches!(event, Event::Dismissed)
    }
}
