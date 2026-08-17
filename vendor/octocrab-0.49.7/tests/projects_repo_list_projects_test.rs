// Tests for calls to the /repos/{owner}/{repo}/projects API.
mod mock_error;

use mock_error::setup_error_handler;
use octocrab::{models::Project, Octocrab, Page};
use serde::{Deserialize, Serialize};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

const OWNER: &str = "octocat";
const REPO: &str = "repo";

#[derive(Serialize, Deserialize)]
struct FakePage<T> {
    items: Vec<T>,
}

async fn setup_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/projects")))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/projects was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_list_repo_projects() {
    let repo_project: Vec<Project> =
        serde_json::from_str(include_str!("resources/projects.json")).unwrap();

    let test_login = repo_project[0].creator.login.clone();
    let test_id = repo_project[1].id;

    let page_response = FakePage {
        items: repo_project,
    };

    let template = ResponseTemplate::new(200).set_body_json(&page_response);
    let mock_server = setup_api(template).await;

    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .projects()
        .list_repository_projects(OWNER, REPO)
        .per_page(100)
        .send()
        .await
        .unwrap();

    let Page { items, .. } = result;
    {
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].creator.login, test_login);
        assert_eq!(items[1].id, test_id);
    }
}
