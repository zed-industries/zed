use agent::SharedThread;
use gpui::{BackgroundExecutor, TestAppContext};
use rpc::proto;
use uuid::Uuid;

use crate::tests::TestServer;

#[gpui::test]
async fn test_share_and_retrieve_thread(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    executor.run_until_parked();

    let session_id = Uuid::new_v4().to_string();

    let original_thread = SharedThread {
        title: "Shared Test Thread".into(),
        messages: vec![],
        updated_at: chrono::Utc::now(),
        model: None,
        version: SharedThread::VERSION.to_string(),
    };

    let thread_data = original_thread
        .to_bytes()
        .expect("Failed to serialize thread");

    client_a
        .client()
        .request(proto::ShareAgentThread {
            session_id: session_id.clone(),
            title: original_thread.title.to_string(),
            thread_data,
        })
        .await
        .expect("Failed to share thread");

    let get_response = client_b
        .client()
        .request(proto::GetSharedAgentThread {
            session_id: session_id.clone(),
        })
        .await
        .expect("Failed to get shared thread");

    let imported_shared_thread =
        SharedThread::from_bytes(&get_response.thread_data).expect("Failed to deserialize thread");

    assert_eq!(imported_shared_thread.title, original_thread.title);
    assert_eq!(imported_shared_thread.version, SharedThread::VERSION);

    let db_thread = imported_shared_thread.to_db_thread();

    assert!(
        db_thread.title.starts_with("🔗"),
        "Imported thread title should have link prefix"
    );
    assert!(
        db_thread.title.contains("Shared Test Thread"),
        "Imported thread should preserve original title"
    );
}

#[gpui::test]
async fn test_reshare_updates_existing_thread(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    executor.run_until_parked();

    let session_id = Uuid::new_v4().to_string();

    client_a
        .client()
        .request(proto::ShareAgentThread {
            session_id: session_id.clone(),
            title: "Original Title".to_string(),
            thread_data: b"original data".to_vec(),
        })
        .await
        .expect("Failed to share thread");

    client_a
        .client()
        .request(proto::ShareAgentThread {
            session_id: session_id.clone(),
            title: "Updated Title".to_string(),
            thread_data: b"updated data".to_vec(),
        })
        .await
        .expect("Failed to re-share thread");

    let get_response = client_b
        .client()
        .request(proto::GetSharedAgentThread {
            session_id: session_id.clone(),
        })
        .await
        .expect("Failed to get shared thread");

    assert_eq!(get_response.title, "Updated Title");
    assert_eq!(get_response.thread_data, b"updated data".to_vec());
}

#[gpui::test]
async fn test_get_nonexistent_thread(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    let mut server = TestServer::start(executor.clone()).await;
    let client = server.create_client(cx, "user_a").await;

    executor.run_until_parked();

    let nonexistent_session_id = Uuid::new_v4().to_string();

    let result = client
        .client()
        .request(proto::GetSharedAgentThread {
            session_id: nonexistent_session_id,
        })
        .await;

    assert!(result.is_err(), "Should fail for nonexistent thread");
}

#[gpui::test]
async fn test_sync_imported_thread(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    executor.run_until_parked();

    let session_id = Uuid::new_v4().to_string();

    // User A shares a thread with initial content.
    let initial_thread = SharedThread {
        title: "Initial Title".into(),
        messages: vec![],
        updated_at: chrono::Utc::now(),
        model: None,
        version: SharedThread::VERSION.to_string(),
    };

    client_a
        .client()
        .request(proto::ShareAgentThread {
            session_id: session_id.clone(),
            title: initial_thread.title.to_string(),
            thread_data: initial_thread.to_bytes().expect("Failed to serialize"),
        })
        .await
        .expect("Failed to share thread");

    // User B imports the thread.
    let initial_response = client_b
        .client()
        .request(proto::GetSharedAgentThread {
            session_id: session_id.clone(),
        })
        .await
        .expect("Failed to get shared thread");

    let initial_imported =
        SharedThread::from_bytes(&initial_response.thread_data).expect("Failed to deserialize");
    assert_eq!(initial_imported.title.as_ref(), "Initial Title");

    // User A updates the shared thread.
    let updated_thread = SharedThread {
        title: "Updated Title".into(),
        messages: vec![],
        updated_at: chrono::Utc::now(),
        model: None,
        version: SharedThread::VERSION.to_string(),
    };

    client_a
        .client()
        .request(proto::ShareAgentThread {
            session_id: session_id.clone(),
            title: updated_thread.title.to_string(),
            thread_data: updated_thread.to_bytes().expect("Failed to serialize"),
        })
        .await
        .expect("Failed to re-share thread");

    // User B syncs the imported thread (fetches the latest version).
    let synced_response = client_b
        .client()
        .request(proto::GetSharedAgentThread {
            session_id: session_id.clone(),
        })
        .await
        .expect("Failed to sync shared thread");

    let synced_thread =
        SharedThread::from_bytes(&synced_response.thread_data).expect("Failed to deserialize");

    // The synced thread should have the updated title.
    assert_eq!(synced_thread.title.as_ref(), "Updated Title");
}
