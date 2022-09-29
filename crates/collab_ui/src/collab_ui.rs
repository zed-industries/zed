mod collab_titlebar_item;
mod contacts_popover;
mod incoming_call_notification;

use client::{Client, UserStore};
pub use collab_titlebar_item::CollabTitlebarItem;
use gpui::{ModelHandle, MutableAppContext};
use std::sync::Arc;

pub fn init(client: Arc<Client>, user_store: ModelHandle<UserStore>, cx: &mut MutableAppContext) {
    contacts_popover::init(cx);
    collab_titlebar_item::init(cx);
    incoming_call_notification::init(client, user_store, cx);
}
