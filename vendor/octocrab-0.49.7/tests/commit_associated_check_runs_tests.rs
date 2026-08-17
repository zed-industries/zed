/// Tests API calls related to check runs of a specific commit.
mod mock_error;

use mock_error::setup_error_handler;
use octocrab::models::checks::ListCheckRuns;
use octocrab::models::CheckRunId;
use octocrab::params::repos::Reference;
use octocrab::{Error, Octocrab};
use serde_json::{json, Value};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

async fn setup_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    let mocked_path = "/repos/some-owner/some-repo/commits/refs/heads/some-branch/check-runs";

    Mock::given(method("GET"))
        .and(path(mocked_path))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on {mocked_path} was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_return_page_with_check_runs() {
    let mocked_response: ListCheckRuns =
        serde_json::from_str(include_str!("resources/commit_check_runs.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .commits("some-owner", "some-repo")
        .associated_check_runs(Reference::Branch("some-branch".into()))
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let response = result.unwrap();
    assert_eq!(response.total_count, 2);

    let items = response.check_runs;
    assert_eq!(items.len(), 2);

    {
        let item = &items[0];

        assert_eq!(CheckRunId(16354767716), item.id);
        assert_eq!("Cargo test on nix (ubuntu-20.04, stable)", item.name);
        assert_eq!(Some("success".into()), item.conclusion);
    }

    {
        let item = &items[1];

        assert_eq!(CheckRunId(16354767496), item.id);
        assert_eq!("Cargo test on nix (ubuntu-20.04, 1.68)", item.name);
        assert_eq!(Some("success".into()), item.conclusion);
    }
}

#[tokio::test]
async fn should_fail_when_not_found() {
    let mocked_response = json!({
        "documentation_url": json!("rtm"),
        "errors": Value::Null,
        "message": json!("Its gone")
    });

    let template = ResponseTemplate::new(404).set_body_json(&mocked_response);
    let mock_server = setup_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .commits("some-owner", "some-repo")
        .associated_check_runs(Reference::Branch("some-branch".into()))
        .send()
        .await;

    match result.unwrap_err() {
        Error::GitHub { source, .. } => {
            assert_eq!(http::StatusCode::NOT_FOUND, source.status_code);
            assert_eq!("Its gone", source.message);
        }
        other => panic!("Unexpected error: {:?}", other),
    }
}
