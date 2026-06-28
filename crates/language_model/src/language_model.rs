mod api_key;
mod registry;
mod request;

#[cfg(any(test, feature = "test-support"))]
pub mod fake_provider;

pub use language_model_core::*;

use anyhow::Result;
use futures::FutureExt;
use futures::{StreamExt, future::BoxFuture, stream::BoxStream};
use gpui::{AnyView, App, AsyncApp, Task, Window};
use icons::IconName;
use parking_lot::Mutex;
use std::sync::Arc;

pub use crate::api_key::{ApiKey, ApiKeyState};
pub use crate::registry::*;
pub use crate::request::{LanguageModelImageExt, gpui_size_to_image_size, image_size_to_gpui};
pub use env_var::{EnvVar, env_var};

pub fn init(cx: &mut App) {
    registry::init(cx);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisabledReason(pub SharedString);

impl DisabledReason {
    pub fn new(reason: impl Into<SharedString>) -> Self {
        Self(reason.into())
    }
}

pub struct LanguageModelTextStream {
    pub message_id: Option<String>,
    pub stream: BoxStream<'static, Result<String, LanguageModelCompletionError>>,
    // Has complete token usage after the stream has finished
    pub last_token_usage: Arc<Mutex<TokenUsage>>,
}

impl Default for LanguageModelTextStream {
    fn default() -> Self {
        Self {
            message_id: None,
            stream: Box::pin(futures::stream::empty()),
            last_token_usage: Arc::new(Mutex::new(TokenUsage::default())),
        }
    }
}

pub trait LanguageModel: Send + Sync {
    fn id(&self) -> LanguageModelId;
    fn name(&self) -> LanguageModelName;
    fn provider_id(&self) -> LanguageModelProviderId;
    fn provider_name(&self) -> LanguageModelProviderName;
    fn upstream_provider_id(&self) -> LanguageModelProviderId {
        self.provider_id()
    }
    fn upstream_provider_name(&self) -> LanguageModelProviderName {
        self.provider_name()
    }

    /// Returns whether this model is the "latest", so we can highlight it in the UI.
    fn is_latest(&self) -> bool {
        false
    }

    /// Whether the model is currently disabled and, if so, why this is the case.
    fn is_disabled(&self) -> Option<DisabledReason> {
        None
    }

    /// Whether requests to this model require the user to consent to the
    /// upstream provider retaining inference logs (i.e. the model cannot be
    /// offered with Zero Data Retention).
    fn requires_data_retention(&self) -> bool {
        false
    }

    /// When this model refuses a request, the model ID to fall back to (same provider).
    fn refusal_fallback_model_id(&self) -> Option<&'static str> {
        None
    }

    fn telemetry_id(&self) -> String;

    fn api_key(&self, _cx: &App) -> Option<String> {
        None
    }

    /// Information about the cost of using this model, if available.
    fn model_cost_info(&self) -> Option<LanguageModelCostInfo> {
        None
    }

    /// Whether this model supports thinking.
    fn supports_thinking(&self) -> bool {
        false
    }

    /// Whether thinking can be turned off entirely for this model. Some
    /// models (e.g. Claude Fable 5) always think and cannot honor an "off"
    /// request. Only meaningful when `supports_thinking` returns `true`.
    fn supports_disabling_thinking(&self) -> bool {
        true
    }

    fn supports_fast_mode(&self) -> bool {
        false
    }

    /// Returns the list of supported effort levels that can be used when thinking.
    fn supported_effort_levels(&self) -> Vec<LanguageModelEffortLevel> {
        Vec::new()
    }

    /// Returns the default effort level to use when thinking.
    fn default_effort_level(&self) -> Option<LanguageModelEffortLevel> {
        self.supported_effort_levels()
            .into_iter()
            .find(|effort_level| effort_level.is_default)
    }

    /// Whether this model supports provider-side automatic context
    /// compaction (requested via `LanguageModelRequest::compact_at_tokens`).
    fn supports_server_side_compaction(&self) -> bool {
        false
    }

    /// Whether this model supports images
    fn supports_images(&self) -> bool;

    /// Whether this model supports tools.
    fn supports_tools(&self) -> bool;

    /// Whether this model supports choosing which tool to use.
    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool;

    /// Returns whether this model or provider supports streaming tool calls;
    fn supports_streaming_tools(&self) -> bool {
        false
    }

    /// Returns whether this model/provider reports accurate split input/output token counts.
    /// When true, the UI may show separate input/output token indicators.
    fn supports_split_token_display(&self) -> bool {
        false
    }

    fn tool_input_format(&self) -> LanguageModelToolSchemaFormat {
        LanguageModelToolSchemaFormat::JsonSchema
    }

    fn max_token_count(&self) -> u64;
    fn max_output_tokens(&self) -> Option<u64> {
        None
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            BoxStream<'static, Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>,
            LanguageModelCompletionError,
        >,
    >;

    fn stream_completion_text(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<LanguageModelTextStream, LanguageModelCompletionError>> {
        let future = self.stream_completion(request, cx);

        async move {
            let events = future.await?;
            let mut events = events.fuse();
            let mut message_id = None;
            let mut first_item_text = None;
            let last_token_usage = Arc::new(Mutex::new(TokenUsage::default()));

            if let Some(first_event) = events.next().await {
                match first_event {
                    Ok(LanguageModelCompletionEvent::StartMessage { message_id: id }) => {
                        message_id = Some(id);
                    }
                    Ok(LanguageModelCompletionEvent::Text(text)) => {
                        first_item_text = Some(text);
                    }
                    _ => (),
                }
            }

            let stream = futures::stream::iter(first_item_text.map(Ok))
                .chain(events.filter_map({
                    let last_token_usage = last_token_usage.clone();
                    move |result| {
                        let last_token_usage = last_token_usage.clone();
                        async move {
                            match result {
                                Ok(LanguageModelCompletionEvent::Queued { .. }) => None,
                                Ok(LanguageModelCompletionEvent::Started) => None,
                                Ok(LanguageModelCompletionEvent::StartMessage { .. }) => None,
                                Ok(LanguageModelCompletionEvent::Text(text)) => Some(Ok(text)),
                                Ok(LanguageModelCompletionEvent::Thinking { .. }) => None,
                                Ok(LanguageModelCompletionEvent::RedactedThinking { .. }) => None,
                                Ok(LanguageModelCompletionEvent::ReasoningDetails(_)) => None,
                                Ok(LanguageModelCompletionEvent::Stop(_)) => None,
                                Ok(LanguageModelCompletionEvent::ToolUse(_)) => None,
                                Ok(LanguageModelCompletionEvent::ToolUseJsonParseError {
                                    ..
                                }) => None,
                                Ok(LanguageModelCompletionEvent::Compaction(_)) => None,
                                Ok(LanguageModelCompletionEvent::UsageUpdate(token_usage)) => {
                                    *last_token_usage.lock() = token_usage;
                                    None
                                }
                                Err(err) => Some(Err(err)),
                            }
                        }
                    }
                }))
                .boxed();

            Ok(LanguageModelTextStream {
                message_id,
                stream,
                last_token_usage,
            })
        }
        .boxed()
    }

    fn stream_completion_tool(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<LanguageModelToolUse, LanguageModelCompletionError>> {
        let future = self.stream_completion(request, cx);

        async move {
            let events = future.await?;
            let mut events = events.fuse();

            // Iterate through events until we find a complete ToolUse
            while let Some(event) = events.next().await {
                match event {
                    Ok(LanguageModelCompletionEvent::ToolUse(tool_use))
                        if tool_use.is_input_complete =>
                    {
                        return Ok(tool_use);
                    }
                    Err(err) => {
                        return Err(err);
                    }
                    _ => {}
                }
            }

            // Stream ended without a complete tool use
            Err(LanguageModelCompletionError::Other(anyhow::anyhow!(
                "Stream ended without receiving a complete tool use"
            )))
        }
        .boxed()
    }

    #[cfg(any(test, feature = "test-support"))]
    fn as_fake(&self) -> &fake_provider::FakeLanguageModel {
        unimplemented!()
    }
}

impl std::fmt::Debug for dyn LanguageModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("<dyn LanguageModel>")
            .field("id", &self.id())
            .field("name", &self.name())
            .field("provider_id", &self.provider_id())
            .field("provider_name", &self.provider_name())
            .field("upstream_provider_name", &self.upstream_provider_name())
            .field("upstream_provider_id", &self.upstream_provider_id())
            .field("upstream_provider_id", &self.upstream_provider_id())
            .field("supports_streaming_tools", &self.supports_streaming_tools())
            .finish()
    }
}

/// Either a built-in icon name or a path to an external SVG.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IconOrSvg {
    /// A built-in icon from Zed's icon set.
    Icon(IconName),
    /// Path to a custom SVG icon file.
    Svg(SharedString),
}

