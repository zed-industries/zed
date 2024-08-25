use anyhow::{Context, Result};
use serde::Serialize;

use crate::clickhouse::write_to_table;

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
    const LLM_USAGE_EVENTS_TABLE: &str = "llm_usage_events";
    write_to_table(LLM_USAGE_EVENTS_TABLE, &[row], client)
        .await
        .with_context(|| format!("failed to upload to table '{LLM_USAGE_EVENTS_TABLE}'"))?;
    Ok(())
}

pub async fn report_llm_rate_limit(
    client: &clickhouse::Client,
    row: LlmRateLimitEventRow,
) -> Result<()> {
    const LLM_RATE_LIMIT_EVENTS_TABLE: &str = "llm_rate_limit_events";
    write_to_table(LLM_RATE_LIMIT_EVENTS_TABLE, &[row], client)
        .await
        .with_context(|| format!("failed to upload to table '{LLM_RATE_LIMIT_EVENTS_TABLE}'"))?;
    Ok(())
}
