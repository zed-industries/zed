mod mock_error;

use mock_error::setup_error_handler;
use octocrab::Octocrab;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

async fn setup_put_api(template: ResponseTemplate) -> MockServer {
    let org: &str = "org";
    let secret_name: &str = "some_secret";
    let repository_id: u64 = 456;

    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path(format!(
            "/orgs/{org}/actions/secrets/{secret_name}/repositories/{repository_id}"
        )))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("PUT on /orgs/{org}/actions/secrets/{secret_name}/repositories/{repository_id} was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

const ORG: &str = "org";
const SECRET_NAME: &str = "some_secret";
const REPOSITORY_ID: u64 = 456;

#[tokio::test]
async fn should_204() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_put_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let actions = client.actions();

    let result = actions
        .add_selected_repo_to_org_secret(
            ORG.to_owned(),
            SECRET_NAME.to_owned(),
            REPOSITORY_ID.into(),
        )
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_500() {
    let template = ResponseTemplate::new(500);
    let mock_server = setup_put_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let actions = client.actions();

    let result = actions
        .add_selected_repo_to_org_secret(
            ORG.to_owned(),
            SECRET_NAME.to_owned(),
            REPOSITORY_ID.into(),
        )
        .await;

    assert!(
        result.is_err(),
        "expected error result, got success somehow: {:#?}",
        result
    );
}
