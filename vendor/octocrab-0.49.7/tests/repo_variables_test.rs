// Tests for calls to the /repos/{owner}/{repo}/actions/variables API.
mod mock_error;

use chrono::DateTime;
use mock_error::setup_error_handler;
use octocrab::{
    models::repos::{RepoVariable, RepoVariables},
    Octocrab,
};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

const OWNER: &str = "owner";
const REPO: &str = "repo";

async fn setup_get_api(template: ResponseTemplate, variables_path: &str) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/actions/variables{variables_path}"
        )))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/actions/variables{variables_path} was not received"),
    )
    .await;

    mock_server
}

async fn setup_post_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(format!("/repos/{OWNER}/{REPO}/actions/variables")))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("POST on /repos/{OWNER}/{REPO}/actions/variables was not received"),
    )
    .await;

    mock_server
}

async fn setup_patch_api(template: ResponseTemplate, variables_path: &str) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/actions/variables{variables_path}"
        )))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!(
            "PATCH on /repos/{OWNER}/{REPO}/actions/variables{variables_path} was not received"
        ),
    )
    .await;

    mock_server
}

async fn setup_delete_api(template: ResponseTemplate, variables_path: &str) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/actions/variables{variables_path}"
        )))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!(
            "DELETE on /repos/{OWNER}/{REPO}/actions/variables{variables_path} was not received"
        ),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_return_repo_variables() {
    let repo_variables: RepoVariables =
        serde_json::from_str(include_str!("resources/repo_variables.json")).unwrap();

    let template = ResponseTemplate::new(200).set_body_json(&repo_variables);
    let mock_server = setup_get_api(template, "").await;
    let result = setup_octocrab(&mock_server.uri())
        .repos(OWNER.to_owned(), REPO.to_owned())
        .variables()
        .list()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let item = result.unwrap();

    assert_eq!(item.total_count, 2);
    assert_eq!(
        item.variables,
        vec![
            RepoVariable {
                name: String::from("USERNAME"),
                value: String::from("octocat"),
                created_at: DateTime::parse_from_rfc3339("2019-08-10T14:59:22Z")
                    .unwrap()
                    .into(),
                updated_at: DateTime::parse_from_rfc3339("2020-01-10T14:59:22Z")
                    .unwrap()
                    .into(),
            },
            RepoVariable {
                name: String::from("EMAIL"),
                value: String::from("octocat@github.com"),
                created_at: DateTime::parse_from_rfc3339("2020-01-10T10:59:22Z")
                    .unwrap()
                    .into(),
                updated_at: DateTime::parse_from_rfc3339("2020-01-11T11:59:22Z")
                    .unwrap()
                    .into(),
            },
        ]
    );
}

#[tokio::test]
async fn should_return_repo_variable() {
    let repo_variables: RepoVariable =
        serde_json::from_str(include_str!("resources/repo_variable.json")).unwrap();

    let template = ResponseTemplate::new(200).set_body_json(&repo_variables);
    let mock_server = setup_get_api(template, "/USERNAME").await;
    let result = setup_octocrab(&mock_server.uri())
        .repos(OWNER.to_owned(), REPO.to_owned())
        .variables()
        .get("USERNAME")
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let item = result.unwrap();
    assert_eq!(
        item,
        RepoVariable {
            name: String::from("USERNAME"),
            value: String::from("octocat"),
            created_at: DateTime::parse_from_rfc3339("2021-08-10T14:59:22Z")
                .unwrap()
                .into(),
            updated_at: DateTime::parse_from_rfc3339("2022-01-10T14:59:22Z")
                .unwrap()
                .into(),
        }
    );
}

#[tokio::test]
async fn should_add_variable() {
    let template = ResponseTemplate::new(201);
    let mock_server = setup_post_api(template).await;
    let result = setup_octocrab(&mock_server.uri())
        .repos(OWNER.to_owned(), REPO.to_owned())
        .variables()
        .create("USERNAME", "octocat")
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_update_variable_204() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_patch_api(template, "/USERNAME").await;
    let result = setup_octocrab(&mock_server.uri())
        .repos(OWNER.to_owned(), REPO.to_owned())
        .variables()
        .update("USERNAME", "octocat")
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_delete_variable() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_delete_api(template, "/USERNAME").await;
    let result = setup_octocrab(&mock_server.uri())
        .repos(OWNER.to_owned(), REPO.to_owned())
        .variables()
        .delete("USERNAME")
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_noop_variable_500() {
    let template = ResponseTemplate::new(500);
    let mock_server = setup_delete_api(template, "/GH_TOKEN").await;
    let result = setup_octocrab(&mock_server.uri())
        .repos(OWNER.to_owned(), REPO.to_owned())
        .variables()
        .delete("GH_TOKEN")
        .await;

    assert!(
        result.is_err(),
        "expected error result, got success somehow: {:#?}",
        result
    );
}
