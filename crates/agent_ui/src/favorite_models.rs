use std::sync::Arc;

use fs::Fs;
use language_model::LanguageModel;
use settings::{LanguageModelSelection, update_settings_file};
use ui::App;

pub trait IntoLanguageModelSelection {
    fn into_language_model_selection(self) -> LanguageModelSelection;
}

impl IntoLanguageModelSelection for Arc<dyn LanguageModel> {
    fn into_language_model_selection(self) -> LanguageModelSelection {
        LanguageModelSelection {
            provider: self.provider_id().to_string().into(),
            model: self.id().0.to_string(),
        }
    }
}

pub fn add_to_settings(
    model: impl IntoLanguageModelSelection + Send + 'static,
    fs: Arc<dyn Fs>,
    cx: &App,
) {
    update_settings_file(fs, cx, |settings, _| {
        settings
            .agent
            .get_or_insert_default()
            .add_favorite_model(model.into_language_model_selection())
    });
}

pub fn remove_from_settings(
    model: impl IntoLanguageModelSelection + Send + 'static,
    fs: Arc<dyn Fs>,
    cx: &App,
) {
    update_settings_file(fs, cx, |settings, _| {
        if let Some(ref mut agent_settings) = settings.agent {
            agent_settings.remove_favorite_model(&model.into_language_model_selection())
        }
    });
}
