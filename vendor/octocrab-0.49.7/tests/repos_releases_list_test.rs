/// Tests API calls related to check runs of a specific commit.
mod mock_error;

use mock_error::setup_error_handler;
use octocrab::models::repos::Release;
use octocrab::models::ReleaseId;
use octocrab::{Error, Octocrab};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[expect(dead_code)]
#[derive(Serialize, Deserialize)]
struct FakePage<T> {
    items: Vec<T>,
}

const OWNER: &str = "XAMPPRocky";
const REPO: &str = "octocrab";

async fn setup_get_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/releases")))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/releases was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_return_releases_for_repository() {
    let mocked_response: Vec<Release> =
        serde_json::from_str(include_str!("resources/repos_releases_list.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_get_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client.repos(OWNER, REPO).releases().list().send().await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let response = result.unwrap();
    let items = response.items;

    assert_eq!(items.len(), 2);

    {
        let item = &items[0];

        assert_eq!(ReleaseId(150372594), item.id);
        assert_eq!("v0.38.0", item.tag_name);
    }

    {
        let item = &items[1];

        assert_eq!(ReleaseId(148681297), item.id);
        assert_eq!("v0.37.0", item.tag_name);
    }
}

#[tokio::test]
async fn should_fail_when_no_releases_found() {
    let mocked_response = json!({
        "documentation_url": json!("rtm"),
        "errors": Value::Null,
        "message": json!("Its gone")
    });

    let template = ResponseTemplate::new(404).set_body_json(&mocked_response);
    let mock_server = setup_get_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client.repos(OWNER, REPO).releases().list().send().await;

    match result.unwrap_err() {
        Error::GitHub { source, .. } => {
            assert_eq!("Its gone", source.message)
        }
        other => panic!("Unexpected error: {:?}", other),
    }
}
