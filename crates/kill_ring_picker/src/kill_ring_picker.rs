use editor::Editor;
use gpui::{App, Context, Window};

pub fn init(cx: &mut App) {
    cx.observe_new(KillRingPicker::register).detach();
}

pub struct KillRingPicker;

impl KillRingPicker {
    fn register(_editor: &mut Editor, _window: Option<&mut Window>, _cx: &mut Context<Editor>) {}
}
