// Tests for calls to the /orgs/{org}/teams/{team}/members API.
mod mock_error;

use mock_error::setup_error_handler;
use octocrab::Octocrab;
use serde::{Deserialize, Serialize};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[expect(dead_code)]
#[derive(Serialize, Deserialize)]
struct FakePage<T> {
    items: Vec<T>,
}

async fn setup_api(template: ResponseTemplate) -> MockServer {
    let org = "org";
    let team = "team-name";
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(format!("/orgs/{org}/teams/{team}")))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("DELETE on /orgs/{org}/teams/{team} was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

const ORG: &str = "org";
const TEAM: &str = "team-name";

#[tokio::test]
async fn should_delete_team() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let teams = client.teams(ORG.to_owned());

    let result = teams.delete(TEAM.to_owned()).await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}
