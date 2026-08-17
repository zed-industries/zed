mod mock_error;

use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::{models, Octocrab};

async fn setup_mock_http_server(
    http_method: &str,
    mocked_path: &str,
    template: ResponseTemplate,
) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method(http_method))
        .and(path(mocked_path))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("http method {http_method} on {mocked_path} was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_respond_to_get_assignment() {
    const ASSIGNMENT_ID: u64 = 42;
    let mocked_response: models::classroom::Assignment =
        serde_json::from_str(include_str!("resources/classroom_get_assignment.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_mock_http_server(
        "GET",
        format!("/assignments/{ASSIGNMENT_ID}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client.assignments().get(ASSIGNMENT_ID.into()).await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    let id = response.id;
    assert_eq!(id, ASSIGNMENT_ID.into());
    let full_name = response.starter_code_repository.full_name;
    assert_eq!(full_name, "octocat/Hello-World");
    let html_url = response.starter_code_repository.html_url;
    assert_eq!(html_url, "https://github.com/octocat/Hello-World");
    let node_id = response.starter_code_repository.node_id;
    assert_eq!(node_id, "MDEwOlJlcG9zaXRvcnkxMjk2MjY5");
    let private = response.starter_code_repository.private;
    assert_eq!(private, false);
    let default_branch = response.starter_code_repository.default_branch;
    assert_eq!(default_branch, "main");
}

#[tokio::test]
async fn should_respond_to_list_assignment() {
    const ASSIGNMENT_ID: u64 = 42;
    let mocked_response: Vec<models::classroom::AcceptedAssignment> =
        serde_json::from_str(include_str!("resources/classroom_list_accepted.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_mock_http_server(
        "GET",
        format!("/assignments/{ASSIGNMENT_ID}/accepted_assignments").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .assignments()
        .list_accepted(ASSIGNMENT_ID.into())
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    let assignment = response.first().unwrap();
    let id = assignment.id;
    assert_eq!(id, ASSIGNMENT_ID.into());
    let private = assignment.repository.private;
    assert_eq!(private, false);
}

#[tokio::test]
async fn should_respond_to_get_grades() {
    const ASSIGNMENT_ID: u64 = 42;
    let mocked_response: Vec<models::classroom::AssignmentGrade> =
        serde_json::from_str(include_str!("resources/classroom_get_grades.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_mock_http_server(
        "GET",
        format!("/assignments/{ASSIGNMENT_ID}/grades").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client.assignments().get_grades(ASSIGNMENT_ID.into()).await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    let assignment = response.first().unwrap();
    assert_eq!(assignment.assignment_name, "Introduction to Strings");
    assert_eq!(assignment.github_username, "octocat");
}
