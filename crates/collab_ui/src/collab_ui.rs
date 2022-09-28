mod collab_titlebar_item;
mod contacts_popover;

pub use collab_titlebar_item::CollabTitlebarItem;
use gpui::MutableAppContext;

pub fn init(cx: &mut MutableAppContext) {
    contacts_popover::init(cx);
    collab_titlebar_item::init(cx);
}
