use super::*;
use crate::{Config, Result};
use queries::providers::ModelRateLimits;

pub async fn seed_database(_config: &Config, db: &mut LlmDatabase, _force: bool) -> Result<()> {
    db.insert_models(&[
        (
            LanguageModelProvider::Anthropic,
            "claude-3-5-sonnet".into(),
            ModelRateLimits {
                max_requests_per_minute: 5,
                max_tokens_per_minute: 20_000,
                max_tokens_per_day: 300_000,
            },
        ),
        (
            LanguageModelProvider::Anthropic,
            "claude-3-opus".into(),
            ModelRateLimits {
                max_requests_per_minute: 5,
                max_tokens_per_minute: 10_000,
                max_tokens_per_day: 300_000,
            },
        ),
        (
            LanguageModelProvider::Anthropic,
            "claude-3-sonnet".into(),
            ModelRateLimits {
                max_requests_per_minute: 5,
                max_tokens_per_minute: 20_000,
                max_tokens_per_day: 300_000,
            },
        ),
        (
            LanguageModelProvider::Anthropic,
            "claude-3-haiku".into(),
            ModelRateLimits {
                max_requests_per_minute: 5,
                max_tokens_per_minute: 25_000,
                max_tokens_per_day: 300_000,
            },
        ),
    ])
    .await
}
