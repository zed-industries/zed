use crate::Tray;
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

pub struct MacTray {
    visible: bool,
    pub(crate) ns_status_bar: id,
}

impl MacTray {
    pub(crate) fn create(tray: &Tray, ns_menu: Option<id>) -> Self {
        let mut this = Self {
            ns_status_bar: Self::create_status_bar(),
            visible: tray.visible,
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

    pub(crate) fn update(&mut self, tray: &Tray, ns_menu: Option<id>) {
        self.set_visible(tray.visible);
        if !tray.visible {
            return;
        }

        unsafe {
            let button = self.ns_status_bar.button();

            if let Some(ns_menu) = &ns_menu {
                let _: () = msg_send![self.ns_status_bar, setMenu: *ns_menu];
            }

            let tooltip = NSString::alloc(nil).init_str(
                tray.tooltip
                    .as_ref()
                    .map(|s| s.as_ref())
                    .unwrap_or_default(),
            );
            let _: () = msg_send![self.ns_status_bar.button(), setToolTip: tooltip];

            let title = NSString::alloc(nil)
                .init_str(tray.title.as_ref().map(|s| s.as_ref()).unwrap_or_default());
            let _: () = msg_send![button, setTitle:title];

            if let Some(icon) = tray.icon.as_ref() {
                let bytes = icon.bytes();
                let nsdata = NSData::dataWithBytes_length_(
                    nil,
                    bytes.as_ptr() as *const _,
                    bytes.len() as u64,
                );

                let nsimage = NSImage::initWithData_(NSImage::alloc(nil), nsdata);
                assert!(
                    !nsimage.is_null(),
                    "Failed to create NSImage from icon data, possibly invalid image data."
                );
                let new_size = NSSize::new(18., 18.);

                button.setImage_(nsimage);
                let _: () = msg_send![nsimage, setSize: new_size];
                let _: () = msg_send![button, setImagePosition: NSCellImagePosition::ImageLeft];
                let _: () = msg_send![nsimage, setTemplate: false];
            } else {
                button.setImage_(nil);
            }
        }
    }

    fn set_visible(&mut self, visible: bool) {
        if self.visible == visible {
            return;
        }

        self.visible = visible;
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
