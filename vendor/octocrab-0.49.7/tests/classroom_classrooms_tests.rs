mod mock_error;

use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::ClassroomId;
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
async fn should_respond_to_list_classrooms() {
    let mocked_response: Vec<models::classroom::Classroom> =
        serde_json::from_str(include_str!("resources/classroom_list_classrooms.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_mock_http_server("GET", "/classrooms", template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client.classrooms().list_classrooms().await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.len(), 1);
    assert_eq!(response[0].name, "Programming Elixir");
    assert_eq!(response[0].archived, false);
    assert_eq!(
        response[0].url,
        "https://classroom.github.com/classrooms/1-programming-elixir"
    );
}

#[tokio::test]
async fn should_respond_to_get_classroom() {
    let classroom_id: ClassroomId = 1296269.into();
    let mocked_response: models::classroom::Classroom =
        serde_json::from_str(include_str!("resources/classroom_get_classroom.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_mock_http_server(
        "GET",
        format!("/classrooms/{classroom_id}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client.classrooms().get_classroom(classroom_id.into()).await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.name, "Programming Elixir");
    assert_eq!(response.archived, false);
    assert_eq!(
        response.url,
        "https://classroom.github.com/classrooms/1-programming-elixir"
    );
}

#[tokio::test]
async fn should_respond_to_list_assignments() {
    let classroom_id: ClassroomId = 1296269.into();
    let mocked_response: Vec<models::classroom::SimpleAssignment> =
        serde_json::from_str(include_str!("resources/classroom_list_assignments.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_mock_http_server(
        "GET",
        format!("/classrooms/{classroom_id}/assignments").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .classrooms()
        .list_assignments(classroom_id.into())
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.len(), 1);
    assert_eq!(response[0].title, "Intro to Binaries");
    assert_eq!(response[0].public_repo, false);
    assert_eq!(response[0].language, "ruby");
}
