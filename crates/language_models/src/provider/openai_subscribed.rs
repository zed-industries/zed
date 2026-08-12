use anyhow::Result;
use credentials_provider::CredentialsProvider;
use futures::future::Shared;
use gpui::{App, Context, Entity, SharedString, Task, Window};
use http_client::HttpClient;
use language_model::{
    AuthenticateError, FastModeConfirmation, IconOrSvg, InlineDescription, LanguageModel,
    LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, ProviderSettingsView,
};
use openai_subscribed::{ChatGptModel, PROVIDER_ID, PROVIDER_NAME, State, create_language_model};
use std::sync::Arc;
use ui::{ConfiguredApiCard, prelude::*};

const SUBSCRIPTION_DESCRIPTION: &str =
    "Sign in with your ChatGPT Plus or Pro subscription to use OpenAI models in Zed's agent.";

pub struct OpenAiSubscribedProvider {
    state: Entity<State>,
}

impl OpenAiSubscribedProvider {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut App,
    ) -> Self {
        let state = cx.new(|cx| State::new(http_client, credentials_provider, cx));
        Self { state }
    }
}

impl LanguageModelProviderState for OpenAiSubscribedProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for OpenAiSubscribedProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiOpenAiGptSub)
    }

    fn default_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(create_language_model(
            ChatGptModel::Gpt56Sol,
            &self.state,
            cx,
        ))
    }

    fn default_fast_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(create_language_model(
            ChatGptModel::Gpt56Luna,
            &self.state,
            cx,
        ))
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        ChatGptModel::all()
            .into_iter()
            .map(|model| create_language_model(model, &self.state, cx))
            .collect()
    }

    fn recommended_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        vec![create_language_model(
            ChatGptModel::Gpt56Sol,
            &self.state,
            cx,
        )]
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        if self.is_authenticated(cx) {
            return Task::ready(Ok(()));
        }
        let load_task: Option<Shared<_>> = self.state.read(cx).load_task();
        if let Some(load_task) = load_task {
            let weak_state = self.state.downgrade();
            cx.spawn(async move |cx| {
                let _ = load_task.await;
                let is_auth = weak_state
                    .read_with(&*cx, |state, _| state.is_authenticated())
                    .unwrap_or(false);
                if is_auth {
                    Ok(())
                } else {
                    Err(AuthenticateError::CredentialsNotFound)
                }
            })
        } else {
            Task::ready(Err(AuthenticateError::CredentialsNotFound))
        }
    }

    fn settings_view(&self, cx: &mut App) -> Option<ProviderSettingsView> {
        let is_authenticated = self.state.read(cx).is_authenticated();
        let title = if is_authenticated {
            None
        } else {
            Some("Configure ChatGPT".into())
        };
        let description = if is_authenticated {
            None
        } else {
            Some(InlineDescription::Text(SUBSCRIPTION_DESCRIPTION.into()))
        };

        Some(ProviderSettingsView::Inline(
            language_model::InlineProviderSettings {
                title,
                description,
                create_view: Arc::new({
                    let state = self.state.clone();
                    move |_window, cx| {
                        cx.new(|_cx| ConfigurationView {
                            state: state.clone(),
                            compact: true,
                        })
                        .into()
                    }
                }),
            },
        ))
    }

    fn authentication_error_message(&self) -> SharedString {
        "Your ChatGPT subscription session is invalid or has expired. \
        Sign in again via Settings > AI > LLM Providers to continue."
            .into()
    }

    fn missing_credentials_error_message(&self) -> SharedString {
        "You are not signed in to your ChatGPT account. \
        Sign in via Settings > AI > LLM Providers to continue."
            .into()
    }

    fn fast_mode_confirmation(&self, _cx: &App) -> Option<FastModeConfirmation> {
        Some(FastModeConfirmation {
            title: "Enable Fast Mode for OpenAI?".into(),
            message: "Fast mode sends requests using OpenAI's Priority processing tier, which \
                targets significantly lower latency than the standard tier and is billed at a \
                premium per-token rate."
                .into(),
        })
    }
}

struct ConfigurationView {
    state: Entity<State>,
    /// When `true`, the description is rendered elsewhere (the settings row's
    /// left column), so it's omitted here to avoid duplication.
    compact: bool,
}

