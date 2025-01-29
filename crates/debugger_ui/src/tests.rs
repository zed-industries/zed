use gpui::{Entity, TestAppContext, WindowHandle};
use project::{Project, Worktree};
use settings::SettingsStore;
use terminal_view::terminal_panel::TerminalPanel;
use workspace::Workspace;

use crate::{debugger_panel::DebugPanel, debugger_panel_item::DebugPanelItem};

mod attach_modal;
mod console;
mod debugger_panel;
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
        crate::init(cx);
        editor::init(cx);
    });
}

pub async fn init_test_workspace(
    project: &Entity<Project>,
    cx: &mut TestAppContext,
) -> WindowHandle<Workspace> {
    let workspace_handle =
        cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));

    let debugger_panel = workspace_handle
        .update(cx, |_, window, cx| cx.spawn_in(window, DebugPanel::load))
        .unwrap()
        .await
        .expect("Failed to load debug panel");

    let terminal_panel = workspace_handle
        .update(cx, |_, window, cx| cx.spawn_in(window, TerminalPanel::load))
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

pub fn active_debug_panel_item(
    workspace: WindowHandle<Workspace>,
    cx: &mut TestAppContext,
) -> Entity<DebugPanelItem> {
    workspace
        .update(cx, |workspace, _window, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            debug_panel
                .update(cx, |this, cx| this.active_debug_panel_item(cx))
                .unwrap()
        })
        .unwrap()
}

pub fn worktree_from_project(
    project: &Entity<Project>,
    cx: &mut TestAppContext,
) -> Entity<Worktree> {
    project.read_with(cx, |project, cx| project.worktrees(cx).next().unwrap())
}
