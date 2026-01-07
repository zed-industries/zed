//! Visual Test Runner
//!
//! This binary runs visual regression tests for Zed's UI. It captures screenshots
//! of real Zed windows and compares them against baseline images.
//!
//! ## How It Works
//!
//! This tool uses direct texture capture - it renders the scene to a Metal texture
//! and reads the pixels back directly. This approach:
//! - Does NOT require Screen Recording permission
//! - Does NOT require the window to be visible on screen
//! - Captures raw GPUI output without system window chrome
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

use anyhow::{Context, Result};
use gpui::{
    App, AppContext as _, Application, Bounds, Window, WindowBounds, WindowHandle, WindowOptions,
    point, px, size,
};
use image::RgbaImage;
use project_panel::ProjectPanel;
use settings::SettingsStore;
use std::any::Any;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use workspace::{AppState, Workspace};

use acp_thread::{AgentConnection, StubAgentConnection};
use agent_client_protocol as acp;
use agent_servers::{AgentServer, AgentServerDelegate};
use gpui::SharedString;

/// Baseline images are stored relative to this file
const BASELINE_DIR: &str = "crates/zed/test_fixtures/visual_tests";

/// Embedded test image (Zed app icon) for visual tests.
const EMBEDDED_TEST_IMAGE: &[u8] = include_bytes!("../resources/app-icon.png");

