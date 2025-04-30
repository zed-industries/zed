use std::time::Duration;

use anyhow::{Result, anyhow, bail};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use time::OffsetDateTime;

pub const API_URL: &str = "https://generativelanguage.googleapis.com";

pub async fn stream_generate_content(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    mut request: GenerateContentRequest,
) -> Result<BoxStream<'static, Result<GenerateContentResponse>>> {
    if request.contents.is_empty() {
        bail!("Request must contain at least one content item");
    }

    if let Some(user_content) = request
        .contents
        .iter()
        .find(|content| content.role == Role::User)
    {
        if user_content.parts.is_empty() {
            bail!("User content must contain at least one part");
        }
    }

    let uri = format!(
        "{api_url}/v1beta/models/{model}:streamGenerateContent?alt=sse&key={api_key}",
        model = request.model
    );
    request.model.clear();

    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json");

    let request = request_builder.body(AsyncBody::from(serde_json::to_string(&request)?))?;
    let mut response = client.send(request).await?;
    if response.status().is_success() {
        let reader = BufReader::new(response.into_body());
        Ok(reader
            .lines()
            .filter_map(|line| async move {
                match line {
                    Ok(line) => {
                        if let Some(line) = line.strip_prefix("data: ") {
                            match serde_json::from_str(line) {
                                Ok(response) => Some(Ok(response)),
                                Err(error) => Some(Err(anyhow!(format!(
                                    "Error parsing JSON: {:?}\n{:?}",
                                    error, line
                                )))),
                            }
                        } else {
                            None
                        }
                    }
                    Err(error) => Some(Err(anyhow!(error))),
                }
            })
            .boxed())
    } else {
        let mut text = String::new();
        response.body_mut().read_to_string(&mut text).await?;
        Err(anyhow!(
            "error during Gemini content generation, status code: {:?}, body: {}",
            response.status(),
            text
        ))
    }
}

pub async fn count_tokens(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    model_id: &str,
    request: CountTokensRequest,
) -> Result<CountTokensResponse> {
    let uri = format!("{api_url}/v1beta/models/{model_id}:countTokens?key={api_key}",);
    let request = serde_json::to_string(&request)?;

    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(&uri)
        .header("Content-Type", "application/json");

    let http_request = request_builder.body(AsyncBody::from(request))?;
    let mut response = client.send(http_request).await?;
    let mut text = String::new();
    response.body_mut().read_to_string(&mut text).await?;
    if response.status().is_success() {
        Ok(serde_json::from_str::<CountTokensResponse>(&text)?)
    } else {
        Err(anyhow!(
            "error during Gemini token counting, status code: {:?}, body: {}",
            response.status(),
            text
        ))
    }
}

pub async fn create_cache(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: CreateCacheRequest,
) -> Result<CreateCacheResponse> {
    if let Some(user_content) = request
        .contents
        .iter()
        .find(|content| content.role == Role::User)
    {
        if user_content.parts.is_empty() {
            bail!("User content must contain at least one part");
        }
    }
    let uri = format!("{api_url}/v1beta/cachedContents?key={api_key}");
    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json");

    let http_request = request_builder.body(AsyncBody::from(serde_json::to_string(&request)?))?;
    let mut response = client.send(http_request).await?;
    let mut text = String::new();
    response.body_mut().read_to_string(&mut text).await?;
    if response.status().is_success() {
        Ok(serde_json::from_str::<CreateCacheResponse>(&text)?)
    } else {
        Err(anyhow!(
            "error during Gemini cache creation, status code: {:?}, body: {}",
            response.status(),
            text
        ))
    }
}

