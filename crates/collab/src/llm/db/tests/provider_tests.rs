use std::sync::Arc;

use pretty_assertions::assert_eq;

use crate::llm::db::LlmDatabase;
use crate::test_both_llm_dbs;

test_both_llm_dbs!(
    test_initialize_providers,
    test_initialize_providers_postgres,
    test_initialize_providers_sqlite
);

async fn test_initialize_providers(db: &Arc<LlmDatabase>) {
    let initial_providers = db.list_providers().await.unwrap();
    assert_eq!(initial_providers, vec![]);

    db.initialize_providers().await.unwrap();

    // Do it twice, to make sure the operation is idempotent.
    db.initialize_providers().await.unwrap();

    let providers = db.list_providers().await.unwrap();

    let provider_names = providers
        .into_iter()
        .map(|provider| provider.name)
        .collect::<Vec<_>>();
    assert_eq!(provider_names, vec!["anthropic".to_string()]);
}
