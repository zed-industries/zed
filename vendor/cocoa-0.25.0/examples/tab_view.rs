extern crate cocoa;

use cocoa::base::{selector, id, nil, NO};


use cocoa::foundation::{NSRect, NSPoint, NSSize, NSAutoreleasePool, NSProcessInfo,
                        NSString};
use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicyRegular, NSWindow,
                    NSMenu, NSMenuItem, NSTabView, NSWindowStyleMask, NSBackingStoreType,
                    NSTabViewItem, NSRunningApplication, NSApplicationActivateIgnoringOtherApps};


fn main() {
    unsafe {

        // create a tab View
        let tab_view = NSTabView::new(nil)
            .initWithFrame_(NSRect::new(NSPoint::new(0., 0.), NSSize::new(200., 200.)));

        // create a tab view item
        let tab_view_item = NSTabViewItem::new(nil)
            .initWithIdentifier_(NSString::alloc(nil).init_str("TabView1"));

        tab_view_item.setLabel_(NSString::alloc(nil).init_str("Tab view item 1"));
        tab_view.addTabViewItem_(tab_view_item);

        // create a second tab view item
        let tab_view_item2 = NSTabViewItem::new(nil)
            .initWithIdentifier_(NSString::alloc(nil).init_str("TabView2"));

        tab_view_item2.setLabel_(NSString::alloc(nil).init_str("Tab view item 2"));
        tab_view.addTabViewItem_(tab_view_item2);

        // Create the app and set the content.
        let app = create_app(NSString::alloc(nil).init_str("Tab View"), tab_view);
        app.run();
    }
}

unsafe fn create_app(title: id, content: id) -> id {
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

    // create Window
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
    window.center();

    window.setTitle_(title);
    window.makeKeyAndOrderFront_(nil);

    window.setContentView_(content);
    let current_app = NSRunningApplication::currentApplication(nil);
    current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);

    return app;
}
