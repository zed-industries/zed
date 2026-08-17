// Tests for calls to the /user/memberships/orgs API.
mod mock_error;

use mock_error::setup_error_handler;
use octocrab::{models::orgs::MembershipInvitation, Octocrab, Page};
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
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/user/memberships/orgs"))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "GET on /user/membership/orgs was not received",
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_return_page_with_invitations() {
    let membership_invitations: Vec<MembershipInvitation> =
        serde_json::from_str(include_str!("resources/user_membership_orgs_event.json")).unwrap();
    let page_response = FakePage {
        items: membership_invitations,
    };
    let template = ResponseTemplate::new(200).set_body_json(&page_response);
    let mock_server = setup_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let orgs = client
        .current()
        .list_org_memberships_for_authenticated_user();

    let result = orgs.send().await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let Page { items, .. } = result.unwrap();
    {
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].role, "admin");
        assert_eq!(items[0].user.login, "davidmhewitt");
        assert_eq!(items[0].organization.login, "elementary");
        assert_eq!(items[1].organization.login, "EpicGames");
    }
}
