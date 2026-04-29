use anyhow::{Context as _, Result};
use convert_case::{Case, Casing};
use credentials_provider::CredentialsProvider;
use futures::{FutureExt, StreamExt, future::BoxFuture, future::Shared};
use gpui::{AnyView, App, AsyncApp, Context, Entity, SharedString, Task, Window};
use http_client::HttpClient;
use language_model::{
    ApiKeyState, AuthenticateError, EnvVar, IconOrSvg, LanguageModel, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest, LanguageModelToolChoice, LanguageModelToolSchemaFormat, RateLimiter,
};
use menu;
use open_ai::{
    ResponseStreamEvent,
    responses::{Request as ResponseRequest, StreamEvent as ResponsesStreamEvent, stream_response},
    stream_completion,
};
use settings::{Settings, SettingsStore};
use std::sync::Arc;
use std::time::{Duration, Instant};
use ui::{ElevationIndex, Tooltip, prelude::*};
use ui_input::InputField;
use util::ResultExt;
use util::shell::ShellKind;

use crate::provider::open_ai::{
    OpenAiEventMapper, OpenAiResponseEventMapper, into_open_ai, into_open_ai_response,
};
pub use settings::OpenAiCompatibleAvailableModel as AvailableModel;
pub use settings::OpenAiCompatibleModelCapabilities as ModelCapabilities;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct OpenAiCompatibleSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
    pub api_key_helper: Option<ApiKeyHelperConfig>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ApiKeyHelperConfig {
    pub command: String,
    pub ttl_seconds: Option<u64>,
}

const MAX_TTL_SECONDS: u64 = 7 * 24 * 60 * 60;

impl From<settings::ApiKeyHelperConfig> for ApiKeyHelperConfig {
    fn from(config: settings::ApiKeyHelperConfig) -> Self {
        Self {
            command: config.command,
            ttl_seconds: config.ttl_seconds.map(|s| s.min(MAX_TTL_SECONDS)),
        }
    }
}

pub struct OpenAiCompatibleLanguageModelProvider {
    id: LanguageModelProviderId,
    name: LanguageModelProviderName,
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

pub struct State {
    id: Arc<str>,
    api_key_state: ApiKeyState,
    settings: OpenAiCompatibleSettings,
    credentials_provider: Arc<dyn CredentialsProvider>,
    helper_key_status: HelperKeyStatus,
}

enum HelperKeyStatus {
    NeverRun,
    Running {
        task: Shared<Task<Result<(), String>>>,
    },
    Succeeded { expires_at: Option<Instant> },
    Failed { error: String },
}

impl State {
    fn is_authenticated(&self) -> bool {
        if self.api_key_state.is_from_env_var() {
            return true;
        }
        self.api_key_state.has_key() && !self.needs_helper_refresh()
    }

    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = SharedString::new(self.settings.api_url.as_str());
        self.api_key_state.store(
            api_url,
            api_key,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        )
    }

    fn reset_helper_status(&mut self) {
        self.helper_key_status = HelperKeyStatus::NeverRun;
    }

    fn needs_helper_refresh(&self) -> bool {
        if self.settings.api_key_helper.is_none() {
            return false;
        }
        match &self.helper_key_status {
            HelperKeyStatus::NeverRun | HelperKeyStatus::Failed { .. } => true,
            HelperKeyStatus::Running { .. } => false,
            HelperKeyStatus::Succeeded { expires_at } => {
                expires_at.is_some_and(|expires_at| Instant::now() >= expires_at)
            }
        }
    }

