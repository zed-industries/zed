use std::mem;

use anyhow::{Result, bail};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::{
    AsyncBody, CustomHeaders, HttpClient, Method, Request as HttpRequest, RequestBuilderExt,
    StatusCode, Url, http::HeaderMap,
};
pub use language_model_core::{GOOGLE_AI_API_URL as API_URL, ModelMode as GoogleModelMode};
use language_model_core::{
    GOOGLE_PROVIDER_NAME, LanguageModelCompletionError, ProviderErrorCategory,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
pub mod completion;

// Google's model catalog reports token limits and whether a model thinks, but not the facts
// below. Update these lists when Google releases or retires models.

/// Catalog entries Zed doesn't offer: models that produce media or serve specialized APIs,
/// and `-latest` aliases, which silently retarget to models already listed by version.
///
/// Matched by ID segment so that new versions of these families stay excluded.
fn is_excluded_model(model_id: &str) -> bool {
    model_id.split('-').any(|segment| {
        matches!(
            segment,
            "latest"
                | "image"
                | "banana"
                | "tts"
                | "audio"
                | "live"
                | "transcribe"
                | "customtools"
                | "computer"
                | "robotics"
                | "research"
                | "omni"
                | "embedding"
        )
    })
}

/// Models that budget thinking in tokens rather than accepting thinking levels.
fn uses_thinking_budget(model_id: &str) -> bool {
    matches!(
        model_id,
        "gemini-2.5-pro" | "gemini-2.5-flash" | "gemini-2.5-flash-lite"
    )
}

/// Thinking-budget models whose thinking can be turned off with a zero budget.
pub(crate) fn supports_disabling_thinking(model_id: &str) -> bool {
    matches!(model_id, "gemini-2.5-flash" | "gemini-2.5-flash-lite")
}

/// Models that take tool result media as sibling parts of the function response.
///
/// Google documents media nested under `functionResponse.parts` for Gemini 3 and later, which
/// reject sibling media with "Requests ending with a model turn are not supported".
pub(crate) fn uses_sibling_function_response_media(model_id: &str) -> bool {
    matches!(
        model_id,
        "gemini-2.5-pro" | "gemini-2.5-flash" | "gemini-2.5-flash-lite"
    )
}

/// Level-based models that accept `MINIMAL`; the others reject it and start at `LOW`.
fn supports_minimal_thinking(model_id: &str) -> bool {
    matches!(
        model_id,
        "gemini-3-flash-preview"
            | "gemini-3.5-flash"
            | "gemini-3.5-flash-lite"
            | "gemini-3.6-flash"
    )
}

/// The thinking level Zed selects for a level-based model until the user picks another.
fn default_thinking_level(model_id: &str) -> ThinkingLevel {
    match model_id {
        "gemini-3.5-flash-lite" => ThinkingLevel::Minimal,
        "gemini-3-flash-preview" | "gemini-3.1-pro-preview" => ThinkingLevel::High,
        _ => ThinkingLevel::Medium,
    }
}

/// The least thinking a model accepts, sent when a request asks for none but the model can't
/// turn thinking off. `None` for thinking-budget models.
pub(crate) fn lowest_thinking_level(model_id: &str) -> Option<ThinkingLevel> {
    if uses_thinking_budget(model_id) {
        None
    } else if supports_minimal_thinking(model_id) {
        Some(ThinkingLevel::Minimal)
    } else {
        Some(ThinkingLevel::Low)
    }
}

/// Whether the ID has a version after `gemini-`, as in `gemini-3.1-pro-preview`.
///
/// Every Gemini text model is versioned. Unversioned IDs such as `gemini-flash-latest` or
/// `gemini-nano-banana-2.1` are aliases or other model families, so Zed doesn't offer them.
fn is_versioned_model(model_id: &str) -> bool {
    model_id
        .strip_prefix("gemini-")
        .and_then(|rest| rest.split_once('-'))
        .is_some_and(|(version, _)| {
            version
                .split('.')
                .all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit()))
        })
}

