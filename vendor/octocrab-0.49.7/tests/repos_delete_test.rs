mod mock_error;

use mock_error::setup_error_handler;
use octocrab::Octocrab;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

async fn setup_delete_repo_api(template: ResponseTemplate) -> MockServer {
    let owner: &str = "org";
    let repo: &str = "some-repo";

    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(format!("/repos/{owner}/{repo}")))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("DELETE on /repos/{owner}/{repo} was not received"),
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
async fn should_delete_repo_204() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_delete_repo_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let repo = client.repos(OWNER.to_owned(), REPO.to_owned());
    let result = repo.delete().await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_delete_repo_404() {
    let template = ResponseTemplate::new(404);
    let mock_server = setup_delete_repo_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let repo = client.repos(OWNER.to_owned(), REPO.to_owned());
    let result = repo.delete().await;

    assert!(
        result.is_err(),
        "expected error result, got success somehow: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_delete_repo_500() {
    let template = ResponseTemplate::new(500);
    let mock_server = setup_delete_repo_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let repo = client.repos(OWNER.to_owned(), REPO.to_owned());
    let result = repo.delete().await;

    assert!(
        result.is_err(),
        "expected error result, got success somehow: {:#?}",
        result
    );
}
