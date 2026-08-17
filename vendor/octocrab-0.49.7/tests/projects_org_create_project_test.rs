// Tests for calls to the /orgs/{org}/projects API.
mod mock_error;

use mock_error::setup_error_handler;
use octocrab::{models::Project, Octocrab};
use serde::{Deserialize, Serialize};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

const ORG: &str = "octocat";

#[derive(Serialize, Deserialize)]
struct FakeProject(Project);

async fn setup_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(format!("/orgs/{ORG}/projects")))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("POST on /orgs/{ORG}/projects was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_create_organization_project() {
    let org_project: Project =
        serde_json::from_str(include_str!("resources/project.json")).unwrap();

    let test_name = org_project.name.clone();
    let test_description = org_project.body.clone();

    let page_response = FakeProject(org_project);

    let template = ResponseTemplate::new(200).set_body_json(&page_response);
    let mock_server = setup_api(template).await;

    let name = "Organization Roadmap";
    let description = "High-level roadmap for the upcoming year.";

    let client = setup_octocrab(&mock_server.uri());

    let project = client
        .projects()
        .create_organization_project(ORG, name)
        .body(description)
        .send()
        .await
        .unwrap();

    assert_eq!(project.name, test_name);
    assert_eq!(project.body, test_description);
}
