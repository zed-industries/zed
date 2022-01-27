use editor::{Editor, EditorSettings};
use gpui::{
    action, elements::*, keymap::Binding, Entity, MutableAppContext, RenderContext, View,
    ViewContext, ViewHandle,
};
use postage::watch;
use std::sync::Arc;
use workspace::{ItemViewHandle, Settings, Toolbar, Workspace};

action!(Deploy);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_bindings([Binding::new(
        "cmd-f",
        Deploy,
        Some("Editor && mode == full"),
    )]);
    cx.add_action(FindBar::deploy);
}

struct FindBar {
    settings: watch::Receiver<Settings>,
    query_editor: ViewHandle<Editor>,
    active_editor: Option<ViewHandle<Editor>>,
}

impl Entity for FindBar {
    type Event = ();
}

impl View for FindBar {
    fn ui_name() -> &'static str {
        "FindBar"
    }

    fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
        ChildView::new(&self.query_editor)
            .contained()
            .with_style(self.settings.borrow().theme.selector.input_editor.container)
            .boxed()
    }
}

impl Toolbar for FindBar {
    fn active_item_changed(
        &mut self,
        item: Option<Box<dyn ItemViewHandle>>,
        cx: &mut ViewContext<Self>,
    ) -> bool {
        self.active_editor = item.and_then(|item| item.act_as::<Editor>(cx));
        self.active_editor.is_some()
    }
}

impl FindBar {
    fn new(settings: watch::Receiver<Settings>, cx: &mut ViewContext<Self>) -> Self {
        let query_editor = cx.add_view(|cx| {
            Editor::single_line(
                {
                    let settings = settings.clone();
                    Arc::new(move |_| {
                        let settings = settings.borrow();
                        EditorSettings {
                            style: settings.theme.selector.input_editor.as_editor(),
                            tab_size: settings.tab_size,
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
            query_editor,
            active_editor: None,
            settings,
        }
    }

    fn deploy(workspace: &mut Workspace, _: &Deploy, cx: &mut ViewContext<Workspace>) {
        let settings = workspace.settings();
        workspace.active_pane().update(cx, |pane, cx| {
            pane.show_toolbar(cx, |cx| FindBar::new(settings, cx));
            if let Some(toolbar) = pane.active_toolbar() {
                cx.focus(toolbar);
            }
        });
    }

    fn cancel(workspace: &mut Workspace, _: &Cancel, cx: &mut ViewContext<Workspace>) {
        workspace
            .active_pane()
            .update(cx, |pane, cx| pane.hide_toolbar(cx));
    }
}