/// Fetches the models available to the API key, following every catalog page.
///
/// Entries that aren't general-purpose text models are skipped; see [`Model::from_listed`].
///
/// See <https://ai.google.dev/api/models#method:-models.list>.
///
/// # Errors
/// Returns a completion error if a request fails or a response cannot be decoded.
pub async fn list_models(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    extra_headers: &CustomHeaders,
) -> Result<Vec<Model>, LanguageModelCompletionError> {
    let mut models = Vec::new();
    let mut page_token: Option<String> = None;
    loop {
        // `v1` omits preview models such as `gemini-3.1-pro-preview`, which completions
        // reach through `v1beta`, so list them from the same API version.
        let mut url = Url::parse(&format!("{}/v1beta/models", api_url.trim_end_matches('/')))
            .map_err(|error| LanguageModelCompletionError::Other(error.into()))?;
        url.query_pairs_mut().append_pair("pageSize", "1000");
        if let Some(token) = &page_token {
            url.query_pairs_mut().append_pair("pageToken", token);
        }
        let request = HttpRequest::builder()
            .method(Method::GET)
            .uri(url.as_str())
            .header("x-goog-api-key", api_key.trim())
            .header("Accept", "application/json")
            .extra_headers(extra_headers)
            .body(AsyncBody::empty())
            .map_err(|error| LanguageModelCompletionError::BuildRequestBody {
                provider: GOOGLE_PROVIDER_NAME,
                error,
            })?;
        let mut response =
            client
                .send(request)
                .await
                .map_err(|error| LanguageModelCompletionError::HttpSend {
                    provider: GOOGLE_PROVIDER_NAME,
                    host: url.host_str().unwrap_or(api_url).to_owned(),
                    error,
                })?;
        let status = response.status();
        let headers = response.headers().clone();
        let mut body = String::new();
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(|error| LanguageModelCompletionError::ApiReadResponseError {
                provider: GOOGLE_PROVIDER_NAME,
                error,
            })?;
        if !status.is_success() {
            return Err(completion_error_from_response(status, &headers, body));
        }
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Page {
            #[serde(default)]
            models: Vec<ListModelEntry>,
            next_page_token: Option<String>,
        }
        let page: Page = serde_json::from_str(&body).map_err(|error| {
            LanguageModelCompletionError::DeserializeResponse {
                provider: GOOGLE_PROVIDER_NAME,
                error,
            }
        })?;
        models.extend(page.models.into_iter().filter_map(Model::from_listed));
        page_token = page.next_page_token.filter(|token| !token.is_empty());
        if page_token.is_none() {
            return Ok(models);
        }
    }
}

pub async fn stream_generate_content(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    mut request: GenerateContentRequest,
    extra_headers: &CustomHeaders,
) -> Result<
    BoxStream<'static, Result<GenerateContentResponse, LanguageModelCompletionError>>,
    LanguageModelCompletionError,
> {
    let api_key = api_key.trim();
    validate_generate_content_request(&request)?;

    // The `model` field is emptied as it is provided as a path parameter.
    let model_id = mem::take(&mut request.model.model_id);

    let uri =
        format!("{api_url}/v1beta/models/{model_id}:streamGenerateContent?alt=sse&key={api_key}",);

    let request = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .extra_headers(extra_headers)
        .body(AsyncBody::from(serde_json::to_string(&request).map_err(
            |error| LanguageModelCompletionError::SerializeRequest {
                provider: GOOGLE_PROVIDER_NAME,
                error,
            },
        )?))
        .map_err(|error| LanguageModelCompletionError::BuildRequestBody {
            provider: GOOGLE_PROVIDER_NAME,
            error,
        })?;
    let mut response =
        client
            .send(request)
            .await
            .map_err(|error| LanguageModelCompletionError::HttpSend {
                provider: GOOGLE_PROVIDER_NAME,
                host: api_url.to_string(),
                error,
            })?;
    if response.status().is_success() {
        let reader = BufReader::new(response.into_body());
        Ok(reader
            .lines()
            .filter_map(|line| async move {
                match line {
                    Ok(line) => {
                        if let Some(line) = line.strip_prefix("data: ") {
                            // Some proxies and gateways append `data: [DONE]` as a
                            // stream-termination sentinel (an OpenAI convention).
                            // Google's streaming spec does not include this sentinel,
                            // but we tolerate it here for the same reason as the
                            // anthropic crate: proxies that inject it would otherwise
                            // cause a spurious JSON deserialization error.
                            if line.trim() == "[DONE]" {
                                return None;
                            }
                            match serde_json::from_str(line) {
                                Ok(response) => Some(Ok(response)),
                                Err(error) => {
                                    Some(Err(LanguageModelCompletionError::DeserializeResponse {
                                        provider: GOOGLE_PROVIDER_NAME,
                                        error,
                                    }))
                                }
                            }
                        } else {
                            None
                        }
                    }
                    Err(error) => Some(Err(LanguageModelCompletionError::ApiReadResponseError {
                        provider: GOOGLE_PROVIDER_NAME,
                        error,
                    })),
                }
            })
            .boxed())
    } else {
        let status = response.status();
        let headers = response.headers().clone();
        let mut body = String::new();
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(|error| LanguageModelCompletionError::ApiReadResponseError {
                provider: GOOGLE_PROVIDER_NAME,
                error,
            })?;
        Err(completion_error_from_response(status, &headers, body))
    }
}

