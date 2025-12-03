use crate::wasm_host::WasmExtension;

use crate::wasm_host::wit::{
    LlmCompletionEvent, LlmCompletionRequest, LlmImageData, LlmMessageContent, LlmMessageRole,
    LlmModelInfo, LlmProviderInfo, LlmRequestMessage, LlmStopReason, LlmThinkingContent,
    LlmToolChoice, LlmToolDefinition, LlmToolInputFormat, LlmToolResult, LlmToolResultContent,
    LlmToolUse,
};
use anyhow::{Result, anyhow};
use credentials_provider::CredentialsProvider;
use editor::Editor;
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use futures::{FutureExt, StreamExt};
use gpui::Focusable;
use gpui::{
    AnyView, App, AppContext as _, AsyncApp, Context, Entity, EventEmitter, Subscription, Task,
    TextStyleRefinement, UnderlineStyle, Window, px,
};
use language_model::tool_schema::LanguageModelToolSchemaFormat;
use language_model::{
    AuthenticateError, ConfigurationViewTargetAgent, LanguageModel,
    LanguageModelCacheConfiguration, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelId, LanguageModelName, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest,
    LanguageModelToolChoice, LanguageModelToolUse, LanguageModelToolUseId, StopReason, TokenUsage,
};
use markdown::{Markdown, MarkdownElement, MarkdownStyle};
use settings::Settings;
use std::sync::Arc;
use theme::ThemeSettings;
use ui::{Label, LabelSize, prelude::*};
use util::ResultExt as _;

/// An extension-based language model provider.
pub struct ExtensionLanguageModelProvider {
    pub extension: WasmExtension,
    pub provider_info: LlmProviderInfo,
    state: Entity<ExtensionLlmProviderState>,
}

pub struct ExtensionLlmProviderState {
    is_authenticated: bool,
    available_models: Vec<LlmModelInfo>,
}

impl EventEmitter<()> for ExtensionLlmProviderState {}

impl ExtensionLanguageModelProvider {
    pub fn new(
        extension: WasmExtension,
        provider_info: LlmProviderInfo,
        models: Vec<LlmModelInfo>,
        is_authenticated: bool,
        cx: &mut App,
    ) -> Self {
        let state = cx.new(|_| ExtensionLlmProviderState {
            is_authenticated,
            available_models: models,
        });

        Self {
            extension,
            provider_info,
            state,
        }
    }

    fn provider_id_string(&self) -> String {
        format!("{}:{}", self.extension.manifest.id, self.provider_info.id)
    }

    /// The credential key used for storing the API key in the system keychain.
    fn credential_key(&self) -> String {
        format!("extension-llm-{}", self.provider_id_string())
    }
}

