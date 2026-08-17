// Tests for calls to the /repos/{owner}/{repo}/projects API.
mod mock_error;

use mock_error::setup_error_handler;
use octocrab::{models::Project, Octocrab};
use serde::{Deserialize, Serialize};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

const OWNER: &str = "octocat";
const REPO: &str = "repo";

#[derive(Serialize, Deserialize)]
struct FakeProject(Project);

async fn setup_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(format!("/repos/{OWNER}/{REPO}/projects")))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("POST on /repos/{OWNER}/{REPO}/projects was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_create_repo_project() {
    let repo_project: Project =
        serde_json::from_str(include_str!("resources/project.json")).unwrap();

    let test_name = repo_project.creator.login.clone();
    let test_description = repo_project.body.clone();

    let page_response = FakeProject(repo_project);

    let template = ResponseTemplate::new(200).set_body_json(&page_response);
    let mock_server = setup_api(template).await;

    let client = setup_octocrab(&mock_server.uri());

    let name = "Organization Roadmap";
    let description = "High-level roadmap for the upcoming year.";

    let result = client
        .projects()
        .create_repository_project(OWNER, REPO)
        .project_name(name)
        .body(description)
        .send()
        .await
        .unwrap();

    assert_eq!(result.creator.login, test_name);
    assert_eq!(result.body, test_description);
}