fn completion_error_from_response(
    status: StatusCode,
    headers: &HeaderMap,
    body: String,
) -> LanguageModelCompletionError {
    let retry_after = headers
        .get(http_client::http::header::RETRY_AFTER)
        .and_then(|value| value.to_str().ok()?.parse::<u64>().ok())
        .map(std::time::Duration::from_secs);
    let parsed_error = serde_json::from_str::<GoogleErrorEnvelope>(&body)
        .ok()
        .map(|envelope| envelope.error);
    let message = parsed_error
        .as_ref()
        .map(|error| error.message.clone())
        .unwrap_or(body);
    // Google uses HTTP 400 / INVALID_ARGUMENT even when the API key is invalid.
    let invalid_api_key = status == StatusCode::BAD_REQUEST
        && (parsed_error.as_ref().is_some_and(|error| {
            error
                .details
                .iter()
                .any(|detail| detail.reason.as_deref() == Some("API_KEY_INVALID"))
        }) || message.starts_with("API key not valid."));
    let code = parsed_error.and_then(|error| error.status);
    let category = if invalid_api_key {
        ProviderErrorCategory::Authentication
    } else {
        ProviderErrorCategory::from_http_status(status, &message)
    };

    LanguageModelCompletionError::from_provider_response(
        GOOGLE_PROVIDER_NAME,
        Some(status),
        code,
        message,
        retry_after,
        category,
    )
}

#[derive(Deserialize)]
struct GoogleErrorEnvelope {
    error: GoogleError,
}

#[derive(Deserialize)]
struct GoogleError {
    message: String,
    status: Option<String>,
    #[serde(default)]
    details: Vec<GoogleErrorDetail>,
}

#[derive(Deserialize)]
struct GoogleErrorDetail {
    reason: Option<String>,
}

#[cfg(test)]
mod provider_error_tests {
    use super::*;

    #[test]
    fn invalid_api_keys_are_authentication_errors_not_invalid_requests() {
        for (body, expected_category) in [
            (
                r#"{"error":{"code":400,"message":"Invalid credentials.","status":"INVALID_ARGUMENT","details":[{"@type":"type.googleapis.com/google.rpc.ErrorInfo","reason":"API_KEY_INVALID","domain":"googleapis.com"}]}}"#,
                ProviderErrorCategory::Authentication,
            ),
            (
                r#"{"error":{"code":400,"message":"API key not valid. Please pass a valid API key.","status":"INVALID_ARGUMENT"}}"#,
                ProviderErrorCategory::Authentication,
            ),
            (
                r#"{"error":{"code":400,"message":"Invalid value at 'contents'.","status":"INVALID_ARGUMENT","details":[{"@type":"type.googleapis.com/google.rpc.BadRequest","fieldViolations":[]}]}}"#,
                ProviderErrorCategory::InvalidRequest,
            ),
        ] {
            let error = completion_error_from_response(
                StatusCode::BAD_REQUEST,
                &HeaderMap::new(),
                body.to_string(),
            );
            assert!(matches!(
                error,
                LanguageModelCompletionError::ProviderRejection {
                    status: Some(StatusCode::BAD_REQUEST),
                    code: Some(code),
                    category,
                    ..
                } if code == "INVALID_ARGUMENT" && category == expected_category
            ));
        }
    }

