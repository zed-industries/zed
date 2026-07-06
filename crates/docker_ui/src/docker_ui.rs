mod confirm_modal;
mod detail_view;
mod docker_panel;
mod docker_settings;
mod endpoint_modal;
mod endpoint_store;
mod inspect_tab;
mod logs_tab;

pub use confirm_modal::{ConfirmModal, ConfirmModalEvent};
pub use detail_view::{DetailView, SelectedItem};
pub use docker_panel::{DockerPanel, Toggle, ToggleFocus};
pub use docker_settings::DockerSettings;
pub use endpoint_modal::DockerEndpointModal;
pub use endpoint_store::*;
pub use inspect_tab::{DockerInspectView, open_inspect_tab};
pub use logs_tab::{DockerLogsView, open_logs_tab};

use gpui::App;
use workspace::Workspace;

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
            workspace.toggle_panel_focus::<DockerPanel>(window, cx);
        });
        workspace.register_action(|workspace, _: &Toggle, window, cx| {
            if !workspace.toggle_panel_focus::<DockerPanel>(window, cx) {
                workspace.close_panel::<DockerPanel>(window, cx);
            }
        });
    })
    .detach();
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use settings::Settings;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            crate::init(cx);
        });
    }

    #[gpui::test]
    fn docker_settings_resolve_from_defaults(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let settings = DockerSettings::get_global(cx);
            assert_eq!(settings.poll_interval_seconds, 300);
            assert!(settings.endpoints.is_empty());
        });
    }
}
