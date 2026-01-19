use std::mem;

use anyhow::{Result, anyhow, bail};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
pub use settings::ModelMode as GoogleModelMode;

pub const API_URL: &str = "https://generativelanguage.googleapis.com";

pub async fn stream_generate_content(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    mut request: GenerateContentRequest,
) -> Result<BoxStream<'static, Result<GenerateContentResponse>>> {
    let api_key = api_key.trim();
    validate_generate_content_request(&request)?;

    // The `model` field is emptied as it is provided as a path parameter.
    let model_id = mem::take(&mut request.model.model_id);

    let uri =
        format!("{api_url}/v1beta/models/{model_id}:streamGenerateContent?alt=sse&key={api_key}",);

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
                                    "Error parsing JSON: {error:?}\n{line:?}"
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
            "error during streamGenerateContent, status code: {:?}, body: {}",
            response.status(),
            text
        ))
    }
}

pub async fn count_tokens(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: CountTokensRequest,
) -> Result<CountTokensResponse> {
    validate_generate_content_request(&request.generate_content_request)?;

    let uri = format!(
        "{api_url}/v1beta/models/{model_id}:countTokens?key={api_key}",
        model_id = &request.generate_content_request.model.model_id,
    );

    let request = serde_json::to_string(&request)?;
    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(&uri)
        .header("Content-Type", "application/json");
    let http_request = request_builder.body(AsyncBody::from(request))?;

    let mut response = client.send(http_request).await?;
    let mut text = String::new();
    response.body_mut().read_to_string(&mut text).await?;
    anyhow::ensure!(
        response.status().is_success(),
        "error during countTokens, status code: {:?}, body: {}",
        response.status(),
        text
    );
    Ok(serde_json::from_str::<CountTokensResponse>(&text)?)
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
    TextPart(TextPart),
    InlineDataPart(InlineDataPart),
    FunctionCallPart(FunctionCallPart),
    FunctionResponsePart(FunctionResponsePart),
    ThoughtPart(ThoughtPart),
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
pub struct ThoughtPart {
    pub thought: bool,
    pub thought_signature: String,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThinkingConfig {
    pub thinking_budget: u32,
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
#[serde(rename_all = "camelCase")]
pub struct CountTokensRequest {
    pub generate_content_request: GenerateContentRequest,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountTokensResponse {
    pub total_tokens: u64,
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
        serializer.serialize_str(&format!("{MODEL_NAME_PREFIX}{}", &self.model_id))
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

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq, Eq, strum::EnumIter)]
pub enum Model {
    #[serde(
        rename = "gemini-2.5-flash-lite",
        alias = "gemini-2.5-flash-lite-preview-06-17",
        alias = "gemini-2.0-flash-lite-preview"
    )]
    Gemini25FlashLite,
    #[serde(
        rename = "gemini-2.5-flash",
        alias = "gemini-2.0-flash-thinking-exp",
        alias = "gemini-2.5-flash-preview-04-17",
        alias = "gemini-2.5-flash-preview-05-20",
        alias = "gemini-2.5-flash-preview-latest",
        alias = "gemini-2.0-flash"
    )]
    #[default]
    Gemini25Flash,
    #[serde(
        rename = "gemini-2.5-pro",
        alias = "gemini-2.0-pro-exp",
        alias = "gemini-2.5-pro-preview-latest",
        alias = "gemini-2.5-pro-exp-03-25",
        alias = "gemini-2.5-pro-preview-03-25",
        alias = "gemini-2.5-pro-preview-05-06",
        alias = "gemini-2.5-pro-preview-06-05"
    )]
    Gemini25Pro,
    #[serde(rename = "gemini-3-pro-preview")]
    Gemini3Pro,
    #[serde(rename = "gemini-3-flash-preview")]
    Gemini3Flash,
    #[serde(rename = "custom")]
    Custom {
        name: String,
        /// The name displayed in the UI, such as in the assistant panel model dropdown menu.
        display_name: Option<String>,
        max_tokens: u64,
        #[serde(default)]
        mode: GoogleModelMode,
    },
}

impl Model {
    pub fn default_fast() -> Self {
        Self::Gemini25FlashLite
    }

    pub fn id(&self) -> &str {
        match self {
            Self::Gemini25FlashLite => "gemini-2.5-flash-lite",
            Self::Gemini25Flash => "gemini-2.5-flash",
            Self::Gemini25Pro => "gemini-2.5-pro",
            Self::Gemini3Pro => "gemini-3-pro-preview",
            Self::Gemini3Flash => "gemini-3-flash-preview",
            Self::Custom { name, .. } => name,
        }
    }
    pub fn request_id(&self) -> &str {
        match self {
            Self::Gemini25FlashLite => "gemini-2.5-flash-lite",
            Self::Gemini25Flash => "gemini-2.5-flash",
            Self::Gemini25Pro => "gemini-2.5-pro",
            Self::Gemini3Pro => "gemini-3-pro-preview",
            Self::Gemini3Flash => "gemini-3-flash-preview",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Gemini25FlashLite => "Gemini 2.5 Flash-Lite",
            Self::Gemini25Flash => "Gemini 2.5 Flash",
            Self::Gemini25Pro => "Gemini 2.5 Pro",
            Self::Gemini3Pro => "Gemini 3 Pro",
            Self::Gemini3Flash => "Gemini 3 Flash",
            Self::Custom {
                name, display_name, ..
            } => display_name.as_ref().unwrap_or(name),
        }
    }

    pub fn max_token_count(&self) -> u64 {
        match self {
            Self::Gemini25FlashLite
            | Self::Gemini25Flash
            | Self::Gemini25Pro
            | Self::Gemini3Pro
            | Self::Gemini3Flash => 1_048_576,
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> Option<u64> {
        match self {
            Model::Gemini25FlashLite
            | Model::Gemini25Flash
            | Model::Gemini25Pro
            | Model::Gemini3Pro
            | Model::Gemini3Flash => Some(65_536),
            Model::Custom { .. } => None,
        }
    }

    pub fn supports_tools(&self) -> bool {
        true
    }

    pub fn supports_images(&self) -> bool {
        true
    }

    pub fn mode(&self) -> GoogleModelMode {
        match self {
            Self::Gemini25FlashLite
            | Self::Gemini25Flash
            | Self::Gemini25Pro
            | Self::Gemini3Pro => {
                GoogleModelMode::Thinking {
                    // By default these models are set to "auto", so we preserve that behavior
                    // but indicate they are capable of thinking mode
                    budget_tokens: None,
                }
            }
            Self::Gemini3Flash => GoogleModelMode::Default,
            Self::Custom { mode, .. } => *mode,
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
    use serde_json::json;

    #[test]
    fn test_function_call_part_with_signature_serializes_correctly() {
        let part = FunctionCallPart {
            function_call: FunctionCall {
                name: "test_function".to_string(),
                args: json!({"arg": "value"}),
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
            },
            thought_signature: Some("".to_string()),
        };

        let serialized = serde_json::to_value(&part).unwrap();

        // Empty string should still be serialized (normalization happens at a higher level)
        assert_eq!(serialized["thoughtSignature"], "");
    }
}
