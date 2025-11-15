use anyhow::{Result, anyhow};
use convert_case::{Case, Casing};
use fs::Fs;
use futures::{FutureExt, StreamExt, future, future::BoxFuture};
use gpui::{AnyView, App, AsyncApp, Context, Entity, SharedString, Task, Window};
use http_client::HttpClient;
use language_model::{
    ApiKeyState, AuthenticateError, EnvVar, IconOrSvg, LanguageModel, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest, LanguageModelToolChoice, LanguageModelToolSchemaFormat, RateLimiter,
};
use menu;
use open_ai::{ResponseStreamEvent, stream_completion};
use settings::{Settings, SettingsStore, update_settings_file};
use std::sync::Arc;
use ui::{ElevationIndex, Tooltip, prelude::*};
use ui_input::InputField;
use util::ResultExt;

use crate::provider::open_ai::{OpenAiEventMapper, into_open_ai};
pub use settings::CustomHeader;
pub use settings::OpenAiCompatibleAvailableModel as AvailableModel;
pub use settings::OpenAiCompatibleModelCapabilities as ModelCapabilities;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct OpenAiCompatibleSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
    pub custom_headers: Option<Vec<CustomHeader>>,
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
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.api_key_state.has_key()
    }

    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let api_url = SharedString::new(self.settings.api_url.as_str());
        self.api_key_state
            .store(api_url, api_key, |this| &mut this.api_key_state, cx)
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let api_url = SharedString::new(self.settings.api_url.clone());
        self.api_key_state
            .load_if_needed(api_url, |this| &mut this.api_key_state, cx)
    }
}

impl OpenAiCompatibleLanguageModelProvider {
    pub fn new(id: Arc<str>, http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
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
                if &this.settings != &settings {
                    let api_url = SharedString::new(settings.api_url.as_str());
                    this.api_key_state.handle_url_change(
                        api_url,
                        |this| &mut this.api_key_state,
                        cx,
                    );
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
        self.state
            .update(cx, |state, cx| state.set_api_key(None, cx))
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

        let Ok((api_key, api_url, custom_headers)) = self.state.read_with(cx, |state, _cx| {
            let api_url = &state.settings.api_url;
            (
                state.api_key_state.key(api_url),
                state.settings.api_url.clone(),
                state.settings.custom_headers.clone(),
            )
        }) else {
            return future::ready(Err(anyhow!("App state dropped").into())).boxed();
        };

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
                custom_headers.as_ref(),
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
        let request = into_open_ai(
            request,
            &self.model.name,
            self.model.capabilities.parallel_tool_calls,
            self.model.capabilities.prompt_cache_key,
            self.max_output_tokens(),
            None,
        );
        let completions = self.stream_completion(request, cx);
        async move {
            let mapper = OpenAiEventMapper::new();
            Ok(mapper.map_stream(completions.await?).boxed())
        }
        .boxed()
    }
}

struct ConfigurationView {
    api_key_editor: Entity<InputField>,
    state: Entity<State>,
    load_credentials_task: Option<Task<()>>,
    header_name_inputs: Vec<Entity<InputField>>,
    header_value_inputs: Vec<Entity<InputField>>,
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

        // Initialize header input fields from current settings
        let existing_headers = state
            .read(cx)
            .settings
            .custom_headers
            .clone()
            .unwrap_or_default();

        let mut header_name_inputs = Vec::with_capacity(existing_headers.len());
        let mut header_value_inputs = Vec::with_capacity(existing_headers.len());
        for h in existing_headers.iter() {
            header_name_inputs.push(cx.new(|cx| InputField::new(window, cx, &h.name)));
            header_value_inputs.push(cx.new(|cx| InputField::new(window, cx, &h.value)));
        }

