// Allow blocking process commands in this binary - it's a synchronous test runner
#![allow(clippy::disallowed_methods)]

//! Visual Test Runner
//!
//! This binary runs visual regression tests for Zed's UI. It captures screenshots
//! of real Zed windows and compares them against baseline images.
//!
//! **Note: This tool is macOS-only** because it uses `VisualTestAppContext` which
//! depends on the macOS Metal renderer for accurate screenshot capture.
//!
//! ## How It Works
//!
//! This tool uses `VisualTestAppContext` which combines:
//! - Real Metal/compositor rendering for accurate screenshots
//! - Deterministic task scheduling via TestDispatcher
//! - Controllable time via `advance_clock` for testing time-based behaviors
//!
//! This approach:
//! - Does NOT require Screen Recording permission
//! - Does NOT require the window to be visible on screen
//! - Captures raw GPUI output without system window chrome
//! - Is fully deterministic - tooltips, animations, etc. work reliably
//!
//! ## Usage
//!
//! Run the visual tests:
//!   cargo run -p zed --bin zed_visual_test_runner --features visual-tests
//!
//! Update baseline images (when UI intentionally changes):
//!   UPDATE_BASELINE=1 cargo run -p zed --bin zed_visual_test_runner --features visual-tests
//!
//! ## Environment Variables
//!
//!   UPDATE_BASELINE - Set to update baseline images instead of comparing
//!   VISUAL_TEST_OUTPUT_DIR - Directory to save test output (default: target/visual_tests)

// Stub main for non-macOS platforms
#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("Visual test runner is only supported on macOS");
    std::process::exit(1);
}

// All macOS-specific imports grouped together
#[cfg(target_os = "macos")]
use {
    acp_thread::{AgentConnection, StubAgentConnection},
    agent_client_protocol as acp,
    agent_servers::{AgentServer, AgentServerDelegate},
    anyhow::{Context as _, Result},
    assets::Assets,
    editor::display_map::DisplayRow,
    feature_flags::FeatureFlagAppExt as _,
    git_ui::project_diff::ProjectDiff,
    gpui::{
        App, AppContext as _, Bounds, KeyBinding, Modifiers, SharedString, VisualTestAppContext,
        WindowBounds, WindowHandle, WindowOptions, point, px, size,
    },
    image::RgbaImage,
    project_panel::ProjectPanel,
    settings::{NotifyWhenAgentWaiting, Settings as _},
    std::{
        any::Any,
        path::{Path, PathBuf},
        rc::Rc,
        sync::Arc,
        time::Duration,
    },
    watch,
    workspace::{AppState, Workspace},
};

// All macOS-specific constants grouped together
#[cfg(target_os = "macos")]
mod constants {
    use std::time::Duration;

    /// Baseline images are stored relative to this file
    pub const BASELINE_DIR: &str = "crates/zed/test_fixtures/visual_tests";

    /// Embedded test image (Zed app icon) for visual tests.
    pub const EMBEDDED_TEST_IMAGE: &[u8] = include_bytes!("../resources/app-icon.png");

    /// Threshold for image comparison (0.0 to 1.0)
    /// Images must match at least this percentage to pass
    pub const MATCH_THRESHOLD: f64 = 0.99;

    /// Tooltip show delay - must match TOOLTIP_SHOW_DELAY in gpui/src/elements/div.rs
    pub const TOOLTIP_SHOW_DELAY: Duration = Duration::from_millis(500);
}

#[cfg(target_os = "macos")]
use constants::*;

#[cfg(target_os = "macos")]
fn main() {
    // Set ZED_STATELESS early to prevent file system access to real config directories
    // This must be done before any code accesses zed_env_vars::ZED_STATELESS
    // SAFETY: We're at the start of main(), before any threads are spawned
    unsafe {
        std::env::set_var("ZED_STATELESS", "1");
    }

    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let update_baseline = std::env::var("UPDATE_BASELINE").is_ok();

    // Create a temporary directory for test files
    // Canonicalize the path to resolve symlinks (on macOS, /var -> /private/var)
    // which prevents "path does not exist" errors during worktree scanning
    // Use keep() to prevent auto-cleanup - background worktree tasks may still be running
    // when tests complete, so we let the OS clean up temp directories on process exit
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.keep();
    let canonical_temp = temp_path
        .canonicalize()
        .expect("Failed to canonicalize temp directory");
    let project_path = canonical_temp.join("project");
    std::fs::create_dir_all(&project_path).expect("Failed to create project directory");

    // Create test files in the real filesystem
    create_test_files(&project_path);

    let test_result = std::panic::catch_unwind(|| run_visual_tests(project_path, update_baseline));

    // Note: We don't delete temp_path here because background worktree tasks may still
    // be running. The directory will be cleaned up when the process exits or by the OS.

    match test_result {
        Ok(Ok(())) => {}
        Ok(Err(e)) => {
            eprintln!("Visual tests failed: {}", e);
            std::process::exit(1);
        }
        Err(_) => {
            eprintln!("Visual tests panicked");
            std::process::exit(1);
        }
    }
}

