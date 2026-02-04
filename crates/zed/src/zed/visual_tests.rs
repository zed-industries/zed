#![allow(dead_code, unused_imports)]

//! Visual testing infrastructure for Zed.
//!
//! This module provides utilities for visual regression testing of Zed's UI.
//! It allows capturing screenshots of the real Zed application window and comparing
//! them against baseline images.
//!
//! ## Important: Main Thread Requirement
//!
//! On macOS, the `VisualTestAppContext` must be created on the main thread.
//! Standard Rust tests run on worker threads, so visual tests that use
//! `VisualTestAppContext::new()` must be run with special consideration.
//!
//! ## Running Visual Tests
//!
//! Visual tests are marked with `#[ignore]` by default because:
//! 1. They require macOS with Screen Recording permission
//! 2. They need to run on the main thread
//! 3. They may produce different results on different displays/resolutions
//!
//! To run visual tests:
//! ```bash
//! # Run all visual tests (requires macOS, may need Screen Recording permission)
//! cargo test -p zed visual_tests -- --ignored --test-threads=1
//!
//! # Update baselines when UI intentionally changes
//! UPDATE_BASELINES=1 cargo test -p zed visual_tests -- --ignored --test-threads=1
//! ```
//!
//! ## Screenshot Output
//!
//! Screenshots are saved to the directory specified by `VISUAL_TEST_OUTPUT_DIR`
//! environment variable, or `target/visual_tests` by default.

use anyhow::{Result, anyhow};
use gpui::{
    AnyWindowHandle, AppContext as _, Empty, Size, VisualTestAppContext, WindowHandle, px, size,
};
use image::{ImageBuffer, Rgba, RgbaImage};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use workspace::AppState;

/// Initialize a visual test context with all necessary Zed subsystems.
pub fn init_visual_test(cx: &mut VisualTestAppContext) -> Arc<AppState> {
    cx.update(|cx| {
        env_logger::builder().is_test(true).try_init().ok();

        let app_state = AppState::test(cx);

        gpui_tokio::init(cx);
        theme::init(theme::LoadThemes::JustBase, cx);
        audio::init(cx);
        workspace::init(app_state.clone(), cx);
        release_channel::init(semver::Version::new(0, 0, 0), cx);
        command_palette::init(cx);
        editor::init(cx);
        project_panel::init(cx);
        outline_panel::init(cx);
        terminal_view::init(cx);
        image_viewer::init(cx);
        search::init(cx);

        app_state
    })
}

/// Open a test workspace with the given app state.
pub async fn open_test_workspace(
    app_state: Arc<AppState>,
    cx: &mut VisualTestAppContext,
) -> Result<WindowHandle<workspace::Workspace>> {
    let window_size = size(px(1280.0), px(800.0));

    let project = cx.update(|cx| {
        project::Project::local(
            app_state.client.clone(),
            app_state.node_runtime.clone(),
            app_state.user_store.clone(),
            app_state.languages.clone(),
            app_state.fs.clone(),
            None,
            project::LocalProjectFlags {
                init_worktree_trust: false,
                ..Default::default()
            },
            cx,
        )
    });

    let window = cx.open_offscreen_window(window_size, |window, cx| {
        cx.new(|cx| workspace::Workspace::new(None, project.clone(), app_state.clone(), window, cx))
    })?;

    cx.run_until_parked();

    Ok(window)
}

/// Returns the default window size for visual tests (1280x800).
pub fn default_window_size() -> Size<gpui::Pixels> {
    size(px(1280.0), px(800.0))
}

/// Waits for the UI to stabilize by running pending work and waiting for animations.
pub async fn wait_for_ui_stabilization(cx: &VisualTestAppContext) {
    cx.run_until_parked();
    cx.background_executor
        .timer(Duration::from_millis(100))
        .await;
    cx.run_until_parked();
}

/// Captures a screenshot of the given window and optionally saves it to a file.
///
/// # Arguments
/// * `cx` - The visual test context
/// * `window` - The window to capture
/// * `output_path` - Optional path to save the screenshot
///
/// # Returns
/// The captured screenshot as an RgbaImage
pub async fn capture_and_save_screenshot(
    cx: &mut VisualTestAppContext,
    window: AnyWindowHandle,
    output_path: Option<&Path>,
) -> Result<RgbaImage> {
    wait_for_ui_stabilization(cx).await;

    let screenshot = cx.capture_screenshot(window)?;

    if let Some(path) = output_path {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        screenshot.save(path)?;
        println!("Screenshot saved to: {}", path.display());
    }

    Ok(screenshot)
}

