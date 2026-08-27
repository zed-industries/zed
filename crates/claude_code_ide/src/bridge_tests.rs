//! Product-path tests: drive the bridge against a real (test) `Workspace` and
//! assert both the discovery lockfile and the real editor→selection payload
//! path, which the transport tests (no workspace) never exercise.

use crate::bridge;
use crate::selection::SharedSelection;
use async_tungstenite::tungstenite::Message;
use futures::AsyncWriteExt as _;
use gpui::{AppContext as _, TestAppContext};
use project::{FakeFs, Project};
use serde_json::json;
use workspace::{AppState, Workspace};

fn init_test(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let settings_store = settings::SettingsStore::test(cx);
        cx.set_global(settings_store);
        cx.set_global(db::AppDatabase::test_new());
        theme_settings::init(theme::LoadThemes::JustBase, cx);
    });
}

/// Heavier init that brings up editor + workspace so a real `Editor` can be
/// opened and driven (needed for the selection-path test).
fn init_test_with_editor(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let app_state = AppState::test(cx);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        editor::init(cx);
        workspace::init(app_state, cx);
    });
}

#[gpui::test]
async fn start_for_workspace_writes_lockfile(cx: &mut TestAppContext) {
    cx.executor().allow_parking();
    init_test(cx);

    // Point the lockfile dir at a throwaway location so we don't touch the real
    // ~/.claude/ide. SAFETY: single-threaded test setup.
    let lock_dir = tempfile::tempdir().expect("temp lock dir");
    unsafe {
        std::env::set_var("CLAUDE_CONFIG_DIR", lock_dir.path());
    }

    let fs = FakeFs::new(cx.executor());
    let project = Project::test(fs, [], cx).await;
    let window =
        cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));

    // Start the bridge for this workspace.
    window
        .update(cx, |workspace, window, cx| {
            bridge::start_for_workspace(workspace, window.window_handle(), cx);
        })
        .expect("update workspace");

    // Let the async server bind and the lockfile get written.
    cx.run_until_parked();

    // Exactly one lockfile should exist under CLAUDE_CONFIG_DIR/ide.
    let ide_dir = lock_dir.path().join("ide");
    let entries: Vec<_> = std::fs::read_dir(&ide_dir)
        .expect("ide dir exists")
        .filter_map(Result::ok)
        .filter(|entry| {
            entry
                .path()
                .extension()
                .is_some_and(|ext| ext == "lock")
        })
        .collect();
    assert_eq!(
        entries.len(),
        1,
        "expected exactly one lockfile, found {entries:?}"
    );

    let lock_path = entries[0].path();
    let port_from_name: u16 = lock_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .and_then(|stem| stem.parse().ok())
        .expect("lockfile name is <port>.lock");

    let json: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&lock_path).expect("read lockfile"))
            .expect("lockfile is valid JSON");
    assert_eq!(json["ideName"], "Zed");
    assert_eq!(json["transport"], "ws");
    assert!(json["pid"].is_number());
    let token = json["authToken"].as_str().expect("authToken is a string");
    assert_eq!(token.len(), 32, "token should be 32 hex chars");
    assert!(
        token.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()),
        "token should be lowercase hex"
    );
    // workspaceFolders is present (empty for a project with no worktrees).
    assert!(json["workspaceFolders"].is_array());

    // The port in the filename is a real bound port (non-zero).
    assert!(port_from_name > 0, "port should be a real bound port");

    unsafe {
        std::env::remove_var("CLAUDE_CONFIG_DIR");
    }
}

#[gpui::test]
async fn selection_capture_reads_real_editor(cx: &mut TestAppContext) {
    init_test_with_editor(cx);

    // A project with one file whose line 2 (0-based) is a known string.
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/project",
        json!({ "main.rs": "fn main() {}\nlet x = 1;\nlet answer = 42;\n" }),
    )
    .await;
    let project = Project::test(fs, ["/project".as_ref()], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    // Open the file as a real editor in the active pane.
    let buffer = project
        .update(cx, |project, cx| {
            let path = project
                .find_project_path("main.rs", cx)
                .expect("find main.rs");
            project.open_buffer(path, cx)
        })
        .await
        .expect("open buffer");
    let editor = workspace.update_in(cx, |workspace, window, cx| {
        let editor =
            cx.new(|cx| editor::Editor::for_buffer(buffer.clone(), Some(project.clone()), window, cx));
        workspace.add_item_to_active_pane(Box::new(editor.clone()), None, true, window, cx);
        editor
    });

    // Select "answer = 42" on line 2 (row 2, cols 4..15).
    editor.update_in(cx, |editor, window, cx| {
        editor.change_selections(editor::SelectionEffects::default(), window, cx, |selections| {
            selections.select_ranges([language::Point::new(2, 4)..language::Point::new(2, 15)]);
        });
    });

    // Capture the selection through the bridge's real payload path.
    let selection = SharedSelection::new();
    selection.set_workspace(workspace.downgrade());
    let payload = cx
        .update(|_window, cx| crate::selection::capture(&selection, &workspace, cx))
        .expect("capture should produce a selection payload");

    assert!(
        payload.file_path.ends_with("main.rs"),
        "file path should point at main.rs, got {}",
        payload.file_path
    );
    assert_eq!(payload.file_url, format!("file://{}", payload.file_path));
    assert_eq!(payload.selection.start.line, 2);
    assert_eq!(payload.selection.start.character, 4);
    assert_eq!(payload.selection.end.line, 2);
    assert_eq!(payload.selection.end.character, 15);
    assert_eq!(payload.text, "answer = 42");
}