pub async fn update_cache(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    cache_name: &CacheName,
    request: UpdateCacheRequest,
) -> Result<UpdateCacheResponse> {
    let uri = format!(
        "{api_url}/v1beta/cachedContents/{}?key={api_key}",
        &cache_name.0
    );
    let request_builder = HttpRequest::builder()
        .method(Method::PATCH)
        .uri(uri)
        .header("Content-Type", "application/json");

    let http_request = request_builder.body(AsyncBody::from(serde_json::to_string(&request)?))?;
    let mut response = client.send(http_request).await?;
    let mut text = String::new();
    response.body_mut().read_to_string(&mut text).await?;
    if response.status().is_success() {
        Ok(serde_json::from_str::<UpdateCacheResponse>(&text)?)
    } else {
        Err(anyhow!(
            "error during Gemini cache update, status code: {:?}, body: {}",
            response.status(),
            text
        ))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Task {
    #[serde(rename = "generateContent")]
    GenerateContent,
    #[serde(rename = "streamGenerateContent")]
    StreamGenerateContent,
    #[serde(rename = "countTokens")]
    CountTokens,
    #[serde(rename = "embedContent")]
    EmbedContent,
    #[serde(rename = "batchEmbedContents")]
    BatchEmbedContents,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentRequest {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub model: String,
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

#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Role {
    User,
    Model,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Part {
    TextPart(TextPart),
    InlineDataPart(InlineDataPart),
    FunctionCallPart(FunctionCallPart),
    FunctionResponsePart(FunctionResponsePart),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextPart {
    pub text: String,
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
    pub safety_ratings: Vec<SafetyRating>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_reason_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UsageMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_token_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_content_token_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidates_token_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_use_prompt_token_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thoughts_token_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_token_count: Option<usize>,
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
#[serde(rename_all = "camelCase")]
pub struct CountTokensRequest {
    pub contents: Vec<Content>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountTokensResponse {
    pub total_tokens: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub args: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionResponse {
    pub name: String,
    pub response: serde_json::Value,
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
    pub parameters: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCacheRequest {
    #[serde(
        serialize_with = "serialize_duration",
        deserialize_with = "deserialize_duration"
    )]
    pub ttl: Duration,
    pub model: String,
    pub contents: Vec<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_instruction: Option<Content>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Tool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_config: Option<ToolConfig>,
    // Other fields that could be provided:
    //
    // name: The resource name referring to the cached content. Format: cachedContents/{id}
    // display_name: user-generated meaningful display name of the cached content. Maximum 128 Unicode characters.
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCacheResponse {
    pub name: CacheName,
    #[serde(
        serialize_with = "time::serde::rfc3339::serialize",
        deserialize_with = "time::serde::rfc3339::deserialize"
    )]
    pub expire_time: OffsetDateTime,
    pub usage_metadata: UsageMetadata,
    // Other fields that could be provided:
    //
    // create_time: Creation time of the cache entry.
    // update_time: When the cache entry was last updated in UTC time.
    // usage_metadata: Metadata on the usage of the cached content.
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCacheRequest {
    #[serde(
        serialize_with = "serialize_duration",
        deserialize_with = "deserialize_duration"
    )]
    pub ttl: Duration,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCacheResponse {
    #[serde(
        serialize_with = "time::serde::rfc3339::serialize",
        deserialize_with = "time::serde::rfc3339::deserialize"
    )]
    pub expire_time: OffsetDateTime,
}

#[derive(Debug)]
pub struct CacheName(String);

const CACHE_NAME_PREFIX: &str = "cachedContents/";

impl Serialize for CacheName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{CACHE_NAME_PREFIX}{}", &self.0))
    }
}

impl<'de> Deserialize<'de> for CacheName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        if let Some(name) = string.strip_prefix(CACHE_NAME_PREFIX) {
            Ok(CacheName(name.to_string()))
        } else {
            return Err(serde::de::Error::custom(format!(
                "Expected cache name to begin with {}, got: {}",
                CACHE_NAME_PREFIX, string
            )));
        }
    }
}

/// Serializes a Duration as a string in the format "X.Ys" where X is the whole seconds
/// and Y is up to 9 decimal places of fractional seconds.
pub fn serialize_duration<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let secs = duration.as_secs();
    let nanos = duration.subsec_nanos();

    // Format with only the necessary decimal places (up to 9)
    let formatted = if nanos == 0 {
        format!("{}s", secs)
    } else {
        // Remove trailing zeros from nanos
        let mut nanos_str = format!("{:09}", nanos);
        while nanos_str.ends_with('0') && nanos_str.len() > 1 {
            nanos_str.pop();
        }
        format!("{}.{}s", secs, nanos_str)
    };

    serializer.serialize_str(&formatted)
}

