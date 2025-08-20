use anyhow::{Context as _, Result, anyhow};
use collections::BTreeMap;
use credentials_provider::CredentialsProvider;

use futures::{FutureExt, StreamExt, future::BoxFuture};
use gpui::{
    Animation, AnimationExt, AnyView, App, AsyncApp, Context, Subscription, Task, Transformation,
    Window, percentage,
};
use http_client::HttpClient;
use language_model::{
    AuthenticateError, LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelId, LanguageModelName, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest,
    LanguageModelToolChoice, LanguageModelToolSchemaFormat, RateLimiter,
};

use open_ai::ResponseStreamEvent;
use oracle_code_assist::oauth::{OAuthToken, OcaOAuthClient};
use oracle_code_assist::{Model, stream_completion};
pub use settings::OracleAvailableModel as AvailableModel;
use settings::{Settings, SettingsStore};
use std::sync::Arc;
use std::time::Duration;
use strum::IntoEnumIterator;

use ui::{CommonAnimationExt, prelude::*};
use util::ResultExt;

use crate::AllLanguageModelSettings;

const PROVIDER_ID: LanguageModelProviderId = language_model::ORACLE_PROVIDER_ID;
const PROVIDER_NAME: LanguageModelProviderName = language_model::ORACLE_PROVIDER_NAME;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct OracleCodeAssistSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
}

pub struct OracleCodeAssistModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: gpui::Entity<State>,
}

pub struct State {
    oauth_token: Option<OAuthToken>,
    _subscription: Subscription,
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.oauth_token.is_some()
            && self
                .oauth_token
                .as_ref()
                .map(|token| !token.is_expired())
                .unwrap()
    }

    fn reset_oauth_token(&self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        let api_url = AllLanguageModelSettings::get_global(cx)
            .oracle
            .api_url
            .clone();
        cx.spawn(async move |this, cx| {
            credentials_provider
                .delete_credentials(&api_url, &cx)
                .await
                .log_err();
            this.update(cx, |this, cx| {
                this.oauth_token = None;
                cx.notify();
            })
        })
    }

    fn set_oauth_token(
        &mut self,
        oauth_token: OAuthToken,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        let api_url = AllLanguageModelSettings::get_global(cx)
            .oracle
            .api_url
            .clone();

        let oauth_token_json = match serde_json::to_string(&oauth_token) {
            Ok(json) => json,
            Err(err) => {
                return Task::ready(Err(anyhow::anyhow!(
                    "Failed to serialize OAuth Token: {}",
                    err
                )));
            }
        };

        cx.spawn(async move |this, cx| {
            credentials_provider
                .write_credentials(&api_url, "OAuth", oauth_token_json.as_bytes(), &cx)
                .await
                .log_err();
            this.update(cx, |this, cx| {
                this.oauth_token = Some(oauth_token);
                cx.notify();
            })
        })
    }

    fn authenticate(&self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        if self.is_authenticated() {
            return Task::ready(Ok(()));
        }

        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        let api_url = AllLanguageModelSettings::get_global(cx)
            .oracle
            .api_url
            .clone();

        let client = cx.http_client();
        cx.spawn(async move |this, cx| {
            let (_, credential_data) = credentials_provider
                .read_credentials(&api_url, &cx)
                .await?
                .ok_or(AuthenticateError::CredentialsNotFound)?;
            let oauth_str =
                String::from_utf8(credential_data).context("Invalid OAuth data format")?;

            let mut oauth_token: OAuthToken =
                serde_json::from_str(&oauth_str).context("Invalid OAuth Token JSON format")?;

            if !oauth_token.refresh_token.is_empty() && !oauth_token.access_token.is_empty() {
                if oauth_token.is_expired() {
                    oauth_token = oauth_token.refresh(client).await?;
                }
                this.update(cx, |this, cx| {
                    this.oauth_token = Some(oauth_token);
                    cx.notify();
                })?;
            }

            Ok(())
        })
    }
}

