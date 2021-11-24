use buffer::{Bias, Point};
use editor::{Autoscroll, Editor, EditorSettings};
use gpui::{
    action, elements::*, keymap::Binding, Entity, MutableAppContext, RenderContext, View,
    ViewContext, ViewHandle,
};
use postage::watch;
use workspace::{Settings, Workspace};

action!(Toggle);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_bindings([
        Binding::new("ctrl-g", Toggle, Some("Editor")),
        Binding::new("escape", Toggle, Some("GoToLine")),
    ]);
    cx.add_action(GoToLine::toggle);
}

pub struct GoToLine {
    settings: watch::Receiver<Settings>,
    line_editor: ViewHandle<Editor>,
    active_editor: ViewHandle<Editor>,
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
                    move |_| {
                        let settings = settings.borrow();
                        EditorSettings {
                            tab_size: settings.tab_size,
                            style: settings.theme.editor.clone(),
                        }
                    }
                },
                cx,
            )
        });
        cx.subscribe(&line_editor, Self::on_line_editor_event)
            .detach();
        Self {
            settings: settings.clone(),
            line_editor,
            active_editor,
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
                let line_editor = self.line_editor.read(cx).buffer().read(cx).text();
                let mut components = line_editor.trim().split(':');
                let row = components.next().and_then(|row| row.parse::<u32>().ok());
                let column = components.next().and_then(|row| row.parse::<u32>().ok());
                if let Some(point) = row.map(|row| Point::new(row, column.unwrap_or(0))) {
                    self.active_editor.update(cx, |active_editor, cx| {
                        let buffer = active_editor.buffer().read(cx);
                        let point = buffer.clip_point(point, Bias::Left);
                        active_editor.select_ranges([point..point], Some(Autoscroll::Center), cx);
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
}

impl View for GoToLine {
    fn ui_name() -> &'static str {
        "GoToLine"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        Align::new(
            ConstrainedBox::new(
                Container::new(ChildView::new(self.line_editor.id()).boxed()).boxed(),
            )
            .with_max_width(500.0)
            .with_max_height(420.0)
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
