/// Tests API calls related to check runs of a specific commit.
mod mock_error;

use mock_error::setup_error_handler;
use octocrab::models::hooks::Delivery;
use octocrab::models::HookId;
use octocrab::{Error, Octocrab};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[expect(dead_code)]
#[derive(Serialize, Deserialize)]
struct FakePage<T> {
    items: Vec<T>,
}

const OWNER: &str = "XAMPPRocky";

async fn setup_get_api(template: ResponseTemplate, number: u64) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/orgs/{OWNER}/hooks/{number}/deliveries")))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on /orgs/{OWNER}/hooks/{number}/deliveries was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_return_deliveries_for_org_by_id() {
    let number: u64 = 148681297;
    let mocked_response: Vec<Delivery> =
        serde_json::from_str(include_str!("resources/hooks_delivery_list.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_get_api(template, number).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .hooks(OWNER)
        .list_deliveries(HookId(number))
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let hooks = result.unwrap().items;
    assert_eq!(hooks.len(), 2);
}

#[tokio::test]
async fn should_fail_when_no_deliveries_found() {
    let mocked_response = json!({
        "documentation_url": json!("rtm"),
        "errors": Value::Null,
        "message": json!("Its gone")
    });

    let template = ResponseTemplate::new(404).set_body_json(&mocked_response);
    let mock_server = setup_get_api(template, 404).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .hooks(OWNER)
        .list_deliveries(HookId(404))
        .send()
        .await;

    match result.unwrap_err() {
        Error::GitHub { source, .. } => {
            assert_eq!("Its gone", source.message)
        }
        other => panic!("Unexpected error: {:?}", other),
    }
}
