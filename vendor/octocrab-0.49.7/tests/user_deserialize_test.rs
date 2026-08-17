/// Tests API calls related to check runs of a specific commit.
mod mock_error;
use mock_error::setup_error_handler;
use octocrab::models::UserProfile;
use octocrab::Octocrab;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

async fn setup_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    let mocked_path = "/users/some-user";

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

#[tokio::test]
async fn should_return_deserialized_user() {
    let mocked_response: UserProfile =
        serde_json::from_str(include_str!("resources/user_data.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client.users("some-user").profile().await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let user = result.unwrap();

    {
        assert_eq!("octocat", user.login);
        assert_eq!(None, user.name.as_deref());
    }
}
