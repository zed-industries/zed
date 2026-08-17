use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::code_scannings::CodeScanningAlert;
use octocrab::params::AlertState;
use octocrab::Octocrab;

mod mock_error;

async fn setup_issue_check_assignee_api(template: ResponseTemplate) -> MockServer {
    let owner: &str = "org";
    let repo: &str = "some-repo";
    let number: &str = "1";

    let mock_server = MockServer::start().await;

    Mock::given(method("PATCH"))
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

#[tokio::test]
async fn check_patch_200() {
    let s = include_str!("resources/codescanning_alert_single.json");
    let alert: CodeScanningAlert = serde_json::from_str(s).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&alert);
    let mock_server = setup_issue_check_assignee_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .code_scannings(OWNER.to_owned(), REPO.to_owned())
        .update(1)
        .state(AlertState::Open)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}
