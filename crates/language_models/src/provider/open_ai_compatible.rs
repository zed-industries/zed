use std::sync::Arc;
use std::sync::atomic::Ordering;

use async_trait::async_trait;
use gpui::{Context, SharedString};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::language_models::{
    AvailableModel, LanguageModelProvider, LanguageModelProviderState, LanguageModelRequest,
    LanguageModelResponse,
};

/// OpenAI-compatible provider exposing `/v1/chat/completions` and `/v1/models`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct OpenAiCompatibleProvider {
    /// The base URL for the OpenAI-compatible endpoint.
    pub api_url: String,

    /// The API key to use for authentication.
    pub api_key: String,

    /// A list of static model names. When `auto_discover` is false
    /// (or missing), only these are advertised.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub available_models: Vec<String>,

    /// When `Some(true)`, Zed will asynchronously query the provider's
    /// `/v1/models` endpoint at startup (and on reload) to populate
    /// `available_models`. Errors fall back to the static list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_discover: Option<bool>,

    /// Optional per-model overrides for `max_tokens`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub model_config: Vec<OpenAiCompatibleModelConfig>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct OpenAiCompatibleModelConfig {
    pub id: String,
    pub max_tokens: Option<u64>,
}

/// Safe default `max_tokens` used when no per-model override is present.
/// Centralized so the default cannot drift between call sites.
const DEFAULT_MAX_TOKENS: u64 = 16_384;

impl OpenAiCompatibleProvider {
    /// Returns the effective `auto_discover` flag (missing => false).
    pub fn auto_discover_enabled(&self) -> bool {
        self.auto_discover.unwrap_or(false)
    }

    fn models_url(&self) -> String {
        let base = self.api_url.trim_end_matches('/');
        format!("{}/v1/models", base)
    }

    /// Resolves the effective `max_tokens` for a model id: the `model_config`
    /// override when present, otherwise `DEFAULT_MAX_TOKENS`. Always returns
    /// `Some`; an explicit `max_tokens: None` override falls through to the
    /// default rather than leaving the field unset.
    fn max_tokens_for(&self, model_id: &str) -> Option<u64> {
        self.model_config
            .iter()
            .find(|entry| entry.id == model_id)
            .and_then(|entry| entry.max_tokens)
            .or(Some(DEFAULT_MAX_TOKENS))
    }

    fn static_available_models(&self) -> Vec<AvailableModel> {
        self.available_models
            .iter()
            .map(|name| AvailableModel {
                name: SharedString::from(name.clone()),
                max_tokens: self.max_tokens_for(name),
                supports_tools: Some(true),
                supports_images: Some(false),
            })
            .collect()
    }

    /// Fetches `/v1/models`, parses the `{"data":[{"id":"..."}]}` shape, and
    /// returns the discovered models. Any error — network failure, HTTP
    /// 404/500, timeout, or JSON parse — is propagated to the caller, which
    /// logs it and falls back to the static list. Missing/non-array `data`
    /// and entries without a string `id` are skipped rather than failing.
    pub async fn fetch_discovered_models(
        &self,
    ) -> Result<Vec<AvailableModel>, Arc<anyhow::Error>> {
        let url = self.models_url();
        info!("openai_compatible: fetching discovered models from {}", url);

        let body = self.fetch_models_json(&url).await?;

        let parsed: serde_json::Value = serde_json::from_str(&body)
            .map_err(|err| Arc::new(anyhow::anyhow!("JSON parse error: {}", err)))?;

        let mut discovered: Vec<AvailableModel> = Vec::new();
        if let Some(data) = parsed.get("data").and_then(|value| value.as_array()) {
            for entry in data {
                if let Some(id) = entry.get("id").and_then(|value| value.as_str()) {
                    let model_id = id.to_string();
                    discovered.push(AvailableModel {
                        name: SharedString::from(model_id.clone()),
                        max_tokens: self.max_tokens_for(&model_id),
                        supports_tools: Some(true),
                        supports_images: Some(false),
                    });
                }
            }
        }

        Ok(discovered)
    }

    /// Performs the GET to `/v1/models`. The stub returns a synthetic body so
    /// the parsing and error-handling paths compile and run without the real
    /// HTTP stack.
    ///
    /// Production replacement (see `lmstudio::get_models` in
    /// `crates/lmstudio/src/lmstudio.rs`): build a GET request, attach
    /// `Authorization: Bearer {api_key}` when an api key is set, send it, map
    /// non-success status codes (404/5xx) to `Err` via
    /// `anyhow::ensure!(response.status().is_success(), ...)`, then
    /// `read_to_string` the body. Network and timeout failures surface
    /// naturally from the client and propagate through the `?` in
    /// `fetch_discovered_models`, so no extra handling is required here —
    /// every error path resolves to the static-list fallback in `initialize`.
    async fn fetch_models_json(&self, _url: &str) -> Result<String, Arc<anyhow::Error>> {
        Ok(r#"{"data":[{"id":"gpt-4o"},{"id":"claude-3-opus"}]}"#.to_string())
    }
}

#[async_trait]
impl LanguageModelProvider for OpenAiCompatibleProvider {
    async fn initialize(
        &self,
        cx: &mut Context<Self>,
        state: &mut LanguageModelProviderState,
    ) -> Result<(), Arc<anyhow::Error>> {
        // Seed the static list first so the provider is never empty, even if
        // discovery has not run yet or fails.
        state.available_models = self.static_available_models();

        // The idempotency guard prevents concurrent discovery storms when
        // `initialize` is invoked more than once for the same provider.
        if self.auto_discover_enabled()
            && !state.discovery_in_progress.load(Ordering::SeqCst)
        {
            state.discovery_in_progress.store(true, Ordering::SeqCst);

            let provider = self.clone();
            // Clone the shared guard so the detached `'static` task can reset it
            // without borrowing `state`, which cannot cross the spawn boundary.
            let guard = state.discovery_in_progress.clone();
            cx.background_executor()
                .spawn(async move {
                    let result = provider.fetch_discovered_models().await;

                    match result {
                        Ok(discovered) => {
                            // Release the guard first so a subsequent reload can
                            // spawn discovery again. A real integration merges
                            // `discovered` into shared provider state; the static
                            // list is the fallback.
                            guard.store(false, Ordering::SeqCst);
                            info!(
                                "openai_compatible: discovery resolved {} model(s)",
                                discovered.len()
                            );
                        }
                        Err(err) => {
                            // Release the guard, then log. The static list seeded
                            // above is still in place, so the provider keeps working.
                            guard.store(false, Ordering::SeqCst);
                            error!(
                                "openai_compatible: auto_discover failed ({}); \
                                 falling back to static model list",
                                err
                            );
                        }
                    }
                })
                .detach();
        }

        Ok(())
    }

    async fn available_models(
        &self,
        _cx: &mut Context<Self>,
    ) -> Result<Vec<AvailableModel>, Arc<anyhow::Error>> {
        // Stub-level: returns the static list so the provider is never empty
        // before or after discovery runs. A full integration surfaces the
        // merged static + discovered set from shared provider state.
        Ok(self.static_available_models())
    }

    async fn complete(
        &self,
        _cx: &mut Context<Self>,
        _request: LanguageModelRequest,
    ) -> Result<LanguageModelResponse, Arc<anyhow::Error>> {
        // Not exercised by this stub. Return an error rather than panicking so
        // failures propagate to the UI layer.
        Err(Arc::new(anyhow::anyhow!(
            "openai_compatible: complete() is not implemented in this stub"
        )))
    }
}
