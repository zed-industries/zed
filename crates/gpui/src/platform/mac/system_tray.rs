use std::rc::Rc;

use crate::{Image, MouseButton, SystemTray};
use cocoa::appkit::NSStatusBar;
use cocoa::{
    appkit::{NSButton, NSImage, NSStatusItem, NSVariableStatusItemLength},
    base::{id, nil},
    foundation::{NSData, NSSize, NSString},
};
use objc::{msg_send, sel, sel_impl};

/// A constant for specifying the position of a button’s image relative to its title.
///
/// https://developer.apple.com/documentation/appkit/nscellimageposition?language=objc
#[allow(dead_code)]
enum NSCellImagePosition {
    ImageOnly = 1,
    ImageLeft = 2,
    ImageRight = 3,
}

pub struct MacSystemTray {
    tray: SystemTray,
    pub(crate) ns_status_bar: id,
    pub(crate) ns_menu: Option<id>,
}

impl MacSystemTray {
    pub(crate) fn create(tray: &SystemTray, ns_menu: Option<id>) -> Self {
        let mut this = Self {
            tray: tray.clone(),
            ns_status_bar: Self::create_status_bar(),
            ns_menu,
        };
        this.update(tray, ns_menu);
        this
    }

    fn create_status_bar() -> id {
        unsafe {
            let ns_status_bar =
                NSStatusBar::systemStatusBar(nil).statusItemWithLength_(NSVariableStatusItemLength);
            let _: () = msg_send![ns_status_bar, retain];

            ns_status_bar
        }
    }

    pub(crate) fn update(&mut self, tray: &SystemTray, ns_menu: Option<id>) {
        self.tray = self.tray.clone();
        self.ns_menu = ns_menu;
        self.set_tooltip(tray.tooltip.as_ref().map(|s| s.as_str()));
        self.set_icon(tray.icon.clone());
        self.set_title(tray.title.as_ref().map(|s| s.as_str()));

        if let Some(ns_menu) = &self.ns_menu {
            unsafe {
                let _: () = msg_send![self.ns_status_bar, setMenu: *ns_menu];
            }
        }
    }

    fn on_icon_click(&self, mouse_button: MouseButton) {
        unsafe {
            let button = self.ns_status_bar.button();

            if mouse_button == MouseButton::Left {
                let _: () = msg_send![button, performClick: nil];
                let _: () = msg_send![button, highlight: true];
            }
        }
    }

    fn set_tooltip(&self, tooltip: Option<&str>) {
        unsafe {
            let tooltip = NSString::alloc(nil).init_str(tooltip.unwrap_or_default());
            let _: () = msg_send![self.ns_status_bar.button(), setToolTip: tooltip];
        }
    }

    fn set_title(&self, title: Option<&str>) {
        unsafe {
            NSButton::setTitle_(
                self.ns_status_bar.button(),
                NSString::alloc(nil).init_str(title.unwrap_or_default()),
            );
        }
    }

    fn set_icon(&self, icon: Option<Rc<Image>>) {
        unsafe {
            let button = self.ns_status_bar.button();
            let Some(icon) = icon.as_ref() else {
                button.setImage_(nil);
                return;
            };

            let image = icon.bytes();
            let nsdata = NSData::dataWithBytes_length_(
                nil,
                image.as_ptr() as *const std::os::raw::c_void,
                image.len() as u64,
            );

            let nsimage = NSImage::initWithData_(NSImage::alloc(nil), nsdata);
            let new_size = NSSize::new(18.0, 18.0);

            button.setImage_(nsimage);
            let _: () = msg_send![nsimage, setSize: new_size];
            let _: () = msg_send![button, setImagePosition: NSCellImagePosition::ImageLeft];
            let _: () = msg_send![nsimage, setTemplate: false];
        }
    }

    fn set_visible(&mut self, visible: bool) {
        if visible {
            self.ns_status_bar = Self::create_status_bar()
        } else {
            self.remove();
        }
    }

    fn remove(&mut self) {
        unsafe {
            let _: () =
                msg_send![NSStatusBar::systemStatusBar(nil), removeStatusItem: self.ns_status_bar];
        }
        self.ns_status_bar = nil;
    }
}
