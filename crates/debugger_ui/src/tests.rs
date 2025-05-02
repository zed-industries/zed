use std::sync::Arc;

use anyhow::{Result, anyhow};
use dap::adapters::DebugTaskDefinition;
use dap::{DebugRequest, client::DebugAdapterClient};
use gpui::{Entity, TestAppContext, WindowHandle};
use project::{Project, debugger::session::Session};
use settings::SettingsStore;
use task::TaskContext;
use terminal_view::terminal_panel::TerminalPanel;
use workspace::Workspace;

use crate::{debugger_panel::DebugPanel, session::DebugSession};

#[cfg(test)]
mod attach_modal;
#[cfg(test)]
mod console;
#[cfg(test)]
mod dap_logger;
#[cfg(test)]
mod debugger_panel;
#[cfg(test)]
mod inline_values;
#[cfg(test)]
mod module_list;
#[cfg(test)]
mod stack_frame_list;
#[cfg(test)]
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
                DebugPanel::load(this, cx).await
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

pub fn start_debug_session_with<T: Fn(&Arc<DebugAdapterClient>) + 'static>(
    workspace: &WindowHandle<Workspace>,
    cx: &mut gpui::TestAppContext,
    config: DebugTaskDefinition,
    configure: T,
) -> Result<Entity<Session>> {
    let _subscription = project::debugger::test::intercept_debug_sessions(cx, configure);
    workspace.update(cx, |workspace, window, cx| {
        workspace.start_debug_session(
            config.to_scenario(),
            TaskContext::default(),
            None,
            window,
            cx,
        )
    })?;
    cx.run_until_parked();
    let session = workspace.read_with(cx, |workspace, cx| {
        workspace
            .panel::<DebugPanel>(cx)
            .and_then(|panel| panel.read(cx).active_session())
            .map(|session| session.read(cx).running_state().read(cx).session())
            .cloned()
            .ok_or_else(|| anyhow!("Failed to get active session"))
    })??;

    Ok(session)
}

pub fn start_debug_session<T: Fn(&Arc<DebugAdapterClient>) + 'static>(
    workspace: &WindowHandle<Workspace>,
    cx: &mut gpui::TestAppContext,
    configure: T,
) -> Result<Entity<Session>> {
    start_debug_session_with(
        workspace,
        cx,
        DebugTaskDefinition {
            adapter: "fake-adapter".into(),
            request: DebugRequest::Launch(Default::default()),
            label: "test".into(),
            initialize_args: None,
            tcp_connection: None,
            stop_on_entry: None,
        },
        configure,
    )
}
