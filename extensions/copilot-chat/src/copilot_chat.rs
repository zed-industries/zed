use std::collections::HashMap;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use zed_extension_api::http_client::{HttpMethod, HttpRequest, HttpResponseStream, RedirectPolicy};
use zed_extension_api::{self as zed, *};

const GITHUB_DEVICE_CODE_URL: &str = "https://github.com/login/device/code";
const GITHUB_ACCESS_TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
const GITHUB_COPILOT_TOKEN_URL: &str = "https://api.github.com/copilot_internal/v2/token";
const GITHUB_COPILOT_CLIENT_ID: &str = "Iv1.b507a08c87ecfe98";

struct DeviceFlowState {
    device_code: String,
    interval: u64,
    expires_in: u64,
}

#[derive(Clone)]
struct ApiToken {
    api_key: String,
    api_endpoint: String,
}

#[derive(Clone, Deserialize)]
struct CopilotModel {
    id: String,
    name: String,
    #[serde(default)]
    is_chat_default: bool,
    #[serde(default)]
    is_chat_fallback: bool,
    #[serde(default)]
    model_picker_enabled: bool,
    #[serde(default)]
    capabilities: ModelCapabilities,
    #[serde(default)]
    policy: Option<ModelPolicy>,
}

#[derive(Clone, Default, Deserialize)]
struct ModelCapabilities {
    #[serde(default)]
    family: String,
    #[serde(default)]
    limits: ModelLimits,
    #[serde(default)]
    supports: ModelSupportedFeatures,
    #[serde(rename = "type", default)]
    model_type: String,
}

#[derive(Clone, Default, Deserialize)]
struct ModelLimits {
    #[serde(default)]
    max_context_window_tokens: u64,
    #[serde(default)]
    max_output_tokens: u64,
}

#[derive(Clone, Default, Deserialize)]
struct ModelSupportedFeatures {
    #[serde(default)]
    streaming: bool,
    #[serde(default)]
    tool_calls: bool,
    #[serde(default)]
    vision: bool,
}

#[derive(Clone, Deserialize)]
struct ModelPolicy {
    state: String,
}

struct CopilotChatProvider {
    streams: Mutex<HashMap<String, StreamState>>,
    next_stream_id: Mutex<u64>,
    device_flow_state: Mutex<Option<DeviceFlowState>>,
    api_token: Mutex<Option<ApiToken>>,
    cached_models: Mutex<Option<Vec<CopilotModel>>>,
}

struct StreamState {
    response_stream: Option<HttpResponseStream>,
    buffer: String,
    started: bool,
    tool_calls: HashMap<usize, AccumulatedToolCall>,
    tool_calls_emitted: bool,
}

#[derive(Clone, Default)]
struct AccumulatedToolCall {
    id: String,
    name: String,
    arguments: String,
}

#[derive(Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<OpenAiMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<OpenAiTool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    stop: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream_options: Option<StreamOptions>,
}

#[derive(Serialize)]
struct StreamOptions {
    include_usage: bool,
}

#[derive(Serialize)]
struct OpenAiMessage {
    role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<OpenAiContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<OpenAiToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
}

#[derive(Serialize, Clone)]
#[serde(untagged)]
enum OpenAiContent {
    Text(String),
    Parts(Vec<OpenAiContentPart>),
}

#[derive(Serialize, Clone)]
#[serde(tag = "type")]
enum OpenAiContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrl },
}

#[derive(Serialize, Clone)]
struct ImageUrl {
    url: String,
}

#[derive(Serialize, Clone)]
struct OpenAiToolCall {
    id: String,
    #[serde(rename = "type")]
    call_type: String,
    function: OpenAiFunctionCall,
}

#[derive(Serialize, Clone)]
struct OpenAiFunctionCall {
    name: String,
    arguments: String,
}

#[derive(Serialize)]
struct OpenAiTool {
    #[serde(rename = "type")]
    tool_type: String,
    function: OpenAiFunctionDef,
}

