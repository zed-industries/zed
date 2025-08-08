use anyhow::{Context as _, Result, anyhow};
use credentials_provider::CredentialsProvider;

use convert_case::{Case, Casing};
use futures::{FutureExt, StreamExt, future::BoxFuture};
use gpui::{AnyView, App, AsyncApp, Context, Entity, Subscription, Task, Window};
use http_client::HttpClient;
use language_model::{
    AuthenticateError, LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelId, LanguageModelName, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest,
    LanguageModelToolChoice, RateLimiter,
};
use menu;
use open_ai::{ResponseStreamEvent, stream_completion};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use std::sync::Arc;

use ui::{ElevationIndex, Tooltip, prelude::*};
use ui_input::SingleLineInput;
use util::ResultExt;

use crate::AllLanguageModelSettings;
use crate::provider::open_ai::{OpenAiEventMapper, into_open_ai};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct OpenAiCompatibleSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AvailableModel {
    pub name: String,
    pub display_name: Option<String>,
    pub max_tokens: u64,
    pub max_output_tokens: Option<u64>,
    pub max_completion_tokens: Option<u64>,
}

pub struct OpenAiCompatibleLanguageModelProvider {
    id: LanguageModelProviderId,
    name: LanguageModelProviderName,
    http_client: Arc<dyn HttpClient>,
    state: gpui::Entity<State>,
}

pub struct State {
    id: Arc<str>,
    env_var_name: Arc<str>,
    api_key: Option<String>,
    api_key_from_env: bool,
    settings: OpenAiCompatibleSettings,
    _subscription: Subscription,
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.api_key.is_some()
    }

    fn reset_api_key(&self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        let api_url = self.settings.api_url.clone();
        cx.spawn(async move |this, cx| {
            credentials_provider
                .delete_credentials(&api_url, &cx)
                .await
                .log_err();
            this.update(cx, |this, cx| {
                this.api_key = None;
                this.api_key_from_env = false;
                cx.notify();
            })
        })
    }

    fn set_api_key(&mut self, api_key: String, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        let api_url = self.settings.api_url.clone();
        cx.spawn(async move |this, cx| {
            credentials_provider
                .write_credentials(&api_url, "Bearer", api_key.as_bytes(), &cx)
                .await
                .log_err();
            this.update(cx, |this, cx| {
                this.api_key = Some(api_key);
                cx.notify();
            })
        })
    }

    fn authenticate(&self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        if self.is_authenticated() {
            return Task::ready(Ok(()));
        }

        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        let env_var_name = self.env_var_name.clone();
        let api_url = self.settings.api_url.clone();
        cx.spawn(async move |this, cx| {
            let (api_key, from_env) = if let Ok(api_key) = std::env::var(env_var_name.as_ref()) {
                (api_key, true)
            } else {
                let (_, api_key) = credentials_provider
                    .read_credentials(&api_url, &cx)
                    .await?
                    .ok_or(AuthenticateError::CredentialsNotFound)?;
                (
                    String::from_utf8(api_key).context("invalid {PROVIDER_NAME} API key")?,
                    false,
                )
            };
            this.update(cx, |this, cx| {
                this.api_key = Some(api_key);
                this.api_key_from_env = from_env;
                cx.notify();
            })?;

            Ok(())
        })
    }
}

impl OpenAiCompatibleLanguageModelProvider {
    pub fn new(id: Arc<str>, http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        fn resolve_settings<'a>(id: &'a str, cx: &'a App) -> Option<&'a OpenAiCompatibleSettings> {
            AllLanguageModelSettings::get_global(cx)
                .openai_compatible
                .get(id)
        }

        let state = cx.new(|cx| State {
            id: id.clone(),
            env_var_name: format!("{}_API_KEY", id).to_case(Case::Constant).into(),
            settings: resolve_settings(&id, cx).cloned().unwrap_or_default(),
            api_key: None,
            api_key_from_env: false,
            _subscription: cx.observe_global::<SettingsStore>(|this: &mut State, cx| {
                let Some(settings) = resolve_settings(&this.id, cx) else {
                    return;
                };
                if &this.settings != settings {
                    this.settings = settings.clone();
                    cx.notify();
                }
            }),
        });

        Self {
            id: id.clone().into(),
            name: id.into(),
            http_client,
            state,
        }
    }

