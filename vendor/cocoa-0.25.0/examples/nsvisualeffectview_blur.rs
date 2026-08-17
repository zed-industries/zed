extern crate cocoa;
extern crate objc;

use objc::*;
use cocoa::base::{selector, nil, NO};

use cocoa::foundation::{NSRect, NSPoint, NSSize, NSAutoreleasePool, NSProcessInfo,
                        NSString};
use cocoa::appkit::{NSApp, NSColor, NSApplication, NSApplicationActivationPolicyRegular, NSMenu, NSMenuItem, NSWindowStyleMask, NSBackingStoreType, NSWindow, NSView, NSVisualEffectMaterial, NSVisualEffectView, NSViewWidthSizable, NSViewHeightSizable, NSVisualEffectBlendingMode, NSVisualEffectState, NSWindowOrderingMode};


fn main() {
    unsafe {
        // Create the app.
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


        // Create some colors
        let clear = NSColor::clearColor(nil);

        // Create windows with different color types.
        let window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
            NSRect::new(NSPoint::new(0., 0.), NSSize::new(200., 200.)),
            NSWindowStyleMask::NSTitledWindowMask |
                NSWindowStyleMask::NSClosableWindowMask |
                NSWindowStyleMask::NSResizableWindowMask |
                NSWindowStyleMask::NSMiniaturizableWindowMask |
                NSWindowStyleMask::NSUnifiedTitleAndToolbarWindowMask,
            NSBackingStoreType::NSBackingStoreBuffered,
            NO
        ).autorelease();

        window.cascadeTopLeftFromPoint_(NSPoint::new(20., 20.));
        window.setTitle_(NSString::alloc(nil).init_str("NSVisualEffectView_blur"));
        window.setBackgroundColor_(clear);
        window.makeKeyAndOrderFront_(nil);

        //NSVisualEffectView blur
        let ns_view = window.contentView();
        let bounds = NSView::bounds(ns_view);
        let blurred_view = NSVisualEffectView::initWithFrame_(NSVisualEffectView::alloc(nil), bounds);
        blurred_view.autorelease();

        blurred_view.setMaterial_(NSVisualEffectMaterial::HudWindow);
        blurred_view.setBlendingMode_(NSVisualEffectBlendingMode::BehindWindow);
        blurred_view.setState_(NSVisualEffectState::FollowsWindowActiveState);
        blurred_view.setAutoresizingMask_(NSViewWidthSizable | NSViewHeightSizable);

        let _: () = msg_send![ns_view, addSubview: blurred_view positioned: NSWindowOrderingMode::NSWindowBelow relativeTo: 0];

        app.run();
    }
}

