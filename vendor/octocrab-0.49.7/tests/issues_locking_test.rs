mod mock_error;

use mock_error::setup_error_handler;
use octocrab::{params::LockReason, Octocrab};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

async fn setup_issue_lock_api(template: ResponseTemplate) -> MockServer {
    let owner: &str = "org";
    let repo: &str = "some-repo";
    let issue_number: u64 = 123;

    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path(format!(
            "/repos/{owner}/{repo}/issues/{issue_number}/lock"
        )))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("PUT on /repos/{owner}/{repo}/issues/{issue_number}/lock was not received"),
    )
    .await;
    mock_server
}

async fn setup_issue_unlock_api(template: ResponseTemplate) -> MockServer {
    let owner: &str = "org";
    let repo: &str = "some-repo";
    let issue_number: u64 = 123;

    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(format!(
            "/repos/{owner}/{repo}/issues/{issue_number}/lock"
        )))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("DELETE on /repos/{owner}/{repo}/issues/{issue_number}/lock was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

const OWNER: &str = "org";
const REPO: &str = "some-repo";
const ISSUE_NUMBER: u64 = 123;

#[tokio::test]
async fn lock_no_reason_returns_true() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_issue_lock_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .issues(OWNER.to_owned(), REPO.to_owned())
        .lock(ISSUE_NUMBER, None)
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let result = result.unwrap();
    assert!(result, "expected the result to be true: {}", result);
}

#[tokio::test]
async fn lock_with_reason_returns_true() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_issue_lock_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .issues(OWNER.to_owned(), REPO.to_owned())
        .lock(ISSUE_NUMBER, Some(LockReason::TooHeated))
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let result = result.unwrap();
    assert!(result, "expected the result to be true: {}", result);
}

#[tokio::test]
async fn lock_no_reason_returns_false() {
    let template = ResponseTemplate::new(404);
    let mock_server = setup_issue_lock_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .issues(OWNER.to_owned(), REPO.to_owned())
        .lock(ISSUE_NUMBER, Some(LockReason::TooHeated))
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let result = result.unwrap();
    assert!(!result, "expected the result to be false: {}", result);
}

#[tokio::test]
async fn unlock_returns_true() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_issue_unlock_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .issues(OWNER.to_owned(), REPO.to_owned())
        .unlock(ISSUE_NUMBER)
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let result = result.unwrap();
    assert!(result, "expected the result to be true: {}", result);
}

#[tokio::test]
async fn lock_returns_false() {
    let template = ResponseTemplate::new(404);
    let mock_server = setup_issue_unlock_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .issues(OWNER.to_owned(), REPO.to_owned())
        .unlock(ISSUE_NUMBER)
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let result = result.unwrap();
    assert!(!result, "expected the result to be false: {}", result);
}
