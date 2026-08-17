mod mock_error;

use mock_error::setup_error_handler;
use octocrab::{models::orgs_copilot::billing::CopilotBilling, Octocrab};
use serde::{Deserialize, Serialize};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[derive(Serialize, Deserialize)]
struct FakePage<T> {
    items: Vec<T>,
}

async fn setup_billing_api(template: ResponseTemplate) -> MockServer {
    let org = "org";
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/orgs/{org}/copilot/billing")))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on /orgs/{org}/copilot/billing was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

const ORG: &str = "org";

#[tokio::test]
async fn should_return_page_with_billing_info() {
    let billing: CopilotBilling =
        serde_json::from_str(include_str!("resources/org_copilot_billing.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&billing);
    let mock_server = setup_billing_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs(ORG.to_owned());
    let result = org.copilot().billing().await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    assert_eq!(result.unwrap().seat_breakdown.total, 12);
}

#[tokio::test]
async fn org_check_copilot_billing_401() {
    let template = ResponseTemplate::new(401);
    let mock_server = setup_billing_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs(ORG.to_owned());
    let result = org.copilot().metrics().await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn org_check_copilot_billing_403() {
    let template = ResponseTemplate::new(403);
    let mock_server = setup_billing_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs(ORG.to_owned());
    let result = org.copilot().metrics().await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn org_check_copilot_billing_404() {
    let template = ResponseTemplate::new(404);
    let mock_server = setup_billing_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs(ORG.to_owned());
    let result = org.copilot().metrics().await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn org_check_copilot_billing_422() {
    let template = ResponseTemplate::new(422);
    let mock_server = setup_billing_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs(ORG.to_owned());
    let result = org.copilot().metrics().await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn org_check_copilot_billing_500() {
    let template = ResponseTemplate::new(500);
    let mock_server = setup_billing_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs(ORG.to_owned());
    let result = org.copilot().metrics().await;

    assert!(
        result.is_err(),
        "expected error result, got success somehow: {:#?}",
        result
    );
}
