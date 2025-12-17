use crate::ExtensionSettings;
use crate::LEGACY_LLM_EXTENSION_IDS;
use crate::wasm_host::WasmExtension;
use crate::wasm_host::wit::LlmDeviceFlowPromptInfo;
use collections::HashSet;

use crate::wasm_host::wit::{
    LlmCompletionEvent, LlmCompletionRequest, LlmImageData, LlmMessageContent, LlmMessageRole,
    LlmModelInfo, LlmProviderInfo, LlmRequestMessage, LlmStopReason, LlmThinkingContent,
    LlmToolChoice, LlmToolDefinition, LlmToolInputFormat, LlmToolResult, LlmToolResultContent,
    LlmToolUse,
};
use anyhow::{Result, anyhow};
use credentials_provider::CredentialsProvider;
use extension::{LanguageModelAuthConfig, OAuthConfig};
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use futures::{FutureExt, StreamExt};

use gpui::{
    AnyView, App, AsyncApp, ClipboardItem, DismissEvent, Entity, EventEmitter, FocusHandle,
    Focusable, MouseDownEvent, Subscription, Task, TextStyleRefinement, UnderlineStyle, Window,
    WindowBounds, WindowOptions, point, prelude::*, px,
};
use language_model::tool_schema::LanguageModelToolSchemaFormat;
use language_model::{
    AuthenticateError, ConfigurationViewTargetAgent, LanguageModel,
    LanguageModelCacheConfiguration, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelId, LanguageModelName, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest,
    LanguageModelToolChoice, LanguageModelToolUse, LanguageModelToolUseId, RateLimiter,
    StopReason, TokenUsage,
};
use markdown::{HeadingLevelStyles, Markdown, MarkdownElement, MarkdownStyle};
use settings::Settings;
use std::sync::Arc;
use theme::ThemeSettings;
use ui::{
    ButtonLike, ButtonLink, Checkbox, ConfiguredApiCard, SpinnerLabel, ToggleState, Vector,
    VectorName, prelude::*,
};
use ui_input::InputField;
use util::ResultExt as _;
use workspace::Workspace;
use workspace::oauth_device_flow_modal::{
    OAuthDeviceFlowModal, OAuthDeviceFlowModalConfig, OAuthDeviceFlowState, OAuthDeviceFlowStatus,
};

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
    /// Set of env var names that are allowed to be read for this provider.
    allowed_env_vars: HashSet<String>,
    /// If authenticated via env var, which one was used.
    env_var_name_used: Option<String>,
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

        // Build set of allowed env vars for this provider
        let settings = ExtensionSettings::get_global(cx);
        let is_legacy_extension =
            LEGACY_LLM_EXTENSION_IDS.contains(&extension.manifest.id.as_ref());

        let mut allowed_env_vars = HashSet::default();
        if let Some(env_vars) = auth_config.as_ref().and_then(|c| c.env_vars.as_ref()) {
            for env_var_name in env_vars {
                let key = format!("{}:{}", provider_id_string, env_var_name);
                // For legacy extensions, auto-allow if env var is set (migration will persist this)
                let env_var_is_set = std::env::var(env_var_name)
                    .map(|v| !v.is_empty())
                    .unwrap_or(false);
                if settings.allowed_env_var_providers.contains(key.as_str())
                    || (is_legacy_extension && env_var_is_set)
                {
                    allowed_env_vars.insert(env_var_name.clone());
                }
            }
        }

        // Check if any allowed env var is set
        let env_var_name_used = allowed_env_vars.iter().find_map(|env_var_name| {
            if let Ok(value) = std::env::var(env_var_name) {
                if !value.is_empty() {
                    return Some(env_var_name.clone());
                }
            }
            None
        });

        let is_authenticated = if env_var_name_used.is_some() {
            true
        } else {
            is_authenticated
        };

        let state = cx.new(|_| ExtensionLlmProviderState {
            is_authenticated,
            available_models: models,
            allowed_env_vars,
            env_var_name_used,
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
        LanguageModelProviderName::from(self.provider_info.name.clone())
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
                    request_limiter: RateLimiter::new(4),
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
                    request_limiter: RateLimiter::new(4),
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
                    request_limiter: RateLimiter::new(4),
                }) as Arc<dyn LanguageModel>
            })
            .collect()
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        // First check cached state
        if self.state.read(cx).is_authenticated {
            return true;
        }

        // Also check env var dynamically (in case settings changed after provider creation)
        if let Some(ref auth_config) = self.auth_config {
            if let Some(ref env_vars) = auth_config.env_vars {
                let provider_id_string = self.provider_id_string();
                let settings = ExtensionSettings::get_global(cx);
                let is_legacy_extension =
                    LEGACY_LLM_EXTENSION_IDS.contains(&self.extension.manifest.id.as_ref());

                for env_var_name in env_vars {
                    let key = format!("{}:{}", provider_id_string, env_var_name);
                    // For legacy extensions, auto-allow if env var is set
                    let env_var_is_set = std::env::var(env_var_name)
                        .map(|v| !v.is_empty())
                        .unwrap_or(false);
                    if settings.allowed_env_var_providers.contains(key.as_str())
                        || (is_legacy_extension && env_var_is_set)
                    {
                        if let Ok(value) = std::env::var(env_var_name) {
                            if !value.is_empty() {
                                return true;
                            }
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
        target_agent: ConfigurationViewTargetAgent,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyView {
        let credential_key = self.credential_key();
        let extension = self.extension.clone();
        let extension_provider_id = self.provider_info.id.clone();
        let full_provider_id = self.provider_id_string();
        let state = self.state.clone();
        let auth_config = self.auth_config.clone();

        let icon_path = self.icon_path.clone();
        cx.new(|cx| {
            ExtensionProviderConfigurationView::new(
                credential_key,
                extension,
                extension_provider_id,
                full_provider_id,
                auth_config,
                state,
                icon_path,
                target_agent,
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
    api_key_editor: Entity<InputField>,
    loading_settings: bool,
    loading_credentials: bool,
    oauth_in_progress: bool,
    oauth_error: Option<String>,
    icon_path: Option<SharedString>,
    target_agent: ConfigurationViewTargetAgent,
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
        icon_path: Option<SharedString>,
        target_agent: ConfigurationViewTargetAgent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let state_subscription = cx.subscribe(&state, |_, _, _, cx| {
            cx.notify();
        });

        let credential_label = auth_config
            .as_ref()
            .and_then(|c| c.credential_label.clone())
            .unwrap_or_else(|| "API Key".to_string());

        let api_key_editor = cx.new(|cx| {
            InputField::new(window, cx, "Enter API key and hit enter").label(credential_label)
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
            icon_path,
            target_agent,
            _subscriptions: vec![state_subscription],
        };

        this.load_settings_text(cx);
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
        let using_env_var = self.state.read(cx).env_var_name_used.is_some();

        cx.spawn(async move |this, cx| {
            // If using env var, we're already authenticated
            if using_env_var {
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

            let has_credentials = credentials.is_some();

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

    fn toggle_env_var_permission(&mut self, env_var_name: String, cx: &mut Context<Self>) {
        let full_provider_id = self.full_provider_id.clone();
        let settings_key: Arc<str> = format!("{}:{}", full_provider_id, env_var_name).into();

        let state = self.state.clone();
        let currently_allowed = self.state.read(cx).allowed_env_vars.contains(&env_var_name);

        // Update settings file
        settings::update_settings_file(<dyn fs::Fs>::global(cx), cx, {
            move |settings, _| {
                let allowed = settings
                    .extension
                    .allowed_env_var_providers
                    .get_or_insert_with(Vec::new);

                if currently_allowed {
                    allowed.retain(|id| id.as_ref() != settings_key.as_ref());
                } else {
                    if !allowed
                        .iter()
                        .any(|id| id.as_ref() == settings_key.as_ref())
                    {
                        allowed.push(settings_key.clone());
                    }
                }
            }
        });

        // Update local state
        let new_allowed = !currently_allowed;

        state.update(cx, |state, cx| {
            if new_allowed {
                state.allowed_env_vars.insert(env_var_name.clone());
                // Check if this env var is set and update env_var_name_used
                if let Ok(value) = std::env::var(&env_var_name) {
                    if !value.is_empty() && state.env_var_name_used.is_none() {
                        state.env_var_name_used = Some(env_var_name.clone());
                        state.is_authenticated = true;
                    }
                }
            } else {
                state.allowed_env_vars.remove(&env_var_name);
                // If this was the env var being used, clear it and find another
                if state.env_var_name_used.as_ref() == Some(&env_var_name) {
                    state.env_var_name_used = state.allowed_env_vars.iter().find_map(|var| {
                        if let Ok(value) = std::env::var(var) {
                            if !value.is_empty() {
                                return Some(var.clone());
                            }
                        }
                        None
                    });
                    if state.env_var_name_used.is_none() {
                        // No env var auth available, need to check keychain
                        state.is_authenticated = false;
                    }
                }
            }
            cx.notify();
        });

        // If all env vars are being disabled, reload credentials from keychain
        if !new_allowed && self.state.read(cx).allowed_env_vars.is_empty() {
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

            let has_credentials = credentials.is_some();

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
            .update(cx, |input, cx| input.clear(window, cx));

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
            .update(cx, |input, cx| input.clear(window, cx));

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

    fn start_oauth_sign_in(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.oauth_in_progress {
            return;
        }

        self.oauth_in_progress = true;
        self.oauth_error = None;
        cx.notify();

        let extension = self.extension.clone();
        let provider_id = self.extension_provider_id.clone();
        let state = self.state.clone();
        let icon_path = self.icon_path.clone();
        let this_handle = cx.weak_entity();
        let use_popup_window = self.is_edit_prediction_mode();

        // Get current window bounds for positioning popup
        let current_window_center = window.bounds().center();

        // For workspace modal mode, find the workspace window
        let workspace_window = if !use_popup_window {
            log::info!("OAuth: Looking for workspace window");
            let ws = window.window_handle().downcast::<Workspace>().or_else(|| {
                log::info!("OAuth: Current window is not a workspace, searching other windows");
                cx.windows()
                    .into_iter()
                    .find_map(|window_handle| window_handle.downcast::<Workspace>())
            });

            if ws.is_none() {
                log::error!("OAuth: Could not find any workspace window");
                self.oauth_in_progress = false;
                self.oauth_error =
                    Some("Could not access workspace to show sign-in modal".to_string());
                cx.notify();
                return;
            }
            ws
        } else {
            None
        };

        log::info!(
            "OAuth: Using {} mode",
            if use_popup_window {
                "popup window"
            } else {
                "workspace modal"
            }
        );
        let state = state.downgrade();
        cx.spawn(async move |_this, cx| {
            // Step 1: Start device flow - get prompt info from extension
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

            log::info!(
                "OAuth: Device flow start result: {:?}",
                start_result.is_ok()
            );
            let prompt_info: LlmDeviceFlowPromptInfo = match start_result {
                Ok(Ok(Ok(info))) => {
                    log::info!(
                        "OAuth: Got device flow prompt info, user_code: {}",
                        info.user_code
                    );
                    info
                }
                Ok(Ok(Err(e))) => {
                    log::error!("OAuth: Device flow start failed: {}", e);
                    this_handle
                        .update(cx, |this, cx| {
                            this.oauth_in_progress = false;
                            this.oauth_error = Some(e);
                            cx.notify();
                        })
                        .log_err();
                    return;
                }
                Ok(Err(e)) | Err(e) => {
                    log::error!("OAuth: Device flow start error: {}", e);
                    this_handle
                        .update(cx, |this, cx| {
                            this.oauth_in_progress = false;
                            this.oauth_error = Some(e.to_string());
                            cx.notify();
                        })
                        .log_err();
                    return;
                }
            };

            // Step 2: Create state entity and show the modal/window
            let modal_config = OAuthDeviceFlowModalConfig {
                user_code: prompt_info.user_code,
                verification_url: prompt_info.verification_url,
                headline: prompt_info.headline,
                description: prompt_info.description,
                connect_button_label: prompt_info.connect_button_label,
                success_headline: prompt_info.success_headline,
                success_message: prompt_info.success_message,
                icon_path,
            };

            let flow_state: Option<Entity<OAuthDeviceFlowState>> = if use_popup_window {
                // Open a popup window like Copilot does
                log::info!("OAuth: Opening popup window");
                cx.update(|cx| {
                    let height = px(450.);
                    let width = px(350.);
                    let window_bounds = WindowBounds::Windowed(gpui::bounds(
                        current_window_center - point(height / 2.0, width / 2.0),
                        gpui::size(height, width),
                    ));

                    let flow_state = cx.new(|_cx| OAuthDeviceFlowState::new(modal_config.clone()));
                    let flow_state_for_window = flow_state.clone();

                    cx.open_window(
                        WindowOptions {
                            kind: gpui::WindowKind::PopUp,
                            window_bounds: Some(window_bounds),
                            is_resizable: false,
                            is_movable: true,
                            titlebar: Some(gpui::TitlebarOptions {
                                appears_transparent: true,
                                ..Default::default()
                            }),
                            ..Default::default()
                        },
                        |window, cx| {
                            cx.new(|cx| {
                                OAuthCodeVerificationWindow::new(
                                    modal_config,
                                    flow_state_for_window,
                                    window,
                                    cx,
                                )
                            })
                        },
                    )
                    .log_err();

                    Some(flow_state)
                })
                .ok()
                .flatten()
            } else {
                // Use workspace modal
                log::info!("OAuth: Attempting to show modal in workspace window");
                workspace_window.as_ref().and_then(|ws| {
                    ws.update(cx, |workspace, window, cx| {
                        log::info!("OAuth: Inside workspace.update, creating modal");
                        window.activate_window();
                        let flow_state = cx.new(|_cx| OAuthDeviceFlowState::new(modal_config));
                        let flow_state_clone = flow_state.clone();
                        workspace.toggle_modal(window, cx, |_window, cx| {
                            log::info!("OAuth: Inside toggle_modal callback");
                            OAuthDeviceFlowModal::new(flow_state_clone, cx)
                        });
                        flow_state
                    })
                    .ok()
                })
            };

            log::info!("OAuth: flow_state created: {:?}", flow_state.is_some());
            let Some(flow_state) = flow_state else {
                log::error!("OAuth: Failed to show sign-in modal/window");
                this_handle
                    .update(cx, |this, cx| {
                        this.oauth_in_progress = false;
                        this.oauth_error = Some("Failed to show sign-in modal".to_string());
                        cx.notify();
                    })
                    .log_err();
                return;
            };
            log::info!("OAuth: Modal/window shown successfully, starting poll");

            // Step 3: Poll for authentication completion
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

            match poll_result {
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

                    state
                        .update(cx, |state, cx| {
                            state.is_authenticated = true;
                            state.available_models = new_models;
                            cx.notify();
                        })
                        .log_err();

                    // Update flow state to show success
                    flow_state
                        .update(cx, |state, cx| {
                            state.set_status(OAuthDeviceFlowStatus::Authorized, cx);
                        })
                        .log_err();
                }
                Ok(Ok(Err(e))) => {
                    log::error!("Device flow poll failed: {}", e);
                    flow_state
                        .update(cx, |state, cx| {
                            state.set_status(OAuthDeviceFlowStatus::Failed(e.clone()), cx);
                        })
                        .log_err();
                    this_handle
                        .update(cx, |this, cx| {
                            this.oauth_error = Some(e);
                            cx.notify();
                        })
                        .log_err();
                }
                Ok(Err(e)) | Err(e) => {
                    log::error!("Device flow poll error: {}", e);
                    let error_string = e.to_string();
                    flow_state
                        .update(cx, |state, cx| {
                            state.set_status(
                                OAuthDeviceFlowStatus::Failed(error_string.clone()),
                                cx,
                            );
                        })
                        .log_err();
                    this_handle
                        .update(cx, |this, cx| {
                            this.oauth_error = Some(error_string);
                            cx.notify();
                        })
                        .log_err();
                }
            };

            this_handle
                .update(cx, |this, cx| {
                    this.oauth_in_progress = false;
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

    fn is_edit_prediction_mode(&self) -> bool {
        self.target_agent == ConfigurationViewTargetAgent::EditPrediction
    }

    fn render_for_edit_prediction(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_loading = self.loading_settings || self.loading_credentials;
        let is_authenticated = self.is_authenticated(cx);
        let has_oauth = self.has_oauth_config();

        // Helper to create the horizontal container layout matching Copilot
        let container = |description: SharedString, action: AnyElement| {
            h_flex()
                .pt_2p5()
                .w_full()
                .justify_between()
                .child(
                    v_flex()
                        .w_full()
                        .max_w_1_2()
                        .child(Label::new("Authenticate To Use"))
                        .child(
                            Label::new(description)
                                .color(Color::Muted)
                                .size(LabelSize::Small),
                        ),
                )
                .child(action)
        };

        // Get the description from OAuth config or use a default
        let oauth_config = self.oauth_config();
        let description: SharedString = oauth_config
            .and_then(|c| c.sign_in_description.clone())
            .unwrap_or_else(|| "Sign in to authenticate with this provider.".to_string())
            .into();

        if is_loading {
            return container(
                description,
                Button::new("loading", "Loading...")
                    .style(ButtonStyle::Outlined)
                    .disabled(true)
                    .into_any_element(),
            )
            .into_any_element();
        }

        // If authenticated, show the configured card
        if is_authenticated {
            let (status_label, button_label) = if has_oauth {
                ("Authorized", "Sign Out")
            } else {
                ("API key configured", "Reset Key")
            };

            return ConfiguredApiCard::new(status_label)
                .button_label(button_label)
                .on_click(cx.listener(|this, _, window, cx| {
                    this.reset_api_key(window, cx);
                }))
                .into_any_element();
        }

        // Not authenticated - show sign in button
        if has_oauth {
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

            let mut button = Button::new("oauth-sign-in", button_label)
                .size(ButtonSize::Medium)
                .style(ButtonStyle::Outlined)
                .disabled(oauth_in_progress)
                .on_click(cx.listener(|this, _, window, cx| {
                    this.start_oauth_sign_in(window, cx);
                }));

            if let Some(icon) = button_icon {
                button = button
                    .icon(icon)
                    .icon_position(ui::IconPosition::Start)
                    .icon_size(ui::IconSize::Small)
                    .icon_color(Color::Muted);
            }

            return container(description, button.into_any_element()).into_any_element();
        }

        // Fallback for API key only providers - show a simple message
        container(
            description,
            Button::new("configure", "Configure")
                .size(ButtonSize::Medium)
                .style(ButtonStyle::Outlined)
                .disabled(true)
                .into_any_element(),
        )
        .into_any_element()
    }
}

impl Render for ExtensionProviderConfigurationView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_edit_prediction_mode() {
            return self
                .render_for_edit_prediction(window, cx)
                .into_any_element();
        }

        let is_loading = self.loading_settings || self.loading_credentials;
        let is_authenticated = self.is_authenticated(cx);
        let allowed_env_vars = self.state.read(cx).allowed_env_vars.clone();
        let env_var_name_used = self.state.read(cx).env_var_name_used.clone();
        let has_oauth = self.has_oauth_config();
        let has_api_key = self.has_api_key_config();

        if is_loading {
            return h_flex()
                .gap_2()
                .child(
                    h_flex()
                        .w_2()
                        .child(SpinnerLabel::sand().size(LabelSize::Small)),
                )
                .child(LoadingLabel::new("Loading").size(LabelSize::Small))
                .into_any_element();
        }

        let mut content = v_flex().size_full().gap_2();

        if let Some(markdown) = &self.settings_markdown {
            content = content.text_sm().child(MarkdownElement::new(
                markdown.clone(),
                markdown_styles(window, cx),
            ));
        }

        if let Some(auth_config) = &self.auth_config {
            if let Some(env_vars) = &auth_config.env_vars {
                for env_var_name in env_vars {
                    let is_allowed = allowed_env_vars.contains(env_var_name);
                    let checkbox_label =
                        format!("Read API key from {} environment variable.", env_var_name);
                    let env_var_for_click = env_var_name.clone();

                    content = content.child(
                        Checkbox::new(
                            SharedString::from(format!("env-var-{}", env_var_name)),
                            if is_allowed {
                                ToggleState::Selected
                            } else {
                                ToggleState::Unselected
                            },
                        )
                        .label(checkbox_label)
                        .on_click(cx.listener(
                            move |this, _, _window, cx| {
                                this.toggle_env_var_permission(env_var_for_click.clone(), cx);
                            },
                        )),
                    );
                }

                if let Some(used_var) = &env_var_name_used {
                    content = content.child(
                        ConfiguredApiCard::new(format!(
                            "API key set in {} environment variable",
                            used_var
                        ))
                        .tooltip_label(format!(
                            "To reset this API key, unset the {} environment variable.",
                            used_var
                        ))
                        .disabled(true),
                    );

                    return content.into_any_element();
                }
            }
        }

        if is_authenticated && env_var_name_used.is_none() {
            let (status_label, button_label) = if has_oauth && !has_api_key {
                ("Signed in", "Sign Out")
            } else {
                ("API key configured", "Reset Key")
            };

            content = content.child(
                ConfiguredApiCard::new(status_label)
                    .button_label(button_label)
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.reset_api_key(window, cx);
                    })),
            );

            return content.into_any_element();
        }

        // Not authenticated - show available auth options
        if env_var_name_used.is_none() {
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

                let mut button = Button::new("oauth-sign-in", button_label)
                    .full_width()
                    .style(ButtonStyle::Outlined)
                    .disabled(oauth_in_progress)
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.start_oauth_sign_in(window, cx);
                    }));
                if let Some(icon) = button_icon {
                    button = button
                        .icon(icon)
                        .icon_position(IconPosition::Start)
                        .icon_size(IconSize::Small)
                        .icon_color(Color::Muted);
                }

                content = content.child(
                    v_flex()
                        .gap_2()
                        .child(button)
                        .when(oauth_in_progress, |this| {
                            this.child(
                                Label::new("Sign-in in progress...")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
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
                                                Icon::new(IconName::Warning)
                                                    .color(Color::Error)
                                                    .size(IconSize::Small),
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
                            .child(div().h_px().flex_1().bg(cx.theme().colors().border_variant))
                            .child(Label::new("or").size(LabelSize::Small).color(Color::Muted))
                            .child(div().h_px().flex_1().bg(cx.theme().colors().border_variant)),
                    );
                }

                content = content.child(
                    div()
                        .on_action(cx.listener(Self::save_api_key))
                        .child(self.api_key_editor.clone()),
                );
            }
        }

        if self.extension_provider_id == "openai" {
            content = content.child(
                h_flex()
                    .gap_1()
                    .child(
                        Icon::new(IconName::Info)
                            .size(IconSize::XSmall)
                            .color(Color::Muted),
                    )
                    .child(
                        Label::new("Zed also supports OpenAI-compatible models.")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .child(
                        ButtonLink::new(
                            "Learn More",
                            "https://zed.dev/docs/configuring-llm-providers#openai-compatible-providers",
                        )
                        .label_size(LabelSize::Small),
                    ),
            );
        }

        content.into_any_element()
    }
}

impl Focusable for ExtensionProviderConfigurationView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.api_key_editor.read(cx).focus_handle(cx)
    }
}

/// A popup window for OAuth device flow, similar to CopilotCodeVerification.
/// This is used when in edit prediction mode to avoid moving the settings panel behind.
pub struct OAuthCodeVerificationWindow {
    config: OAuthDeviceFlowModalConfig,
    status: OAuthDeviceFlowStatus,
    connect_clicked: bool,
    focus_handle: FocusHandle,
    _subscription: Option<Subscription>,
}

impl Focusable for OAuthCodeVerificationWindow {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for OAuthCodeVerificationWindow {}

impl OAuthCodeVerificationWindow {
    pub fn new(
        config: OAuthDeviceFlowModalConfig,
        state: Entity<OAuthDeviceFlowState>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        window.on_window_should_close(cx, |window, cx| {
            if let Some(this) = window.root::<OAuthCodeVerificationWindow>().flatten() {
                this.update(cx, |_, cx| {
                    cx.emit(DismissEvent);
                });
            }
            true
        });
        cx.subscribe_in(
            &cx.entity(),
            window,
            |_, _, _: &DismissEvent, window, _cx| {
                window.remove_window();
            },
        )
        .detach();

        let subscription = cx.observe(&state, |this, state, cx| {
            let status = state.read(cx).status.clone();
            this.status = status;
            cx.notify();
        });

        Self {
            config,
            status: state.read(cx).status.clone(),
            connect_clicked: false,
            focus_handle: cx.focus_handle(),
            _subscription: Some(subscription),
        }
    }

    fn render_icon(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let icon_color = Color::Custom(cx.theme().colors().icon);
        let icon_size = rems(2.5);
        let plus_size = rems(0.875);
        let plus_color = cx.theme().colors().icon.opacity(0.5);

        if let Some(icon_path) = &self.config.icon_path {
            h_flex()
                .gap_2()
                .items_center()
                .child(
                    Icon::from_external_svg(icon_path.clone())
                        .size(IconSize::Custom(icon_size))
                        .color(icon_color),
                )
                .child(
                    gpui::svg()
                        .size(plus_size)
                        .path("icons/plus.svg")
                        .text_color(plus_color),
                )
                .child(Vector::new(VectorName::ZedLogo, icon_size, icon_size).color(icon_color))
                .into_any_element()
        } else {
            Vector::new(VectorName::ZedLogo, icon_size, icon_size)
                .color(icon_color)
                .into_any_element()
        }
    }

    fn render_device_code(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let user_code = self.config.user_code.clone();
        let copied = cx
            .read_from_clipboard()
            .map(|item| item.text().as_ref() == Some(&user_code))
            .unwrap_or(false);
        let user_code_for_click = user_code.clone();

        ButtonLike::new("copy-button")
            .full_width()
            .style(ButtonStyle::Tinted(ui::TintColor::Accent))
            .size(ButtonSize::Medium)
            .child(
                h_flex()
                    .w_full()
                    .p_1()
                    .justify_between()
                    .child(Label::new(user_code))
                    .child(Label::new(if copied { "Copied!" } else { "Copy" })),
            )
            .on_click(move |_, window, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(user_code_for_click.clone()));
                window.refresh();
            })
    }

    fn render_prompting_modal(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let connect_button_label: String = if self.connect_clicked {
            "Waiting for connection…".to_string()
        } else {
            self.config.connect_button_label.clone()
        };
        let verification_url = self.config.verification_url.clone();

        v_flex()
            .flex_1()
            .gap_2p5()
            .items_center()
            .text_center()
            .child(Headline::new(self.config.headline.clone()).size(HeadlineSize::Large))
            .child(Label::new(self.config.description.clone()).color(Color::Muted))
            .child(self.render_device_code(cx))
            .child(
                Label::new("Paste this code after clicking the button below.").color(Color::Muted),
            )
            .child(
                v_flex()
                    .w_full()
                    .gap_1()
                    .child(
                        Button::new("connect-button", connect_button_label)
                            .full_width()
                            .style(ButtonStyle::Outlined)
                            .size(ButtonSize::Medium)
                            .on_click(cx.listener(move |this, _, _window, cx| {
                                cx.open_url(&verification_url);
                                this.connect_clicked = true;
                            })),
                    )
                    .child(
                        Button::new("cancel-button", "Cancel")
                            .full_width()
                            .size(ButtonSize::Medium)
                            .on_click(cx.listener(|_, _, _, cx| {
                                cx.emit(DismissEvent);
                            })),
                    ),
            )
    }

    fn render_authorized_modal(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .text_center()
            .justify_center()
            .child(Headline::new(self.config.success_headline.clone()).size(HeadlineSize::Large))
            .child(Label::new(self.config.success_message.clone()).color(Color::Muted))
            .child(
                Button::new("done-button", "Done")
                    .full_width()
                    .style(ButtonStyle::Outlined)
                    .size(ButtonSize::Medium)
                    .on_click(cx.listener(|_, _, _, cx| cx.emit(DismissEvent))),
            )
    }

    fn render_failed_modal(&self, error: &str, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .text_center()
            .justify_center()
            .child(Headline::new("Authorization Failed").size(HeadlineSize::Large))
            .child(Label::new(error.to_string()).color(Color::Error))
            .child(
                Button::new("close-button", "Close")
                    .full_width()
                    .size(ButtonSize::Medium)
                    .on_click(cx.listener(|_, _, _, cx| cx.emit(DismissEvent))),
            )
    }
}

impl Render for OAuthCodeVerificationWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let prompt = match &self.status {
            OAuthDeviceFlowStatus::Prompting | OAuthDeviceFlowStatus::WaitingForAuthorization => {
                self.render_prompting_modal(cx).into_any_element()
            }
            OAuthDeviceFlowStatus::Authorized => {
                self.render_authorized_modal(cx).into_any_element()
            }
            OAuthDeviceFlowStatus::Failed(error) => {
                self.render_failed_modal(error, cx).into_any_element()
            }
        };

        v_flex()
            .id("oauth_code_verification")
            .track_focus(&self.focus_handle(cx))
            .size_full()
            .px_4()
            .py_8()
            .gap_2()
            .items_center()
            .justify_center()
            .elevation_3(cx)
            .on_action(cx.listener(|_, _: &menu::Cancel, _, cx| {
                cx.emit(DismissEvent);
            }))
            .on_any_mouse_down(cx.listener(|this, _: &MouseDownEvent, window, cx| {
                window.focus(&this.focus_handle, cx);
            }))
            .child(self.render_icon(cx))
            .child(prompt)
    }
}

fn markdown_styles(window: &Window, cx: &App) -> MarkdownStyle {
    let settings = ThemeSettings::get_global(cx);
    let colors = cx.theme().colors();

    let mut text_style = window.text_style();
    text_style.refine(&TextStyleRefinement {
        font_family: Some(settings.ui_font.family.clone()),
        font_fallbacks: settings.ui_font.fallbacks.clone(),
        font_features: Some(settings.ui_font.features.clone()),
        font_size: Some(settings.ui_font_size(cx).into()),
        line_height: Some(relative(1.5)),
        color: Some(colors.text_muted),
        ..Default::default()
    });

    MarkdownStyle {
        base_text_style: text_style.clone(),
        syntax: cx.theme().syntax().clone(),
        selection_background_color: colors.element_selection_background,
        heading_level_styles: Some(HeadingLevelStyles {
            h1: Some(TextStyleRefinement {
                font_size: Some(rems(1.15).into()),
                ..Default::default()
            }),
            h2: Some(TextStyleRefinement {
                font_size: Some(rems(1.1).into()),
                ..Default::default()
            }),
            h3: Some(TextStyleRefinement {
                font_size: Some(rems(1.05).into()),
                ..Default::default()
            }),
            h4: Some(TextStyleRefinement {
                font_size: Some(rems(1.).into()),
                ..Default::default()
            }),
            h5: Some(TextStyleRefinement {
                font_size: Some(rems(0.95).into()),
                ..Default::default()
            }),
            h6: Some(TextStyleRefinement {
                font_size: Some(rems(0.875).into()),
                ..Default::default()
            }),
        }),
        inline_code: TextStyleRefinement {
            font_family: Some(settings.buffer_font.family.clone()),
            font_fallbacks: settings.buffer_font.fallbacks.clone(),
            font_features: Some(settings.buffer_font.features.clone()),
            font_size: Some(settings.buffer_font_size(cx).into()),
            background_color: Some(colors.editor_foreground.opacity(0.08)),
            ..Default::default()
        },
        link: TextStyleRefinement {
            background_color: Some(colors.editor_foreground.opacity(0.025)),
            color: Some(colors.text_accent),
            underline: Some(UnderlineStyle {
                color: Some(colors.text_accent.opacity(0.5)),
                thickness: px(1.),
                ..Default::default()
            }),
            ..Default::default()
        },
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
    request_limiter: RateLimiter,
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
            LlmToolInputFormat::JsonSchemaSubset => {
                LanguageModelToolSchemaFormat::JsonSchemaSubset
            }
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

        let future = self.request_limiter.stream(async move {
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
        });

        async move { Ok(future.await?.boxed()) }.boxed()
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
                        width: image.size.map(|s| s.width.0 as u32),
                        height: image.size.map(|s| s.height.0 as u32),
                    }),
                    MessageContent::ToolUse(tool_use) => LlmMessageContent::ToolUse(LlmToolUse {
                        id: tool_use.id.to_string(),
                        name: tool_use.name.to_string(),
                        input: serde_json::to_string(&tool_use.input).unwrap_or_default(),
                        is_input_complete: tool_use.is_input_complete,
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
                                    width: image.size.map(|s| s.width.0 as u32),
                                    height: image.size.map(|s| s.height.0 as u32),
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
        thinking_allowed: request.thinking_allowed,
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
                    is_input_complete: tool_use.is_input_complete,
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
