use anyhow::Result;
use http_client::HttpClient;
use indoc::indoc;
use open_ai::{
    MessageContent, OPEN_AI_API_URL, Request as OpenAiRequest, RequestMessage,
    Response as OpenAiResponse, batches, non_streaming_completion,
};
use reqwest_client::ReqwestClient;
use sqlez::bindable::Bind;
use sqlez::bindable::StaticColumnCount;
use sqlez_macros::sql;
use std::hash::Hash;
use std::hash::Hasher;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub struct PlainOpenAiClient {
    pub http_client: Arc<dyn HttpClient>,
    pub api_key: String,
}

impl PlainOpenAiClient {
    pub fn new() -> Result<Self> {
        let http_client: Arc<dyn http_client::HttpClient> = Arc::new(ReqwestClient::new());
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| anyhow::anyhow!("OPENAI_API_KEY environment variable not set"))?;
        Ok(Self {
            http_client,
            api_key,
        })
    }

    pub async fn generate(
        &self,
        model: &str,
        max_tokens: u64,
        messages: Vec<RequestMessage>,
    ) -> Result<OpenAiResponse> {
        let request = OpenAiRequest {
            model: model.to_string(),
            messages,
            stream: false,
            max_completion_tokens: Some(max_tokens),
            stop: Vec::new(),
            temperature: None,
            tool_choice: None,
            parallel_tool_calls: None,
            tools: Vec::new(),
            prompt_cache_key: None,
            reasoning_effort: None,
        };

        let response = non_streaming_completion(
            self.http_client.as_ref(),
            OPEN_AI_API_URL,
            &self.api_key,
            request,
        )
        .await
        .map_err(|e| anyhow::anyhow!("{:?}", e))?;

        Ok(response)
    }
}

pub struct BatchingOpenAiClient {
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

impl BatchingOpenAiClient {
    fn new(cache_path: &Path) -> Result<Self> {
        let http_client: Arc<dyn http_client::HttpClient> = Arc::new(ReqwestClient::new());
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| anyhow::anyhow!("OPENAI_API_KEY environment variable not set"))?;

