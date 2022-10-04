mod collab_titlebar_item;
mod contacts_popover;
mod incoming_call_notification;
mod project_shared_notification;

pub use collab_titlebar_item::CollabTitlebarItem;
use gpui::MutableAppContext;
use std::sync::Arc;
use workspace::AppState;

pub fn init(app_state: Arc<AppState>, cx: &mut MutableAppContext) {
    contacts_popover::init(cx);
    collab_titlebar_item::init(cx);
    incoming_call_notification::init(app_state.user_store.clone(), cx);
    project_shared_notification::init(app_state, cx);
}