    fn run_api_key_helper(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let Some(helper) = self.settings.api_key_helper.clone() else {
            return Task::ready(Err(AuthenticateError::CredentialsNotFound));
        };

        if helper.command.trim().is_empty() {
            let error = "api_key_helper command is empty".to_string();
            self.helper_key_status = HelperKeyStatus::Failed {
                error: error.clone(),
            };
            cx.notify();
            return Task::ready(Err(AuthenticateError::Other(anyhow::anyhow!(error))));
        }

        if let HelperKeyStatus::Running { task } = &self.helper_key_status {
            let task = task.clone();
            return cx.spawn(
                async move |_state, _cx| {
                    task.await
                        .map_err(|e| AuthenticateError::Other(anyhow::anyhow!(e)))
                },
            );
        }

        let launched_for_url = self.settings.api_url.clone();
        let launched_for_helper = self.settings.api_key_helper.clone();

        let credentials_provider = self.credentials_provider.clone();
        let api_url = SharedString::new(launched_for_url.as_str());
        let ttl_seconds = helper.ttl_seconds;

        log::debug!("Running api_key_helper command");

        let inner_task = cx
            .spawn(async move |state, cx| {
                let result: Result<String, String> = async {
                    let command_str = helper.command;
                    let command_task = cx.background_spawn(async move {
                        let shell =
                            util::get_default_system_shell_preferring_bash();
                        let shell_kind =
                            ShellKind::new(&shell, cfg!(windows));
                        let args =
                            shell_kind.args_for_shell(false, command_str);
                        util::command::new_command(&shell)
                            .args(&args)
                            .kill_on_drop(true)
                            .output()
                            .await
                    });

                    let timeout = cx.background_executor().timer(Duration::from_secs(30));

                    let output = match futures::future::select(
                        std::pin::pin!(command_task),
                        std::pin::pin!(timeout),
                    )
                    .await
                    {
                        futures::future::Either::Left((result, _)) => result,
                        futures::future::Either::Right((_, _)) => {
                            return Err(
                                "api_key_helper timed out after 30 seconds".to_string()
                            );
                        }
                    };

                    let output = output
                        .context("Failed to execute api_key_helper")
                        .map_err(|e| e.to_string())?;

                    if !output.status.success() {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        let truncated = truncate_for_display(stderr.trim(), 200);
                        return Err(format!(
                            "api_key_helper exited with {}: {}",
                            output.status, truncated,
                        ));
                    }

                    let api_key = String::from_utf8(output.stdout)
                        .context("api_key_helper output is not valid UTF-8")
                        .map_err(|e| e.to_string())?
                        .trim()
                        .to_string();

                    if api_key.is_empty() {
                        return Err("api_key_helper produced empty output".to_string());
                    }

                    Ok(api_key)
                }
                .await;

                match result {
                    Ok(api_key) => {
                        if let Err(err) = credentials_provider
                            .write_credentials(&api_url, "Bearer", api_key.as_bytes(), cx)
                            .await
                        {
                            log::warn!(
                                "Failed to persist API key from helper to keychain: {err}. \
                                 The key will only be available for this session."
                            );
                        }

                        state
                            .update(cx, |state, cx| {
                                // Guard against stale results: if the URL or helper
                                // config changed while this task was running, discard
                                // the result so the new config takes effect.
                                if state.settings.api_url != launched_for_url
                                    || state.settings.api_key_helper != launched_for_helper
                                {
                                    state.helper_key_status = HelperKeyStatus::NeverRun;
                                    cx.notify();
                                    return;
                                }
                                state.helper_key_status = HelperKeyStatus::Succeeded {
                                    expires_at: ttl_seconds
                                        .map(|secs| Instant::now() + Duration::from_secs(secs)),
                                };
                                state
                                    .api_key_state
                                    .set_key_from_helper(api_url, api_key.into());
                                cx.notify();
                            })
                            .map_err(|e| e.to_string())?;

                        Ok(())
                    }
                    Err(error_msg) => {
                        state
                            .update(cx, |state, cx| {
                                if state.settings.api_url != launched_for_url
                                    || state.settings.api_key_helper != launched_for_helper
                                {
                                    state.helper_key_status = HelperKeyStatus::NeverRun;
                                    cx.notify();
                                    return;
                                }
                                state.helper_key_status =
                                    HelperKeyStatus::Failed {
                                        error: error_msg.clone(),
                                    };
                                state.api_key_state.clear_helper_key();
                                cx.notify();
                            })
                            .log_err();
                        Err(error_msg)
                    }
                }
            })
            .shared();

        self.helper_key_status = HelperKeyStatus::Running {
            task: inner_task.clone(),
        };

        cx.spawn(async move |_state, _cx| {
            inner_task
                .await
                .map_err(|e| AuthenticateError::Other(anyhow::anyhow!(e)))
        })
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = SharedString::new(self.settings.api_url.clone());
        let has_helper = self.settings.api_key_helper.is_some();

        let load_task = self.api_key_state.load_if_needed(
            api_url,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        );

        if !has_helper {
            return load_task;
        }

        cx.spawn(async move |state, cx| {
            // load_if_needed always returns Ok(()) — errors are stored in load_status,
            // not propagated through the task. Check state afterwards instead.
            let _ = load_task.await;

            let should_run_helper = state
                .read_with(cx, |state, _| {
                    if state.api_key_state.is_from_env_var() {
                        return false;
                    }
                    !state.api_key_state.has_key() || state.needs_helper_refresh()
                })
                .unwrap_or(false);

            if !should_run_helper {
                return Ok(());
            }

            let helper_task = state
                .update(cx, |state, cx| state.run_api_key_helper(cx))?;
            helper_task.await
        })
    }
}