        let connection = sqlez::connection::Connection::open_file(cache_path.to_str().unwrap());
        let mut statement = sqlez::statement::Statement::prepare(
            &connection,
            indoc! {"
                CREATE TABLE IF NOT EXISTS openai_cache (
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
        messages: &[RequestMessage],
        seed: Option<usize>,
    ) -> Result<Option<OpenAiResponse>> {
        let request_hash_str = Self::request_hash(model, max_tokens, messages, seed);
        let connection = self.connection.lock().unwrap();
        let response: Vec<String> = connection.select_bound(
            &sql!(SELECT response FROM openai_cache WHERE request_hash = ?1 AND response IS NOT NULL;),
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
        messages: &[RequestMessage],
        seed: Option<usize>,
    ) -> Result<()> {
        let request_hash = Self::request_hash(model, max_tokens, messages, seed);

        let serializable_messages: Vec<SerializableMessage> = messages
            .iter()
            .map(|msg| SerializableMessage {
                role: message_role_to_string(msg),
                content: message_content_to_string(msg),
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
            INSERT OR IGNORE INTO openai_cache(request_hash, request, response, batch_id) VALUES (?, ?, ?, ?)))?(
            cache_row,
        )
    }

    async fn generate(
        &self,
        model: &str,
        max_tokens: u64,
        messages: Vec<RequestMessage>,
        seed: Option<usize>,
        cache_only: bool,
    ) -> Result<Option<OpenAiResponse>> {
        let response = self.lookup(model, max_tokens, &messages, seed)?;
        if let Some(response) = response {
            return Ok(Some(response));
        }

        if !cache_only {
            self.mark_for_batch(model, max_tokens, &messages, seed)?;
        }

        Ok(None)
    }

    async fn sync_batches(&self) -> Result<()> {
        let _batch_ids = self.upload_pending_requests().await?;
        self.download_finished_batches().await
    }

    pub async fn import_batches(&self, batch_ids: &[String]) -> Result<()> {
        for batch_id in batch_ids {
            log::info!("Importing OpenAI batch {}", batch_id);

            let batch_status = batches::retrieve_batch(
                self.http_client.as_ref(),
                OPEN_AI_API_URL,
                &self.api_key,
                batch_id,
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to retrieve batch {}: {:?}", batch_id, e))?;

            log::info!("Batch {} status: {}", batch_id, batch_status.status);

            if batch_status.status != "completed" {
                log::warn!(
                    "Batch {} is not completed (status: {}), skipping",
                    batch_id,
                    batch_status.status
                );
                continue;
            }

            let output_file_id = batch_status.output_file_id.ok_or_else(|| {
                anyhow::anyhow!("Batch {} completed but has no output file", batch_id)
            })?;

            let results_content = batches::download_file(
                self.http_client.as_ref(),
                OPEN_AI_API_URL,
                &self.api_key,
                &output_file_id,
            )
            .await
            .map_err(|e| {
                anyhow::anyhow!("Failed to download batch results for {}: {:?}", batch_id, e)
            })?;

            let results = batches::parse_batch_output(&results_content)
                .map_err(|e| anyhow::anyhow!("Failed to parse batch output: {:?}", e))?;

            let mut updates: Vec<(String, String, String)> = Vec::new();
            let mut success_count = 0;
            let mut error_count = 0;

            for result in results {
                let request_hash = result
                    .custom_id
                    .strip_prefix("req_hash_")
                    .unwrap_or(&result.custom_id)
                    .to_string();

                if let Some(response_body) = result.response {
                    if response_body.status_code == 200 {
                        let response_json = serde_json::to_string(&response_body.body)?;
                        updates.push((request_hash, response_json, batch_id.clone()));
                        success_count += 1;
                    } else {
                        log::error!(
                            "Batch request {} failed with status {}",
                            request_hash,
                            response_body.status_code
                        );
                        let error_json = serde_json::json!({
                            "error": {
                                "type": "http_error",
                                "status_code": response_body.status_code
                            }
                        })
                        .to_string();
                        updates.push((request_hash, error_json, batch_id.clone()));
                        error_count += 1;
                    }
                } else if let Some(error) = result.error {
                    log::error!(
                        "Batch request {} failed: {}: {}",
                        request_hash,
                        error.code,
                        error.message
                    );
                    let error_json = serde_json::json!({
                        "error": {
                            "type": error.code,
                            "message": error.message
                        }
                    })
                    .to_string();
                    updates.push((request_hash, error_json, batch_id.clone()));
                    error_count += 1;
                }
            }

            let connection = self.connection.lock().unwrap();
            connection.with_savepoint("batch_import", || {
                let q = sql!(
                    INSERT OR REPLACE INTO openai_cache(request_hash, request, response, batch_id)
                    VALUES (?, (SELECT request FROM openai_cache WHERE request_hash = ?), ?, ?)
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
            let q = sql!(SELECT DISTINCT batch_id FROM openai_cache WHERE batch_id IS NOT NULL AND response IS NULL);
            connection.select(q)?()?
        };

        for batch_id in &batch_ids {
            let batch_status = batches::retrieve_batch(
                self.http_client.as_ref(),
                OPEN_AI_API_URL,
                &self.api_key,
                batch_id,
            )
            .await
            .map_err(|e| anyhow::anyhow!("{:?}", e))?;

            log::info!("Batch {} status: {}", batch_id, batch_status.status);

            if batch_status.status == "completed" {
                let output_file_id = match batch_status.output_file_id {
                    Some(id) => id,
                    None => {
                        log::warn!("Batch {} completed but has no output file", batch_id);
                        continue;
                    }
                };

                let results_content = batches::download_file(
                    self.http_client.as_ref(),
                    OPEN_AI_API_URL,
                    &self.api_key,
                    &output_file_id,
                )
                .await
                .map_err(|e| anyhow::anyhow!("{:?}", e))?;

                let results = batches::parse_batch_output(&results_content)
                    .map_err(|e| anyhow::anyhow!("Failed to parse batch output: {:?}", e))?;

                let mut updates: Vec<(String, String)> = Vec::new();
                let mut success_count = 0;

                for result in results {
                    let request_hash = result
                        .custom_id
                        .strip_prefix("req_hash_")
                        .unwrap_or(&result.custom_id)
                        .to_string();

                    if let Some(response_body) = result.response {
                        if response_body.status_code == 200 {
                            let response_json = serde_json::to_string(&response_body.body)?;
                            updates.push((response_json, request_hash));
                            success_count += 1;
                        } else {
                            log::error!(
                                "Batch request {} failed with status {}",
                                request_hash,
                                response_body.status_code
                            );
                            let error_json = serde_json::json!({
                                "error": {
                                    "type": "http_error",
                                    "status_code": response_body.status_code
                                }
                            })
                            .to_string();
                            updates.push((error_json, request_hash));
                        }
                    } else if let Some(error) = result.error {
                        log::error!(
                            "Batch request {} failed: {}: {}",
                            request_hash,
                            error.code,
                            error.message
                        );
                        let error_json = serde_json::json!({
                            "error": {
                                "type": error.code,
                                "message": error.message
                            }
                        })
                        .to_string();
                        updates.push((error_json, request_hash));
                    }
                }

                let connection = self.connection.lock().unwrap();
                connection.with_savepoint("batch_download", || {
                    let q = sql!(UPDATE openai_cache SET response = ? WHERE request_hash = ?);
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
                    SELECT request_hash, request FROM openai_cache
                    WHERE batch_id IS NULL AND response IS NULL
                    LIMIT ?
                );
                connection.select_bound(q)?(BATCH_CHUNK_SIZE)?
            };

            if rows.is_empty() {
                break;
            }

            let request_hashes: Vec<String> = rows.iter().map(|(hash, _)| hash.clone()).collect();

            let mut jsonl_content = String::new();
            for (hash, request_str) in &rows {
                let serializable_request: SerializableRequest =
                    serde_json::from_str(request_str).unwrap();

                let messages: Vec<RequestMessage> = serializable_request
                    .messages
                    .into_iter()
                    .map(|msg| match msg.role.as_str() {
                        "user" => RequestMessage::User {
                            content: MessageContent::Plain(msg.content),
                        },
                        "assistant" => RequestMessage::Assistant {
                            content: Some(MessageContent::Plain(msg.content)),
                            tool_calls: Vec::new(),
                        },
                        "system" => RequestMessage::System {
                            content: MessageContent::Plain(msg.content),
                        },
                        _ => RequestMessage::User {
                            content: MessageContent::Plain(msg.content),
                        },
                    })
                    .collect();

                let request = OpenAiRequest {
                    model: serializable_request.model,
                    messages,
                    stream: false,
                    max_completion_tokens: Some(serializable_request.max_tokens),
                    stop: Vec::new(),
                    temperature: None,
                    tool_choice: None,
                    parallel_tool_calls: None,
                    tools: Vec::new(),
                    prompt_cache_key: None,
                    reasoning_effort: None,
                };

                let custom_id = format!("req_hash_{}", hash);
                let batch_item = batches::BatchRequestItem::new(custom_id, request);
                let line = batch_item
                    .to_jsonl_line()
                    .map_err(|e| anyhow::anyhow!("Failed to serialize batch item: {:?}", e))?;
                jsonl_content.push_str(&line);
                jsonl_content.push('\n');
            }

            let filename = format!("batch_{}.jsonl", chrono::Utc::now().timestamp());
            let file_obj = batches::upload_batch_file(
                self.http_client.as_ref(),
                OPEN_AI_API_URL,
                &self.api_key,
                &filename,
                jsonl_content.into_bytes(),
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to upload batch file: {:?}", e))?;

            let batch = batches::create_batch(
                self.http_client.as_ref(),
                OPEN_AI_API_URL,
                &self.api_key,
                batches::CreateBatchRequest::new(file_obj.id),
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create batch: {:?}", e))?;

            {
                let connection = self.connection.lock().unwrap();
                connection.with_savepoint("batch_upload", || {
                    let q = sql!(UPDATE openai_cache SET batch_id = ? WHERE request_hash = ?);
                    let mut exec = connection.exec_bound::<(&str, &str)>(q)?;
                    for hash in &request_hashes {
                        exec((batch.id.as_str(), hash.as_str()))?;
                    }
                    Ok(())
                })?;
            }

            let batch_len = rows.len();
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
        messages: &[RequestMessage],
        seed: Option<usize>,
    ) -> String {
        let mut hasher = std::hash::DefaultHasher::new();
        "openai".hash(&mut hasher);
        model.hash(&mut hasher);
        max_tokens.hash(&mut hasher);
        for msg in messages {
            message_content_to_string(msg).hash(&mut hasher);
        }
        if let Some(seed) = seed {
            seed.hash(&mut hasher);
        }
        let request_hash = hasher.finish();
        format!("{request_hash:016x}")
    }
}

fn message_role_to_string(msg: &RequestMessage) -> String {
    match msg {
        RequestMessage::User { .. } => "user".to_string(),
        RequestMessage::Assistant { .. } => "assistant".to_string(),
        RequestMessage::System { .. } => "system".to_string(),
        RequestMessage::Tool { .. } => "tool".to_string(),
    }
}

fn message_content_to_string(msg: &RequestMessage) -> String {
    match msg {
        RequestMessage::User { content } => content_to_string(content),
        RequestMessage::Assistant { content, .. } => {
            content.as_ref().map(content_to_string).unwrap_or_default()
        }
        RequestMessage::System { content } => content_to_string(content),
        RequestMessage::Tool { content, .. } => content_to_string(content),
    }
}

fn content_to_string(content: &MessageContent) -> String {
    match content {
        MessageContent::Plain(text) => text.clone(),
        MessageContent::Multipart(parts) => parts
            .iter()
            .filter_map(|part| match part {
                open_ai::MessagePart::Text { text } => Some(text.clone()),
                _ => None,
            })
            .collect::<Vec<String>>()
            .join("\n"),
    }
}

pub enum OpenAiClient {
    Plain(PlainOpenAiClient),
    Batch(BatchingOpenAiClient),
    #[allow(dead_code)]
    Dummy,
}

impl OpenAiClient {
    pub fn plain() -> Result<Self> {
        Ok(Self::Plain(PlainOpenAiClient::new()?))
    }

    pub fn batch(cache_path: &Path) -> Result<Self> {
        Ok(Self::Batch(BatchingOpenAiClient::new(cache_path)?))
    }

    #[allow(dead_code)]
    pub fn dummy() -> Self {
        Self::Dummy
    }

    pub async fn generate(
        &self,
        model: &str,
        max_tokens: u64,
        messages: Vec<RequestMessage>,
        seed: Option<usize>,
        cache_only: bool,
    ) -> Result<Option<OpenAiResponse>> {
        match self {
            OpenAiClient::Plain(plain_client) => plain_client
                .generate(model, max_tokens, messages)
                .await
                .map(Some),
            OpenAiClient::Batch(batching_client) => {
                batching_client
                    .generate(model, max_tokens, messages, seed, cache_only)
                    .await
            }
            OpenAiClient::Dummy => panic!("Dummy OpenAI client is not expected to be used"),
        }
    }

    pub async fn sync_batches(&self) -> Result<()> {
        match self {
            OpenAiClient::Plain(_) => Ok(()),
            OpenAiClient::Batch(batching_client) => batching_client.sync_batches().await,
            OpenAiClient::Dummy => panic!("Dummy OpenAI client is not expected to be used"),
        }
    }

    pub async fn import_batches(&self, batch_ids: &[String]) -> Result<()> {
        match self {
            OpenAiClient::Plain(_) => {
                anyhow::bail!("Import batches is only supported with batching client")
            }
            OpenAiClient::Batch(batching_client) => batching_client.import_batches(batch_ids).await,
            OpenAiClient::Dummy => panic!("Dummy OpenAI client is not expected to be used"),
        }
    }
}
