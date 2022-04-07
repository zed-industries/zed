use editor::{display_map::ToDisplayPoint, Autoscroll, DisplayPoint, Editor};
use gpui::{
    actions, elements::*, geometry::vector::Vector2F, keymap::Binding, Axis, Entity,
    MutableAppContext, RenderContext, View, ViewContext, ViewHandle,
};
use settings::Settings;
use text::{Bias, Point};
use workspace::Workspace;

actions!(go_to_line, [Toggle, Confirm]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_bindings([
        Binding::new("ctrl-g", Toggle, Some("Editor")),
        Binding::new("escape", Toggle, Some("GoToLine")),
        Binding::new("enter", Confirm, Some("GoToLine")),
    ]);
    cx.add_action(GoToLine::toggle);
    cx.add_action(GoToLine::confirm);
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
            Editor::single_line(Some(|theme| theme.selector.input_editor.clone()), cx)
        });
        cx.subscribe(&line_editor, Self::on_line_editor_event)
            .detach();

        let (scroll_position, cursor_point, max_point) = active_editor.update(cx, |editor, cx| {
            let scroll_position = editor.scroll_position(cx);
            let buffer = editor.buffer().read(cx).read(cx);
            (
                Some(scroll_position),
                editor.newest_selection_with_snapshot(&buffer).head(),
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
            workspace.toggle_modal(cx, |cx, _| {
                let view = cx.add_view(|cx| GoToLine::new(editor, cx));
                cx.subscribe(&view, Self::on_event).detach();
                view
            });
        }
    }

    fn confirm(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {
        self.prev_scroll_position.take();
        self.active_editor.update(cx, |active_editor, cx| {
            if let Some(rows) = active_editor.highlighted_rows() {
                let snapshot = active_editor.snapshot(cx).display_snapshot;
                let position = DisplayPoint::new(rows.start, 0).to_point(&snapshot);
                active_editor.select_ranges([position..position], Some(Autoscroll::Center), cx);
            }
        });
        cx.emit(Event::Dismissed);
    }

    fn on_event(
        workspace: &mut Workspace,
        _: ViewHandle<Self>,
        event: &Event,
        cx: &mut ViewContext<Workspace>,
    ) {
        match event {
            Event::Dismissed => workspace.dismiss_modal(cx),
        }
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
                let line_editor = self.line_editor.read(cx).buffer().read(cx).read(cx).text();
                let mut components = line_editor.trim().split(&[',', ':'][..]);
                let row = components.next().and_then(|row| row.parse::<u32>().ok());
                let column = components.next().and_then(|row| row.parse::<u32>().ok());
                if let Some(point) = row.map(|row| {
                    Point::new(
                        row.saturating_sub(1),
                        column.map(|column| column.saturating_sub(1)).unwrap_or(0),
                    )
                }) {
                    self.active_editor.update(cx, |active_editor, cx| {
                        let snapshot = active_editor.snapshot(cx).display_snapshot;
                        let point = snapshot.buffer_snapshot.clip_point(point, Bias::Left);
                        let display_point = point.to_display_point(&snapshot);
                        let row = display_point.row();
                        active_editor.highlight_rows(Some(row..row + 1));
                        active_editor.request_autoscroll(Autoscroll::Center, cx);
                    });
                    cx.notify();
                }
            }
            _ => {}
        }
    }
}

impl Entity for GoToLine {
    type Event = Event;

    fn release(&mut self, cx: &mut MutableAppContext) {
        let scroll_position = self.prev_scroll_position.take();
        self.active_editor.update(cx, |editor, cx| {
            editor.highlight_rows(None);
            if let Some(scroll_position) = scroll_position {
                editor.set_scroll_position(scroll_position, cx);
            }
        })
    }
}

impl View for GoToLine {
    fn ui_name() -> &'static str {
        "GoToLine"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = &cx.global::<Settings>().theme.selector;

        let label = format!(
            "{},{} of {} lines",
            self.cursor_point.row + 1,
            self.cursor_point.column + 1,
            self.max_point.row + 1
        );

        Align::new(
            ConstrainedBox::new(
                Container::new(
                    Flex::new(Axis::Vertical)
                        .with_child(
                            Container::new(ChildView::new(&self.line_editor).boxed())
                                .with_style(theme.input_editor.container)
                                .boxed(),
                        )
                        .with_child(
                            Container::new(Label::new(label, theme.empty.label.clone()).boxed())
                                .with_style(theme.empty.container)
                                .boxed(),
                        )
                        .boxed(),
                )
                .with_style(theme.container)
                .boxed(),
            )
            .with_max_width(500.0)
            .boxed(),
        )
        .top()
        .named("go to line")
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.focus(&self.line_editor);
    }
}
