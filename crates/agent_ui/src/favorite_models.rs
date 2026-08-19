use std::sync::Arc;

use agent_settings::{AgentSettings, language_model_to_selection};
use fs::Fs;
use language_model::LanguageModel;
use settings::{Settings as _, update_settings_file};
use ui::App;

pub fn toggle_in_settings(
    model: Arc<dyn LanguageModel>,
    should_be_favorite: bool,
    fs: Arc<dyn Fs>,
    cx: &mut App,
) {
    let current_user_selection = AgentSettings::get_global(cx)
        .default_model
        .as_ref()
        .filter(|selection| {
            selection.provider.0 == model.provider_id().0.as_ref()
                && selection.model == model.id().0.as_ref()
        })
        .cloned();

    let selection = language_model_to_selection(&model, current_user_selection.as_ref());
    update_settings_file(fs, cx, move |settings, _| {
        let agent = settings.agent.get_or_insert_default();
        if should_be_favorite {
            agent.add_favorite_model(selection.clone());
        } else {
            agent.remove_favorite_model(&selection);
        }
    });
}
