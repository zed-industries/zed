extern crate cocoa;

use cocoa::base::{selector, id, nil, NO};

use cocoa::foundation::{NSRect, NSPoint, NSSize, NSAutoreleasePool, NSProcessInfo,
                        NSString};
use cocoa::appkit::{NSApp, NSColor, NSColorSpace, NSApplication, NSApplicationActivationPolicyRegular,
                    NSMenu, NSMenuItem, NSWindowStyleMask, NSBackingStoreType, NSWindow,
                    NSRunningApplication, NSApplicationActivateIgnoringOtherApps};


fn main() {
    unsafe {
        // Create the app.
        let app = create_app();
        
        // Create some colors
        let clear = NSColor::clearColor(nil);
        let black = NSColor::colorWithRed_green_blue_alpha_(nil, 0.0, 0.0, 0.0, 1.0);
        let srgb_red = NSColor::colorWithSRGBRed_green_blue_alpha_(nil, 1.0, 0.0, 0.0, 1.0);
        let device_green = NSColor::colorWithDeviceRed_green_blue_alpha_(nil, 0.0, 1.0, 0.0, 1.0);
        let display_p3_blue = NSColor::colorWithDisplayP3Red_green_blue_alpha_(nil, 0.0, 0.0, 1.0, 1.0);
        let calibrated_cyan = NSColor::colorWithCalibratedRed_green_blue_alpha_(nil, 0.0, 1.0, 1.0, 1.0);

        // Create windows with different color types.
        let _win_clear = create_window(NSString::alloc(nil).init_str("clear"), clear);
        let _win_black = create_window(NSString::alloc(nil).init_str("black"), black);
        let _win_srgb_red = create_window(NSString::alloc(nil).init_str("srgb_red"), srgb_red);
        let _win_device_green = create_window(NSString::alloc(nil).init_str("device_green"), device_green);
        let _win_display_p3_blue = create_window(NSString::alloc(nil).init_str("display_p3_blue"), display_p3_blue);
        let _win_calibrated_cyan = create_window(NSString::alloc(nil).init_str("calibrated_cyan"), calibrated_cyan);

        // Extract component values from a color.
        // NOTE: some components will raise an exception if the color is not
        // in the correct NSColorSpace. Refer to Apple's documentation for details.
        // https://developer.apple.com/documentation/appkit/nscolor?language=objc
        let my_color = NSColor::colorWithRed_green_blue_alpha_(nil, 0.25, 0.75, 0.5, 0.25);
        println!("alphaComponent: {:?}", my_color.alphaComponent());
        println!("redComponent: {:?}", my_color.redComponent());
        println!("greenComponent: {:?}", my_color.greenComponent());
        println!("blueComponent: {:?}", my_color.blueComponent());
        println!("hueComponent: {:?}", my_color.hueComponent());
        println!("saturationComponent: {:?}", my_color.saturationComponent());
        println!("brightnessComponent: {:?}", my_color.brightnessComponent());
        
        // Changing color spaces.
        let my_color_cmyk_cs = my_color.colorUsingColorSpace_(NSColorSpace::deviceCMYKColorSpace(nil));
        println!("blackComponent: {:?}", my_color_cmyk_cs.blackComponent());
        println!("cyanComponent: {:?}", my_color_cmyk_cs.cyanComponent());
        println!("magentaComponent: {:?}", my_color_cmyk_cs.magentaComponent());
        println!("yellowComponent: {:?}", my_color_cmyk_cs.yellowComponent());

        // Getting NSColorSpace name.
        let cs = NSColorSpace::genericGamma22GrayColorSpace(nil);
        let cs_name = cs.localizedName();
        let cs_name_bytes = cs_name.UTF8String() as *const u8;
        let cs_name_string = std::str::from_utf8(std::slice::from_raw_parts(cs_name_bytes, cs_name.len())).unwrap();
        println!("NSColorSpace: {:?}", cs_name_string);

        // Creating an NSColorSpace from CGColorSpaceRef.
        let cg_cs = cs.CGColorSpace();
        let cs = NSColorSpace::alloc(nil).initWithCGColorSpace_(cg_cs);
        let cs_name = cs.localizedName();
        let cs_name_bytes = cs_name.UTF8String() as *const u8;
        let cs_name_string = std::str::from_utf8(std::slice::from_raw_parts(cs_name_bytes, cs_name.len())).unwrap();
        println!("initWithCGColorSpace_: {:?}", cs_name_string);

        app.run();
    }
}

unsafe fn create_window(title: id, color: id) -> id {
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
    window.setTitle_(title);
    window.setBackgroundColor_(color);
    window.makeKeyAndOrderFront_(nil);
    window
}

unsafe fn create_app() -> id {
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

    let current_app = NSRunningApplication::currentApplication(nil);
    current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);

    return app;
}
