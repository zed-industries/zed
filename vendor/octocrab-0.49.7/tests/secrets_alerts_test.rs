use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::repos::secret_scanning_alert::SecretScanningAlert;
use octocrab::models::repos::secret_scanning_alert::SecretsScanningAlertLocation;
use octocrab::Octocrab;

mod mock_error;

const OWNER: &str = "org";
const REPO: &str = "some-repo";
const ALERT_NUMBER: u32 = 5;

async fn setup_secrets_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/secret-scanning/alerts"
        )))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/secret-scanning/alerts was not received"),
    )
    .await;

    mock_server
}

async fn setup_secrets_locations_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/secret-scanning/alerts/5/locations"
        )))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!(
            "GET on /repos/{OWNER}/{REPO}/secret-scanning/alerts/{ALERT_NUMBER}/locations was not received"
        ),
    )
    .await;

    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn check_secrets_alert_list_200() {
    let s: &str = include_str!("resources/check_secrets_alerts.json");
    let alert: Vec<SecretScanningAlert> = serde_json::from_str(s).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&alert);
    let mock_server = setup_secrets_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER.to_owned(), REPO.to_owned())
        .secrets_scanning()
        .get_alerts()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:?}",
        result
    );

    let response = result.unwrap();
    let items = response.items;

    assert_eq!(items.len(), 2);

    {
        let item = &items[0];

        assert_eq!(2, item.number);
        assert_eq!(
            octocrab::models::repos::secret_scanning_alert::State::Resolved,
            item.state
        );
    }
}

#[tokio::test]
async fn check_secrets_alert_locations_list_200() {
    let s: &str = include_str!("resources/check_secrets_alerts_locations.json");
    let alert: Vec<SecretsScanningAlertLocation> = serde_json::from_str(s).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&alert);
    let mock_server = setup_secrets_locations_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER.to_owned(), REPO.to_owned())
        .secrets_scanning()
        .get_alert_locations(ALERT_NUMBER)
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:?}",
        result
    );

    let response = result.unwrap();
    let items = response.items;

    assert_eq!(items.len(), 13);
}
