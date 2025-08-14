use anyhow::{Context as _, Result, anyhow};
use collections::{BTreeMap, HashMap};
use credentials_provider::CredentialsProvider;

use futures::Stream;
use futures::{FutureExt, StreamExt, future::BoxFuture};
use gpui::{AnyView, App, AsyncApp, Context, Entity, Subscription, Task, Window};
use http_client::HttpClient;
use language_model::{
    AuthenticateError, LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelId, LanguageModelName, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest,
    LanguageModelToolChoice, LanguageModelToolSchemaFormat, LanguageModelToolResultContent, LanguageModelToolUse, MessageContent,
    RateLimiter, Role, StopReason, TokenUsage,
};
use menu;
use open_ai::{ImageUrl, Model, ReasoningEffort, ResponseStreamEvent, ResponsesRequest, ResponsesStreamingEvent, responses_stream, stream_completion};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use std::pin::Pin;
use std::str::FromStr as _;
use std::sync::Arc;
use strum::IntoEnumIterator;

use ui::{ElevationIndex, List, Tooltip, prelude::*};
use ui_input::SingleLineInput;
use util::ResultExt;

use crate::{AllLanguageModelSettings, ui::InstructionListItem};

const PROVIDER_ID: LanguageModelProviderId = language_model::OPEN_AI_PROVIDER_ID;
const PROVIDER_NAME: LanguageModelProviderName = language_model::OPEN_AI_PROVIDER_NAME;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct OpenAiSettings {
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
    pub reasoning_effort: Option<ReasoningEffort>,
}

pub struct OpenAiLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: gpui::Entity<State>,
}

pub struct State {
    api_key: Option<String>,
    api_key_from_env: bool,
    _subscription: Subscription,
}

const OPENAI_API_KEY_VAR: &str = "OPENAI_API_KEY";

impl State {
    //
    fn is_authenticated(&self) -> bool {
        self.api_key.is_some()
    }