impl Default for IconOrSvg {
    fn default() -> Self {
        Self::Icon(IconName::ZedAssistant)
    }
}

pub trait LanguageModelProvider: 'static {
    fn id(&self) -> LanguageModelProviderId;
    fn name(&self) -> LanguageModelProviderName;
    fn icon(&self) -> IconOrSvg {
        IconOrSvg::default()
    }
    fn default_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>>;
    fn default_fast_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>>;
    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>>;
    fn recommended_models(&self, _cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        Vec::new()
    }
    fn is_authenticated(&self, cx: &App) -> bool;
    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>>;
    fn configuration_view(
        &self,
        target_agent: ConfigurationViewTargetAgent,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyView;
    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>>;

    /// Copy shown when this provider rejects a request as unauthenticated
    /// (HTTP 401). The default assumes API-key authentication; providers using
    /// other mechanisms (account or subscription based auth) should override
    /// this so users aren't told to check an API key they don't have.
    fn authentication_error_message(&self) -> SharedString {
        format!(
            "The API key for {} is invalid or has expired. \
            Update your key via the Agent Panel settings to continue.",
            self.name().0
        )
        .into()
    }

    /// Copy shown when a request fails because no credentials are configured
    /// for this provider. The default assumes API-key authentication;
    /// providers using other mechanisms (account or subscription based auth)
    /// should override this.
    fn missing_credentials_error_message(&self) -> SharedString {
        format!(
            "No API key is configured for {}. \
            Add your key via the Agent Panel settings to continue.",
            self.name().0
        )
        .into()
    }

    /// Returns the provider's configuration UI together with how it prefers to
    /// be presented: [`ProviderConfigurationView::Inline`] for a compact control
    /// that can sit in a list row (e.g. a single API-key field), or
    /// [`ProviderConfigurationView::SubPage`] for a richer view that needs its
    /// own surface.
    ///
    /// The default reuses [`Self::configuration_view`] as a sub-page, so
    /// providers only override this when they have a compact inline form.
    fn configuration_view_v2(
        &self,
        target_agent: ConfigurationViewTargetAgent,
        window: &mut Window,
        cx: &mut App,
    ) -> ProviderConfigurationView {
        ProviderConfigurationView::SubPage(self.configuration_view(target_agent, window, cx))
    }

    /// Copy shown the first time a user enables fast mode for a model from
    /// this provider. Returning `None` skips the confirmation prompt and lets
    /// the toggle apply silently.
    fn fast_mode_confirmation(&self, _cx: &App) -> Option<FastModeConfirmation> {
        None
    }
}

