mod bookmark_picker;

use gpui::App;
use jj::JujutsuStore;
use workspace::Workspace;

pub fn init(cx: &mut App) {
    JujutsuStore::init_global(cx);

    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        bookmark_picker::register(workspace);
    })
    .detach();
}
