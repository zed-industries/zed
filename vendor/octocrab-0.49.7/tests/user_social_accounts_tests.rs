use http::StatusCode;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::SocialAccount;
use octocrab::Octocrab;

/// Tests API calls related to check runs of a specific commit.
mod mock_error;

async fn setup_social_accounts_mock(
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
async fn should_respond_to_social_accounts_list() {
    let mocked_response: Vec<SocialAccount> =
        serde_json::from_str(include_str!("resources/user_social_accounts.json")).unwrap();
    let template = ResponseTemplate::new(StatusCode::OK).set_body_json(&mocked_response);
    let mock_server = setup_social_accounts_mock("GET", "/user/social_accounts", template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .users("some_other_user")
        .social_accounts()
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
    let provider = &response.items.first().unwrap().provider;
    assert_eq!(provider, "twitter");
}

#[tokio::test]
async fn should_respond_to_social_accounts_add() {
    let mocked_response: Vec<SocialAccount> =
        serde_json::from_str(include_str!("resources/user_social_accounts.json")).unwrap();
    let template = ResponseTemplate::new(StatusCode::CREATED).set_body_json(&mocked_response);
    let mock_server = setup_social_accounts_mock("POST", "/user/social_accounts", template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .users("some_user")
        .social_accounts()
        .add(vec![
            "https://facebook.com/GitHub".to_string(),
            "https://www.youtube.com/@GitHub".to_string(),
        ])
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let result = result.unwrap();
    assert_eq!(result.first().unwrap().provider, "twitter");
}

#[tokio::test]
async fn should_respond_to_social_account_delete() {
    let template = ResponseTemplate::new(StatusCode::NO_CONTENT);
    let mock_server = setup_social_accounts_mock("DELETE", "/user/social_accounts", template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .users("some_user")
        .social_accounts()
        .delete(vec![
            "https://facebook.com/GitHub".to_string(),
            "https://www.youtube.com/@GitHub".to_string(),
        ])
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}
