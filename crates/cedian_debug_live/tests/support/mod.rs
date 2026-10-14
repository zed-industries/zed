//! A real project and workspace for the live tests, built without any
//! `test-support` feature that would make Zed's debug transport fake.
#![allow(dead_code, reason = "each test binary uses part of it")]

use gpui::{AppContext as _, Entity, TestAppContext, WindowHandle};
use project::Project;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use workspace::{AppState, Workspace, WorkspaceStore};

pub const MAIN_C: &str = "#include <stdio.h>\n\
int main(void) {\n\
    int x = 42;\n\
    printf(\"%d\\n\", x);\n\
    return 0;\n\
}\n";
/// The `printf` line, where `x` is 42.
pub const BREAK_LINE: u32 = 4;

/// The globals the panels need.
pub fn init(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let store = settings::SettingsStore::test(cx);
        cx.set_global(store);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        editor::init(cx);
        command_palette_hooks::init(cx);
        debugger_ui::init(cx);
    });
}

/// A folder `ws` under `root` with `main.c` built by `clang -g`, and a
/// debug.json scenario `main` that launches it under `lldb-dap`.
pub fn c_workspace(root: &Path) -> PathBuf {
    let ws = root.join("ws");
    std::fs::create_dir_all(ws.join(".zed")).unwrap();
    std::fs::write(ws.join("main.c"), MAIN_C).unwrap();
    #[expect(clippy::disallowed_methods, reason = "before the app starts")]
    let built = std::process::Command::new("clang")
        .args(["-g", "-O0", "-o"])
        .arg(ws.join("main"))
        .arg(ws.join("main.c"))
        .status()
        .expect("clang");
    assert!(built.success(), "clang -g main.c");
    let scenario = serde_json::json!([{
        "label": "main",
        "adapter": cedian_debug_live::ADAPTER,
        "request": "launch",
        "program": ws.join("main"),
    }]);
    std::fs::write(ws.join(".zed/debug.json"), scenario.to_string()).unwrap();
    ws.canonicalize().unwrap()
}

/// A local project on `ws`, as the app opens one.
pub async fn project(ws: &Path, cx: &mut TestAppContext) -> Entity<Project> {
    let languages = Arc::new(language::LanguageRegistry::new(cx.executor()));
    let http = http_client::FakeHttpClient::with_404_response();
    let client =
        cx.update(|cx| client::Client::new(Arc::new(clock::RealSystemClock), http.clone(), cx));
    let user_store = cx.new(|cx| client::UserStore::new(client.clone(), cx));
    let fs = fs::RealFs::new(None, cx.executor());
    let project = cx.update(|cx| {
        Project::local(
            client,
            node_runtime::NodeRuntime::unavailable(),
            user_store,
            languages,
            fs,
            None,
            Default::default(),
            cx,
        )
    });
    let (tree, _) = project
        .update(cx, |p, cx| p.find_or_create_worktree(ws, true, cx))
        .await
        .unwrap();
    tree.read_with(cx, |tree, _| tree.as_local().unwrap().scan_complete())
        .await;
    project
}

/// A workspace window on `project`, as `Workspace::test_new` makes one.
pub fn workspace(project: &Entity<Project>, cx: &mut TestAppContext) -> WindowHandle<Workspace> {
    cx.add_window(|window, cx| {
        let client = project.read(cx).client();
        let app_state = Arc::new(AppState {
            languages: project.read(cx).languages().clone(),
            workspace_store: cx.new(|cx| WorkspaceStore::new(client.clone(), cx)),
            client,
            user_store: project.read(cx).user_store(),
            fs: project.read(cx).fs().clone(),
            build_window_options: |_, _| Default::default(),
            node_runtime: node_runtime::NodeRuntime::unavailable(),
            session: cx.new(|cx| session::AppSession::new(session::Session::test(), cx)),
        });
        Workspace::new(Default::default(), project.clone(), app_state, window, cx)
    })
}

/// Scratch outside /tmp (a restart wipes it), one dir per run.
pub fn scratch(name: &str) -> PathBuf {
    let home = std::env::var_os("HOME").map(PathBuf::from).unwrap();
    let root = home
        .join(".cache/cedian-s10/S1-dap")
        .join(format!("{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    root.canonicalize().unwrap()
}
