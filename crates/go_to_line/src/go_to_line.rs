use editor::{display_map::ToDisplayPoint, Autoscroll, Editor, EditorSettings};
use gpui::{
    action, elements::*, geometry::vector::Vector2F, keymap::Binding, Axis, Entity,
    MutableAppContext, RenderContext, View, ViewContext, ViewHandle,
};
use postage::watch;
use std::sync::Arc;
use text::{Bias, Point, Selection};
use workspace::{Settings, Workspace};

action!(Toggle);
action!(Confirm);

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
    settings: watch::Receiver<Settings>,
    line_editor: ViewHandle<Editor>,
    active_editor: ViewHandle<Editor>,
    restore_state: Option<RestoreState>,
    line_selection_id: Option<usize>,
    cursor_point: Point,
    max_point: Point,
}

struct RestoreState {
    scroll_position: Vector2F,
    selections: Vec<Selection<usize>>,
}

pub enum Event {
    Dismissed,
}

impl GoToLine {
    pub fn new(
        active_editor: ViewHandle<Editor>,
        settings: watch::Receiver<Settings>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let line_editor = cx.add_view(|cx| {
            Editor::single_line(
                {
                    let settings = settings.clone();
                    Arc::new(move |_| {
                        let settings = settings.borrow();
                        EditorSettings {
                            tab_size: settings.tab_size,
                            style: settings.theme.selector.input_editor.as_editor(),
                            soft_wrap: editor::SoftWrap::None,
                        }
                    })
                },
                cx,
            )
        });
        cx.subscribe(&line_editor, Self::on_line_editor_event)
            .detach();

        let (restore_state, cursor_point, max_point) = active_editor.update(cx, |editor, cx| {
            let restore_state = Some(RestoreState {
                scroll_position: editor.scroll_position(cx),
                selections: editor.local_selections::<usize>(cx),
            });

            let buffer = editor.buffer().read(cx).read(cx);
            (
                restore_state,
                editor.newest_selection(&buffer).head(),
                buffer.max_point(),
            )
        });

        Self {
            settings: settings.clone(),
            line_editor,
            active_editor,
            restore_state,
            line_selection_id: None,
            cursor_point,
            max_point,
        }
    }

    fn toggle(workspace: &mut Workspace, _: &Toggle, cx: &mut ViewContext<Workspace>) {
        workspace.toggle_modal(cx, |cx, workspace| {
            let editor = workspace
                .active_item(cx)
                .unwrap()
                .to_any()
                .downcast::<Editor>()
                .unwrap();
            let view = cx.add_view(|cx| GoToLine::new(editor, workspace.settings.clone(), cx));
            cx.subscribe(&view, Self::on_event).detach();
            view
        });
    }

    fn confirm(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {
        self.restore_state.take();
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
            editor::Event::Edited => {
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
                    self.line_selection_id = self.active_editor.update(cx, |active_editor, cx| {
                        let snapshot = active_editor.snapshot(cx).display_snapshot;
                        let point = snapshot.buffer_snapshot.clip_point(point, Bias::Left);
                        let display_point = point.to_display_point(&snapshot);
                        let row = display_point.row();
                        active_editor.select_ranges([point..point], Some(Autoscroll::Center), cx);
                        active_editor.set_highlighted_rows(Some(row..row + 1));
                        Some(
                            active_editor
                                .newest_selection::<usize>(&snapshot.buffer_snapshot)
                                .id,
                        )
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
        let line_selection_id = self.line_selection_id.take();
        let restore_state = self.restore_state.take();
        self.active_editor.update(cx, |editor, cx| {
            editor.set_highlighted_rows(None);
            if let Some((line_selection_id, restore_state)) = line_selection_id.zip(restore_state) {
                let newest_selection =
                    editor.newest_selection::<usize>(&editor.buffer().read(cx).read(cx));
                if line_selection_id == newest_selection.id {
                    editor.set_scroll_position(restore_state.scroll_position, cx);
                    editor.update_selections(restore_state.selections, None, cx);
                }
            }
        })
    }
}

impl View for GoToLine {
    fn ui_name() -> &'static str {
        "GoToLine"
    }

    fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
        let theme = &self.settings.borrow().theme.selector;

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
                            Container::new(ChildView::new(self.line_editor.id()).boxed())
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

    fn on_blur(&mut self, _: &mut ViewContext<Self>) {}
}
