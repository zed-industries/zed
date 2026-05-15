// Tests for the `blocking_io_on_foreground` lint.

#![allow(unused)]

extern crate gpui;

use gpui::*;

struct Editor;

// ============================================================
// SHOULD WARN — blocking IO in functions with GPUI context params
// ============================================================

fn read_config_with_app(cx: &mut App) {
    let _ = std::fs::read_to_string("config.toml");
}

fn write_file_with_app(cx: &App) {
    let _ = std::fs::write("out.txt", b"data");
}

fn sleep_with_context(cx: &mut Context<'_, Editor>) {
    std::thread::sleep(std::time::Duration::from_millis(100));
}

fn read_with_window(window: &mut Window, cx: &mut App) {
    let _ = std::fs::read("data.bin");
}

fn metadata_with_app(cx: &mut App) {
    let _ = std::fs::metadata("file.txt");
}

fn create_dir_with_app(cx: &mut App) {
    let _ = std::fs::create_dir_all("some/path");
}

fn remove_file_with_app(cx: &mut App) {
    let _ = std::fs::remove_file("old.txt");
}

fn canonicalize_with_app(cx: &mut App) {
    let _ = std::fs::canonicalize("./relative");
}

// Path methods

fn path_metadata_with_app(cx: &mut App) {
    let _ = std::path::Path::new("file.txt").metadata();
}

fn path_symlink_metadata_with_app(cx: &mut App) {
    let _ = std::path::Path::new("link").symlink_metadata();
}

fn path_read_link_with_app(cx: &mut App) {
    let _ = std::path::Path::new("link").read_link();
}

fn path_read_dir_with_app(cx: &mut App) {
    let _ = std::path::Path::new("dir").read_dir();
}

fn path_exists_with_app(cx: &mut App) {
    let _ = std::path::Path::new("file.txt").exists();
}

fn path_canonicalize_with_app(cx: &mut App) {
    let _ = std::path::Path::new("./relative").canonicalize();
}

// PathBuf derefs to Path, so the same methods fire.
fn pathbuf_exists_with_app(cx: &mut App) {
    let _ = std::path::PathBuf::from("file.txt").exists();
}

struct BlockingRenderView;

impl Render for BlockingRenderView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let _ = std::fs::read_to_string("layout.toml");
        ()
    }
}

// ============================================================
// SHOULD NOT WARN — no GPUI context, or inside closure
// ============================================================

// Plain function with no GPUI parameter.
fn load_config_plain() -> String {
    std::fs::read_to_string("config.toml").unwrap_or_default()
}

// Blocking IO inside a closure (could be passed to background_spawn).
fn setup_with_closure(cx: &mut App) {
    let _handler = || {
        let _ = std::fs::read_to_string("config.toml");
    };
}

// Blocking IO in a function with no GPUI types at all.
fn standalone_sleep() {
    std::thread::sleep(std::time::Duration::from_millis(10));
}

// Path methods with no GPUI parameter.
fn path_exists_plain() -> bool {
    std::path::Path::new("file.txt").exists()
}

fn main() {}