    #[test]
    fn google_rejections_preserve_wire_details_and_semantic_category() {
        let mut headers = HeaderMap::new();
        headers.insert(
            http_client::http::header::RETRY_AFTER,
            http_client::http::HeaderValue::from_static("7"),
        );
        let error = completion_error_from_response(
            StatusCode::SERVICE_UNAVAILABLE,
            &headers,
            r#"{"error":{"code":503,"message":"The model is overloaded. Please try again later.","status":"UNAVAILABLE"}}"#
                .to_string(),
        );

        assert!(matches!(
            error,
            LanguageModelCompletionError::ProviderRejection {
                provider,
                status: Some(StatusCode::SERVICE_UNAVAILABLE),
                code: Some(code),
                message,
                retry_after: Some(retry_after),
                category: ProviderErrorCategory::Overloaded,
            } if provider == GOOGLE_PROVIDER_NAME
                && code == "UNAVAILABLE"
                && message == "The model is overloaded. Please try again later."
                && retry_after == std::time::Duration::from_secs(7)
        ));
    }

    #[test]
    fn google_rejections_use_shared_http_classification() {
        for (status, expected_category) in [
            (
                StatusCode::UNAUTHORIZED,
                ProviderErrorCategory::Authentication,
            ),
            (
                StatusCode::PAYMENT_REQUIRED,
                ProviderErrorCategory::PaymentRequired,
            ),
            (
                StatusCode::TOO_MANY_REQUESTS,
                ProviderErrorCategory::RateLimit,
            ),
        ] {
            let error = completion_error_from_response(
                status,
                &HeaderMap::new(),
                r#"{"error":{"code":0,"message":"rejected","status":"REJECTED"}}"#.to_string(),
            );
            assert!(matches!(
                error,
                LanguageModelCompletionError::ProviderRejection { category, .. }
                    if category == expected_category
            ));
        }
    }
}

