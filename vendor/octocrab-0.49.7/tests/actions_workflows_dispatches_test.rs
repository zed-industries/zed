mod mock_error;

use mock_error::setup_error_handler;
use octocrab::Octocrab;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

async fn setup_post_api(template: ResponseTemplate) -> MockServer {
    let owner: &str = "org";
    let repo: &str = "some-repo";
    let workflow_id: &str = "workflow.yaml";

    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(format!(
            "/repos/{owner}/{repo}/actions/workflows/{workflow_id}/dispatches"
        )))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("POST on /repos/{owner}/{repo}/actions/workflows/{workflow_id}/dispatches was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

const OWNER: &str = "org";
const REPO: &str = "some-repo";
const WORKFLOW_ID: &str = "workflow.yaml";
const REF: &str = "ref";

#[tokio::test]
async fn should_be_ok_with_204() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_post_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .actions()
        .create_workflow_dispatch(
            OWNER.to_owned(),
            REPO.to_owned(),
            WORKFLOW_ID.to_owned(),
            REF.to_owned(),
        )
        .inputs(serde_json::json!({"foo":"bar"}))
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_be_err_with_500() {
    let template = ResponseTemplate::new(500);
    let mock_server = setup_post_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .actions()
        .create_workflow_dispatch(
            OWNER.to_owned(),
            REPO.to_owned(),
            WORKFLOW_ID.to_owned(),
            REF.to_owned(),
        )
        .inputs(serde_json::json!({"foo":"bar"}))
        .send()
        .await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}