#[cfg(target_os = "macos")]
fn run_visual_tests(project_path: PathBuf, update_baseline: bool) -> Result<()> {
    // Create the visual test context with deterministic task scheduling
    // Use real Assets so that SVG icons render properly
    let mut cx = VisualTestAppContext::with_asset_source(Arc::new(Assets));

    // Load embedded fonts (IBM Plex Sans, Lilex, etc.) so UI renders with correct fonts
    cx.update(|cx| {
        Assets.load_fonts(cx).unwrap();
    });

    // Initialize settings store with real default settings (not test settings)
    // Test settings use Courier font, but we want the real Zed fonts for visual tests
    cx.update(|cx| {
        settings::init(cx);
    });

    // Create AppState using the test initialization
    let app_state = cx.update(|cx| init_app_state(cx));

    // Initialize all Zed subsystems
    cx.update(|cx| {
        gpui_tokio::init(cx);
        theme::init(theme::LoadThemes::JustBase, cx);
        client::init(&app_state.client, cx);
        audio::init(cx);
        workspace::init(app_state.clone(), cx);
        release_channel::init(semver::Version::new(0, 0, 0), cx);
        command_palette::init(cx);
        editor::init(cx);
        call::init(app_state.client.clone(), app_state.user_store.clone(), cx);
        title_bar::init(cx);
        project_panel::init(cx);
        outline_panel::init(cx);
        terminal_view::init(cx);
        image_viewer::init(cx);
        search::init(cx);
        prompt_store::init(cx);
        language_model::init(app_state.client.clone(), cx);
        language_models::init(app_state.user_store.clone(), app_state.client.clone(), cx);
        git_ui::init(cx);

        // Load default keymaps so tooltips can show keybindings like "f9" for ToggleBreakpoint
        // We load a minimal set of editor keybindings needed for visual tests
        cx.bind_keys([KeyBinding::new(
            "f9",
            editor::actions::ToggleBreakpoint,
            Some("Editor"),
        )]);

        // Disable agent notifications during visual tests to avoid popup windows
        agent_settings::AgentSettings::override_global(
            agent_settings::AgentSettings {
                notify_when_agent_waiting: NotifyWhenAgentWaiting::Never,
                play_sound_when_agent_done: false,
                ..agent_settings::AgentSettings::get_global(cx).clone()
            },
            cx,
        );
    });

    // Run until all initialization tasks complete
    cx.run_until_parked();

    // Open workspace window
    let window_size = size(px(1280.0), px(800.0));
    let bounds = Bounds {
        origin: point(px(0.0), px(0.0)),
        size: window_size,
    };

    // Create a project for the workspace
    let project = cx.update(|cx| {
        project::Project::local(
            app_state.client.clone(),
            app_state.node_runtime.clone(),
            app_state.user_store.clone(),
            app_state.languages.clone(),
            app_state.fs.clone(),
            None,
            false,
            cx,
        )
    });

    let workspace_window: WindowHandle<Workspace> = cx
        .update(|cx| {
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    focus: false,
                    show: false,
                    ..Default::default()
                },
                |window, cx| {
                    cx.new(|cx| {
                        Workspace::new(None, project.clone(), app_state.clone(), window, cx)
                    })
                },
            )
        })
        .context("Failed to open workspace window")?;

    cx.run_until_parked();

    // Add the test project as a worktree
    let add_worktree_task = workspace_window
        .update(&mut cx, |workspace, _window, cx| {
            let project = workspace.project().clone();
            project.update(cx, |project, cx| {
                project.find_or_create_worktree(&project_path, true, cx)
            })
        })
        .context("Failed to start adding worktree")?;

    // Use block_test to wait for the worktree task
    // block_test runs both foreground and background tasks, which is needed because
    // worktree creation spawns foreground tasks via cx.spawn
    // Allow parking since filesystem operations happen outside the test dispatcher
    cx.background_executor.allow_parking();
    let worktree_result = cx.foreground_executor.block_test(add_worktree_task);
    cx.background_executor.forbid_parking();
    worktree_result.context("Failed to add worktree")?;

    cx.run_until_parked();

    // Create and add the project panel
    let (weak_workspace, async_window_cx) = workspace_window
        .update(&mut cx, |workspace, window, cx| {
            (workspace.weak_handle(), window.to_async(cx))
        })
        .context("Failed to get workspace handle")?;

    cx.background_executor.allow_parking();
    let panel = cx
        .foreground_executor
        .block_test(ProjectPanel::load(weak_workspace, async_window_cx))
        .context("Failed to load project panel")?;
    cx.background_executor.forbid_parking();

    workspace_window
        .update(&mut cx, |workspace, window, cx| {
            workspace.add_panel(panel, window, cx);
        })
        .ok();

    cx.run_until_parked();

    // Open the project panel
    workspace_window
        .update(&mut cx, |workspace, window, cx| {
            workspace.open_panel::<ProjectPanel>(window, cx);
        })
        .ok();

    cx.run_until_parked();

    // Open main.rs in the editor
    let open_file_task = workspace_window
        .update(&mut cx, |workspace, window, cx| {
            let worktree = workspace.project().read(cx).worktrees(cx).next();
            if let Some(worktree) = worktree {
                let worktree_id = worktree.read(cx).id();
                let rel_path: std::sync::Arc<util::rel_path::RelPath> =
                    util::rel_path::rel_path("src/main.rs").into();
                let project_path: project::ProjectPath = (worktree_id, rel_path).into();
                Some(workspace.open_path(project_path, None, true, window, cx))
            } else {
                None
            }
        })
        .ok()
        .flatten();

    if let Some(task) = open_file_task {
        cx.background_executor.allow_parking();
        let block_result = cx.foreground_executor.block_test(task);
        cx.background_executor.forbid_parking();
        if let Ok(item) = block_result {
            workspace_window
                .update(&mut cx, |workspace, window, cx| {
                    let pane = workspace.active_pane().clone();
                    pane.update(cx, |pane, cx| {
                        if let Some(index) = pane.index_for_item(item.as_ref()) {
                            pane.activate_item(index, true, true, window, cx);
                        }
                    });
                })
                .ok();
        }
    }

    cx.run_until_parked();

    // Request a window refresh
    cx.update_window(workspace_window.into(), |_, window, _cx| {
        window.refresh();
    })
    .ok();

    cx.run_until_parked();

    // Track test results
    let mut passed = 0;
    let mut failed = 0;
    let mut updated = 0;

    // Run Test 1: Project Panel (with project panel visible)
    println!("\n--- Test 1: project_panel ---");
    match run_visual_test(
        "project_panel",
        workspace_window.into(),
        &mut cx,
        update_baseline,
    ) {
        Ok(TestResult::Passed) => {
            println!("✓ project_panel: PASSED");
            passed += 1;
        }
        Ok(TestResult::BaselineUpdated(_)) => {
            println!("✓ project_panel: Baseline updated");
            updated += 1;
        }
        Err(e) => {
            eprintln!("✗ project_panel: FAILED - {}", e);
            failed += 1;
        }
    }

    // Run Test 2: Workspace with Editor
    println!("\n--- Test 2: workspace_with_editor ---");

    // Close project panel for this test
    workspace_window
        .update(&mut cx, |workspace, window, cx| {
            workspace.close_panel::<ProjectPanel>(window, cx);
        })
        .ok();

    cx.run_until_parked();

    match run_visual_test(
        "workspace_with_editor",
        workspace_window.into(),
        &mut cx,
        update_baseline,
    ) {
        Ok(TestResult::Passed) => {
            println!("✓ workspace_with_editor: PASSED");
            passed += 1;
        }
        Ok(TestResult::BaselineUpdated(_)) => {
            println!("✓ workspace_with_editor: Baseline updated");
            updated += 1;
        }
        Err(e) => {
            eprintln!("✗ workspace_with_editor: FAILED - {}", e);
            failed += 1;
        }
    }

    // Run Test 3: Agent Thread View tests
    #[cfg(feature = "visual-tests")]
    {
        println!("\n--- Test 3: agent_thread_with_image (collapsed + expanded) ---");
        match run_agent_thread_view_test(app_state.clone(), &mut cx, update_baseline) {
            Ok(TestResult::Passed) => {
                println!("✓ agent_thread_with_image (collapsed + expanded): PASSED");
                passed += 1;
            }
            Ok(TestResult::BaselineUpdated(_)) => {
                println!("✓ agent_thread_with_image: Baselines updated (collapsed + expanded)");
                updated += 1;
            }
            Err(e) => {
                eprintln!("✗ agent_thread_with_image: FAILED - {}", e);
                failed += 1;
            }
        }
    }

    // Run Test 4: Subagent Cards visual tests
    #[cfg(feature = "visual-tests")]
    {
        println!("\n--- Test 4: subagent_cards (running, completed, expanded) ---");
        match run_subagent_visual_tests(app_state.clone(), &mut cx, update_baseline) {
            Ok(TestResult::Passed) => {
                println!("✓ subagent_cards: PASSED");
                passed += 1;
            }
            Ok(TestResult::BaselineUpdated(_)) => {
                println!("✓ subagent_cards: Baselines updated");
                updated += 1;
            }
            Err(e) => {
                eprintln!("✗ subagent_cards: FAILED - {}", e);
                failed += 1;
            }
        }
    }

    // Run Test 5: Breakpoint Hover visual tests
    println!("\n--- Test 5: breakpoint_hover (3 variants) ---");
    match run_breakpoint_hover_visual_tests(app_state.clone(), &mut cx, update_baseline) {
        Ok(TestResult::Passed) => {
            println!("✓ breakpoint_hover: PASSED");
            passed += 1;
        }
        Ok(TestResult::BaselineUpdated(_)) => {
            println!("✓ breakpoint_hover: Baselines updated");
            updated += 1;
        }
        Err(e) => {
            eprintln!("✗ breakpoint_hover: FAILED - {}", e);
            failed += 1;
        }
    }

    // Run Test 6: Diff Review Button visual tests
    println!("\n--- Test 6: diff_review_button (3 variants) ---");
    match run_diff_review_visual_tests(app_state.clone(), &mut cx, update_baseline) {
        Ok(TestResult::Passed) => {
            println!("✓ diff_review_button: PASSED");
            passed += 1;
        }
        Ok(TestResult::BaselineUpdated(_)) => {
            println!("✓ diff_review_button: Baselines updated");
            updated += 1;
        }
        Err(e) => {
            eprintln!("✗ diff_review_button: FAILED - {}", e);
            failed += 1;
        }
    }

    // Clean up the main workspace's worktree to stop background scanning tasks
    // This prevents "root path could not be canonicalized" errors when main() drops temp_dir
    workspace_window
        .update(&mut cx, |workspace, _window, cx| {
            let project = workspace.project().clone();
            project.update(cx, |project, cx| {
                let worktree_ids: Vec<_> =
                    project.worktrees(cx).map(|wt| wt.read(cx).id()).collect();
                for id in worktree_ids {
                    project.remove_worktree(id, cx);
                }
            });
        })
        .ok();

    cx.run_until_parked();

    // Close the main window
    let _ = cx.update_window(workspace_window.into(), |_, window, _cx| {
        window.remove_window();
    });

    // Run until all cleanup tasks complete
    cx.run_until_parked();

    // Give background tasks time to finish, including scrollbar hide timers (1 second)
    for _ in 0..15 {
        cx.advance_clock(Duration::from_millis(100));
        cx.run_until_parked();
    }

    // Print summary
    println!("\n=== Test Summary ===");
    println!("Passed: {}", passed);
    println!("Failed: {}", failed);
    if updated > 0 {
        println!("Baselines Updated: {}", updated);
    }

    if failed > 0 {
        eprintln!("\n=== Visual Tests FAILED ===");
        Err(anyhow::anyhow!("{} tests failed", failed))
    } else {
        println!("\n=== All Visual Tests PASSED ===");
        Ok(())
    }
}

#[cfg(target_os = "macos")]
enum TestResult {
    Passed,
    BaselineUpdated(PathBuf),
}

