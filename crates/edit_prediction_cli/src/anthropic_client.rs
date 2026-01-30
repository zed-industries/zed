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
use std::sync::{Arc, Mutex};

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
    connection: Mutex<sqlez::connection::Connection>,
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
            connection: Mutex::new(connection),
            http_client,
            api_key,
        })
    }

    pub fn lookup(
        &self,
        model: &str,
        max_tokens: u64,
        messages: &[Message],
        seed: Option<usize>,
    ) -> Result<Option<AnthropicResponse>> {
        let request_hash_str = Self::request_hash(model, max_tokens, messages, seed);
        let connection = self.connection.lock().unwrap();
        let response: Vec<String> = connection.select_bound(
            &sql!(SELECT response FROM cache WHERE request_hash = ?1 AND response IS NOT NULL;),
        )?(request_hash_str.as_str())?;
        Ok(response
            .into_iter()
            .next()
            .and_then(|text| serde_json::from_str(&text).ok()))
    }

    pub fn mark_for_batch(
        &self,
        model: &str,
        max_tokens: u64,
        messages: &[Message],
        seed: Option<usize>,
    ) -> Result<()> {
        let request_hash = Self::request_hash(model, max_tokens, messages, seed);

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
        let connection = self.connection.lock().unwrap();
        connection.exec_bound::<CacheRow>(sql!(
            INSERT OR IGNORE INTO cache(request_hash, request, response, batch_id) VALUES (?, ?, ?, ?)))?(
            cache_row,
        )
    }

    async fn generate(
        &self,
        model: &str,
        max_tokens: u64,
        messages: Vec<Message>,
        seed: Option<usize>,
    ) -> Result<Option<AnthropicResponse>> {
        let response = self.lookup(model, max_tokens, &messages, seed)?;
        if let Some(response) = response {
            return Ok(Some(response));
        }

        self.mark_for_batch(model, max_tokens, &messages, seed)?;

        Ok(None)
    }

    /// Uploads pending requests as batches (chunked to 16k each); downloads finished batches if any.
    async fn sync_batches(&self) -> Result<()> {
        let _batch_ids = self.upload_pending_requests().await?;
        self.download_finished_batches().await
    }

    /// Import batch results from external batch IDs (useful for recovering after database loss)
    pub async fn import_batches(&self, batch_ids: &[String]) -> Result<()> {
        for batch_id in batch_ids {
            log::info!("Importing batch {}", batch_id);

            let batch_status = anthropic::batches::retrieve_batch(
                self.http_client.as_ref(),
                ANTHROPIC_API_URL,
                &self.api_key,
                batch_id,
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to retrieve batch {}: {:?}", batch_id, e))?;

            log::info!(
                "Batch {} status: {}",
                batch_id,
                batch_status.processing_status
            );

            if batch_status.processing_status != "ended" {
                log::warn!(
                    "Batch {} is not finished (status: {}), skipping",
                    batch_id,
                    batch_status.processing_status
                );
                continue;
            }

            let results = anthropic::batches::retrieve_batch_results(
                self.http_client.as_ref(),
                ANTHROPIC_API_URL,
                &self.api_key,
                batch_id,
            )
            .await
            .map_err(|e| {
                anyhow::anyhow!("Failed to retrieve batch results for {}: {:?}", batch_id, e)
            })?;

            let mut updates: Vec<(String, String, String)> = Vec::new();
            let mut success_count = 0;
            let mut error_count = 0;

            for result in results {
                let request_hash = result
                    .custom_id
                    .strip_prefix("req_hash_")
                    .unwrap_or(&result.custom_id)
                    .to_string();

                match result.result {
                    anthropic::batches::BatchResult::Succeeded { message } => {
                        let response_json = serde_json::to_string(&message)?;
                        updates.push((request_hash, response_json, batch_id.clone()));
                        success_count += 1;
                    }
                    anthropic::batches::BatchResult::Errored { error } => {
                        log::error!(
                            "Batch request {} failed: {}: {}",
                            request_hash,
                            error.error.error_type,
                            error.error.message
                        );
                        let error_json = serde_json::json!({
                            "error": {
                                "type": error.error.error_type,
                                "message": error.error.message
                            }
                        })
                        .to_string();
                        updates.push((request_hash, error_json, batch_id.clone()));
                        error_count += 1;
                    }
                    anthropic::batches::BatchResult::Canceled => {
                        log::warn!("Batch request {} was canceled", request_hash);
                        error_count += 1;
                    }
                    anthropic::batches::BatchResult::Expired => {
                        log::warn!("Batch request {} expired", request_hash);
                        error_count += 1;
                    }
                }
            }

            let connection = self.connection.lock().unwrap();
            connection.with_savepoint("batch_import", || {
                // Use INSERT OR REPLACE to handle both new entries and updating existing ones
                let q = sql!(
                    INSERT OR REPLACE INTO cache(request_hash, request, response, batch_id)
                    VALUES (?, (SELECT request FROM cache WHERE request_hash = ?), ?, ?)
                );
                let mut exec = connection.exec_bound::<(&str, &str, &str, &str)>(q)?;
                for (request_hash, response_json, batch_id) in &updates {
                    exec((
                        request_hash.as_str(),
                        request_hash.as_str(),
                        response_json.as_str(),
                        batch_id.as_str(),
                    ))?;
                }
                Ok(())
            })?;

            log::info!(
                "Imported batch {}: {} successful, {} errors",
                batch_id,
                success_count,
                error_count
            );
        }

        Ok(())
    }

    async fn download_finished_batches(&self) -> Result<()> {
        let batch_ids: Vec<String> = {
            let connection = self.connection.lock().unwrap();
            let q = sql!(SELECT DISTINCT batch_id FROM cache WHERE batch_id IS NOT NULL AND response IS NULL);
            connection.select(q)?()?
        };

        for batch_id in &batch_ids {
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

                let mut updates: Vec<(String, String)> = Vec::new();
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
                            updates.push((response_json, request_hash));
                            success_count += 1;
                        }
                        anthropic::batches::BatchResult::Errored { error } => {
                            log::error!(
                                "Batch request {} failed: {}: {}",
                                request_hash,
                                error.error.error_type,
                                error.error.message
                            );
                            let error_json = serde_json::json!({
                                "error": {
                                    "type": error.error.error_type,
                                    "message": error.error.message
                                }
                            })
                            .to_string();
                            updates.push((error_json, request_hash));
                        }
                        anthropic::batches::BatchResult::Canceled => {
                            log::warn!("Batch request {} was canceled", request_hash);
                            let error_json = serde_json::json!({
                                "error": {
                                    "type": "canceled",
                                    "message": "Batch request was canceled"
                                }
                            })
                            .to_string();
                            updates.push((error_json, request_hash));
                        }
                        anthropic::batches::BatchResult::Expired => {
                            log::warn!("Batch request {} expired", request_hash);
                            let error_json = serde_json::json!({
                                "error": {
                                    "type": "expired",
                                    "message": "Batch request expired"
                                }
                            })
                            .to_string();
                            updates.push((error_json, request_hash));
                        }
                    }
                }

                let connection = self.connection.lock().unwrap();
                connection.with_savepoint("batch_download", || {
                    let q = sql!(UPDATE cache SET response = ? WHERE request_hash = ?);
                    let mut exec = connection.exec_bound::<(&str, &str)>(q)?;
                    for (response_json, request_hash) in &updates {
                        exec((response_json.as_str(), request_hash.as_str()))?;
                    }
                    Ok(())
                })?;
                log::info!("Downloaded {} successful requests", success_count);
            }
        }

        Ok(())
    }

    async fn upload_pending_requests(&self) -> Result<Vec<String>> {
        const BATCH_CHUNK_SIZE: i32 = 16_000;
        let mut all_batch_ids = Vec::new();
        let mut total_uploaded = 0;

        loop {
            let rows: Vec<(String, String)> = {
                let connection = self.connection.lock().unwrap();
                let q = sql!(
                    SELECT request_hash, request FROM cache
                    WHERE batch_id IS NULL AND response IS NULL
                    LIMIT ?
                );
                connection.select_bound(q)?(BATCH_CHUNK_SIZE)?
            };

            if rows.is_empty() {
                break;
            }

            let request_hashes: Vec<String> = rows.iter().map(|(hash, _)| hash.clone()).collect();

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

            {
                let connection = self.connection.lock().unwrap();
                connection.with_savepoint("batch_upload", || {
                    let q = sql!(UPDATE cache SET batch_id = ? WHERE request_hash = ?);
                    let mut exec = connection.exec_bound::<(&str, &str)>(q)?;
                    for hash in &request_hashes {
                        exec((batch.id.as_str(), hash.as_str()))?;
                    }
                    Ok(())
                })?;
            }

            total_uploaded += batch_len;
            log::info!(
                "Uploaded batch {} with {} requests ({} total)",
                batch.id,
                batch_len,
                total_uploaded
            );

            all_batch_ids.push(batch.id);
        }

        if !all_batch_ids.is_empty() {
            log::info!(
                "Finished uploading {} batches with {} total requests",
                all_batch_ids.len(),
                total_uploaded
            );
        }

        Ok(all_batch_ids)
    }

    fn request_hash(
        model: &str,
        max_tokens: u64,
        messages: &[Message],
        seed: Option<usize>,
    ) -> String {
        let mut hasher = std::hash::DefaultHasher::new();
        model.hash(&mut hasher);
        max_tokens.hash(&mut hasher);
        for msg in messages {
            message_content_to_string(&msg.content).hash(&mut hasher);
        }
        if let Some(seed) = seed {
            seed.hash(&mut hasher);
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
        seed: Option<usize>,
    ) -> Result<Option<AnthropicResponse>> {
        match self {
            AnthropicClient::Plain(plain_llm_client) => plain_llm_client
                .generate(model, max_tokens, messages)
                .await
                .map(Some),
            AnthropicClient::Batch(batching_llm_client) => {
                batching_llm_client
                    .generate(model, max_tokens, messages, seed)
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

    pub async fn import_batches(&self, batch_ids: &[String]) -> Result<()> {
        match self {
            AnthropicClient::Plain(_) => {
                anyhow::bail!("Import batches is only supported with batching client")
            }
            AnthropicClient::Batch(batching_llm_client) => {
                batching_llm_client.import_batches(batch_ids).await
            }
            AnthropicClient::Dummy => panic!("Dummy LLM client is not expected to be used"),
        }
    }
}
