mod sign_in;

use std::sync::Arc;

use copilot::GlobalCopilotAuth;
use gpui::AppContext;
use language::language_settings::AllLanguageSettings;
use project::DisableAiSettings;
use settings::SettingsStore;
pub use sign_in::{
    ConfigurationMode, ConfigurationView, CopilotCodeVerification, initiate_sign_in,
    initiate_sign_out, reinstall_and_sign_in,
};
use ui::{App, Window};
use workspace::{AppState, Toast, Workspace, notifications::NotificationId};

struct CopilotStatsToast;

pub fn show_toast(window: &mut Window, cx: &mut App, message: &str) {
    let id = NotificationId::unique::<CopilotStatsToast>();

    if let Some(workspace) = window.root::<Workspace>().flatten() {
        workspace.update(cx, |workspace, cx| {
            workspace.show_toast(Toast::new(id, message.to_string()), cx);
        });
    }
}

pub fn init(app_state: &Arc<AppState>, cx: &mut App) {
    let disable_ai = cx.read_global(|settings: &SettingsStore, _| {
        settings.get::<DisableAiSettings>(None).disable_ai
    });
    let provider = cx.read_global(|settings: &SettingsStore, _| {
        settings
            .get::<AllLanguageSettings>(None)
            .edit_predictions
            .provider
    });
    if !disable_ai && provider == settings::EditPredictionProvider::Copilot {
        GlobalCopilotAuth::set_global(
            app_state.languages.next_language_server_id(),
            app_state.fs.clone(),
            app_state.node_runtime.clone(),
            cx,
        );
    }
}
