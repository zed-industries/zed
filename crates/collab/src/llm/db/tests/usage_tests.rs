use crate::{
    db::UserId,
    llm::db::{
        queries::{providers::ModelParams, usages::Usage},
        LlmDatabase,
    },
    test_llm_db,
};
use chrono::{DateTime, Duration, Utc};
use pretty_assertions::assert_eq;
use rpc::LanguageModelProvider;

test_llm_db!(test_tracking_usage, test_tracking_usage_postgres);

async fn test_tracking_usage(db: &mut LlmDatabase) {
    let provider = LanguageModelProvider::Anthropic;
    let model = "claude-3-5-sonnet";

    db.initialize().await.unwrap();
    db.insert_models(&[ModelParams {
        provider,
        name: model.to_string(),
        max_requests_per_minute: 5,
        max_tokens_per_minute: 10_000,
        max_tokens_per_day: 50_000,
        price_per_million_input_tokens: 50,
        price_per_million_output_tokens: 50,
    }])
    .await
    .unwrap();

    // We're using a fixed datetime to prevent flakiness based on the clock.
    let t0 = DateTime::parse_from_rfc3339("2024-08-08T22:46:33Z")
        .unwrap()
        .with_timezone(&Utc);
    let user_id = UserId::from_proto(123);

    let now = t0;
    db.record_usage(user_id, false, provider, model, 1000, 0, 0, 0, now)
        .await
        .unwrap();

    let now = t0 + Duration::seconds(10);
    db.record_usage(user_id, false, provider, model, 2000, 0, 0, 0, now)
        .await
        .unwrap();

    let usage = db.get_usage(user_id, provider, model, now).await.unwrap();
    assert_eq!(
        usage,
        Usage {
            requests_this_minute: 2,
            tokens_this_minute: 3000,
            tokens_this_day: 3000,
            input_tokens_this_month: 3000,
            cache_creation_input_tokens_this_month: 0,
            cache_read_input_tokens_this_month: 0,
            output_tokens_this_month: 0,
            spending_this_month: 0,
            lifetime_spending: 0,
        }
    );

    let now = t0 + Duration::seconds(60);
    let usage = db.get_usage(user_id, provider, model, now).await.unwrap();
    assert_eq!(
        usage,
        Usage {
            requests_this_minute: 1,
            tokens_this_minute: 2000,
            tokens_this_day: 3000,
            input_tokens_this_month: 3000,
            cache_creation_input_tokens_this_month: 0,
            cache_read_input_tokens_this_month: 0,
            output_tokens_this_month: 0,
            spending_this_month: 0,
            lifetime_spending: 0,
        }
    );

    let now = t0 + Duration::seconds(60);
    db.record_usage(user_id, false, provider, model, 3000, 0, 0, 0, now)
        .await
        .unwrap();

    let usage = db.get_usage(user_id, provider, model, now).await.unwrap();
    assert_eq!(
        usage,
        Usage {
            requests_this_minute: 2,
            tokens_this_minute: 5000,
            tokens_this_day: 6000,
            input_tokens_this_month: 6000,
            cache_creation_input_tokens_this_month: 0,
            cache_read_input_tokens_this_month: 0,
            output_tokens_this_month: 0,
            spending_this_month: 0,
            lifetime_spending: 0,
        }
    );

    let t1 = t0 + Duration::hours(24);
    let now = t1;
    let usage = db.get_usage(user_id, provider, model, now).await.unwrap();
    assert_eq!(
        usage,
        Usage {
            requests_this_minute: 0,
            tokens_this_minute: 0,
            tokens_this_day: 5000,
            input_tokens_this_month: 6000,
            cache_creation_input_tokens_this_month: 0,
            cache_read_input_tokens_this_month: 0,
            output_tokens_this_month: 0,
            spending_this_month: 0,
            lifetime_spending: 0,
        }
    );

    db.record_usage(user_id, false, provider, model, 4000, 0, 0, 0, now)
        .await
        .unwrap();

    let usage = db.get_usage(user_id, provider, model, now).await.unwrap();
    assert_eq!(
        usage,
        Usage {
            requests_this_minute: 1,
            tokens_this_minute: 4000,
            tokens_this_day: 9000,
            input_tokens_this_month: 10000,
            cache_creation_input_tokens_this_month: 0,
            cache_read_input_tokens_this_month: 0,
            output_tokens_this_month: 0,
            spending_this_month: 0,
            lifetime_spending: 0,
        }
    );

    // We're using a fixed datetime to prevent flakiness based on the clock.
    let now = DateTime::parse_from_rfc3339("2024-10-08T22:15:58Z")
        .unwrap()
        .with_timezone(&Utc);

    // Test cache creation input tokens
    db.record_usage(user_id, false, provider, model, 1000, 500, 0, 0, now)
        .await
        .unwrap();

    let usage = db.get_usage(user_id, provider, model, now).await.unwrap();
    assert_eq!(
        usage,
        Usage {
            requests_this_minute: 1,
            tokens_this_minute: 1500,
            tokens_this_day: 1500,
            input_tokens_this_month: 1000,
            cache_creation_input_tokens_this_month: 500,
            cache_read_input_tokens_this_month: 0,
            output_tokens_this_month: 0,
            spending_this_month: 0,
            lifetime_spending: 0,
        }
    );

    // Test cache read input tokens
    db.record_usage(user_id, false, provider, model, 1000, 0, 300, 0, now)
        .await
        .unwrap();

    let usage = db.get_usage(user_id, provider, model, now).await.unwrap();
    assert_eq!(
        usage,
        Usage {
            requests_this_minute: 2,
            tokens_this_minute: 2800,
            tokens_this_day: 2800,
            input_tokens_this_month: 2000,
            cache_creation_input_tokens_this_month: 500,
            cache_read_input_tokens_this_month: 300,
            output_tokens_this_month: 0,
            spending_this_month: 0,
            lifetime_spending: 0,
        }
    );
}
