use std::sync::Arc;

use anyhow::{anyhow, Result};
use futures::{io::BufReader, stream::BoxStream, AsyncBufReadExt, AsyncReadExt, StreamExt};
use serde::{Deserialize, Serialize};
use util::http::HttpClient;

pub const API_URL: &str = "https://generativelanguage.googleapis.com";

pub async fn stream_generate_content(
    client: Arc<dyn HttpClient>,
    api_url: &str,
    api_key: &str,
    request: GenerateContentRequest,
) -> Result<BoxStream<'static, Result<GenerateContentResponse>>> {
    let uri = format!(
        "{}/v1beta/models/gemini-pro:streamGenerateContent?alt=sse&key={}",
        api_url, api_key
    );

    let request = serde_json::to_string(&request)?;
    let mut response = client.post_json(&uri, request.into()).await?;
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
                                Err(error) => Some(Err(anyhow!(error))),
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

pub async fn count_tokens<T: HttpClient>(
    client: &T,
    api_url: &str,
    api_key: &str,
    request: CountTokensRequest,
) -> Result<CountTokensResponse> {
    let uri = format!(
        "{}/v1beta/models/gemini-pro:countTokens?key={}",
        api_url, api_key
    );
    let request = serde_json::to_string(&request)?;
    let mut response = client.post_json(&uri, request.into()).await?;
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentRequest {
    pub contents: Vec<Content>,
    pub generation_config: Option<GenerationConfig>,
    pub safety_settings: Option<Vec<SafetySetting>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentResponse {
    pub candidates: Option<Vec<GenerateContentCandidate>>,
    pub prompt_feedback: Option<PromptFeedback>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentCandidate {
    pub index: usize,
    pub content: Content,
    pub finish_reason: Option<String>,
    pub finish_message: Option<String>,
    pub safety_ratings: Option<Vec<SafetyRating>>,
    pub citation_metadata: Option<CitationMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub parts: Vec<Part>,
    pub role: Role,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationSource {
    pub start_index: Option<usize>,
    pub end_index: Option<usize>,
    pub uri: Option<String>,
    pub license: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationMetadata {
    pub citation_sources: Vec<CitationSource>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptFeedback {
    pub block_reason: Option<String>,
    pub safety_ratings: Vec<SafetyRating>,
    pub block_reason_message: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationConfig {
    pub candidate_count: Option<usize>,
    pub stop_sequences: Option<Vec<String>>,
    pub max_output_tokens: Option<usize>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub top_k: Option<usize>,
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmProbability {
    #[serde(rename = "HARM_PROBABILITY_UNSPECIFIED")]
    Unspecified,
    Negligible,
    Low,
    Medium,
    High,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SafetyRating {
    pub category: HarmCategory,
    pub probability: HarmProbability,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CountTokensRequest {
    pub contents: Vec<Content>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountTokensResponse {
    pub total_tokens: usize,
}
