use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::code_scannings::CodeScanningAlert;
use octocrab::Octocrab;

mod mock_error;

const OWNER: &str = "org";
const REPO: &str = "some-repo";

async fn setup_codescanning_list_api(template: ResponseTemplate, is_org: bool) -> MockServer {
    let mock_server = MockServer::start().await;

    if is_org {
        Mock::given(method("GET"))
            .and(path(format!("/orgs/{OWNER}/code-scanning/alerts")))
            .respond_with(template.clone())
            .mount(&mock_server)
            .await;
        setup_error_handler(
            &mock_server,
            &format!("GET on /org/{OWNER}/code-scanning/alerts was not received"),
        )
        .await;
    } else {
        Mock::given(method("GET"))
            .and(path(format!("/repos/{OWNER}/{REPO}/code-scanning/alerts")))
            .respond_with(template.clone())
            .mount(&mock_server)
            .await;
        setup_error_handler(
            &mock_server,
            &format!("GET on /repos/{OWNER}/{REPO}/code-scanning/alerts was not received"),
        )
        .await;
    }
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn check_list_200() {
    let s = include_str!("resources/codescanning_alerts_multiple.json");
    let alert: Vec<CodeScanningAlert> = serde_json::from_str(s).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&alert);
    let mock_server = setup_codescanning_list_api(template, false).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .code_scannings(OWNER.to_owned(), REPO.to_owned())
        .list()
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:?}",
        result
    );
}

#[tokio::test]
async fn check_list_organisation_200() {
    let s = include_str!("resources/codescanning_alerts_multiple.json");
    let alert: Vec<CodeScanningAlert> = serde_json::from_str(s).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&alert);
    let mock_server = setup_codescanning_list_api(template, true).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .code_scannings_organisation(OWNER.to_owned())
        .list()
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:?}",
        result
    );
}
