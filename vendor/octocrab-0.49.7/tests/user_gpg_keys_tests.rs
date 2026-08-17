use http::StatusCode;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::GpgKey;
use octocrab::Octocrab;

/// Tests API calls related to check runs of a specific commit.
mod mock_error;

const GPG_KEY_ID: u64 = 42;

async fn setup_gpg_keys_mock(
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
        &format!("http method {http_method} on {mocked_path} was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_respond_to_get_gpg_key() {
    let mocked_response: GpgKey =
        serde_json::from_str(include_str!("resources/user_gpg_key_created.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_gpg_keys_mock(
        "GET",
        format!("/user/gpg_keys/{GPG_KEY_ID}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .users("some_other_user")
        .gpg_keys()
        .get(GPG_KEY_ID)
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    let name = response.name;
    assert_eq!(name, "Octocat's GPG Key");
}

#[tokio::test]
async fn should_respond_to_gpg_keys_list() {
    let mocked_response: Vec<GpgKey> =
        serde_json::from_str(include_str!("resources/user_gpg_keys.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_gpg_keys_mock("GET", "/user/gpg_keys", template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .users("some_other_user")
        .gpg_keys()
        .per_page(42)
        .page(3u32)
        .list()
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    let name = &response.items.first().unwrap().name;
    assert_eq!(name, "Octocat's GPG Key");
}

#[tokio::test]
async fn should_respond_to_gpg_keys_add() {
    let mocked_response: GpgKey =
        serde_json::from_str(include_str!("resources/user_gpg_key_created.json")).unwrap();
    let template = ResponseTemplate::new(StatusCode::CREATED).set_body_json(&mocked_response);
    let mock_server = setup_gpg_keys_mock("POST", "/user/gpg_keys", template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .users("some_user")
        .gpg_keys()
        .add(
            "A descriptive name for the new key".to_string(),
            "A GPG key in ASCII-armored format".to_string(),
        )
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let result = result.unwrap();
    assert_eq!(result.name, "Octocat's GPG Key");
}

#[tokio::test]
async fn should_respond_to_gpg_key_delete() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_gpg_keys_mock(
        "DELETE",
        format!("/user/gpg_keys/{GPG_KEY_ID}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .users("some_user")
        .gpg_keys()
        .delete(GPG_KEY_ID)
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}
