use cocoa::{
    appkit::{NSSquareStatusItemLength, NSStatusBar, NSStatusItem, NSView},
    base::{id, nil, NO, YES},
    foundation::NSRect,
    quartzcore::AutoresizingMask,
};
use core_foundation::base::TCFType;
use core_graphics::color::CGColor;
use foreign_types::ForeignType;
use objc::{class, msg_send, rc::StrongPtr, sel, sel_impl};

pub struct StatusItem(StrongPtr);

impl StatusItem {
    pub fn add() -> Self {
        const PIXEL_FORMAT: metal::MTLPixelFormat = metal::MTLPixelFormat::BGRA8Unorm;

        unsafe {
            let status_bar = NSStatusBar::systemStatusBar(nil);
            let native_item =
                StrongPtr::retain(status_bar.statusItemWithLength_(NSSquareStatusItemLength));
            native_item.button().setWantsLayer(true);

            let device: metal::Device = if let Some(device) = metal::Device::system_default() {
                device
            } else {
                log::error!("unable to access a compatible graphics device");
                std::process::exit(1);
            };

            let layer: id = msg_send![class!(CAMetalLayer), layer];
            let _: () = msg_send![layer, setDevice: device.as_ptr()];
            let _: () = msg_send![layer, setPixelFormat: PIXEL_FORMAT];
            let _: () = msg_send![layer, setAllowsNextDrawableTimeout: NO];
            let _: () = msg_send![layer, setNeedsDisplayOnBoundsChange: YES];
            let _: () = msg_send![layer, setPresentsWithTransaction: YES];
            let _: () = msg_send![
                layer,
                setAutoresizingMask: AutoresizingMask::WIDTH_SIZABLE
                    | AutoresizingMask::HEIGHT_SIZABLE
            ];
            let _: () = msg_send![
                layer,
                setBackgroundColor: CGColor::rgb(1., 0., 0., 1.).as_concrete_TypeRef()
            ];

            let _: () = msg_send![native_item.button(), setLayer: layer];
            let native_item_window: id = msg_send![native_item.button(), window];

            dbg!(native_item_window.frame().as_CGRect());
            // let rect_in_window: NSRect = msg_send![native_item.button(), convertRect: native_item.button().bounds() toView: nil];
            // let screen_rect: NSRect =
            //     msg_send![native_item_window, convertRectToScreen: rect_in_window];
            // dbg!(screen_rect.as_CGRect());

            StatusItem(native_item)
        }
    }
}

impl crate::StatusItem for StatusItem {}
