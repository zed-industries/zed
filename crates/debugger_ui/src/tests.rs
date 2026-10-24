use std::sync::Arc;
use std::sync::Mutex;

use anyhow::{Context as _, Result};
use dap::adapters::DebugTaskDefinition;
use dap::client::DebugAdapterClient;
use gpui::{Entity, TestAppContext, WindowHandle};
use project::{Project, debugger::session::Session};
use settings::SettingsStore;
use task::SharedTaskContext;
use terminal_view::terminal_panel::TerminalPanel;
use workspace::MultiWorkspace;

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
mod hover_values;
#[cfg(test)]
mod inline_values;
#[cfg(test)]
mod module_list;
#[cfg(test)]
mod new_process_modal;
#[cfg(test)]
mod persistence;
#[cfg(test)]
mod stack_frame_list;
#[cfg(test)]
mod variable_list;

pub fn init_test(cx: &mut gpui::TestAppContext) {
    #[cfg(test)]
    zlog::init_test();

    cx.update(|cx| {
        let settings = SettingsStore::test(cx);
        cx.set_global(settings);
        terminal_view::init(cx);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        command_palette_hooks::init(cx);
        editor::init(cx);
        crate::init(cx);
        dap_adapters::init(cx);
    });
}

pub async fn init_test_workspace(
    project: &Entity<Project>,
    cx: &mut TestAppContext,
) -> WindowHandle<MultiWorkspace> {
    let workspace_handle =
        cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));

    let debugger_panel = workspace_handle
        .update(cx, |multi, window, cx| {
            multi.workspace().update(cx, |_workspace, cx| {
                cx.spawn_in(window, async move |this, cx| {
                    DebugPanel::load(this, cx).await
                })
            })
        })
        .unwrap()
        .await
        .expect("Failed to load debug panel");

    let terminal_panel = workspace_handle
        .update(cx, |multi, window, cx| {
            let weak_workspace = multi.workspace().downgrade();
            cx.spawn_in(window, async move |_, cx| {
                TerminalPanel::load(weak_workspace, cx.clone()).await
            })
        })
        .unwrap()
        .await
        .expect("Failed to load terminal panel");

    workspace_handle
        .update(cx, |multi, window, cx| {
            multi.workspace().update(cx, |workspace, cx| {
                workspace.add_panel(debugger_panel, window, cx);
                workspace.add_panel(terminal_panel, window, cx);
            });
        })
        .unwrap();
    workspace_handle
}

#[track_caller]
pub fn active_debug_session_panel(
    workspace: WindowHandle<MultiWorkspace>,
    cx: &mut TestAppContext,
) -> Entity<DebugSession> {
    workspace
        .update(cx, |multi, _window, cx| {
            multi.workspace().update(cx, |workspace, cx| {
                let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
                debug_panel
                    .update(cx, |this, _| this.active_session())
                    .unwrap()
            })
        })
        .unwrap()
}

pub fn start_debug_session_with<T: Fn(&Arc<DebugAdapterClient>) + 'static>(
    workspace: &WindowHandle<MultiWorkspace>,
    cx: &mut gpui::TestAppContext,
    config: DebugTaskDefinition,
    configure: T,
) -> Result<Entity<Session>> {
    let _subscription = project::debugger::test::intercept_debug_sessions(cx, configure);
    workspace.update(cx, |multi, window, cx| {
        multi.workspace().update(cx, |workspace, cx| {
            workspace.start_debug_session(
                config.to_scenario(),
                SharedTaskContext::default(),
                None,
                None,
                window,
                cx,
            )
        })
    })?;
    cx.run_until_parked();
    let session = workspace.read_with(cx, |workspace, cx| {
        workspace
            .workspace()
            .read(cx)
            .panel::<DebugPanel>(cx)
            .and_then(|panel| {
                panel
                    .read(cx)
                    .sessions_with_children
                    .keys()
                    .max_by_key(|session| session.read(cx).session_id(cx))
            })
            .map(|session| session.read(cx).running_state().read(cx).session())
            .cloned()
            .context("Failed to get active session")
    })??;

    Ok(session)
}

pub fn start_debug_session<T: Fn(&Arc<DebugAdapterClient>) + 'static>(
    workspace: &WindowHandle<MultiWorkspace>,
    cx: &mut gpui::TestAppContext,
    configure: T,
) -> Result<Entity<Session>> {
    use serde_json::json;

    start_debug_session_with(
        workspace,
        cx,
        DebugTaskDefinition {
            adapter: "fake-adapter".into(),
            label: "test".into(),
            config: json!({
                "request": "launch"
            }),
            tcp_connection: None,
        },
        configure,
    )
}

/// The inputs a [`dap::DapLocator`] receives when asked to resolve a debug
/// scenario, captured by [`FakeLocator`] so tests can assert on them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapturedScenarioRequest {
    pub task_template: task::TaskTemplate,
    pub resolved_label: String,
    pub adapter: dap::adapters::DebugAdapterName,
}

/// A [`dap::DapLocator`] used in tests that captures the arguments passed to
/// `create_scenario` instead of resolving a debug scenario.
pub struct FakeLocator {
    pub captured: Arc<Mutex<Option<CapturedScenarioRequest>>>,
}

#[async_trait::async_trait]
impl dap::DapLocator for FakeLocator {
    fn name(&self) -> gpui::SharedString {
        gpui::SharedString::new_static("fake-locator")
    }

    async fn create_scenario(
        &self,
        build_config: &task::TaskTemplate,
        resolved_label: &str,
        adapter: &dap::adapters::DebugAdapterName,
    ) -> Option<task::DebugScenario> {
        *self.captured.lock().unwrap() = Some(CapturedScenarioRequest {
            task_template: build_config.clone(),
            resolved_label: resolved_label.to_owned(),
            adapter: adapter.clone(),
        });
        None
    }

    async fn run(
        &self,
        _build_config: task::SpawnInTerminal,
        _executor: gpui::BackgroundExecutor,
    ) -> anyhow::Result<task::DebugRequest> {
        anyhow::bail!("fake locator does not run tasks")
    }
}
