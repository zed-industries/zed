mod supported_countries;

use anyhow::{Result, anyhow, bail};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use serde::{Deserialize, Serialize};

pub use supported_countries::*;

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
    let uri = format!(
        "{}/v1beta/models/gemini-pro:countTokens?key={}",
        api_url, api_key
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
    if response.status().is_success() {
        Ok(serde_json::from_str::<CountTokensResponse>(&text)?)
    } else {
        Err(anyhow!(
            "error during countTokens, status code: {:?}, body: {}",
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
    pub system_instruction: Option<SystemInstruction>,
    pub generation_config: Option<GenerationConfig>,
    pub safety_settings: Option<Vec<SafetySetting>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_config: Option<ToolConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentResponse {
    pub candidates: Option<Vec<GenerateContentCandidate>>,
    pub prompt_feedback: Option<PromptFeedback>,
    pub usage_metadata: Option<UsageMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentCandidate {
    pub index: Option<usize>,
    pub content: Content,
    pub finish_reason: Option<String>,
    pub finish_message: Option<String>,
    pub safety_ratings: Option<Vec<SafetyRating>>,
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
    pub start_index: Option<usize>,
    pub end_index: Option<usize>,
    pub uri: Option<String>,
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
    pub block_reason: Option<String>,
    pub safety_ratings: Vec<SafetyRating>,
    pub block_reason_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UsageMetadata {
    pub prompt_token_count: Option<usize>,
    pub cached_content_token_count: Option<usize>,
    pub candidates_token_count: Option<usize>,
    pub tool_use_prompt_token_count: Option<usize>,
    pub thoughts_token_count: Option<usize>,
    pub total_token_count: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationConfig {
    pub candidate_count: Option<usize>,
    pub stop_sequences: Option<Vec<String>>,
    pub max_output_tokens: Option<usize>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
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
pub enum HarmBlockThreshold {
    #[serde(rename = "HARM_BLOCK_THRESHOLD_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "BLOCK_LOW_AND_ABOVE")]
    BlockLowAndAbove,
    #[serde(rename = "BLOCK_MEDIUM_AND_ABOVE")]
    BlockMediumAndAbove,
    #[serde(rename = "BLOCK_ONLY_HIGH")]
    BlockOnlyHigh,
    #[serde(rename = "BLOCK_NONE")]
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

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq, Eq, strum::EnumIter)]
pub enum Model {
    #[serde(rename = "gemini-1.5-pro")]
    Gemini15Pro,
    #[serde(rename = "gemini-1.5-flash")]
    Gemini15Flash,
    #[serde(rename = "gemini-2.0-pro-exp")]
    Gemini20Pro,
    #[serde(rename = "gemini-2.0-flash")]
    #[default]
    Gemini20Flash,
    #[serde(rename = "gemini-2.0-flash-thinking-exp")]
    Gemini20FlashThinking,
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
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id())
    }
}
