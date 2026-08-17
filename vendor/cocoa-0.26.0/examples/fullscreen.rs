use cocoa::appkit::{
    NSApp, NSApplication, NSApplicationActivateIgnoringOtherApps,
    NSApplicationActivationPolicyRegular, NSApplicationPresentationOptions, NSBackingStoreBuffered,
    NSMenu, NSMenuItem, NSRunningApplication, NSWindow, NSWindowCollectionBehavior,
    NSWindowStyleMask,
};
use cocoa::base::{id, nil, selector, NO};
use cocoa::foundation::{
    NSAutoreleasePool, NSPoint, NSProcessInfo, NSRect, NSSize, NSString, NSUInteger,
};

use core_graphics::display::CGDisplay;

use objc::declare::ClassDecl;
use objc::runtime::{Object, Sel};
use objc::{class, msg_send, sel, sel_impl};

fn main() {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

        let app = NSApp();
        app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

        // create Menu Bar
        let menubar = NSMenu::new(nil).autorelease();
        let app_menu_item = NSMenuItem::new(nil).autorelease();
        menubar.addItem_(app_menu_item);
        app.setMainMenu_(menubar);

        // create Application menu
        let app_menu = NSMenu::new(nil).autorelease();
        let quit_prefix = NSString::alloc(nil).init_str("Quit ");
        let quit_title =
            quit_prefix.stringByAppendingString_(NSProcessInfo::processInfo(nil).processName());
        let quit_action = selector("terminate:");
        let quit_key = NSString::alloc(nil).init_str("q");
        let quit_item = NSMenuItem::alloc(nil)
            .initWithTitle_action_keyEquivalent_(quit_title, quit_action, quit_key)
            .autorelease();
        app_menu.addItem_(quit_item);
        app_menu_item.setSubmenu_(app_menu);

        // Create NSWindowDelegate
        let superclass = class!(NSObject);
        let mut decl = ClassDecl::new("MyWindowDelegate", superclass).unwrap();

        extern "C" fn will_use_fillscreen_presentation_options(
            _: &Object,
            _: Sel,
            _: id,
            _: NSUInteger,
        ) -> NSUInteger {
            // Set initial presentation options for fullscreen
            let options = NSApplicationPresentationOptions::NSApplicationPresentationFullScreen
                | NSApplicationPresentationOptions::NSApplicationPresentationHideDock
                | NSApplicationPresentationOptions::NSApplicationPresentationHideMenuBar
                | NSApplicationPresentationOptions::NSApplicationPresentationDisableProcessSwitching;
            options.bits()
        }

        extern "C" fn window_entering_fullscreen(_: &Object, _: Sel, _: id) {
            // Reset HideDock and HideMenuBar settings during/after we entered fullscreen.
            let options = NSApplicationPresentationOptions::NSApplicationPresentationHideDock
                | NSApplicationPresentationOptions::NSApplicationPresentationHideMenuBar;
            unsafe {
                NSApp().setPresentationOptions_(options);
            }
        }

        decl.add_method(
            sel!(window:willUseFullScreenPresentationOptions:),
            will_use_fillscreen_presentation_options
                as extern "C" fn(&Object, Sel, id, NSUInteger) -> NSUInteger,
        );
        decl.add_method(
            sel!(windowWillEnterFullScreen:),
            window_entering_fullscreen as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(windowDidEnterFullScreen:),
            window_entering_fullscreen as extern "C" fn(&Object, Sel, id),
        );

        let delegate_class = decl.register();
        let delegate_object = msg_send![delegate_class, new];

        // create Window
        let display = CGDisplay::main();
        let size = NSSize::new(display.pixels_wide() as _, display.pixels_high() as _);
        let window = NSWindow::alloc(nil)
            .initWithContentRect_styleMask_backing_defer_(
                NSRect::new(NSPoint::new(0., 0.), size),
                NSWindowStyleMask::NSTitledWindowMask,
                NSBackingStoreBuffered,
                NO,
            )
            .autorelease();
        window.setDelegate_(delegate_object);
        let title = NSString::alloc(nil).init_str("Fullscreen!");
        window.setTitle_(title);
        window.makeKeyAndOrderFront_(nil);

        let current_app = NSRunningApplication::currentApplication(nil);
        current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
        window.setCollectionBehavior_(
            NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenPrimary,
        );
        window.toggleFullScreen_(nil);
        app.run();
    }
}
