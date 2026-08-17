/// Tests generating release notes:
/// /repos/{owner}/{repo}/releases/generate-notes
mod mock_error;
use mock_error::setup_error_handler;
use octocrab::models::repos::ReleaseNotes;
use octocrab::Octocrab;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

async fn setup_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    let mocked_path = "/repos/owner/repo/releases/generate-notes";

    Mock::given(method("POST"))
        .and(path(mocked_path))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("POST on {mocked_path} was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_return_page_with_check_runs() {
    let owner = "owner";
    let repo = "repo";
    let tag_name = "2.0.0";
    let mocked_response: ReleaseNotes =
        serde_json::from_str(include_str!("resources/generate_release_notes.json")).unwrap();

    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .repos(owner, repo)
        .releases()
        .generate_release_notes(tag_name)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let response = result.unwrap();
    assert_eq!(response.name, tag_name);
    assert!(!response.body.is_empty());
}
