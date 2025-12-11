use crate::ExtensionSettings;
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
use extension::{LanguageModelAuthConfig, OAuthConfig};
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use futures::{FutureExt, StreamExt};
use gpui::Focusable;
use gpui::{
    AnyView, App, AppContext as _, AsyncApp, ClipboardItem, Context, Entity, EventEmitter,
    MouseButton, Subscription, Task, TextStyleRefinement, UnderlineStyle, Window, px,
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
    icon_path: Option<SharedString>,
    auth_config: Option<LanguageModelAuthConfig>,
    state: Entity<ExtensionLlmProviderState>,
}

pub struct ExtensionLlmProviderState {
    is_authenticated: bool,
    available_models: Vec<LlmModelInfo>,
    env_var_allowed: bool,
    api_key_from_env: bool,
}

impl EventEmitter<()> for ExtensionLlmProviderState {}

impl ExtensionLanguageModelProvider {
    pub fn new(
        extension: WasmExtension,
        provider_info: LlmProviderInfo,
        models: Vec<LlmModelInfo>,
        is_authenticated: bool,
        icon_path: Option<SharedString>,
        auth_config: Option<LanguageModelAuthConfig>,
        cx: &mut App,
    ) -> Self {
        let provider_id_string = format!("{}:{}", extension.manifest.id, provider_info.id);
        let env_var_allowed = ExtensionSettings::get_global(cx)
            .allowed_env_var_providers
            .contains(provider_id_string.as_str());

        let (is_authenticated, api_key_from_env) =
            if env_var_allowed && auth_config.as_ref().is_some_and(|c| c.env_var.is_some()) {
                let env_var_name = auth_config.as_ref().unwrap().env_var.as_ref().unwrap();
                if let Ok(value) = std::env::var(env_var_name) {
                    if !value.is_empty() {
                        (true, true)
                    } else {
                        (is_authenticated, false)
                    }
                } else {
                    (is_authenticated, false)
                }
            } else {
                (is_authenticated, false)
            };

        let state = cx.new(|_| ExtensionLlmProviderState {
            is_authenticated,
            available_models: models,
            env_var_allowed,
            api_key_from_env,
        });

        Self {
            extension,
            provider_info,
            icon_path,
            auth_config,
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
        LanguageModelProviderName::from(format!("(Extension) {}", self.provider_info.name))
    }

    fn icon(&self) -> ui::IconName {
        ui::IconName::ZedAssistant
    }

    fn icon_path(&self) -> Option<SharedString> {
        self.icon_path.clone()
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
        // First check cached state
        if self.state.read(cx).is_authenticated {
            return true;
        }

        // Also check env var dynamically (in case migration happened after provider creation)
        if let Some(ref auth_config) = self.auth_config {
            if let Some(ref env_var_name) = auth_config.env_var {
                let provider_id_string = self.provider_id_string();
                let env_var_allowed = ExtensionSettings::get_global(cx)
                    .allowed_env_var_providers
                    .contains(provider_id_string.as_str());

                if env_var_allowed {
                    if let Ok(value) = std::env::var(env_var_name) {
                        if !value.is_empty() {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        // Check if already authenticated via is_authenticated
        if self.is_authenticated(cx) {
            return Task::ready(Ok(()));
        }

        // Not authenticated - return error indicating credentials not found
        Task::ready(Err(AuthenticateError::CredentialsNotFound))
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
        let full_provider_id = self.provider_id_string();
        let state = self.state.clone();
        let auth_config = self.auth_config.clone();

        cx.new(|cx| {
            ExtensionProviderConfigurationView::new(
                credential_key,
                extension,
                extension_provider_id,
                full_provider_id,
                auth_config,
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
    full_provider_id: String,
    auth_config: Option<LanguageModelAuthConfig>,
    state: Entity<ExtensionLlmProviderState>,
    settings_markdown: Option<Entity<Markdown>>,
    api_key_editor: Entity<Editor>,
    loading_settings: bool,
    loading_credentials: bool,
    oauth_in_progress: bool,
    oauth_error: Option<String>,
    device_user_code: Option<String>,
    _subscriptions: Vec<Subscription>,
}

impl ExtensionProviderConfigurationView {
    fn new(
        credential_key: String,
        extension: WasmExtension,
        extension_provider_id: String,
        full_provider_id: String,
        auth_config: Option<LanguageModelAuthConfig>,
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
            full_provider_id,
            auth_config,
            state,
            settings_markdown: None,
            api_key_editor,
            loading_settings: true,
            loading_credentials: true,
            oauth_in_progress: false,
            oauth_error: None,
            device_user_code: None,
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

        // Check if we should use env var (already set in state during provider construction)
        let api_key_from_env = self.state.read(cx).api_key_from_env;

        cx.spawn(async move |this, cx| {
            // If using env var, we're already authenticated
            if api_key_from_env {
                this.update(cx, |this, cx| {
                    this.loading_credentials = false;
                    cx.notify();
                })
                .log_err();
                return;
            }

            let credentials = credentials_provider
                .read_credentials(&credential_key, cx)
                .await
                .log_err()
                .flatten();

            // Treat empty credentials as not authenticated (used as migration marker)
            let has_credentials = credentials
                .map(|(_, password)| !password.is_empty())
                .unwrap_or(false);

            // Update authentication state based on stored credentials
            cx.update(|cx| {
                state.update(cx, |state, cx| {
                    state.is_authenticated = has_credentials;
                    cx.notify();
                });
            })
            .log_err();

            this.update(cx, |this, cx| {
                this.loading_credentials = false;
                cx.notify();
            })
            .log_err();
        })
        .detach();
    }

    fn toggle_env_var_permission(&mut self, cx: &mut Context<Self>) {
        let full_provider_id: Arc<str> = self.full_provider_id.clone().into();
        let env_var_name = match &self.auth_config {
            Some(config) => config.env_var.clone(),
            None => return,
        };

        let state = self.state.clone();
        let currently_allowed = self.state.read(cx).env_var_allowed;

        // Update settings file
        settings::update_settings_file(<dyn fs::Fs>::global(cx), cx, move |settings, _| {
            let providers = settings
                .extension
                .allowed_env_var_providers
                .get_or_insert_with(Vec::new);

            if currently_allowed {
                providers.retain(|id| id.as_ref() != full_provider_id.as_ref());
            } else {
                if !providers
                    .iter()
                    .any(|id| id.as_ref() == full_provider_id.as_ref())
                {
                    providers.push(full_provider_id.clone());
                }
            }
        });

        // Update local state
        let new_allowed = !currently_allowed;
        let new_from_env = if new_allowed {
            if let Some(var_name) = &env_var_name {
                if let Ok(value) = std::env::var(var_name) {
                    !value.is_empty()
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };

        state.update(cx, |state, cx| {
            state.env_var_allowed = new_allowed;
            state.api_key_from_env = new_from_env;
            if new_from_env {
                state.is_authenticated = true;
            }
            cx.notify();
        });

        // If env var is being enabled, clear any stored keychain credentials
        // so there's only one source of truth for the API key
        if new_allowed {
            let credential_key = self.credential_key.clone();
            let credentials_provider = <dyn CredentialsProvider>::global(cx);
            cx.spawn(async move |_this, cx| {
                credentials_provider
                    .delete_credentials(&credential_key, cx)
                    .await
                    .log_err();
            })
            .detach();
        }

        // If env var is being disabled, reload credentials from keychain
        if !new_allowed {
            self.reload_keychain_credentials(cx);
        }

        cx.notify();
    }

    fn reload_keychain_credentials(&mut self, cx: &mut Context<Self>) {
        let credential_key = self.credential_key.clone();
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        let state = self.state.clone();

        cx.spawn(async move |_this, cx| {
            let credentials = credentials_provider
                .read_credentials(&credential_key, cx)
                .await
                .log_err()
                .flatten();

            // Treat empty credentials as not authenticated (used as migration marker)
            let has_credentials = credentials
                .map(|(_, password)| !password.is_empty())
                .unwrap_or(false);

            cx.update(|cx| {
                state.update(cx, |state, cx| {
                    state.is_authenticated = has_credentials;
                    cx.notify();
                });
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
            cx.update(|cx| {
                state.update(cx, |state, cx| {
                    state.is_authenticated = true;
                    cx.notify();
                });
            })
            .log_err();
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
            cx.update(|cx| {
                state.update(cx, |state, cx| {
                    state.is_authenticated = false;
                    cx.notify();
                });
            })
            .log_err();
        })
        .detach();
    }

    fn start_oauth_sign_in(&mut self, cx: &mut Context<Self>) {
        if self.oauth_in_progress {
            return;
        }

        self.oauth_in_progress = true;
        self.oauth_error = None;
        self.device_user_code = None;
        cx.notify();

        let extension = self.extension.clone();
        let provider_id = self.extension_provider_id.clone();
        let state = self.state.clone();

        cx.spawn(async move |this, cx| {
            // Step 1: Start device flow - opens browser and returns user code
            let start_result = extension
                .call({
                    let provider_id = provider_id.clone();
                    |ext, store| {
                        async move {
                            ext.call_llm_provider_start_device_flow_sign_in(store, &provider_id)
                                .await
                        }
                        .boxed()
                    }
                })
                .await;

            let user_code = match start_result {
                Ok(Ok(Ok(code))) => code,
                Ok(Ok(Err(e))) => {
                    log::error!("Device flow start failed: {}", e);
                    this.update(cx, |this, cx| {
                        this.oauth_in_progress = false;
                        this.oauth_error = Some(e);
                        cx.notify();
                    })
                    .log_err();
                    return;
                }
                Ok(Err(e)) | Err(e) => {
                    log::error!("Device flow start error: {}", e);
                    this.update(cx, |this, cx| {
                        this.oauth_in_progress = false;
                        this.oauth_error = Some(e.to_string());
                        cx.notify();
                    })
                    .log_err();
                    return;
                }
            };

            // Update UI to show the user code before polling
            this.update(cx, |this, cx| {
                this.device_user_code = Some(user_code);
                cx.notify();
            })
            .log_err();

            // Step 2: Poll for authentication completion
            let poll_result = extension
                .call({
                    let provider_id = provider_id.clone();
                    |ext, store| {
                        async move {
                            ext.call_llm_provider_poll_device_flow_sign_in(store, &provider_id)
                                .await
                        }
                        .boxed()
                    }
                })
                .await;

            let error_message = match poll_result {
                Ok(Ok(Ok(()))) => {
                    // After successful auth, refresh the models list
                    let models_result = extension
                        .call({
                            let provider_id = provider_id.clone();
                            |ext, store| {
                                async move {
                                    ext.call_llm_provider_models(store, &provider_id).await
                                }
                                .boxed()
                            }
                        })
                        .await;

                    let new_models: Vec<LlmModelInfo> = match models_result {
                        Ok(Ok(Ok(models))) => models,
                        _ => Vec::new(),
                    };

                    cx.update(|cx| {
                        state.update(cx, |state, cx| {
                            state.is_authenticated = true;
                            state.available_models = new_models;
                            cx.notify();
                        });
                    })
                    .log_err();
                    None
                }
                Ok(Ok(Err(e))) => {
                    log::error!("Device flow poll failed: {}", e);
                    Some(e)
                }
                Ok(Err(e)) | Err(e) => {
                    log::error!("Device flow poll error: {}", e);
                    Some(e.to_string())
                }
            };

            this.update(cx, |this, cx| {
                this.oauth_in_progress = false;
                this.oauth_error = error_message;
                this.device_user_code = None;
                cx.notify();
            })
            .log_err();
        })
        .detach();
    }

    fn is_authenticated(&self, cx: &Context<Self>) -> bool {
        self.state.read(cx).is_authenticated
    }

    fn has_oauth_config(&self) -> bool {
        self.auth_config.as_ref().is_some_and(|c| c.oauth.is_some())
    }

    fn oauth_config(&self) -> Option<&OAuthConfig> {
        self.auth_config.as_ref().and_then(|c| c.oauth.as_ref())
    }

    fn has_api_key_config(&self) -> bool {
        // API key is available if there's a credential_label or no oauth-only config
        self.auth_config
            .as_ref()
            .map(|c| c.credential_label.is_some() || c.oauth.is_none())
            .unwrap_or(true)
    }
}

impl gpui::Render for ExtensionProviderConfigurationView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_loading = self.loading_settings || self.loading_credentials;
        let is_authenticated = self.is_authenticated(cx);
        let env_var_allowed = self.state.read(cx).env_var_allowed;
        let api_key_from_env = self.state.read(cx).api_key_from_env;
        let has_oauth = self.has_oauth_config();
        let has_api_key = self.has_api_key_config();

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

        // Render env var checkbox if the extension specifies an env var
        if let Some(auth_config) = &self.auth_config {
            if let Some(env_var_name) = &auth_config.env_var {
                let env_var_name = env_var_name.clone();
                let checkbox_label =
                    format!("Read API key from {} environment variable", env_var_name);

                content = content.child(
                    h_flex()
                        .gap_2()
                        .child(
                            ui::Checkbox::new("env-var-permission", env_var_allowed.into())
                                .on_click(cx.listener(|this, _, _window, cx| {
                                    this.toggle_env_var_permission(cx);
                                })),
                        )
                        .child(Label::new(checkbox_label).size(LabelSize::Small)),
                );

                // Show status if env var is allowed
                if env_var_allowed {
                    if api_key_from_env {
                        content = content.child(
                            h_flex()
                                .gap_2()
                                .child(
                                    ui::Icon::new(ui::IconName::Check)
                                        .color(Color::Success)
                                        .size(ui::IconSize::Small),
                                )
                                .child(
                                    Label::new(format!("API key loaded from {}", env_var_name))
                                        .color(Color::Success),
                                ),
                        );
                        return content.into_any_element();
                    } else {
                        content = content.child(
                            h_flex()
                                .gap_2()
                                .child(
                                    ui::Icon::new(ui::IconName::Warning)
                                        .color(Color::Warning)
                                        .size(ui::IconSize::Small),
                                )
                                .child(
                                    Label::new(format!(
                                        "{} is not set or empty. You can set it and restart Zed, or use another authentication method below.",
                                        env_var_name
                                    ))
                                    .color(Color::Warning)
                                    .size(LabelSize::Small),
                                ),
                        );
                    }
                }
            }
        }

        // If authenticated, show success state with sign out option
        if is_authenticated && !api_key_from_env {
            let reset_label = if has_oauth && !has_api_key {
                "Sign Out"
            } else {
                "Reset Credentials"
            };

            let status_label = if has_oauth && !has_api_key {
                "Signed in"
            } else {
                "Authenticated"
            };

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
                            .child(Label::new(status_label).color(Color::Success)),
                    )
                    .child(
                        ui::Button::new("reset-credentials", reset_label)
                            .style(ui::ButtonStyle::Subtle)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.reset_api_key(window, cx);
                            })),
                    ),
            );

            return content.into_any_element();
        }

        // Not authenticated - show available auth options
        if !api_key_from_env {
            // Render OAuth sign-in button if configured
            if has_oauth {
                let oauth_config = self.oauth_config();
                let button_label = oauth_config
                    .and_then(|c| c.sign_in_button_label.clone())
                    .unwrap_or_else(|| "Sign In".to_string());
                let button_icon = oauth_config
                    .and_then(|c| c.sign_in_button_icon.as_ref())
                    .and_then(|icon_name| match icon_name.as_str() {
                        "github" => Some(ui::IconName::Github),
                        _ => None,
                    });

                let oauth_in_progress = self.oauth_in_progress;

                let oauth_error = self.oauth_error.clone();

                content = content.child(
                    v_flex()
                        .gap_2()
                        .child({
                            let mut button = ui::Button::new("oauth-sign-in", button_label)
                                .full_width()
                                .style(ui::ButtonStyle::Outlined)
                                .disabled(oauth_in_progress)
                                .on_click(cx.listener(|this, _, _window, cx| {
                                    this.start_oauth_sign_in(cx);
                                }));
                            if let Some(icon) = button_icon {
                                button = button
                                    .icon(icon)
                                    .icon_position(ui::IconPosition::Start)
                                    .icon_size(ui::IconSize::Small)
                                    .icon_color(Color::Muted);
                            }
                            button
                        })
                        .when(oauth_in_progress, |this| {
                            let user_code = self.device_user_code.clone();
                            this.child(
                                v_flex()
                                    .gap_1()
                                    .when_some(user_code, |this, code| {
                                        let copied = cx
                                            .read_from_clipboard()
                                            .map(|item| item.text().as_ref() == Some(&code))
                                            .unwrap_or(false);
                                        let code_for_click = code.clone();
                                        this.child(
                                            h_flex()
                                                .gap_1()
                                                .child(
                                                    Label::new("Enter code:")
                                                        .size(LabelSize::Small)
                                                        .color(Color::Muted),
                                                )
                                                .child(
                                                    h_flex()
                                                        .gap_1()
                                                        .px_1()
                                                        .border_1()
                                                        .border_color(cx.theme().colors().border)
                                                        .rounded_sm()
                                                        .cursor_pointer()
                                                        .on_mouse_down(
                                                            MouseButton::Left,
                                                            move |_, window, cx| {
                                                                cx.write_to_clipboard(
                                                                    ClipboardItem::new_string(
                                                                        code_for_click.clone(),
                                                                    ),
                                                                );
                                                                window.refresh();
                                                            },
                                                        )
                                                        .child(
                                                            Label::new(code)
                                                                .size(LabelSize::Small)
                                                                .color(Color::Accent),
                                                        )
                                                        .child(
                                                            ui::Icon::new(if copied {
                                                                ui::IconName::Check
                                                            } else {
                                                                ui::IconName::Copy
                                                            })
                                                            .size(ui::IconSize::Small)
                                                            .color(if copied {
                                                                Color::Success
                                                            } else {
                                                                Color::Muted
                                                            }),
                                                        ),
                                                ),
                                        )
                                    })
                                    .child(
                                        Label::new("Waiting for authorization in browser...")
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    ),
                            )
                        })
                        .when_some(oauth_error, |this, error| {
                            this.child(
                                v_flex()
                                    .gap_1()
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .child(
                                                ui::Icon::new(ui::IconName::Warning)
                                                    .color(Color::Error)
                                                    .size(ui::IconSize::Small),
                                            )
                                            .child(
                                                Label::new("Authentication failed")
                                                    .color(Color::Error)
                                                    .size(LabelSize::Small),
                                            ),
                                    )
                                    .child(
                                        div().pl_6().child(
                                            Label::new(error)
                                                .color(Color::Error)
                                                .size(LabelSize::Small),
                                        ),
                                    ),
                            )
                        }),
                );
            }

            // Render API key input if configured (and we have both options, show a separator)
            if has_api_key {
                if has_oauth {
                    content = content.child(
                        h_flex()
                            .gap_2()
                            .items_center()
                            .child(div().h_px().flex_1().bg(cx.theme().colors().border))
                            .child(Label::new("or").size(LabelSize::Small).color(Color::Muted))
                            .child(div().h_px().flex_1().bg(cx.theme().colors().border)),
                    );
                }

                let credential_label = self
                    .auth_config
                    .as_ref()
                    .and_then(|c| c.credential_label.clone())
                    .unwrap_or_else(|| "API Key".to_string());

                content = content.child(
                    v_flex()
                        .gap_2()
                        .on_action(cx.listener(Self::save_api_key))
                        .child(
                            Label::new(credential_label)
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
            let stream_id_result = extension
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
                .await;

            let stream_id = stream_id_result
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