#[gpui::test]
async fn selection_capture_concats_multiple_regions(cx: &mut TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/project",
        json!({ "main.rs": "fn main() {}\nlet x = 1;\nlet answer = 42;\n" }),
    )
    .await;
    let project = Project::test(fs, ["/project".as_ref()], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    let buffer = project
        .update(cx, |project, cx| {
            let path = project
                .find_project_path("main.rs", cx)
                .expect("find main.rs");
            project.open_buffer(path, cx)
        })
        .await
        .expect("open buffer");
    let editor = workspace.update_in(cx, |workspace, window, cx| {
        let editor =
            cx.new(|cx| editor::Editor::for_buffer(buffer.clone(), Some(project.clone()), window, cx));
        workspace.add_item_to_active_pane(Box::new(editor.clone()), None, true, window, cx);
        editor
    });

    // Two disjoint selections: "x = 1" on line 1 and "answer = 42" on line 2.
    // select_ranges makes the LAST range the newest => the primary region.
    editor.update_in(cx, |editor, window, cx| {
        editor.change_selections(editor::SelectionEffects::default(), window, cx, |selections| {
            selections.select_ranges([
                language::Point::new(1, 4)..language::Point::new(1, 9),
                language::Point::new(2, 4)..language::Point::new(2, 15),
            ]);
        });
    });

    let selection = SharedSelection::new();
    selection.set_workspace(workspace.downgrade());
    let payload = cx
        .update(|_window, cx| crate::selection::capture(&selection, &workspace, cx))
        .expect("capture should produce a selection payload");

    // Top-level selection/filePath stay TRUTHFUL to the primary (newest) region
    // so the CLI's single-region banner never lies.
    assert!(payload.file_path.ends_with("main.rs"));
    assert_eq!(payload.selection.start.line, 2);
    assert_eq!(payload.selection.start.character, 4);
    assert_eq!(payload.selection.end.line, 2);
    assert_eq!(payload.selection.end.character, 15);

    // text carries BOTH regions under `# <path>:<startLine>-<endLine>` headers,
    // in document order (1-based line numbers).
    assert!(
        payload.text.contains("x = 1"),
        "text should include the first region, got:\n{}",
        payload.text
    );
    assert!(
        payload.text.contains("answer = 42"),
        "text should include the primary region, got:\n{}",
        payload.text
    );
    assert!(
        payload.text.contains(":2-2\n"),
        "text should carry a 1-based header for line 2, got:\n{}",
        payload.text
    );
    assert!(
        payload.text.contains(":3-3\n"),
        "text should carry a 1-based header for line 3, got:\n{}",
        payload.text
    );
}

/// Start a bridge on a test workspace, discover it the way Claude Code does (read
/// the lockfile for port and token), and return the selection socket plus a
/// connected, handshaken client.
async fn bridge_with_client(
    cx: &mut TestAppContext,
    lock_dir: &std::path::Path,
) -> (std::path::PathBuf, crate::server_tests::ClientWs) {
    let fs = FakeFs::new(cx.executor());
    let project = Project::test(fs, [], cx).await;
    let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    window
        .update(cx, |workspace, window, cx| {
            bridge::start_for_workspace(workspace, window.window_handle(), cx);
        })
        .expect("update workspace");
    cx.run_until_parked();

    let entry = std::fs::read_dir(lock_dir.join("ide"))
        .expect("ide dir exists")
        .next()
        .expect("one lockfile was written")
        .expect("lockfile is readable");
    let port: u16 = entry
        .path()
        .file_stem()
        .and_then(|stem| stem.to_str())
        .and_then(|stem| stem.parse().ok())
        .expect("lockfile is named <port>.lock");
    let lock: serde_json::Value =
        serde_json::from_slice(&std::fs::read(entry.path()).expect("read lockfile"))
            .expect("lockfile holds JSON");
    let token = lock["authToken"].as_str().expect("authToken").to_string();

    let socket = crate::selection_socket::socket_path_for_port(port);
    assert!(socket.exists(), "bridge should have bound {socket:?}");

    // Handshake so the client's outgoing sender is registered before any push.
    let connect = cx.background_executor.spawn(async move {
        let mut ws = crate::server_tests::connect(port, Some(&token))
            .await
            .expect("client connects");
        ws.send(Message::text(
            r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#,
        ))
        .await
        .expect("send initialize");
        let _ = crate::server_tests::next_json(&mut ws).await;
        ws
    });
    cx.run_until_parked();
    (socket, connect.await)
}

