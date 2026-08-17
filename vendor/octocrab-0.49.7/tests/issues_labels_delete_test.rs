mod mock_error;

use mock_error::setup_error_handler;
use octocrab::{models, Octocrab};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

async fn setup_remove_label_api(template: ResponseTemplate) -> MockServer {
    let owner: &str = "org";
    let repo: &str = "some-repo";
    let issue_number: u64 = 123;
    // Gotta love URL encoding
    let label_name: &str = "some%2Dlabel";

    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(format!(
            "/repos/{owner}/{repo}/issues/{issue_number}/labels/{label_name}"
        )))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("DELETE on /repos/{owner}/{repo}/issues/{issue_number}/labels/{label_name} was not received"),
    )
    .await;
    mock_server
}

async fn setup_delete_label_api(template: ResponseTemplate) -> MockServer {
    let owner: &str = "org";
    let repo: &str = "some-repo";
    let label_name: &str = "some%2Dlabel";

    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(format!("/repos/{owner}/{repo}/labels/{label_name}")))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("DELETE on /repos/{owner}/{repo}/labels/{label_name} was not received"),
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
const LABEL_NAME: &str = "some-label";

#[tokio::test]
async fn should_remove_label() {
    let remaining_issue_labels: Vec<models::Label> =
        serde_json::from_str(include_str!("resources/issues_remove_label.json")).unwrap();

    let template = ResponseTemplate::new(200).set_body_json(&remaining_issue_labels);
    let mock_server = setup_remove_label_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let issues = client.issues(OWNER.to_owned(), REPO.to_owned());

    let result = issues
        .remove_label(ISSUE_NUMBER, LABEL_NAME.to_owned())
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_delete_label() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_delete_label_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let issues = client.issues(OWNER.to_owned(), REPO.to_owned());

    let result = issues.delete_label(LABEL_NAME.to_owned()).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}
