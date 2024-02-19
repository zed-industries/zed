use futures::AsyncReadExt;
use serde::{Deserialize, Serialize};
use util::http::{AsyncBody, Error, HttpClient, Method, Request, Response};

pub async fn make_request<T: HttpClient>(
    client: &T,
    uri: &str,
    method: Method,
    body: AsyncBody,
) -> Result<(), Error> {
    let request = Request::builder().method(method).uri(uri).body(body)?;

    let response = client.send(request).await?;

    // Read chunks of the response body and print them out.
    let mut stream = response.into_body();

    let mut start = 0;
    let mut buffer = Vec::new();
    while let Ok(n) = stream.read(&mut buffer).await {
        if n == 0 {
            break;
        }
        println!("New chunk: {:?}", &buffer[start..start + n]);
        start += n;
    }

    println!("Response body: {:?}", String::from_utf8_lossy(&buffer));

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

#[derive(Debug)]
pub struct GenerateContentRequest {
    pub contents: Vec<Content>,
    pub generation_config: Option<GenerationConfig>,
    pub safety_settings: Option<Vec<SafetySetting>>,
}

#[derive(Debug)]
pub struct GenerateContentResponse {
    pub candidates: Option<Vec<GenerateContentCandidate>>,
    pub prompt_feedback: Option<PromptFeedback>,
}

#[derive(Debug)]
pub struct GenerateContentCandidate {
    pub index: usize,
    pub content: Content,
    pub finish_reason: Option<FinishReason>,
    pub finish_message: Option<String>,
    pub safety_ratings: Option<Vec<SafetyRating>>,
    pub citation_metadata: Option<CitationMetadata>,
}

#[derive(Debug)]
pub struct Content {
    pub parts: Vec<Part>,
}

#[derive(Debug)]
pub enum Part {
    TextPart(TextPart),
    InlineDataPart(InlineDataPart),
}

#[derive(Debug)]
pub struct TextPart {
    pub text: String,
}

#[derive(Debug)]
pub struct InlineDataPart {
    pub inline_data: GenerativeContentBlob,
}

#[derive(Debug)]
pub struct GenerativeContentBlob {
    pub mime_type: String,
    pub data: String,
}

#[derive(Debug)]
pub enum FinishReason {
    Unspecified,
    Stop,
    MaxTokens,
    Safety,
    Recitation,
    Other,
}

#[derive(Debug)]
pub enum BlockReason {
    Unspecified,
    Safety,
    Other,
}

#[derive(Debug)]
pub struct CitationSource {
    pub start_index: Option<usize>,
    pub end_index: Option<usize>,
    pub uri: Option<String>,
    pub license: Option<String>,
}

#[derive(Debug)]
pub struct CitationMetadata {
    pub citation_sources: Vec<CitationSource>,
}

#[derive(Debug)]
pub struct PromptFeedback {
    pub block_reason: BlockReason,
    pub safety_ratings: Vec<SafetyRating>,
    pub block_reason_message: Option<String>,
}

#[derive(Debug)]
pub struct GenerationConfig {
    pub candidate_count: Option<usize>,
    pub stop_sequences: Option<Vec<String>>,
    pub max_output_tokens: Option<usize>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub top_k: Option<usize>,
}

#[derive(Debug)]
pub struct SafetySetting {
    pub category: HarmCategory,
    pub threshold: HarmBlockThreshold,
}

#[derive(Debug)]
pub enum HarmCategory {
    Unspecified,
    HateSpeech,
    SexuallyExplicit,
    Harassment,
    DangerousContent,
}

#[derive(Debug)]
pub enum HarmBlockThreshold {
    Unspecified,
    BlockLowAndAbove,
    BlockMediumAndAbove,
    BlockOnlyHigh,
    BlockNone,
}

#[derive(Debug)]
pub enum HarmProbability {
    Unspecified,
    Negligible,
    Low,
    Medium,
    High,
}

#[derive(Debug)]
pub struct SafetyRating {
    pub category: HarmCategory,
    pub probability: HarmProbability,
}
