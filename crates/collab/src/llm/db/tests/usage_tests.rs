use std::sync::Arc;

use pretty_assertions::assert_eq;
use rpc::LanguageModelProvider;

use crate::llm::db::LlmDatabase;
use crate::test_both_llm_dbs;

test_both_llm_dbs!(
    test_find_or_create_usage,
    test_find_or_create_usage_postgres,
    test_find_or_create_usage_sqlite
);

async fn test_find_or_create_usage(db: &Arc<LlmDatabase>) {
    db.initialize_providers().await.unwrap();

    let usage = db
        .find_or_create_usage(123, LanguageModelProvider::Anthropic, "claude-3-5-sonnet")
        .await
        .unwrap();

    assert_eq!(usage.user_id, 123);
}
