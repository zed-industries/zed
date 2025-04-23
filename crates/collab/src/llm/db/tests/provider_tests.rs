use pretty_assertions::assert_eq;
use zed_llm_client::LanguageModelProvider;

use crate::llm::db::LlmDatabase;
use crate::test_llm_db;

test_llm_db!(
    test_initialize_providers,
    test_initialize_providers_postgres
);

async fn test_initialize_providers(db: &mut LlmDatabase) {
    let initial_providers = db.list_providers().await.unwrap();
    assert_eq!(initial_providers, vec![]);

    db.initialize_providers().await.unwrap();

    // Do it twice, to make sure the operation is idempotent.
    db.initialize_providers().await.unwrap();

    let providers = db.list_providers().await.unwrap();

    assert_eq!(
        providers,
        &[
            LanguageModelProvider::Anthropic,
            LanguageModelProvider::Google,
            LanguageModelProvider::OpenAi,
        ]
    )
}