impl OpenAiCompatibleLanguageModelProvider {
    pub fn new(
        id: Arc<str>,
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut App,
    ) -> Self {
        fn resolve_settings<'a>(id: &'a str, cx: &'a App) -> Option<&'a OpenAiCompatibleSettings> {
            crate::AllLanguageModelSettings::get_global(cx)
                .openai_compatible
                .get(id)
        }

        let api_key_env_var_name = format!("{}_API_KEY", id).to_case(Case::UpperSnake).into();
        let state = cx.new(|cx| {
            cx.observe_global::<SettingsStore>(|this: &mut State, cx| {
                let Some(settings) = resolve_settings(&this.id, cx).cloned() else {
                    return;
                };
                if this.settings != settings {
                    let credentials_provider = this.credentials_provider.clone();
                    let api_url = SharedString::new(settings.api_url.as_str());
                    this.api_key_state.handle_url_change(
                        api_url,
                        |this| &mut this.api_key_state,
                        credentials_provider,
                        cx,
                    );
                    if this.settings.api_key_helper != settings.api_key_helper
                        || this.settings.api_url != settings.api_url
                    {
                        this.reset_helper_status();
                    }
                    this.settings = settings;
                    cx.notify();
                }
            })
            .detach();
            let settings = resolve_settings(&id, cx).cloned().unwrap_or_default();
            State {
                id: id.clone(),
                api_key_state: ApiKeyState::new(
                    SharedString::new(settings.api_url.as_str()),
                    EnvVar::new(api_key_env_var_name),
                ),
                settings,
                credentials_provider,
                helper_key_status: HelperKeyStatus::NeverRun,
            }
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

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
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

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiOpenAiCompat)
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
        self.state.update(cx, |state, cx| {
            state.reset_helper_status();
            state.set_api_key(None, cx)
        })
    }
}

pub struct OpenAiCompatibleLanguageModel {
    id: LanguageModelId,
    provider_id: LanguageModelProviderId,
    provider_name: LanguageModelProviderName,
    model: AvailableModel,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl OpenAiCompatibleLanguageModel {
    fn stream_completion(
        &self,
        request: open_ai::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            futures::stream::BoxStream<'static, Result<ResponseStreamEvent>>,
            LanguageModelCompletionError,
        >,
    > {
        let http_client = self.http_client.clone();

        let (api_key, api_url) = self.state.read_with(cx, |state, _cx| {
            let api_url = &state.settings.api_url;
            (
                state.api_key_state.key(api_url),
                state.settings.api_url.clone(),
            )
        });

        let provider = self.provider_name.clone();
        let future = self.request_limiter.stream(async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey { provider });
            };
            let request = stream_completion(
                http_client.as_ref(),
                provider.0.as_str(),
                &api_url,
                &api_key,
                request,
            );
            let response = request.await?;
            Ok(response)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }

    fn stream_response(
        &self,
        request: ResponseRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<futures::stream::BoxStream<'static, Result<ResponsesStreamEvent>>>>
    {
        let http_client = self.http_client.clone();

        let (api_key, api_url) = self.state.read_with(cx, |state, _cx| {
            let api_url = &state.settings.api_url;
            (
                state.api_key_state.key(api_url),
                state.settings.api_url.clone(),
            )
        });

        let provider = self.provider_name.clone();
        let future = self.request_limiter.stream(async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey { provider });
            };
            let request = stream_response(
                http_client.as_ref(),
                provider.0.as_str(),
                &api_url,
                &api_key,
                request,
            );
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
        self.model.capabilities.tools
    }

    fn tool_input_format(&self) -> LanguageModelToolSchemaFormat {
        LanguageModelToolSchemaFormat::JsonSchemaSubset
    }

    fn supports_images(&self) -> bool {
        self.model.capabilities.images
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto => self.model.capabilities.tools,
            LanguageModelToolChoice::Any => self.model.capabilities.tools,
            LanguageModelToolChoice::None => true,
        }
    }