#[derive(Serialize)]
struct OpenAiFunctionDef {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Deserialize, Debug)]
struct OpenAiStreamResponse {
    choices: Vec<OpenAiStreamChoice>,
    #[serde(default)]
    usage: Option<OpenAiUsage>,
}

#[derive(Deserialize, Debug)]
struct OpenAiStreamChoice {
    delta: OpenAiDelta,
    finish_reason: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
struct OpenAiDelta {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    tool_calls: Option<Vec<OpenAiToolCallDelta>>,
}

#[derive(Deserialize, Debug)]
struct OpenAiToolCallDelta {
    index: usize,
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    function: Option<OpenAiFunctionDelta>,
}

#[derive(Deserialize, Debug, Default)]
struct OpenAiFunctionDelta {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    arguments: Option<String>,
}

#[derive(Deserialize, Debug)]
struct OpenAiUsage {
    prompt_tokens: u64,
    completion_tokens: u64,
}

fn convert_request(
    model_id: &str,
    request: &LlmCompletionRequest,
) -> Result<OpenAiRequest, String> {
    let mut messages: Vec<OpenAiMessage> = Vec::new();

    for msg in &request.messages {
        match msg.role {
            LlmMessageRole::System => {
                let mut text_content = String::new();
                for content in &msg.content {
                    if let LlmMessageContent::Text(text) = content {
                        if !text_content.is_empty() {
                            text_content.push('\n');
                        }
                        text_content.push_str(text);
                    }
                }
                if !text_content.is_empty() {
                    messages.push(OpenAiMessage {
                        role: "system".to_string(),
                        content: Some(OpenAiContent::Text(text_content)),
                        tool_calls: None,
                        tool_call_id: None,
                    });
                }
            }
            LlmMessageRole::User => {
                let mut parts: Vec<OpenAiContentPart> = Vec::new();
                let mut tool_result_messages: Vec<OpenAiMessage> = Vec::new();

                for content in &msg.content {
                    match content {
                        LlmMessageContent::Text(text) => {
                            if !text.is_empty() {
                                parts.push(OpenAiContentPart::Text { text: text.clone() });
                            }
                        }
                        LlmMessageContent::Image(img) => {
                            let data_url = format!("data:image/png;base64,{}", img.source);
                            parts.push(OpenAiContentPart::ImageUrl {
                                image_url: ImageUrl { url: data_url },
                            });
                        }
                        LlmMessageContent::ToolResult(result) => {
                            let content_text = match &result.content {
                                LlmToolResultContent::Text(t) => t.clone(),
                                LlmToolResultContent::Image(_) => "[Image]".to_string(),
                            };
                            tool_result_messages.push(OpenAiMessage {
                                role: "tool".to_string(),
                                content: Some(OpenAiContent::Text(content_text)),
                                tool_calls: None,
                                tool_call_id: Some(result.tool_use_id.clone()),
                            });
                        }
                        _ => {}
                    }
                }

                if !parts.is_empty() {
                    let content = if parts.len() == 1 {
                        if let OpenAiContentPart::Text { text } = &parts[0] {
                            OpenAiContent::Text(text.clone())
                        } else {
                            OpenAiContent::Parts(parts)
                        }
                    } else {
                        OpenAiContent::Parts(parts)
                    };

                    messages.push(OpenAiMessage {
                        role: "user".to_string(),
                        content: Some(content),
                        tool_calls: None,
                        tool_call_id: None,
                    });
                }

                messages.extend(tool_result_messages);
            }
            LlmMessageRole::Assistant => {
                let mut text_content = String::new();
                let mut tool_calls: Vec<OpenAiToolCall> = Vec::new();

                for content in &msg.content {
                    match content {
                        LlmMessageContent::Text(text) => {
                            if !text.is_empty() {
                                if !text_content.is_empty() {
                                    text_content.push('\n');
                                }
                                text_content.push_str(text);
                            }
                        }
                        LlmMessageContent::ToolUse(tool_use) => {
                            tool_calls.push(OpenAiToolCall {
                                id: tool_use.id.clone(),
                                call_type: "function".to_string(),
                                function: OpenAiFunctionCall {
                                    name: tool_use.name.clone(),
                                    arguments: tool_use.input.clone(),
                                },
                            });
                        }
                        _ => {}
                    }
                }

                messages.push(OpenAiMessage {
                    role: "assistant".to_string(),
                    content: if text_content.is_empty() {
                        None
                    } else {
                        Some(OpenAiContent::Text(text_content))
                    },
                    tool_calls: if tool_calls.is_empty() {
                        None
                    } else {
                        Some(tool_calls)
                    },
                    tool_call_id: None,
                });
            }
        }
    }

    let tools: Vec<OpenAiTool> = request
        .tools
        .iter()
        .map(|t| OpenAiTool {
            tool_type: "function".to_string(),
            function: OpenAiFunctionDef {
                name: t.name.clone(),
                description: t.description.clone(),
                parameters: serde_json::from_str(&t.input_schema)
                    .unwrap_or(serde_json::Value::Object(Default::default())),
            },
        })
        .collect();

    let tool_choice = request.tool_choice.as_ref().map(|tc| match tc {
        LlmToolChoice::Auto => "auto".to_string(),
        LlmToolChoice::Any => "required".to_string(),
        LlmToolChoice::None => "none".to_string(),
    });

    let max_tokens = request.max_tokens;

    Ok(OpenAiRequest {
        model: model_id.to_string(),
        messages,
        max_tokens,
        tools,
        tool_choice,
        stop: request.stop_sequences.clone(),
        temperature: request.temperature,
        stream: true,
        stream_options: Some(StreamOptions {
            include_usage: true,
        }),
    })
}

fn parse_sse_line(line: &str) -> Option<OpenAiStreamResponse> {
    let data = line.strip_prefix("data: ")?;
    if data.trim() == "[DONE]" {
        return None;
    }
    serde_json::from_str(data).ok()
}

impl zed::Extension for CopilotChatProvider {
    fn new() -> Self {
        Self {
            streams: Mutex::new(HashMap::new()),
            next_stream_id: Mutex::new(0),
            device_flow_state: Mutex::new(None),
            api_token: Mutex::new(None),
            cached_models: Mutex::new(None),
        }
    }

