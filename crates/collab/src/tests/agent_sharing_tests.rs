use gpui::{BackgroundExecutor, TestAppContext};
use rpc::proto;

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

    let title = "My Test Thread";
    let thread_data = br#"{"title":"My Test Thread","messages":[],"updated_at":"2024-01-01T00:00:00Z","version":"1.0.0"}"#.to_vec();

    let share_response = client_a
        .client()
        .request(proto::ShareAgentThread {
            title: title.to_string(),
            thread_data: thread_data.clone(),
        })
        .await
        .expect("Failed to share thread");

    assert!(share_response.share_id > 0, "Share ID should be positive");

    let get_response = client_b
        .client()
        .request(proto::GetSharedAgentThread {
            share_id: share_response.share_id,
        })
        .await
        .expect("Failed to get shared thread");

    assert_eq!(get_response.title, title);
    assert_eq!(get_response.thread_data, thread_data);
    assert_eq!(get_response.sharer_username, "user_a");
    assert!(!get_response.created_at.is_empty());
}

#[gpui::test]
async fn test_get_nonexistent_thread(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    let mut server = TestServer::start(executor.clone()).await;
    let client = server.create_client(cx, "user_a").await;

    executor.run_until_parked();

    let result = client
        .client()
        .request(proto::GetSharedAgentThread { share_id: 99999999 })
        .await;

    assert!(result.is_err(), "Should fail for nonexistent thread");
}
