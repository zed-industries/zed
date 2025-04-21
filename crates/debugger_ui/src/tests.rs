use gpui::{Entity, TestAppContext, WindowHandle};
use project::Project;
use settings::SettingsStore;
use terminal_view::terminal_panel::TerminalPanel;
use workspace::Workspace;

use crate::{debugger_panel::DebugPanel, session::DebugSession};

mod attach_modal;
mod console;
mod debugger_panel;
mod module_list;
mod stack_frame_list;
mod variable_list;

pub fn init_test(cx: &mut gpui::TestAppContext) {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::try_init().ok();
    }

    cx.update(|cx| {
        let settings = SettingsStore::test(cx);
        cx.set_global(settings);
        terminal_view::init(cx);
        theme::init(theme::LoadThemes::JustBase, cx);
        command_palette_hooks::init(cx);
        language::init(cx);
        workspace::init_settings(cx);
        Project::init_settings(cx);
        editor::init(cx);
        crate::init(cx);
    });
}

pub async fn init_test_workspace(
    project: &Entity<Project>,
    cx: &mut TestAppContext,
) -> WindowHandle<Workspace> {
    let workspace_handle =
        cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));

    let debugger_panel = workspace_handle
        .update(cx, |_, window, cx| {
            cx.spawn_in(window, async move |this, cx| {
                DebugPanel::load(this, cx.clone()).await
            })
        })
        .unwrap()
        .await
        .expect("Failed to load debug panel");

    let terminal_panel = workspace_handle
        .update(cx, |_, window, cx| {
            cx.spawn_in(window, async |this, cx| {
                TerminalPanel::load(this, cx.clone()).await
            })
        })
        .unwrap()
        .await
        .expect("Failed to load terminal panel");

    workspace_handle
        .update(cx, |workspace, window, cx| {
            workspace.add_panel(debugger_panel, window, cx);
            workspace.add_panel(terminal_panel, window, cx);
        })
        .unwrap();
    workspace_handle
}

#[track_caller]
pub fn active_debug_session_panel(
    workspace: WindowHandle<Workspace>,
    cx: &mut TestAppContext,
) -> Entity<DebugSession> {
    workspace
        .update(cx, |workspace, _window, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            debug_panel
                .update(cx, |this, _| this.active_session())
                .unwrap()
        })
        .unwrap()
}
