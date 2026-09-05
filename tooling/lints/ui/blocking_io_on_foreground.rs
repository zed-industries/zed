// Tests for the `blocking_io_on_foreground` lint.

#![allow(unused, let_underscore_lock)]

extern crate gpui;

use gpui::*;

struct Editor;

// ============================================================
// SHOULD WARN — blocking IO in functions with GPUI context params
// ============================================================

// --- std::fs free functions ---

fn read_config_with_app(cx: &mut App) {
    let _ = std::fs::read_to_string("config.toml");
}

fn write_file_with_app(cx: &App) {
    let _ = std::fs::write("out.txt", b"data");
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

fn read_dir_with_app(cx: &mut App) {
    let _ = std::fs::read_dir("some/dir");
}

fn read_link_with_app(cx: &mut App) {
    let _ = std::fs::read_link("some/link");
}

fn symlink_metadata_with_app(cx: &mut App) {
    let _ = std::fs::symlink_metadata("file.txt");
}

fn set_permissions_with_app(cx: &mut App) {
    if let Ok(meta) = std::fs::metadata("file.txt") {
        let _ = std::fs::set_permissions("file.txt", meta.permissions());
    }
}

fn copy_with_app(cx: &mut App) {
    let _ = std::fs::copy("a.txt", "b.txt");
}

fn rename_with_app(cx: &mut App) {
    let _ = std::fs::rename("old.txt", "new.txt");
}

fn hard_link_with_app(cx: &mut App) {
    let _ = std::fs::hard_link("original", "link");
}

fn create_dir_with_app_single(cx: &mut App) {
    let _ = std::fs::create_dir("one_dir");
}

fn remove_dir_with_app(cx: &mut App) {
    let _ = std::fs::remove_dir("empty_dir");
}

fn remove_dir_all_with_app(cx: &mut App) {
    let _ = std::fs::remove_dir_all("dir_tree");
}

// --- std::fs::File associated functions ---

fn file_open_with_app(cx: &mut App) {
    let _ = std::fs::File::open("data.bin");
}

fn file_create_with_app(cx: &mut App) {
    let _ = std::fs::File::create("out.bin");
}

fn file_create_new_with_app(cx: &mut App) {
    let _ = std::fs::File::create_new("new.bin");
}

// --- std::fs::File instance methods ---

fn file_sync_all_with_app(cx: &mut App) {
    if let Ok(f) = std::fs::File::open("x") {
        let _ = f.sync_all();
    }
}

fn file_sync_data_with_app(cx: &mut App) {
    if let Ok(f) = std::fs::File::open("x") {
        let _ = f.sync_data();
    }
}

fn file_set_len_with_app(cx: &mut App) {
    if let Ok(f) = std::fs::File::open("x") {
        let _ = f.set_len(0);
    }
}

fn file_metadata_with_app(cx: &mut App) {
    if let Ok(f) = std::fs::File::open("x") {
        let _ = f.metadata();
    }
}

fn file_try_clone_with_app(cx: &mut App) {
    if let Ok(f) = std::fs::File::open("x") {
        let _ = f.try_clone();
    }
}

// --- std::thread ---

fn sleep_with_context(cx: &mut Context<'_, Editor>) {
    std::thread::sleep(std::time::Duration::from_millis(100));
}

// --- std::path::Path methods ---

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

fn path_try_exists_with_app(cx: &mut App) {
    let _ = std::path::Path::new("file.txt").try_exists();
}

fn path_is_file_with_app(cx: &mut App) {
    let _ = std::path::Path::new("file.txt").is_file();
}

fn path_is_dir_with_app(cx: &mut App) {
    let _ = std::path::Path::new("dir").is_dir();
}

fn path_is_symlink_with_app(cx: &mut App) {
    let _ = std::path::Path::new("link").is_symlink();
}

fn path_canonicalize_with_app(cx: &mut App) {
    let _ = std::path::Path::new("./relative").canonicalize();
}

// PathBuf derefs to Path, so the same methods fire.
fn pathbuf_exists_with_app(cx: &mut App) {
    let _ = std::path::PathBuf::from("file.txt").exists();
}

// --- std::net ---

fn tcp_listener_bind_with_app(cx: &mut App) {
    let _ = std::net::TcpListener::bind("127.0.0.1:0");
}

fn tcp_stream_connect_with_app(cx: &mut App) {
    let _ = std::net::TcpStream::connect("127.0.0.1:80");
}

fn tcp_stream_connect_timeout_with_app(cx: &mut App) {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 80));
    let _ = std::net::TcpStream::connect_timeout(&addr, std::time::Duration::from_secs(1));
}

fn tcp_listener_accept_with_app(cx: &mut App) {
    if let Ok(listener) = std::net::TcpListener::bind("127.0.0.1:0") {
        let _ = listener.accept();
    }
}

fn udp_socket_bind_with_app(cx: &mut App) {
    let _ = std::net::UdpSocket::bind("127.0.0.1:0");
}

fn udp_socket_send_recv_with_app(cx: &mut App) {
    if let Ok(socket) = std::net::UdpSocket::bind("127.0.0.1:0") {
        let mut buf = [0u8; 64];
        let _ = socket.recv_from(&mut buf);
    }
}

// --- std::process ---

fn command_output_with_app(cx: &mut App) {
    let _ = std::process::Command::new("echo").output();
}

fn command_status_with_app(cx: &mut App) {
    let _ = std::process::Command::new("echo").status();
}

fn command_spawn_with_app(cx: &mut App) {
    let _ = std::process::Command::new("echo").spawn();
}

fn child_wait_with_app(cx: &mut App) {
    if let Ok(mut child) = std::process::Command::new("echo").spawn() {
        let _ = child.wait();
    }
}

fn child_wait_with_output_with_app(cx: &mut App) {
    if let Ok(child) = std::process::Command::new("echo").spawn() {
        let _ = child.wait_with_output();
    }
}

// --- std::sync ---

fn mutex_lock_with_app(cx: &mut App) {
    let m = std::sync::Mutex::new(42);
    let _ = m.lock();
}

fn rwlock_read_with_app(cx: &mut App) {
    let rw = std::sync::RwLock::new(42);
    let _ = rw.read();
}

fn rwlock_write_with_app(cx: &mut App) {
    let rw = std::sync::RwLock::new(42);
    let _ = rw.write();
}

fn barrier_wait_with_app(cx: &mut App) {
    let b = std::sync::Barrier::new(1);
    b.wait();
}

fn receiver_recv_with_app(cx: &mut App) {
    let (_tx, rx) = std::sync::mpsc::channel::<i32>();
    let _ = rx.recv();
}

fn sync_sender_send_with_app(cx: &mut App) {
    let (tx, _rx) = std::sync::mpsc::sync_channel::<i32>(1);
    let _ = tx.send(42);
}

// --- Render impl ---

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

// Net calls with no GPUI parameter.
fn tcp_bind_plain() {
    let _ = std::net::TcpListener::bind("127.0.0.1:0");
}

// Mutex lock with no GPUI parameter.
fn mutex_lock_plain() {
    let m = std::sync::Mutex::new(0);
    let _ = m.lock();
}

fn main() {}