/// Check if we should update baselines (controlled by UPDATE_BASELINES env var).
pub fn should_update_baselines() -> bool {
    std::env::var("UPDATE_BASELINES").is_ok()
}

/// Assert that a screenshot matches a baseline, or update the baseline if UPDATE_BASELINES is set.
pub fn assert_or_update_baseline(
    actual: &RgbaImage,
    baseline_path: &Path,
    tolerance: f64,
    per_pixel_threshold: u8,
) -> Result<()> {
    if should_update_baselines() {
        save_baseline(actual, baseline_path)?;
        println!("Updated baseline: {}", baseline_path.display());
        Ok(())
    } else {
        assert_screenshot_matches(actual, baseline_path, tolerance, per_pixel_threshold)
    }
}

/// Result of comparing two screenshots.
#[derive(Debug)]
pub struct ScreenshotComparison {
    /// Percentage of pixels that match (0.0 to 1.0)
    pub match_percentage: f64,
    /// Optional diff image highlighting differences (red = different, green = same)
    pub diff_image: Option<RgbaImage>,
    /// Number of pixels that differ
    pub diff_pixel_count: u64,
    /// Total number of pixels compared
    pub total_pixels: u64,
}

impl ScreenshotComparison {
    /// Returns true if the images match within the given tolerance.
    pub fn matches(&self, tolerance: f64) -> bool {
        self.match_percentage >= (1.0 - tolerance)
    }
}

/// Compare two screenshots with tolerance for minor differences (e.g., anti-aliasing).
///
/// # Arguments
/// * `actual` - The screenshot to test
/// * `expected` - The baseline screenshot to compare against
/// * `per_pixel_threshold` - Maximum color difference per channel (0-255) to consider pixels equal
///
/// # Returns
/// A `ScreenshotComparison` containing match statistics and an optional diff image.
pub fn compare_screenshots(
    actual: &RgbaImage,
    expected: &RgbaImage,
    per_pixel_threshold: u8,
) -> ScreenshotComparison {
    let (width, height) = actual.dimensions();
    let (exp_width, exp_height) = expected.dimensions();

    if width != exp_width || height != exp_height {
        return ScreenshotComparison {
            match_percentage: 0.0,
            diff_image: None,
            diff_pixel_count: (width * height).max(exp_width * exp_height) as u64,
            total_pixels: (width * height).max(exp_width * exp_height) as u64,
        };
    }

    let total_pixels = (width * height) as u64;
    let mut diff_pixel_count = 0u64;
    let mut diff_image: RgbaImage = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let actual_pixel = actual.get_pixel(x, y);
            let expected_pixel = expected.get_pixel(x, y);

            let pixels_match =
                pixels_are_similar(actual_pixel, expected_pixel, per_pixel_threshold);

            if pixels_match {
                diff_image.put_pixel(x, y, Rgba([0, 128, 0, 255]));
            } else {
                diff_pixel_count += 1;
                diff_image.put_pixel(x, y, Rgba([255, 0, 0, 255]));
            }
        }
    }

    let matching_pixels = total_pixels - diff_pixel_count;
    let match_percentage = if total_pixels > 0 {
        matching_pixels as f64 / total_pixels as f64
    } else {
        1.0
    };

    ScreenshotComparison {
        match_percentage,
        diff_image: Some(diff_image),
        diff_pixel_count,
        total_pixels,
    }
}

/// Check if two pixels are similar within a threshold.
fn pixels_are_similar(a: &Rgba<u8>, b: &Rgba<u8>, threshold: u8) -> bool {
    let threshold = threshold as i16;

    let diff_r = (a[0] as i16 - b[0] as i16).abs();
    let diff_g = (a[1] as i16 - b[1] as i16).abs();
    let diff_b = (a[2] as i16 - b[2] as i16).abs();
    let diff_a = (a[3] as i16 - b[3] as i16).abs();

    diff_r <= threshold && diff_g <= threshold && diff_b <= threshold && diff_a <= threshold
}

