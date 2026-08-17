mod mock_error;

use mock_error::setup_error_handler;
use octocrab::Octocrab;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

async fn setup_delete_thread_sub_api(template: ResponseTemplate) -> MockServer {
    let thread_id = 123u64;

    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(format!(
            "/notifications/threads/{thread_id}/subscription"
        )))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("DELETE on /notifications/threads/{thread_id}/subscription was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

const THREAD_ID: u64 = 123u64;

#[tokio::test]
async fn delete_thread_subscription_204() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_delete_thread_sub_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .activity()
        .notifications()
        .delete_thread_subscription(THREAD_ID.into())
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn delete_thread_subscription_404() {
    let template = ResponseTemplate::new(404);
    let mock_server = setup_delete_thread_sub_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .activity()
        .notifications()
        .delete_thread_subscription(THREAD_ID.into())
        .await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn delete_thread_subscription_500() {
    let template = ResponseTemplate::new(500);
    let mock_server = setup_delete_thread_sub_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .activity()
        .notifications()
        .delete_thread_subscription(THREAD_ID.into())
        .await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}
