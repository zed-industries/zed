//! Utility: Capture Screenshots of Running Zed Windows
//!
//! This utility finds running Zed windows and captures screenshots of them.
//! It can be used for debugging, documentation, or visual testing.
//!
//! Usage:
//!   cargo run -p gpui --example capture_zed
//!
//! Options (via environment variables):
//!   CAPTURE_OUTPUT_DIR - Directory to save screenshots (default: current directory)
//!   CAPTURE_WINDOW_INDEX - Which Zed window to capture, 0-indexed (default: all)
//!
//! Note: This requires macOS and Screen Recording permissions.
//! The first time you run this, macOS will prompt you to grant permission.

use std::path::PathBuf;

fn main() {
    #[cfg(target_os = "macos")]
    {
        macos::run();
    }

    #[cfg(not(target_os = "macos"))]
    {
        eprintln!("This utility only works on macOS");
        std::process::exit(1);
    }
}

#[cfg(target_os = "macos")]
mod macos {
    use std::path::PathBuf;

    // FFI declarations for CoreGraphics window list
    #[link(name = "CoreGraphics", kind = "framework")]
    unsafe extern "C" {
        fn CGWindowListCopyWindowInfo(option: u32, relativeToWindow: u32) -> CFArrayRef;
        fn CGWindowListCreateImage(
            rect: CGRect,
            list_option: u32,
            window_id: u32,
            image_option: u32,
        ) -> CGImageRef;
        fn CGImageGetWidth(image: CGImageRef) -> usize;
        fn CGImageGetHeight(image: CGImageRef) -> usize;
        fn CGImageGetBytesPerRow(image: CGImageRef) -> usize;
        fn CGImageGetDataProvider(image: CGImageRef) -> CGDataProviderRef;
        fn CGImageRelease(image: CGImageRef);
        fn CGDataProviderCopyData(provider: CGDataProviderRef) -> CFDataRef;
    }

    #[link(name = "CoreFoundation", kind = "framework")]
    unsafe extern "C" {
        fn CFArrayGetCount(array: CFArrayRef) -> isize;
        fn CFArrayGetValueAtIndex(array: CFArrayRef, idx: isize) -> *const std::ffi::c_void;
        fn CFDictionaryGetValue(
            dict: CFDictionaryRef,
            key: *const std::ffi::c_void,
        ) -> *const std::ffi::c_void;
        fn CFStringCreateWithCString(
            alloc: *const std::ffi::c_void,
            cstr: *const i8,
            encoding: u32,
        ) -> CFStringRef;
        fn CFStringGetCStringPtr(string: CFStringRef, encoding: u32) -> *const i8;
        fn CFNumberGetValue(
            number: CFNumberRef,
            theType: i32,
            valuePtr: *mut std::ffi::c_void,
        ) -> bool;
        fn CFDataGetLength(data: CFDataRef) -> isize;
        fn CFDataGetBytePtr(data: CFDataRef) -> *const u8;
        fn CFRelease(cf: *const std::ffi::c_void);
    }

    type CFArrayRef = *const std::ffi::c_void;
    type CFDictionaryRef = *const std::ffi::c_void;
    type CFStringRef = *const std::ffi::c_void;
    type CFNumberRef = *const std::ffi::c_void;
    type CGImageRef = *mut std::ffi::c_void;
    type CGDataProviderRef = *mut std::ffi::c_void;
    type CFDataRef = *mut std::ffi::c_void;