#[cfg(target_os = "macos")]
fn run_visual_test(
    test_name: &str,
    window: gpui::AnyWindowHandle,
    cx: &mut VisualTestAppContext,
    update_baseline: bool,
) -> Result<TestResult> {
    // Ensure all pending work is done
    cx.run_until_parked();

    // Refresh the window to ensure it's fully rendered
    cx.update_window(window, |_, window, _cx| {
        window.refresh();
    })?;

    cx.run_until_parked();

    // Capture the screenshot using direct texture capture
    let screenshot = cx.capture_screenshot(window)?;

    // Get paths
    let baseline_path = get_baseline_path(test_name);
    let output_dir = std::env::var("VISUAL_TEST_OUTPUT_DIR")
        .unwrap_or_else(|_| "target/visual_tests".to_string());
    let output_path = PathBuf::from(&output_dir).join(format!("{}.png", test_name));

    // Ensure output directory exists
    std::fs::create_dir_all(&output_dir)?;

    // Always save the current screenshot
    screenshot.save(&output_path)?;
    println!("  Screenshot saved to: {}", output_path.display());

    if update_baseline {
        // Update the baseline
        if let Some(parent) = baseline_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        screenshot.save(&baseline_path)?;
        println!("  Baseline updated: {}", baseline_path.display());
        return Ok(TestResult::BaselineUpdated(baseline_path));
    }

    // Compare with baseline
    if !baseline_path.exists() {
        return Err(anyhow::anyhow!(
            "Baseline not found: {}. Run with UPDATE_BASELINE=1 to create it.",
            baseline_path.display()
        ));
    }

    let baseline = image::open(&baseline_path)?.to_rgba8();
    let comparison = compare_images(&screenshot, &baseline);

    println!(
        "  Match: {:.2}% ({} different pixels)",
        comparison.match_percentage * 100.0,
        comparison.diff_pixel_count
    );

    if comparison.match_percentage >= MATCH_THRESHOLD {
        Ok(TestResult::Passed)
    } else {
        // Save diff image
        let diff_path = PathBuf::from(&output_dir).join(format!("{}_diff.png", test_name));
        comparison.diff_image.save(&diff_path)?;
        println!("  Diff image saved to: {}", diff_path.display());

        Err(anyhow::anyhow!(
            "Image mismatch: {:.2}% match (threshold: {:.2}%)",
            comparison.match_percentage * 100.0,
            MATCH_THRESHOLD * 100.0
        ))
    }
}

#[cfg(target_os = "macos")]
fn get_baseline_path(test_name: &str) -> PathBuf {
    // Get the workspace root (where Cargo.toml is)
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let workspace_root = PathBuf::from(manifest_dir)
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));

    workspace_root
        .join(BASELINE_DIR)
        .join(format!("{}.png", test_name))
}

#[cfg(target_os = "macos")]
struct ImageComparison {
    match_percentage: f64,
    diff_image: RgbaImage,
    diff_pixel_count: u32,
    #[allow(dead_code)]
    total_pixels: u32,
}

#[cfg(target_os = "macos")]
fn compare_images(actual: &RgbaImage, expected: &RgbaImage) -> ImageComparison {
    let width = actual.width().max(expected.width());
    let height = actual.height().max(expected.height());
    let total_pixels = width * height;

    let mut diff_image = RgbaImage::new(width, height);
    let mut matching_pixels = 0u32;

    for y in 0..height {
        for x in 0..width {
            let actual_pixel = if x < actual.width() && y < actual.height() {
                *actual.get_pixel(x, y)
            } else {
                image::Rgba([0, 0, 0, 0])
            };

            let expected_pixel = if x < expected.width() && y < expected.height() {
                *expected.get_pixel(x, y)
            } else {
                image::Rgba([0, 0, 0, 0])
            };

            if pixels_are_similar(&actual_pixel, &expected_pixel) {
                matching_pixels += 1;
                // Semi-transparent green for matching pixels
                diff_image.put_pixel(x, y, image::Rgba([0, 255, 0, 64]));
            } else {
                // Bright red for differing pixels
                diff_image.put_pixel(x, y, image::Rgba([255, 0, 0, 255]));
            }
        }
    }

    let match_percentage = matching_pixels as f64 / total_pixels as f64;
    let diff_pixel_count = total_pixels - matching_pixels;

    ImageComparison {
        match_percentage,
        diff_image,
        diff_pixel_count,
        total_pixels,
    }
}

#[cfg(target_os = "macos")]
fn pixels_are_similar(a: &image::Rgba<u8>, b: &image::Rgba<u8>) -> bool {
    const TOLERANCE: i16 = 2;
    (a.0[0] as i16 - b.0[0] as i16).abs() <= TOLERANCE
        && (a.0[1] as i16 - b.0[1] as i16).abs() <= TOLERANCE
        && (a.0[2] as i16 - b.0[2] as i16).abs() <= TOLERANCE
        && (a.0[3] as i16 - b.0[3] as i16).abs() <= TOLERANCE
}

#[cfg(target_os = "macos")]
fn create_test_files(project_path: &Path) {
    // Create src directory
    let src_dir = project_path.join("src");
    std::fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    // Create main.rs
    let main_rs = r#"fn main() {
    println!("Hello, world!");

    let x = 42;
    let y = x * 2;

    if y > 50 {
        println!("y is greater than 50");
    } else {
        println!("y is not greater than 50");
    }

    for i in 0..10 {
        println!("i = {}", i);
    }
}

fn helper_function(a: i32, b: i32) -> i32 {
    a + b
}

struct MyStruct {
    field1: String,
    field2: i32,
}

impl MyStruct {
    fn new(name: &str, value: i32) -> Self {
        Self {
            field1: name.to_string(),
            field2: value,
        }
    }

    fn get_value(&self) -> i32 {
        self.field2
    }
}
"#;
    std::fs::write(src_dir.join("main.rs"), main_rs).expect("Failed to write main.rs");

    // Create lib.rs
    let lib_rs = r#"//! A sample library for visual testing

pub mod utils;

/// A public function in the library
pub fn library_function() -> String {
    "Hello from lib".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(library_function(), "Hello from lib");
    }
}
"#;
    std::fs::write(src_dir.join("lib.rs"), lib_rs).expect("Failed to write lib.rs");

    // Create utils.rs
    let utils_rs = r#"//! Utility functions

/// Format a number with commas
pub fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

/// Calculate fibonacci number
pub fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
"#;
    std::fs::write(src_dir.join("utils.rs"), utils_rs).expect("Failed to write utils.rs");

    // Create Cargo.toml
    let cargo_toml = r#"[package]
name = "test_project"
version = "0.1.0"
edition = "2021"

[dependencies]
"#;
    std::fs::write(project_path.join("Cargo.toml"), cargo_toml)
        .expect("Failed to write Cargo.toml");

    // Create README.md
    let readme = r#"# Test Project

This is a test project for visual testing of Zed.

## Features

- Feature 1
- Feature 2
- Feature 3

## Usage

```bash
cargo run
```
"#;
    std::fs::write(project_path.join("README.md"), readme).expect("Failed to write README.md");
}

#[cfg(target_os = "macos")]
fn init_app_state(cx: &mut App) -> Arc<AppState> {
    use fs::Fs;
    use node_runtime::NodeRuntime;
    use session::Session;
    use settings::SettingsStore;

    if !cx.has_global::<SettingsStore>() {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
    }

    // Use the real filesystem instead of FakeFs so we can access actual files on disk
    let fs: Arc<dyn Fs> = Arc::new(fs::RealFs::new(None, cx.background_executor().clone()));
    <dyn Fs>::set_global(fs.clone(), cx);

    let languages = Arc::new(language::LanguageRegistry::test(
        cx.background_executor().clone(),
    ));
    let clock = Arc::new(clock::FakeSystemClock::new());
    let http_client = http_client::FakeHttpClient::with_404_response();
    let client = client::Client::new(clock, http_client, cx);
    let session = cx.new(|cx| session::AppSession::new(Session::test(), cx));
    let user_store = cx.new(|cx| client::UserStore::new(client.clone(), cx));
    let workspace_store = cx.new(|cx| workspace::WorkspaceStore::new(client.clone(), cx));

    theme::init(theme::LoadThemes::JustBase, cx);
    client::init(&client, cx);

    Arc::new(AppState {
        client,
        fs,
        languages,
        user_store,
        workspace_store,
        node_runtime: NodeRuntime::unavailable(),
        build_window_options: |_, _| Default::default(),
        session,
    })
}

