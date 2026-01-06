use anthropic::{
    ANTHROPIC_API_URL, Event, Message, Request as AnthropicRequest, RequestContent,
    Response as AnthropicResponse, ResponseContent, Role, non_streaming_completion,
    stream_completion,
};
use anyhow::Result;
use futures::StreamExt as _;
use http_client::HttpClient;
use indoc::indoc;
use reqwest_client::ReqwestClient;
use sqlez::bindable::Bind;
use sqlez::bindable::StaticColumnCount;
use sqlez_macros::sql;
use std::hash::Hash;
use std::hash::Hasher;
use std::path::Path;
use std::sync::Arc;

pub struct PlainLlmClient {
    pub http_client: Arc<dyn HttpClient>,
    pub api_key: String,
}

impl PlainLlmClient {
    pub fn new() -> Result<Self> {
        let http_client: Arc<dyn http_client::HttpClient> = Arc::new(ReqwestClient::new());
        let api_key = std::env::var("ANTHROPIC_API_KEY")
            .map_err(|_| anyhow::anyhow!("ANTHROPIC_API_KEY environment variable not set"))?;
        Ok(Self {
            http_client,
            api_key,
        })
    }

    pub async fn generate(
        &self,
        model: &str,
        max_tokens: u64,
        messages: Vec<Message>,
    ) -> Result<AnthropicResponse> {
        let request = AnthropicRequest {
            model: model.to_string(),
            max_tokens,
            messages,
            tools: Vec::new(),
            thinking: None,
            tool_choice: None,
            system: None,
            metadata: None,
            stop_sequences: Vec::new(),
            temperature: None,
            top_k: None,
            top_p: None,
        };

        let response = non_streaming_completion(
            self.http_client.as_ref(),
            ANTHROPIC_API_URL,
            &self.api_key,
            request,
            None,
        )
        .await
        .map_err(|e| anyhow::anyhow!("{:?}", e))?;

        Ok(response)
    }

    pub async fn generate_streaming<F>(
        &self,
        model: &str,
        max_tokens: u64,
        messages: Vec<Message>,
        mut on_progress: F,
    ) -> Result<AnthropicResponse>
    where
        F: FnMut(usize, &str),
    {
        let request = AnthropicRequest {
            model: model.to_string(),
            max_tokens,
            messages,
            tools: Vec::new(),
            thinking: None,
            tool_choice: None,
            system: None,
            metadata: None,
            stop_sequences: Vec::new(),
            temperature: None,
            top_k: None,
            top_p: None,
        };

        let mut stream = stream_completion(
            self.http_client.as_ref(),
            ANTHROPIC_API_URL,
            &self.api_key,
            request,
            None,
        )
        .await
        .map_err(|e| anyhow::anyhow!("{:?}", e))?;

        let mut response: Option<AnthropicResponse> = None;
        let mut text_content = String::new();

        while let Some(event_result) = stream.next().await {
            let event = event_result.map_err(|e| anyhow::anyhow!("{:?}", e))?;

            match event {
                Event::MessageStart { message } => {
                    response = Some(message);
                }
                Event::ContentBlockDelta { delta, .. } => {
                    if let anthropic::ContentDelta::TextDelta { text } = delta {
                        text_content.push_str(&text);
                        on_progress(text_content.len(), &text_content);
                    }
                }
                _ => {}
            }
        }

        let mut response = response.ok_or_else(|| anyhow::anyhow!("No response received"))?;

        if response.content.is_empty() && !text_content.is_empty() {
            response
                .content
                .push(ResponseContent::Text { text: text_content });
        }

        Ok(response)
    }
}

pub struct BatchingLlmClient {
    connection: sqlez::connection::Connection,
    http_client: Arc<dyn HttpClient>,
    api_key: String,
}

struct CacheRow {
    request_hash: String,
    request: Option<String>,
    response: Option<String>,
    batch_id: Option<String>,
}

impl StaticColumnCount for CacheRow {
    fn column_count() -> usize {
        4
    }
}