/// Threshold for image comparison (0.0 to 1.0)
/// Images must match at least this percentage to pass
const MATCH_THRESHOLD: f64 = 0.99;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let update_baseline = std::env::var("UPDATE_BASELINE").is_ok();

    if update_baseline {
        println!("=== Visual Test Runner (UPDATE MODE) ===\n");
        println!("Baseline images will be updated.\n");
    } else {
        println!("=== Visual Test Runner ===\n");
    }

    // Create a temporary directory for test files
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let project_path = temp_dir.path().join("project");
    std::fs::create_dir_all(&project_path).expect("Failed to create project directory");

    // Create test files in the real filesystem
    create_test_files(&project_path);

    let test_result = std::panic::catch_unwind(|| {
        let project_path = project_path;
        Application::new()
            .with_assets(assets::Assets)
            .run(move |cx| {
                // Initialize settings store first (required by theme and other subsystems)
                let settings_store = SettingsStore::test(cx);
                cx.set_global(settings_store);

                // Create AppState using the production-like initialization
                let app_state = init_app_state(cx);

                // Initialize all Zed subsystems
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

                // Open a real Zed workspace window
                let window_size = size(px(1280.0), px(800.0));
                // Window can be hidden since we use direct texture capture (reading pixels from
                // Metal texture) instead of ScreenCaptureKit which requires visible windows.
                let bounds = Bounds {
                    origin: point(px(0.0), px(0.0)),
                    size: window_size,
                };

                // Create a project for the workspace
                let project = project::Project::local(
                    app_state.client.clone(),
                    app_state.node_runtime.clone(),
                    app_state.user_store.clone(),
                    app_state.languages.clone(),
                    app_state.fs.clone(),
                    None,
                    false,
                    cx,
                );

                let workspace_window: WindowHandle<Workspace> = cx
                    .open_window(
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
                    .expect("Failed to open workspace window");

                // Add the test project as a worktree directly to the project
                let add_worktree_task = workspace_window
                    .update(cx, |workspace, _window, cx| {
                        workspace.project().update(cx, |project, cx| {
                            project.find_or_create_worktree(&project_path, true, cx)
                        })
                    })
                    .expect("Failed to update workspace");

                // Clone app_state for the async block
                let app_state_for_tests = app_state.clone();

                // Spawn async task to set up the UI and capture screenshot
                cx.spawn(async move |mut cx| {
                    // Wait for the worktree to be added
                    if let Err(e) = add_worktree_task.await {
                        eprintln!("Failed to add worktree: {:?}", e);
                    }

                    // Wait for UI to settle
                    cx.background_executor()
                        .timer(std::time::Duration::from_millis(500))
                        .await;

                    // Create and add the project panel to the workspace
                    let panel_task = cx.update(|cx| {
                        workspace_window
                            .update(cx, |_workspace, window, cx| {
                                let weak_workspace = cx.weak_entity();
                                window.spawn(cx, async move |cx| {
                                    ProjectPanel::load(weak_workspace, cx.clone()).await
                                })
                            })
                            .ok()
                    });

                    if let Ok(Some(task)) = panel_task {
                        if let Ok(panel) = task.await {
                            cx.update(|cx| {
                                workspace_window
                                    .update(cx, |workspace, window, cx| {
                                        workspace.add_panel(panel, window, cx);
                                    })
                                    .ok();
                            })
                            .ok();
                        }
                    }

                    // Wait for panel to be added
                    cx.background_executor()
                        .timer(std::time::Duration::from_millis(500))
                        .await;

                    // Open the project panel
                    cx.update(|cx| {
                        workspace_window
                            .update(cx, |workspace, window, cx| {
                                workspace.open_panel::<ProjectPanel>(window, cx);
                            })
                            .ok();
                    })
                    .ok();

                    // Wait for project panel to render
                    cx.background_executor()
                        .timer(std::time::Duration::from_millis(500))
                        .await;

                    // Open main.rs in the editor
                    let open_file_task = cx.update(|cx| {
                        workspace_window
                            .update(cx, |workspace, window, cx| {
                                let worktree = workspace.project().read(cx).worktrees(cx).next();
                                if let Some(worktree) = worktree {
                                    let worktree_id = worktree.read(cx).id();
                                    let rel_path: std::sync::Arc<util::rel_path::RelPath> =
                                        util::rel_path::rel_path("src/main.rs").into();
                                    let project_path: project::ProjectPath =
                                        (worktree_id, rel_path).into();
                                    Some(workspace.open_path(project_path, None, true, window, cx))
                                } else {
                                    None
                                }
                            })
                            .ok()
                            .flatten()
                    });

                    if let Ok(Some(task)) = open_file_task {
                        if let Ok(item) = task.await {
                            // Focus the opened item to dismiss the welcome screen
                            cx.update(|cx| {
                                workspace_window
                                    .update(cx, |workspace, window, cx| {
                                        let pane = workspace.active_pane().clone();
                                        pane.update(cx, |pane, cx| {
                                            if let Some(index) = pane.index_for_item(item.as_ref())
                                            {
                                                pane.activate_item(index, true, true, window, cx);
                                            }
                                        });
                                    })
                                    .ok();
                            })
                            .ok();

                            // Wait for item activation to render
                            cx.background_executor()
                                .timer(std::time::Duration::from_millis(500))
                                .await;
                        }
                    }

                    // Request a window refresh to ensure all pending effects are processed
                    cx.refresh().ok();

                    // Wait for UI to fully stabilize
                    cx.background_executor()
                        .timer(std::time::Duration::from_secs(2))
                        .await;

                    // Track test results
                    let mut passed = 0;
                    let mut failed = 0;
                    let mut updated = 0;

                    // Run Test 1: Project Panel (with project panel visible)
                    println!("\n--- Test 1: project_panel ---");
                    let test_result = run_visual_test(
                        "project_panel",
                        workspace_window.into(),
                        &mut cx,
                        update_baseline,
                    )
                    .await;

                    match test_result {
                        Ok(TestResult::Passed) => {
                            println!("✓ project_panel: PASSED");
                            passed += 1;
                        }
                        Ok(TestResult::BaselineUpdated(path)) => {
                            println!("✓ project_panel: Baseline updated at {}", path.display());
                            updated += 1;
                        }
                        Err(e) => {
                            eprintln!("✗ project_panel: FAILED - {}", e);
                            failed += 1;
                        }
                    }

                    // Close the project panel for the second test
                    cx.update(|cx| {
                        workspace_window
                            .update(cx, |workspace, window, cx| {
                                workspace.close_panel::<ProjectPanel>(window, cx);
                            })
                            .ok();
                    })
                    .ok();

                    // Refresh and wait for panel to close
                    cx.refresh().ok();
                    cx.background_executor()
                        .timer(std::time::Duration::from_millis(500))
                        .await;

                    // Run Test 2: Workspace with Editor (without project panel)
                    println!("\n--- Test 2: workspace_with_editor ---");
                    let test_result = run_visual_test(
                        "workspace_with_editor",
                        workspace_window.into(),
                        &mut cx,
                        update_baseline,
                    )
                    .await;

                    match test_result {
                        Ok(TestResult::Passed) => {
                            println!("✓ workspace_with_editor: PASSED");
                            passed += 1;
                        }
                        Ok(TestResult::BaselineUpdated(path)) => {
                            println!(
                                "✓ workspace_with_editor: Baseline updated at {}",
                                path.display()
                            );
                            updated += 1;
                        }
                        Err(e) => {
                            eprintln!("✗ workspace_with_editor: FAILED - {}", e);
                            failed += 1;
                        }
                    }

                    // Run Test 3: Agent Thread View with Image (collapsed and expanded)
                    println!("\n--- Test 3: agent_thread_with_image (collapsed + expanded) ---");
                    let test_result = run_agent_thread_view_test(
                        app_state_for_tests.clone(),
                        &mut cx,
                        update_baseline,
                    )
                    .await;

                    match test_result {
                        Ok(TestResult::Passed) => {
                            println!("✓ agent_thread_with_image (collapsed + expanded): PASSED");
                            passed += 1;
                        }
                        Ok(TestResult::BaselineUpdated(_)) => {
                            println!(
                                "✓ agent_thread_with_image: Baselines updated (collapsed + expanded)"
                            );
                            updated += 1;
                        }
                        Err(e) => {
                            eprintln!("✗ agent_thread_with_image: FAILED - {}", e);
                            failed += 1;
                        }
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
                        cx.update(|cx| cx.quit()).ok();
                        std::process::exit(1);
                    } else {
                        println!("\n=== All Visual Tests PASSED ===");
                    }

                    cx.update(|cx| cx.quit()).ok();
                })
                .detach();
            });
    });

    // Keep temp_dir alive until we're done
    drop(temp_dir);

    if test_result.is_err() {
        std::process::exit(1);
    }
}

