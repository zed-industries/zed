use gpui::{action, keymap::Binding, MutableAppContext};

action!(Confirm);
action!(SelectPrev);
action!(SelectNext);
action!(SelectFirst);
action!(SelectLast);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_bindings([
        Binding::new("up", SelectPrev, Some("menu")),
        Binding::new("ctrl-p", SelectPrev, Some("menu")),
        Binding::new("down", SelectNext, Some("menu")),
        Binding::new("ctrl-n", SelectNext, Some("menu")),
        Binding::new("cmd-up", SelectFirst, Some("menu")),
        Binding::new("cmd-down", SelectLast, Some("menu")),
        Binding::new("enter", Confirm, Some("menu")),
    ]);
}
