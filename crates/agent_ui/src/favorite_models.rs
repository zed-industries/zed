use std::sync::Arc;

use agent_settings::AgentSettings;
use fs::Fs;
use language_model::LanguageModel;
use settings::{LanguageModelSelection, Settings as _, update_settings_file};
use ui::App;

fn language_model_to_selection(model: &Arc<dyn LanguageModel>, cx: &App) -> LanguageModelSelection {
    let provider_id = model.provider_id().0.to_string();
    let model_id = model.id().0.to_string();

    let user_current_model = AgentSettings::get_global(cx)
        .default_model
        .as_ref()
        .filter(|selection| selection.provider.0 == provider_id && selection.model == model_id);

    let (enable_thinking, effort, speed) = match user_current_model {
        Some(current) => (
            current.enable_thinking && model.supports_thinking(),
            current
                .effort
                .clone()
                .filter(|value| {
                    model
                        .supported_effort_levels()
                        .iter()
                        .any(|level| level.value.as_ref() == value.as_str())
                })
                .or_else(|| {
                    model
                        .default_effort_level()
                        .map(|effort| effort.value.to_string())
                }),
            current.speed.filter(|_| model.supports_fast_mode()),
        ),
        None => (
            model.supports_thinking(),
            model
                .default_effort_level()
                .map(|effort| effort.value.to_string()),
            None,
        ),
    };

    LanguageModelSelection {
        provider: provider_id.into(),
        model: model_id,
        enable_thinking,
        effort,
        speed,
    }
}

pub fn toggle_in_settings(
    model: Arc<dyn LanguageModel>,
    should_be_favorite: bool,
    fs: Arc<dyn Fs>,
    cx: &mut App,
) {
    let selection = language_model_to_selection(&model, cx);
    update_settings_file(fs, cx, move |settings, _| {
        let agent = settings.agent.get_or_insert_default();
        if should_be_favorite {
            agent.add_favorite_model(selection.clone());
        } else {
            agent.remove_favorite_model(&selection);
        }
    });
}