    fn llm_providers(&self) -> Vec<LlmProviderInfo> {
        vec![LlmProviderInfo {
            id: "copilot-chat".into(),
            name: "Copilot Chat".into(),
            icon: Some("icons/copilot.svg".into()),
        }]
    }

    fn llm_provider_models(&self, _provider_id: &str) -> Result<Vec<LlmModelInfo>, String> {
        // Try to get models from cache first
        if let Some(models) = self.cached_models.lock().unwrap().as_ref() {
            return Ok(convert_models_to_llm_info(models));
        }

        // Need to fetch models - requires authentication
        let oauth_token = match llm_get_credential("copilot-chat") {
            Some(token) => token,
            None => return Ok(Vec::new()), // Not authenticated, return empty
        };

        // Get API token
        let api_token = self.get_api_token(&oauth_token)?;

        // Fetch models from API
        let models = self.fetch_models(&api_token)?;

        // Cache the models
        *self.cached_models.lock().unwrap() = Some(models.clone());

        Ok(convert_models_to_llm_info(&models))
    }

    fn llm_provider_is_authenticated(&self, _provider_id: &str) -> bool {
        llm_get_credential("copilot-chat").is_some()
    }

    fn llm_provider_settings_markdown(&self, _provider_id: &str) -> Option<String> {
        Some(
            "To use Copilot Chat, sign in with your GitHub account. This requires an active [GitHub Copilot subscription](https://github.com/features/copilot).".to_string(),
        )
    }

