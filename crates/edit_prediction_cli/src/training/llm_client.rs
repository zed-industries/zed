use indoc::indoc;
use sqlez::bindable::Bind;
use sqlez::bindable::StaticColumnCount;
use sqlez_macros::sql;
use std::hash::Hash;
use std::hash::Hasher;

use anthropic_sdk::{Anthropic, Message, MessageCreateParams};
use anyhow::Result;

pub enum LlmClient {
    // No batching
    Plain(PlainLlmClient),
    Batch(BatchingLlmClient),
}

pub struct PlainLlmClient;

impl PlainLlmClient {
    async fn generate(&self, message: MessageCreateParams) -> Result<Message> {
        let client = Anthropic::from_env()?;
        Ok(client.messages().create(message).await?)
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

// prompt_hash  request   null
// prompt_hash  request   batch_id response

// If I'm preparing a batch, will I reuse any of the existing results?
//
impl LlmClient {
    pub fn plain() -> Self {
        Self::Plain(PlainLlmClient)
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

    // let response = client
    //     .messages()
    //     .create(
    //         MessageCreateBuilder::new(self.llm_name.clone(), 16384)
    //             .user(prompt.clone())
    //             .build(),
    //     )
    //     .await?;
}
