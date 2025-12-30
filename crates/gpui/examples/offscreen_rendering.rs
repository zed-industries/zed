//! Off-Screen Rendering Example
//!
//! This example demonstrates how to use GPUI's off-screen rendering capabilities.
//! Off-screen rendering allows you to render GPUI content to a texture without
//! displaying it in a window, which is useful for:
//!
//! - Embedding GPUI views in other applications (e.g., CEF, Electron)
//! - Headless rendering for testing or screenshots
//! - Video capture without a visible window
//! - Zero-copy texture sharing between processes
//!
//! Note: This example requires platform support for off-screen rendering.
//! Currently supported on Windows with DirectX 11.

use gpui::{App, Application, DevicePixels, OffScreenRenderer, OffScreenTargetConfig, size};

fn main() {
    Application::new().run(|cx: &mut App| {
        println!("=== GPUI Off-Screen Rendering Example ===\n");

        // Check if the platform supports off-screen rendering
        if !cx.supports_offscreen_rendering() {
            println!("❌ Off-screen rendering is not supported on this platform.");
            println!("   Currently supported: Windows (DirectX 11)");
            cx.quit();
            return;
        }

        println!("✓ Off-screen rendering is supported on this platform.\n");

        // Create an off-screen target configuration
        let config = OffScreenTargetConfig::new(size(DevicePixels(800), DevicePixels(600)));

        println!("Creating off-screen target: 800x600 pixels");

        // Create an off-screen renderer
        let renderer = match cx.create_offscreen_renderer(config) {
            Some(r) => r,
            None => {
                println!("❌ Failed to create off-screen renderer.");
                cx.quit();
                return;
            }
        };

        println!("✓ Off-screen renderer created successfully.\n");

        // Display information about the renderer
        display_renderer_info(&renderer);

        // Demonstrate resizing
        demonstrate_resize(renderer);

        // Create another renderer with sharing enabled
        demonstrate_shared_texture(cx);

        println!("\n=== Example Complete ===");
        cx.quit();
    });
}

fn display_renderer_info(renderer: &OffScreenRenderer) {
    let size = renderer.size();
    let format = renderer.pixel_format();

    println!("Renderer Information:");
    println!("  Size: {}x{} pixels", size.width.0, size.height.0);
    println!("  Pixel Format: {:?}", format);
    println!(
        "  Supports Shared Textures: {}",
        renderer.supports_shared_textures()
    );

    if let Some(handle) = renderer.shared_texture_handle() {
        println!("  Shared Texture Handle: {:?}", handle);
    }
    println!();
}

fn demonstrate_resize(mut renderer: OffScreenRenderer) {
    println!("Demonstrating resize...");

    let original_size = renderer.size();
    println!(
        "  Original size: {}x{}",
        original_size.width.0, original_size.height.0
    );

    // Resize to a new size
    renderer.resize(size(DevicePixels(1024), DevicePixels(768)));

    let new_size = renderer.size();
    println!("  New size: {}x{}", new_size.width.0, new_size.height.0);

    println!("✓ Resize completed.\n");
}

fn demonstrate_shared_texture(cx: &mut App) {
    println!("Creating off-screen target with sharing enabled...");

    let config =
        OffScreenTargetConfig::new(size(DevicePixels(640), DevicePixels(480))).with_sharing();

    let renderer = match cx.create_offscreen_renderer(config) {
        Some(r) => r,
        None => {
            println!("❌ Failed to create shared off-screen renderer.");
            return;
        }
    };

    println!("✓ Shared off-screen renderer created.\n");

    println!("Shared Renderer Information:");
    println!(
        "  Supports Shared Textures: {}",
        renderer.supports_shared_textures()
    );

    if let Some(handle) = renderer.shared_texture_handle() {
        println!("  ✓ Shared texture handle obtained!");
        println!("    Handle details: {:?}", handle);
        println!("\n  This handle can be used to share the texture with:");
        println!("    - Other DirectX 11 applications (Windows)");
        println!("    - CEF/Chromium for browser embedding");
        println!("    - Video encoding pipelines");
        println!("    - Other processes via handle duplication");
    } else {
        println!("  ℹ No shared texture handle available.");
        println!("    This may be expected if sharing is not supported.");
    }
}
