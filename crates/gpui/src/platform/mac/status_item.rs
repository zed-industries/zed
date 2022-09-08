use cocoa::{
    appkit::{NSSquareStatusItemLength, NSStatusBar},
    base::{id, nil},
};
use core_foundation::base::TCFType;
use core_graphics::color::CGColor;
use objc::{msg_send, sel, sel_impl};

pub struct StatusItem(id);

impl StatusItem {
    pub fn add() -> Self {
        unsafe {
            let status_bar = NSStatusBar::systemStatusBar(nil);
            let native_item: id =
                msg_send![status_bar, statusItemWithLength: NSSquareStatusItemLength];
            let button: id = msg_send![native_item, button];
            let layer: id = msg_send![button, layer];
            let _: () = msg_send![layer, setBackgroundColor: CGColor::rgb(1., 0., 0., 1.).as_concrete_TypeRef()];
            StatusItem(native_item)
        }
    }
}

impl crate::StatusItem for StatusItem {}