    fn llm_provider_start_device_flow_sign_in(
        &mut self,
        _provider_id: &str,
    ) -> Result<String, String> {
        // Step 1: Request device and user verification codes
        let device_code_response = llm_oauth_http_request(&LlmOauthHttpRequest {
            url: GITHUB_DEVICE_CODE_URL.to_string(),
            method: "POST".to_string(),
            headers: vec![
                ("Accept".to_string(), "application/json".to_string()),
                (
                    "Content-Type".to_string(),
                    "application/x-www-form-urlencoded".to_string(),
                ),
            ],
            body: format!("client_id={}&scope=read:user", GITHUB_COPILOT_CLIENT_ID),
        })?;

        if device_code_response.status != 200 {
            return Err(format!(
                "Failed to get device code: HTTP {}",
                device_code_response.status
            ));
        }

        #[derive(Deserialize)]
        struct DeviceCodeResponse {
            device_code: String,
            user_code: String,
            verification_uri: String,
            #[serde(default)]
            verification_uri_complete: Option<String>,
            expires_in: u64,
            interval: u64,
        }

        let device_info: DeviceCodeResponse = serde_json::from_str(&device_code_response.body)
            .map_err(|e| format!("Failed to parse device code response: {}", e))?;

        // Store device flow state for polling
        *self.device_flow_state.lock().unwrap() = Some(DeviceFlowState {
            device_code: device_info.device_code,
            interval: device_info.interval,
            expires_in: device_info.expires_in,
        });

        // Step 2: Open browser to verification URL
        // Use verification_uri_complete if available (has code pre-filled), otherwise construct URL
        let verification_url = device_info.verification_uri_complete.unwrap_or_else(|| {
            format!(
                "{}?user_code={}",
                device_info.verification_uri, &device_info.user_code
            )
        });
        llm_oauth_open_browser(&verification_url)?;

        // Return the user code for the host to display
        Ok(device_info.user_code)
    }

    fn llm_provider_poll_device_flow_sign_in(&mut self, _provider_id: &str) -> Result<(), String> {
        let state = self
            .device_flow_state
            .lock()
            .unwrap()
            .take()
            .ok_or("No device flow in progress")?;

        let poll_interval = Duration::from_secs(state.interval.max(5));
        let max_attempts = (state.expires_in / state.interval.max(5)) as usize;

        for _ in 0..max_attempts {
            thread::sleep(poll_interval);

            let token_response = llm_oauth_http_request(&LlmOauthHttpRequest {
                url: GITHUB_ACCESS_TOKEN_URL.to_string(),
                method: "POST".to_string(),
                headers: vec![
                    ("Accept".to_string(), "application/json".to_string()),
                    (
                        "Content-Type".to_string(),
                        "application/x-www-form-urlencoded".to_string(),
                    ),
                ],
                body: format!(
                    "client_id={}&device_code={}&grant_type=urn:ietf:params:oauth:grant-type:device_code",
                    GITHUB_COPILOT_CLIENT_ID, state.device_code
                ),
            })?;

            #[derive(Deserialize)]
            struct TokenResponse {
                access_token: Option<String>,
                error: Option<String>,
                error_description: Option<String>,
            }

            let token_json: TokenResponse = serde_json::from_str(&token_response.body)
                .map_err(|e| format!("Failed to parse token response: {}", e))?;

            if let Some(access_token) = token_json.access_token {
                llm_store_credential("copilot-chat", &access_token)?;
                return Ok(());
            }

            if let Some(error) = &token_json.error {
                match error.as_str() {
                    "authorization_pending" => {
                        // User hasn't authorized yet, keep polling
                        continue;
                    }
                    "slow_down" => {
                        // Need to slow down polling
                        thread::sleep(Duration::from_secs(5));
                        continue;
                    }
                    "expired_token" => {
                        return Err("Device code expired. Please try again.".to_string());
                    }
                    "access_denied" => {
                        return Err("Authorization was denied.".to_string());
                    }
                    _ => {
                        let description = token_json.error_description.unwrap_or_default();
                        return Err(format!("OAuth error: {} - {}", error, description));
                    }
                }
            }
        }

        Err("Authorization timed out. Please try again.".to_string())
    }