pub fn deserialize_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let duration_str = String::deserialize(deserializer)?;

    let Some(num_part) = duration_str.strip_suffix('s') else {
        return Err(serde::de::Error::custom(format!(
            "Duration must end with 's', got: {}",
            duration_str
        )));
    };

    // Check if the string contains a decimal point
    if let Some(decimal_ix) = num_part.find('.') {
        let secs_part = &num_part[0..decimal_ix];
        let frac_len = (num_part.len() - (decimal_ix + 1)).min(9);
        let frac_start_ix = decimal_ix + 1;
        let frac_end_ix = frac_start_ix + frac_len;
        let frac_part = &num_part[frac_start_ix..frac_end_ix];

        let secs = u64::from_str_radix(secs_part, 10).map_err(|e| {
            serde::de::Error::custom(format!(
                "Invalid seconds in duration: {}. Error: {}",
                duration_str, e
            ))
        })?;

        let frac_number = frac_part.parse::<u32>().map_err(|e| {
            serde::de::Error::custom(format!(
                "Invalid fractional seconds in duration: {}. Error: {}",
                duration_str, e
            ))
        })?;

        let nanos = frac_number * 10u32.pow(9 - frac_len as u32);

        Ok(Duration::new(secs, nanos))
    } else {
        // No decimal point, just whole seconds
        let secs = u64::from_str_radix(num_part, 10).map_err(|e| {
            serde::de::Error::custom(format!(
                "Invalid duration format: {}. Error: {}",
                duration_str, e
            ))
        })?;

        Ok(Duration::new(secs, 0))
    }
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq, Eq, strum::EnumIter)]
pub enum Model {
    #[serde(rename = "gemini-1.5-pro")]
    Gemini15Pro,
    #[serde(rename = "gemini-1.5-flash")]
    Gemini15Flash,
    /// Note: replaced by `gemini-2.5-pro-exp-03-25` (continues to work in API).
    #[serde(rename = "gemini-2.0-pro-exp")]
    Gemini20Pro,
    #[serde(rename = "gemini-2.0-flash")]
    #[default]
    Gemini20Flash,
    /// Note: replaced by `gemini-2.5-flash-preview-04-17` (continues to work in API).
    #[serde(rename = "gemini-2.0-flash-thinking-exp")]
    Gemini20FlashThinking,
    /// Note: replaced by `gemini-2.0-flash-lite` (continues to work in API).
    #[serde(rename = "gemini-2.0-flash-lite-preview")]
    Gemini20FlashLite,
    #[serde(rename = "gemini-2.5-pro-exp-03-25")]
    Gemini25ProExp0325,
    #[serde(rename = "gemini-2.5-pro-preview-03-25")]
    Gemini25ProPreview0325,
    #[serde(rename = "gemini-2.5-flash-preview-04-17")]
    Gemini25FlashPreview0417,
    #[serde(rename = "custom")]
    Custom {
        name: String,
        /// The name displayed in the UI, such as in the assistant panel model dropdown menu.
        display_name: Option<String>,
        max_tokens: usize,
        caching: bool,
    },
}

impl Model {
    pub fn default_fast() -> Model {
        Model::Gemini15Flash
    }

    pub fn id(&self) -> &str {
        match self {
            Model::Gemini15Pro => "gemini-1.5-pro",
            Model::Gemini15Flash => "gemini-1.5-flash",
            Model::Gemini20Pro => "gemini-2.0-pro-exp",
            Model::Gemini20Flash => "gemini-2.0-flash",
            Model::Gemini20FlashThinking => "gemini-2.0-flash-thinking-exp",
            Model::Gemini20FlashLite => "gemini-2.0-flash-lite-preview",
            Model::Gemini25ProExp0325 => "gemini-2.5-pro-exp-03-25",
            Model::Gemini25ProPreview0325 => "gemini-2.5-pro-preview-03-25",
            Model::Gemini25FlashPreview0417 => "gemini-2.5-flash-preview-04-17",
            Model::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Model::Gemini15Pro => "Gemini 1.5 Pro",
            Model::Gemini15Flash => "Gemini 1.5 Flash",
            Model::Gemini20Pro => "Gemini 2.0 Pro",
            Model::Gemini20Flash => "Gemini 2.0 Flash",
            Model::Gemini20FlashThinking => "Gemini 2.0 Flash Thinking",
            Model::Gemini20FlashLite => "Gemini 2.0 Flash Lite",
            Model::Gemini25ProExp0325 => "Gemini 2.5 Pro Exp",
            Model::Gemini25ProPreview0325 => "Gemini 2.5 Pro Preview",
            Model::Gemini25FlashPreview0417 => "Gemini 2.5 Flash Preview",
            Self::Custom {
                name, display_name, ..
            } => display_name.as_ref().unwrap_or(name),
        }
    }

