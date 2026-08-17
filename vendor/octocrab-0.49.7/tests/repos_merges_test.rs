mod mock_error;

use mock_error::setup_error_handler;
use octocrab::{models::repos::MergeCommit, Octocrab};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

async fn setup_repos_merges_api(template: ResponseTemplate) -> MockServer {
    let owner: &str = "org";
    let repo: &str = "some-repo";

    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(format!("/repos/{owner}/{repo}/merges")))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("POST on /repos/{owner}/{repo}/merges was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

const OWNER: &str = "org";
const REPO: &str = "some-repo";
const BRANCH_HEAD: &str = "head";
const BRANCH_BASE: &str = "base";
const COMMIT_MESSAGE: &str = "message here";

#[tokio::test]
async fn test_merges_returns_204() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_repos_merges_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER.to_owned(), REPO.to_owned())
        .merge(BRANCH_HEAD.to_owned(), BRANCH_BASE.to_owned())
        .commit_message(COMMIT_MESSAGE.to_owned())
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let result = result.unwrap();

    assert!(
        result.is_none(),
        "expected None() value, got Some(): {:#?}",
        result
    );
}

#[tokio::test]
async fn test_merges_returns_201() {
    let repo_merges_json: MergeCommit =
        serde_json::from_str(include_str!("resources/repos_merges_201.json")).unwrap();
    let template = ResponseTemplate::new(201).set_body_json(&repo_merges_json);
    let mock_server = setup_repos_merges_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER.to_owned(), REPO.to_owned())
        .merge(BRANCH_HEAD.to_owned(), BRANCH_BASE.to_owned())
        .commit_message(COMMIT_MESSAGE.to_owned())
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let result = result.unwrap();

    assert!(
        result.is_some(),
        "expected Some() value, got None: {:#?}",
        result
    );

    assert_eq!(
        result.unwrap().sha,
        String::from("6dcb09b5b57875f334f61aebed695e2e4193db5e"),
        "Unable to verify SHA from fixture data."
    );
}
