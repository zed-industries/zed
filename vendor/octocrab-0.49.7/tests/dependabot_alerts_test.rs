use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::repos::dependabot::DependabotAlert;
use octocrab::Octocrab;

mod mock_error;

const OWNER: &str = "org";
const REPO: &str = "some-repo";

async fn setup_dependabot_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/dependabot/alerts")))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/dependabot/alerts was not received"),
    )
    .await;

    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn check_dependabot_alerts_list_200() {
    let s = include_str!("resources/check_dependabot_alerts.json");
    let alert: Vec<DependabotAlert> = serde_json::from_str(s).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&alert);
    let mock_server = setup_dependabot_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER.to_owned(), REPO.to_owned())
        .dependabot()
        .get_alerts()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:?}",
        result
    );

    let response = result.unwrap();
    let items = response.items;

    assert_eq!(items.len(), 5);

    {
        let item = &items[0];

        assert_eq!(5, item.number);
        assert_eq!(octocrab::models::repos::dependabot::State::Open, item.state);
    }
}
