use std::sync::Arc;

use agent_client_protocol::ModelId;
use fs::Fs;
use language_model::LanguageModel;
use settings::{LanguageModelSelection, update_settings_file};
use ui::App;

fn language_model_to_selection(model: &Arc<dyn LanguageModel>) -> LanguageModelSelection {
    LanguageModelSelection {
        provider: model.provider_id().to_string().into(),
        model: model.id().0.to_string(),
    }
}

fn model_id_to_selection(model_id: &ModelId) -> LanguageModelSelection {
    let id = model_id.0.as_ref();
    let (provider, model) = id.split_once('/').unwrap_or(("", id));
    LanguageModelSelection {
        provider: provider.to_owned().into(),
        model: model.to_owned(),
    }
}

pub fn toggle_in_settings(
    model: Arc<dyn LanguageModel>,
    should_be_favorite: bool,
    fs: Arc<dyn Fs>,
    cx: &App,
) {
    let selection = language_model_to_selection(&model);
    update_settings_file(fs, cx, move |settings, _| {
        let agent = settings.agent.get_or_insert_default();
        if should_be_favorite {
            agent.add_favorite_model(selection.clone());
        } else {
            agent.remove_favorite_model(&selection);
        }
    });
}

pub fn toggle_model_id_in_settings(
    model_id: ModelId,
    should_be_favorite: bool,
    fs: Arc<dyn Fs>,
    cx: &App,
) {
    let selection = model_id_to_selection(&model_id);
    update_settings_file(fs, cx, move |settings, _| {
        let agent = settings.agent.get_or_insert_default();
        if should_be_favorite {
            agent.add_favorite_model(selection.clone());
        } else {
            agent.remove_favorite_model(&selection);
        }
    });
}
