use serde::{Deserialize, Serialize};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::code_scannings::CodeScanningAlert;
use octocrab::Octocrab;

mod mock_error;

async fn setup_issue_check_assignee_api(template: ResponseTemplate) -> MockServer {
    let owner: &str = "org";
    let repo: &str = "some-repo";
    let number: &str = "1";

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{owner}/{repo}/code-scanning/alerts/{number}"
        )))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{owner}/{repo}/code-scanning/alerts was not received"),
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
async fn check_get_200() {
    let s = include_str!("resources/codescanning_alert_single.json");
    let alert: CodeScanningAlert = serde_json::from_str(s).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&alert);
    let mock_server = setup_issue_check_assignee_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .code_scannings(OWNER.to_owned(), REPO.to_owned())
        .get(1)
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn check_get_404() {
    let s = include_str!("resources/codescanning_alert_error.json");
    let alert: GitHubErrorBody = serde_json::from_str(s).unwrap();
    let template = ResponseTemplate::new(404).set_body_json(&alert);
    let mock_server = setup_issue_check_assignee_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .code_scannings(OWNER.to_owned(), REPO.to_owned())
        .get(1)
        .await;

    assert!(
        result.is_err(),
        "expected error result, got success somehow: {:#?}",
        result
    );
}

#[tokio::test]
async fn check_get_500() {
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

#[derive(Debug, Deserialize, Serialize)]
struct GitHubErrorBody {
    pub documentation_url: Option<String>,
    pub message: Option<String>,
    pub status: Option<String>,
}
