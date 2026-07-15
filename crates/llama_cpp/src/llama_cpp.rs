use anyhow::{Context as _, Result, anyhow};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::{
    AsyncBody, CustomHeaders, HttpClient, HttpRequestExt, Method, Request as HttpRequest,
    RequestBuilderExt, http,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

pub const LLAMA_CPP_API_URL: &str = "http://localhost:8080";

const DEFAULT_CONTEXT_LENGTH: u64 = 4096;

/// A model exposed to the rest of Zed, after merging API discovery with
/// user-configured overrides.
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Model {
    pub name: String,
    pub display_name: Option<String>,
    pub max_tokens: u64,
    pub supports_tools: bool,
    pub supports_images: bool,
    pub supports_thinking: bool,
}

impl Model {
    pub fn new(
        name: &str,
        display_name: Option<&str>,
        max_tokens: Option<u64>,
        supports_tools: bool,
        supports_images: bool,
        supports_thinking: bool,
    ) -> Self {
        Self {
            name: name.to_owned(),
            display_name: display_name.map(ToString::to_string),
            max_tokens: max_tokens.unwrap_or(DEFAULT_CONTEXT_LENGTH),
            supports_tools,
            supports_images,
            supports_thinking,
        }
    }

    pub fn display_name(&self) -> &str {
        self.display_name.as_deref().unwrap_or(&self.name)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoice {
    Auto,
    Required,
    None,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolDefinition {
    Function { function: FunctionDefinition },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum ChatMessage {
    Assistant {
        #[serde(default)]
        content: Option<MessageContent>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        reasoning_content: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        tool_calls: Vec<ToolCall>,
    },
    User {
        content: MessageContent,
    },
    System {
        content: MessageContent,
    },
    Tool {
        content: MessageContent,
        tool_call_id: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(untagged)]
pub enum MessageContent {
    Plain(String),
    Multipart(Vec<MessagePart>),
}

impl MessageContent {
    pub fn push_part(&mut self, part: MessagePart) {
        match self {
            MessageContent::Plain(text) => {
                *self =
                    MessageContent::Multipart(vec![MessagePart::Text { text: text.clone() }, part]);
            }
            MessageContent::Multipart(parts) if parts.is_empty() => match part {
                MessagePart::Text { text } => *self = MessageContent::Plain(text),
                MessagePart::Image { .. } => *self = MessageContent::Multipart(vec![part]),
            },
            MessageContent::Multipart(parts) => parts.push(part),
        }
    }
}

impl From<Vec<MessagePart>> for MessageContent {
    fn from(mut parts: Vec<MessagePart>) -> Self {
        if let [MessagePart::Text { text }] = parts.as_mut_slice() {
            MessageContent::Plain(std::mem::take(text))
        } else {
            MessageContent::Multipart(parts)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessagePart {
    Text {
        text: String,
    },
    #[serde(rename = "image_url")]
    Image {
        image_url: ImageUrl,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ImageUrl {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ToolCall {
    pub id: String,
    #[serde(flatten)]
    pub content: ToolCallContent,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ToolCallContent {
    Function { function: FunctionContent },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct FunctionContent {
    pub name: String,
    pub arguments: String,
}

#[derive(Serialize, Debug)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<ToolDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,
}

/// Asks the server to include a final `usage` chunk in the stream.
#[derive(Serialize, Debug)]
pub struct StreamOptions {
    pub include_usage: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LlamaCppError {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ResponseStreamResult {
    Ok(ResponseStreamEvent),
    Err { error: LlamaCppError },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseStreamEvent {
    pub model: String,
    pub object: String,
    pub choices: Vec<ChoiceDelta>,
    pub usage: Option<Usage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChoiceDelta {
    pub index: u32,
    pub delta: ResponseMessageDelta,
    pub finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ResponseMessageDelta {
    pub content: Option<String>,
    /// `llama-server` emits reasoning as a dedicated `reasoning_content` field
    /// when started with a reasoning format (e.g. `--reasoning-format deepseek`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCallChunk>>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ToolCallChunk {
    pub index: usize,
    pub id: Option<String>,
    pub function: Option<FunctionChunk>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct FunctionChunk {
    pub name: Option<String>,
    pub arguments: Option<String>,
}

/// Response of `GET /v1/models`.
///
/// In single-model mode `data` has exactly one entry describing the loaded
/// model; in router mode it lists every model the server knows about.
#[derive(Deserialize, Debug)]
pub struct ListModelsResponse {
    #[serde(default)]
    pub data: Vec<ModelEntry>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ModelEntry {
    pub id: String,
    /// Present in single-model mode; carries context information.
    #[serde(default)]
    pub meta: Option<ModelMeta>,
    /// Present in router mode; reports the modalities the model accepts.
    #[serde(default)]
    pub architecture: Option<Architecture>,
    /// Present in router mode; reports whether the model is currently loaded.
    #[serde(default)]
    pub status: Option<ModelStatus>,
}

impl ModelEntry {
    /// Whether this entry came from a server running in router mode.
    pub fn is_router_entry(&self) -> bool {
        self.status.is_some()
    }

    /// Whether the model is loaded and can be probed for capabilities without
    /// triggering a (potentially expensive) load.
    pub fn is_loaded(&self) -> bool {
        self.status
            .as_ref()
            .is_some_and(|status| status.value == "loaded")
    }

    /// Whether the model is currently loading, so a progress label applies.
    pub fn is_loading(&self) -> bool {
        self.status
            .as_ref()
            .is_some_and(|status| status.value == "loading")
    }

    pub fn supports_images_hint(&self) -> bool {
        self.architecture
            .as_ref()
            .is_some_and(|architecture| architecture.input_modalities.iter().any(|m| m == "image"))
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct ModelMeta {
    /// The runtime per-slot context size.
    #[serde(default)]
    pub n_ctx: Option<u64>,
    /// The context size the model was trained with.
    #[serde(default)]
    pub n_ctx_train: Option<u64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct Architecture {
    #[serde(default)]
    pub input_modalities: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ModelStatus {
    /// One of `loaded`, `loading`, `unloaded`, `downloading`, `downloaded`, `sleeping`.
    pub value: String,
}

/// Response of `GET /props`, describing the loaded model.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct Props {
    #[serde(default)]
    pub default_generation_settings: Option<GenerationSettings>,
    #[serde(default)]
    pub modalities: Option<Modalities>,
    #[serde(default)]
    pub chat_template_caps: Option<ChatTemplateCaps>,
}

impl Props {
    /// The runtime context length the loaded model was configured with.
    pub fn context_length(&self) -> Option<u64> {
        self.default_generation_settings
            .as_ref()
            .and_then(|settings| settings.n_ctx)
            .filter(|n_ctx| *n_ctx > 0)
    }

    pub fn supports_images(&self) -> bool {
        self.modalities
            .as_ref()
            .is_some_and(|modalities| modalities.vision)
    }

    pub fn supports_tools(&self) -> bool {
        self.chat_template_caps
            .as_ref()
            .is_some_and(|caps| caps.supports_tool_calls || caps.supports_tools)
    }

    pub fn supports_thinking(&self) -> bool {
        self.chat_template_caps
            .as_ref()
            .is_some_and(|caps| caps.supports_preserve_reasoning)
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct GenerationSettings {
    #[serde(default)]
    pub n_ctx: Option<u64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct Modalities {
    #[serde(default)]
    pub vision: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct ChatTemplateCaps {
    /// llama.cpp's `/props` reports both of these keys, so we read them into
    /// separate fields (aliasing one to the other would be a serde duplicate-field
    /// error) and treat either being true as tool support. `supports_tools` is the
    /// older name; some builds report only one of the two.
    #[serde(default)]
    pub supports_tool_calls: bool,
    #[serde(default)]
    pub supports_tools: bool,
    #[serde(default)]
    pub supports_preserve_reasoning: bool,
}

/// An event from the router's `/models/sse` feed, which the provider subscribes
/// to so model capabilities stay current as models load and unload. `model` is
/// `*` for events that aren't about a single model (e.g. the list reloading).
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ModelEvent {
    #[serde(default)]
    pub model: String,
    pub event: String,
    #[serde(default)]
    pub data: Option<ModelEventData>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct ModelEventData {
    #[serde(default)]
    pub status: Option<String>,
    /// Present on an `unloaded` status; non-zero means the model failed to load.
    #[serde(default)]
    pub exit_code: Option<i32>,
    /// Present on a `loading` status, reporting per-stage load progress.
    #[serde(default)]
    pub progress: Option<LoadProgress>,
}

/// Per-stage load progress carried by a `loading` event. A model loads its
/// stages in order (the text model, plus an optional draft and/or multimodal
/// projector), each reporting a `0.0..=1.0` fraction.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct LoadProgress {
    #[serde(default)]
    pub stages: Vec<String>,
    #[serde(default)]
    pub current: String,
    #[serde(default)]
    pub value: f32,
}

impl LoadProgress {
    /// A human label for the stage currently loading, matching the llama.cpp
    /// WebUI's labels. We report the current stage's own `value` rather than a
    /// blended overall percentage (as the WebUI does), so each stage runs
    /// `0→100%` and the label says which stage it is.
    pub fn stage_label(&self) -> &'static str {
        match self.current.as_str() {
            "text_model" => "Loading weights",
            "spec_model" => "Loading draft",
            "mmproj_model" => "Loading projector",
            _ => "Loading",
        }
    }

    /// The full load-status label shown in the model selector, e.g.
    /// `"Loading weights 42%"`: the current stage plus its own progress rounded
    /// to a percentage.
    pub fn progress_label(&self) -> String {
        format!(
            "{} {}%",
            self.stage_label(),
            (self.value * 100.0).round() as u32
        )
    }
}

impl ModelEvent {
    /// Whether this event means the set of models or their loaded state changed,
    /// so the provider should re-run discovery. Intermediate `loading` progress
    /// ticks return `false` — only terminal load/unload and list changes matter.
    pub fn changes_model_state(&self) -> bool {
        match self.event.as_str() {
            "models_reload" | "model_remove" => true,
            _ => matches!(
                self.data.as_ref().and_then(|data| data.status.as_deref()),
                Some("loaded" | "unloaded")
            ),
        }
    }

    /// The non-zero exit code of a failed load, if this event reports one.
    pub fn load_failure(&self) -> Option<i32> {
        let data = self.data.as_ref()?;
        if data.status.as_deref() == Some("unloaded") {
            data.exit_code.filter(|code| *code != 0)
        } else {
            None
        }
    }

    /// This event's load progress if it is a loading event carrying usable stage
    /// data, else `None`.
    pub fn load_progress(&self) -> Option<&LoadProgress> {
        let data = self.data.as_ref()?;
        if data.status.as_deref() != Some("loading") {
            return None;
        }
        let progress = data.progress.as_ref()?;
        // The server also emits bare stage-transition markers (e.g.
        // `{"stage": "mmproj_model"}`) with no `stages`/`current`/`value`. Skip
        // them so the indicator holds its last value rather than dropping to 0%.
        if progress.stages.is_empty() || progress.current.is_empty() {
            return None;
        }
        Some(progress)
    }
}

pub async fn stream_chat_completion(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: Option<&str>,
    request: ChatCompletionRequest,
    extra_headers: &CustomHeaders,
) -> Result<BoxStream<'static, Result<ResponseStreamEvent>>> {
    let uri = format!("{api_url}/v1/chat/completions");
    let request_builder = http::Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .when_some(api_key, |builder, api_key| {
            builder.header("Authorization", format!("Bearer {api_key}"))
        });

    let request = request_builder
        .extra_headers(extra_headers)
        .body(AsyncBody::from(serde_json::to_string(&request)?))?;
    let mut response = client.send(request).await?;
    if response.status().is_success() {
        let reader = BufReader::new(response.into_body());
        Ok(reader
            .lines()
            .filter_map(|line| async move {
                match line {
                    Ok(line) => {
                        let line = line.strip_prefix("data: ")?;
                        if line == "[DONE]" {
                            None
                        } else {
                            match serde_json::from_str(line) {
                                Ok(ResponseStreamResult::Ok(response)) => Some(Ok(response)),
                                Ok(ResponseStreamResult::Err { error }) => {
                                    Some(Err(anyhow!(error.message)))
                                }
                                Err(error) => Some(Err(anyhow!(error))),
                            }
                        }
                    }
                    Err(error) => Some(Err(anyhow!(error))),
                }
            })
            .boxed())
    } else {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;
        anyhow::bail!(
            "Failed to connect to llama.cpp API: {} {}",
            response.status(),
            body,
        );
    }
}

/// Lists the models the server is serving via `GET /v1/models`.
pub async fn get_models(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: Option<&str>,
    extra_headers: &CustomHeaders,
) -> Result<Vec<ModelEntry>> {
    let uri = format!("{api_url}/v1/models");
    let request = HttpRequest::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Accept", "application/json")
        .when_some(api_key, |builder, api_key| {
            builder.header("Authorization", format!("Bearer {api_key}"))
        })
        .extra_headers(extra_headers)
        .body(AsyncBody::default())?;

    let mut response = client.send(request).await?;

    let mut body = String::new();
    response.body_mut().read_to_string(&mut body).await?;

    anyhow::ensure!(
        response.status().is_success(),
        "Failed to connect to llama.cpp API: {} {}",
        response.status(),
        body,
    );
    let response: ListModelsResponse =
        serde_json::from_str(&body).context("Unable to parse llama.cpp models response")?;
    Ok(response.data)
}

/// Fetches `GET /props` to discover the loaded model's capabilities.
///
/// In router mode `model` selects which model instance to query; passing
/// `None` queries the single loaded model in single-model mode.
pub async fn get_props(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: Option<&str>,
    model: Option<&str>,
    extra_headers: &CustomHeaders,
) -> Result<Props> {
    // Router-mode `/props` takes a `model` query parameter selecting which
    // instance to describe. Model ids contain `/` and `:` (e.g.
    // `unsloth/Qwen3.5-2B-GGUF:Q4_1`), so the value is URL-encoded.
    let uri = match model {
        Some(model) => Url::parse_with_params(&format!("{api_url}/props"), [("model", model)])
            .context("invalid llama.cpp API URL")?
            .to_string(),
        None => format!("{api_url}/props"),
    };
    let request = HttpRequest::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Accept", "application/json")
        .when_some(api_key, |builder, api_key| {
            builder.header("Authorization", format!("Bearer {api_key}"))
        })
        .extra_headers(extra_headers)
        .body(AsyncBody::default())?;

    let mut response = client.send(request).await?;

    let mut body = String::new();
    response.body_mut().read_to_string(&mut body).await?;

    anyhow::ensure!(
        response.status().is_success(),
        "Failed to connect to llama.cpp API: {} {}",
        response.status(),
        body,
    );
    let props: Props =
        serde_json::from_str(&body).context("Unable to parse llama.cpp props response")?;
    Ok(props)
}

/// Opens the router's `GET /models/sse` event stream. Each item is one parsed
/// event; the stream ends when the connection closes. Only available on builds
/// that expose `/models/sse` (router mode).
pub async fn stream_model_events(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: Option<&str>,
    extra_headers: &CustomHeaders,
) -> Result<BoxStream<'static, Result<ModelEvent>>> {
    let uri = format!("{api_url}/models/sse");
    let request = HttpRequest::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Accept", "text/event-stream")
        .when_some(api_key, |builder, api_key| {
            builder.header("Authorization", format!("Bearer {api_key}"))
        })
        .extra_headers(extra_headers)
        .body(AsyncBody::default())?;

    let mut response = client.send(request).await?;
    if !response.status().is_success() {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;
        anyhow::bail!(
            "Failed to open llama.cpp model event stream: {} {}",
            response.status(),
            body,
        );
    }

    let reader = BufReader::new(response.into_body());
    Ok(reader
        .lines()
        .filter_map(|line| async move {
            // Each event is a single `data:` line carrying the JSON envelope;
            // other SSE lines (comments, blank separators) are ignored.
            let line = line.ok()?;
            let payload = line.strip_prefix("data:")?.trim_start();
            Some(serde_json::from_str::<ModelEvent>(payload).map_err(|error| anyhow!(error)))
        })
        .boxed())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_model_listing() {
        let response = serde_json::json!({
            "object": "list",
            "data": [
                {
                    "id": "../models/Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf",
                    "object": "model",
                    "created": 1735142223u64,
                    "owned_by": "llamacpp",
                    "meta": {
                        "n_ctx": 8192,
                        "n_ctx_train": 131072,
                        "n_embd": 4096,
                    }
                }
            ]
        });
        let response: ListModelsResponse = serde_json::from_value(response).unwrap();
        assert_eq!(response.data.len(), 1);
        let entry = &response.data[0];
        assert!(!entry.is_router_entry());
        assert_eq!(entry.meta.as_ref().unwrap().n_ctx, Some(8192));
        assert_eq!(entry.meta.as_ref().unwrap().n_ctx_train, Some(131072));
    }

    #[test]
    fn parse_router_model_listing() {
        let response = serde_json::json!({
            "object": "list",
            "data": [
                {
                    "id": "qwen2.5-coder",
                    "object": "model",
                    "owned_by": "llamacpp",
                    "status": { "value": "loaded", "args": [] },
                    "architecture": {
                        "input_modalities": ["text", "image"],
                        "output_modalities": ["text"]
                    }
                },
                {
                    "id": "gemma-3",
                    "object": "model",
                    "owned_by": "llamacpp",
                    "status": { "value": "unloaded" },
                    "architecture": { "input_modalities": ["text"] }
                }
            ]
        });
        let response: ListModelsResponse = serde_json::from_value(response).unwrap();
        assert_eq!(response.data.len(), 2);

        let loaded = &response.data[0];
        assert!(loaded.is_router_entry());
        assert!(loaded.is_loaded());
        assert!(loaded.supports_images_hint());

        let unloaded = &response.data[1];
        assert!(unloaded.is_router_entry());
        assert!(!unloaded.is_loaded());
        assert!(!unloaded.supports_images_hint());
    }

    #[test]
    fn parse_props() {
        let response = serde_json::json!({
            "default_generation_settings": {
                "id": 0,
                "n_ctx": 8192,
                "params": {}
            },
            "total_slots": 1,
            "model_path": "../models/Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf",
            "modalities": { "vision": true, "audio": false },
            // The real `/props` reports both keys; parsing must not treat that as
            // a duplicate field.
            "chat_template_caps": {
                "supports_tools": true,
                "supports_tool_calls": true,
                "supports_preserve_reasoning": true,
                "supports_system_role": true
            }
        });
        let props: Props = serde_json::from_value(response).unwrap();
        assert_eq!(props.context_length(), Some(8192));
        assert!(props.supports_images());
        assert!(props.supports_tools());
        assert!(props.supports_thinking());
    }

    fn model_event(value: serde_json::Value) -> ModelEvent {
        serde_json::from_value(value).unwrap()
    }

    #[test]
    fn model_event_state_changes() {
        // Terminal load/unload and list changes warrant re-discovery.
        assert!(
            model_event(serde_json::json!({
                "model": "m", "event": "status_change", "data": { "status": "loaded" }
            }))
            .changes_model_state()
        );
        assert!(
            model_event(serde_json::json!({
                "model": "m", "event": "status_change", "data": { "status": "unloaded" }
            }))
            .changes_model_state()
        );
        assert!(
            model_event(serde_json::json!({ "model": "*", "event": "models_reload" }))
                .changes_model_state()
        );
        assert!(
            model_event(serde_json::json!({ "model": "m", "event": "model_remove" }))
                .changes_model_state()
        );

        // Intermediate loading-progress ticks do not.
        assert!(
            !model_event(serde_json::json!({
                "model": "m", "event": "status_change",
                "data": { "status": "loading", "progress": { "value": 0.4 } }
            }))
            .changes_model_state()
        );
        assert!(
            !model_event(serde_json::json!({ "model": "m", "event": "download_progress" }))
                .changes_model_state()
        );
    }

    #[test]
    fn model_event_load_failure() {
        let failed = model_event(serde_json::json!({
            "model": "m", "event": "status_change", "data": { "status": "unloaded", "exit_code": 1 }
        }));
        assert_eq!(failed.load_failure(), Some(1));

        // A clean unload (exit code 0 or absent) is not a failure.
        let clean = model_event(serde_json::json!({
            "model": "m", "event": "status_change", "data": { "status": "unloaded", "exit_code": 0 }
        }));
        assert_eq!(clean.load_failure(), None);
        let loaded = model_event(serde_json::json!({
            "model": "m", "event": "status_change", "data": { "status": "loaded" }
        }));
        assert_eq!(loaded.load_failure(), None);
    }

    #[test]
    fn model_event_load_progress() {
        // We report the current stage's own value (0→1), not a blended overall.
        let weights = model_event(serde_json::json!({
            "model": "m", "event": "status_change",
            "data": { "status": "loading",
                      "progress": { "stages": ["text_model"], "current": "text_model", "value": 0.4 } }
        }));
        let progress = weights.load_progress().unwrap();
        assert!((progress.value - 0.4).abs() < 1e-4);
        assert_eq!(progress.stage_label(), "Loading weights");
        assert_eq!(progress.progress_label(), "Loading weights 40%");

        // The projector stage runs 0→1 on its own (not 90→100%).
        let projector = model_event(serde_json::json!({
            "model": "m", "event": "status_change",
            "data": { "status": "loading",
                      "progress": { "stages": ["text_model", "mmproj_model"],
                                    "current": "mmproj_model", "value": 0.5 } }
        }));
        let progress = projector.load_progress().unwrap();
        assert!((progress.value - 0.5).abs() < 1e-4);
        assert_eq!(progress.stage_label(), "Loading projector");

        // Non-loading events carry no progress.
        let loaded = model_event(serde_json::json!({
            "model": "m", "event": "status_change", "data": { "status": "loaded" }
        }));
        assert!(loaded.load_progress().is_none());

        // Bare stage-transition markers (no stages/current/value) are skipped so
        // the indicator doesn't flicker to 0% between stages.
        let transition = model_event(serde_json::json!({
            "model": "m", "event": "status_change",
            "data": { "status": "loading", "progress": { "stage": "mmproj_model" } }
        }));
        assert!(transition.load_progress().is_none());
    }

    #[test]
    fn parse_streaming_reasoning_and_tool_calls() {
        let event = serde_json::json!({
            "model": "llama",
            "object": "chat.completion.chunk",
            "choices": [
                {
                    "index": 0,
                    "delta": {
                        "role": "assistant",
                        "content": null,
                        "reasoning_content": "thinking...",
                        "tool_calls": [
                            {
                                "index": 0,
                                "id": "call_1",
                                "function": { "name": "weather", "arguments": "{\"city\":" }
                            }
                        ]
                    },
                    "finish_reason": null
                }
            ]
        });
        let event: ResponseStreamEvent = serde_json::from_value(event).unwrap();
        let delta = &event.choices[0].delta;
        assert_eq!(delta.reasoning_content.as_deref(), Some("thinking..."));
        assert_eq!(delta.tool_calls.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn encodes_router_model_id_for_props_query() {
        // Router ids contain `/` and `:`, which must be URL-encoded in the
        // `?model=` query, while unreserved characters (`.`, `-`) stay literal.
        let url = Url::parse_with_params(
            "http://localhost:8080/props",
            [("model", "unsloth/Qwen3.5-2B-GGUF:Q4_1")],
        )
        .unwrap();
        assert_eq!(
            url.as_str(),
            "http://localhost:8080/props?model=unsloth%2FQwen3.5-2B-GGUF%3AQ4_1"
        );
    }
}