impl LanguageModelProvider for ExtensionLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        LanguageModelProviderId::from(self.provider_id_string())
    }

    fn name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName::from(self.provider_info.name.clone())
    }

    fn icon(&self) -> ui::IconName {
        ui::IconName::ZedAssistant
    }

    fn default_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        let state = self.state.read(cx);
        state
            .available_models
            .iter()
            .find(|m| m.is_default)
            .or_else(|| state.available_models.first())
            .map(|model_info| {
                Arc::new(ExtensionLanguageModel {
                    extension: self.extension.clone(),
                    model_info: model_info.clone(),
                    provider_id: self.id(),
                    provider_name: self.name(),
                    provider_info: self.provider_info.clone(),
                }) as Arc<dyn LanguageModel>
            })
    }

    fn default_fast_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        let state = self.state.read(cx);
        state
            .available_models
            .iter()
            .find(|m| m.is_default_fast)
            .map(|model_info| {
                Arc::new(ExtensionLanguageModel {
                    extension: self.extension.clone(),
                    model_info: model_info.clone(),
                    provider_id: self.id(),
                    provider_name: self.name(),
                    provider_info: self.provider_info.clone(),
                }) as Arc<dyn LanguageModel>
            })
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let state = self.state.read(cx);
        state
            .available_models
            .iter()
            .map(|model_info| {
                Arc::new(ExtensionLanguageModel {
                    extension: self.extension.clone(),
                    model_info: model_info.clone(),
                    provider_id: self.id(),
                    provider_name: self.name(),
                    provider_info: self.provider_info.clone(),
                }) as Arc<dyn LanguageModel>
            })
            .collect()
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        let extension = self.extension.clone();
        let provider_id = self.provider_info.id.clone();
        let state = self.state.clone();

        cx.spawn(async move |cx| {
            let result = extension
                .call(|extension, store| {
                    async move {
                        extension
                            .call_llm_provider_authenticate(store, &provider_id)
                            .await
                    }
                    .boxed()
                })
                .await;

            match result {
                Ok(Ok(Ok(()))) => {
                    cx.update(|cx| {
                        state.update(cx, |state, _| {
                            state.is_authenticated = true;
                        });
                    })?;
                    Ok(())
                }
                Ok(Ok(Err(e))) => Err(AuthenticateError::Other(anyhow!("{}", e))),
                Ok(Err(e)) => Err(AuthenticateError::Other(e)),
                Err(e) => Err(AuthenticateError::Other(e)),
            }
        })
    }

    fn configuration_view(
        &self,
        _target_agent: ConfigurationViewTargetAgent,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyView {
        let credential_key = self.credential_key();
        let extension = self.extension.clone();
        let extension_provider_id = self.provider_info.id.clone();
        let state = self.state.clone();

        cx.new(|cx| {
            ExtensionProviderConfigurationView::new(
                credential_key,
                extension,
                extension_provider_id,
                state,
                window,
                cx,
            )
        })
        .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        let extension = self.extension.clone();
        let provider_id = self.provider_info.id.clone();
        let state = self.state.clone();
        let credential_key = self.credential_key();

        let credentials_provider = <dyn CredentialsProvider>::global(cx);

        cx.spawn(async move |cx| {
            // Delete from system keychain
            credentials_provider
                .delete_credentials(&credential_key, cx)
                .await
                .log_err();

            // Call extension's reset_credentials
            let result = extension
                .call(|extension, store| {
                    async move {
                        extension
                            .call_llm_provider_reset_credentials(store, &provider_id)
                            .await
                    }
                    .boxed()
                })
                .await;

            // Update state
            cx.update(|cx| {
                state.update(cx, |state, _| {
                    state.is_authenticated = false;
                });
            })?;

            match result {
                Ok(Ok(Ok(()))) => Ok(()),
                Ok(Ok(Err(e))) => Err(anyhow!("{}", e)),
                Ok(Err(e)) => Err(e),
                Err(e) => Err(e),
            }
        })
    }
}

impl LanguageModelProviderState for ExtensionLanguageModelProvider {
    type ObservableEntity = ExtensionLlmProviderState;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }

    fn subscribe<T: 'static>(
        &self,
        cx: &mut Context<T>,
        callback: impl Fn(&mut T, &mut Context<T>) + 'static,
    ) -> Option<Subscription> {
        Some(cx.subscribe(&self.state, move |this, _, _, cx| callback(this, cx)))
    }
}

/// Configuration view for extension-based LLM providers.
struct ExtensionProviderConfigurationView {
    credential_key: String,
    extension: WasmExtension,
    extension_provider_id: String,
    state: Entity<ExtensionLlmProviderState>,
    settings_markdown: Option<Entity<Markdown>>,
    api_key_editor: Entity<Editor>,
    loading_settings: bool,
    loading_credentials: bool,
    _subscriptions: Vec<Subscription>,
}