/// Runs visual tests for breakpoint hover states in the editor gutter.
///
/// This test captures three states:
/// 1. Gutter with line numbers, no breakpoint hover (baseline)
/// 2. Gutter with breakpoint hover indicator (gray circle)
/// 3. Gutter with breakpoint hover AND tooltip
#[cfg(target_os = "macos")]
fn run_breakpoint_hover_visual_tests(
    app_state: Arc<AppState>,
    cx: &mut VisualTestAppContext,
    update_baseline: bool,
) -> Result<TestResult> {
    // Create a temporary directory with a simple test file
    let temp_dir = tempfile::tempdir()?;
    let temp_path = temp_dir.keep();
    let canonical_temp = temp_path.canonicalize()?;
    let project_path = canonical_temp.join("project");
    std::fs::create_dir_all(&project_path)?;

    // Create a simple file with a few lines
    let src_dir = project_path.join("src");
    std::fs::create_dir_all(&src_dir)?;

    let test_content = r#"fn main() {
    println!("Hello");
    let x = 42;
}
"#;
    std::fs::write(src_dir.join("test.rs"), test_content)?;

    // Create a small window - just big enough to show gutter and a few lines
    let window_size = size(px(300.0), px(200.0));
    let bounds = Bounds {
        origin: point(px(0.0), px(0.0)),
        size: window_size,
    };

    // Create project
    let project = cx.update(|cx| {
        project::Project::local(
            app_state.client.clone(),
            app_state.node_runtime.clone(),
            app_state.user_store.clone(),
            app_state.languages.clone(),
            app_state.fs.clone(),
            None,
            false,
            cx,
        )
    });

    // Open workspace window
    let workspace_window: WindowHandle<Workspace> = cx
        .update(|cx| {
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    focus: false,
                    show: false,
                    ..Default::default()
                },
                |window, cx| {
                    cx.new(|cx| {
                        Workspace::new(None, project.clone(), app_state.clone(), window, cx)
                    })
                },
            )
        })
        .context("Failed to open breakpoint test window")?;

    cx.run_until_parked();

    // Add the project as a worktree
    let add_worktree_task = workspace_window
        .update(cx, |workspace, _window, cx| {
            let project = workspace.project().clone();
            project.update(cx, |project, cx| {
                project.find_or_create_worktree(&project_path, true, cx)
            })
        })
        .context("Failed to start adding worktree")?;

    cx.background_executor.allow_parking();
    let worktree_result = cx.foreground_executor.block_test(add_worktree_task);
    cx.background_executor.forbid_parking();
    worktree_result.context("Failed to add worktree")?;

    cx.run_until_parked();

    // Open the test file
    let open_file_task = workspace_window
        .update(cx, |workspace, window, cx| {
            let worktree = workspace.project().read(cx).worktrees(cx).next();
            if let Some(worktree) = worktree {
                let worktree_id = worktree.read(cx).id();
                let rel_path: std::sync::Arc<util::rel_path::RelPath> =
                    util::rel_path::rel_path("src/test.rs").into();
                let project_path: project::ProjectPath = (worktree_id, rel_path).into();
                Some(workspace.open_path(project_path, None, true, window, cx))
            } else {
                None
            }
        })
        .ok()
        .flatten();

    if let Some(task) = open_file_task {
        cx.background_executor.allow_parking();
        let _ = cx.foreground_executor.block_test(task);
        cx.background_executor.forbid_parking();
    }

    cx.run_until_parked();

    // Wait for the editor to fully load
    for _ in 0..10 {
        cx.advance_clock(Duration::from_millis(100));
        cx.run_until_parked();
    }

    // Refresh window
    cx.update_window(workspace_window.into(), |_, window, _cx| {
        window.refresh();
    })?;

    cx.run_until_parked();

    // Test 1: Gutter visible with line numbers, no breakpoint hover
    let test1_result = run_visual_test(
        "breakpoint_hover_none",
        workspace_window.into(),
        cx,
        update_baseline,
    )?;

    // Test 2: Breakpoint hover indicator (circle) visible
    // The gutter is on the left side. We need to position the mouse over the gutter area
    // for line 1. The breakpoint indicator appears in the leftmost part of the gutter.
    //
    // The breakpoint hover requires multiple steps:
    // 1. Draw to register mouse listeners
    // 2. Mouse move to trigger gutter_hovered and create PhantomBreakpointIndicator
    // 3. Wait 200ms for is_active to become true
    // 4. Draw again to render the indicator
    //
    // The gutter_position should be in the gutter area to trigger the phantom breakpoint.
    // The button_position should be directly over the breakpoint icon button for tooltip hover.
    // Based on debug output: button is at origin=(3.12, 66.5) with size=(14, 16)
    let gutter_position = point(px(30.0), px(85.0));
    let button_position = point(px(10.0), px(75.0)); // Center of the breakpoint button

    // Step 1: Initial draw to register mouse listeners
    cx.update_window(workspace_window.into(), |_, window, cx| {
        window.draw(cx).clear();
    })?;
    cx.run_until_parked();

    // Step 2: Simulate mouse move into gutter area
    cx.simulate_mouse_move(
        workspace_window.into(),
        gutter_position,
        None,
        Modifiers::default(),
    );

    // Step 3: Advance clock past 200ms debounce
    cx.advance_clock(Duration::from_millis(300));
    cx.run_until_parked();

    // Step 4: Draw again to pick up the indicator state change
    cx.update_window(workspace_window.into(), |_, window, cx| {
        window.draw(cx).clear();
    })?;
    cx.run_until_parked();

    // Step 5: Another mouse move to keep hover state active
    cx.simulate_mouse_move(
        workspace_window.into(),
        gutter_position,
        None,
        Modifiers::default(),
    );

    // Step 6: Final draw
    cx.update_window(workspace_window.into(), |_, window, cx| {
        window.draw(cx).clear();
    })?;
    cx.run_until_parked();

    let test2_result = run_visual_test(
        "breakpoint_hover_circle",
        workspace_window.into(),
        cx,
        update_baseline,
    )?;

    // Test 3: Breakpoint hover with tooltip visible
    // The tooltip delay is 500ms (TOOLTIP_SHOW_DELAY constant)
    // We need to position the mouse directly over the breakpoint button for the tooltip to show.
    // The button hitbox is approximately at (3.12, 66.5) with size (14, 16).

    // Move mouse directly over the button to trigger tooltip hover
    cx.simulate_mouse_move(
        workspace_window.into(),
        button_position,
        None,
        Modifiers::default(),
    );

    // Draw to register the button's tooltip hover listener
    cx.update_window(workspace_window.into(), |_, window, cx| {
        window.draw(cx).clear();
    })?;
    cx.run_until_parked();

    // Move mouse over button again to trigger tooltip scheduling
    cx.simulate_mouse_move(
        workspace_window.into(),
        button_position,
        None,
        Modifiers::default(),
    );

    // Advance clock past TOOLTIP_SHOW_DELAY (500ms)
    cx.advance_clock(TOOLTIP_SHOW_DELAY + Duration::from_millis(100));
    cx.run_until_parked();

    // Draw to render the tooltip
    cx.update_window(workspace_window.into(), |_, window, cx| {
        window.draw(cx).clear();
    })?;
    cx.run_until_parked();

    // Refresh window
    cx.update_window(workspace_window.into(), |_, window, _cx| {
        window.refresh();
    })?;

    cx.run_until_parked();

    let test3_result = run_visual_test(
        "breakpoint_hover_tooltip",
        workspace_window.into(),
        cx,
        update_baseline,
    )?;

    // Clean up: remove worktrees to stop background scanning
    workspace_window
        .update(cx, |workspace, _window, cx| {
            let project = workspace.project().clone();
            project.update(cx, |project, cx| {
                let worktree_ids: Vec<_> =
                    project.worktrees(cx).map(|wt| wt.read(cx).id()).collect();
                for id in worktree_ids {
                    project.remove_worktree(id, cx);
                }
            });
        })
        .ok();

    cx.run_until_parked();

    // Close the window
    let _ = cx.update_window(workspace_window.into(), |_, window, _cx| {
        window.remove_window();
    });

    cx.run_until_parked();

    // Give background tasks time to finish
    for _ in 0..15 {
        cx.advance_clock(Duration::from_millis(100));
        cx.run_until_parked();
    }

    // Return combined result
    match (&test1_result, &test2_result, &test3_result) {
        (TestResult::Passed, TestResult::Passed, TestResult::Passed) => Ok(TestResult::Passed),
        (TestResult::BaselineUpdated(p), _, _)
        | (_, TestResult::BaselineUpdated(p), _)
        | (_, _, TestResult::BaselineUpdated(p)) => Ok(TestResult::BaselineUpdated(p.clone())),
    }
}

