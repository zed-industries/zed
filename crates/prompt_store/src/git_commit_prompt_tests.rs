use super::*;
use gpui::TestAppContext;
use util::TryFutureExt;

fn init_test(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let settings_store = settings::SettingsStore::test(cx);
        cx.set_global(settings_store);
    });
}

#[gpui::test]
async fn test_commit_prompt_initializes_with_default_metadata(cx: &mut TestAppContext) {
    init_test(cx);

    let db_path = std::env::temp_dir().join("test-commit-init.mdb");
    let _ = std::fs::remove_dir_all(&db_path);
    let store_task = cx.update(|cx| PromptStore::new(db_path, cx));
    let store = store_task.await.unwrap();
    let store = cx.update(|cx| cx.new(|_| store));

    let metadata = cx.read_entity(&store, |store: &PromptStore, _| {
        store.metadata(PromptId::CommitMessage).unwrap()
    });

    assert_eq!(
        metadata.title.as_ref().map(|s| s.as_ref()),
        Some("Git Commit Prompt")
    );
    assert!(metadata.default);

    let body_task = cx.update(|cx| {
        store
            .update(cx, |store, cx| store.load(PromptId::CommitMessage, cx))
            .unwrap()
    });
    let body = body_task.await;
    const DEFAULT_COMMIT_PROMPT: &str = include_str!("../../git_ui/src/commit_message_prompt.txt");
    let mut expected_prompt = DEFAULT_COMMIT_PROMPT.to_string();
    text::LineEnding::normalize(&mut expected_prompt);
    assert_eq!(body, expected_prompt);
}

#[gpui::test]
async fn test_save_commit_prompt_updates_body_and_preserves_title(cx: &mut TestAppContext) {
    init_test(cx);

    let db_path = std::env::temp_dir().join("test-commit-save.mdb");
    let _ = std::fs::remove_dir_all(&db_path);
    let store_task = cx.update(|cx| PromptStore::new(db_path, cx));
    let store = store_task.await.unwrap();
    let store = cx.update(|cx| cx.new(|_| store));

    let custom_body = "Line one\nLine two";

    let save_task = cx.update(|cx| {
        store.update(cx, |store, cx| {
            store.save(
                PromptId::CommitMessage,
                Some("Ignored title".into()),
                true,
                custom_body.into(),
                cx,
            )
        })
    });
    save_task.await.unwrap();
    cx.run_until_parked();

    let (metadata, body_task) = cx.update(|cx| {
        let metadata = cx.read_entity(&store, |store: &PromptStore, _| {
            store.metadata(PromptId::CommitMessage).unwrap()
        });
        let body = store
            .update(cx, |store, cx| store.load(PromptId::CommitMessage, cx))
            .unwrap();
        (metadata, body)
    });
    let body = body_task.await;

    assert_eq!(
        metadata.title.as_ref().map(|s| s.as_ref()),
        Some("Git Commit Prompt")
    );
    assert_eq!(body, custom_body);
}

#[gpui::test]
async fn test_commit_prompt_default_flag_persists_across_sessions(cx: &mut TestAppContext) {
    init_test(cx);

    let db_path = std::env::temp_dir().join("test-commit-default.mdb");
    let _ = std::fs::remove_dir_all(&db_path);

    let store_task = cx.update(|cx| PromptStore::new(db_path.clone(), cx));
    let store = store_task.await.unwrap();
    let store = cx.update(|cx| cx.new(|_| store));

    let save_task = cx.update(|cx| {
        store.update(cx, |store, cx| {
            store.save(
                PromptId::CommitMessage,
                Some("Ignored".into()),
                false,
                "custom instructions".into(),
                cx,
            )
        })
    });
    save_task.await.unwrap();
    cx.run_until_parked();
    drop(store);

    let store_task = cx.update(|cx| PromptStore::new(db_path, cx));
    let reopened = store_task.await.unwrap();
    let reopened = cx.update(|cx| cx.new(|_| reopened));

    let metadata = cx.read_entity(&reopened, |store: &PromptStore, _| {
        store.metadata(PromptId::CommitMessage).unwrap()
    });

    assert!(!metadata.default);
    assert_eq!(
        metadata.title.as_ref().map(|s| s.as_ref()),
        Some("Git Commit Prompt")
    );
}

#[gpui::test]
async fn test_metadata_cache_updates_after_commit_prompt_save(cx: &mut TestAppContext) {
    init_test(cx);

    let db_path = std::env::temp_dir().join("test-commit-cache.mdb");
    let _ = std::fs::remove_dir_all(&db_path);
    let store_task = cx.update(|cx| PromptStore::new(db_path, cx));
    let store = store_task.await.unwrap();
    let store = cx.update(|cx| cx.new(|_| store));

    let initial_metadata = cx.read_entity(&store, |store: &PromptStore, _| {
        store.metadata(PromptId::CommitMessage).unwrap()
    });

    let updated_body = "updated commit body";
    let save_task = cx.update(|cx| {
        store.update(cx, |store, cx| {
            store.save(
                PromptId::CommitMessage,
                None,
                initial_metadata.default,
                updated_body.into(),
                cx,
            )
        })
    });
    save_task.await.unwrap();
    cx.run_until_parked();

    let (metadata, body_task) = cx.update(|cx| {
        let metadata = cx.read_entity(&store, |store: &PromptStore, _| {
            store.metadata(PromptId::CommitMessage).unwrap()
        });
        let body = store
            .update(cx, |store, cx| store.load(PromptId::CommitMessage, cx))
            .unwrap();
        (metadata, body)
    });
    let body = body_task.await;

    assert_ne!(metadata.saved_at, initial_metadata.saved_at);
    assert_eq!(body, updated_body);
}
