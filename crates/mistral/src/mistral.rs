use anyhow::{Context as _, Result, anyhow};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::{
    AsyncBody, CustomHeaders, HttpClient, HttpRequestExt, Method, Request as HttpRequest,
    RequestBuilderExt,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap, hash_map};
use std::convert::TryFrom;
use strum::EnumIter;

pub const MISTRAL_API_URL: &str = "https://api.mistral.ai/v1";

pub const DEFAULT_MAX_TOKEN_COUNT: u64 = 128_000;

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
    Tool,
}

impl TryFrom<String> for Role {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        match value.as_str() {
            "user" => Ok(Self::User),
            "assistant" => Ok(Self::Assistant),
            "system" => Ok(Self::System),
            "tool" => Ok(Self::Tool),
            _ => anyhow::bail!("invalid role '{value}'"),
        }
    }
}

impl From<Role> for String {
    fn from(val: Role) -> Self {
        match val {
            Role::User => "user".to_owned(),
            Role::Assistant => "assistant".to_owned(),
            Role::System => "system".to_owned(),
            Role::Tool => "tool".to_owned(),
        }
    }
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Model {
    #[serde(rename = "codestral-latest", alias = "codestral-latest")]
    #[default]
    CodestralLatest,

    #[serde(rename = "mistral-large-latest", alias = "mistral-large-latest")]
    MistralLargeLatest,
    #[serde(rename = "mistral-medium-latest", alias = "mistral-medium-latest")]
    MistralMediumLatest,
    #[serde(rename = "mistral-small-latest", alias = "mistral-small-latest")]
    MistralSmallLatest,

    #[serde(rename = "ministral-3b-latest", alias = "ministral-3b-latest")]
    Ministral3bLatest,
    #[serde(rename = "ministral-8b-latest", alias = "ministral-8b-latest")]
    Ministral8bLatest,
    #[serde(rename = "ministral-14b-latest", alias = "ministral-14b-latest")]
    Ministral14bLatest,

    #[serde(rename = "custom")]
    Custom {
        name: String,
        /// The name displayed in the UI, such as in the agent panel model dropdown menu.
        display_name: Option<String>,
        max_tokens: u64,
        max_output_tokens: Option<u64>,
        max_completion_tokens: Option<u64>,
        supports_tools: Option<bool>,
        supports_images: Option<bool>,
        supports_thinking: Option<bool>,
    },
}

impl Model {
    pub fn default_fast() -> Self {
        Model::MistralSmallLatest
    }

    pub fn from_listed(entry: ListModelEntry) -> Self {
        Model::Custom {
            name: entry.id,
            // The listing's `name` field duplicates `id`, so it adds nothing.
            display_name: None,
            max_tokens: entry.max_context_length.unwrap_or(DEFAULT_MAX_TOKEN_COUNT),
            max_output_tokens: None,
            max_completion_tokens: None,
            supports_tools: Some(entry.capabilities.function_calling),
            supports_images: Some(entry.capabilities.vision),
            supports_thinking: Some(entry.capabilities.reasoning),
        }
    }

    pub fn from_id(id: &str) -> Result<Self> {
        match id {
            "codestral-latest" => Ok(Self::CodestralLatest),
            "mistral-large-latest" => Ok(Self::MistralLargeLatest),
            "mistral-medium-latest" => Ok(Self::MistralMediumLatest),
            "mistral-small-latest" => Ok(Self::MistralSmallLatest),
            "ministral-3b-latest" => Ok(Self::Ministral3bLatest),
            "ministral-8b-latest" => Ok(Self::Ministral8bLatest),
            "ministral-14b-latest" => Ok(Self::Ministral14bLatest),
            invalid_id => anyhow::bail!("invalid model id '{invalid_id}'"),
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Self::CodestralLatest => "codestral-latest",
            Self::MistralLargeLatest => "mistral-large-latest",
            Self::MistralMediumLatest => "mistral-medium-latest",
            Self::MistralSmallLatest => "mistral-small-latest",
            Self::Ministral3bLatest => "ministral-3b-latest",
            Self::Ministral8bLatest => "ministral-8b-latest",
            Self::Ministral14bLatest => "ministral-14b-latest",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::CodestralLatest => "codestral-latest",
            Self::MistralLargeLatest => "mistral-large-latest",
            Self::MistralMediumLatest => "mistral-medium-latest",
            Self::MistralSmallLatest => "mistral-small-latest",
            Self::Ministral3bLatest => "ministral-3b-latest",
            Self::Ministral8bLatest => "ministral-8b-latest",
            Self::Ministral14bLatest => "ministral-14b-latest",
            Self::Custom {
                name, display_name, ..
            } => display_name.as_ref().unwrap_or(name),
        }
    }

