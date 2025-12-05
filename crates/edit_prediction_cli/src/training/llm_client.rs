use anthropic_sdk::BatchCreateParams;
use anthropic_sdk::BatchRequest;
use anthropic_sdk::MessageCreateBuilder;
use indoc::indoc;
use sqlez::bindable::Bind;
use sqlez::bindable::StaticColumnCount;
use sqlez_macros::sql;
use std::hash::Hash;
use std::hash::Hasher;

use anthropic_sdk::{Anthropic, BatchRequestBuilder, Message, MessageCreateParams};
use anyhow::Result;

pub struct PlainLlmClient {
    client: Anthropic,
}

impl PlainLlmClient {
    fn new() -> Result<Self> {
        let client = Anthropic::from_env()?;
        Ok(Self { client })
    }
    async fn generate(&self, message: MessageCreateParams) -> Result<Message> {
        Ok(self.client.messages().create(message).await?)
    }
}

pub struct BatchingLlmClient {
    connection: sqlez::connection::Connection,
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

impl BatchingLlmClient {
    fn new(cache_path: &str) -> Result<Self> {
        let connection = sqlez::connection::Connection::open_file(&cache_path);
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

        Ok(Self { connection })
    }

    pub fn lookup(&self, message: &MessageCreateParams) -> Result<Option<Message>> {
        let request_hash_str = Self::request_hash(message);
        let response: Vec<String> = self.connection.select_bound(
            &sql!(SELECT response FROM cache WHERE request_hash = ?1 AND response IS NOT NULL;),
        )?(request_hash_str.as_str())?;
        Ok(response
            .into_iter()
            .next()
            .and_then(|text| serde_json::from_str(&text).ok()))
    }

    pub fn mark_for_batch(&self, message: &MessageCreateParams) -> Result<()> {
        let request_hash = Self::request_hash(message);

        let request = Some(serde_json::to_string(message).unwrap());
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

    async fn generate(&self, message: MessageCreateParams) -> Result<Option<Message>> {
        let response = self.lookup(&message)?;
        if let Some(response) = response {
            return Ok(Some(response));
        }

        self.mark_for_batch(&message)?;

        Ok(None)
    }

    /// Uploads pending requests as a new batch; downloads finished batches if any.
    async fn sync_batches(&self) -> Result<()> {
        self.upload_pending_requests().await?;
        self.download_finished_batches().await
    }
    async fn download_finished_batches(&self) -> Result<()> {
        Ok(())
    }

    // https://on.tty-share.com/s/scB68CoH4O9K0xGHResjdM34DziPJGxyNzrUfFO3BsMm0YaodM49qmPxSP3yGPs7mh8/

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
                let params: MessageCreateParams = serde_json::from_str(&request_str).unwrap();
                let custom_id = format!("req_hash_{}", hash);
                BatchRequest {
                    custom_id,
                    method: "POST".to_string(),
                    url: "/v1/messages".to_string(),
                    params,
                }
            })
            .collect::<Vec<_>>();

        let client = Anthropic::from_env()?;

        let batch = client
            .batches()
            .create(BatchCreateParams::new(batch_requests))
            .await?;

        let q = sql!(
            UPDATE cache SET batch_id = ? WHERE batch_id is NULL
        );
        self.connection.exec_bound(q)?(batch.id.as_str())?;

        Ok(batch.id)

        // // Check batch status
        // let status = client.batches().retrieve(&batch.id).await?;
        // println!("Batch status: {:?}", status.processing_status);
    }

    fn request_hash(message: &MessageCreateParams) -> String {
        let mut hasher = std::hash::DefaultHasher::new();
        message_text(&message).hash(&mut hasher);
        let request_hash = hasher.finish();
        format!("{request_hash:016x}")
    }
}

fn message_text(message: &MessageCreateParams) -> String {
    message
        .messages
        .iter()
        .filter_map(|msg| match &msg.content {
            anthropic_sdk::MessageContent::Text(text) => Some(text.clone()),
            _ => None,
        })
        .collect::<Vec<String>>()
        .join("\n")
}

pub enum LlmClient {
    // No batching
    Plain(PlainLlmClient),
    Batch(BatchingLlmClient),
}

impl LlmClient {
    pub fn plain() -> Result<Self> {
        Ok(Self::Plain(PlainLlmClient::new()?))
    }
    pub fn batch(cache_path: &str) -> Result<Self> {
        Ok(Self::Batch(BatchingLlmClient::new(cache_path)?))
    }
    pub async fn generate(&self, message: MessageCreateParams) -> Result<Option<Message>> {
        match self {
            LlmClient::Plain(plain_llm_client) => {
                plain_llm_client.generate(message).await.map(Some)
            }
            LlmClient::Batch(batching_llm_client) => batching_llm_client.generate(message).await,
        }
    }

    pub async fn sync_batches(&self) -> Result<()> {
        match self {
            LlmClient::Plain(_) => Ok(()),
            LlmClient::Batch(batching_llm_client) => batching_llm_client.sync_batches().await,
        }
    }

    // let response = client
    //     .messages()
    //     .create(
    //         MessageCreateBuilder::new(self.llm_name.clone(), 16384)
    //             .user(prompt.clone())
    //             .build(),
    //     )
    //     .await?;
}
