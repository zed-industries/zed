mod mock_error;

use mock_error::setup_error_handler;
use octocrab::{params::repos::Reference, Octocrab};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

async fn setup_delete_ref_api(template: ResponseTemplate) -> MockServer {
    let owner: &str = "org";
    let repo: &str = "some-repo";
    let reference: &str = "heads/foo";

    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(format!("/repos/{owner}/{repo}/git/refs/{reference}")))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("DELETE on /repos/{owner}/{repo}/git/refs/{reference} was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

const OWNER: &str = "org";
const REPO: &str = "some-repo";
const BRANCH: &str = "foo";

#[tokio::test]
async fn should_delete_reference() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_delete_ref_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let repos = client.repos(OWNER.to_owned(), REPO.to_owned());

    let result = repos
        .delete_ref(&Reference::Branch(BRANCH.to_owned()))
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}