impl OracleCodeAssistModelProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        let state = cx.new(|cx| State {
            oauth_token: None,
            _subscription: cx.observe_global::<SettingsStore>(|_this: &mut State, cx| {
                cx.notify();
            }),
        });

        Self { http_client, state }
    }

    fn create_language_model(&self, model: Model) -> Arc<dyn LanguageModel> {
        Arc::new(OracleCodeAssistLanguageModel {
            id: LanguageModelId::from(model.id().to_string()),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }
}

impl LanguageModelProviderState for OracleCodeAssistModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<gpui::Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for OracleCodeAssistModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconName {
        IconName::AiOpenAiCompat
    }

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(Model::default()))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(Model::default_fast()))
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models = BTreeMap::default();

        for model in Model::iter() {
            if !matches!(model, Model::Custom { .. }) {
                models.insert(model.id().to_string(), model);
            }
        }

        // Override with available models from settings
        for model in &AllLanguageModelSettings::get_global(cx)
            .oracle
            .available_models
        {
            models.insert(
                model.name.clone(),
                Model::Custom {
                    name: model.name.clone(),
                    display_name: model.display_name.clone(),
                    max_tokens: model.max_tokens,
                    max_output_tokens: model.max_output_tokens,
                    max_completion_tokens: model.max_completion_tokens,
                    reasoning_effort: model.reasoning_effort.clone(),
                },
            );
        }

        models
            .into_values()
            .map(|model| self.create_language_model(model))
            .collect()
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        self.state.update(cx, |state, cx| state.authenticate(cx))
    }

    fn configuration_view(
        &self,
        _target_agent: language_model::ConfigurationViewTargetAgent,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyView {
        cx.new(|cx| ConfigurationView::new(self.state.clone(), window, cx))
            .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.reset_oauth_token(cx))
    }
}

