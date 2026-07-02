mod connection_modal;
mod connection_store;
mod database_panel;
mod database_settings;
mod table_data_view;

pub use connection_modal::ConnectionModal;
pub use connection_store::*;
pub use database_panel::{DatabasePanel, Toggle, ToggleFocus};
pub use database_settings::DatabaseSettings;
pub use table_data_view::{TableDataView, open_table_tab};

use gpui::App;
use workspace::Workspace;

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
            workspace.toggle_panel_focus::<DatabasePanel>(window, cx);
        });
        workspace.register_action(|workspace, _: &Toggle, window, cx| {
            if !workspace.toggle_panel_focus::<DatabasePanel>(window, cx) {
                workspace.close_panel::<DatabasePanel>(window, cx);
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
    fn database_settings_resolve_from_defaults(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let settings = DatabaseSettings::get_global(cx);
            assert_eq!(settings.page_size, 100);
            assert_eq!(settings.query_timeout_seconds, 30);
            assert_eq!(settings.mcp_max_rows, 200);
            assert!(settings.connections.is_empty());
        });
    }
}
