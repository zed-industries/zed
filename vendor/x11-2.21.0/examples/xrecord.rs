// Example for X Record Extension

#![cfg_attr(not(feature = "xlib"), allow(dead_code))]
#![cfg_attr(not(feature = "xlib"), allow(unused_imports))]

extern crate libc;
extern crate x11;

use std::ffi::CString;
use std::ptr::{null, null_mut};

use std::os::raw::c_int;
use x11::xlib;
use x11::xrecord;

static mut EVENT_COUNT: u32 = 0;

#[cfg(not(all(feature = "xlib", feature = "xrecord")))]
fn main() {
    panic!("this example requires `--features 'xlib xrecord'`");
}

#[cfg(all(feature = "xlib", feature = "xrecord"))]
fn main() {
    unsafe {
        // Open displays
        let dpy_control = xlib::XOpenDisplay(null());
        let dpy_data = xlib::XOpenDisplay(null());
        if dpy_control == null_mut() || dpy_data == null_mut() {
            panic!("can't open display");
        }
        // Enable synchronization
        xlib::XSynchronize(dpy_control, 1);

        let extension_name = CString::new("RECORD").unwrap();

        let extension = xlib::XInitExtension(dpy_control, extension_name.as_ptr());
        if extension.is_null() {
            panic!("Error init X Record Extension");
        }

        // Get version
        let mut version_major: c_int = 0;
        let mut version_minor: c_int = 0;
        xrecord::XRecordQueryVersion(dpy_control, &mut version_major, &mut version_minor);
        println!(
            "RECORD extension version {}.{}",
            version_major, version_minor
        );

        // Prepare record range
        let mut record_range: xrecord::XRecordRange = *xrecord::XRecordAllocRange();
        record_range.device_events.first = xlib::KeyPress as u8;
        record_range.device_events.last = xlib::MotionNotify as u8;

        // Create context
        let context = xrecord::XRecordCreateContext(
            dpy_control,
            0,
            &mut xrecord::XRecordAllClients,
            1,
            std::mem::transmute(&mut &mut record_range),
            1,
        );

        if context == 0 {
            panic!("Fail create Record context\n");
        }

        // Run
        let result =
            xrecord::XRecordEnableContext(dpy_data, context, Some(record_callback), &mut 0);
        if result == 0 {
            panic!("Cound not enable the Record context!\n");
        }
    }
}

unsafe extern "C" fn record_callback(_: *mut i8, raw_data: *mut xrecord::XRecordInterceptData) {
    EVENT_COUNT += 1;
    let data = &*raw_data;

    // Skip server events
    if data.category != xrecord::XRecordFromServer {
        return;
    }

    // Cast binary data
    let xdatum = &*(data.data as *mut XRecordDatum);

    let event_type = match xdatum.xtype as i32 {
        xlib::KeyPress => "KeyPress",
        xlib::KeyRelease => "KeyRelease",
        xlib::ButtonPress => "ButtonPress",
        xlib::ButtonRelease => "ButtonRelease",
        xlib::MotionNotify => "MotionNotify",
        _ => "Other",
    };

    println!("Event recieve\t{:?}\tevent.", event_type);

    xrecord::XRecordFreeData(raw_data);
}

#[repr(C)]
struct XRecordDatum {
    xtype: u8,
    code: u8,
    unknown1: u8,
    unknown2: u8,
}
