use std::sync::Arc;

use agent_client_protocol::ModelId;
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

impl IntoLanguageModelSelection for ModelId {
    fn into_language_model_selection(self) -> LanguageModelSelection {
        let model_id = self.0.as_ref();
        let (provider, model) = model_id.split_once('/').unwrap_or(("", model_id));

        LanguageModelSelection {
            provider: provider.to_owned().into(),
            model: model.to_owned(),
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
