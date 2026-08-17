use http::StatusCode;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::UserEmailInfo;
use octocrab::params::users::emails::EmailVisibilityState;
use octocrab::Octocrab;

/// Tests API calls related to check runs of a specific commit.
mod mock_error;

async fn setup_emails_mock(
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
async fn should_respond_to_primary_email_visibility() {
    let mocked_response: Vec<UserEmailInfo> =
        serde_json::from_str(include_str!("resources/user_emails.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_emails_mock("PATCH", "/user/email/visibility", template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .users("some_other_user")
        .set_primary_email_visibility(EmailVisibilityState::Private)
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    let visibility = response.first().unwrap().visibility;
    assert_eq!(visibility, Some(EmailVisibilityState::Private));
}

#[tokio::test]
async fn should_respond_to_email_list() {
    let mocked_response: Vec<UserEmailInfo> =
        serde_json::from_str(include_str!("resources/user_emails.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_emails_mock("GET", "/user/emails", template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .users("some_other_user")
        .emails()
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
    let visibility = response.items.first().unwrap().visibility;
    assert_eq!(visibility, Some(EmailVisibilityState::Private));

    let visibility = response.items[1].visibility;
    assert_eq!(visibility, None);
}

#[tokio::test]
async fn should_respond_to_public_email_list() {
    let mocked_response: Vec<UserEmailInfo> =
        serde_json::from_str(include_str!("resources/user_emails.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_emails_mock("GET", "/user/public_emails", template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .users("some_other_user")
        .emails()
        .per_page(42)
        .page(3u32)
        .list_public()
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    let visibility = response.items.first().unwrap().visibility;
    assert_eq!(visibility, Some(EmailVisibilityState::Private));
}

#[tokio::test]
async fn should_respond_to_emails_add() {
    let mocked_response: Vec<UserEmailInfo> =
        serde_json::from_str(include_str!("resources/user_emails.json")).unwrap();
    let template = ResponseTemplate::new(StatusCode::CREATED).set_body_json(&mocked_response);
    let mock_server = setup_emails_mock("POST", "/user/emails", template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .users("some_other_user")
        .emails()
        .add(vec!["newemail1@mail.com".to_string()])
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_respond_to_emails_delete() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_emails_mock("DELETE", "/user/emails", template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .users("some_other_user")
        .emails()
        .delete(vec!["newemail1@mail.com".to_string()])
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}
