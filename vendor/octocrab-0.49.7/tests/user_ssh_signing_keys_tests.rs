use http::StatusCode;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::SshSigningKey;
use octocrab::Octocrab;

/// Tests API calls related to check runs of a specific commit.
mod mock_error;

const SSH_SIGNING_KEY_ID: u64 = 42;

async fn setup_ssh_signing_keys_mock(
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
async fn should_respond_to_get_ssh_signing_key() {
    let mocked_response: SshSigningKey =
        serde_json::from_str(include_str!("resources/user_ssh_signing_key_created.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_ssh_signing_keys_mock(
        "GET",
        format!("/user/ssh_signing_keys/{SSH_SIGNING_KEY_ID}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .users("some_other_user")
        .ssh_signing_keys()
        .get(SSH_SIGNING_KEY_ID)
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    let id = response.id;
    assert_eq!(id, 2);
}

#[tokio::test]
async fn should_respond_to_ssh_signing_keys_list() {
    let mocked_response: Vec<SshSigningKey> =
        serde_json::from_str(include_str!("resources/user_ssh_signing_keys.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_ssh_signing_keys_mock("GET", "/user/ssh_signing_keys", template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .users("some_other_user")
        .ssh_signing_keys()
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
    let id = response.items.first().unwrap().id;
    assert_eq!(id, 2);
}

#[tokio::test]
async fn should_respond_to_ssh_signing_keys_add() {
    let mocked_response: SshSigningKey =
        serde_json::from_str(include_str!("resources/user_ssh_signing_key_created.json")).unwrap();
    let template = ResponseTemplate::new(StatusCode::CREATED).set_body_json(&mocked_response);
    let mock_server = setup_ssh_signing_keys_mock("POST", "/user/ssh_signing_keys", template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .users("some_user")
        .ssh_signing_keys()
        .add(
            "Assh-rsa AAAAB3NzaC1yc2EAA".to_string(),
            "A2Sg8iYjAxxmI2LvUXpJjkYrMxURPc8r+dB7TJyvv123".to_string(),
        )
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let result = result.unwrap();
    assert_eq!(result.id, 2);
}

#[tokio::test]
async fn should_respond_to_ssh_signing_key_delete() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_ssh_signing_keys_mock(
        "DELETE",
        format!("/user/ssh_signing_keys/{SSH_SIGNING_KEY_ID}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .users("some_user")
        .ssh_signing_keys()
        .delete(SSH_SIGNING_KEY_ID)
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}
