mod modal;
mod toolbar;

use configuration::ConfigurationTemplate;
use gpui::{actions, App, Global, UpdateGlobal};
pub use modal::ConfigurationsModal;
use project::ConfigurationSourceKind;
use std::sync::Arc;
pub use toolbar::ConfigurationsToolbar;
use workspace::Workspace;

actions!(
    configurations,
    [
        RunConfiguration,
        DebugConfiguration,
        SelectConfiguration
    ]
);

/// Global state for the currently selected configuration
#[derive(Default, Clone)]
pub struct SelectedConfiguration {
    pub selection: Option<(ConfigurationSourceKind, Arc<ConfigurationTemplate>)>,
}

impl Global for SelectedConfiguration {}

pub fn init(cx: &mut App) {
    cx.set_global(SelectedConfiguration::default());
    
    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        workspace.register_action(|workspace, _: &RunConfiguration, window, cx| {
            run_active_configuration(workspace, window, cx);
        });
        workspace.register_action(|workspace, _: &DebugConfiguration, window, cx| {
            debug_active_configuration(workspace, window, cx);
        });
        workspace.register_action(|workspace, _: &SelectConfiguration, window, cx| {
            select_configuration(workspace, window, cx);
        });
    })
    .detach();
}

fn run_active_configuration(
    _workspace: &mut Workspace,
    _window: &mut gpui::Window,
    _cx: &mut gpui::Context<Workspace>,
) {
    // TODO: Implement run logic
}

fn debug_active_configuration(
    _workspace: &mut Workspace,
    _window: &mut gpui::Window,
    _cx: &mut gpui::Context<Workspace>,
) {
    // TODO: Implement debug logic
}

pub fn set_selected_configuration(
    source: ConfigurationSourceKind,
    template: ConfigurationTemplate,
    cx: &mut App,
) {
    log::info!("Configuration selected: '{}'", template.label);
    SelectedConfiguration::update_global(cx, |state, _cx| {
        state.selection = Some((source, Arc::new(template)));
    });
}

pub fn select_configuration(
    workspace: &mut Workspace,
    window: &mut gpui::Window,
    cx: &mut gpui::Context<Workspace>,
) {
    let configuration_store = workspace.project().read(cx).configuration_store().cloned();
    let Some(configuration_store) = configuration_store else {
        return;
    };
    let workspace_handle = workspace.weak_handle();
    
    workspace.toggle_modal(window, cx, |window, cx| {
        ConfigurationsModal::new(
            configuration_store,
            workspace_handle,
            window,
            cx,
        )
    });
}
