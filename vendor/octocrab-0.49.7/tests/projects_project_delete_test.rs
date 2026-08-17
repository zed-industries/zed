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

#[expect(dead_code)]
#[derive(Serialize, Deserialize)]
struct FakeProject(Project);

async fn setup_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(format!("/projects/{PROJECT_ID}")))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("DELETE on /projects/{PROJECT_ID} was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_delete_project_204() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_api(template).await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client.projects().delete_project(PROJECT_ID).send().await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_delete_project_410() {
    let template = ResponseTemplate::new(410);
    let mock_server = setup_api(template).await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client.projects().delete_project(PROJECT_ID).send().await;

    assert!(
        result.is_err(),
        "expected error result, got success somehow: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_delete_project_500() {
    let template = ResponseTemplate::new(500);
    let mock_server = setup_api(template).await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client.projects().delete_project(PROJECT_ID).send().await;

    assert!(
        result.is_err(),
        "expected error result, got success somehow: {:#?}",
        result
    );
}