enum TestResult {
    Passed,
    BaselineUpdated(PathBuf),
}

async fn run_visual_test(
    test_name: &str,
    window: gpui::AnyWindowHandle,
    cx: &mut gpui::AsyncApp,
    update_baseline: bool,
) -> Result<TestResult> {
    // Capture the screenshot using direct texture capture (no ScreenCaptureKit needed)
    let screenshot = cx.update(|cx| capture_screenshot(window, cx))??;

    // Get paths
    let baseline_path = get_baseline_path(test_name);
    let output_dir = std::env::var("VISUAL_TEST_OUTPUT_DIR")
        .unwrap_or_else(|_| "target/visual_tests".to_string());
    let actual_path = Path::new(&output_dir).join(format!("{}.png", test_name));

    // Create output directory
    if let Some(parent) = actual_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Save the actual screenshot
    screenshot.save(&actual_path)?;
    println!("Screenshot saved to: {}", actual_path.display());

    if update_baseline {
        // Update the baseline
        if let Some(parent) = baseline_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        screenshot.save(&baseline_path)?;
        return Ok(TestResult::BaselineUpdated(baseline_path));
    }

    // Compare against baseline
    if !baseline_path.exists() {
        return Err(anyhow::anyhow!(
            "Baseline image not found: {}\n\
             Run with UPDATE_BASELINE=1 to create it.",
            baseline_path.display()
        ));
    }

    let baseline = image::open(&baseline_path)
        .context("Failed to load baseline image")?
        .to_rgba8();

    let comparison = compare_images(&baseline, &screenshot);

    println!(
        "Image comparison: {:.2}% match ({} different pixels out of {})",
        comparison.match_percentage * 100.0,
        comparison.diff_pixel_count,
        comparison.total_pixels
    );

    if comparison.match_percentage >= MATCH_THRESHOLD {
        Ok(TestResult::Passed)
    } else {
        // Save the diff image for debugging
        if let Some(diff_image) = comparison.diff_image {
            let diff_path = Path::new(&output_dir).join(format!("{}_diff.png", test_name));
            diff_image.save(&diff_path)?;
            println!("Diff image saved to: {}", diff_path.display());
        }

        Err(anyhow::anyhow!(
            "Screenshot does not match baseline.\n\
             Match: {:.2}% (threshold: {:.2}%)\n\
             Actual: {}\n\
             Baseline: {}\n\
             \n\
             Run with UPDATE_BASELINE=1 to update the baseline if this change is intentional.",
            comparison.match_percentage * 100.0,
            MATCH_THRESHOLD * 100.0,
            actual_path.display(),
            baseline_path.display()
        ))
    }
}