    fn reset_api_key(&self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        let api_url = AllLanguageModelSettings::get_global(cx)
            .openai
            .api_url
            .clone();
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
        let api_url = AllLanguageModelSettings::get_global(cx)
            .openai
            .api_url
            .clone();
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
        let api_url = AllLanguageModelSettings::get_global(cx)
            .openai
            .api_url
            .clone();
        cx.spawn(async move |this, cx| {
            let (api_key, from_env) = if let Ok(api_key) = std::env::var(OPENAI_API_KEY_VAR) {
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

impl OpenAiLanguageModelProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        let state = cx.new(|cx| State {
            api_key: None,
            api_key_from_env: false,
            _subscription: cx.observe_global::<SettingsStore>(|_this: &mut State, cx| {
                cx.notify();
            }),
        });

        Self { http_client, state }
    }

    fn create_language_model(&self, model: open_ai::Model) -> Arc<dyn LanguageModel> {
        Arc::new(OpenAiLanguageModel {
            id: LanguageModelId::from(model.id().to_string()),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }
}

impl LanguageModelProviderState for OpenAiLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<gpui::Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for OpenAiLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconName {
        IconName::AiOpenAi
    }

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(open_ai::Model::default()))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(open_ai::Model::default_fast()))
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models = BTreeMap::default();

        // Add base models from open_ai::Model::iter()
        for model in open_ai::Model::iter() {
            if !matches!(model, open_ai::Model::Custom { .. }) {
                models.insert(model.id().to_string(), model);
            }
        }

        // Override with available models from settings
        for model in &AllLanguageModelSettings::get_global(cx)
            .openai
            .available_models
        {
            models.insert(
                model.name.clone(),
                open_ai::Model::Custom {
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

    fn configuration_view(&self, window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| ConfigurationView::new(self.state.clone(), window, cx))
            .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state.update(cx, |state, cx| state.reset_api_key(cx))
    }
}

pub struct OpenAiLanguageModel {
    id: LanguageModelId,
    model: open_ai::Model,
    state: gpui::Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl OpenAiLanguageModel {
    fn stream_completion(
        &self,
        request: open_ai::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<futures::stream::BoxStream<'static, Result<ResponseStreamEvent>>>>
    {
        let http_client = self.http_client.clone();
        let Ok((api_key, api_url)) = cx.read_entity(&self.state, |state, cx| {
            let settings = &AllLanguageModelSettings::get_global(cx).openai;
            (state.api_key.clone(), settings.api_url.clone())
        }) else {
            return futures::future::ready(Err(anyhow!("App state dropped"))).boxed();
        };

        let future = self.request_limiter.stream(async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey {
                    provider: PROVIDER_NAME,
                });
            };
            let request = stream_completion(http_client.as_ref(), &api_url, &api_key, request);
            let response = request.await?;
            Ok(response)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

impl LanguageModel for OpenAiLanguageModel {
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
        use open_ai::Model;
        match &self.model {
            Model::FourOmni
            | Model::FourOmniMini
            | Model::FourPointOne
            | Model::FourPointOneMini
            | Model::FourPointOneNano
            | Model::Five
            | Model::FiveMini
            | Model::FiveNano
            | Model::O1
            | Model::O3
            | Model::O4Mini => true,
            Model::ThreePointFiveTurbo
            | Model::Four
            | Model::FourTurbo
            | Model::O3Mini
            | Model::Custom { .. } => false,
        }
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto => true,
            LanguageModelToolChoice::Any => true,
            LanguageModelToolChoice::None => true,
        }
    }

    fn tool_input_format(&self) -> LanguageModelToolSchemaFormat {
        language_model::LanguageModelToolSchemaFormat::JsonSchemaSubset
    }

    fn telemetry_id(&self) -> String {
        format!("openai/{}", self.model.id())
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
        count_open_ai_tokens(request, self.model.clone(), cx)
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
        let model_id = self.model.id().to_string();
        let max_output_tokens = self.max_output_tokens();

        // Fallback to Chat Completions for models that may not support Responses well.
        let prefer_responses = !model_id.starts_with("o1-");

        if !prefer_responses {
            let request = into_open_ai(
                request,
                &model_id,
                self.model.supports_parallel_tool_calls(),
                max_output_tokens,
                self.model.reasoning_effort(),
            );
            let completions = self.stream_completion(request, cx);
            return async move {
                let mapper = OpenAiEventMapper::new();
                Ok(mapper.map_stream(completions.await?).boxed())
            }
            .boxed();
        }


        let mut input_items: Vec<serde_json::Value> = Vec::new();

        for message in &request.messages {
            match message.role {
                Role::System => {
                    let mut content_items = Vec::<serde_json::Value>::new();
                    for part in &message.content {
                        match part {
                            MessageContent::Text(t) | MessageContent::Thinking { text: t, .. } => {
                                if !t.is_empty() {
                                    content_items.push(serde_json::json!({"type":"input_text","text": t}));
                                }
                            }
                            MessageContent::Image(img) => {
                                content_items.push(serde_json::json!({
                                    "type":"input_image",
                                    "image_url": img.to_base64_url(),
                                }));
                            }
                            MessageContent::RedactedThinking(_)
                            | MessageContent::ToolUse(_)
                            | MessageContent::ToolResult(_) => {}
                        }
                    }
                    if !content_items.is_empty() {
                        input_items.push(serde_json::json!({
                            "role":"developer",
                            "content": content_items
                        }));
                    }
                }
                Role::User => {
                    let mut content_items = Vec::<serde_json::Value>::new();
                    let mut pending_fc_outputs: Vec<serde_json::Value> = Vec::new();
                    for part in &message.content {
                        match part {
                            MessageContent::Text(t) | MessageContent::Thinking { text: t, .. } => {
                                if !t.is_empty() {
                                    content_items.push(serde_json::json!({"type":"input_text","text": t}));
                                }
                            }
                            MessageContent::Image(img) => {
                                content_items.push(serde_json::json!({
                                    "type":"input_image",
                                    "image_url": img.to_base64_url(),
                                }));
                            }
                            MessageContent::ToolResult(tool_result) => {
                                let call_id = tool_result.tool_use_id.to_string();
                                let output_string = if let Some(output) = &tool_result.output {
                                    if let Some(s) = output.as_str() { s.to_string() } else { output.to_string() }
                                } else {
                                    match &tool_result.content {
                                        LanguageModelToolResultContent::Text(text) => text.as_ref().to_string(),
                                        LanguageModelToolResultContent::Image(_image) => "[image-output]".to_string(),
                                    }
                                };
                                let fco_id = if let Some(suffix) = call_id.strip_prefix("call_") {
                                    format!("fco_{}", suffix)
                                } else {
                                    format!("fco_{}", call_id)
                                };
                                pending_fc_outputs.push(serde_json::json!({
                                    "type":"function_call_output",
                                    "id": fco_id,
                                    "call_id": call_id,
                                    "output": output_string
                                }));
                            }
                            MessageContent::RedactedThinking(_)
                            | MessageContent::ToolUse(_) => {}
                        }
                    }
                    if !content_items.is_empty() {
                        input_items.push(serde_json::json!({
                            "role":"user",
                            "content": content_items
                        }));
                    }
                    for f in pending_fc_outputs {
                        input_items.push(f);
                    }
                }
                Role::Assistant => {
                    let mut assistant_text = String::new();
                    let mut pending_fc_outputs: Vec<serde_json::Value> = Vec::new();

                    for part in &message.content {
                        match part {
                            MessageContent::Text(t) | MessageContent::Thinking { text: t, .. } => {
                                if !t.is_empty() {
                                    if !assistant_text.is_empty() {
                                        assistant_text.push_str("\n\n");
                                    }
                                    assistant_text.push_str(t);
                                }
                            }
                            MessageContent::Image(_img) => {}
                            MessageContent::RedactedThinking(_) => {}
                            MessageContent::ToolUse(tool_use) => {
                                let args = if !tool_use.raw_input.is_empty() {
                                    tool_use.raw_input.clone()
                                } else {
                                    tool_use.input.to_string()
                                };
                                let call_id = tool_use.id.to_string();
                                let fc_id = if let Some(suffix) = call_id.strip_prefix("call_") {
                                    format!("fc_{}", suffix)
                                } else if call_id.starts_with("fc_") {
                                    call_id.clone()
                                } else {
                                    format!("fc_{}", call_id)
                                };
                                input_items.push(serde_json::json!({
                                    "type":"function_call",
                                    "id": fc_id,
                                    "call_id": call_id,
                                    "name": tool_use.name.to_string(),
                                    "arguments": args
                                }));
                            }
                            MessageContent::ToolResult(tool_result) => {
                                let call_id = tool_result.tool_use_id.to_string();
                                let output_string = if let Some(output) = &tool_result.output {
                                    if let Some(s) = output.as_str() { s.to_string() } else { output.to_string() }
                                } else {
                                    match &tool_result.content {
                                        LanguageModelToolResultContent::Text(text) => text.as_ref().to_string(),
                                        LanguageModelToolResultContent::Image(_image) => "[image-output]".to_string(),
                                    }
                                };
                                let fco_id = if let Some(suffix) = call_id.strip_prefix("call_") {
                                    format!("fco_{}", suffix)
                                } else {
                                    format!("fco_{}", call_id)
                                };
                                pending_fc_outputs.push(serde_json::json!({
                                    "type":"function_call_output",
                                    "id": fco_id,
                                    "call_id": call_id,
                                    "output": output_string
                                }));
                            }
                        }
                    }

                    if !assistant_text.is_empty() {
                        input_items.push(serde_json::json!({
                            "role":"assistant",
                            "content": assistant_text
                        }));
                    }
                    for f in pending_fc_outputs {
                        input_items.push(f);
                    }
                }
            }
        }


        // Normalize IDs for top-level tool items to ensure stable mapping across streamed events.
        // - function_call  => "fc_<call_id without 'call_'>" or "fc_auto_<n>" if call_id is missing
        // - function_call_output => "fco_<call_id without 'call_'>" or "fco_auto_<n>"
        // This preserves referential integrity between function_call and function_call_output when
        // Responses omits ids or uses temporary ones. The idx_counter provides deterministic unique
        // fallbacks so interim SSE chunks can be correlated with the final items.

        {
            let mut idx_counter: usize = 0;
            for item in input_items.iter_mut() {
                if let Some(obj) = item.as_object_mut() {
                    if obj.get("role").is_none() {
                        let item_type = obj.get("type").and_then(|v| v.as_str()).unwrap_or("");
                        if !obj.contains_key("id") {
                            let new_id = match item_type {
                                "function_call" => {
                                    if let Some(call_id) = obj.get("call_id").and_then(|v| v.as_str()) {
                                        let suffix = call_id.strip_prefix("call_").unwrap_or(call_id);
                                        format!("fc_{}", suffix)
                                    } else {
                                        format!("fc_auto_{}", idx_counter)
                                    }
                                }
                                "function_call_output" => {
                                    if let Some(call_id) = obj.get("call_id").and_then(|v| v.as_str()) {
                                        let suffix = call_id.strip_prefix("call_").unwrap_or(call_id);
                                        format!("fco_{}", suffix)
                                    } else {
                                        format!("fco_auto_{}", idx_counter)
                                    }
                                }
                                other => format!("item_{}_{}", other, idx_counter),
                            };
                            obj.insert("id".to_string(), serde_json::json!(new_id));
                        }
                    }
                }
                idx_counter += 1;
            }
        }


        let tools_json = request
            .tools
            .iter()
            .map(|t| {
                let mut params = t.input_schema.clone();
                if let serde_json::Value::Object(ref mut o) = params {
                    if !o.contains_key("type") {
                        o.insert("type".into(), serde_json::Value::String("object".into()));
                    }
                    if !o.contains_key("properties") {
                        o.insert(
                            "properties".into(),
                            serde_json::Value::Object(serde_json::Map::new()),
                        );
                    }
                    if !o.contains_key("additionalProperties") {
                        o.insert("additionalProperties".into(), serde_json::Value::Bool(false));
                    }
                    let required = if let Some(props) = o.get("properties").and_then(|v| v.as_object()) {
                        serde_json::Value::Array(
                            props
                                .keys()
                                .cloned()
                                .map(serde_json::Value::String)
                                .collect(),
                        )
                    } else {
                        serde_json::Value::Array(vec![])
                    };
                    o.insert("required".into(), required);
                }
                serde_json::json!({
                    "type":"function",
                    "name": t.name,
                    "description": t.description,
                    "parameters": params,
                    "strict": true
                })
            })
            .collect::<Vec<_>>();


        let mut responses_req = open_ai::ResponsesRequest {
            model: model_id.clone(),
            input: Some(serde_json::Value::Array(input_items)),
            instructions: None,
            reasoning: None,
            prompt: None,
            tools: tools_json,
            tool_choice: request.tool_choice.as_ref().map(|c| match c {
                LanguageModelToolChoice::Auto => serde_json::Value::String("auto".into()),
                LanguageModelToolChoice::Any => serde_json::Value::String("required".into()),
                LanguageModelToolChoice::None => serde_json::Value::String("none".into()),
            }),
            max_output_tokens,
            temperature: request.temperature,
            parallel_tool_calls: Some(false),
            stream: Some(true),
        };


        let http_client = self.http_client.clone();
        let Ok((api_key, api_url, settings_effort, settings_summary)) = cx.read_entity(&self.state, |state, cx| {
            let settings = &AllLanguageModelSettings::get_global(cx).openai;


            let rp = agent_settings::AgentSettings::get_global(cx)
                .model_parameters
                .iter()
                .rfind(|p| {
                    let provider_ok = p
                        .provider
                        .as_ref()
                        .map(|pr| pr.0.as_str() == "openai")
                        .unwrap_or(true);
                    let model_ok = p
                        .model
                        .as_ref()
                        .map(|m| m.as_ref() == model_id)
                        .unwrap_or(true);
                    provider_ok && model_ok
                })
                .and_then(|p| p.reasoning.clone());

            let (eff, sum) = if let Some(rp) = rp {
                // Map settings enums to Responses strings
                let eff = rp.effort.map(|e| match e {
                    agent_settings::ReasoningEffortSetting::Minimal => "minimal".to_string(),
                    agent_settings::ReasoningEffortSetting::Low => "low".to_string(),
                    agent_settings::ReasoningEffortSetting::Medium => "medium".to_string(),
                    agent_settings::ReasoningEffortSetting::High => "high".to_string(),
                });
                let sum = rp.summary.and_then(|s| match s {
                    agent_settings::ReasoningSummarySetting::Auto => Some("auto".to_string()),
                    agent_settings::ReasoningSummarySetting::Concise => Some("concise".to_string()),
                    agent_settings::ReasoningSummarySetting::Detailed => Some("detailed".to_string()),
                    agent_settings::ReasoningSummarySetting::None => None,
                });
                (eff, sum)
            } else {
                (None, None)
            };

            (state.api_key.clone(), settings.api_url.clone(), eff, sum)
        }) else {
            return futures::future::ready(Err(LanguageModelCompletionError::from(anyhow!("App state dropped")))).boxed();
        };


        responses_req.reasoning = {
            let eff = settings_effort.or_else(|| {
                self.model.reasoning_effort().and_then(|e| match e {
                    ReasoningEffort::Minimal => Some("minimal".to_string()),
                    ReasoningEffort::Low => Some("low".to_string()),
                    ReasoningEffort::Medium => Some("medium".to_string()),
                    ReasoningEffort::High => Some("high".to_string()),
                })
            });
            // Default summary to "auto" when not set in settings
            let sum = settings_summary.or(Some("auto".to_string()));
            Some(open_ai::ResponsesReasoning { effort: eff, summary: sum })
        };

        let future = self.request_limiter.stream(async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey {
                    provider: PROVIDER_NAME,
                });
            };

            let stream = open_ai::responses_stream(http_client.as_ref(), &api_url, &api_key, responses_req).await?;


            struct ResponsesState {
                args_by_id: std::collections::HashMap<String, String>,
                names_by_id: std::collections::HashMap<String, String>,
                call_ids_by_id: std::collections::HashMap<String, String>,
                saw_function_done: bool,
            }
            impl ResponsesState {
                fn new() -> Self {
                    Self {
                        args_by_id: std::collections::HashMap::new(),
                        names_by_id: std::collections::HashMap::new(),
                        call_ids_by_id: std::collections::HashMap::new(),
                        saw_function_done: false,
                    }
                }
            }
            let mut state = ResponsesState::new();

            let mapped = stream.flat_map(move |event| {
                let mut out: Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> = Vec::new();
                match event {
                    Ok(ev) => {
                        let ty = ev.event_type.as_str();
                        let p = &ev.payload;

                        match ty {
                            // Text streaming
                            "response.output_text.delta" => {
                                if let Some(s) = p.get("delta").and_then(|v| v.as_str()) {
                                    out.push(Ok(LanguageModelCompletionEvent::Text(s.to_string())));
                                }
                            }
                            // Reasoning summary (Thinking) streaming
                            "response.reasoning_summary_text.delta" => {
                                if let Some(delta) = p.get("delta").and_then(|v| v.as_str()) {
                                    out.push(Ok(LanguageModelCompletionEvent::Thinking {
                                        text: delta.to_string(),
                                        signature: None,
                                    }));
                                }
                            }
                            "response.reasoning_summary_text.done" => {
                                if let Some(text) = p.get("text").and_then(|v| v.as_str()) {
                                    out.push(Ok(LanguageModelCompletionEvent::Thinking {
                                        text: text.to_string(),
                                        signature: None,
                                    }));
                                }
                            }


                            "response.output_item.added" | "response.output_item.done" => {

                                let item = p.get("item").or_else(|| p.get("output_item"));
                                if let Some(item) = item {
                                    if item.get("type").and_then(|v| v.as_str()) == Some("function_call") {
                                        if let Some(item_id) = item.get("id").and_then(|v| v.as_str()) {
                                            if let Some(name) = item.get("name").and_then(|v| v.as_str()) {
                                                state.names_by_id.insert(item_id.to_string(), name.to_string());
                                            }
                                            if let Some(call_id) = item.get("call_id").and_then(|v| v.as_str()) {
                                                state.call_ids_by_id.insert(item_id.to_string(), call_id.to_string());
                                            }
                                        }
                                    }
                                }
                            }


                            "response.function_call_arguments.delta" => {

                                let id_opt = p.get("item_id").and_then(|v| v.as_str())
                                    .or_else(|| p.get("id").and_then(|v| v.as_str()));
                                let delta_opt = p.get("delta").and_then(|v| v.as_str());
                                if let (Some(item_id), Some(delta)) = (id_opt, delta_opt) {
                                    let entry = state.args_by_id.entry(item_id.to_string()).or_default();
                                    entry.push_str(delta);




                                    if let Ok(value) = serde_json::from_str::<serde_json::Value>(entry) {
                                        let emit_id = state.call_ids_by_id.get(item_id).cloned().unwrap_or_else(|| item_id.to_string());
                                        let name = state.names_by_id.get(item_id).cloned().unwrap_or_default();
                                        out.push(Ok(LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse {
                                            id: emit_id.clone().into(),
                                            name: name.clone().into(),
                                            is_input_complete: false,
                                            raw_input: entry.clone(),
                                            input: value,
                                        })));
                                    }
                                }
                            }


                            "response.function_call_arguments.done" => {

                                let id_opt = p.get("item_id").and_then(|v| v.as_str())
                                    .or_else(|| p.get("id").and_then(|v| v.as_str()));
                                if let Some(item_id) = id_opt {
                                    if let Some(raw) = state.args_by_id.remove(item_id) {
                                        state.saw_function_done = true;
                                        let name = state.names_by_id.get(item_id).cloned().unwrap_or_default();
                                        let emit_id = state.call_ids_by_id.get(item_id).cloned().unwrap_or_else(|| item_id.to_string());
                                        match serde_json::from_str::<serde_json::Value>(&raw) {
                                            Ok(value) => out.push(Ok(LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse {
                                                id: emit_id.clone().into(),
                                                name: name.clone().into(),
                                                is_input_complete: true,
                                                raw_input: raw.clone(),
                                                input: value,
                                            }))),
                                            Err(err) => out.push(Ok(LanguageModelCompletionEvent::ToolUseJsonParseError {
                                                id: emit_id.into(),
                                                tool_name: name.into(),
                                                raw_input: raw.clone().into(),
                                                json_parse_error: err.to_string(),
                                            })),
                                        }
                                    }
                                }
                            }


                            "response.completed" => {


                                let usage = p.get("response").and_then(|r| r.get("usage")).or_else(|| p.get("usage"));

                                if let Some(u) = usage {
                                    let input_tokens = u.get("input_tokens").and_then(|v| v.as_u64()).unwrap_or(0);
                                    let output_tokens = u.get("output_tokens").and_then(|v| v.as_u64()).unwrap_or(0);
                                    let cached_tokens = u
                                        .get("input_tokens_details")
                                        .and_then(|d| d.get("cached_tokens"))
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(0);
                                    out.push(Ok(LanguageModelCompletionEvent::UsageUpdate(TokenUsage {
                                        input_tokens,
                                        output_tokens,
                                        cache_creation_input_tokens: 0,
                                        cache_read_input_tokens: cached_tokens,
                                    })));
                                }
                                // Stop reason
                                let stop = if state.saw_function_done { StopReason::ToolUse } else { StopReason::EndTurn };
                                out.push(Ok(LanguageModelCompletionEvent::Stop(stop)));
                            }


                            "error" => {

                                let msg = p
                                    .get("error")
                                    .and_then(|e| e.get("message"))
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("Responses stream error")
                                    .to_string();
                                out.push(Err(LanguageModelCompletionError::from(anyhow!(msg))));
                            }
                            _ => {}
                        }
                    }
                    Err(e) => out.push(Err(LanguageModelCompletionError::from(anyhow!(e)))),
                }
                futures::stream::iter(out)
            });

            Ok(mapped.boxed())
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

pub fn into_open_ai(
    request: LanguageModelRequest,
    model_id: &str,
    supports_parallel_tool_calls: bool,
    max_output_tokens: Option<u64>,
    reasoning_effort: Option<ReasoningEffort>,
) -> open_ai::Request {
    let stream = !model_id.starts_with("o1-");

    let mut messages = Vec::new();
    for message in request.messages {
        for content in message.content {
            match content {
                MessageContent::Text(text) | MessageContent::Thinking { text, .. } => {
                    add_message_content_part(
                        open_ai::MessagePart::Text { text: text },
                        message.role,
                        &mut messages,
                    )
                }
                MessageContent::RedactedThinking(_) => {}
                MessageContent::Image(image) => {
                    add_message_content_part(
                        open_ai::MessagePart::Image {
                            image_url: ImageUrl {
                                url: image.to_base64_url(),
                                detail: None,
                            },
                        },
                        message.role,
                        &mut messages,
                    );
                }
                MessageContent::ToolUse(tool_use) => {
                    let tool_call = open_ai::ToolCall {
                        id: tool_use.id.to_string(),
                        content: open_ai::ToolCallContent::Function {
                            function: open_ai::FunctionContent {
                                name: tool_use.name.to_string(),
                                arguments: serde_json::to_string(&tool_use.input)
                                    .unwrap_or_default(),
                            },
                        },
                    };

                    if let Some(open_ai::RequestMessage::Assistant { tool_calls, .. }) =
                        messages.last_mut()
                    {
                        tool_calls.push(tool_call);
                    } else {
                        messages.push(open_ai::RequestMessage::Assistant {
                            content: None,
                            tool_calls: vec![tool_call],
                        });
                    }
                }
                MessageContent::ToolResult(tool_result) => {
                    let content = match &tool_result.content {
                        LanguageModelToolResultContent::Text(text) => {
                            vec![open_ai::MessagePart::Text {
                                text: text.to_string(),
                            }]
                        }
                        LanguageModelToolResultContent::Image(image) => {
                            vec![open_ai::MessagePart::Image {
                                image_url: ImageUrl {
                                    url: image.to_base64_url(),
                                    detail: None,
                                },
                            }]
                        }
                    };

                    messages.push(open_ai::RequestMessage::Tool {
                        content: content.into(),
                        tool_call_id: tool_result.tool_use_id.to_string(),
                    });
                }
            }
        }
    }

    open_ai::Request {
        model: model_id.into(),
        messages,
        stream,
        stop: request.stop,
        temperature: request.temperature.unwrap_or(1.0),
        max_completion_tokens: max_output_tokens,
        parallel_tool_calls: if supports_parallel_tool_calls && !request.tools.is_empty() {
            // Disable parallel tool calls, as the Agent currently expects a maximum of one per turn.
            Some(false)
        } else {
            None
        },
        prompt_cache_key: request.thread_id,
        tools: request
            .tools
            .into_iter()
            .map(|tool| open_ai::ToolDefinition::Function {
                function: open_ai::FunctionDefinition {
                    name: tool.name,
                    description: Some(tool.description),
                    parameters: Some(tool.input_schema),
                },
            })
            .collect(),
        tool_choice: request.tool_choice.map(|choice| match choice {
            LanguageModelToolChoice::Auto => open_ai::ToolChoice::Auto,
            LanguageModelToolChoice::Any => open_ai::ToolChoice::Required,
            LanguageModelToolChoice::None => open_ai::ToolChoice::None,
        }),
        reasoning_effort,
    }
}

fn add_message_content_part(
    new_part: open_ai::MessagePart,
    role: Role,
    messages: &mut Vec<open_ai::RequestMessage>,
) {
    match (role, messages.last_mut()) {
        (Role::User, Some(open_ai::RequestMessage::User { content }))
        | (
            Role::Assistant,
            Some(open_ai::RequestMessage::Assistant {
                content: Some(content),
                ..
            }),
        )
        | (Role::System, Some(open_ai::RequestMessage::System { content, .. })) => {
            content.push_part(new_part);
        }
        _ => {
            messages.push(match role {
                Role::User => open_ai::RequestMessage::User {
                    content: open_ai::MessageContent::from(vec![new_part]),
                },
                Role::Assistant => open_ai::RequestMessage::Assistant {
                    content: Some(open_ai::MessageContent::from(vec![new_part])),
                    tool_calls: Vec::new(),
                },
                Role::System => open_ai::RequestMessage::System {
                    content: open_ai::MessageContent::from(vec![new_part]),
                },
            });
        }
    }
}

pub struct OpenAiEventMapper {
    tool_calls_by_index: HashMap<usize, RawToolCall>,
}

impl OpenAiEventMapper {
    pub fn new() -> Self {
        Self {
            tool_calls_by_index: HashMap::default(),
        }
    }

    pub fn map_stream(
        mut self,
        events: Pin<Box<dyn Send + Stream<Item = Result<ResponseStreamEvent>>>>,
    ) -> impl Stream<Item = Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>
    {
        events.flat_map(move |event| {
            futures::stream::iter(match event {
                Ok(event) => self.map_event(event),
                Err(error) => vec![Err(LanguageModelCompletionError::from(anyhow!(error)))],
            })
        })
    }

    pub fn map_event(
        &mut self,
        event: ResponseStreamEvent,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        let mut events = Vec::new();
        if let Some(usage) = event.usage {
            events.push(Ok(LanguageModelCompletionEvent::UsageUpdate(TokenUsage {
                input_tokens: usage.prompt_tokens,
                output_tokens: usage.completion_tokens,
                cache_creation_input_tokens: 0,
                cache_read_input_tokens: 0,
            })));
        }

        let Some(choice) = event.choices.first() else {
            return events;
        };

        if let Some(content) = choice.delta.content.clone() {
            events.push(Ok(LanguageModelCompletionEvent::Text(content)));
        }

        if let Some(tool_calls) = choice.delta.tool_calls.as_ref() {
            for tool_call in tool_calls {
                let entry = self.tool_calls_by_index.entry(tool_call.index).or_default();

                if let Some(tool_id) = tool_call.id.clone() {
                    entry.id = tool_id;
                }

                if let Some(function) = tool_call.function.as_ref() {
                    if let Some(name) = function.name.clone() {
                        entry.name = name;
                    }

                    if let Some(arguments) = function.arguments.clone() {
                        entry.arguments.push_str(&arguments);
                    }
                }
            }
        }

        match choice.finish_reason.as_deref() {
            Some("stop") => {
                events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::EndTurn)));
            }
            Some("tool_calls") => {
                events.extend(self.tool_calls_by_index.drain().map(|(_, tool_call)| {
                    match serde_json::from_str(&tool_call.arguments) {
                        Ok(input) => Ok(LanguageModelCompletionEvent::ToolUse(
                            LanguageModelToolUse {
                                id: tool_call.id.clone().into(),
                                name: tool_call.name.as_str().into(),
                                is_input_complete: true,
                                input,
                                raw_input: tool_call.arguments.clone(),
                            },
                        )),
                        Err(error) => Ok(LanguageModelCompletionEvent::ToolUseJsonParseError {
                            id: tool_call.id.into(),
                            tool_name: tool_call.name.into(),
                            raw_input: tool_call.arguments.clone().into(),
                            json_parse_error: error.to_string(),
                        }),
                    }
                }));

                events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::ToolUse)));
            }
            Some(stop_reason) => {
                log::error!("Unexpected OpenAI stop_reason: {stop_reason:?}",);
                events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::EndTurn)));
            }
            None => {}
        }

        events
    }
}