impl ExtensionProviderConfigurationView {
    fn new(
        credential_key: String,
        extension: WasmExtension,
        extension_provider_id: String,
        state: Entity<ExtensionLlmProviderState>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        // Subscribe to state changes
        let state_subscription = cx.subscribe(&state, |_, _, _, cx| {
            cx.notify();
        });

        // Create API key editor
        let api_key_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Enter API key...", window, cx);
            editor
        });

        let mut this = Self {
            credential_key,
            extension,
            extension_provider_id,
            state,
            settings_markdown: None,
            api_key_editor,
            loading_settings: true,
            loading_credentials: true,
            _subscriptions: vec![state_subscription],
        };

        // Load settings text from extension
        this.load_settings_text(cx);

        // Load existing credentials
        this.load_credentials(cx);

        this
    }

    fn load_settings_text(&mut self, cx: &mut Context<Self>) {
        let extension = self.extension.clone();
        let provider_id = self.extension_provider_id.clone();

        cx.spawn(async move |this, cx| {
            let result = extension
                .call({
                    let provider_id = provider_id.clone();
                    |ext, store| {
                        async move {
                            ext.call_llm_provider_settings_markdown(store, &provider_id)
                                .await
                        }
                        .boxed()
                    }
                })
                .await;

            let settings_text = result.ok().and_then(|inner| inner.ok()).flatten();

            this.update(cx, |this, cx| {
                this.loading_settings = false;
                if let Some(text) = settings_text {
                    let markdown = cx.new(|cx| Markdown::new(text.into(), None, None, cx));
                    this.settings_markdown = Some(markdown);
                }
                cx.notify();
            })
            .log_err();
        })
        .detach();
    }

    fn load_credentials(&mut self, cx: &mut Context<Self>) {
        let credential_key = self.credential_key.clone();
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        let state = self.state.clone();

        cx.spawn(async move |this, cx| {
            let credentials = credentials_provider
                .read_credentials(&credential_key, cx)
                .await
                .log_err()
                .flatten();

            let has_credentials = credentials.is_some();

            // Update authentication state based on stored credentials
            let _ = cx.update(|cx| {
                state.update(cx, |state, cx| {
                    state.is_authenticated = has_credentials;
                    cx.notify();
                });
            });

            this.update(cx, |this, cx| {
                this.loading_credentials = false;
                cx.notify();
            })
            .log_err();
        })
        .detach();
    }

    fn save_api_key(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let api_key = self.api_key_editor.read(cx).text(cx);
        if api_key.is_empty() {
            return;
        }

        // Clear the editor
        self.api_key_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));

        let credential_key = self.credential_key.clone();
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        let state = self.state.clone();

        cx.spawn(async move |_this, cx| {
            // Store in system keychain
            credentials_provider
                .write_credentials(&credential_key, "Bearer", api_key.as_bytes(), cx)
                .await
                .log_err();

            // Update state to authenticated
            let _ = cx.update(|cx| {
                state.update(cx, |state, cx| {
                    state.is_authenticated = true;
                    cx.notify();
                });
            });
        })
        .detach();
    }

    fn reset_api_key(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // Clear the editor
        self.api_key_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));

        let credential_key = self.credential_key.clone();
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        let state = self.state.clone();

        cx.spawn(async move |_this, cx| {
            // Delete from system keychain
            credentials_provider
                .delete_credentials(&credential_key, cx)
                .await
                .log_err();

            // Update state to unauthenticated
            let _ = cx.update(|cx| {
                state.update(cx, |state, cx| {
                    state.is_authenticated = false;
                    cx.notify();
                });
            });
        })
        .detach();
    }

    fn is_authenticated(&self, cx: &Context<Self>) -> bool {
        self.state.read(cx).is_authenticated
    }
}

impl gpui::Render for ExtensionProviderConfigurationView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_loading = self.loading_settings || self.loading_credentials;
        let is_authenticated = self.is_authenticated(cx);

        if is_loading {
            return v_flex()
                .gap_2()
                .child(Label::new("Loading...").color(Color::Muted))
                .into_any_element();
        }

        let mut content = v_flex().gap_4().size_full();

        // Render settings markdown if available
        if let Some(markdown) = &self.settings_markdown {
            let style = settings_markdown_style(_window, cx);
            content = content.child(
                div()
                    .p_2()
                    .rounded_md()
                    .bg(cx.theme().colors().surface_background)
                    .child(MarkdownElement::new(markdown.clone(), style)),
            );
        }

        // Render API key section
        if is_authenticated {
            content = content.child(
                v_flex()
                    .gap_2()
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                ui::Icon::new(ui::IconName::Check)
                                    .color(Color::Success)
                                    .size(ui::IconSize::Small),
                            )
                            .child(Label::new("API key configured").color(Color::Success)),
                    )
                    .child(
                        ui::Button::new("reset-api-key", "Reset API Key")
                            .style(ui::ButtonStyle::Subtle)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.reset_api_key(window, cx);
                            })),
                    ),
            );
        } else {
            content = content.child(
                v_flex()
                    .gap_2()
                    .on_action(cx.listener(Self::save_api_key))
                    .child(
                        Label::new("API Key")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .child(self.api_key_editor.clone())
                    .child(
                        Label::new("Enter your API key and press Enter to save")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
            );
        }

        content.into_any_element()
    }
}

