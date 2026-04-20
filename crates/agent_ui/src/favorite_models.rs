use std::sync::Arc;

use agent_settings::favorite_selection_for_model;
use fs::Fs;
use language_model::LanguageModel;
use settings::update_settings_file;
use ui::App;

pub fn toggle_in_settings(
    model: Arc<dyn LanguageModel>,
    should_be_favorite: bool,
    fs: Arc<dyn Fs>,
    cx: &mut App,
) {
    let selection = favorite_selection_for_model(&model, cx);
    update_settings_file(fs, cx, move |settings, _| {
        let agent = settings.agent.get_or_insert_default();
        if should_be_favorite {
            agent.add_favorite_model(selection.clone());
        } else {
            agent.remove_favorite_model(&selection);
        }
    });
}