/// Assert that a screenshot matches a baseline image within tolerance.
///
/// # Arguments
/// * `actual` - The screenshot to test
/// * `baseline_path` - Path to the baseline image file
/// * `tolerance` - Percentage of pixels that can differ (0.0 to 1.0)
/// * `per_pixel_threshold` - Maximum color difference per channel (0-255) to consider pixels equal
///
/// # Returns
/// Ok(()) if the images match, Err with details if they don't.
pub fn assert_screenshot_matches(
    actual: &RgbaImage,
    baseline_path: &Path,
    tolerance: f64,
    per_pixel_threshold: u8,
) -> Result<()> {
    if !baseline_path.exists() {
        return Err(anyhow!(
            "Baseline image not found at: {}. Run with UPDATE_BASELINES=1 to create it.",
            baseline_path.display()
        ));
    }

    let expected = image::open(baseline_path)
        .map_err(|e| anyhow!("Failed to open baseline image: {}", e))?
        .to_rgba8();

    let comparison = compare_screenshots(actual, &expected, per_pixel_threshold);

    if comparison.matches(tolerance) {
        Ok(())
    } else {
        let diff_path = baseline_path.with_extension("diff.png");
        if let Some(diff_image) = &comparison.diff_image {
            diff_image.save(&diff_path).ok();
        }

        let actual_path = baseline_path.with_extension("actual.png");
        actual.save(&actual_path).ok();

        Err(anyhow!(
            "Screenshot does not match baseline.\n\
             Match: {:.2}% (required: {:.2}%)\n\
             Differing pixels: {} / {}\n\
             Baseline: {}\n\
             Actual saved to: {}\n\
             Diff saved to: {}",
            comparison.match_percentage * 100.0,
            (1.0 - tolerance) * 100.0,
            comparison.diff_pixel_count,
            comparison.total_pixels,
            baseline_path.display(),
            actual_path.display(),
            diff_path.display()
        ))
    }
}

/// Save an image as the new baseline, creating parent directories if needed.
pub fn save_baseline(image: &RgbaImage, baseline_path: &Path) -> Result<()> {
    if let Some(parent) = baseline_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| anyhow!("Failed to create baseline directory: {}", e))?;
    }

    image
        .save(baseline_path)
        .map_err(|e| anyhow!("Failed to save baseline image: {}", e))?;

    Ok(())
}

