//! Visual Test Runner
//!
//! This binary runs visual tests on the main thread, which is required on macOS
//! because App construction must happen on the main thread.
//!
//! ## Prerequisites
//!
//! **Screen Recording Permission Required**: This tool uses macOS ScreenCaptureKit
//! to capture window screenshots. You must grant Screen Recording permission:
//!
//! 1. Run this tool once - macOS will prompt for permission
//! 2. Or manually: System Settings > Privacy & Security > Screen Recording
//! 3. Enable the terminal app you're running from (e.g., Terminal.app, iTerm2)
//! 4. You may need to restart your terminal after granting permission
//!
//! ## Usage
//!
//!   cargo run -p zed --bin visual_test_runner --features visual-tests
//!
//! ## Environment variables
//!
//!   VISUAL_TEST_OUTPUT_DIR - Directory to save screenshots (default: target/visual_tests)

use anyhow::Result;
use gpui::{
    AppContext as _, Application, Bounds, Window, WindowBounds, WindowHandle, WindowOptions, point,
    px, size,
};
use settings::SettingsStore;
use std::path::Path;
use std::sync::Arc;
use workspace::{AppState, Workspace};

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    println!("=== Visual Test Runner ===\n");

    // Create a temporary directory for test files
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let project_path = temp_dir.path().join("project");
    std::fs::create_dir_all(&project_path).expect("Failed to create project directory");

    // Create test files in the real filesystem
    println!("Setting up test project at: {:?}", project_path);
    create_test_files(&project_path);

    let project_path_clone = project_path.clone();

    Application::new().run(move |cx| {
        println!("Initializing Zed...");

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
        project_panel::init(cx);
        outline_panel::init(cx);
        terminal_view::init(cx);
        image_viewer::init(cx);
        search::init(cx);

        println!("Opening Zed workspace...");

        // Open a real Zed workspace window
        let window_size = size(px(1280.0), px(800.0));
        let bounds = Bounds {
            origin: point(px(100.0), px(100.0)),
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
                    focus: true,
                    show: true,
                    ..Default::default()
                },
                |window, cx| {
                    cx.new(|cx| {
                        Workspace::new(None, project.clone(), app_state.clone(), window, cx)
                    })
                },
            )
            .expect("Failed to open workspace window");

        println!("Workspace window opened, adding project folder...");

        // Add the test project as a worktree
        let add_folder_task = workspace_window
            .update(cx, |workspace, window, cx| {
                workspace.open_paths(
                    vec![project_path_clone.clone()],
                    workspace::OpenOptions::default(),
                    None,
                    window,
                    cx,
                )
            })
            .expect("Failed to update workspace");

        // Spawn async task to wait for project to load, then capture screenshot
        cx.spawn(async move |mut cx| {
            // Wait for the folder to be added
            println!("Waiting for project to load...");
            add_folder_task.await;

            // Wait for the UI to fully render
            println!("Waiting for UI to stabilize...");
            cx.background_executor()
                .timer(std::time::Duration::from_secs(2))
                .await;

            println!("Capturing screenshot...");

            // Try multiple times in case the first attempt fails
            let mut result = Err(anyhow::anyhow!("No capture attempts"));
            for attempt in 1..=3 {
                println!("Capture attempt {}...", attempt);
                result = capture_screenshot(workspace_window.into(), &mut cx).await;
                if result.is_ok() {
                    break;
                }
                if attempt < 3 {
                    println!("Attempt {} failed, retrying...", attempt);
                    cx.background_executor()
                        .timer(std::time::Duration::from_millis(500))
                        .await;
                }
            }

            match result {
                Ok(path) => {
                    println!("\n=== Visual Test PASSED ===");
                    println!("Screenshot saved to: {}", path);
                }
                Err(e) => {
                    eprintln!("\n=== Visual Test FAILED ===");
                    eprintln!("Error: {}", e);
                    eprintln!();
                    eprintln!("If you see 'Screen Recording permission' errors:");
                    eprintln!("  1. Open System Settings > Privacy & Security > Screen Recording");
                    eprintln!("  2. Enable your terminal app (Terminal.app, iTerm2, etc.)");
                    eprintln!("  3. Restart your terminal and try again");
                }
            }

            cx.update(|cx| cx.quit()).ok();
        })
        .detach();
    });

    // Keep temp_dir alive until we're done - it will be dropped here
    drop(temp_dir);
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
/// This creates a minimal AppState without FakeFs to avoid test dispatcher issues.
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

async fn capture_screenshot(
    window: gpui::AnyWindowHandle,
    cx: &mut gpui::AsyncApp,
) -> Result<String> {
    // Get the native window ID
    let window_id = cx
        .update(|cx| {
            cx.update_window(window, |_view, window: &mut Window, _cx| {
                window.native_window_id()
            })
        })??
        .ok_or_else(|| anyhow::anyhow!("Failed to get native window ID"))?;

    println!("Window ID: {}", window_id);

    // Capture the screenshot
    let screenshot = gpui::capture_window_screenshot(window_id)
        .await
        .map_err(|_| anyhow::anyhow!("Screenshot capture was cancelled"))??;

    println!(
        "Screenshot captured: {}x{} pixels",
        screenshot.width(),
        screenshot.height()
    );

    // Determine output path
    let output_dir =
        std::env::var("VISUAL_TEST_OUTPUT_DIR").unwrap_or_else(|_| "target/visual_tests".into());
    let output_path = Path::new(&output_dir).join("zed_workspace.png");

    // Create output directory
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Save the screenshot
    screenshot.save(&output_path)?;

    // Return absolute path
    let abs_path = output_path
        .canonicalize()
        .unwrap_or_else(|_| output_path.clone());
    Ok(abs_path.display().to_string())
}
