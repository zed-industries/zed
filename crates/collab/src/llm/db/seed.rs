use super::*;
use crate::{Config, Result};
use queries::providers::ModelParams;

pub async fn seed_database(_config: &Config, db: &mut LlmDatabase, _force: bool) -> Result<()> {
    db.insert_models(&[
        ModelParams {
            provider: LanguageModelProvider::Anthropic,
            name: "claude-3-5-sonnet".into(),
            max_requests_per_minute: 5,
            max_tokens_per_minute: 20_000,
            max_tokens_per_day: 300_000,
            price_per_million_input_tokens: 300,   // $3.00/MTok
            price_per_million_output_tokens: 1500, // $15.00/MTok
        },
        ModelParams {
            provider: LanguageModelProvider::Anthropic,
            name: "claude-3-opus".into(),
            max_requests_per_minute: 5,
            max_tokens_per_minute: 10_000,
            max_tokens_per_day: 300_000,
            price_per_million_input_tokens: 1500,  // $15.00/MTok
            price_per_million_output_tokens: 7500, // $75.00/MTok
        },
        ModelParams {
            provider: LanguageModelProvider::Anthropic,
            name: "claude-3-sonnet".into(),
            max_requests_per_minute: 5,
            max_tokens_per_minute: 20_000,
            max_tokens_per_day: 300_000,
            price_per_million_input_tokens: 1500,  // $15.00/MTok
            price_per_million_output_tokens: 7500, // $75.00/MTok
        },
        ModelParams {
            provider: LanguageModelProvider::Anthropic,
            name: "claude-3-haiku".into(),
            max_requests_per_minute: 5,
            max_tokens_per_minute: 25_000,
            max_tokens_per_day: 300_000,
            price_per_million_input_tokens: 25,   // $0.25/MTok
            price_per_million_output_tokens: 125, // $1.25/MTok
        },
    ])
    .await
}
