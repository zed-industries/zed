use std::sync::Arc;

use anyhow::Result;
use copilot_chat::{
    CopilotChat, CopilotChatConfiguration, Model as CopilotChatModel, PROVIDER_ID, PROVIDER_NAME,
    create_language_model,
};
use gpui::{App, Entity, Subscription, Task};
use language::language_settings::all_language_settings;
use language_model::{
    AuthenticateError, IconOrSvg, LanguageModel, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, ProviderSettingsView,
};
use settings::SettingsStore;
use ui::prelude::*;

pub struct CopilotChatLanguageModelProvider {
    state: Entity<State>,
}

pub struct State {
    _copilot_chat_subscription: Option<Subscription>,
    _settings_subscription: Subscription,
}

impl State {
    fn is_authenticated(&self, cx: &App) -> bool {
        CopilotChat::global(cx)
            .map(|model| model.read(cx).is_authenticated())
            .unwrap_or(false)
    }
}

impl CopilotChatLanguageModelProvider {
    pub fn new(cx: &mut App) -> Self {
        let state = cx.new(|cx| {
            let copilot_chat_subscription = CopilotChat::global(cx)
                .map(|copilot_chat| cx.observe(&copilot_chat, |_, _, cx| cx.notify()));
            State {
                _copilot_chat_subscription: copilot_chat_subscription,
                _settings_subscription: cx.observe_global::<SettingsStore>(|_, cx| {
                    if let Some(copilot_chat) = CopilotChat::global(cx) {
                        let language_settings = all_language_settings(None, cx);
                        let configuration = CopilotChatConfiguration {
                            enterprise_uri: language_settings
                                .edit_predictions
                                .copilot
                                .enterprise_uri
                                .clone(),
                        };
                        copilot_chat.update(cx, |chat, cx| {
                            chat.set_configuration(configuration, cx);
                        });
                    }
                    cx.notify();
                }),
            }
        });

        Self { state }
    }

    fn create_language_model(
        &self,
        model: CopilotChatModel,
        copilot_chat: Entity<CopilotChat>,
    ) -> Arc<dyn LanguageModel> {
        create_language_model(model, copilot_chat)
    }
}

impl LanguageModelProviderState for CopilotChatLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for CopilotChatLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::Copilot)
    }

    fn default_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        let copilot_chat = CopilotChat::global(cx)?;
        let model = copilot_chat.read(cx).models()?.first()?.clone();
        Some(self.create_language_model(model, copilot_chat))
    }

    fn default_fast_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        // The default model should be Copilot Chat's 'base model', which is likely a relatively fast
        // model (e.g. 4o) and a sensible choice when considering premium requests
        self.default_model(cx)
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let Some(copilot_chat) = CopilotChat::global(cx) else {
            return Vec::new();
        };
        let Some(models) = copilot_chat.read(cx).models() else {
            return Vec::new();
        };
        models
            .iter()
            .cloned()
            .map(|model| self.create_language_model(model, copilot_chat.clone()))
            .collect()
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated(cx)
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        if self.is_authenticated(cx) {
            return Task::ready(Ok(()));
        }

        Task::ready(Err(AuthenticateError::CredentialsNotFound))
    }

    fn settings_view(&self, cx: &mut App) -> Option<ProviderSettingsView> {
        let is_authenticated = self.state.read(cx).is_authenticated(cx);
        let title = if is_authenticated {
            None
        } else {
            Some("Configure Copilot Chat".into())
        };
        let description = if is_authenticated {
            None
        } else {
            Some(language_model::InlineDescription::Text(
                "Requires an active GitHub Copilot subscription.".into(),
            ))
        };

        Some(ProviderSettingsView::Inline(
            language_model::InlineProviderSettings {
                title,
                description,
                create_view: Arc::new(|_window, cx| {
                    cx.new(|cx| {
                        copilot_ui::ConfigurationView::new(
                            |cx| {
                                CopilotChat::global(cx)
                                    .map(|model| model.read(cx).is_authenticated())
                                    .unwrap_or(false)
                            },
                            copilot_ui::ConfigurationMode::Chat,
                            cx,
                        )
                        .compact()
                    })
                    .into()
                }),
            },
        ))
    }

    fn set_api_key(&self, key: Option<String>, cx: &mut App) -> Task<Result<()>> {
        // Copilot authenticates via an OAuth device flow rather than an API key,
        // so the only meaningful credential change here is clearing it (which
        // signs the user out of the agent provider).
        if key.is_some() {
            return Task::ready(Ok(()));
        }
        let Some(copilot_chat) = CopilotChat::global(cx) else {
            return Task::ready(Ok(()));
        };
        copilot_chat.update(cx, |chat, cx| chat.sign_out(cx))
    }
}