    fn supports_streaming_tools(&self) -> bool {
        true
    }

    fn supports_split_token_display(&self) -> bool {
        true
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
        if self.model.capabilities.chat_completions {
            let request = into_open_ai(
                request,
                &self.model.name,
                self.model.capabilities.parallel_tool_calls,
                self.model.capabilities.prompt_cache_key,
                self.max_output_tokens(),
                self.model.reasoning_effort,
                self.model.capabilities.interleaved_reasoning,
            );
            let completions = self.stream_completion(request, cx);
            async move {
                let mapper = OpenAiEventMapper::new();
                Ok(mapper.map_stream(completions.await?).boxed())
            }
            .boxed()
        } else {
            let request = into_open_ai_response(
                request,
                &self.model.name,
                self.model.capabilities.parallel_tool_calls,
                self.model.capabilities.prompt_cache_key,
                self.max_output_tokens(),
                self.model.reasoning_effort,
            );
            let completions = self.stream_response(request, cx);
            async move {
                let mapper = OpenAiResponseEventMapper::new();
                Ok(mapper.map_stream(completions.await?).boxed())
            }
            .boxed()
        }
    }
}

struct ConfigurationView {
    api_key_editor: Entity<InputField>,
    state: Entity<State>,
    load_credentials_task: Option<Task<()>>,
}

impl ConfigurationView {
    fn new(state: Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let api_key_editor = cx.new(|cx| {
            InputField::new(
                window,
                cx,
                "000000000000000000000000000000000000000000000000000",
            )
        });

        cx.observe(&state, |_, _, cx| {
            cx.notify();
        })
        .detach();

        let load_credentials_task = Some(Self::spawn_authenticate(
            state.clone(),
            window,
            cx,
        ));

        Self {
            api_key_editor,
            state,
            load_credentials_task,
        }
    }