    fn create_language_model(&self, model: AvailableModel) -> Arc<dyn LanguageModel> {
        Arc::new(OpenAiCompatibleLanguageModel {
            id: LanguageModelId::from(model.name.clone()),
            provider_id: self.id.clone(),
            provider_name: self.name.clone(),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }
}

impl LanguageModelProviderState for OpenAiCompatibleLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<gpui::Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for OpenAiCompatibleLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelProviderName {
        self.name.clone()
    }

    fn icon(&self) -> IconName {
        IconName::AiOpenAiCompat
    }

    fn default_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        self.state
            .read(cx)
            .settings
            .available_models
            .first()
            .map(|model| self.create_language_model(model.clone()))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        None
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        self.state
            .read(cx)
            .settings
            .available_models
            .iter()
            .map(|model| self.create_language_model(model.clone()))
            .collect()
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        self.state.update(cx, |state, cx| state.authenticate(cx))
    }

    fn configuration_view(&self, window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| ConfigurationView::new(self.state.clone(), window, cx))
            .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state.update(cx, |state, cx| state.reset_api_key(cx))
    }
}

pub struct OpenAiCompatibleLanguageModel {
    id: LanguageModelId,
    provider_id: LanguageModelProviderId,
    provider_name: LanguageModelProviderName,
    model: AvailableModel,
    state: gpui::Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl OpenAiCompatibleLanguageModel {
    fn stream_completion(
        &self,
        request: open_ai::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<futures::stream::BoxStream<'static, Result<ResponseStreamEvent>>>>
    {
        let http_client = self.http_client.clone();
        let Ok((api_key, api_url)) = cx.read_entity(&self.state, |state, _| {
            (state.api_key.clone(), state.settings.api_url.clone())
        }) else {
            return futures::future::ready(Err(anyhow!("App state dropped"))).boxed();
        };

        let provider = self.provider_name.clone();
        let future = self.request_limiter.stream(async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey { provider });
            };
            let request = stream_completion(http_client.as_ref(), &api_url, &api_key, request);
            let response = request.await?;
            Ok(response)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

impl LanguageModel for OpenAiCompatibleLanguageModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(
            self.model
                .display_name
                .clone()
                .unwrap_or_else(|| self.model.name.clone()),
        )
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        self.provider_id.clone()
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        self.provider_name.clone()
    }

    fn supports_tools(&self) -> bool {
        true
    }

    fn supports_images(&self) -> bool {
        false
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto => true,
            LanguageModelToolChoice::Any => true,
            LanguageModelToolChoice::None => true,
        }
    }

    fn telemetry_id(&self) -> String {
        format!("openai/{}", self.model.name)
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_tokens
    }

    fn max_output_tokens(&self) -> Option<u64> {
        self.model.max_output_tokens
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &App,
    ) -> BoxFuture<'static, Result<u64>> {
        let max_token_count = self.max_token_count();
        cx.background_spawn(async move {
            let messages = super::open_ai::collect_tiktoken_messages(request);
            let model = if max_token_count >= 100_000 {
                // If the max tokens is 100k or more, it is likely the o200k_base tokenizer from gpt4o
                "gpt-4o"
            } else {
                // Otherwise fallback to gpt-4, since only cl100k_base and o200k_base are
                // supported with this tiktoken method
                "gpt-4"
            };
            tiktoken_rs::num_tokens_from_messages(model, &messages).map(|tokens| tokens as u64)
        })
        .boxed()
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            futures::stream::BoxStream<
                'static,
                Result<LanguageModelCompletionEvent, LanguageModelCompletionError>,
            >,
            LanguageModelCompletionError,
        >,
    > {
        let request = into_open_ai(request, &self.model.name, true, self.max_output_tokens());
        let completions = self.stream_completion(request, cx);
        async move {
            let mapper = OpenAiEventMapper::new();
            Ok(mapper.map_stream(completions.await?).boxed())
        }
        .boxed()
    }
}