    fn llm_provider_reset_credentials(&mut self, _provider_id: &str) -> Result<(), String> {
        // Clear cached API token and models
        *self.api_token.lock().unwrap() = None;
        *self.cached_models.lock().unwrap() = None;
        llm_delete_credential("copilot-chat")
    }

    fn llm_stream_completion_start(
        &mut self,
        _provider_id: &str,
        model_id: &str,
        request: &LlmCompletionRequest,
    ) -> Result<String, String> {
        let oauth_token = llm_get_credential("copilot-chat").ok_or_else(|| {
            "No token configured. Please add your GitHub Copilot token in settings.".to_string()
        })?;

        // Get or refresh API token
        let api_token = self.get_api_token(&oauth_token)?;

        let openai_request = convert_request(model_id, request)?;

        let body = serde_json::to_vec(&openai_request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;

        let completions_url = format!("{}/chat/completions", api_token.api_endpoint);

        let http_request = HttpRequest {
            method: HttpMethod::Post,
            url: completions_url,
            headers: vec![
                ("Content-Type".to_string(), "application/json".to_string()),
                (
                    "Authorization".to_string(),
                    format!("Bearer {}", api_token.api_key),
                ),
                (
                    "Copilot-Integration-Id".to_string(),
                    "vscode-chat".to_string(),
                ),
                ("Editor-Version".to_string(), "Zed/1.0.0".to_string()),
            ],
            body: Some(body),
            redirect_policy: RedirectPolicy::FollowAll,
        };

        let response_stream = http_request
            .fetch_stream()
            .map_err(|e| format!("HTTP request failed: {}", e))?;

        let stream_id = {
            let mut id_counter = self.next_stream_id.lock().unwrap();
            let id = format!("copilot-stream-{}", *id_counter);
            *id_counter += 1;
            id
        };

        self.streams.lock().unwrap().insert(
            stream_id.clone(),
            StreamState {
                response_stream: Some(response_stream),
                buffer: String::new(),
                started: false,
                tool_calls: HashMap::new(),
                tool_calls_emitted: false,
            },
        );

        Ok(stream_id)
    }

    fn llm_stream_completion_next(
        &mut self,
        stream_id: &str,
    ) -> Result<Option<LlmCompletionEvent>, String> {
        let mut streams = self.streams.lock().unwrap();
        let state = streams
            .get_mut(stream_id)
            .ok_or_else(|| format!("Unknown stream: {}", stream_id))?;

        if !state.started {
            state.started = true;
            return Ok(Some(LlmCompletionEvent::Started));
        }

        let response_stream = state
            .response_stream
            .as_mut()
            .ok_or_else(|| "Stream already closed".to_string())?;

        loop {
            if let Some(newline_pos) = state.buffer.find('\n') {
                let line = state.buffer[..newline_pos].to_string();
                state.buffer = state.buffer[newline_pos + 1..].to_string();

                if line.trim().is_empty() {
                    continue;
                }

                if let Some(response) = parse_sse_line(&line) {
                    if let Some(choice) = response.choices.first() {
                        if let Some(content) = &choice.delta.content {
                            if !content.is_empty() {
                                return Ok(Some(LlmCompletionEvent::Text(content.clone())));
                            }
                        }

                        if let Some(tool_calls) = &choice.delta.tool_calls {
                            for tc in tool_calls {
                                let entry = state
                                    .tool_calls
                                    .entry(tc.index)
                                    .or_insert_with(AccumulatedToolCall::default);

                                if let Some(id) = &tc.id {
                                    entry.id = id.clone();
                                }
                                if let Some(func) = &tc.function {
                                    if let Some(name) = &func.name {
                                        entry.name = name.clone();
                                    }
                                    if let Some(args) = &func.arguments {
                                        entry.arguments.push_str(args);
                                    }
                                }
                            }
                        }

                        if let Some(finish_reason) = &choice.finish_reason {
                            if !state.tool_calls.is_empty() && !state.tool_calls_emitted {
                                state.tool_calls_emitted = true;
                                let mut tool_calls: Vec<_> = state.tool_calls.drain().collect();
                                tool_calls.sort_by_key(|(idx, _)| *idx);

                                if let Some((_, tc)) = tool_calls.into_iter().next() {
                                    return Ok(Some(LlmCompletionEvent::ToolUse(LlmToolUse {
                                        id: tc.id,
                                        name: tc.name,
                                        input: tc.arguments,
                                        thought_signature: None,
                                    })));
                                }
                            }

                            let stop_reason = match finish_reason.as_str() {
                                "stop" => LlmStopReason::EndTurn,
                                "length" => LlmStopReason::MaxTokens,
                                "tool_calls" => LlmStopReason::ToolUse,
                                "content_filter" => LlmStopReason::Refusal,
                                _ => LlmStopReason::EndTurn,
                            };
                            return Ok(Some(LlmCompletionEvent::Stop(stop_reason)));
                        }
                    }

                    if let Some(usage) = response.usage {
                        return Ok(Some(LlmCompletionEvent::Usage(LlmTokenUsage {
                            input_tokens: usage.prompt_tokens,
                            output_tokens: usage.completion_tokens,
                            cache_creation_input_tokens: None,
                            cache_read_input_tokens: None,
                        })));
                    }
                }

                continue;
            }

            match response_stream.next_chunk() {
                Ok(Some(chunk)) => {
                    let text = String::from_utf8_lossy(&chunk);
                    state.buffer.push_str(&text);
                }
                Ok(None) => {
                    return Ok(None);
                }
                Err(e) => {
                    return Err(format!("Stream error: {}", e));
                }
            }
        }
    }

    fn llm_stream_completion_close(&mut self, stream_id: &str) {
        self.streams.lock().unwrap().remove(stream_id);
    }
}

impl CopilotChatProvider {
    fn get_api_token(&self, oauth_token: &str) -> Result<ApiToken, String> {
        // Check if we have a cached token
        if let Some(token) = self.api_token.lock().unwrap().clone() {
            return Ok(token);
        }

        // Request a new API token
        let http_request = HttpRequest {
            method: HttpMethod::Get,
            url: GITHUB_COPILOT_TOKEN_URL.to_string(),
            headers: vec![
                (
                    "Authorization".to_string(),
                    format!("token {}", oauth_token),
                ),
                ("Accept".to_string(), "application/json".to_string()),
            ],
            body: None,
            redirect_policy: RedirectPolicy::FollowAll,
        };

        let response = http_request
            .fetch()
            .map_err(|e| format!("Failed to request API token: {}", e))?;

        #[derive(Deserialize)]
        struct ApiTokenResponse {
            token: String,
            endpoints: ApiEndpoints,
        }

        #[derive(Deserialize)]
        struct ApiEndpoints {
            api: String,
        }

        let token_response: ApiTokenResponse =
            serde_json::from_slice(&response.body).map_err(|e| {
                format!(
                    "Failed to parse API token response: {} - body: {}",
                    e,
                    String::from_utf8_lossy(&response.body)
                )
            })?;

        let api_token = ApiToken {
            api_key: token_response.token,
            api_endpoint: token_response.endpoints.api,
        };

        // Cache the token
        *self.api_token.lock().unwrap() = Some(api_token.clone());

        Ok(api_token)
    }