    pub fn max_token_count(&self) -> u64 {
        match self {
            Self::CodestralLatest => 128000,
            Self::MistralLargeLatest => 256000,
            Self::MistralMediumLatest => 256000,
            Self::MistralSmallLatest => 256000,
            Self::Ministral3bLatest => 256000,
            Self::Ministral8bLatest => 256000,
            Self::Ministral14bLatest => 256000,
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> Option<u64> {
        match self {
            Self::Custom {
                max_output_tokens, ..
            } => *max_output_tokens,
            _ => None,
        }
    }

    pub fn supports_tools(&self) -> bool {
        match self {
            Self::CodestralLatest
            | Self::MistralLargeLatest
            | Self::MistralMediumLatest
            | Self::MistralSmallLatest
            | Self::Ministral3bLatest
            | Self::Ministral8bLatest
            | Self::Ministral14bLatest => true,
            Self::Custom { supports_tools, .. } => supports_tools.unwrap_or(false),
        }
    }

    pub fn supports_images(&self) -> bool {
        match self {
            Self::MistralLargeLatest
            | Self::MistralMediumLatest
            | Self::MistralSmallLatest
            | Self::Ministral3bLatest
            | Self::Ministral8bLatest
            | Self::Ministral14bLatest => true,
            Self::CodestralLatest => false,
            Self::Custom {
                supports_images, ..
            } => supports_images.unwrap_or(false),
        }
    }

    pub fn supports_thinking(&self) -> bool {
        match self {
            Self::MistralMediumLatest | Self::MistralSmallLatest => true,
            Self::Custom {
                supports_thinking, ..
            } => supports_thinking.unwrap_or(false),
            _ => false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub model: String,
    pub messages: Vec<RequestMessage>,
    pub stream: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<ToolDefinition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<ReasoningEffort>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningEffort {
    None,
    High,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream_tool_calls: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseFormat {
    Text,
    #[serde(rename = "json_object")]
    JsonObject,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolDefinition {
    Function { function: FunctionDefinition },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoice {
    Auto,
    Required,
    None,
    Any,
    #[serde(untagged)]
    Function(ToolDefinition),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum RequestMessage {
    Assistant {
        #[serde(flatten)]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        content: Option<MessageContent>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        tool_calls: Vec<ToolCall>,
    },
    User {
        #[serde(flatten)]
        content: MessageContent,
    },
    System {
        #[serde(flatten)]
        content: MessageContent,
    },
    Tool {
        content: String,
        tool_call_id: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(untagged)]
pub enum MessageContent {
    #[serde(rename = "content")]
    Plain { content: String },
    #[serde(rename = "content")]
    Multipart { content: Vec<MessagePart> },
}

impl MessageContent {
    pub fn empty() -> Self {
        Self::Plain {
            content: String::new(),
        }
    }

    pub fn push_part(&mut self, part: MessagePart) {
        match self {
            Self::Plain { content } => match part {
                MessagePart::Text { text } => {
                    content.push_str(&text);
                }
                part => {
                    let mut parts = if content.is_empty() {
                        Vec::new()
                    } else {
                        vec![MessagePart::Text {
                            text: content.clone(),
                        }]
                    };
                    parts.push(part);
                    *self = Self::Multipart { content: parts };
                }
            },
            Self::Multipart { content } => {
                content.push(part);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessagePart {
    Text { text: String },
    ImageUrl { image_url: String },
    Thinking { thinking: Vec<ThinkingPart> },
}

// Backwards-compatibility alias for provider code that refers to ContentPart
pub type ContentPart = MessagePart;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ThinkingPart {
    Text { text: String },
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamResponse {
    pub id: String,
    #[serde(default)]
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<StreamChoice>,
    pub usage: Option<Usage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamChoice {
    pub index: u32,
    pub delta: StreamDelta,
    pub finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamDelta {
    pub role: Option<Role>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<MessageContentDelta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCallChunk>>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(untagged)]
pub enum MessageContentDelta {
    Text(String),
    Parts(Vec<MessagePart>),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ToolCallChunk {
    pub index: usize,
    pub id: Option<String>,
    pub function: Option<FunctionChunk>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct FunctionChunk {
    pub name: Option<String>,
    pub arguments: Option<String>,
}

pub async fn stream_completion(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: Request,
    affinity: Option<String>,
    extra_headers: &CustomHeaders,
) -> Result<BoxStream<'static, Result<StreamResponse>>> {
    let uri = format!("{api_url}/chat/completions");
    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key.trim()))
        .when_some(affinity, |this, affinity| {
            this.header("x-affinity", affinity)
        })
        .extra_headers(extra_headers);

    let request = request_builder.body(AsyncBody::from(serde_json::to_string(&request)?))?;
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
                                Ok(response) => Some(Ok(response)),
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
            "Failed to connect to Mistral API: {} {}",
            response.status(),
            body,
        );
    }
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct ModelCapabilities {
    #[serde(default)]
    pub completion_chat: bool,
    #[serde(default)]
    pub function_calling: bool,
    #[serde(default)]
    pub vision: bool,
    #[serde(default)]
    pub reasoning: bool,
}

/// A raw model entry returned by `GET {api_url}/models`.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ListModelEntry {
    pub id: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub capabilities: ModelCapabilities,
    #[serde(default)]
    pub max_context_length: Option<u64>,
    #[serde(default)]
    pub deprecation: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ListModelsResponse {
    data: Vec<ListModelEntry>,
}

/// Fetch the models available to the given API key, filtered and deduplicated
/// via [`chat_models_from_listing`].
///
/// See https://docs.mistral.ai/api/#tag/models
pub async fn list_models(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    extra_headers: &CustomHeaders,
) -> Result<Vec<Model>> {
    let uri = format!("{api_url}/models");

    let request = HttpRequest::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", api_key.trim()))
        .extra_headers(extra_headers)
        .body(AsyncBody::default())
        .context("failed to build Mistral models list request")?;

    let mut response = client
        .send(request)
        .await
        .context("failed to send Mistral models list request")?;

    let mut body = String::new();
    response
        .body_mut()
        .read_to_string(&mut body)
        .await
        .context("failed to read Mistral models list response")?;

    anyhow::ensure!(
        response.status().is_success(),
        "failed to list Mistral models: {} {}",
        response.status(),
        body,
    );

    let parsed: ListModelsResponse =
        serde_json::from_str(&body).context("failed to parse Mistral models list response")?;

    Ok(chat_models_from_listing(parsed.data))
}

/// Convert the raw `/models` listing into the chat models to offer.
///
/// The listing includes non-chat models (embeddings, OCR, transcription) and
/// repeats each model once per alias, cross-referencing the other names in
/// `aliases`. Entries are filtered to chat-capable, non-deprecated models and
/// collapsed to one per alias group, preferring ids ending in `-latest`, then
/// the shortest id, then the lexicographically smallest.
pub fn chat_models_from_listing(entries: Vec<ListModelEntry>) -> Vec<Model> {
    let entries: Vec<ListModelEntry> = entries
        .into_iter()
        .filter(|entry| entry.capabilities.completion_chat && entry.deprecation.is_none())
        .collect();

    fn find(parent: &mut [usize], mut index: usize) -> usize {
        while parent[index] != index {
            parent[index] = parent[parent[index]];
            index = parent[index];
        }
        index
    }

    let mut parent: Vec<usize> = (0..entries.len()).collect();
    let mut index_by_name: HashMap<&str, usize> = HashMap::new();
    for (index, entry) in entries.iter().enumerate() {
        for name in
            std::iter::once(entry.id.as_str()).chain(entry.aliases.iter().map(String::as_str))
        {
            match index_by_name.entry(name) {
                hash_map::Entry::Occupied(occupied) => {
                    let root_a = find(&mut parent, index);
                    let root_b = find(&mut parent, *occupied.get());
                    parent[root_a] = root_b;
                }
                hash_map::Entry::Vacant(vacant) => {
                    vacant.insert(index);
                }
            }
        }
    }

    let mut groups: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for index in 0..entries.len() {
        let root = find(&mut parent, index);
        groups.entry(root).or_default().push(index);
    }

    let mut models: Vec<Model> = groups
        .into_values()
        .filter_map(|members| {
            members.into_iter().min_by(|&a, &b| {
                let a = &entries[a].id;
                let b = &entries[b].id;
                (!a.ends_with("-latest"), a.len(), a).cmp(&(!b.ends_with("-latest"), b.len(), b))
            })
        })
        .map(|index| Model::from_listed(entries[index].clone()))
        .collect();
    models.sort_by(|a, b| a.id().cmp(b.id()));
    models
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chat_entry(id: &str, aliases: &[&str]) -> ListModelEntry {
        ListModelEntry {
            id: id.to_string(),
            aliases: aliases.iter().map(|alias| alias.to_string()).collect(),
            capabilities: ModelCapabilities {
                completion_chat: true,
                function_calling: true,
                vision: false,
                reasoning: false,
            },
            max_context_length: Some(262144),
            deprecation: None,
        }
    }

    fn model_ids(models: &[Model]) -> Vec<&str> {
        models.iter().map(|model| model.id()).collect()
    }

    #[test]
    fn deserializes_models_list_response() {
        let json = r#"{
            "object": "list",
            "data": [
                {
                    "id": "mistral-large-latest",
                    "object": "model",
                    "created": 1786516708,
                    "owned_by": "mistralai",
                    "capabilities": {
                        "completion_chat": true,
                        "function_calling": true,
                        "reasoning": false,
                        "completion_fim": false,
                        "fine_tuning": true,
                        "vision": true,
                        "ocr": false
                    },
                    "name": "mistral-large-latest",
                    "description": "Frontier-class model.",
                    "max_context_length": 262144,
                    "aliases": ["mistral-large-2512"],
                    "deprecation": null,
                    "default_model_temperature": 0.3,
                    "type": "base"
                },
                {
                    "id": "mistral-embed",
                    "capabilities": {"completion_chat": false},
                    "max_context_length": 8192,
                    "aliases": []
                },
                {
                    "id": "mistral-medium-2505",
                    "capabilities": {"completion_chat": true, "function_calling": true},
                    "max_context_length": 131072,
                    "aliases": [],
                    "deprecation": "2026-08-31T12:00:00Z"
                },
                {
                    "id": "bare-entry"
                }
            ]
        }"#;

        let parsed: ListModelsResponse = serde_json::from_str(json).expect("valid response JSON");
        assert_eq!(parsed.data.len(), 4);

        let large = &parsed.data[0];
        assert_eq!(large.id, "mistral-large-latest");
        assert_eq!(large.aliases, vec!["mistral-large-2512".to_string()]);
        assert!(large.capabilities.completion_chat);
        assert!(large.capabilities.function_calling);
        assert!(large.capabilities.vision);
        assert!(!large.capabilities.reasoning);
        assert_eq!(large.max_context_length, Some(262144));
        assert_eq!(large.deprecation, None);

        let embed = &parsed.data[1];
        assert!(!embed.capabilities.completion_chat);

        let deprecated = &parsed.data[2];
        assert_eq!(
            deprecated.deprecation,
            Some("2026-08-31T12:00:00Z".to_string())
        );

        let bare = &parsed.data[3];
        assert_eq!(bare.capabilities, ModelCapabilities::default());
        assert!(bare.aliases.is_empty());
        assert_eq!(bare.max_context_length, None);
        assert_eq!(bare.deprecation, None);
    }

    #[test]
    fn filters_non_chat_and_deprecated_models() {
        let mut embed = chat_entry("mistral-embed", &[]);
        embed.capabilities.completion_chat = false;
        let mut deprecated = chat_entry("mistral-medium-2505", &[]);
        deprecated.deprecation = Some("2026-08-31T12:00:00Z".to_string());
        let chat = chat_entry("mistral-large-latest", &[]);

        let models = chat_models_from_listing(vec![embed, deprecated, chat]);
        assert_eq!(model_ids(&models), vec!["mistral-large-latest"]);
    }

    #[test]
    fn dedups_alias_closures_preferring_latest() {
        let models = chat_models_from_listing(vec![
            chat_entry(
                "mistral-medium",
                &["mistral-medium-latest", "mistral-medium-2604"],
            ),
            chat_entry(
                "mistral-medium-latest",
                &["mistral-medium", "mistral-medium-2604"],
            ),
            chat_entry(
                "mistral-medium-2604",
                &["mistral-medium", "mistral-medium-latest"],
            ),
            chat_entry("codestral-latest", &["codestral-2508"]),
            chat_entry("codestral-2508", &["codestral-latest"]),
        ]);
        assert_eq!(
            model_ids(&models),
            vec!["codestral-latest", "mistral-medium-latest"]
        );
    }

    #[test]
    fn dedups_transitive_alias_closures() {
        // `a` and `c` never mention each other, but both alias `shared`, so
        // all three collapse into one group.
        let models = chat_models_from_listing(vec![
            chat_entry("model-a", &["shared"]),
            chat_entry("model-c-latest", &["shared"]),
        ]);
        assert_eq!(model_ids(&models), vec!["model-c-latest"]);
    }

    #[test]
    fn dedup_tie_breaks_shortest_then_lexicographic() {
        let models = chat_models_from_listing(vec![
            chat_entry("magistral-small-latest", &["mistral-small-latest"]),
            chat_entry("mistral-small-latest", &["magistral-small-latest"]),
            chat_entry("zai-glm-5-2", &["glm-5-2"]),
            chat_entry("glm-5-2", &["zai-glm-5-2"]),
            chat_entry("bb-latest", &["aa-latest"]),
            chat_entry("aa-latest", &["bb-latest"]),
        ]);
        assert_eq!(
            model_ids(&models),
            vec!["aa-latest", "glm-5-2", "mistral-small-latest"]
        );
    }

    #[test]
    fn from_listed_maps_capabilities() {
        let entry = ListModelEntry {
            id: "magistral-small-latest".to_string(),
            aliases: Vec::new(),
            capabilities: ModelCapabilities {
                completion_chat: true,
                function_calling: true,
                vision: false,
                reasoning: true,
            },
            max_context_length: Some(262144),
            deprecation: None,
        };

        let model = Model::from_listed(entry);
        assert_eq!(model.id(), "magistral-small-latest");
        assert_eq!(model.display_name(), "magistral-small-latest");
        assert_eq!(model.max_token_count(), 262144);
        assert_eq!(model.max_output_tokens(), None);
        assert!(model.supports_tools());
        assert!(!model.supports_images());
        assert!(model.supports_thinking());
    }

    #[test]
    fn from_listed_defaults_max_tokens_when_missing() {
        let mut entry = chat_entry("some-model", &[]);
        entry.max_context_length = None;
        let model = Model::from_listed(entry);
        assert_eq!(model.max_token_count(), DEFAULT_MAX_TOKEN_COUNT);
    }

    #[test]
    fn empty_listing_produces_no_models() {
        assert!(chat_models_from_listing(Vec::new()).is_empty());
    }
}