impl Focusable for ExtensionProviderConfigurationView {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.api_key_editor.focus_handle(cx)
    }
}

fn settings_markdown_style(window: &Window, cx: &App) -> MarkdownStyle {
    let theme_settings = ThemeSettings::get_global(cx);
    let colors = cx.theme().colors();
    let mut text_style = window.text_style();
    text_style.refine(&TextStyleRefinement {
        font_family: Some(theme_settings.ui_font.family.clone()),
        font_fallbacks: theme_settings.ui_font.fallbacks.clone(),
        font_features: Some(theme_settings.ui_font.features.clone()),
        color: Some(colors.text),
        ..Default::default()
    });

    MarkdownStyle {
        base_text_style: text_style,
        selection_background_color: colors.element_selection_background,
        inline_code: TextStyleRefinement {
            background_color: Some(colors.editor_background),
            ..Default::default()
        },
        link: TextStyleRefinement {
            color: Some(colors.text_accent),
            underline: Some(UnderlineStyle {
                color: Some(colors.text_accent.opacity(0.5)),
                thickness: px(1.),
                ..Default::default()
            }),
            ..Default::default()
        },
        syntax: cx.theme().syntax().clone(),
        ..Default::default()
    }
}

/// An extension-based language model.
pub struct ExtensionLanguageModel {
    extension: WasmExtension,
    model_info: LlmModelInfo,
    provider_id: LanguageModelProviderId,
    provider_name: LanguageModelProviderName,
    provider_info: LlmProviderInfo,
}

impl LanguageModel for ExtensionLanguageModel {
    fn id(&self) -> LanguageModelId {
        LanguageModelId::from(self.model_info.id.clone())
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(self.model_info.name.clone())
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        self.provider_id.clone()
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        self.provider_name.clone()
    }

    fn telemetry_id(&self) -> String {
        format!("extension-{}", self.model_info.id)
    }

    fn supports_images(&self) -> bool {
        self.model_info.capabilities.supports_images
    }