    fn fetch_models(&self, api_token: &ApiToken) -> Result<Vec<CopilotModel>, String> {
        let models_url = format!("{}/models", api_token.api_endpoint);

        let http_request = HttpRequest {
            method: HttpMethod::Get,
            url: models_url,
            headers: vec![
                (
                    "Authorization".to_string(),
                    format!("Bearer {}", api_token.api_key),
                ),
                ("Content-Type".to_string(), "application/json".to_string()),
                (
                    "Copilot-Integration-Id".to_string(),
                    "vscode-chat".to_string(),
                ),
                ("Editor-Version".to_string(), "Zed/1.0.0".to_string()),
                ("x-github-api-version".to_string(), "2025-05-01".to_string()),
            ],
            body: None,
            redirect_policy: RedirectPolicy::FollowAll,
        };

        let response = http_request
            .fetch()
            .map_err(|e| format!("Failed to fetch models: {}", e))?;

        #[derive(Deserialize)]
        struct ModelsResponse {
            data: Vec<CopilotModel>,
        }

        let models_response: ModelsResponse =
            serde_json::from_slice(&response.body).map_err(|e| {
                format!(
                    "Failed to parse models response: {} - body: {}",
                    e,
                    String::from_utf8_lossy(&response.body)
                )
            })?;

        // Filter models like the built-in Copilot Chat does
        let mut models: Vec<CopilotModel> = models_response
            .data
            .into_iter()
            .filter(|model| {
                model.model_picker_enabled
                    && model.capabilities.model_type == "chat"
                    && model
                        .policy
                        .as_ref()
                        .map(|p| p.state == "enabled")
                        .unwrap_or(true)
            })
            .collect();

        // Sort so default model is first
        if let Some(pos) = models.iter().position(|m| m.is_chat_default) {
            let default_model = models.remove(pos);
            models.insert(0, default_model);
        }

        Ok(models)
    }
}

fn convert_models_to_llm_info(models: &[CopilotModel]) -> Vec<LlmModelInfo> {
    models
        .iter()
        .map(|m| {
            let max_tokens = if m.capabilities.limits.max_context_window_tokens > 0 {
                m.capabilities.limits.max_context_window_tokens
            } else {
                128_000 // Default fallback
            };
            let max_output = if m.capabilities.limits.max_output_tokens > 0 {
                Some(m.capabilities.limits.max_output_tokens)
            } else {
                None
            };

            LlmModelInfo {
                id: m.id.clone(),
                name: m.name.clone(),
                max_token_count: max_tokens,
                max_output_tokens: max_output,
                capabilities: LlmModelCapabilities {
                    supports_images: m.capabilities.supports.vision,
                    supports_tools: m.capabilities.supports.tool_calls,
                    supports_tool_choice_auto: m.capabilities.supports.tool_calls,
                    supports_tool_choice_any: m.capabilities.supports.tool_calls,
                    supports_tool_choice_none: m.capabilities.supports.tool_calls,
                    supports_thinking: false,
                    tool_input_format: LlmToolInputFormat::JsonSchema,
                },
                is_default: m.is_chat_default,
                is_default_fast: m.is_chat_fallback,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_flow_request_body() {
        let body = format!("client_id={}&scope=read:user", GITHUB_COPILOT_CLIENT_ID);
        assert!(body.contains("client_id=Iv1.b507a08c87ecfe98"));
        assert!(body.contains("scope=read:user"));
    }

    #[test]
    fn test_token_poll_request_body() {
        let device_code = "test_device_code_123";
        let body = format!(
            "client_id={}&device_code={}&grant_type=urn:ietf:params:oauth:grant-type:device_code",
            GITHUB_COPILOT_CLIENT_ID, device_code
        );
        assert!(body.contains("client_id=Iv1.b507a08c87ecfe98"));
        assert!(body.contains("device_code=test_device_code_123"));
        assert!(body.contains("grant_type=urn:ietf:params:oauth:grant-type:device_code"));
    }
}

zed::register_extension!(CopilotChatProvider);
