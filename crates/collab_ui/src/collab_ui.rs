mod collab_titlebar_item;
mod contacts_popover;
mod incoming_call_notification;

use client::UserStore;
pub use collab_titlebar_item::CollabTitlebarItem;
use gpui::{ModelHandle, MutableAppContext};

pub fn init(user_store: ModelHandle<UserStore>, cx: &mut MutableAppContext) {
    contacts_popover::init(cx);
    collab_titlebar_item::init(cx);
    incoming_call_notification::init(user_store, cx);
}