#[derive(Default)]
struct RawToolCall {
    id: String,
    name: String,
    arguments: String,
}

pub(crate) fn collect_tiktoken_messages(
    request: LanguageModelRequest,
) -> Vec<tiktoken_rs::ChatCompletionRequestMessage> {
    request
        .messages
        .into_iter()
        .map(|message| tiktoken_rs::ChatCompletionRequestMessage {
            role: match message.role {
                Role::User => "user".into(),
                Role::Assistant => "assistant".into(),
                Role::System => "system".into(),
            },
            content: Some(message.string_contents()),
            name: None,
            function_call: None,
        })
        .collect::<Vec<_>>()
}

pub fn count_open_ai_tokens(
    request: LanguageModelRequest,
    model: Model,
    cx: &App,
) -> BoxFuture<'static, Result<u64>> {
    cx.background_spawn(async move {
        let messages = collect_tiktoken_messages(request);

        match model {
            Model::Custom { max_tokens, .. } => {
                let model = if max_tokens >= 100_000 {
                    // If the max tokens is 100k or more, it is likely the o200k_base tokenizer from gpt4o
                    "gpt-4o"
                } else {
                    // Otherwise fallback to gpt-4, since only cl100k_base and o200k_base are
                    // supported with this tiktoken method
                    "gpt-4"
                };
                tiktoken_rs::num_tokens_from_messages(model, &messages)
            }
            // Currently supported by tiktoken_rs
            // Sometimes tiktoken-rs is behind on model support. If that is the case, make a new branch
            // arm with an override. We enumerate all supported models here so that we can check if new
            // models are supported yet or not.
            Model::ThreePointFiveTurbo
            | Model::Four
            | Model::FourTurbo
            | Model::FourOmni
            | Model::FourOmniMini
            | Model::FourPointOne
            | Model::FourPointOneMini
            | Model::FourPointOneNano
            | Model::O1
            | Model::O3
            | Model::O3Mini
            | Model::O4Mini => tiktoken_rs::num_tokens_from_messages(model.id(), &messages),
            // GPT-5 models don't have tiktoken support yet; fall back on gpt-4o tokenizer
            Model::Five | Model::FiveMini | Model::FiveNano => {
                tiktoken_rs::num_tokens_from_messages("gpt-4o", &messages)
            }
        }
        .map(|tokens| tokens as u64)
    })
    .boxed()
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
                "sk-000000000000000000000000000000000000000000000000",
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

        let api_key_section = if self.should_render_editor(cx) {
            v_flex()
                .on_action(cx.listener(Self::save_api_key))
                .child(Label::new("To use Zed's agent with OpenAI, you need to add an API key. Follow these steps:"))
                .child(
                    List::new()
                        .child(InstructionListItem::new(
                            "Create one by visiting",
                            Some("OpenAI's console"),
                            Some("https://platform.openai.com/api-keys"),
                        ))
                        .child(InstructionListItem::text_only(
                            "Ensure your OpenAI account has credits",
                        ))
                        .child(InstructionListItem::text_only(
                            "Paste your API key below and hit enter to start using the assistant",
                        )),
                )
                .child(self.api_key_editor.clone())
                .child(
                    Label::new(
                        format!("You can also assign the {OPENAI_API_KEY_VAR} environment variable and restart Zed."),
                    )
                    .size(LabelSize::Small).color(Color::Muted),
                )
                .child(
                    Label::new(
                        "Note that having a subscription for another service like GitHub Copilot won't work.",
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
                            format!("API key set in {OPENAI_API_KEY_VAR} environment variable.")
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
                            this.tooltip(Tooltip::text(format!("To reset your API key, unset the {OPENAI_API_KEY_VAR} environment variable.")))
                        })
                        .on_click(cx.listener(|this, _, window, cx| this.reset_api_key(window, cx))),
                )
                .into_any()
        };

        let compatible_api_section = h_flex()
            .mt_1p5()
            .gap_0p5()
            .flex_wrap()
            .when(self.should_render_editor(cx), |this| {
                this.pt_1p5()
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
            })
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Icon::new(IconName::Info)
                            .size(IconSize::XSmall)
                            .color(Color::Muted),
                    )
                    .child(Label::new("Zed also supports OpenAI-compatible models.")),
            )
            .child(
                Button::new("docs", "Learn More")
                    .icon(IconName::ArrowUpRight)
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Muted)
                    .on_click(move |_, _window, cx| {
                        cx.open_url("https://zed.dev/docs/ai/llm-providers#openai-api-compatible")
                    }),
            );

        if self.load_credentials_task.is_some() {
            div().child(Label::new("Loading credentials…")).into_any()
        } else {
            v_flex()
                .size_full()
                .child(api_key_section)
                .child(compatible_api_section)
                .into_any()
        }
    }
}

#[cfg(test)]
mod tests {
    use gpui::TestAppContext;
    use language_model::LanguageModelRequestMessage;

    use super::*;

    #[gpui::test]
    fn tiktoken_rs_support(cx: &TestAppContext) {
        let request = LanguageModelRequest {
            thread_id: None,
            prompt_id: None,
            intent: None,
            mode: None,
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text("message".into())],
                cache: false,
            }],
            tools: vec![],
            tool_choice: None,
            stop: vec![],
            temperature: None,
            thinking_allowed: true,
        };

        // Validate that all models are supported by tiktoken-rs
        for model in Model::iter() {
            let count = cx
                .executor()
                .block(count_open_ai_tokens(
                    request.clone(),
                    model,
                    &cx.app.borrow(),
                ))
                .unwrap();
            assert!(count > 0);
        }
    }
}