    #[repr(C)]
    #[derive(Copy, Clone)]
    struct CGPoint {
        x: f64,
        y: f64,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    struct CGSize {
        width: f64,
        height: f64,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    struct CGRect {
        origin: CGPoint,
        size: CGSize,
    }

    impl CGRect {
        fn null() -> Self {
            CGRect {
                origin: CGPoint {
                    x: f64::INFINITY,
                    y: f64::INFINITY,
                },
                size: CGSize {
                    width: 0.0,
                    height: 0.0,
                },
            }
        }
    }

    // Constants
    #[allow(non_upper_case_globals)]
    const kCGWindowListOptionOnScreenOnly: u32 = 1 << 0;
    #[allow(non_upper_case_globals)]
    const kCGWindowListExcludeDesktopElements: u32 = 1 << 4;
    #[allow(non_upper_case_globals)]
    const kCGWindowListOptionIncludingWindow: u32 = 1 << 3;
    #[allow(non_upper_case_globals)]
    const kCGWindowImageBoundsIgnoreFraming: u32 = 1 << 0;
    #[allow(non_upper_case_globals)]
    const kCFStringEncodingUTF8: u32 = 0x08000100;
    #[allow(non_upper_case_globals)]
    const kCFNumberSInt32Type: i32 = 3;

    #[derive(Debug)]
    struct WindowInfo {
        window_id: u32,
        owner_name: String,
        window_name: String,
        bounds: (f64, f64, f64, f64), // x, y, width, height
    }

    fn get_cf_string(key: &str) -> CFStringRef {
        unsafe {
            let cstr = std::ffi::CString::new(key).unwrap();
            CFStringCreateWithCString(std::ptr::null(), cstr.as_ptr(), kCFStringEncodingUTF8)
        }
    }

    fn cf_string_to_rust(cf_string: CFStringRef) -> Option<String> {
        if cf_string.is_null() {
            return None;
        }
        unsafe {
            let ptr = CFStringGetCStringPtr(cf_string, kCFStringEncodingUTF8);
            if ptr.is_null() {
                return None;
            }
            Some(std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned())
        }
    }

    fn cf_number_to_i32(cf_number: CFNumberRef) -> Option<i32> {
        if cf_number.is_null() {
            return None;
        }
        unsafe {
            let mut value: i32 = 0;
            if CFNumberGetValue(
                cf_number,
                kCFNumberSInt32Type,
                &mut value as *mut i32 as *mut std::ffi::c_void,
            ) {
                Some(value)
            } else {
                None
            }
        }
    }

    fn get_zed_windows() -> Vec<WindowInfo> {
        let mut windows = Vec::new();

        unsafe {
            let window_list = CGWindowListCopyWindowInfo(
                kCGWindowListOptionOnScreenOnly | kCGWindowListExcludeDesktopElements,
                0,
            );

            if window_list.is_null() {
                return windows;
            }

            let count = CFArrayGetCount(window_list);

            let key_owner_name = get_cf_string("kCGWindowOwnerName");
            let key_window_name = get_cf_string("kCGWindowName");
            let key_window_number = get_cf_string("kCGWindowNumber");
            let key_bounds = get_cf_string("kCGWindowBounds");
            let key_x = get_cf_string("X");
            let key_y = get_cf_string("Y");
            let key_width = get_cf_string("Width");
            let key_height = get_cf_string("Height");

            for i in 0..count {
                let dict = CFArrayGetValueAtIndex(window_list, i) as CFDictionaryRef;
                if dict.is_null() {
                    continue;
                }

                // Get owner name
                let owner_name_cf = CFDictionaryGetValue(dict, key_owner_name) as CFStringRef;
                let owner_name = cf_string_to_rust(owner_name_cf).unwrap_or_default();

                // Check if this is a Zed window
                if !owner_name.contains("Zed") {
                    continue;
                }

                // Get window name
                let window_name_cf = CFDictionaryGetValue(dict, key_window_name) as CFStringRef;
                let window_name = cf_string_to_rust(window_name_cf).unwrap_or_default();

                // Get window ID
                let window_number_cf = CFDictionaryGetValue(dict, key_window_number) as CFNumberRef;
                let window_id = cf_number_to_i32(window_number_cf).unwrap_or(0) as u32;

                // Get bounds
                let bounds_dict = CFDictionaryGetValue(dict, key_bounds) as CFDictionaryRef;
                let (x, y, width, height) = if !bounds_dict.is_null() {
                    let x_cf = CFDictionaryGetValue(bounds_dict, key_x) as CFNumberRef;
                    let y_cf = CFDictionaryGetValue(bounds_dict, key_y) as CFNumberRef;
                    let w_cf = CFDictionaryGetValue(bounds_dict, key_width) as CFNumberRef;
                    let h_cf = CFDictionaryGetValue(bounds_dict, key_height) as CFNumberRef;

                    (
                        cf_number_to_i32(x_cf).unwrap_or(0) as f64,
                        cf_number_to_i32(y_cf).unwrap_or(0) as f64,
                        cf_number_to_i32(w_cf).unwrap_or(0) as f64,
                        cf_number_to_i32(h_cf).unwrap_or(0) as f64,
                    )
                } else {
                    (0.0, 0.0, 0.0, 0.0)
                };

                // Skip windows with zero size (like menu bar items)
                if width < 100.0 || height < 100.0 {
                    continue;
                }

                windows.push(WindowInfo {
                    window_id,
                    owner_name,
                    window_name,
                    bounds: (x, y, width, height),
                });
            }

            // Clean up CF strings
            CFRelease(key_owner_name);
            CFRelease(key_window_name);
            CFRelease(key_window_number);
            CFRelease(key_bounds);
            CFRelease(key_x);
            CFRelease(key_y);
            CFRelease(key_width);
            CFRelease(key_height);
            CFRelease(window_list);
        }

        windows
    }

    fn capture_window_to_png(
        window_id: u32,
        output_path: &std::path::Path,
    ) -> Result<(usize, usize), Box<dyn std::error::Error>> {
        use std::fs::File;
        use std::io::BufWriter;

        // Capture the window
        let image = unsafe {
            CGWindowListCreateImage(
                CGRect::null(),
                kCGWindowListOptionIncludingWindow,
                window_id,
                kCGWindowImageBoundsIgnoreFraming,
            )
        };

        if image.is_null() {
            return Err("Failed to capture window - image is null. \
                        Make sure Screen Recording permission is granted in \
                        System Preferences > Privacy & Security > Screen Recording."
                .into());
        }

        // Get image dimensions
        let width = unsafe { CGImageGetWidth(image) };
        let height = unsafe { CGImageGetHeight(image) };

        if width == 0 || height == 0 {
            unsafe { CGImageRelease(image) };
            return Err("Captured image has zero dimensions".into());
        }

        // Get the image data
        let data_provider = unsafe { CGImageGetDataProvider(image) };
        if data_provider.is_null() {
            unsafe { CGImageRelease(image) };
            return Err("Failed to get image data provider".into());
        }

        let data = unsafe { CGDataProviderCopyData(data_provider) };
        if data.is_null() {
            unsafe { CGImageRelease(image) };
            return Err("Failed to copy image data".into());
        }

        let length = unsafe { CFDataGetLength(data) } as usize;
        let ptr = unsafe { CFDataGetBytePtr(data) };
        let bytes = unsafe { std::slice::from_raw_parts(ptr, length) };
        let bytes_per_row = unsafe { CGImageGetBytesPerRow(image) };

        // The image is in BGRA format with potential row padding, convert to RGBA for PNG
        let mut rgba_bytes = Vec::with_capacity(width * height * 4);
        for row in 0..height {
            let row_start = row * bytes_per_row;
            for col in 0..width {
                let pixel_start = row_start + col * 4;
                if pixel_start + 3 < length {
                    rgba_bytes.push(bytes[pixel_start + 2]); // R (was B)
                    rgba_bytes.push(bytes[pixel_start + 1]); // G
                    rgba_bytes.push(bytes[pixel_start]); // B (was R)
                    rgba_bytes.push(bytes[pixel_start + 3]); // A
                }
            }
        }

        // Write PNG file
        let file = File::create(output_path)?;
        let w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, width as u32, height as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&rgba_bytes)?;

        // Cleanup
        unsafe {
            CFRelease(data as *const _);
            CGImageRelease(image);
        }

        Ok((width, height))
    }