/// Runs visual tests for the diff review button in git diff views.
///
/// This test captures three states:
/// 1. Diff view with feature flag enabled (button visible)
/// 2. Diff view with feature flag disabled (no button)
/// 3. Regular editor with feature flag enabled (no button - only shows in diff views)
#[cfg(target_os = "macos")]
fn run_diff_review_visual_tests(
    app_state: Arc<AppState>,
    cx: &mut VisualTestAppContext,
    update_baseline: bool,
) -> Result<TestResult> {
    // Create a temporary directory with test files and a real git repo
    let temp_dir = tempfile::tempdir()?;
    let temp_path = temp_dir.keep();
    let canonical_temp = temp_path.canonicalize()?;
    let project_path = canonical_temp.join("project");
    std::fs::create_dir_all(&project_path)?;

    // Initialize a real git repository
    std::process::Command::new("git")
        .args(["init"])
        .current_dir(&project_path)
        .output()?;

    // Configure git user for commits
    std::process::Command::new("git")
        .args(["config", "user.email", "test@test.com"])
        .current_dir(&project_path)
        .output()?;
    std::process::Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(&project_path)
        .output()?;

    // Create a test file with original content
    let original_content = "// Original content\n";
    std::fs::write(project_path.join("thread-view.tsx"), original_content)?;

    // Commit the original file
    std::process::Command::new("git")
        .args(["add", "thread-view.tsx"])
        .current_dir(&project_path)
        .output()?;
    std::process::Command::new("git")
        .args(["commit", "-m", "Initial commit"])
        .current_dir(&project_path)
        .output()?;

    // Modify the file to create a diff
    let modified_content = r#"import { ScrollArea } from 'components';
import { ButtonAlt, Tooltip } from 'ui';
import { Message, FileEdit } from 'types';
import { AiPaneTabContext } from 'context';
"#;
    std::fs::write(project_path.join("thread-view.tsx"), modified_content)?;

    // Create window for the diff view - sized to show just the editor
    let window_size = size(px(600.0), px(400.0));
    let bounds = Bounds {
        origin: point(px(0.0), px(0.0)),
        size: window_size,
    };

    // Create project
    let project = cx.update(|cx| {
        project::Project::local(
            app_state.client.clone(),
            app_state.node_runtime.clone(),
            app_state.user_store.clone(),
            app_state.languages.clone(),
            app_state.fs.clone(),
            None,
            false,
            cx,
        )
    });

    // Add the test directory as a worktree
    let add_worktree_task = project.update(cx, |project, cx| {
        project.find_or_create_worktree(&project_path, true, cx)
    });

    cx.background_executor.allow_parking();
    let _ = cx.foreground_executor.block_test(add_worktree_task);
    cx.background_executor.forbid_parking();

    cx.run_until_parked();

    // Wait for worktree to be fully scanned and git status to be detected
    for _ in 0..5 {
        cx.advance_clock(Duration::from_millis(100));
        cx.run_until_parked();
    }

    // Test 1: Diff view with feature flag enabled
    // Enable the feature flag
    cx.update(|cx| {
        cx.update_flags(true, vec!["diff-review".to_string()]);
    });

    let workspace_window: WindowHandle<Workspace> = cx
        .update(|cx| {
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    focus: false,
                    show: false,
                    ..Default::default()
                },
                |window, cx| {
                    cx.new(|cx| {
                        Workspace::new(None, project.clone(), app_state.clone(), window, cx)
                    })
                },
            )
        })
        .context("Failed to open diff review test window")?;

    cx.run_until_parked();

    // Create and add the ProjectDiff using the public deploy_at method
    workspace_window
        .update(cx, |workspace, window, cx| {
            ProjectDiff::deploy_at(workspace, None, window, cx);
        })
        .ok();

    // Wait for diff to render
    for _ in 0..5 {
        cx.advance_clock(Duration::from_millis(100));
        cx.run_until_parked();
    }

    // Refresh window
    cx.update_window(workspace_window.into(), |_, window, _cx| {
        window.refresh();
    })?;

    cx.run_until_parked();

    // Capture Test 1: Diff with flag enabled
    let test1_result = run_visual_test(
        "diff_review_button_enabled",
        workspace_window.into(),
        cx,
        update_baseline,
    )?;

    // Test 2: Diff view with feature flag disabled
    // Disable the feature flag
    cx.update(|cx| {
        cx.update_flags(false, vec![]);
    });

    // Refresh window
    cx.update_window(workspace_window.into(), |_, window, _cx| {
        window.refresh();
    })?;

    for _ in 0..3 {
        cx.advance_clock(Duration::from_millis(100));
        cx.run_until_parked();
    }

    // Capture Test 2: Diff with flag disabled
    let test2_result = run_visual_test(
        "diff_review_button_disabled",
        workspace_window.into(),
        cx,
        update_baseline,
    )?;

    // Test 3: Regular editor with flag enabled (should NOT show button)
    // Re-enable the feature flag
    cx.update(|cx| {
        cx.update_flags(true, vec!["diff-review".to_string()]);
    });

    // Create a new window with just a regular editor
    let regular_window: WindowHandle<Workspace> = cx
        .update(|cx| {
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    focus: false,
                    show: false,
                    ..Default::default()
                },
                |window, cx| {
                    cx.new(|cx| {
                        Workspace::new(None, project.clone(), app_state.clone(), window, cx)
                    })
                },
            )
        })
        .context("Failed to open regular editor window")?;

    cx.run_until_parked();

    // Open a regular file (not a diff view)
    let open_file_task = regular_window
        .update(cx, |workspace, window, cx| {
            let worktree = workspace.project().read(cx).worktrees(cx).next();
            if let Some(worktree) = worktree {
                let worktree_id = worktree.read(cx).id();
                let rel_path: std::sync::Arc<util::rel_path::RelPath> =
                    util::rel_path::rel_path("thread-view.tsx").into();
                let project_path: project::ProjectPath = (worktree_id, rel_path).into();
                Some(workspace.open_path(project_path, None, true, window, cx))
            } else {
                None
            }
        })
        .ok()
        .flatten();

    if let Some(task) = open_file_task {
        cx.background_executor.allow_parking();
        let _ = cx.foreground_executor.block_test(task);
        cx.background_executor.forbid_parking();
    }

    // Wait for file to open
    for _ in 0..3 {
        cx.advance_clock(Duration::from_millis(100));
        cx.run_until_parked();
    }

    // Refresh window
    cx.update_window(regular_window.into(), |_, window, _cx| {
        window.refresh();
    })?;

    cx.run_until_parked();

    // Capture Test 3: Regular editor with flag enabled (no button)
    let test3_result = run_visual_test(
        "diff_review_button_regular_editor",
        regular_window.into(),
        cx,
        update_baseline,
    )?;

    // Test 4: Show the diff review overlay on the regular editor
    regular_window
        .update(cx, |workspace, window, cx| {
            // Get the first editor from the workspace
            let editors: Vec<_> = workspace.items_of_type::<editor::Editor>(cx).collect();
            if let Some(editor) = editors.into_iter().next() {
                editor.update(cx, |editor, cx| {
                    editor.show_diff_review_overlay(DisplayRow(1), window, cx);
                });
            }
        })
        .ok();

    // Wait for overlay to render
    for _ in 0..3 {
        cx.advance_clock(Duration::from_millis(100));
        cx.run_until_parked();
    }

    // Refresh window
    cx.update_window(regular_window.into(), |_, window, _cx| {
        window.refresh();
    })?;

    cx.run_until_parked();

    // Capture Test 4: Regular editor with overlay shown
    let test4_result = run_visual_test(
        "diff_review_overlay_shown",
        regular_window.into(),
        cx,
        update_baseline,
    )?;

    // Test 5: Type text into the diff review prompt and submit it
    // First, get the prompt editor from the overlay and type some text
    regular_window
        .update(cx, |workspace, window, cx| {
            let editors: Vec<_> = workspace.items_of_type::<editor::Editor>(cx).collect();
            if let Some(editor) = editors.into_iter().next() {
                editor.update(cx, |editor, cx| {
                    // Get the prompt editor from the overlay and insert text
                    if let Some(prompt_editor) = editor.diff_review_prompt_editor().cloned() {
                        prompt_editor.update(cx, |prompt_editor: &mut editor::Editor, cx| {
                            prompt_editor.insert(
                                "This change needs better error handling",
                                window,
                                cx,
                            );
                        });
                    }
                });
            }
        })
        .ok();

    // Wait for text to be inserted
    for _ in 0..3 {
        cx.advance_clock(Duration::from_millis(100));
        cx.run_until_parked();
    }

    // Refresh window
    cx.update_window(regular_window.into(), |_, window, _cx| {
        window.refresh();
    })?;

    cx.run_until_parked();

    // Capture Test 5: Diff review overlay with typed text
    let test5_result = run_visual_test(
        "diff_review_overlay_with_text",
        regular_window.into(),
        cx,
        update_baseline,
    )?;

    // Test 6: Submit a comment to store it locally
    regular_window
        .update(cx, |workspace, window, cx| {
            let editors: Vec<_> = workspace.items_of_type::<editor::Editor>(cx).collect();
            if let Some(editor) = editors.into_iter().next() {
                editor.update(cx, |editor, cx| {
                    // Submit the comment that was typed in test 5
                    editor.submit_diff_review_comment(window, cx);
                });
            }
        })
        .ok();

    // Wait for comment to be stored
    for _ in 0..3 {
        cx.advance_clock(Duration::from_millis(100));
        cx.run_until_parked();
    }

    // Refresh window
    cx.update_window(regular_window.into(), |_, window, _cx| {
        window.refresh();
    })?;

    cx.run_until_parked();

    // Capture Test 6: Overlay with one stored comment
    let test6_result = run_visual_test(
        "diff_review_one_comment",
        regular_window.into(),
        cx,
        update_baseline,
    )?;

    // Test 7: Add more comments to show multiple comments expanded
    regular_window
        .update(cx, |workspace, window, cx| {
            let editors: Vec<_> = workspace.items_of_type::<editor::Editor>(cx).collect();
            if let Some(editor) = editors.into_iter().next() {
                editor.update(cx, |editor, cx| {
                    // Add second comment
                    if let Some(prompt_editor) = editor.diff_review_prompt_editor().cloned() {
                        prompt_editor.update(cx, |pe, cx| {
                            pe.insert("Second comment about imports", window, cx);
                        });
                    }
                    editor.submit_diff_review_comment(window, cx);

                    // Add third comment
                    if let Some(prompt_editor) = editor.diff_review_prompt_editor().cloned() {
                        prompt_editor.update(cx, |pe, cx| {
                            pe.insert("Third comment about naming conventions", window, cx);
                        });
                    }
                    editor.submit_diff_review_comment(window, cx);
                });
            }
        })
        .ok();

    // Wait for comments to be stored
    for _ in 0..3 {
        cx.advance_clock(Duration::from_millis(100));
        cx.run_until_parked();
    }

    // Refresh window
    cx.update_window(regular_window.into(), |_, window, _cx| {
        window.refresh();
    })?;

    cx.run_until_parked();

    // Capture Test 7: Overlay with multiple comments expanded
    let test7_result = run_visual_test(
        "diff_review_multiple_comments_expanded",
        regular_window.into(),
        cx,
        update_baseline,
    )?;

    // Test 8: Collapse the comments section
    regular_window
        .update(cx, |workspace, _window, cx| {
            let editors: Vec<_> = workspace.items_of_type::<editor::Editor>(cx).collect();
            if let Some(editor) = editors.into_iter().next() {
                editor.update(cx, |editor, cx| {
                    // Toggle collapse using the public method
                    editor.set_diff_review_comments_expanded(false, cx);
                });
            }
        })
        .ok();

    // Wait for UI to update
    for _ in 0..3 {
        cx.advance_clock(Duration::from_millis(100));
        cx.run_until_parked();
    }

    // Refresh window
    cx.update_window(regular_window.into(), |_, window, _cx| {
        window.refresh();
    })?;

    cx.run_until_parked();

    // Capture Test 8: Comments collapsed
    let test8_result = run_visual_test(
        "diff_review_comments_collapsed",
        regular_window.into(),
        cx,
        update_baseline,
    )?;

    // Clean up: remove worktrees to stop background scanning
    workspace_window
        .update(cx, |workspace, _window, cx| {
            let project = workspace.project().clone();
            project.update(cx, |project, cx| {
                let worktree_ids: Vec<_> =
                    project.worktrees(cx).map(|wt| wt.read(cx).id()).collect();
                for id in worktree_ids {
                    project.remove_worktree(id, cx);
                }
            });
        })
        .ok();

    cx.run_until_parked();

    // Close windows
    let _ = cx.update_window(workspace_window.into(), |_, window, _cx| {
        window.remove_window();
    });
    let _ = cx.update_window(regular_window.into(), |_, window, _cx| {
        window.remove_window();
    });

    cx.run_until_parked();

    // Give background tasks time to finish
    for _ in 0..15 {
        cx.advance_clock(Duration::from_millis(100));
        cx.run_until_parked();
    }

    // Return combined result
    let all_results = [
        &test1_result,
        &test2_result,
        &test3_result,
        &test4_result,
        &test5_result,
        &test6_result,
        &test7_result,
        &test8_result,
    ];

    // Combine results: if any test updated a baseline, return BaselineUpdated;
    // otherwise return Passed. The exhaustive match ensures the compiler
    // verifies we handle all TestResult variants.
    let result = all_results
        .iter()
        .fold(TestResult::Passed, |acc, r| match r {
            TestResult::Passed => acc,
            TestResult::BaselineUpdated(p) => TestResult::BaselineUpdated(p.clone()),
        });
    Ok(result)
}