/// How a provider's configuration UI prefers to be presented by the settings UI.
#[derive(Clone)]
pub enum ProviderConfigurationView {
    /// A compact control suitable for rendering inline in a list row, such as a
    /// single API-key field.
    Inline(AnyView),
    /// A richer view that should be shown on its own dedicated sub-page.
    SubPage(AnyView),
}

/// Provider-specific copy shown the first time a user enables fast mode.
#[derive(Debug, Clone)]
pub struct FastModeConfirmation {
    pub title: SharedString,
    pub message: SharedString,
}

#[derive(Default, Clone, PartialEq, Eq)]
pub enum ConfigurationViewTargetAgent {
    #[default]
    ZedAgent,
    Other(SharedString),
}

pub trait LanguageModelProviderState: 'static {
    type ObservableEntity;

    fn observable_entity(&self) -> Option<gpui::Entity<Self::ObservableEntity>>;

    fn subscribe<T: 'static>(
        &self,
        cx: &mut gpui::Context<T>,
        callback: impl Fn(&mut T, &mut gpui::Context<T>) + 'static,
    ) -> Option<gpui::Subscription> {
        let entity = self.observable_entity()?;
        Some(cx.observe(&entity, move |this, _, cx| {
            callback(this, cx);
        }))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LanguageModelCostInfo {
    /// Cost per 1,000 input and output tokens
    TokenCost {
        input_token_cost_per_1m: f64,
        output_token_cost_per_1m: f64,
    },
    /// Cost per request
    RequestCost { cost_per_request: f64 },
}

impl LanguageModelCostInfo {
    pub fn to_shared_string(&self) -> SharedString {
        match self {
            LanguageModelCostInfo::RequestCost { cost_per_request } => {
                let cost_str = format!("{}×", Self::cost_value_to_string(cost_per_request));
                SharedString::from(cost_str)
            }
            LanguageModelCostInfo::TokenCost {
                input_token_cost_per_1m,
                output_token_cost_per_1m,
            } => {
                let input_cost = Self::cost_value_to_string(input_token_cost_per_1m);
                let output_cost = Self::cost_value_to_string(output_token_cost_per_1m);
                SharedString::from(format!("{}$/{}$", input_cost, output_cost))
            }
        }
    }

    fn cost_value_to_string(cost: &f64) -> SharedString {
        if (cost.fract() - 0.0).abs() < std::f64::EPSILON {
            SharedString::from(format!("{:.0}", cost))
        } else {
            SharedString::from(format!("{:.2}", cost))
        }
    }
}