fn get_baseline_path(test_name: &str) -> PathBuf {
    // Find the workspace root by looking for Cargo.toml
    let mut path = std::env::current_dir().expect("Failed to get current directory");
    while !path.join("Cargo.toml").exists() || !path.join("crates").exists() {
        if !path.pop() {
            panic!("Could not find workspace root");
        }
    }
    path.join(BASELINE_DIR).join(format!("{}.png", test_name))
}

struct ImageComparison {
    match_percentage: f64,
    diff_image: Option<RgbaImage>,
    diff_pixel_count: u64,
    total_pixels: u64,
}

fn compare_images(baseline: &RgbaImage, actual: &RgbaImage) -> ImageComparison {
    // Check dimensions
    if baseline.dimensions() != actual.dimensions() {
        return ImageComparison {
            match_percentage: 0.0,
            diff_image: None,
            diff_pixel_count: baseline.width() as u64 * baseline.height() as u64,
            total_pixels: baseline.width() as u64 * baseline.height() as u64,
        };
    }

    let (width, height) = baseline.dimensions();
    let total_pixels = width as u64 * height as u64;
    let mut diff_count: u64 = 0;
    let mut diff_image = RgbaImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let baseline_pixel = baseline.get_pixel(x, y);
            let actual_pixel = actual.get_pixel(x, y);

            if pixels_are_similar(baseline_pixel, actual_pixel) {
                // Matching pixel - show as dimmed version of actual
                diff_image.put_pixel(
                    x,
                    y,
                    image::Rgba([
                        actual_pixel[0] / 3,
                        actual_pixel[1] / 3,
                        actual_pixel[2] / 3,
                        255,
                    ]),
                );
            } else {
                diff_count += 1;
                // Different pixel - highlight in red
                diff_image.put_pixel(x, y, image::Rgba([255, 0, 0, 255]));
            }
        }
    }

    let match_percentage = if total_pixels > 0 {
        (total_pixels - diff_count) as f64 / total_pixels as f64
    } else {
        1.0
    };

    ImageComparison {
        match_percentage,
        diff_image: Some(diff_image),
        diff_pixel_count: diff_count,
        total_pixels,
    }
}

fn pixels_are_similar(a: &image::Rgba<u8>, b: &image::Rgba<u8>) -> bool {
    // Allow small differences due to anti-aliasing, font rendering, etc.
    const TOLERANCE: i16 = 2;

    (a[0] as i16 - b[0] as i16).abs() <= TOLERANCE
        && (a[1] as i16 - b[1] as i16).abs() <= TOLERANCE
        && (a[2] as i16 - b[2] as i16).abs() <= TOLERANCE
        && (a[3] as i16 - b[3] as i16).abs() <= TOLERANCE
}

