mod mock_error;

use mock_error::setup_error_handler;
use octocrab::Octocrab;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

async fn setup_issue_check_assignee_api(template: ResponseTemplate) -> MockServer {
    let owner: &str = "org";
    let repo: &str = "some-repo";
    let assignee: &str = "some-user";

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/repos/{owner}/{repo}/assignees/{assignee}")))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{owner}/{repo}/assignees/{assignee} was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

const OWNER: &str = "org";
const REPO: &str = "some-repo";
const ASSIGNEE: &str = "some-user";

#[tokio::test]
async fn check_assignee_204() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_issue_check_assignee_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .issues(OWNER.to_owned(), REPO.to_owned())
        .check_assignee(ASSIGNEE)
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
async fn check_assignee_404() {
    let template = ResponseTemplate::new(404);
    let mock_server = setup_issue_check_assignee_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .issues(OWNER.to_owned(), REPO.to_owned())
        .check_assignee(ASSIGNEE)
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
async fn check_assignee_500() {
    let template = ResponseTemplate::new(500);
    let mock_server = setup_issue_check_assignee_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .issues(OWNER.to_owned(), REPO.to_owned())
        .check_assignee(ASSIGNEE)
        .await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}