impl Bind for CacheRow {
    fn bind(&self, statement: &sqlez::statement::Statement, start_index: i32) -> Result<i32> {
        let next_index = statement.bind(&self.request_hash, start_index)?;
        let next_index = statement.bind(&self.request, next_index)?;
        let next_index = statement.bind(&self.response, next_index)?;
        let next_index = statement.bind(&self.batch_id, next_index)?;
        Ok(next_index)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct SerializableRequest {
    model: String,
    max_tokens: u64,
    messages: Vec<SerializableMessage>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct SerializableMessage {
    role: String,
    content: String,
}

impl BatchingLlmClient {
    fn new(cache_path: &Path) -> Result<Self> {
        let http_client: Arc<dyn http_client::HttpClient> = Arc::new(ReqwestClient::new());
        let api_key = std::env::var("ANTHROPIC_API_KEY")
            .map_err(|_| anyhow::anyhow!("ANTHROPIC_API_KEY environment variable not set"))?;

        let connection = sqlez::connection::Connection::open_file(&cache_path.to_str().unwrap());
        let mut statement = sqlez::statement::Statement::prepare(
            &connection,
            indoc! {"
                CREATE TABLE IF NOT EXISTS cache (
                    request_hash TEXT PRIMARY KEY,
                    request TEXT,
                    response TEXT,
                    batch_id TEXT
                );
                "},
        )?;
        statement.exec()?;
        drop(statement);

        Ok(Self {
            connection,
            http_client,
            api_key,
        })
    }

    pub fn lookup(
        &self,
        model: &str,
        max_tokens: u64,
        messages: &[Message],
    ) -> Result<Option<AnthropicResponse>> {
        let request_hash_str = Self::request_hash(model, max_tokens, messages);
        let response: Vec<String> = self.connection.select_bound(
            &sql!(SELECT response FROM cache WHERE request_hash = ?1 AND response IS NOT NULL;),
        )?(request_hash_str.as_str())?;
        Ok(response
            .into_iter()
            .next()
            .and_then(|text| serde_json::from_str(&text).ok()))
    }

    pub fn mark_for_batch(&self, model: &str, max_tokens: u64, messages: &[Message]) -> Result<()> {
        let request_hash = Self::request_hash(model, max_tokens, messages);

        let serializable_messages: Vec<SerializableMessage> = messages
            .iter()
            .map(|msg| SerializableMessage {
                role: match msg.role {
                    Role::User => "user".to_string(),
                    Role::Assistant => "assistant".to_string(),
                },
                content: message_content_to_string(&msg.content),
            })
            .collect();

        let serializable_request = SerializableRequest {
            model: model.to_string(),
            max_tokens,
            messages: serializable_messages,
        };

        let request = Some(serde_json::to_string(&serializable_request)?);
        let cache_row = CacheRow {
            request_hash,
            request,
            response: None,
            batch_id: None,
        };
        self.connection.exec_bound(sql!(
            INSERT OR IGNORE INTO cache(request_hash, request, response, batch_id) VALUES (?, ?, ?, ?)))?(
            cache_row,
        )
    }

    async fn generate(
        &self,
        model: &str,
        max_tokens: u64,
        messages: Vec<Message>,
    ) -> Result<Option<AnthropicResponse>> {
        let response = self.lookup(model, max_tokens, &messages)?;
        if let Some(response) = response {
            return Ok(Some(response));
        }

        self.mark_for_batch(model, max_tokens, &messages)?;

        Ok(None)
    }

    /// Uploads pending requests as a new batch; downloads finished batches if any.
    async fn sync_batches(&self) -> Result<()> {
        self.upload_pending_requests().await?;
        self.download_finished_batches().await
    }

    async fn download_finished_batches(&self) -> Result<()> {
        let q = sql!(SELECT DISTINCT batch_id FROM cache WHERE batch_id IS NOT NULL AND response IS NULL);
        let batch_ids: Vec<String> = self.connection.select(q)?()?;

        for batch_id in batch_ids {
            let batch_status = anthropic::batches::retrieve_batch(
                self.http_client.as_ref(),
                ANTHROPIC_API_URL,
                &self.api_key,
                &batch_id,
            )
            .await
            .map_err(|e| anyhow::anyhow!("{:?}", e))?;

            log::info!(
                "Batch {} status: {}",
                batch_id,
                batch_status.processing_status
            );

            if batch_status.processing_status == "ended" {
                let results = anthropic::batches::retrieve_batch_results(
                    self.http_client.as_ref(),
                    ANTHROPIC_API_URL,
                    &self.api_key,
                    &batch_id,
                )
                .await
                .map_err(|e| anyhow::anyhow!("{:?}", e))?;

                let mut success_count = 0;
                for result in results {
                    let request_hash = result
                        .custom_id
                        .strip_prefix("req_hash_")
                        .unwrap_or(&result.custom_id)
                        .to_string();

                    match result.result {
                        anthropic::batches::BatchResult::Succeeded { message } => {
                            let response_json = serde_json::to_string(&message)?;
                            let q = sql!(UPDATE cache SET response = ? WHERE request_hash = ?);
                            self.connection.exec_bound(q)?((response_json, request_hash))?;
                            success_count += 1;
                        }
                        anthropic::batches::BatchResult::Errored { error } => {
                            log::error!("Batch request {} failed: {:?}", request_hash, error);
                        }
                        anthropic::batches::BatchResult::Canceled => {
                            log::warn!("Batch request {} was canceled", request_hash);
                        }
                        anthropic::batches::BatchResult::Expired => {
                            log::warn!("Batch request {} expired", request_hash);
                        }
                    }
                }
                log::info!("Downloaded {} successful requests", success_count);
            }
        }

        Ok(())
    }

    async fn upload_pending_requests(&self) -> Result<String> {
        let q = sql!(
        SELECT request_hash, request FROM cache WHERE batch_id IS NULL AND response IS NULL
        );

        let rows: Vec<(String, String)> = self.connection.select(q)?()?;

        if rows.is_empty() {
            return Ok(String::new());
        }

        let batch_requests = rows
            .iter()
            .map(|(hash, request_str)| {
                let serializable_request: SerializableRequest =
                    serde_json::from_str(&request_str).unwrap();

                let messages: Vec<Message> = serializable_request
                    .messages
                    .into_iter()
                    .map(|msg| Message {
                        role: match msg.role.as_str() {
                            "user" => Role::User,
                            "assistant" => Role::Assistant,
                            _ => Role::User,
                        },
                        content: vec![RequestContent::Text {
                            text: msg.content,
                            cache_control: None,
                        }],
                    })
                    .collect();

                let params = AnthropicRequest {
                    model: serializable_request.model,
                    max_tokens: serializable_request.max_tokens,
                    messages,
                    tools: Vec::new(),
                    thinking: None,
                    tool_choice: None,
                    system: None,
                    metadata: None,
                    stop_sequences: Vec::new(),
                    temperature: None,
                    top_k: None,
                    top_p: None,
                };

                let custom_id = format!("req_hash_{}", hash);
                anthropic::batches::BatchRequest { custom_id, params }
            })
            .collect::<Vec<_>>();

        let batch_len = batch_requests.len();
        let batch = anthropic::batches::create_batch(
            self.http_client.as_ref(),
            ANTHROPIC_API_URL,
            &self.api_key,
            anthropic::batches::CreateBatchRequest {
                requests: batch_requests,
            },
        )
        .await
        .map_err(|e| anyhow::anyhow!("{:?}", e))?;

        let q = sql!(
            UPDATE cache SET batch_id = ? WHERE batch_id is NULL
        );
        self.connection.exec_bound(q)?(batch.id.as_str())?;

        log::info!("Uploaded batch with {} requests", batch_len);

        Ok(batch.id)
    }

    fn request_hash(model: &str, max_tokens: u64, messages: &[Message]) -> String {
        let mut hasher = std::hash::DefaultHasher::new();
        model.hash(&mut hasher);
        max_tokens.hash(&mut hasher);
        for msg in messages {
            message_content_to_string(&msg.content).hash(&mut hasher);
        }
        let request_hash = hasher.finish();
        format!("{request_hash:016x}")
    }
}

fn message_content_to_string(content: &[RequestContent]) -> String {
    content
        .iter()
        .filter_map(|c| match c {
            RequestContent::Text { text, .. } => Some(text.clone()),
            _ => None,
        })
        .collect::<Vec<String>>()
        .join("\n")
}

pub enum AnthropicClient {
    // No batching
    Plain(PlainLlmClient),
    Batch(BatchingLlmClient),
    Dummy,
}

impl AnthropicClient {
    pub fn plain() -> Result<Self> {
        Ok(Self::Plain(PlainLlmClient::new()?))
    }

    pub fn batch(cache_path: &Path) -> Result<Self> {
        Ok(Self::Batch(BatchingLlmClient::new(cache_path)?))
    }

    #[allow(dead_code)]
    pub fn dummy() -> Self {
        Self::Dummy
    }

    pub async fn generate(
        &self,
        model: &str,
        max_tokens: u64,
        messages: Vec<Message>,
    ) -> Result<Option<AnthropicResponse>> {
        match self {
            AnthropicClient::Plain(plain_llm_client) => plain_llm_client
                .generate(model, max_tokens, messages)
                .await
                .map(Some),
            AnthropicClient::Batch(batching_llm_client) => {
                batching_llm_client
                    .generate(model, max_tokens, messages)
                    .await
            }
            AnthropicClient::Dummy => panic!("Dummy LLM client is not expected to be used"),
        }
    }

    #[allow(dead_code)]
    pub async fn generate_streaming<F>(
        &self,
        model: &str,
        max_tokens: u64,
        messages: Vec<Message>,
        on_progress: F,
    ) -> Result<Option<AnthropicResponse>>
    where
        F: FnMut(usize, &str),
    {
        match self {
            AnthropicClient::Plain(plain_llm_client) => plain_llm_client
                .generate_streaming(model, max_tokens, messages, on_progress)
                .await
                .map(Some),
            AnthropicClient::Batch(_) => {
                anyhow::bail!("Streaming not supported with batching client")
            }
            AnthropicClient::Dummy => panic!("Dummy LLM client is not expected to be used"),
        }
    }

    pub async fn sync_batches(&self) -> Result<()> {
        match self {
            AnthropicClient::Plain(_) => Ok(()),
            AnthropicClient::Batch(batching_llm_client) => batching_llm_client.sync_batches().await,
            AnthropicClient::Dummy => panic!("Dummy LLM client is not expected to be used"),
        }
    }
}
