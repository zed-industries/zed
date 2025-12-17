//! Example: Off-screen Window Rendering with Screenshots
//!
//! This example demonstrates how to:
//! 1. Create a window positioned off-screen (so it's not visible to the user)
//! 2. Render real GPUI content using Metal
//! 3. Take screenshots of the window using CGWindowListCreateImage
//! 4. Save the screenshots as PNG files
//!
//! This is useful for automated visual testing where you want real rendering
//! but don't want windows appearing on screen.
//!
//! Usage:
//!   cargo run -p gpui --example screenshot
//!
//! Note: This requires macOS and Screen Recording permissions.
//! The first time you run this, macOS will prompt you to grant permission.

use gpui::{
    App, AppContext, Application, Bounds, Context, Entity, IntoElement, Render, SharedString,
    Window, WindowBounds, WindowHandle, WindowOptions, div, point, prelude::*, px, rgb, size,
};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use std::path::PathBuf;
use std::time::Duration;

// ============================================================================
// GPUI View to Render
// ============================================================================

struct ScreenshotDemo {
    counter: u32,
    message: SharedString,
}

impl ScreenshotDemo {
    fn new() -> Self {
        Self {
            counter: 0,
            message: "Hello, Screenshot!".into(),
        }
    }

    fn increment(&mut self) {
        self.counter += 1;
        self.message = format!("Counter: {}", self.counter).into();
    }
}

impl Render for ScreenshotDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .bg(rgb(0x1e1e2e)) // Dark background
            .size_full()
            .justify_center()
            .items_center()
            .child(
                div()
                    .text_3xl()
                    .text_color(rgb(0xcdd6f4))
                    .child(self.message.clone()),
            )
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(colored_box(rgb(0xf38ba8))) // Red
                    .child(colored_box(rgb(0xa6e3a1))) // Green
                    .child(colored_box(rgb(0x89b4fa))) // Blue
                    .child(colored_box(rgb(0xf9e2af))), // Yellow
            )
            .child(
                div()
                    .mt_4()
                    .px_4()
                    .py_2()
                    .bg(rgb(0x313244))
                    .rounded_md()
                    .text_color(rgb(0xbac2de))
                    .child(format!("Frame: {}", self.counter)),
            )
    }
}

fn colored_box(color: gpui::Rgba) -> impl IntoElement {
    div()
        .size_16()
        .bg(color)
        .rounded_lg()
        .shadow_md()
        .border_2()
        .border_color(rgb(0x45475a))
}

// ============================================================================
// Screenshot Capture (macOS-specific using CGWindowListCreateImage)
// ============================================================================

#[cfg(target_os = "macos")]
mod screenshot {
    use std::path::Path;

    // FFI declarations for CoreGraphics
    #[link(name = "CoreGraphics", kind = "framework")]
    unsafe extern "C" {
        fn CGWindowListCreateImage(
            rect: CGRect,
            list_option: u32,
            window_id: u32,
            image_option: u32,
        ) -> CGImageRef;

        fn CGImageGetWidth(image: CGImageRef) -> usize;
        fn CGImageGetHeight(image: CGImageRef) -> usize;
        fn CGImageGetDataProvider(image: CGImageRef) -> CGDataProviderRef;
        fn CGImageRelease(image: CGImageRef);
        fn CGDataProviderCopyData(provider: CGDataProviderRef) -> CFDataRef;
    }

    #[link(name = "CoreFoundation", kind = "framework")]
    unsafe extern "C" {
        fn CFDataGetLength(data: CFDataRef) -> isize;
        fn CFDataGetBytePtr(data: CFDataRef) -> *const u8;
        fn CFRelease(cf: *const std::ffi::c_void);
    }

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

    #[allow(non_upper_case_globals)]
    const kCGWindowListOptionIncludingWindow: u32 = 1 << 3;
    #[allow(non_upper_case_globals)]
    const kCGWindowImageBoundsIgnoreFraming: u32 = 1 << 0;