impl Render for ConfigurationView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.state.read(cx);

        if state.is_authenticated() {
            let label = state
                .email()
                .map(|e| format!("Signed in as {e}"))
                .unwrap_or_else(|| "Signed in".to_string());

            let state_entity = self.state.clone();

            return v_flex()
                .child(
                    ConfiguredApiCard::new("openai-subscribed-sign-out", SharedString::from(label))
                        .button_label("Sign Out")
                        .on_click(cx.listener(move |_this, _, _window, cx| {
                            state_entity
                                .update(cx, |state, cx| state.sign_out(cx))
                                .detach_and_log_err(cx);
                        })),
                )
                .into_any_element();
        }

        let last_auth_error = state.last_auth_error();
        let provider_state = self.state.clone();

        let is_signing_in = state.is_signing_in();
        let button_label = if is_signing_in {
            "Signing in…"
        } else {
            "Sign In"
        };

        v_flex()
            .gap_2()
            .when(!self.compact, |this| {
                this.child(Label::new(SUBSCRIPTION_DESCRIPTION))
            })
            .child(
                Button::new("sign-in", button_label)
                    .when(!self.compact, |this| this.full_width())
                    .style(ButtonStyle::Outlined)
                    .size(ButtonSize::Medium)
                    .loading(is_signing_in)
                    .disabled(is_signing_in)
                    .on_click(move |_, _window, cx| {
                        provider_state.update(cx, |state, cx| state.sign_in(cx));
                    }),
            )
            .when_some(last_auth_error, |this, error| {
                this.child(
                    h_flex()
                        .gap_1()
                        .justify_center()
                        .child(
                            Icon::new(IconName::XCircle)
                                .color(Color::Error)
                                .size(IconSize::Small),
                        )
                        .child(Label::new(error).color(Color::Muted)),
                )
            })
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{AsyncApp, TestAppContext};
    use http_client::FakeHttpClient;
    use parking_lot::Mutex;
    use std::future::Future;
    use std::pin::Pin;

    #[gpui::test]
    async fn test_authenticate_awaits_initial_load(cx: &mut TestAppContext) {
        let creds_json = serde_json::json!({
            "access_token": "fresh_access",
            "refresh_token": "fresh_refresh",
            "expires_at_ms": u64::MAX,
            "account_id": null,
            "email": null,
        });
        let creds_provider = Arc::new(FakeCredentialsProvider::new());
        creds_provider.storage.lock().replace((
            "Bearer".to_string(),
            serde_json::to_vec(&creds_json).unwrap(),
        ));

        let http_client = FakeHttpClient::create(|_| async {
            Ok(http_client::Response::builder()
                .status(200)
                .body(http_client::AsyncBody::default())?)
        });

        let provider =
            cx.update(|cx| OpenAiSubscribedProvider::new(http_client, creds_provider, cx));

        // Before load completes, authenticate should still await the load.
        let auth_task = cx.update(|cx| provider.authenticate(cx));

        // Drive the load to completion.
        cx.run_until_parked();

        let result = auth_task.await;
        assert!(
            result.is_ok(),
            "authenticate should succeed after load completes with valid credentials"
        );
    }

    struct FakeCredentialsProvider {
        storage: Mutex<Option<(String, Vec<u8>)>>,
    }

    impl FakeCredentialsProvider {
        fn new() -> Self {
            Self {
                storage: Mutex::new(None),
            }
        }
    }

    impl CredentialsProvider for FakeCredentialsProvider {
        fn read_credentials<'a>(
            &'a self,
            _url: &'a str,
            _cx: &'a AsyncApp,
        ) -> Pin<Box<dyn Future<Output = Result<Option<(String, Vec<u8>)>>> + 'a>> {
            Box::pin(async { Ok(self.storage.lock().clone()) })
        }

        fn write_credentials<'a>(
            &'a self,
            _url: &'a str,
            username: &'a str,
            password: &'a [u8],
            _cx: &'a AsyncApp,
        ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
            self.storage
                .lock()
                .replace((username.to_string(), password.to_vec()));
            Box::pin(async { Ok(()) })
        }

        fn delete_credentials<'a>(
            &'a self,
            _url: &'a str,
            _cx: &'a AsyncApp,
        ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
            *self.storage.lock() = None;
            Box::pin(async { Ok(()) })
        }
    }
}
