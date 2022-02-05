use crate::ProjectPanel;
use editor::{Editor, EditorSettings};
use gpui::{
    action, elements::*, keymap::Binding, Axis, Entity, MutableAppContext, RenderContext, View,
    ViewContext, ViewHandle,
};
use postage::watch;
use std::sync::Arc;
use workspace::Settings;

action!(Dismiss);
action!(Confirm);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_bindings([
        Binding::new("escape", Dismiss, Some("InputBar")),
        Binding::new("enter", Confirm, Some("InputBar")),
    ]);
    cx.add_action(InputBar::confirm);
    cx.add_action(InputBar::dismiss);
}

pub struct InputBar {
    settings: watch::Receiver<Settings>,
    query_editor: ViewHandle<Editor>,
}

pub enum Event {
    Confirmed,
    Dismissed,
}

impl InputBar {
    pub fn new(settings: watch::Receiver<Settings>, cx: &mut ViewContext<Self>) -> Self {
        let query_editor = cx.add_view(|cx| {
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
        cx.subscribe(&query_editor, Self::on_query_editor_event)
            .detach();

        Self {
            settings: settings.clone(),
            query_editor,
        }
    }

    fn confirm(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Confirmed);
    }

    fn dismiss(&mut self, _: &Dismiss, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Dismissed)
    }

    pub(crate) fn on_event(
        project_panel: &mut ProjectPanel,
        this: ViewHandle<Self>,
        event: &Event,
        cx: &mut ViewContext<ProjectPanel>,
    ) {
        match event {
            Event::Confirmed => {
                let name = this.read(cx).query_editor.read(cx).text(cx);
                if !name.is_empty() {
                    let worktree_id = project_panel
                        .selected_entry(cx)
                        .map(|(worktree, _)| worktree.id());
                    if let Some(worktree_id) = worktree_id {
                        cx.emit(crate::Event::CreateFile { worktree_id, name });
                    }
                    project_panel.dismiss_input_bar(cx)
                }
            }
            Event::Dismissed => project_panel.dismiss_input_bar(cx),
        }
    }

    fn on_query_editor_event(
        &mut self,
        _: ViewHandle<Editor>,
        event: &editor::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            editor::Event::Blurred => cx.emit(Event::Dismissed),
            _ => {}
        }
    }
}

impl Entity for InputBar {
    type Event = Event;
}

impl View for InputBar {
    fn ui_name() -> &'static str {
        "InputBar"
    }

    fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
        let theme = &self.settings.borrow().theme.selector;

        let label = format!("enter file name");

        Align::new(
            ConstrainedBox::new(
                Container::new(
                    Flex::new(Axis::Vertical)
                        .with_child(
                            Container::new(ChildView::new(&self.query_editor).boxed())
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
        .named("input bar")
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.focus(&self.query_editor);
    }
}
