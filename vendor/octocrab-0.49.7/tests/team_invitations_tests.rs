// Tests for calls to the /orgs/{org}/teams/{team}/invitations API.
mod mock_error;

use mock_error::setup_error_handler;
use octocrab::{models::teams::TeamInvitation, Octocrab, Page};
use serde::{Deserialize, Serialize};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[derive(Serialize, Deserialize)]
struct FakePage<T> {
    items: Vec<T>,
}

async fn setup_api(template: ResponseTemplate) -> MockServer {
    let org = "org";
    let team = "team-name";
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/orgs/{org}/teams/{team}/invitations")))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on /orgs/{org}/teams/{team}/invitations was not received"),
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
async fn should_return_page_with_invitations() {
    let team_invitations: TeamInvitation =
        serde_json::from_str(include_str!("resources/team_invitations.json")).unwrap();
    let page_response = FakePage {
        items: vec![team_invitations],
    };
    let template = ResponseTemplate::new(200).set_body_json(&page_response);
    let mock_server = setup_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let teams = client.teams(ORG.to_owned());

    let result = teams.invitations(TEAM.to_owned()).send().await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let Page { items, .. } = result.unwrap();
    {
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].login.clone().unwrap(), String::from("monalisa"));
        assert_eq!(items[0].inviter.r#type, String::from("User"));
        assert_eq!(items[0].role, String::from("direct_member"));
    }
}
