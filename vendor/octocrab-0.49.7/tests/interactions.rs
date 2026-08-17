mod mock_error;

use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use models::interaction_limits::InteractionLimitType;
use octocrab::models::interaction_limits::InteractionLimitExpiry;
use octocrab::{models, Octocrab};

async fn setup_mock_http_server(
    http_method: &str,
    mocked_path: &str,
    template: &ResponseTemplate,
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
async fn should_respond_to_get_interaction_restrictions() {
    let org_id = "octocrab";
    let repo = "octocat";
    let mocked_response: models::interaction_limits::InteractionLimit =
        serde_json::from_str(include_str!("resources/interactions.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    //
    // for an organization
    //
    let mock_server = setup_mock_http_server(
        "GET",
        format!("/orgs/{org_id}/interaction-limits").as_str(),
        &template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());
    //
    let result = client.orgs(org_id).get_interaction_restrictions().await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.limit, InteractionLimitType::CollaboratorsOnly);
    assert_eq!(response.origin, "organization");
    //
    // for a repository
    //
    let mock_server = setup_mock_http_server(
        "GET",
        format!("/repos/{org_id}/{repo}/interaction-limits").as_str(),
        &template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());
    //
    let result = client
        .repos(org_id, repo)
        .get_interaction_restrictions()
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.limit, InteractionLimitType::CollaboratorsOnly);
    assert_eq!(response.origin, "organization");
    //
    // for a user
    //
    let mock_server = setup_mock_http_server("GET", "/user/interaction-limits", &template).await;
    let client = setup_octocrab(&mock_server.uri());
    //
    let result = client.current().get_interaction_restrictions().await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.limit, InteractionLimitType::CollaboratorsOnly);
    assert_eq!(response.origin, "organization");
}

#[tokio::test]
async fn should_respond_to_set_interaction_restrictions() {
    let org_id = "octocrab";
    let repo = "octocat";
    let mocked_response: models::interaction_limits::InteractionLimit =
        serde_json::from_str(include_str!("resources/interactions.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    //
    // for an organization
    //
    let mock_server = setup_mock_http_server(
        "PUT",
        format!("/orgs/{org_id}/interaction-limits").as_str(),
        &template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());
    //
    let result = client
        .orgs(org_id)
        .set_interaction_restrictions(
            InteractionLimitType::CollaboratorsOnly,
            InteractionLimitExpiry::OneWeek,
        )
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.limit, InteractionLimitType::CollaboratorsOnly);
    assert_eq!(response.origin, "organization");

    //
    // for a repository
    //
    let mock_server = setup_mock_http_server(
        "PUT",
        format!("/repos/{org_id}/{repo}/interaction-limits").as_str(),
        &template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());
    //
    let result = client
        .repos(org_id, repo)
        .set_interaction_restrictions(
            InteractionLimitType::CollaboratorsOnly,
            InteractionLimitExpiry::OneWeek,
        )
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.limit, InteractionLimitType::CollaboratorsOnly);
    assert_eq!(response.origin, "organization");
    //
    // for a user
    //
    let mock_server = setup_mock_http_server("PUT", "/user/interaction-limits", &template).await;
    let client = setup_octocrab(&mock_server.uri());
    //
    let result = client
        .current()
        .set_interaction_restrictions(
            InteractionLimitType::CollaboratorsOnly,
            InteractionLimitExpiry::OneWeek,
        )
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.limit, InteractionLimitType::CollaboratorsOnly);
    assert_eq!(response.origin, "organization");
}

#[tokio::test]
async fn should_respond_to_remove_interaction_restrictions() {
    let org_id = "octocrab";
    let repo = "octocat";
    let template = ResponseTemplate::new(204);
    //
    // for an organization
    //
    let mock_server = setup_mock_http_server(
        "DELETE",
        format!("/orgs/{org_id}/interaction-limits").as_str(),
        &template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());
    //
    let result = client.orgs(org_id).remove_interaction_restrictions().await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    //
    // for a repository
    //
    let mock_server = setup_mock_http_server(
        "DELETE",
        format!("/repos/{org_id}/{repo}/interaction-limits").as_str(),
        &template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());
    //
    let result = client
        .repos(org_id, repo)
        .remove_interaction_restrictions()
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    //
    // for a user
    //
    let mock_server = setup_mock_http_server("DELETE", "/user/interaction-limits", &template).await;
    let client = setup_octocrab(&mock_server.uri());
    //
    let result = client.current().remove_interaction_restrictions().await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}