pub struct OracleCodeAssistLanguageModel {
    id: LanguageModelId,
    model: Model,
    state: gpui::Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl OracleCodeAssistLanguageModel {
    fn stream_completion(
        &self,
        request: open_ai::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<futures::stream::BoxStream<'static, Result<ResponseStreamEvent>>>>
    {
        let http_client = self.http_client.clone();
        let Ok((oauth_token, api_url)) = cx.read_entity(&self.state, |state, cx| {
            let settings = &AllLanguageModelSettings::get_global(cx).oracle;
            (state.oauth_token.clone(), settings.api_url.clone())
        }) else {
            return futures::future::ready(Err(anyhow!("App state dropped"))).boxed();
        };

        let future = self.request_limiter.stream(async move {
            let Some(oauth_token) = oauth_token else {
                return Err(LanguageModelCompletionError::NoApiKey {
                    provider: PROVIDER_NAME,
                });
            };
            let request = stream_completion(
                http_client.as_ref(),
                &api_url,
                &oauth_token.access_token,
                request,
            );
            let response = request.await?;
            Ok(response)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

impl LanguageModel for OracleCodeAssistLanguageModel {
    fn tool_input_format(&self) -> LanguageModelToolSchemaFormat {
        match self.model {
            Model::Grok4 => LanguageModelToolSchemaFormat::JsonSchemaSubset,
            _ => LanguageModelToolSchemaFormat::JsonSchema,
        }
    }

    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(self.model.display_name().to_string())
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
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
        format!("oracle/{}", self.model.id())
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_token_count()
    }

    fn max_output_tokens(&self) -> Option<u64> {
        self.model.max_output_tokens()
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &App,
    ) -> BoxFuture<'static, Result<u64>> {
        // TODO: Use .model_vendor()
        match self.model {
            Model::Grok3 => super::x_ai::count_xai_tokens(request, x_ai::Model::Grok3, cx),
            Model::Grok4 => super::x_ai::count_xai_tokens(request, x_ai::Model::Grok4, cx),
            Model::FourPointOne => {
                super::open_ai::count_open_ai_tokens(request, open_ai::Model::FourPointOne, cx)
            }
            Model::O3 => super::open_ai::count_open_ai_tokens(request, open_ai::Model::O3, cx),
            _ => unimplemented!(),
        }
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
        let request = super::open_ai::into_open_ai(
            request,
            self.model.id(),
            self.model.supports_parallel_tool_calls(),
            self.model.supports_prompt_cache_key(),
            self.max_output_tokens(),
            self.model.reasoning_effort(),
        );
        let completions = self.stream_completion(request, cx);
        async move {
            let mapper = super::open_ai::OpenAiEventMapper::new();
            Ok(mapper.map_stream(completions.await?).boxed())
        }
        .boxed()
    }
}

struct ConfigurationView {
    state: gpui::Entity<State>,
    load_credentials_task: Option<Task<()>>,
    authentication_task: Option<Task<()>>,
    connecting: bool,
}

impl ConfigurationView {
    fn new(state: gpui::Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
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
            state,
            load_credentials_task,
            authentication_task: None,
            connecting: false,
        }
    }

    fn initate_oauth(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.connecting {
            return;
        }
        self.connecting = true;
        cx.notify();

        let state = self.state.clone();
        let authentication_task = cx.spawn_in(window, async move |this, cx| {
            let result = cx
                .update(|_window, cx| (OcaOAuthClient::initiate_oauth(cx), cx.http_client()))
                .log_err();

            if let Some((oauth_session, http_client)) = result {
                if let Some(oauth_session) = oauth_session.log_err() {
                    let auth_result = cx
                        .background_spawn(async move {
                            OcaOAuthClient::authenticate(http_client, oauth_session).await
                        })
                        .await;

                    match auth_result {
                        Ok(oauth_token) => {
                            let save_task = state
                                .update(cx, |state, cx| state.set_oauth_token(oauth_token, cx))
                                .log_err();

                            if let Some(task) = save_task {
                                task.await.log_err();
                            }
                        }
                        Err(_) => {}
                    }
                }
            }

            // Reset the connecting state and clear the authentication task
            this.update(cx, |this, cx| {
                this.connecting = false;
                this.authentication_task = None;
                cx.notify();
            })
            .log_err();
        });

        self.authentication_task = Some(authentication_task);
    }

    fn sign_out(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.reset_oauth_token(cx))?
                .await
                .log_err();
            Ok::<(), anyhow::Error>(())
        })
        .detach_and_log_err(cx);
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.load_credentials_task.is_some() {
            let loading_icon = Icon::new(IconName::ArrowCircle).with_rotate_animation(2);

            return h_flex()
                .gap_2()
                .child(loading_icon)
                .child(Label::new("Loading Oracle Code Assist credentials…"));
        }

        let is_authenticated = self.state.read(cx).is_authenticated();

        if is_authenticated {
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
                        .child(Label::new("Connected to Oracle Code Assist")),
                )
                .child(
                    Button::new("sign_out", "Sign Out")
                        .label_size(LabelSize::Small)
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.sign_out(window, cx);
                        })),
                )
        } else {
            let loading_icon = Icon::new(IconName::ArrowCircle).with_rotate_animation(2);

            if self.connecting {
                h_flex()
                    .gap_2()
                    .child(loading_icon)
                    .child(Label::new("Connecting to Oracle Code Assist…"))
            } else {
                const DESCRIPTION: &str = "To use Oracle Code Assist language models, you need to authenticate with your Oracle account. This will provide access to advanced AI-powered code completion and assistance features.";

                v_flex().gap_2().child(Label::new(DESCRIPTION)).child(
                    Button::new("connect_oca", "Connect to Oracle Code Assist")
                        .icon_color(Color::Muted)
                        .icon(IconName::Ai)
                        .icon_position(IconPosition::Start)
                        .icon_size(IconSize::Medium)
                        .full_width()
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.initate_oauth(window, cx);
                        })),
                )
            }
        }
    }
}
