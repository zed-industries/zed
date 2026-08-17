mod mock_error;

use mock_error::setup_error_handler;
use octocrab::Octocrab;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

async fn setup_remove_reaction_api(template: ResponseTemplate) -> MockServer {
    let owner: &str = "org";
    let repo: &str = "some-repo";
    let issue_number: u64 = 123;
    let reaction_id: u64 = 456;

    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(format!(
            "/repos/{owner}/{repo}/issues/{issue_number}/reactions/{reaction_id}"
        )))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("DELETE on /repos/{owner}/{repo}/issues/{issue_number}/reactions/{reaction_id} was not received"),
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
const REACTION_ID: u64 = 456;

#[tokio::test]
async fn should_delete_reaction() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_remove_reaction_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let issues = client.issues(OWNER.to_owned(), REPO.to_owned());

    let result = issues
        .delete_reaction(ISSUE_NUMBER.to_owned(), REACTION_ID.to_owned())
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}
