use gpui::{
    action, color::Color, elements::*, keymap::Binding, Entity, MutableAppContext, RenderContext,
    View, ViewContext,
};
use workspace::Workspace;

action!(Deploy);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_bindings([Binding::new(
        "cmd-f",
        Deploy,
        Some("Editor && mode == full"),
    )]);
    cx.add_action(FindBar::deploy);
}

struct FindBar;

impl Entity for FindBar {
    type Event = ();
}

impl View for FindBar {
    fn ui_name() -> &'static str {
        "FindBar"
    }

    fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
        Empty::new()
            .contained()
            .with_background_color(Color::red())
            .constrained()
            .with_height(30.)
            .boxed()
    }
}

impl FindBar {
    fn deploy(workspace: &mut Workspace, _: &Deploy, cx: &mut ViewContext<Workspace>) {
        workspace
            .active_pane()
            .update(cx, |pane, cx| pane.show_toolbar(cx, |_| FindBar));
    }
}