/// Load an image from a file path.
pub fn load_image(path: &Path) -> Result<RgbaImage> {
    image::open(path)
        .map_err(|e| anyhow!("Failed to load image from {}: {}", path.display(), e))
        .map(|img| img.to_rgba8())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_image(width: u32, height: u32, color: Rgba<u8>) -> RgbaImage {
        let mut img = ImageBuffer::new(width, height);
        for pixel in img.pixels_mut() {
            *pixel = color;
        }
        img
    }

    #[test]
    fn test_identical_images_match() {
        let img1 = create_test_image(100, 100, Rgba([255, 0, 0, 255]));
        let img2 = create_test_image(100, 100, Rgba([255, 0, 0, 255]));

        let comparison = compare_screenshots(&img1, &img2, 0);

        assert_eq!(comparison.match_percentage, 1.0);
        assert_eq!(comparison.diff_pixel_count, 0);
        assert!(comparison.matches(0.0));
    }

    #[test]
    fn test_different_images_dont_match() {
        let img1 = create_test_image(100, 100, Rgba([255, 0, 0, 255]));
        let img2 = create_test_image(100, 100, Rgba([0, 255, 0, 255]));

        let comparison = compare_screenshots(&img1, &img2, 0);

        assert_eq!(comparison.match_percentage, 0.0);
        assert_eq!(comparison.diff_pixel_count, 10000);
        assert!(!comparison.matches(0.5));
    }

    #[test]
    fn test_similar_images_match_with_threshold() {
        let img1 = create_test_image(100, 100, Rgba([255, 0, 0, 255]));
        let img2 = create_test_image(100, 100, Rgba([250, 5, 0, 255]));

        let comparison_strict = compare_screenshots(&img1, &img2, 0);
        assert_eq!(comparison_strict.match_percentage, 0.0);

        let comparison_lenient = compare_screenshots(&img1, &img2, 10);
        assert_eq!(comparison_lenient.match_percentage, 1.0);
    }

    #[test]
    fn test_different_size_images() {
        let img1 = create_test_image(100, 100, Rgba([255, 0, 0, 255]));
        let img2 = create_test_image(200, 200, Rgba([255, 0, 0, 255]));

        let comparison = compare_screenshots(&img1, &img2, 0);

        assert_eq!(comparison.match_percentage, 0.0);
        assert!(comparison.diff_image.is_none());
    }

    #[test]
    fn test_partial_difference() {
        let mut img1 = create_test_image(100, 100, Rgba([255, 0, 0, 255]));
        let img2 = create_test_image(100, 100, Rgba([255, 0, 0, 255]));

        for x in 0..50 {
            for y in 0..100 {
                img1.put_pixel(x, y, Rgba([0, 255, 0, 255]));
            }
        }

        let comparison = compare_screenshots(&img1, &img2, 0);

        assert_eq!(comparison.match_percentage, 0.5);
        assert_eq!(comparison.diff_pixel_count, 5000);
        assert!(comparison.matches(0.5));
        assert!(!comparison.matches(0.49));
    }

    #[test]
    #[ignore]
    fn test_visual_test_smoke() {
        let mut cx = VisualTestAppContext::new();

        let _window = cx
            .open_offscreen_window_default(|_, cx| cx.new(|_| Empty))
            .expect("Failed to open offscreen window");

        cx.run_until_parked();
    }

    #[test]
    #[ignore]
    fn test_workspace_opens() {
        let mut cx = VisualTestAppContext::new();
        let app_state = init_visual_test(&mut cx);

        smol::block_on(async {
            app_state
                .fs
                .as_fake()
                .insert_tree(
                    "/project",
                    serde_json::json!({
                        "src": {
                            "main.rs": "fn main() {\n    println!(\"Hello, world!\");\n}\n"
                        }
                    }),
                )
                .await;
        });

        let workspace_result = smol::block_on(open_test_workspace(app_state, &mut cx));
        assert!(
            workspace_result.is_ok(),
            "Failed to open workspace: {:?}",
            workspace_result.err()
        );

        cx.run_until_parked();
    }

    /// This test captures a screenshot of an empty Zed workspace.
    ///
    /// Note: This test is ignored by default because:
    /// 1. It requires macOS with Screen Recording permission granted
    /// 2. It must run on the main thread (standard test threads won't work)
    /// 3. Screenshot capture may fail in CI environments without display access
    ///
    /// The test will gracefully handle screenshot failures and print an error
    /// message rather than failing hard, to allow running in environments
    /// where screen capture isn't available.
    #[test]
    #[ignore]
    fn test_workspace_screenshot() {
        let mut cx = VisualTestAppContext::new();
        let app_state = init_visual_test(&mut cx);

        smol::block_on(async {
            app_state
                .fs
                .as_fake()
                .insert_tree(
                    "/project",
                    serde_json::json!({
                        "src": {
                            "main.rs": "fn main() {\n    println!(\"Hello, world!\");\n}\n"
                        },
                        "README.md": "# Test Project\n\nThis is a test project for visual testing.\n"
                    }),
                )
                .await;
        });

        let workspace = smol::block_on(open_test_workspace(app_state, &mut cx))
            .expect("Failed to open workspace");

        smol::block_on(async {
            wait_for_ui_stabilization(&cx).await;

            let screenshot_result = cx.capture_screenshot(workspace.into());

            match screenshot_result {
                Ok(screenshot) => {
                    println!(
                        "Screenshot captured successfully: {}x{}",
                        screenshot.width(),
                        screenshot.height()
                    );

                    let output_dir = std::env::var("VISUAL_TEST_OUTPUT_DIR")
                        .unwrap_or_else(|_| "target/visual_tests".to_string());
                    let output_path = Path::new(&output_dir).join("workspace_screenshot.png");

                    if let Err(e) = std::fs::create_dir_all(&output_dir) {
                        eprintln!("Warning: Failed to create output directory: {}", e);
                    }

                    if let Err(e) = screenshot.save(&output_path) {
                        eprintln!("Warning: Failed to save screenshot: {}", e);
                    } else {
                        println!("Screenshot saved to: {}", output_path.display());
                    }

                    assert!(
                        screenshot.width() > 0,
                        "Screenshot width should be positive"
                    );
                    assert!(
                        screenshot.height() > 0,
                        "Screenshot height should be positive"
                    );
                }
                Err(e) => {
                    eprintln!(
                        "Screenshot capture failed (this may be expected in CI without screen recording permission): {}",
                        e
                    );
                }
            }
        });

        cx.run_until_parked();
    }
}