pub fn validate_generate_content_request(request: &GenerateContentRequest) -> Result<()> {
    if request.model.is_empty() {
        bail!("Model must be specified");
    }

    if request.contents.is_empty() {
        bail!("Request must contain at least one content item");
    }

    if let Some(user_content) = request
        .contents
        .iter()
        .find(|content| content.role == Role::User)
        && user_content.parts.is_empty()
    {
        bail!("User content must contain at least one part");
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Task {
    #[serde(rename = "generateContent")]
    GenerateContent,
    #[serde(rename = "streamGenerateContent")]
    StreamGenerateContent,
    #[serde(rename = "embedContent")]
    EmbedContent,
    #[serde(rename = "batchEmbedContents")]
    BatchEmbedContents,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentRequest {
    #[serde(default, skip_serializing_if = "ModelName::is_empty")]
    pub model: ModelName,
    pub contents: Vec<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_instruction: Option<SystemInstruction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<GenerationConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_settings: Option<Vec<SafetySetting>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_config: Option<ToolConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidates: Option<Vec<GenerateContentCandidate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_feedback: Option<PromptFeedback>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_metadata: Option<UsageMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentCandidate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
    pub content: Content,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_ratings: Option<Vec<SafetyRating>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citation_metadata: Option<CitationMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    #[serde(default)]
    pub parts: Vec<Part>,
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInstruction {
    pub parts: Vec<Part>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Role {
    User,
    Model,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Part {
    FunctionCallPart(FunctionCallPart),
    FunctionResponsePart(FunctionResponsePart),
    InlineDataPart(InlineDataPart),
    TextPart(TextPart),
}

fn is_false(value: &bool) -> bool {
    !*value
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextPart {
    pub text: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub thought: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thought_signature: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineDataPart {
    pub inline_data: GenerativeContentBlob,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerativeContentBlob {
    pub mime_type: String,
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCallPart {
    pub function_call: FunctionCall,
    /// Thought signature returned by the model for function calls.
    /// Only present on the first function call in parallel call scenarios.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thought_signature: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionResponsePart {
    pub function_response: FunctionResponse,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationMetadata {
    pub citation_sources: Vec<CitationSource>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptFeedback {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_reason: Option<String>,
    pub safety_ratings: Option<Vec<SafetyRating>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_reason_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UsageMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_token_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_content_token_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidates_token_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_use_prompt_token_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thoughts_token_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_token_count: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ThinkingConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking_budget: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking_level: Option<ThinkingLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_thoughts: Option<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ThinkingLevel {
    Minimal,
    Low,
    Medium,
    High,
}

impl ThinkingLevel {
    pub fn from_effort(effort: &str) -> Option<Self> {
        match effort.to_lowercase().as_str() {
            "minimal" => Some(Self::Minimal),
            "low" => Some(Self::Low),
            "medium" => Some(Self::Medium),
            "high" => Some(Self::High),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Minimal => "Minimal",
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
        }
    }

    pub fn value(self) -> &'static str {
        match self {
            Self::Minimal => "minimal",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidate_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking_config: Option<ThinkingConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SafetySetting {
    pub category: HarmCategory,
    pub threshold: HarmBlockThreshold,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum HarmCategory {
    #[serde(rename = "HARM_CATEGORY_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "HARM_CATEGORY_DEROGATORY")]
    Derogatory,
    #[serde(rename = "HARM_CATEGORY_TOXICITY")]
    Toxicity,
    #[serde(rename = "HARM_CATEGORY_VIOLENCE")]
    Violence,
    #[serde(rename = "HARM_CATEGORY_SEXUAL")]
    Sexual,
    #[serde(rename = "HARM_CATEGORY_MEDICAL")]
    Medical,
    #[serde(rename = "HARM_CATEGORY_DANGEROUS")]
    Dangerous,
    #[serde(rename = "HARM_CATEGORY_HARASSMENT")]
    Harassment,
    #[serde(rename = "HARM_CATEGORY_HATE_SPEECH")]
    HateSpeech,
    #[serde(rename = "HARM_CATEGORY_SEXUALLY_EXPLICIT")]
    SexuallyExplicit,
    #[serde(rename = "HARM_CATEGORY_DANGEROUS_CONTENT")]
    DangerousContent,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmBlockThreshold {
    #[serde(rename = "HARM_BLOCK_THRESHOLD_UNSPECIFIED")]
    Unspecified,
    BlockLowAndAbove,
    BlockMediumAndAbove,
    BlockOnlyHigh,
    BlockNone,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmProbability {
    #[serde(rename = "HARM_PROBABILITY_UNSPECIFIED")]
    Unspecified,
    Negligible,
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SafetyRating {
    pub category: HarmCategory,
    pub probability: HarmProbability,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub args: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionResponse {
    pub name: String,
    pub response: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parts: Vec<InlineDataPart>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tool {
    pub function_declarations: Vec<FunctionDeclaration>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolConfig {
    pub function_calling_config: FunctionCallingConfig,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCallingConfig {
    pub mode: FunctionCallingMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_function_names: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FunctionCallingMode {
    Auto,
    Any,
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionDeclaration {
    pub name: String,
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[serde(
        rename = "parametersJsonSchema",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parameters_json_schema: Option<serde_json::Value>,
}

#[derive(Debug, Default)]
pub struct ModelName {
    pub model_id: String,
}

impl ModelName {
    pub fn is_empty(&self) -> bool {
        self.model_id.is_empty()
    }
}

const MODEL_NAME_PREFIX: &str = "models/";

impl Serialize for ModelName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{MODEL_NAME_PREFIX}{}", self.model_id))
    }
}

impl<'de> Deserialize<'de> for ModelName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        if let Some(id) = string.strip_prefix(MODEL_NAME_PREFIX) {
            Ok(Self {
                model_id: id.to_string(),
            })
        } else {
            Err(serde::de::Error::custom(format!(
                "Expected model name to begin with {}, got: {}",
                MODEL_NAME_PREFIX, string
            )))
        }
    }
}

/// A Gemini model and the request settings Zed uses for it.
#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    pub id: String,
    pub display_name: String,
    pub max_input_tokens: u64,
    pub max_output_tokens: Option<u64>,
    pub mode: GoogleModelMode,
    /// Empty for models that budget thinking by tokens instead of by level.
    pub supported_thinking_levels: Vec<ThinkingLevel>,
    pub default_thinking_level: Option<ThinkingLevel>,
}

impl Model {
    /// Builds a model from a catalog entry, or `None` if Zed doesn't offer it.
    fn from_listed(entry: ListModelEntry) -> Option<Self> {
        let id = entry.name.strip_prefix("models/")?;
        let generates_content = entry
            .supported_generation_methods
            .iter()
            .any(|method| method == "generateContent");
        if !is_versioned_model(id) || is_excluded_model(id) || !generates_content {
            return None;
        }
        let max_input_tokens = entry.input_token_limit.filter(|limit| *limit > 0)?;
        let max_output_tokens = entry.output_token_limit.filter(|limit| *limit > 0)?;

        let (mode, supported_thinking_levels, default_thinking_level) =
            match (entry.thinking, lowest_thinking_level(id)) {
                (false, _) => (GoogleModelMode::Default, Vec::new(), None),
                (true, None) => (
                    GoogleModelMode::Thinking {
                        budget_tokens: None,
                    },
                    Vec::new(),
                    None,
                ),
                (true, Some(lowest)) => (
                    GoogleModelMode::Thinking {
                        budget_tokens: None,
                    },
                    [
                        ThinkingLevel::Minimal,
                        ThinkingLevel::Low,
                        ThinkingLevel::Medium,
                        ThinkingLevel::High,
                    ]
                    .into_iter()
                    .skip_while(|level| *level != lowest)
                    .collect(),
                    Some(default_thinking_level(id)),
                ),
            };

        Some(Self {
            id: id.to_string(),
            display_name: entry.display_name.unwrap_or_else(|| id.to_string()),
            max_input_tokens,
            max_output_tokens: Some(max_output_tokens),
            mode,
            supported_thinking_levels,
            default_thinking_level,
        })
    }

    pub fn supports_thinking(&self) -> bool {
        matches!(self.mode, GoogleModelMode::Thinking { .. })
    }

    /// Whether the model accepts requests that turn off reasoning entirely.
    pub fn supports_disabling_thinking(&self) -> bool {
        supports_disabling_thinking(&self.id)
    }
}

/// An entry returned by the `models.list` endpoint.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListModelEntry {
    name: String,
    display_name: Option<String>,
    input_token_limit: Option<u64>,
    output_token_limit: Option<u64>,
    #[serde(default)]
    supported_generation_methods: Vec<String>,
    #[serde(default)]
    thinking: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use completion::GoogleEventMapper;
    use futures::{TryStreamExt, stream};
    use http_client::{FakeHttpClient, Response};
    use language_model_core::LanguageModelCompletionEvent;
    use serde_json::json;
    use std::io::{Error, ErrorKind};

    #[test]
    fn stream_body_failures_remain_retryable() {
        for kind in [ErrorKind::TimedOut, ErrorKind::ConnectionReset] {
            let error = completion_error(Some(kind));
            assert!(error.is_transient());
            assert!(matches!(
                error,
                LanguageModelCompletionError::ApiReadResponseError { provider, error }
                    if provider == GOOGLE_PROVIDER_NAME && error.kind() == kind
            ));
        }
    }

    #[test]
    fn malformed_stream_data_remains_non_retryable() {
        let error = completion_error(None);
        assert!(!error.is_transient());
        assert!(matches!(
            error,
            LanguageModelCompletionError::DeserializeResponse { provider, .. }
                if provider == GOOGLE_PROVIDER_NAME
        ));
    }

    fn completion_error(read_error: Option<ErrorKind>) -> LanguageModelCompletionError {
        futures::executor::block_on(async {
            let client = FakeHttpClient::create(move |_| async move {
                let chunks = vec![
                    Ok(b"data: {\"candidates\":[{\"content\":{\"role\":\"model\",\"parts\":[{\"text\":\"Thinking\"}]}}]}\n\n".to_vec()),
                    match read_error {
                        Some(kind) => Err(Error::new(kind, "stream read failed")),
                        None => Ok(b"data: not json\n\n".to_vec()),
                    },
                ];
                Ok(Response::builder().body(AsyncBody::from_reader(
                    stream::iter(chunks).into_async_read(),
                ))?)
            });
            let request = serde_json::from_value(json!({
                "model": "models/gemini-3.8-flash",
                "contents": [{"role": "user", "parts": [{"text": "Think"}]}]
            }))
            .unwrap();
            let responses = stream_generate_content(
                client.as_ref(),
                API_URL,
                "test-key",
                request,
                &CustomHeaders::default(),
            )
            .await
            .unwrap();
            let mut events = GoogleEventMapper::new().map_stream(responses).boxed();
            assert!(matches!(
                events.next().await,
                Some(Ok(LanguageModelCompletionEvent::Text(text))) if text == "Thinking"
            ));
            let error = events.next().await.unwrap().unwrap_err();
            assert!(events.next().await.is_none());
            error
        })
    }

    #[test]
    fn test_lowest_thinking_level_per_model() {
        assert_eq!(
            lowest_thinking_level("gemini-3.6-flash"),
            Some(ThinkingLevel::Minimal)
        );
        assert_eq!(
            lowest_thinking_level("gemini-3.8-flash"),
            Some(ThinkingLevel::Low)
        );
        assert_eq!(
            lowest_thinking_level("gemini-4-argon"),
            Some(ThinkingLevel::Low)
        );
        assert_eq!(lowest_thinking_level("gemini-2.5-flash"), None);
    }

    #[test]
    fn test_is_versioned_model() {
        assert!(is_versioned_model("gemini-3.1-pro-preview"));
        assert!(is_versioned_model("gemini-4-argon"));
        assert!(!is_versioned_model("gemini-flash-latest"));
        assert!(!is_versioned_model("gemini-nano-banana-2.1"));
    }

    #[test]
    fn test_function_call_part_with_signature_serializes_correctly() {
        let part = FunctionCallPart {
            function_call: FunctionCall {
                name: "test_function".to_string(),
                args: json!({"arg": "value"}),
                id: None,
            },
            thought_signature: Some("test_signature".to_string()),
        };

        let serialized = serde_json::to_value(&part).unwrap();

        assert_eq!(serialized["functionCall"]["name"], "test_function");
        assert_eq!(serialized["functionCall"]["args"]["arg"], "value");
        assert_eq!(serialized["thoughtSignature"], "test_signature");
    }

    #[test]
    fn test_function_call_part_without_signature_omits_field() {
        let part = FunctionCallPart {
            function_call: FunctionCall {
                name: "test_function".to_string(),
                args: json!({"arg": "value"}),
                id: None,
            },
            thought_signature: None,
        };

        let serialized = serde_json::to_value(&part).unwrap();

        assert_eq!(serialized["functionCall"]["name"], "test_function");
        assert_eq!(serialized["functionCall"]["args"]["arg"], "value");
        // thoughtSignature field should not be present when None
        assert!(serialized.get("thoughtSignature").is_none());
    }

    #[test]
    fn test_function_call_part_deserializes_with_signature() {
        let json = json!({
            "functionCall": {
                "name": "test_function",
                "args": {"arg": "value"}
            },
            "thoughtSignature": "test_signature"
        });

        let part: FunctionCallPart = serde_json::from_value(json).unwrap();

        assert_eq!(part.function_call.name, "test_function");
        assert_eq!(part.thought_signature, Some("test_signature".to_string()));
    }

    #[test]
    fn test_function_call_part_deserializes_without_signature() {
        let json = json!({
            "functionCall": {
                "name": "test_function",
                "args": {"arg": "value"}
            }
        });

        let part: FunctionCallPart = serde_json::from_value(json).unwrap();

        assert_eq!(part.function_call.name, "test_function");
        assert_eq!(part.thought_signature, None);
    }

    #[test]
    fn test_function_call_part_round_trip() {
        let original = FunctionCallPart {
            function_call: FunctionCall {
                name: "test_function".to_string(),
                args: json!({"arg": "value", "nested": {"key": "val"}}),
                id: None,
            },
            thought_signature: Some("round_trip_signature".to_string()),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: FunctionCallPart = serde_json::from_value(serialized).unwrap();

        assert_eq!(deserialized.function_call.name, original.function_call.name);
        assert_eq!(deserialized.function_call.args, original.function_call.args);
        assert_eq!(deserialized.thought_signature, original.thought_signature);
    }

    #[test]
    fn test_function_call_part_with_empty_signature_serializes() {
        let part = FunctionCallPart {
            function_call: FunctionCall {
                name: "test_function".to_string(),
                args: json!({"arg": "value"}),
                id: None,
            },
            thought_signature: Some("".to_string()),
        };

        let serialized = serde_json::to_value(&part).unwrap();

        // Empty string should still be serialized (normalization happens at a higher level)
        assert_eq!(serialized["thoughtSignature"], "");
    }
}