/// A stub AgentServer for visual testing that returns a pre-programmed connection.
#[derive(Clone)]
#[cfg(target_os = "macos")]
struct StubAgentServer {
    connection: StubAgentConnection,
}

#[cfg(target_os = "macos")]
impl StubAgentServer {
    fn new(connection: StubAgentConnection) -> Self {
        Self { connection }
    }
}

#[cfg(target_os = "macos")]
impl AgentServer for StubAgentServer {
    fn logo(&self) -> ui::IconName {
        ui::IconName::ZedAssistant
    }

    fn name(&self) -> SharedString {
        "Visual Test Agent".into()
    }

    fn connect(
        &self,
        _root_dir: Option<&Path>,
        _delegate: AgentServerDelegate,
        _cx: &mut App,
    ) -> gpui::Task<gpui::Result<(Rc<dyn AgentConnection>, Option<task::SpawnInTerminal>)>> {
        gpui::Task::ready(Ok((Rc::new(self.connection.clone()), None)))
    }

    fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}

#[cfg(all(target_os = "macos", feature = "visual-tests"))]
fn run_subagent_visual_tests(
    app_state: Arc<AppState>,
    cx: &mut VisualTestAppContext,
    update_baseline: bool,
) -> Result<TestResult> {
    use acp_thread::{
        AcpThread, SUBAGENT_TOOL_NAME, ToolCallUpdateSubagentThread, meta_with_tool_name,
    };
    use agent_ui::AgentPanel;

    // Create a temporary project directory
    let temp_dir = tempfile::tempdir()?;
    let temp_path = temp_dir.keep();
    let canonical_temp = temp_path.canonicalize()?;
    let project_path = canonical_temp.join("project");
    std::fs::create_dir_all(&project_path)?;

    // Create a project
    let project = cx.update(|cx| {
        project::Project::local(
            app_state.client.clone(),
            app_state.node_runtime.clone(),
            app_state.user_store.clone(),
            app_state.languages.clone(),
            app_state.fs.clone(),
            None,
            false,
            cx,
        )
    });

    // Add the test directory as a worktree
    let add_worktree_task = project.update(cx, |project, cx| {
        project.find_or_create_worktree(&project_path, true, cx)
    });

    let _ = cx.foreground_executor.block_test(add_worktree_task);

    cx.run_until_parked();

    // Create stub connection - we'll manually inject the subagent content
    let connection = StubAgentConnection::new();

    // Create a subagent tool call (in progress state)
    let tool_call = acp::ToolCall::new("subagent-tool-1", "2 subagents")
        .kind(acp::ToolKind::Other)
        .meta(meta_with_tool_name(SUBAGENT_TOOL_NAME))
        .status(acp::ToolCallStatus::InProgress);

    connection.set_next_prompt_updates(vec![acp::SessionUpdate::ToolCall(tool_call)]);

    let stub_agent: Rc<dyn AgentServer> = Rc::new(StubAgentServer::new(connection.clone()));

    // Create a window sized for the agent panel
    let window_size = size(px(600.0), px(700.0));
    let bounds = Bounds {
        origin: point(px(0.0), px(0.0)),
        size: window_size,
    };

    let workspace_window: WindowHandle<Workspace> = cx
        .update(|cx| {
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    focus: false,
                    show: false,
                    ..Default::default()
                },
                |window, cx| {
                    cx.new(|cx| {
                        Workspace::new(None, project.clone(), app_state.clone(), window, cx)
                    })
                },
            )
        })
        .context("Failed to open agent window")?;

    cx.run_until_parked();

    // Load the AgentPanel
    let (weak_workspace, async_window_cx) = workspace_window
        .update(cx, |workspace, window, cx| {
            (workspace.weak_handle(), window.to_async(cx))
        })
        .context("Failed to get workspace handle")?;

    let prompt_builder =
        cx.update(|cx| prompt_store::PromptBuilder::load(app_state.fs.clone(), false, cx));
    let panel = cx
        .foreground_executor
        .block_test(AgentPanel::load(
            weak_workspace,
            prompt_builder,
            async_window_cx,
        ))
        .context("Failed to load AgentPanel")?;

    cx.update_window(workspace_window.into(), |_, _window, cx| {
        workspace_window
            .update(cx, |workspace, window, cx| {
                workspace.add_panel(panel.clone(), window, cx);
                workspace.open_panel::<AgentPanel>(window, cx);
            })
            .ok();
    })?;

    cx.run_until_parked();

    // Open the stub thread
    cx.update_window(workspace_window.into(), |_, window, cx| {
        panel.update(cx, |panel: &mut agent_ui::AgentPanel, cx| {
            panel.open_external_thread_with_server(stub_agent.clone(), window, cx);
        });
    })?;

    cx.run_until_parked();

    // Get the thread view and send a message to trigger the subagent tool call
    let thread_view = cx
        .read(|cx| panel.read(cx).active_thread_view_for_tests().cloned())
        .ok_or_else(|| anyhow::anyhow!("No active thread view"))?;

    let thread = cx
        .read(|cx| thread_view.read(cx).thread().cloned())
        .ok_or_else(|| anyhow::anyhow!("Thread not available"))?;

    // Send the message to trigger the subagent response
    let send_future = thread.update(cx, |thread: &mut acp_thread::AcpThread, cx| {
        thread.send(vec!["Run two subagents".into()], cx)
    });

    let _ = cx.foreground_executor.block_test(send_future);

    cx.run_until_parked();

    // Get the tool call ID
    let tool_call_id = cx
        .read(|cx| {
            thread.read(cx).entries().iter().find_map(|entry| {
                if let acp_thread::AgentThreadEntry::ToolCall(tool_call) = entry {
                    Some(tool_call.id.clone())
                } else {
                    None
                }
            })
        })
        .ok_or_else(|| anyhow::anyhow!("Expected a ToolCall entry in thread"))?;

    // Create two subagent AcpThreads and inject them
    let subagent1 = cx.update(|cx| {
        let action_log = cx.new(|_| action_log::ActionLog::new(project.clone()));
        let session_id = acp::SessionId::new("subagent-1");
        cx.new(|cx| {
            let mut thread = AcpThread::new(
                "Exploring test-repo",
                Rc::new(connection.clone()),
                project.clone(),
                action_log,
                session_id,
                watch::Receiver::constant(acp::PromptCapabilities::new()),
                cx,
            );
            // Add some content to this subagent
            thread.push_assistant_content_block(
                "## Summary of test-repo\n\nThis is a test repository with:\n\n- **Files:** test.txt\n- **Purpose:** Testing".into(),
                false,
                cx,
            );
            thread
        })
    });

    let subagent2 = cx.update(|cx| {
        let action_log = cx.new(|_| action_log::ActionLog::new(project.clone()));
        let session_id = acp::SessionId::new("subagent-2");
        cx.new(|cx| {
            let mut thread = AcpThread::new(
                "Exploring test-worktree",
                Rc::new(connection.clone()),
                project.clone(),
                action_log,
                session_id,
                watch::Receiver::constant(acp::PromptCapabilities::new()),
                cx,
            );
            // Add some content to this subagent
            thread.push_assistant_content_block(
                "## Summary of test-worktree\n\nThis directory contains:\n\n- A single `config.json` file\n- Basic project setup".into(),
                false,
                cx,
            );
            thread
        })
    });

    // Inject subagent threads into the tool call
    thread.update(cx, |thread: &mut acp_thread::AcpThread, cx| {
        thread
            .update_tool_call(
                ToolCallUpdateSubagentThread {
                    id: tool_call_id.clone(),
                    thread: subagent1,
                },
                cx,
            )
            .ok();
        thread
            .update_tool_call(
                ToolCallUpdateSubagentThread {
                    id: tool_call_id.clone(),
                    thread: subagent2,
                },
                cx,
            )
            .ok();
    });

    cx.run_until_parked();

    cx.update_window(workspace_window.into(), |_, window, _cx| {
        window.refresh();
    })?;

    cx.run_until_parked();

    // Capture subagents in RUNNING state (tool call still in progress)
    let running_result = run_visual_test(
        "subagent_cards_running",
        workspace_window.into(),
        cx,
        update_baseline,
    )?;

    // Now mark the tool call as completed by updating it through the thread
    thread.update(cx, |thread: &mut acp_thread::AcpThread, cx| {
        thread
            .handle_session_update(
                acp::SessionUpdate::ToolCallUpdate(acp::ToolCallUpdate::new(
                    tool_call_id.clone(),
                    acp::ToolCallUpdateFields::new().status(acp::ToolCallStatus::Completed),
                )),
                cx,
            )
            .ok();
    });

    cx.run_until_parked();

    cx.update_window(workspace_window.into(), |_, window, _cx| {
        window.refresh();
    })?;

    cx.run_until_parked();

    // Capture subagents in COMPLETED state
    let completed_result = run_visual_test(
        "subagent_cards_completed",
        workspace_window.into(),
        cx,
        update_baseline,
    )?;

    // Expand the first subagent
    thread_view.update(cx, |view: &mut agent_ui::acp::AcpThreadView, cx| {
        view.expand_subagent(acp::SessionId::new("subagent-1"), cx);
    });

    cx.run_until_parked();

    cx.update_window(workspace_window.into(), |_, window, _cx| {
        window.refresh();
    })?;

    cx.run_until_parked();

    // Capture subagent in EXPANDED state
    let expanded_result = run_visual_test(
        "subagent_cards_expanded",
        workspace_window.into(),
        cx,
        update_baseline,
    )?;

    // Cleanup
    workspace_window
        .update(cx, |workspace, _window, cx| {
            let project = workspace.project().clone();
            project.update(cx, |project, cx| {
                let worktree_ids: Vec<_> =
                    project.worktrees(cx).map(|wt| wt.read(cx).id()).collect();
                for id in worktree_ids {
                    project.remove_worktree(id, cx);
                }
            });
        })
        .ok();

    cx.run_until_parked();

    let _ = cx.update_window(workspace_window.into(), |_, window, _cx| {
        window.remove_window();
    });

    cx.run_until_parked();

    for _ in 0..15 {
        cx.advance_clock(Duration::from_millis(100));
        cx.run_until_parked();
    }

    match (&running_result, &completed_result, &expanded_result) {
        (TestResult::Passed, TestResult::Passed, TestResult::Passed) => Ok(TestResult::Passed),
        (TestResult::BaselineUpdated(p), _, _)
        | (_, TestResult::BaselineUpdated(p), _)
        | (_, _, TestResult::BaselineUpdated(p)) => Ok(TestResult::BaselineUpdated(p.clone())),
    }
}