    pub fn max_token_count(&self) -> usize {
        const ONE_MILLION: usize = 1_048_576;
        const TWO_MILLION: usize = 2_097_152;
        match self {
            Model::Gemini15Pro => TWO_MILLION,
            Model::Gemini15Flash => ONE_MILLION,
            Model::Gemini20Pro => TWO_MILLION,
            Model::Gemini20Flash => ONE_MILLION,
            Model::Gemini20FlashThinking => ONE_MILLION,
            Model::Gemini20FlashLite => ONE_MILLION,
            Model::Gemini25ProExp0325 => ONE_MILLION,
            Model::Gemini25ProPreview0325 => ONE_MILLION,
            Model::Gemini25FlashPreview0417 => ONE_MILLION,
            Model::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    // todo! From a blog post:
    //
    // > Context caching only works with stable models with fixed versions. (Think
    // “gemini-1.5-pro-001”, not just “gemini-1.5-pro”).
    //
    // Is this still true?
    pub fn caching(&self) -> bool {
        match self {
            Model::Gemini15Pro => true,
            Model::Gemini15Flash => true,
            Model::Gemini20Pro => true,
            Model::Gemini20Flash => true,
            // TODO: Check again whether this now supports caching (note it's replaced by
            // Gemini25FlashPreview0417).
            Model::Gemini20FlashThinking => false,
            // todo! https://ai.google.dev/gemini-api/docs/pricing#gemini-2.0-flash-lite says "Not
            // available" for this, but
            // https://ai.google.dev/gemini-api/docs/models#gemini-2.0-flash-lite says caching is
            // supported.
            Model::Gemini20FlashLite => true,
            Model::Gemini25ProExp0325 => true,
            Model::Gemini25ProPreview0325 => true,
            // TODO: Check again whether this now supports caching
            // (https://ai.google.dev/gemini-api/docs/pricing says "Coming soon!")
            Model::Gemini25FlashPreview0417 => false,
            Model::Custom { caching, .. } => *caching,
        }
    }
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration_serialization() {
        #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
        struct Example {
            #[serde(
                serialize_with = "serialize_duration",
                deserialize_with = "deserialize_duration"
            )]
            duration: Duration,
        }

        let example = Example {
            duration: Duration::from_secs(5),
        };
        let serialized = serde_json::to_string(&example).unwrap();
        let deserialized: Example = serde_json::from_str(&serialized).unwrap();
        assert_eq!(serialized, r#"{"duration":"5s"}"#);
        assert_eq!(deserialized, example);

        let example = Example {
            duration: Duration::from_millis(5534),
        };
        let serialized = serde_json::to_string(&example).unwrap();
        let deserialized: Example = serde_json::from_str(&serialized).unwrap();
        assert_eq!(serialized, r#"{"duration":"5.534s"}"#);
        assert_eq!(deserialized, example);

        let example = Example {
            duration: Duration::from_nanos(12345678900),
        };
        let serialized = serde_json::to_string(&example).unwrap();
        let deserialized: Example = serde_json::from_str(&serialized).unwrap();
        assert_eq!(serialized, r#"{"duration":"12.3456789s"}"#);
        assert_eq!(deserialized, example);

        // Deserializer doesn't panic for too many fractional digits
        let deserialized: Example =
            serde_json::from_str(r#"{"duration":"5.12345678905s"}"#).unwrap();
        assert_eq!(
            deserialized,
            Example {
                duration: Duration::from_nanos(5123456789)
            }
        );
    }
}
