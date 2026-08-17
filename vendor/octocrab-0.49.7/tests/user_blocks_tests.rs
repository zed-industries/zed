use serde::{Deserialize, Serialize};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::SimpleUser;
use octocrab::Octocrab;

/// Tests API calls related to check runs of a specific commit.
mod mock_error;

#[expect(dead_code)]
#[derive(Serialize, Deserialize)]
struct FakePage<T> {
    items: Vec<T>,
}

const NOT_BLOCKED: &str = "XAMPPRocky";

async fn setup_blocks_mock(
    http_method: &str,
    mocked_path: &str,
    template: ResponseTemplate,
) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method(http_method))
        .and(path(mocked_path))
        .respond_with(template.clone())
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
async fn should_return_list_of_blocked_by_user() {
    let mocked_response: Vec<SimpleUser> =
        serde_json::from_str(include_str!("resources/user_blocks.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_blocks_mock("GET", "/user/blocks", template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client.users("some-user").blocks().per_page(10).list().await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let response = result.unwrap();
    let items = response.items;

    assert_eq!(items.len(), 1);

    {
        let item = &items[0];

        assert_eq!("octocat", item.login);
        assert_eq!(
            "https://api.github.com/users/octocat/received_events",
            item.received_events_url.as_str()
        );
    }
}

#[tokio::test]
async fn should_check_if_user_blocked() {
    /* status 204 for blocked */
    let template = ResponseTemplate::new(200);
    let mock_server = setup_blocks_mock(
        "GET",
        format!("/user/blocks/{NOT_BLOCKED}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client.users("some-user").is_blocked(NOT_BLOCKED).await;
    assert!(!result.is_ok_and(|is_blocked| is_blocked));
}

#[tokio::test]
async fn should_respond_user_blocked() {
    /* status 204 for blocked */
    let template = ResponseTemplate::new(204);
    let mock_server = setup_blocks_mock(
        "PUT",
        format!("/user/blocks/{NOT_BLOCKED}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client.users("some-user").block_user(NOT_BLOCKED).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn should_respond_user_unblocked() {
    /* status 204 for unblocked */
    let template = ResponseTemplate::new(200);
    let mock_server = setup_blocks_mock(
        "DELETE",
        format!("/user/blocks/{NOT_BLOCKED}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client.users("some-user").unblock_user(NOT_BLOCKED).await;
    assert!(result.is_err());
}
