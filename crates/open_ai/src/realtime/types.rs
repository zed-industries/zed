use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SessionRequest {
    session: Option<Session>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<SessionAudio>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_modalities: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<RealtimeVoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_audio_transcription: Option<AudioTranscription>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<MaxOutputTokens>,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            r#type: "realtime".to_string(),
            audio: None,
            model: None,
            output_modalities: None,
            instructions: None,
            voice: None,
            input_audio_transcription: None,
            tools: None,
            tool_choice: None,
            temperature: None,
            max_output_tokens: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionAudio {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<SessionAudioInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<SessionAudioOutput>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionAudioInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<AudioFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_detection: Option<TurnDetection>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionAudioOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<AudioFormat>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum RealtimeVoice {
    Alloy,
    Ash,
    Ballad,
    Coral,
    Echo,
    Sage,
    Shimmer,
    Verse,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioFormat {
    pub r#type: String,
    pub rate: u32, //#[serde(rename = "pcm16")]
                   //PCM16,
                   //#[serde(rename = "g711_ulaw")]
                   //G711ULAW,
                   //#[serde(rename = "g711_alaw")]
                   //G711ALAW,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioTranscription {
    pub language: Option<String>,
    pub model: Option<String>,
    pub prompt: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum TurnDetection {
    #[serde(rename = "server_vad")]
    ServerVAD {
        threshold: f32,
        prefix_padding_ms: u32,
        silence_duration_ms: u32,
        create_response: Option<bool>,
        interrupt_response: Option<bool>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ToolDefinition {
    #[serde(rename = "function")]
    Function {
        name: String,
        description: String,
        parameters: serde_json::Value,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoice {
    Auto,
    None,
    Required,
    #[serde(untagged)]
    Function {
        r#type: FunctionType,
        name: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FunctionType {
    Function,
}

#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum MaxOutputTokens {
    Num(u16),
    Inf,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum RawMaxOutputTokens {
    Num(u16),
    Str(String),
}

impl<'de> Deserialize<'de> for MaxOutputTokens {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match RawMaxOutputTokens::deserialize(de)? {
            RawMaxOutputTokens::Num(n) => {
                let n = u16::try_from(n).map_err(serde::de::Error::custom)?;
                Ok(MaxOutputTokens::Num(n))
            }
            RawMaxOutputTokens::Str(s) if s.eq_ignore_ascii_case("inf") => Ok(MaxOutputTokens::Inf),
            RawMaxOutputTokens::Str(s) => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&s),
                &"\"inf\" or a non-negative integer",
            )),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ItemType {
    Message,
    FunctionCall,
    FunctionCallOutput,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ItemStatus {
    Completed,
    InProgress,
    Incomplete,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ItemRole {
    User,
    Assistant,
    System,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ItemContentType {
    InputText,
    OutputAudio,
    InputAudio,
    Text,
    Audio,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemContent {
    pub r#type: ItemContentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Item {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ItemType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<ItemRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<ItemContent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

impl TryFrom<serde_json::Value> for Item {
    type Error = serde_json::Error;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct APIError {
    pub r#type: String,
    pub code: Option<String>,
    pub message: String,
    pub param: Option<String>,
    pub event_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Conversation {
    pub id: String,
    pub object: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateResponse {
    pub instructions: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub status: ResponseStatus,
    pub status_details: Option<ResponseStatusDetail>,
    pub output: Vec<Item>,
    pub usage: Option<Usage>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Usage {
    pub total_tokens: u32,
    pub input_tokens: u32,
    pub output_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ResponseStatus {
    InProgress,
    Completed,
    Cancelled,
    Failed,
    Incomplete,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ResponseStatusDetail {
    #[serde(rename = "cancelled")]
    Cancelled { reason: CancelledReason },
    #[serde(rename = "incomplete")]
    Incomplete { reason: IncompleteReason },
    #[serde(rename = "failed")]
    Failed { error: Option<FailedError> },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FailedError {
    pub code: Option<String>,
    pub message: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CancelledReason {
    TurnDetected,
    ClientCancelled,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum IncompleteReason {
    Interruption,
    MaxOutputTokens,
    ContentFilter,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "audio")]
    Audio {
        audio: Option<String>,
        transcript: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RateLimit {
    pub name: String,
    pub limit: u32,
    pub remaining: u32,
    pub reset_seconds: f32,
}
