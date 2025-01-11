use gpui::{Model, TestAppContext, View, WindowHandle};
use project::Project;
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
    project: &Model<Project>,
    cx: &mut TestAppContext,
) -> WindowHandle<Workspace> {
    let window = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));

    let debugger_panel = window
        .update(cx, |_, cx| cx.spawn(DebugPanel::load))
        .unwrap()
        .await
        .expect("Failed to load debug panel");

    let terminal_panel = window
        .update(cx, |_, cx| cx.spawn(TerminalPanel::load))
        .unwrap()
        .await
        .expect("Failed to load terminal panel");

    window
        .update(cx, |workspace, cx| {
            workspace.add_panel(debugger_panel, cx);
            workspace.add_panel(terminal_panel, cx);
        })
        .unwrap();
    window
}

pub fn active_debug_panel_item(
    workspace: WindowHandle<Workspace>,
    cx: &mut TestAppContext,
) -> View<DebugPanelItem> {
    workspace
        .update(cx, |workspace, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            debug_panel
                .update(cx, |this, cx| this.active_debug_panel_item(cx))
                .unwrap()
        })
        .unwrap()
}
