use crate::{
    db::UserId,
    llm::{
        db::{
            queries::{providers::ModelParams, usages::Usage},
            LlmDatabase, TokenUsage,
        },
        FREE_TIER_MONTHLY_SPENDING_LIMIT,
    },
    test_llm_db, Cents,
};
use chrono::{DateTime, Utc};
use pretty_assertions::assert_eq;
use rpc::LanguageModelProvider;

test_llm_db!(
    test_billing_limit_exceeded,
    test_billing_limit_exceeded_postgres
);

async fn test_billing_limit_exceeded(db: &mut LlmDatabase) {
    let provider = LanguageModelProvider::Anthropic;
    let model = "fake-claude-limerick";
    const PRICE_PER_MILLION_INPUT_TOKENS: i32 = 5;
    const PRICE_PER_MILLION_OUTPUT_TOKENS: i32 = 5;

    // Initialize the database and insert the model
    db.initialize().await.unwrap();
    db.insert_models(&[ModelParams {
        provider,
        name: model.to_string(),
        max_requests_per_minute: 5,
        max_tokens_per_minute: 10_000,
        max_tokens_per_day: 50_000,
        price_per_million_input_tokens: PRICE_PER_MILLION_INPUT_TOKENS,
        price_per_million_output_tokens: PRICE_PER_MILLION_OUTPUT_TOKENS,
    }])
    .await
    .unwrap();

    // Set a fixed datetime for consistent testing
    let now = DateTime::parse_from_rfc3339("2024-08-08T22:46:33Z")
        .unwrap()
        .with_timezone(&Utc);

    let user_id = UserId::from_proto(123);

    let max_monthly_spend = Cents::from_dollars(10);

    // Record usage that brings us close to the limit but doesn't exceed it
    // Let's say we use $9.50 worth of tokens
    let tokens_to_use = 190_000_000; // This will cost $9.50 at $0.05 per 1 million tokens
    let usage = TokenUsage {
        input: tokens_to_use,
        input_cache_creation: 0,
        input_cache_read: 0,
        output: 0,
    };
    let cost = Cents::new(tokens_to_use as u32 / 1_000_000 * PRICE_PER_MILLION_INPUT_TOKENS as u32);

    assert_eq!(
        cost,
        Cents::new(950),
        "expected the cost to be $9.50, based on the inputs, but it wasn't"
    );

    // Verify that before we record any usage, there are 0 billing events
    let billing_events = db.get_billing_events().await.unwrap();
    assert_eq!(billing_events.len(), 0);

    db.record_usage(
        user_id,
        false,
        provider,
        model,
        usage,
        true,
        max_monthly_spend,
        now,
    )
    .await
    .unwrap();

    // Verify the recorded usage and spending
    let recorded_usage = db.get_usage(user_id, provider, model, now).await.unwrap();

    // Verify that we exceeded the free tier usage
    assert!(
        recorded_usage.spending_this_month > FREE_TIER_MONTHLY_SPENDING_LIMIT,
        "Expected spending to exceed free tier limit"
    );

    assert_eq!(
        recorded_usage,
        Usage {
            requests_this_minute: 1,
            tokens_this_minute: tokens_to_use,
            tokens_this_day: tokens_to_use,
            tokens_this_month: TokenUsage {
                input: tokens_to_use,
                input_cache_creation: 0,
                input_cache_read: 0,
                output: 0,
            },
            spending_this_month: Cents::new(950),
            lifetime_spending: Cents::new(950),
        }
    );

    // Verify that there is one `billing_event` record
    let billing_events = db.get_billing_events().await.unwrap();
    assert_eq!(billing_events.len(), 1);

    let (billing_event, _model) = &billing_events[0];
    assert_eq!(billing_event.user_id, user_id);
    assert_eq!(billing_event.input_tokens, tokens_to_use as i64);
    assert_eq!(billing_event.input_cache_creation_tokens, 0);
    assert_eq!(billing_event.input_cache_read_tokens, 0);
    assert_eq!(billing_event.output_tokens, 0);

    let tokens_to_exceed = 20_000_000; // This will cost $1.00 more, pushing us from $9.50 to $10.50, which is over the $10 monthly maximum limit
    let usage_exceeding = TokenUsage {
        input: tokens_to_exceed,
        input_cache_creation: 0,
        input_cache_read: 0,
        output: 0,
    };

    // This should still create a billing event as it's the first request that exceeds the limit
    db.record_usage(
        user_id,
        false,
        provider,
        model,
        usage_exceeding,
        true,
        max_monthly_spend,
        now,
    )
    .await
    .unwrap();

    // Verify that there is still one billing record
    let billing_events = db.get_billing_events().await.unwrap();
    assert_eq!(billing_events.len(), 1);

    // Verify the updated usage and spending
    let updated_usage = db.get_usage(user_id, provider, model, now).await.unwrap();
    assert_eq!(
        updated_usage,
        Usage {
            requests_this_minute: 2,
            tokens_this_minute: tokens_to_use + tokens_to_exceed,
            tokens_this_day: tokens_to_use + tokens_to_exceed,
            tokens_this_month: TokenUsage {
                input: tokens_to_use + tokens_to_exceed,
                input_cache_creation: 0,
                input_cache_read: 0,
                output: 0,
            },
            spending_this_month: Cents::new(1050),
            lifetime_spending: Cents::new(1050),
        }
    );
}