    pub fn run() {
        println!("Looking for Zed windows...\n");

        let windows = get_zed_windows();

        if windows.is_empty() {
            eprintln!("No Zed windows found!");
            eprintln!("\nMake sure Zed is running and visible on screen.");
            eprintln!("Note: Minimized windows cannot be captured.");
            std::process::exit(1);
        }

        println!("Found {} Zed window(s):\n", windows.len());
        for (i, window) in windows.iter().enumerate() {
            println!(
                "  [{}] Window ID: {}, Title: \"{}\", Size: {}x{}",
                i, window.window_id, window.window_name, window.bounds.2, window.bounds.3
            );
        }
        println!();

        // Get output directory
        let output_dir = std::env::var("CAPTURE_OUTPUT_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

        // Get window index filter
        let window_index_filter: Option<usize> = std::env::var("CAPTURE_WINDOW_INDEX")
            .ok()
            .and_then(|s| s.parse().ok());

        // Capture windows
        let windows_to_capture: Vec<_> = match window_index_filter {
            Some(idx) => {
                if idx < windows.len() {
                    vec![&windows[idx]]
                } else {
                    eprintln!(
                        "Window index {} is out of range (0-{})",
                        idx,
                        windows.len() - 1
                    );
                    std::process::exit(1);
                }
            }
            None => windows.iter().collect(),
        };

        println!("Capturing {} window(s)...\n", windows_to_capture.len());

        for (i, window) in windows_to_capture.iter().enumerate() {
            let filename = if window.window_name.is_empty() {
                format!("zed_window_{}.png", i)
            } else {
                // Sanitize window name for filename
                let safe_name: String = window
                    .window_name
                    .chars()
                    .map(|c| {
                        if c.is_alphanumeric() || c == '-' || c == '_' {
                            c
                        } else {
                            '_'
                        }
                    })
                    .collect();
                format!("zed_{}.png", safe_name)
            };

            let output_path = output_dir.join(&filename);

            match capture_window_to_png(window.window_id, &output_path) {
                Ok((width, height)) => {
                    println!(
                        "✓ Captured \"{}\" -> {} ({}x{})",
                        window.window_name,
                        output_path.display(),
                        width,
                        height
                    );
                }
                Err(e) => {
                    eprintln!("✗ Failed to capture \"{}\": {}", window.window_name, e);
                }
            }
        }

        println!("\nDone!");
    }
}