    /// Captures a screenshot of the specified window and saves it as a PNG.
    pub fn capture_window_to_png(
        window_number: i64,
        output_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs::File;
        use std::io::BufWriter;

        // Capture the window
        let image = unsafe {
            CGWindowListCreateImage(
                CGRect::null(),
                kCGWindowListOptionIncludingWindow,
                window_number as u32,
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

        // The image is in BGRA format, convert to RGBA for PNG
        let mut rgba_bytes = Vec::with_capacity(length);
        for chunk in bytes.chunks(4) {
            if chunk.len() == 4 {
                rgba_bytes.push(chunk[2]); // R (was B)
                rgba_bytes.push(chunk[1]); // G
                rgba_bytes.push(chunk[0]); // B (was R)
                rgba_bytes.push(chunk[3]); // A
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

        println!(
            "Screenshot saved to {} ({}x{})",
            output_path.display(),
            width,
            height
        );
        Ok(())
    }
}

#[cfg(not(target_os = "macos"))]
mod screenshot {
    use std::path::Path;

    pub fn capture_window_to_png(
        _window_number: i64,
        _output_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Err("Screenshot capture is only supported on macOS".into())
    }
}

// ============================================================================
// Main Application
// ============================================================================

fn main() {
    env_logger::init();

    Application::new().run(|cx: &mut App| {
        // Position the window FAR off-screen so it's not visible
        // but macOS still renders it (unlike minimized/hidden windows)
        let off_screen_origin = point(px(-10000.0), px(-10000.0));
        let window_size = size(px(800.0), px(600.0));

        let bounds = Bounds {
            origin: off_screen_origin,
            size: window_size,
        };

        println!("Creating off-screen window at {:?}", bounds);
        println!("(The window is positioned off-screen but is still being rendered by macOS)");

        // Open the window
        let window_handle: WindowHandle<ScreenshotDemo> = cx
            .open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    focus: false, // Don't steal focus
                    show: true,   // Must be true for rendering to occur
                    ..Default::default()
                },
                |_, cx| cx.new(|_| ScreenshotDemo::new()),
            )
            .expect("Failed to open window");

        // Get the entity for later updates
        let view_entity: Entity<ScreenshotDemo> =
            window_handle.entity(cx).expect("Failed to get root entity");

        // Get output directory
        let output_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

        // Schedule screenshot captures after allowing time for rendering
        cx.spawn(async move |cx| {
            // Wait for the window to fully render
            smol::Timer::after(Duration::from_millis(500)).await;

            // Get the window number for screenshots
            let window_number = cx
                .update(|app: &mut App| get_window_number_from_handle(&window_handle, app))
                .ok()
                .flatten();

            let Some(window_number) = window_number else {
                eprintln!("Could not get window number. Are you running on macOS?");
                let _ = cx.update(|app: &mut App| app.quit());
                return;
            };

            println!("Window number: {}", window_number);

            // Take screenshot 1
            let output_path = output_dir.join("screenshot_1.png");
            match screenshot::capture_window_to_png(window_number, &output_path) {
                Ok(()) => println!("✓ Captured screenshot_1.png"),
                Err(e) => eprintln!("✗ Failed to capture screenshot_1.png: {}", e),
            }

            // Update the view (update the entity directly, not through window_handle.update)
            let _ = cx.update_entity(&view_entity, |view: &mut ScreenshotDemo, ecx| {
                view.increment();
                view.increment();
                view.increment();
                ecx.notify(); // Trigger a re-render
            });

            // Wait for re-render
            smol::Timer::after(Duration::from_millis(200)).await;

            // Take screenshot 2
            let output_path = output_dir.join("screenshot_2.png");
            match screenshot::capture_window_to_png(window_number, &output_path) {
                Ok(()) => println!("✓ Captured screenshot_2.png"),
                Err(e) => eprintln!("✗ Failed to capture screenshot_2.png: {}", e),
            }

            // Update again
            let _ = cx.update_entity(&view_entity, |view: &mut ScreenshotDemo, ecx| {
                for _ in 0..7 {
                    view.increment();
                }
                ecx.notify(); // Trigger a re-render
            });

            // Wait for re-render
            smol::Timer::after(Duration::from_millis(200)).await;

            // Take screenshot 3
            let output_path = output_dir.join("screenshot_3.png");
            match screenshot::capture_window_to_png(window_number, &output_path) {
                Ok(()) => println!("✓ Captured screenshot_3.png"),
                Err(e) => eprintln!("✗ Failed to capture screenshot_3.png: {}", e),
            }

            println!("\nAll screenshots captured!");
            println!(
                "Check {} for screenshot_1.png, screenshot_2.png, screenshot_3.png",
                output_dir.display()
            );

            // Quit after screenshots are taken
            smol::Timer::after(Duration::from_millis(500)).await;
            let _ = cx.update(|app: &mut App| app.quit());
        })
        .detach();
    });
}

/// Extract the window number from a GPUI WindowHandle using raw_window_handle
#[cfg(target_os = "macos")]
fn get_window_number_from_handle<V: 'static + Render>(
    window_handle: &WindowHandle<V>,
    cx: &mut App,
) -> Option<i64> {
    use objc::{msg_send, sel, sel_impl};

    window_handle
        .update(cx, |_root: &mut V, window: &mut Window, _cx| {
            let handle = window.window_handle().ok()?;
            match handle.as_raw() {
                RawWindowHandle::AppKit(appkit_handle) => {
                    let ns_view = appkit_handle.ns_view.as_ptr();
                    unsafe {
                        let ns_window: *mut std::ffi::c_void =
                            msg_send![ns_view as cocoa::base::id, window];
                        if ns_window.is_null() {
                            return None;
                        }
                        let window_number: i64 =
                            msg_send![ns_window as cocoa::base::id, windowNumber];
                        Some(window_number)
                    }
                }
                _ => None,
            }
        })
        .ok()
        .flatten()
}

#[cfg(not(target_os = "macos"))]
fn get_window_number_from_handle<V: 'static + Render>(
    _window_handle: &WindowHandle<V>,
    _cx: &mut App,
) -> Option<i64> {
    None
}
