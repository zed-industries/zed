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
        Binding::new("cmd-shift-O", Toggle, Some("Editor")),
        Binding::new("escape", Toggle, Some("GoToLine")),
        Binding::new("enter", Confirm, Some("GoToLine")),
    ]);
    cx.add_action(OutlineView::toggle);
    cx.add_action(OutlineView::confirm);
}

struct OutlineView {}

impl Entity for OutlineView {
    type Event = ();
}

impl View for OutlineView {
    fn ui_name() -> &'static str {
        "OutlineView"
    }

    fn render(&mut self, cx: &mut RenderContext<'_, Self>) -> ElementBox {
        todo!()
    }
}

impl OutlineView {
    fn toggle(workspace: &mut Workspace, _: &Toggle, cx: &mut ViewContext<Workspace>) {
        let editor = workspace
            .active_item(cx)
            .unwrap()
            .to_any()
            .downcast::<Editor>()
            .unwrap();
        let buffer = editor.read(cx).buffer().read(cx);
        dbg!(buffer.read(cx).outline());
    }

    fn confirm(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {}
}