        Self {
            api_key_editor,
            state,
            load_credentials_task,
            header_name_inputs,
            header_value_inputs,
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
                .update(cx, |state, cx| state.set_api_key(Some(api_key), cx))?
                .await
        })
        .detach_and_log_err(cx);
    }

    fn reset_api_key(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.api_key_editor
            .update(cx, |input, cx| input.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(None, cx))?
                .await
        })
        .detach_and_log_err(cx);
    }

    fn should_render_editor(&self, cx: &Context<Self>) -> bool {
        !self.state.read(cx).is_authenticated()
    }

    fn add_header(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.header_name_inputs
            .push(cx.new(|cx| InputField::new(window, cx, "")));
        self.header_value_inputs
            .push(cx.new(|cx| InputField::new(window, cx, "")));
    }

    fn remove_header(&mut self, index: usize) {
        if index < self.header_name_inputs.len() && index < self.header_value_inputs.len() {
            self.header_name_inputs.remove(index);
            self.header_value_inputs.remove(index);
        }
    }

    fn save_headers(&mut self, cx: &mut Context<Self>) {
        // Collect header entries from inputs
        let mut headers: Vec<CustomHeader> = Vec::new();
        for i in 0..self.header_name_inputs.len() {
            let name = self.header_name_inputs[i]
                .read(cx)
                .text(cx)
                .trim()
                .to_string();
            let value = self.header_value_inputs[i].read(cx).text(cx).to_string();
            if !name.is_empty() {
                headers.push(CustomHeader { name, value });
            }
        }

        // Optional: basic dedup by case-insensitive name; keep first occurrence
        let mut seen = std::collections::HashSet::<String>::new();
        headers.retain(|h| seen.insert(h.name.to_lowercase()));

        // Persist to settings
        let fs = <dyn Fs>::global(cx);
        let id = self.state.read(cx).id.clone();
        update_settings_file(fs, cx, move |settings, _| {
            if let Some(ref mut map) = settings
                .language_models
                .as_mut()
                .and_then(|lm| lm.openai_compatible.as_mut())
            {
                if let Some(entry) = map.get_mut(&id) {
                    entry.custom_headers = if headers.is_empty() {
                        None
                    } else {
                        Some(headers.clone())
                    };
                }
            }
        });
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.state.read(cx);
        let env_var_set = state.api_key_state.is_from_env_var();
        let env_var_name = state.api_key_state.env_var_name();

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
                        .flex_1()
                        .min_w_0()
                        .gap_1()
                        .child(Icon::new(IconName::Check).color(Color::Success))
                        .child(
                            div()
                                .w_full()
                                .overflow_x_hidden()
                                .text_ellipsis()
                                .child(Label::new(
                                    if env_var_set {
                                        format!("API key set in {env_var_name} environment variable")
                                    } else {
                                        format!("API key configured for {}", &state.settings.api_url)
                                    }
                                ))
                        ),
                )
                .child(
                    h_flex()
                        .flex_shrink_0()
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
                        ),
                )
                .into_any()
        };

        if self.load_credentials_task.is_some() {
            div().child(Label::new("Loading credentials…")).into_any()
        } else {
            // Build Custom Headers section
            let mut headers_list = v_flex().gap_1();
            for i in 0..self.header_name_inputs.len() {
                headers_list = headers_list.child(
                    h_flex()
                        .gap_1()
                        .items_center()
                        .child(
                            v_flex()
                                .flex_1()
                                .min_w_0()
                                .child(self.header_name_inputs[i].clone()),
                        )
                        .child(
                            v_flex()
                                .flex_1()
                                .min_w_0()
                                .child(self.header_value_inputs[i].clone()),
                        )
                        .child(
                            Button::new(("remove-header", i), "")
                                .style(ButtonStyle::Transparent)
                                .icon(IconName::Trash)
                                .icon_size(IconSize::Small)
                                .on_click(cx.listener({
                                    let index = i;
                                    move |this, _evt, _window, _cx| {
                                        this.remove_header(index);
                                    }
                                })),
                        ),
                );
            }

            let headers_section = v_flex()
                .mt_3()
                .gap_1()
                .child(Headline::new("Custom HTTP headers"))
                .child(
                    Label::new(
                        "These headers will be sent with each request to this provider’s API URL.",
                    )
                    .size(LabelSize::Small)
                    .color(Color::Muted),
                )
                .child(headers_list)
                .child(
                    h_flex()
                        .gap_1()
                        .justify_between()
                        .child(
                            Button::new("add-header", "Add header")
                                .style(ButtonStyle::Outlined)
                                .icon(IconName::Plus)
                                .icon_size(IconSize::Small)
                                .icon_position(IconPosition::Start)
                                .on_click(cx.listener(|this, _evt, window, cx| {
                                    this.add_header(window, cx)
                                })),
                        )
                        .child(
                            Button::new("save-headers", "Save headers")
                                .style(ButtonStyle::Filled)
                                .icon(IconName::Check)
                                .icon_size(IconSize::Small)
                                .icon_position(IconPosition::Start)
                                .on_click(
                                    cx.listener(|this, _evt, _window, cx| this.save_headers(cx)),
                                ),
                        ),
                );

            v_flex()
                .size_full()
                .on_action(cx.listener(Self::save_api_key))
                .child(api_key_section)
                .child(headers_section)
                .into_any()
        }
    }
}