#[cfg(all(target_os = "macos", feature = "visual-tests"))]
fn run_agent_thread_view_test(
    app_state: Arc<AppState>,
    cx: &mut VisualTestAppContext,
    update_baseline: bool,
) -> Result<TestResult> {
    use agent::AgentTool;
    use agent_ui::AgentPanel;

    // Create a temporary directory with the test image
    // Canonicalize to resolve symlinks (on macOS, /var -> /private/var)
    // Use keep() to prevent auto-cleanup - we'll clean up manually after stopping background tasks
    let temp_dir = tempfile::tempdir()?;
    let temp_path = temp_dir.keep();
    let canonical_temp = temp_path.canonicalize()?;
    let project_path = canonical_temp.join("project");
    std::fs::create_dir_all(&project_path)?;
    let image_path = project_path.join("test-image.png");
    std::fs::write(&image_path, EMBEDDED_TEST_IMAGE)?;

    // Create a project with the test image
    let project = cx.update(|cx| {
        project::Project::local(
            app_state.client.clone(),
            app_state.node_runtime.clone(),
            app_state.user_store.clone(),
            app_state.languages.clone(),
            app_state.fs.clone(),
            None,
            false,
            cx,
        )
    });

    // Add the test directory as a worktree
    let add_worktree_task = project.update(cx, |project, cx| {
        project.find_or_create_worktree(&project_path, true, cx)
    });

    cx.background_executor.allow_parking();
    let (worktree, _) = cx
        .foreground_executor
        .block_test(add_worktree_task)
        .context("Failed to add worktree")?;
    cx.background_executor.forbid_parking();

    cx.run_until_parked();

    let worktree_name = cx.read(|cx| worktree.read(cx).root_name_str().to_string());

    // Create the necessary entities for the ReadFileTool
    let action_log = cx.update(|cx| cx.new(|_| action_log::ActionLog::new(project.clone())));
    let context_server_registry = cx.update(|cx| {
        cx.new(|cx| agent::ContextServerRegistry::new(project.read(cx).context_server_store(), cx))
    });
    let fake_model = Arc::new(language_model::fake_provider::FakeLanguageModel::default());
    let project_context = cx.update(|cx| cx.new(|_| prompt_store::ProjectContext::default()));

    // Create the agent Thread
    let thread = cx.update(|cx| {
        cx.new(|cx| {
            agent::Thread::new(
                project.clone(),
                project_context,
                context_server_registry,
                agent::Templates::new(),
                Some(fake_model),
                cx,
            )
        })
    });

    // Create the ReadFileTool
    let tool = Arc::new(agent::ReadFileTool::new(
        thread.downgrade(),
        project.clone(),
        action_log,
    ));

    // Create a test event stream to capture tool output
    let (event_stream, mut event_receiver) = agent::ToolCallEventStream::test();

    // Run the real ReadFileTool to get the actual image content
    let input = agent::ReadFileToolInput {
        path: format!("{}/test-image.png", worktree_name),
        start_line: None,
        end_line: None,
    };
    let run_task = cx.update(|cx| tool.clone().run(input, event_stream, cx));

    cx.background_executor.allow_parking();
    let run_result = cx.foreground_executor.block_test(run_task);
    cx.background_executor.forbid_parking();
    run_result.context("ReadFileTool failed")?;

    cx.run_until_parked();

    // Collect the events from the tool execution
    let mut tool_content: Vec<acp::ToolCallContent> = Vec::new();
    let mut tool_locations: Vec<acp::ToolCallLocation> = Vec::new();

    while let Ok(Some(event)) = event_receiver.try_next() {
        if let Ok(agent::ThreadEvent::ToolCallUpdate(acp_thread::ToolCallUpdate::UpdateFields(
            update,
        ))) = event
        {
            if let Some(content) = update.fields.content {
                tool_content.extend(content);
            }
            if let Some(locations) = update.fields.locations {
                tool_locations.extend(locations);
            }
        }
    }

    if tool_content.is_empty() {
        return Err(anyhow::anyhow!("ReadFileTool did not produce any content"));
    }

    // Create stub connection with the real tool output
    let connection = StubAgentConnection::new();
    connection.set_next_prompt_updates(vec![acp::SessionUpdate::ToolCall(
        acp::ToolCall::new(
            "read_file",
            format!("Read file `{}/test-image.png`", worktree_name),
        )
        .kind(acp::ToolKind::Read)
        .status(acp::ToolCallStatus::Completed)
        .locations(tool_locations)
        .content(tool_content),
    )]);

    let stub_agent: Rc<dyn AgentServer> = Rc::new(StubAgentServer::new(connection));

    // Create a window sized for the agent panel
    let window_size = size(px(500.0), px(900.0));
    let bounds = Bounds {
        origin: point(px(0.0), px(0.0)),
        size: window_size,
    };

    let workspace_window: WindowHandle<Workspace> = cx
        .update(|cx| {
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    focus: false,
                    show: false,
                    ..Default::default()
                },
                |window, cx| {
                    cx.new(|cx| {
                        Workspace::new(None, project.clone(), app_state.clone(), window, cx)
                    })
                },
            )
        })
        .context("Failed to open agent window")?;

    cx.run_until_parked();

    // Load the AgentPanel
    let (weak_workspace, async_window_cx) = workspace_window
        .update(cx, |workspace, window, cx| {
            (workspace.weak_handle(), window.to_async(cx))
        })
        .context("Failed to get workspace handle")?;

    let prompt_builder =
        cx.update(|cx| prompt_store::PromptBuilder::load(app_state.fs.clone(), false, cx));
    cx.background_executor.allow_parking();
    let panel = cx
        .foreground_executor
        .block_test(AgentPanel::load(
            weak_workspace,
            prompt_builder,
            async_window_cx,
        ))
        .context("Failed to load AgentPanel")?;
    cx.background_executor.forbid_parking();

    cx.update_window(workspace_window.into(), |_, _window, cx| {
        workspace_window
            .update(cx, |workspace, window, cx| {
                workspace.add_panel(panel.clone(), window, cx);
                workspace.open_panel::<AgentPanel>(window, cx);
            })
            .ok();
    })?;

    cx.run_until_parked();

    // Inject the stub server and open the stub thread
    cx.update_window(workspace_window.into(), |_, window, cx| {
        panel.update(cx, |panel, cx| {
            panel.open_external_thread_with_server(stub_agent.clone(), window, cx);
        });
    })?;

    cx.run_until_parked();

    // Get the thread view and send a message
    let thread_view = cx
        .read(|cx| panel.read(cx).active_thread_view_for_tests().cloned())
        .ok_or_else(|| anyhow::anyhow!("No active thread view"))?;

    let thread = cx
        .read(|cx| thread_view.read(cx).thread().cloned())
        .ok_or_else(|| anyhow::anyhow!("Thread not available"))?;

    // Send the message to trigger the image response
    let send_future = thread.update(cx, |thread, cx| {
        thread.send(vec!["Show me the Zed logo".into()], cx)
    });

    cx.background_executor.allow_parking();
    let send_result = cx.foreground_executor.block_test(send_future);
    cx.background_executor.forbid_parking();
    send_result.context("Failed to send message")?;

    cx.run_until_parked();

    // Get the tool call ID for expanding later
    let tool_call_id = cx
        .read(|cx| {
            thread.read(cx).entries().iter().find_map(|entry| {
                if let acp_thread::AgentThreadEntry::ToolCall(tool_call) = entry {
                    Some(tool_call.id.clone())
                } else {
                    None
                }
            })
        })
        .ok_or_else(|| anyhow::anyhow!("Expected a ToolCall entry in thread"))?;

    cx.update_window(workspace_window.into(), |_, window, _cx| {
        window.refresh();
    })?;

    cx.run_until_parked();

    // Capture the COLLAPSED state
    let collapsed_result = run_visual_test(
        "agent_thread_with_image_collapsed",
        workspace_window.into(),
        cx,
        update_baseline,
    )?;

    // Now expand the tool call so the image is visible
    thread_view.update(cx, |view, cx| {
        view.expand_tool_call(tool_call_id, cx);
    });

    cx.run_until_parked();

    cx.update_window(workspace_window.into(), |_, window, _cx| {
        window.refresh();
    })?;

    cx.run_until_parked();

    // Capture the EXPANDED state
    let expanded_result = run_visual_test(
        "agent_thread_with_image_expanded",
        workspace_window.into(),
        cx,
        update_baseline,
    )?;

    // Remove the worktree from the project to stop background scanning tasks
    // This prevents "root path could not be canonicalized" errors when we clean up
    workspace_window
        .update(cx, |workspace, _window, cx| {
            let project = workspace.project().clone();
            project.update(cx, |project, cx| {
                let worktree_ids: Vec<_> =
                    project.worktrees(cx).map(|wt| wt.read(cx).id()).collect();
                for id in worktree_ids {
                    project.remove_worktree(id, cx);
                }
            });
        })
        .ok();

    cx.run_until_parked();

    // Close the window
    // Note: This may cause benign "editor::scroll window not found" errors from scrollbar
    // auto-hide timers that were scheduled before the window was closed. These errors
    // don't affect test results.
    let _ = cx.update_window(workspace_window.into(), |_, window, _cx| {
        window.remove_window();
    });

    // Run until all cleanup tasks complete
    cx.run_until_parked();

    // Give background tasks time to finish, including scrollbar hide timers (1 second)
    for _ in 0..15 {
        cx.advance_clock(Duration::from_millis(100));
        cx.run_until_parked();
    }

    // Note: We don't delete temp_path here because background worktree tasks may still
    // be running. The directory will be cleaned up when the process exits.

    match (&collapsed_result, &expanded_result) {
        (TestResult::Passed, TestResult::Passed) => Ok(TestResult::Passed),
        (TestResult::BaselineUpdated(p), _) | (_, TestResult::BaselineUpdated(p)) => {
            Ok(TestResult::BaselineUpdated(p.clone()))
        }
    }
}
