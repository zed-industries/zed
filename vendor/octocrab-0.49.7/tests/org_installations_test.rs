// Tests for calls to the /orgs/{org}/installation endpoint.
mod mock_error;

use mock_error::setup_error_handler;
use octocrab::models::{Author, Installation, InstallationId};
use octocrab::Octocrab;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

async fn setup_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/orgs/github/installation"))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "GET on /orgs/github/installation was not received",
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_return_org_installation() {
    let org_installation_response: Installation =
        serde_json::from_str(include_str!("resources/orgs_installation_event.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&org_installation_response);
    let mock_server = setup_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.apps().get_org_installation("github").await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let Installation {
        id: installation_id,
        account: Author { login, .. },
        ..
    } = result.unwrap();
    {
        assert_eq!(installation_id, InstallationId(1));
        assert_eq!(login, "github");
    }
}