fn capture_screenshot(window: gpui::AnyWindowHandle, cx: &mut gpui::App) -> Result<RgbaImage> {
    // Use direct texture capture - renders the scene to a texture and reads pixels back.
    // This does not require the window to be visible on screen.
    let screenshot = cx.update_window(window, |_view, window: &mut Window, _cx| {
        window.render_to_image()
    })??;

    println!(
        "Screenshot captured: {}x{} pixels",
        screenshot.width(),
        screenshot.height()
    );

    Ok(screenshot)
}

/// Create test files in a real filesystem directory
fn create_test_files(project_path: &Path) {
    let src_dir = project_path.join("src");
    std::fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    std::fs::write(
        src_dir.join("main.rs"),
        r#"fn main() {
    println!("Hello, world!");

    let message = greet("Zed");
    println!("{}", message);
}

fn greet(name: &str) -> String {
    format!("Welcome to {}, the editor of the future!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("World"), "Welcome to World, the editor of the future!");
    }
}
"#,
    )
    .expect("Failed to write main.rs");

    std::fs::write(
        src_dir.join("lib.rs"),
        r#"//! A sample library for visual testing.

pub mod utils;

/// Adds two numbers together.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Subtracts the second number from the first.
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 3), 2);
    }
}
"#,
    )
    .expect("Failed to write lib.rs");

    std::fs::write(
        src_dir.join("utils.rs"),
        r#"//! Utility functions for the sample project.

/// Formats a greeting message.
pub fn format_greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// Formats a farewell message.
pub fn format_farewell(name: &str) -> String {
    format!("Goodbye, {}!", name)
}
"#,
    )
    .expect("Failed to write utils.rs");

    std::fs::write(
        project_path.join("Cargo.toml"),
        r#"[package]
name = "test-project"
version = "0.1.0"
edition = "2021"

[dependencies]

[dev-dependencies]
"#,
    )
    .expect("Failed to write Cargo.toml");

    std::fs::write(
        project_path.join("README.md"),
        r#"# Test Project

This is a test project for visual testing of Zed.

## Description

A simple Rust project used to verify that Zed's visual testing
infrastructure can capture screenshots of real workspaces.

## Features

- Sample Rust code with main.rs, lib.rs, and utils.rs
- Standard Cargo.toml configuration
- Example tests

## Building

```bash
cargo build
```

## Testing

```bash
cargo test
```
"#,
    )
    .expect("Failed to write README.md");
}

