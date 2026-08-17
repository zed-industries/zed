mod mock_error;

use mock_error::setup_error_handler;
use octocrab::{
    models::orgs_copilot::billing::{SeatsCancelled, SeatsCreated},
    Octocrab,
};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

const ORG: &str = "org";

async fn setup_billing_api(template: ResponseTemplate, teams: bool, delete: bool) -> MockServer {
    let mock_server = MockServer::start().await;

    let route = format!(
        "/orgs/{ORG}/copilot/billing/selected_{}",
        if teams { "teams" } else { "users" }
    );
    let meth = if delete { "DELETE" } else { "POST" };

    Mock::given(method(meth))
        .and(path(&route))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(&mock_server, &format!("{meth} on {route} was not received")).await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_create_seats_team() {
    let billing: SeatsCreated = serde_json::from_str(include_str!(
        "resources/org_copilot_billing_seats_created.json"
    ))
    .unwrap();

    let template = ResponseTemplate::new(201).set_body_json(&billing);
    let mock_server = setup_billing_api(template, true, false).await;
    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs(ORG.to_owned());
    let result = org
        .copilot()
        .manage_seats()
        .add_teams(vec!["testteam".to_string()])
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    assert_eq!(result.unwrap().seats_created, 5);
}

#[tokio::test]
async fn should_create_seats_user() {
    let billing: SeatsCreated = serde_json::from_str(include_str!(
        "resources/org_copilot_billing_seats_created.json"
    ))
    .unwrap();

    let template = ResponseTemplate::new(201).set_body_json(&billing);
    let mock_server = setup_billing_api(template, false, false).await;
    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs(ORG.to_owned());
    let result = org
        .copilot()
        .manage_seats()
        .add_usernames(vec!["testuser".to_string()])
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    assert_eq!(result.unwrap().seats_created, 5);
}

#[tokio::test]
async fn should_remove_seats_team() {
    let billing: SeatsCancelled = serde_json::from_str(include_str!(
        "resources/org_copilot_billing_seats_cancelled.json"
    ))
    .unwrap();

    let template = ResponseTemplate::new(200).set_body_json(&billing);
    let mock_server = setup_billing_api(template, true, true).await;
    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs(ORG.to_owned());
    let result = org
        .copilot()
        .manage_seats()
        .remove_teams(vec!["testteam".to_string()])
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    assert_eq!(result.unwrap().seats_cancelled, 5);
}

#[tokio::test]
async fn should_remove_seats_user() {
    let billing: SeatsCancelled = serde_json::from_str(include_str!(
        "resources/org_copilot_billing_seats_cancelled.json"
    ))
    .unwrap();

    let template = ResponseTemplate::new(200).set_body_json(&billing);
    let mock_server = setup_billing_api(template, false, true).await;
    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs(ORG.to_owned());
    let result = org
        .copilot()
        .manage_seats()
        .remove_usernames(vec!["testuser".to_string()])
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    assert_eq!(result.unwrap().seats_cancelled, 5);
}

#[tokio::test]
async fn org_check_copilot_seats_401() {
    let template = ResponseTemplate::new(401);
    let mock_server = setup_billing_api(template, false, false).await;
    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs(ORG.to_owned());
    let result = org
        .copilot()
        .manage_seats()
        .add_teams(vec!["foo".to_string()])
        .await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn org_check_copilot_seats_403() {
    let template = ResponseTemplate::new(403);
    let mock_server = setup_billing_api(template, false, false).await;
    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs(ORG.to_owned());
    let result = org
        .copilot()
        .manage_seats()
        .add_teams(vec!["foo".to_string()])
        .await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn org_check_copilot_seats_404() {
    let template = ResponseTemplate::new(404);
    let mock_server = setup_billing_api(template, false, false).await;
    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs(ORG.to_owned());
    let result = org
        .copilot()
        .manage_seats()
        .add_teams(vec!["foo".to_string()])
        .await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn org_check_copilot_seats_422() {
    let template = ResponseTemplate::new(422);
    let mock_server = setup_billing_api(template, false, false).await;
    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs(ORG.to_owned());
    let result = org
        .copilot()
        .manage_seats()
        .add_teams(vec!["foo".to_string()])
        .await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn org_check_copilot_seats_500() {
    let template = ResponseTemplate::new(500);
    let mock_server = setup_billing_api(template, false, false).await;
    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs(ORG.to_owned());
    let result = org
        .copilot()
        .manage_seats()
        .add_teams(vec!["foo".to_string()])
        .await;

    assert!(
        result.is_err(),
        "expected error result, got success somehow: {:#?}",
        result
    );
}