    fn save_api_key(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let api_key = self.api_key_editor.read(cx).text(cx).trim().to_string();
        if api_key.is_empty() {
            return;
        }

        // url changes can cause the editor to be displayed again
        self.api_key_editor
            .update(cx, |input, cx| input.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(Some(api_key), cx))
                .await
        })
        .detach_and_log_err(cx);
    }

    fn reset_api_key(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.api_key_editor
            .update(cx, |input, cx| input.set_text("", window, cx));

        let state = self.state.clone();
        self.load_credentials_task = Some(cx.spawn_in(window, async move |this, cx| {
            state
                .update(cx, |state, cx| {
                    state.reset_helper_status();
                    state.set_api_key(None, cx)
                })
                .await
                .log_err();

            Self::run_authenticate_and_clear_task(state, this, cx).await;
        }));
    }

    fn retry_helper(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let state = self.state.clone();
        self.load_credentials_task = Some(cx.spawn_in(window, async move |this, cx| {
            state.update(cx, |state, cx| {
                state.reset_helper_status();
                cx.notify();
            });

            Self::run_authenticate_and_clear_task(state, this, cx).await;
        }));
    }

    fn spawn_authenticate(
        state: Entity<State>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<()> {
        cx.spawn_in(window, async move |this, cx| {
            Self::run_authenticate_and_clear_task(state, this, cx).await;
        })
    }

    async fn run_authenticate_and_clear_task(
        state: Entity<State>,
        this: gpui::WeakEntity<Self>,
        cx: &mut AsyncApp,
    ) {
        let task = state.update(cx, |state, cx| state.authenticate(cx));
        match task.await {
            Ok(()) | Err(AuthenticateError::CredentialsNotFound) => {}
            Err(err) => log::error!("Authentication failed: {err}"),
        }
        this.update(cx, |this, cx| {
            this.load_credentials_task = None;
            cx.notify();
        })
        .log_err();
    }

    fn should_render_editor(&self, cx: &Context<Self>) -> bool {
        let state = self.state.read(cx);
        !state.is_authenticated() && state.settings.api_key_helper.is_none()
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.state.read(cx);
        let env_var_set = state.api_key_state.is_from_env_var();
        let env_var_name = state.api_key_state.env_var_name();
        let has_helper = state.settings.api_key_helper.is_some();
        let is_authenticated = state.is_authenticated();
        let helper_error = match &state.helper_key_status {
            HelperKeyStatus::Failed { error } => Some(error.clone()),
            _ => None,
        };

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
                        format!("You can also set the {env_var_name} environment variable and restart Zed."),
                    )
                    .size(LabelSize::Small).color(Color::Muted),
                )
                .into_any()
        } else if let Some(error) = helper_error {
            v_flex()
                .gap_2()
                .child(
                    Label::new(format!("api_key_helper failed: {error}"))
                        .color(Color::Error),
                )
                .child(
                    Button::new("retry-helper", "Retry")
                        .label_size(LabelSize::Small)
                        .layer(ElevationIndex::ModalSurface)
                        .on_click(
                            cx.listener(|this, _, window, cx| this.retry_helper(window, cx)),
                        ),
                )
                .into_any()
        } else if has_helper && !is_authenticated {
            v_flex()
                .child(Label::new("Waiting for api_key_helper to provide credentials…"))
                .into_any()
        } else {
            let status_label = if env_var_set {
                format!("API key set in {env_var_name} environment variable")
            } else if has_helper {
                "API key provided by api_key_helper".to_string()
            } else {
                format!("API key configured for {}", &state.settings.api_url)
            };
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
                        .flex_1()
                        .min_w_0()
                        .gap_1()
                        .child(Icon::new(IconName::Check).color(Color::Success))
                        .child(
                            div()
                                .w_full()
                                .overflow_x_hidden()
                                .text_ellipsis()
                                .child(Label::new(status_label))
                        ),
                )
                .child(
                    h_flex()
                        .flex_shrink_0()
                        .child(
                            Button::new("reset-api-key", "Reset API Key")
                                .label_size(LabelSize::Small)
                                .start_icon(Icon::new(IconName::Undo).size(IconSize::Small))
                                .layer(ElevationIndex::ModalSurface)
                                .when(env_var_set, |this| {
                                    this.disabled(true)
                                        .tooltip(Tooltip::text(
                                            format!("To reset your API key, unset the {env_var_name} environment variable.")
                                        ))
                                })
                                .on_click(cx.listener(|this, _, window, cx| this.reset_api_key(window, cx))),
                        ),
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

fn truncate_for_display(text: &str, max_len: usize) -> &str {
    match text.char_indices().nth(max_len) {
        Some((idx, _)) => &text[..idx],
        None => text,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::future;
    use language_model::EnvVar;
    use std::pin::Pin;

    struct NoopCredentialsProvider;

    impl CredentialsProvider for NoopCredentialsProvider {
        fn read_credentials<'a>(
            &'a self,
            _url: &'a str,
            _cx: &'a AsyncApp,
        ) -> Pin<Box<dyn Future<Output = Result<Option<(String, Vec<u8>)>>> + 'a>> {
            Box::pin(future::ready(Ok(None)))
        }

        fn write_credentials<'a>(
            &'a self,
            _url: &'a str,
            _username: &'a str,
            _password: &'a [u8],
            _cx: &'a AsyncApp,
        ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
            Box::pin(future::ready(Ok(())))
        }

        fn delete_credentials<'a>(
            &'a self,
            _url: &'a str,
            _cx: &'a AsyncApp,
        ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
            Box::pin(future::ready(Ok(())))
        }
    }

    fn test_state(helper: Option<ApiKeyHelperConfig>, status: HelperKeyStatus) -> State {
        State {
            id: Arc::from("test"),
            api_key_state: ApiKeyState::new(
                SharedString::from("http://test"),
                EnvVar::new("TEST_API_KEY".into()),
            ),
            settings: OpenAiCompatibleSettings {
                api_url: "http://test".to_string(),
                available_models: vec![],
                api_key_helper: helper,
            },
            credentials_provider: Arc::new(NoopCredentialsProvider),
            helper_key_status: status,
        }
    }

    fn helper_config() -> ApiKeyHelperConfig {
        ApiKeyHelperConfig {
            command: "echo test-key".to_string(),
            ttl_seconds: Some(60),
        }
    }

    #[test]
    fn test_needs_helper_refresh_no_helper() {
        let state = test_state(None, HelperKeyStatus::NeverRun);
        assert!(!state.needs_helper_refresh());
    }

    #[test]
    fn test_needs_helper_refresh_never_run() {
        let state = test_state(Some(helper_config()), HelperKeyStatus::NeverRun);
        assert!(state.needs_helper_refresh());
    }

    #[test]
    fn test_needs_helper_refresh_failed() {
        let state = test_state(
            Some(helper_config()),
            HelperKeyStatus::Failed {
                error: "command failed".to_string(),
            },
        );
        assert!(state.needs_helper_refresh());
    }

    #[test]
    fn test_needs_helper_refresh_running() {
        let state = test_state(
            Some(helper_config()),
            HelperKeyStatus::Running {
                task: Task::ready(Ok(())).shared(),
            },
        );
        assert!(!state.needs_helper_refresh());
    }

    #[test]
    fn test_needs_helper_refresh_succeeded_no_ttl() {
        let state = test_state(
            Some(helper_config()),
            HelperKeyStatus::Succeeded { expires_at: None },
        );
        assert!(!state.needs_helper_refresh());
    }

    #[test]
    fn test_needs_helper_refresh_succeeded_not_expired() {
        let state = test_state(
            Some(helper_config()),
            HelperKeyStatus::Succeeded {
                expires_at: Some(Instant::now() + Duration::from_secs(3600)),
            },
        );
        assert!(!state.needs_helper_refresh());
    }

    #[test]
    fn test_needs_helper_refresh_succeeded_expired() {
        let state = test_state(
            Some(helper_config()),
            HelperKeyStatus::Succeeded {
                expires_at: Some(Instant::now() - Duration::from_secs(1)),
            },
        );
        assert!(state.needs_helper_refresh());
    }

    #[test]
    fn test_is_authenticated_no_key() {
        let state = test_state(None, HelperKeyStatus::NeverRun);
        assert!(!state.is_authenticated());
    }

    #[test]
    fn test_is_authenticated_with_key_no_helper() {
        let mut state = test_state(None, HelperKeyStatus::NeverRun);
        state
            .api_key_state
            .set_key_from_helper("http://test".into(), "test-key".into());
        assert!(state.is_authenticated());
    }

    #[test]
    fn test_is_authenticated_with_key_helper_expired() {
        let mut state = test_state(
            Some(helper_config()),
            HelperKeyStatus::Succeeded {
                expires_at: Some(Instant::now() - Duration::from_secs(1)),
            },
        );
        state
            .api_key_state
            .set_key_from_helper("http://test".into(), "test-key".into());
        assert!(!state.is_authenticated());
    }

    #[test]
    fn test_is_authenticated_with_key_helper_valid() {
        let mut state = test_state(
            Some(helper_config()),
            HelperKeyStatus::Succeeded {
                expires_at: Some(Instant::now() + Duration::from_secs(3600)),
            },
        );
        state
            .api_key_state
            .set_key_from_helper("http://test".into(), "test-key".into());
        assert!(state.is_authenticated());
    }

    #[test]
    fn test_running_guard_prevents_duplicate_launch() {
        let state = test_state(
            Some(helper_config()),
            HelperKeyStatus::Running {
                task: Task::ready(Ok(())).shared(),
            },
        );
        assert!(matches!(
            state.helper_key_status,
            HelperKeyStatus::Running { .. }
        ));
        // When status is Running, needs_helper_refresh returns false,
        // and run_api_key_helper returns Ok(()) without spawning.
        assert!(!state.needs_helper_refresh());
    }

    #[test]
    fn test_reset_helper_status() {
        let mut state = test_state(
            Some(helper_config()),
            HelperKeyStatus::Succeeded {
                expires_at: Some(Instant::now() + Duration::from_secs(3600)),
            },
        );
        state.reset_helper_status();
        assert!(matches!(
            state.helper_key_status,
            HelperKeyStatus::NeverRun
        ));
    }

    #[test]
    fn test_truncate_for_display() {
        assert_eq!(truncate_for_display("hello", 10), "hello");
        assert_eq!(truncate_for_display("hello world", 5), "hello");
        assert_eq!(truncate_for_display("", 5), "");
    }
}