    fn supports_tools(&self) -> bool {
        self.model_info.capabilities.supports_tools
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto => self.model_info.capabilities.supports_tool_choice_auto,
            LanguageModelToolChoice::Any => self.model_info.capabilities.supports_tool_choice_any,
            LanguageModelToolChoice::None => self.model_info.capabilities.supports_tool_choice_none,
        }
    }

    fn tool_input_format(&self) -> LanguageModelToolSchemaFormat {
        match self.model_info.capabilities.tool_input_format {
            LlmToolInputFormat::JsonSchema => LanguageModelToolSchemaFormat::JsonSchema,
            LlmToolInputFormat::Simplified => LanguageModelToolSchemaFormat::JsonSchema,
        }
    }

    fn max_token_count(&self) -> u64 {
        self.model_info.max_token_count
    }

    fn max_output_tokens(&self) -> Option<u64> {
        self.model_info.max_output_tokens
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &App,
    ) -> BoxFuture<'static, Result<u64>> {
        let extension = self.extension.clone();
        let provider_id = self.provider_info.id.clone();
        let model_id = self.model_info.id.clone();

        let wit_request = convert_request_to_wit(request);

        cx.background_spawn(async move {
            extension
                .call({
                    let provider_id = provider_id.clone();
                    let model_id = model_id.clone();
                    let wit_request = wit_request.clone();
                    |ext, store| {
                        async move {
                            let count = ext
                                .call_llm_count_tokens(store, &provider_id, &model_id, &wit_request)
                                .await?
                                .map_err(|e| anyhow!("{}", e))?;
                            Ok(count)
                        }
                        .boxed()
                    }
                })
                .await?
        })
        .boxed()
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        _cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            BoxStream<'static, Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>,
            LanguageModelCompletionError,
        >,
    > {
        let extension = self.extension.clone();
        let provider_id = self.provider_info.id.clone();
        let model_id = self.model_info.id.clone();

        let wit_request = convert_request_to_wit(request);

        async move {
            // Start the stream
            let stream_id = extension
                .call({
                    let provider_id = provider_id.clone();
                    let model_id = model_id.clone();
                    let wit_request = wit_request.clone();
                    |ext, store| {
                        async move {
                            let id = ext
                                .call_llm_stream_completion_start(
                                    store,
                                    &provider_id,
                                    &model_id,
                                    &wit_request,
                                )
                                .await?
                                .map_err(|e| anyhow!("{}", e))?;
                            Ok(id)
                        }
                        .boxed()
                    }
                })
                .await
                .map_err(LanguageModelCompletionError::Other)?
                .map_err(LanguageModelCompletionError::Other)?;

            // Create a stream that polls for events
            let stream = futures::stream::unfold(
                (extension.clone(), stream_id, false),
                move |(extension, stream_id, done)| async move {
                    if done {
                        return None;
                    }

                    let result = extension
                        .call({
                            let stream_id = stream_id.clone();
                            |ext, store| {
                                async move {
                                    let event = ext
                                        .call_llm_stream_completion_next(store, &stream_id)
                                        .await?
                                        .map_err(|e| anyhow!("{}", e))?;
                                    Ok(event)
                                }
                                .boxed()
                            }
                        })
                        .await
                        .and_then(|inner| inner);

                    match result {
                        Ok(Some(event)) => {
                            let converted = convert_completion_event(event);
                            let is_done =
                                matches!(&converted, Ok(LanguageModelCompletionEvent::Stop(_)));
                            Some((converted, (extension, stream_id, is_done)))
                        }
                        Ok(None) => {
                            // Stream complete, close it
                            let _ = extension
                                .call({
                                    let stream_id = stream_id.clone();
                                    |ext, store| {
                                        async move {
                                            ext.call_llm_stream_completion_close(store, &stream_id)
                                                .await?;
                                            Ok::<(), anyhow::Error>(())
                                        }
                                        .boxed()
                                    }
                                })
                                .await;
                            None
                        }
                        Err(e) => Some((
                            Err(LanguageModelCompletionError::Other(e)),
                            (extension, stream_id, true),
                        )),
                    }
                },
            );

            Ok(stream.boxed())
        }
        .boxed()
    }

    fn cache_configuration(&self) -> Option<LanguageModelCacheConfiguration> {
        // Extensions can implement this via llm_cache_configuration
        None
    }
}

fn convert_request_to_wit(request: LanguageModelRequest) -> LlmCompletionRequest {
    use language_model::{MessageContent, Role};

    let messages: Vec<LlmRequestMessage> = request
        .messages
        .into_iter()
        .map(|msg| {
            let role = match msg.role {
                Role::User => LlmMessageRole::User,
                Role::Assistant => LlmMessageRole::Assistant,
                Role::System => LlmMessageRole::System,
            };

            let content: Vec<LlmMessageContent> = msg
                .content
                .into_iter()
                .map(|c| match c {
                    MessageContent::Text(text) => LlmMessageContent::Text(text),
                    MessageContent::Image(image) => LlmMessageContent::Image(LlmImageData {
                        source: image.source.to_string(),
                        width: Some(image.size.width.0 as u32),
                        height: Some(image.size.height.0 as u32),
                    }),
                    MessageContent::ToolUse(tool_use) => LlmMessageContent::ToolUse(LlmToolUse {
                        id: tool_use.id.to_string(),
                        name: tool_use.name.to_string(),
                        input: serde_json::to_string(&tool_use.input).unwrap_or_default(),
                        thought_signature: tool_use.thought_signature,
                    }),
                    MessageContent::ToolResult(tool_result) => {
                        let content = match tool_result.content {
                            language_model::LanguageModelToolResultContent::Text(text) => {
                                LlmToolResultContent::Text(text.to_string())
                            }
                            language_model::LanguageModelToolResultContent::Image(image) => {
                                LlmToolResultContent::Image(LlmImageData {
                                    source: image.source.to_string(),
                                    width: Some(image.size.width.0 as u32),
                                    height: Some(image.size.height.0 as u32),
                                })
                            }
                        };
                        LlmMessageContent::ToolResult(LlmToolResult {
                            tool_use_id: tool_result.tool_use_id.to_string(),
                            tool_name: tool_result.tool_name.to_string(),
                            is_error: tool_result.is_error,
                            content,
                        })
                    }
                    MessageContent::Thinking { text, signature } => {
                        LlmMessageContent::Thinking(LlmThinkingContent { text, signature })
                    }
                    MessageContent::RedactedThinking(data) => {
                        LlmMessageContent::RedactedThinking(data)
                    }
                })
                .collect();

            LlmRequestMessage {
                role,
                content,
                cache: msg.cache,
            }
        })
        .collect();

    let tools: Vec<LlmToolDefinition> = request
        .tools
        .into_iter()
        .map(|tool| LlmToolDefinition {
            name: tool.name,
            description: tool.description,
            input_schema: serde_json::to_string(&tool.input_schema).unwrap_or_default(),
        })
        .collect();

    let tool_choice = request.tool_choice.map(|tc| match tc {
        LanguageModelToolChoice::Auto => LlmToolChoice::Auto,
        LanguageModelToolChoice::Any => LlmToolChoice::Any,
        LanguageModelToolChoice::None => LlmToolChoice::None,
    });

    LlmCompletionRequest {
        messages,
        tools,
        tool_choice,
        stop_sequences: request.stop,
        temperature: request.temperature,
        thinking_allowed: false,
        max_tokens: None,
    }
}