struct ConfigurationView {
    api_key_editor: Entity<SingleLineInput>,
    state: gpui::Entity<State>,
    load_credentials_task: Option<Task<()>>,
}

impl ConfigurationView {
    fn new(state: gpui::Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let api_key_editor = cx.new(|cx| {
            SingleLineInput::new(
                window,
                cx,
                "000000000000000000000000000000000000000000000000000",
            )
        });

        cx.observe(&state, |_, _, cx| {
            cx.notify();
        })
        .detach();

        let load_credentials_task = Some(cx.spawn_in(window, {
            let state = state.clone();
            async move |this, cx| {
                if let Some(task) = state
                    .update(cx, |state, cx| state.authenticate(cx))
                    .log_err()
                {
                    // We don't log an error, because "not signed in" is also an error.
                    let _ = task.await;
                }
                this.update(cx, |this, cx| {
                    this.load_credentials_task = None;
                    cx.notify();
                })
                .log_err();
            }
        }));

        Self {
            api_key_editor,
            state,
            load_credentials_task,
        }
    }

    fn save_api_key(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let api_key = self
            .api_key_editor
            .read(cx)
            .editor()
            .read(cx)
            .text(cx)
            .trim()
            .to_string();

        // Don't proceed if no API key is provided and we're not authenticated
        if api_key.is_empty() && !self.state.read(cx).is_authenticated() {
            return;
        }

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(api_key, cx))?
                .await
        })
        .detach_and_log_err(cx);

        cx.notify();
    }

    fn reset_api_key(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.api_key_editor.update(cx, |input, cx| {
            input.editor.update(cx, |editor, cx| {
                editor.set_text("", window, cx);
            });
        });

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state.update(cx, |state, cx| state.reset_api_key(cx))?.await
        })
        .detach_and_log_err(cx);

        cx.notify();
    }

    fn should_render_editor(&self, cx: &mut Context<Self>) -> bool {
        !self.state.read(cx).is_authenticated()
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let env_var_set = self.state.read(cx).api_key_from_env;
        let env_var_name = self.state.read(cx).env_var_name.clone();

        let api_key_section = if self.should_render_editor(cx) {
            v_flex()
                .on_action(cx.listener(Self::save_api_key))
                .child(Label::new("To use Zed's agent with an OpenAI-compatible provider, you need to add an API key."))
                .child(
                    div()
                        .pt(DynamicSpacing::Base04.rems(cx))
                        .child(self.api_key_editor.clone())
                )
                .child(
                    Label::new(
                        format!("You can also assign the {env_var_name} environment variable and restart Zed."),
                    )
                    .size(LabelSize::Small).color(Color::Muted),
                )
                .into_any()
        } else {
            h_flex()
                .mt_1()
                .p_1()
                .justify_between()
                .rounded_md()
                .border_1()
                .border_color(cx.theme().colors().border)
                .bg(cx.theme().colors().background)
                .child(
                    h_flex()
                        .gap_1()
                        .child(Icon::new(IconName::Check).color(Color::Success))
                        .child(Label::new(if env_var_set {
                            format!("API key set in {env_var_name} environment variable.")
                        } else {
                            "API key configured.".to_string()
                        })),
                )
                .child(
                    Button::new("reset-api-key", "Reset API Key")
                        .label_size(LabelSize::Small)
                        .icon(IconName::Undo)
                        .icon_size(IconSize::Small)
                        .icon_position(IconPosition::Start)
                        .layer(ElevationIndex::ModalSurface)
                        .when(env_var_set, |this| {
                            this.tooltip(Tooltip::text(format!("To reset your API key, unset the {env_var_name} environment variable.")))
                        })
                        .on_click(cx.listener(|this, _, window, cx| this.reset_api_key(window, cx))),
                )
                .into_any()
        };

        if self.load_credentials_task.is_some() {
            div().child(Label::new("Loading credentials…")).into_any()
        } else {
            v_flex().size_full().child(api_key_section).into_any()
        }
    }
}