/// Write one newline-terminated line to the selection socket, as a client does, and
/// let the bridge process it.
async fn push_line(cx: &mut TestAppContext, socket: &std::path::Path, line: &str) {
    let task = cx.background_executor.spawn({
        let socket = socket.to_path_buf();
        let line = format!("{line}\n");
        async move {
            let mut stream = net::async_net::UnixStream::connect(&socket)
                .await
                .expect("connect to the selection socket");
            stream.write_all(line.as_bytes()).await.expect("write line");
            stream.flush().await.ok();
        }
    });
    cx.run_until_parked();
    task.await;
    cx.run_until_parked();
}

/// Read the next frame, handing the client back so later phases reuse it.
async fn next_frame(
    cx: &mut TestAppContext,
    mut ws: crate::server_tests::ClientWs,
) -> (serde_json::Value, crate::server_tests::ClientWs) {
    let task = cx
        .background_executor
        .spawn(async move { (crate::server_tests::next_json(&mut ws).await, ws) });
    cx.run_until_parked();
    let (frame, ws) = task.await;
    log::info!("observed frame: {frame}");
    (frame, ws)
}

/// The whole external-selection chain, headless: a client payload written to the
/// socket travels parse -> aggregate -> `selection_changed` to a connected Claude
/// Code client. Then a second push under the same source must REPLACE the first
/// region rather than add one, and a clear must announce that nothing is selected.
/// The one test crossing every seam at once.
#[gpui::test]
async fn client_payload_over_the_socket_reaches_a_connected_client(cx: &mut TestAppContext) {
    cx.executor().allow_parking();
    init_test(cx);

    // Keep the real ~/.claude/ide untouched. SAFETY: single-threaded test setup.
    let lock_dir = tempfile::tempdir().expect("temp lock dir");
    unsafe {
        std::env::set_var("CLAUDE_CONFIG_DIR", lock_dir.path());
    }
    let (socket, ws) = bridge_with_client(cx, lock_dir.path()).await;

    // Byte-for-byte what zed-claude-selection.nvim builds, including the escaped
    // newline, the zero-based span and the `nvim:<pid>` source it states for itself.
    push_line(
        cx,
        &socket,
        concat!(
            r#"{"op":"push_selection","text":"line two\nline three","#,
            r#""filePath":"/repo/a.rs","fileUrl":"file:///repo/a.rs","#,
            r#""source":"nvim:4242","#,
            r#""selection":{"start":{"line":1,"character":0},"end":{"line":2,"character":0}}}"#
        ),
    )
    .await;
    let (frame, ws) = next_frame(cx, ws).await;
    assert_eq!(frame["method"], "selection_changed", "got {frame}");
    assert_eq!(frame["params"]["text"], "line two\nline three", "got {frame}");
    assert_eq!(frame["params"]["filePath"], "/repo/a.rs", "got {frame}");
    assert_eq!(frame["params"]["selection"]["start"]["line"], 1, "got {frame}");
    assert_eq!(frame["params"]["selection"]["end"]["line"], 2, "got {frame}");

    // Same source, different file: replaces rather than appends. Were the key
    // derived per file, this would be two regions with both files under headers.
    push_line(
        cx,
        &socket,
        concat!(
            r#"{"op":"push_selection","text":"other file","#,
            r#""filePath":"/repo/b.rs","source":"nvim:4242","#,
            r#""selection":{"start":{"line":0,"character":0},"end":{"line":1,"character":0}}}"#
        ),
    )
    .await;
    let (frame, ws) = next_frame(cx, ws).await;
    assert_eq!(frame["params"]["text"], "other file", "should replace, got {frame}");

    // With no regions left the bridge must say so, or the client keeps showing the
    // selection we just dropped.
    push_line(cx, &socket, r#"{"op":"clear_selection","source":"nvim:4242"}"#).await;
    let (frame, _ws) = next_frame(cx, ws).await;
    assert_eq!(frame["method"], "selection_changed", "got {frame}");
    assert!(frame["params"]["selection"].is_null(), "got {frame}");
    assert_eq!(frame["params"]["text"], "", "got {frame}");
}
