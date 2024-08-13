use anyhow::Result;
use serde::Serialize;

#[derive(Serialize, Debug, clickhouse::Row)]
pub struct LlmUsageEventRow {
    pub time: i64,
    pub user_id: i32,
    pub is_staff: bool,
    pub plan: String,
    pub model: String,
    pub provider: String,
    pub input_token_count: u64,
    pub output_token_count: u64,
    pub requests_this_minute: u64,
    pub tokens_this_minute: u64,
    pub tokens_this_day: u64,
    pub input_tokens_this_month: u64,
    pub output_tokens_this_month: u64,
    pub spending_this_month: u64,
    pub lifetime_spending: u64,
}

#[derive(Serialize, Debug, clickhouse::Row)]
pub struct LlmRateLimitEventRow {
    pub time: i64,
    pub user_id: i32,
    pub is_staff: bool,
    pub plan: String,
    pub model: String,
    pub provider: String,
    pub usage_measure: String,
    pub requests_this_minute: u64,
    pub tokens_this_minute: u64,
    pub tokens_this_day: u64,
    pub users_in_recent_minutes: u64,
    pub users_in_recent_days: u64,
    pub max_requests_per_minute: u64,
    pub max_tokens_per_minute: u64,
    pub max_tokens_per_day: u64,
}

pub async fn report_llm_usage(client: &clickhouse::Client, row: LlmUsageEventRow) -> Result<()> {
    let mut insert = client.insert("llm_usage_events")?;
    insert.write(&row).await?;
    insert.end().await?;
    Ok(())
}

pub async fn report_llm_rate_limit(
    client: &clickhouse::Client,
    row: LlmRateLimitEventRow,
) -> Result<()> {
    let mut insert = client.insert("llm_rate_limits")?;
    insert.write(&row).await?;
    insert.end().await?;
    Ok(())
}