fn convert_completion_event(
    event: LlmCompletionEvent,
) -> Result<LanguageModelCompletionEvent, LanguageModelCompletionError> {
    match event {
        LlmCompletionEvent::Started => Ok(LanguageModelCompletionEvent::StartMessage {
            message_id: String::new(),
        }),
        LlmCompletionEvent::Text(text) => Ok(LanguageModelCompletionEvent::Text(text)),
        LlmCompletionEvent::Thinking(thinking) => Ok(LanguageModelCompletionEvent::Thinking {
            text: thinking.text,
            signature: thinking.signature,
        }),
        LlmCompletionEvent::RedactedThinking(data) => {
            Ok(LanguageModelCompletionEvent::RedactedThinking { data })
        }
        LlmCompletionEvent::ToolUse(tool_use) => {
            let raw_input = tool_use.input.clone();
            let input = serde_json::from_str(&tool_use.input).unwrap_or(serde_json::Value::Null);
            Ok(LanguageModelCompletionEvent::ToolUse(
                LanguageModelToolUse {
                    id: LanguageModelToolUseId::from(tool_use.id),
                    name: tool_use.name.into(),
                    raw_input,
                    input,
                    is_input_complete: true,
                    thought_signature: tool_use.thought_signature,
                },
            ))
        }
        LlmCompletionEvent::ToolUseJsonParseError(error) => {
            Ok(LanguageModelCompletionEvent::ToolUseJsonParseError {
                id: LanguageModelToolUseId::from(error.id),
                tool_name: error.tool_name.into(),
                raw_input: error.raw_input.into(),
                json_parse_error: error.error,
            })
        }
        LlmCompletionEvent::Stop(reason) => {
            let stop_reason = match reason {
                LlmStopReason::EndTurn => StopReason::EndTurn,
                LlmStopReason::MaxTokens => StopReason::MaxTokens,
                LlmStopReason::ToolUse => StopReason::ToolUse,
                LlmStopReason::Refusal => StopReason::Refusal,
            };
            Ok(LanguageModelCompletionEvent::Stop(stop_reason))
        }
        LlmCompletionEvent::Usage(usage) => {
            Ok(LanguageModelCompletionEvent::UsageUpdate(TokenUsage {
                input_tokens: usage.input_tokens,
                output_tokens: usage.output_tokens,
                cache_creation_input_tokens: usage.cache_creation_input_tokens.unwrap_or(0),
                cache_read_input_tokens: usage.cache_read_input_tokens.unwrap_or(0),
            }))
        }
        LlmCompletionEvent::ReasoningDetails(json) => {
            Ok(LanguageModelCompletionEvent::ReasoningDetails(
                serde_json::from_str(&json).unwrap_or(serde_json::Value::Null),
            ))
        }
    }
}