/// Initialize AppState with real filesystem for visual testing.
fn init_app_state(cx: &mut gpui::App) -> Arc<AppState> {
    use client::Client;
    use clock::FakeSystemClock;
    use fs::RealFs;
    use language::LanguageRegistry;
    use node_runtime::NodeRuntime;
    use session::Session;

    let fs = Arc::new(RealFs::new(None, cx.background_executor().clone()));
    let languages = Arc::new(LanguageRegistry::new(cx.background_executor().clone()));
    let clock = Arc::new(FakeSystemClock::new());
    let http_client = http_client::FakeHttpClient::with_404_response();
    let client = Client::new(clock, http_client, cx);
    let session = cx.new(|cx| session::AppSession::new(Session::test(), cx));
    let user_store = cx.new(|cx| client::UserStore::new(client.clone(), cx));
    let workspace_store = cx.new(|cx| workspace::WorkspaceStore::new(client.clone(), cx));

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

/// A stub AgentServer for visual testing that returns a pre-programmed connection.
#[derive(Clone)]
struct StubAgentServer {
    connection: StubAgentConnection,
}

impl StubAgentServer {
    fn new(connection: StubAgentConnection) -> Self {
        Self { connection }
    }
}

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

/// Runs the agent panel visual test with full UI chrome.
/// This test actually runs the real ReadFileTool to capture image output.
async fn run_agent_thread_view_test(
    app_state: Arc<AppState>,
    cx: &mut gpui::AsyncApp,
    update_baseline: bool,
) -> Result<TestResult> {
    use agent::AgentTool;
    use agent_ui::AgentPanel;

    // Create a temporary directory with the test image using real filesystem
    let temp_dir = tempfile::tempdir()?;
    let project_path = temp_dir.path().join("project");
    std::fs::create_dir_all(&project_path)?;
    let image_path = project_path.join("test-image.png");
    std::fs::write(&image_path, EMBEDDED_TEST_IMAGE)?;

    // Create a project with the real filesystem containing the test image
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
    })?;

    // Add the test directory as a worktree
    let add_worktree_task = project.update(cx, |project, cx| {
        project.find_or_create_worktree(&project_path, true, cx)
    })?;
    let (worktree, _) = add_worktree_task.await?;

    // Wait for worktree to scan and find the image file
    let worktree_name = worktree.read_with(cx, |wt, _| wt.root_name_str().to_string())?;

    // Wait for worktree to be fully scanned
    cx.background_executor()
        .timer(std::time::Duration::from_millis(100))
        .await;

    // Create the necessary entities for the ReadFileTool
    let action_log = cx.new(|_| action_log::ActionLog::new(project.clone()))?;
    let context_server_registry = cx
        .new(|cx| agent::ContextServerRegistry::new(project.read(cx).context_server_store(), cx))?;
    let fake_model = Arc::new(language_model::fake_provider::FakeLanguageModel::default());
    let project_context = cx.new(|_| prompt_store::ProjectContext::default())?;

    // Create the agent Thread
    let thread = cx.new(|cx| {
        agent::Thread::new(
            project.clone(),
            project_context,
            context_server_registry,
            agent::Templates::new(),
            Some(fake_model),
            cx,
        )
    })?;

    // Create the ReadFileTool
    let tool = Arc::new(agent::ReadFileTool::new(
        thread.downgrade(),
        project.clone(),
        action_log,
    ));

    // Create a test event stream to capture tool output
    let (event_stream, mut event_receiver) = agent::ToolCallEventStream::test();

    // Run the real ReadFileTool to get the actual image content
    // The path is relative to the worktree root name
    let input = agent::ReadFileToolInput {
        path: format!("{}/test-image.png", worktree_name),
        start_line: None,
        end_line: None,
    };
    let run_task = cx.update(|cx| tool.clone().run(input, event_stream, cx))?;

    // The tool runs async - wait for it
    run_task.await?;

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

    // Verify we got image content from the real tool
    if tool_content.is_empty() {
        return Err(anyhow::anyhow!(
            "ReadFileTool did not produce any content - the tool is broken!"
        ));
    }

    // Create stub connection with the REAL tool output
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

    let stub_agent: Rc<dyn AgentServer> = Rc::new(StubAgentServer::new(connection.clone()));

    // Create a window sized for the agent panel (500x900)
    let window_size = size(px(500.0), px(900.0));
    let bounds = Bounds {
        origin: point(px(0.0), px(0.0)),
        size: window_size,
    };

    // Create a workspace window
    let workspace_window: WindowHandle<Workspace> = cx.update(|cx| {
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                focus: false,
                show: false,
                ..Default::default()
            },
            |window, cx| {
                cx.new(|cx| Workspace::new(None, project.clone(), app_state.clone(), window, cx))
            },
        )
    })??;

    // Wait for workspace to initialize
    cx.background_executor()
        .timer(std::time::Duration::from_millis(100))
        .await;

    // Load the AgentPanel
    let panel_task = workspace_window.update(cx, |_workspace, window, cx| {
        let weak_workspace = cx.weak_entity();
        let prompt_builder = prompt_store::PromptBuilder::load(app_state.fs.clone(), false, cx);
        let async_window_cx = window.to_async(cx);
        AgentPanel::load(weak_workspace, prompt_builder, async_window_cx)
    })?;

    let panel = panel_task.await?;

    // Add the panel to the workspace
    workspace_window.update(cx, |workspace, window, cx| {
        workspace.add_panel(panel.clone(), window, cx);
        workspace.open_panel::<AgentPanel>(window, cx);
    })?;

    // Wait for panel to be ready
    cx.background_executor()
        .timer(std::time::Duration::from_millis(200))
        .await;

    // Inject the stub server and open the stub thread
    workspace_window.update(cx, |_workspace, window, cx| {
        panel.update(cx, |panel, cx| {
            panel.open_external_thread_with_server(stub_agent.clone(), window, cx);
        });
    })?;

    // Wait for thread view to initialize
    cx.background_executor()
        .timer(std::time::Duration::from_millis(200))
        .await;

    // Get the thread view and send a message
    let thread_view = panel
        .read_with(cx, |panel, _| panel.active_thread_view_for_tests().cloned())?
        .ok_or_else(|| anyhow::anyhow!("No active thread view"))?;

    let thread = thread_view
        .update(cx, |view, _cx| view.thread().cloned())?
        .ok_or_else(|| anyhow::anyhow!("Thread not available"))?;

    // Send the message to trigger the image response
    thread
        .update(cx, |thread, cx| thread.send_raw("Show me the Zed logo", cx))?
        .await?;

    // Wait for response to be processed
    cx.background_executor()
        .timer(std::time::Duration::from_millis(500))
        .await;

    // Get the tool call ID for expanding later
    let tool_call_id = thread
        .update(cx, |thread, _cx| {
            thread.entries().iter().find_map(|entry| {
                if let acp_thread::AgentThreadEntry::ToolCall(tool_call) = entry {
                    Some(tool_call.id.clone())
                } else {
                    None
                }
            })
        })?
        .ok_or_else(|| anyhow::anyhow!("Expected a ToolCall entry in thread for visual test"))?;

    // Refresh window for collapsed state
    cx.update_window(
        workspace_window.into(),
        |_view, window: &mut Window, _cx| {
            window.refresh();
        },
    )?;

    cx.background_executor()
        .timer(std::time::Duration::from_millis(300))
        .await;

    // First, capture the COLLAPSED state (image tool call not expanded)
    let collapsed_result = run_visual_test(
        "agent_thread_with_image_collapsed",
        workspace_window.into(),
        cx,
        update_baseline,
    )
    .await?;

    // Now expand the tool call so its content (the image) is visible
    thread_view.update(cx, |view, cx| {
        view.expand_tool_call(tool_call_id, cx);
    })?;

    // Wait for UI to update
    cx.background_executor()
        .timer(std::time::Duration::from_millis(300))
        .await;

    // Refresh window for expanded state
    cx.update_window(
        workspace_window.into(),
        |_view, window: &mut Window, _cx| {
            window.refresh();
        },
    )?;

    cx.background_executor()
        .timer(std::time::Duration::from_millis(300))
        .await;

    // Capture the EXPANDED state (image visible)
    let expanded_result = run_visual_test(
        "agent_thread_with_image_expanded",
        workspace_window.into(),
        cx,
        update_baseline,
    )
    .await?;

    // Return pass only if both tests passed
    match (&collapsed_result, &expanded_result) {
        (TestResult::Passed, TestResult::Passed) => Ok(TestResult::Passed),
        (TestResult::BaselineUpdated(p1), TestResult::BaselineUpdated(_)) => {
            Ok(TestResult::BaselineUpdated(p1.clone()))
        }
        (TestResult::Passed, TestResult::BaselineUpdated(p)) => {
            Ok(TestResult::BaselineUpdated(p.clone()))
        }
        (TestResult::BaselineUpdated(p), TestResult::Passed) => {
            Ok(TestResult::BaselineUpdated(p.clone()))
        }
    }
}
