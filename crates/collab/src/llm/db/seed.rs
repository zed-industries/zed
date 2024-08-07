use super::LlmDatabase;
use super::*;
use crate::{Config, Result};

pub async fn seed_database(_config: &Config, db: &LlmDatabase, _force: bool) -> Result<()> {
    db.transaction(|tx| async move {
        let anthropic_id = db.provider_ids[&LanguageModelProvider::Anthropic];

        model::Entity::insert_many(vec![
            model::ActiveModel {
                provider_id: ActiveValue::set(anthropic_id),
                name: ActiveValue::set("claude-3-5-sonnet".to_string()),
                max_requests_per_minute: ActiveValue::set(5),
                max_tokens_per_minute: ActiveValue::set(20_000),
                max_tokens_per_day: ActiveValue::set(300_000),
                ..Default::default()
            },
            model::ActiveModel {
                provider_id: ActiveValue::set(anthropic_id),
                name: ActiveValue::set("claude-3-opus".to_string()),
                max_requests_per_minute: ActiveValue::set(5),
                max_tokens_per_minute: ActiveValue::set(10_000),
                max_tokens_per_day: ActiveValue::set(300_000),
                ..Default::default()
            },
            model::ActiveModel {
                provider_id: ActiveValue::set(anthropic_id),
                name: ActiveValue::set("claude-3-sonnet".to_string()),
                max_requests_per_minute: ActiveValue::set(5),
                max_tokens_per_minute: ActiveValue::set(20_000),
                max_tokens_per_day: ActiveValue::set(300_000),
                ..Default::default()
            },
            model::ActiveModel {
                provider_id: ActiveValue::set(anthropic_id),
                name: ActiveValue::set("claude-3-haiku".to_string()),
                max_requests_per_minute: ActiveValue::set(5),
                max_tokens_per_minute: ActiveValue::set(25_000),
                max_tokens_per_day: ActiveValue::set(300_000),
                ..Default::default()
            },
        ])
        .exec_without_returning(&*tx)
        .await?;

        Ok(())
    })
    .await
}
