mod mock_error;

use mock_error::setup_error_handler;
use octocrab::Octocrab;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

async fn setup_remove_comment_api(template: ResponseTemplate) -> MockServer {
    let owner: &str = "org";
    let repo: &str = "some-repo";
    let comment_id: u64 = 123;

    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(format!(
            "/repos/{owner}/{repo}/issues/comments/{comment_id}"
        )))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("DELETE on /repos/{owner}/{repo}/issues/comments/{comment_id} was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

const OWNER: &str = "org";
const REPO: &str = "some-repo";
const COMMENT_ID: u64 = 123;

#[tokio::test]
async fn should_delete_comment() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_remove_comment_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let issues = client.issues(OWNER.to_owned(), REPO.to_owned());

    let result = issues.delete_comment(COMMENT_ID.into()).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
// Quick negative test, just to make sure errors still report properly
async fn should_delete_comment_500() {
    let template = ResponseTemplate::new(500);
    let mock_server = setup_remove_comment_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let issues = client.issues(OWNER.to_owned(), REPO.to_owned());

    let result = issues.delete_comment(COMMENT_ID.into()).await;

    assert!(
        result.is_err(),
        "expected error result, got success somehow: {:#?}",
        result
    );
}
