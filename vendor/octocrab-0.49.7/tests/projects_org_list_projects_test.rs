// Tests for calls to the /orgs/{org}/projects API.
mod mock_error;

use mock_error::setup_error_handler;
use octocrab::{models::Project, Octocrab, Page};
use serde::{Deserialize, Serialize};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

const ORG: &str = "octocat";

#[derive(Serialize, Deserialize)]
struct FakePage<T> {
    items: Vec<T>,
}

async fn setup_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/orgs/{ORG}/projects")))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on /orgs/{ORG}/projects was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_list_org_projects() {
    let org_project: Vec<Project> =
        serde_json::from_str(include_str!("resources/projects.json")).unwrap();
    let owner = org_project[0].creator.login.clone();

    let page_response = FakePage { items: org_project };

    let template = ResponseTemplate::new(200).set_body_json(&page_response);
    let mock_server = setup_api(template).await;

    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .projects()
        .list_organization_projects(ORG)
        .state("all")
        .per_page(1)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let Page { items, .. } = result.unwrap();
    {
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].creator.login, owner);
    }
}
