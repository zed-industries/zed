use editor::{display_map::ToDisplayPoint, scroll::autoscroll::Autoscroll, Editor};
use gpui::{
    actions, div, prelude::*, AppContext, Div, EventEmitter, FocusHandle, FocusableView, Manager,
    ParentComponent, Render, SharedString, Styled, Subscription, View, ViewContext, VisualContext,
    WindowContext,
};
use text::{Bias, Point};
use theme::ActiveTheme;
use ui::{h_stack, v_stack, Label, StyledExt, TextColor};
use util::paths::FILE_ROW_COLUMN_DELIMITER;
use workspace::Workspace;

actions!(Toggle);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(GoToLine::register).detach();
}

pub struct GoToLine {
    line_editor: View<Editor>,
    active_editor: View<Editor>,
    current_text: SharedString,
    prev_scroll_position: Option<gpui::Point<f32>>,
    _subscriptions: Vec<Subscription>,
}

impl FocusableView for GoToLine {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.active_editor.focus_handle(cx)
    }
}
impl EventEmitter<Manager> for GoToLine {}

impl GoToLine {
    fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
        workspace.register_action(|workspace, _: &Toggle, cx| {
            let Some(editor) = workspace
                .active_item(cx)
                .and_then(|active_item| active_item.downcast::<Editor>())
            else {
                return;
            };

            workspace.toggle_modal(cx, move |cx| GoToLine::new(editor, cx));
        });
    }

    pub fn new(active_editor: View<Editor>, cx: &mut ViewContext<Self>) -> Self {
        let line_editor = cx.build_view(|cx| Editor::single_line(cx));
        let line_editor_change = cx.subscribe(&line_editor, Self::on_line_editor_event);

        let editor = active_editor.read(cx);
        let cursor = editor.selections.last::<Point>(cx).head();
        let last_line = editor.buffer().read(cx).snapshot(cx).max_point().row;
        let scroll_position = active_editor.update(cx, |editor, cx| editor.scroll_position(cx));

        let current_text = format!(
            "line {} of {} (column {})",
            cursor.row + 1,
            last_line + 1,
            cursor.column + 1,
        );

        Self {
            line_editor,
            active_editor,
            current_text: current_text.into(),
            prev_scroll_position: Some(scroll_position),
            _subscriptions: vec![line_editor_change, cx.on_release(Self::release)],
        }
    }

    fn release(&mut self, cx: &mut WindowContext) {
        let scroll_position = self.prev_scroll_position.take();
        self.active_editor.update(cx, |editor, cx| {
            editor.highlight_rows(None);
            if let Some(scroll_position) = scroll_position {
                editor.set_scroll_position(scroll_position, cx);
            }
            cx.notify();
        })
    }

    fn on_line_editor_event(
        &mut self,
        _: View<Editor>,
        event: &editor::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            // todo!() this isn't working...
            editor::Event::Blurred => cx.emit(Manager::Dismiss),
            editor::Event::BufferEdited { .. } => self.highlight_current_line(cx),
            _ => {}
        }
    }

    fn highlight_current_line(&mut self, cx: &mut ViewContext<Self>) {
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

    fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(Manager::Dismiss);
    }

    fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        if let Some(point) = self.point_from_query(cx) {
            self.active_editor.update(cx, |editor, cx| {
                let snapshot = editor.snapshot(cx).display_snapshot;
                let point = snapshot.buffer_snapshot.clip_point(point, Bias::Left);
                editor.change_selections(Some(Autoscroll::center()), cx, |s| {
                    s.select_ranges([point..point])
                });
                editor.focus(cx);
                cx.notify();
            });
            self.prev_scroll_position.take();
        }

        cx.emit(Manager::Dismiss);
    }
}

impl Render for GoToLine {
    type Element = Div<Self>;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        div()
            .elevation_2(cx)
            .key_context("GoToLine")
            .on_action(Self::cancel)
            .on_action(Self::confirm)
            .w_96()
            .child(
                v_stack()
                    .px_1()
                    .pt_0p5()
                    .gap_px()
                    .child(
                        v_stack()
                            .py_0p5()
                            .px_1()
                            .child(div().px_1().py_0p5().child(self.line_editor.clone())),
                    )
                    .child(
                        div()
                            .h_px()
                            .w_full()
                            .bg(cx.theme().colors().element_background),
                    )
                    .child(
                        h_stack()
                            .justify_between()
                            .px_2()
                            .py_1()
                            .child(Label::new(self.current_text.clone()).color(TextColor::Muted)),
                    ),
            )
    }
}
