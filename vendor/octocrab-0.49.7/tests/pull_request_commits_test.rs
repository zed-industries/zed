use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::repos::RepoCommit;
use octocrab::Octocrab;

/// Unit test for calls to the `/repos/OWNER/REPO/contributors` endpoint
mod mock_error;

const OWNER: &str = "XAMPPRocky";
const REPO: &str = "octocrab";
const PULL_NUMBER: u64 = 42;

async fn setup_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/pulls/{PULL_NUMBER}/commits"
        )))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "GET on /repos/OWNER/REPO/pulls/{PULL_NUMBER}/commits not called",
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_return_pull_request_commits() {
    let pull_request_commits_response: Vec<RepoCommit> =
        serde_json::from_str(include_str!("resources/pull_request_commits.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&pull_request_commits_response);
    let mock_server = setup_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .pulls(OWNER, REPO)
        .pr_commits(PULL_NUMBER)
        .page(0u32)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let commits = result.unwrap();

    assert!(!commits.items.is_empty());
    assert!(commits.items.first().unwrap().author.is_some());
    assert!(commits.items.first().unwrap().committer.is_some());

    let RepoCommit { author, .. } = commits.items.first().unwrap();

    {
        assert_eq!(author.clone().unwrap().login, "octocat");
    }
}

#[tokio::test]
async fn should_return_pull_request_commits_empty_author_object() {
    let pull_request_commits_response: Vec<RepoCommit> = serde_json::from_str(include_str!(
        "resources/pull_request_commits_empty_author_object.json"
    ))
    .unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&pull_request_commits_response);
    let mock_server = setup_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .pulls(OWNER, REPO)
        .pr_commits(PULL_NUMBER)
        .page(0u32)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let mut commits = result.unwrap();
    let items = commits.take_items();

    assert!(items[11].author.is_some());
    assert!(items[12].author.is_none());
}
