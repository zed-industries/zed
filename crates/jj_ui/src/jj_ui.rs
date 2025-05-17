mod bookmark_picker;

use gpui::App;
use workspace::Workspace;

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        bookmark_picker::register(workspace);
    })
    .detach();
}
