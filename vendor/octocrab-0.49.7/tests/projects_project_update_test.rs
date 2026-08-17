// Tests for calls to the /projects/{project_id} API.
mod mock_error;

use mock_error::setup_error_handler;
use octocrab::{models::Project, Octocrab};
use serde::{Deserialize, Serialize};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

const PROJECT_ID: u32 = 1002605;

#[derive(Serialize, Deserialize)]
struct FakeProject(Project);

async fn setup_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path(format!("/projects/{PROJECT_ID}")))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("PATCH on /projects/{PROJECT_ID} was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_update_project_name() {
    let org_project: Project =
        serde_json::from_str(include_str!("resources/project.json")).unwrap();

    let test_name = org_project.name.clone();

    let page_response = FakeProject(org_project);

    let template = ResponseTemplate::new(200).set_body_json(&page_response);
    let mock_server = setup_api(template).await;

    let body = serde_json::json!({ "name": "Week One Sprint" });

    let client = setup_octocrab(&mock_server.uri());
    let project = client
        .projects()
        .update_project(PROJECT_ID)
        .body(&body)
        .send()
        .await
        .unwrap();

    assert_eq!(project.name, test_name);
}
