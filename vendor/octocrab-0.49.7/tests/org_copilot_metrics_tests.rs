mod mock_error;

use mock_error::setup_error_handler;
use octocrab::{models::orgs_copilot::metrics::CopilotMetrics, Octocrab};
use serde::{Deserialize, Serialize};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[derive(Serialize, Deserialize)]
struct FakePage<T> {
    items: Vec<T>,
}

async fn setup_metrics_api(template: ResponseTemplate, team_query: bool) -> MockServer {
    let org = "org";
    let team = "team";
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(if !team_query {
            format!("/orgs/{org}/copilot/metrics")
        } else {
            format!("/orgs/{org}/team/{team}/copilot/metrics")
        }))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on /orgs/{org}/copilot/metrics (team = {team_query}) was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

const ORG: &str = "org";

#[tokio::test]
async fn should_return_page_with_metrics() {
    let metrics: Vec<CopilotMetrics> =
        serde_json::from_str(include_str!("resources/org_copilot_metrics.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&metrics);
    let mock_server = setup_metrics_api(template, false).await;
    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs(ORG.to_owned());
    let result = org.copilot().metrics().await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    assert_eq!(metrics.len(), 1);
    let first_item = metrics.first().unwrap();
    assert_eq!(first_item.total_active_users, 24);
}

#[tokio::test]
async fn should_return_page_with_metrics_by_team() {
    let metrics: Vec<CopilotMetrics> =
        serde_json::from_str(include_str!("resources/org_copilot_metrics.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&metrics);
    let mock_server = setup_metrics_api(template, true).await;
    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs(ORG.to_owned());
    let result = org.copilot().metrics_team("team".to_string()).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    assert_eq!(metrics.len(), 1);
    let first_item = metrics.first().unwrap();
    assert_eq!(first_item.total_active_users, 24);
}

#[tokio::test]
async fn org_check_metrics_403() {
    let template = ResponseTemplate::new(403);
    let mock_server = setup_metrics_api(template, false).await;
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
async fn org_check_metrics_404() {
    let template = ResponseTemplate::new(404);
    let mock_server = setup_metrics_api(template, false).await;
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
async fn org_check_metrics_422() {
    let template = ResponseTemplate::new(422);
    let mock_server = setup_metrics_api(template, false).await;
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
async fn org_check_metrics_500() {
    let template = ResponseTemplate::new(500);
    let mock_server = setup_metrics_api(template, false).await;
    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs(ORG.to_owned());
    let result = org.copilot().metrics().await;

    assert!(
        result.is_err(),
        "expected error result, got success somehow: {:#?}",
        result
    );
}
