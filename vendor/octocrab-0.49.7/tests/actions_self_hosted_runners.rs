// Tests for calls to the actions self-hosted runners API:
// - /repos/{owner}/{repo}/actions/runners
// - /orgs/{org}/actions/runners
mod mock_error;

use http::StatusCode;
use mock_error::setup_error_handler;
use octocrab::{
    models::{
        actions::{SelfHostedRunner, SelfHostedRunnerJitConfig, SelfHostedRunnerToken},
        RunnerGroupId,
    },
    Octocrab, Page,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use wiremock::{
    matchers::{body_json, method, path},
    Mock, MockServer, ResponseTemplate,
};

const OWNER: &str = "owner";
const REPO: &str = "repo";
const ORG: &str = "org";

enum Scope {
    Repo,
    Org,
}

#[derive(Clone, Serialize, Deserialize)]
struct FakePage {
    total_count: u64,
    runners: Vec<SelfHostedRunner>,
}

struct TestContext {
    _server: MockServer,
    client: Octocrab,
}

#[derive(Clone, Serialize)]
struct JitConfigBody {
    name: String,
    runner_group_id: RunnerGroupId,
    labels: Vec<String>,
}

impl JitConfigBody {
    fn expected_response(&self) -> SelfHostedRunnerJitConfig {
        let json = json!({
            "runner": {
                "id": 23,
                "runner_group_id": self.runner_group_id,
                "name": self.name.clone(),
                "os": "unknown",
                "status": "offline",
                "busy": false,
                "labels": self.labels.iter().enumerate().map(|(index, label)| json!({
                    "id": index,
                    "name": label,
                    "type": "custom"
                })).collect::<Vec<_>>()
            },
            "encoded_jit_config": "abc123"
        });

        serde_json::from_value(json).unwrap()
    }
}

impl JitConfigBody {}

async fn setup_api(
    scope: Scope,
    method_name: &str,
    sub_uri: &str,
    request: Option<impl Serialize>,
    response: ResponseTemplate,
) -> MockServer {
    let mock_server = MockServer::start().await;
    let mut uri = match scope {
        Scope::Repo => format!("/repos/{OWNER}/{REPO}"),
        Scope::Org => format!("/orgs/{ORG}"),
    };
    uri.push_str(sub_uri);

    let mut builder = Mock::given(method(method_name)).and(path(&uri));
    if let Some(request) = request {
        builder = builder.and(body_json(request));
    }
    builder.respond_with(response).mount(&mock_server).await;

    setup_error_handler(&mock_server, &format!("GET on {uri} was not received")).await;
    mock_server
}

async fn test_context(
    scope: Scope,
    method_name: &str,
    actions_uri: &str,
    response_code: StatusCode,
    resp_body: Option<impl Serialize>,
) -> TestContext {
    let template = match resp_body {
        Some(resp_body) => ResponseTemplate::new(response_code.as_u16()).set_body_json(resp_body),
        None => ResponseTemplate::new(response_code.as_u16()),
    };
    let server = setup_api(scope, method_name, actions_uri, None::<()>, template).await;
    let client = Octocrab::builder()
        .base_uri(server.uri())
        .unwrap()
        .build()
        .unwrap();

    TestContext {
        _server: server,
        client,
    }
}

async fn test_context_with_request_body(
    scope: Scope,
    method_name: &str,
    actions_uri: &str,
    response_code: StatusCode,
    request_body: impl Serialize,
    resp_body: Option<impl Serialize>,
) -> TestContext {
    let template = ResponseTemplate::new(response_code.as_u16()).set_body_json(resp_body);
    let server = setup_api(
        scope,
        method_name,
        actions_uri,
        Some(request_body),
        template,
    )
    .await;
    let client = Octocrab::builder()
        .base_uri(server.uri())
        .unwrap()
        .build()
        .unwrap();

    TestContext {
        _server: server,
        client,
    }
}

///////////////////////////////////////////////////////////////////////////////
// Tests
///////////////////////////////////////////////////////////////////////////////

#[tokio::test]
async fn should_return_page_with_org_self_hosted_runners() {
    let expected_runners: FakePage =
        serde_json::from_str(include_str!("resources/self_hosted_runners.json")).unwrap();
    let test_context = test_context(
        Scope::Org,
        "GET",
        "/actions/runners",
        StatusCode::OK,
        Some(expected_runners.clone()),
    )
    .await;
    let result = test_context
        .client
        .actions()
        .list_org_self_hosted_runners(ORG)
        .send()
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let Page {
        total_count, items, ..
    } = result.unwrap();
    assert_eq!(total_count.unwrap(), expected_runners.total_count);
    assert_eq!(items, expected_runners.runners);
}

#[tokio::test]
async fn should_return_page_with_repo_self_hosted_runners() {
    let expected_runners: FakePage =
        serde_json::from_str(include_str!("resources/self_hosted_runners.json")).unwrap();
    let test_context = test_context(
        Scope::Repo,
        "GET",
        "/actions/runners",
        StatusCode::OK,
        Some(expected_runners.clone()),
    )
    .await;
    let result = test_context
        .client
        .actions()
        .list_repo_self_hosted_runners(OWNER, REPO)
        .send()
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let Page {
        total_count, items, ..
    } = result.unwrap();
    assert_eq!(total_count.unwrap(), expected_runners.total_count);
    assert_eq!(items, expected_runners.runners);
}

#[tokio::test]
async fn should_return_org_jit_config() {
    let jit_config_req = JitConfigBody {
        name: "jit_config".into(),
        runner_group_id: 20.into(),
        labels: vec!["label-1".into(), "label-2".into()],
    };
    let expected_response = jit_config_req.expected_response();

    let test_context = test_context_with_request_body(
        Scope::Org,
        "POST",
        "/actions/runners/generate-jitconfig",
        StatusCode::CREATED,
        jit_config_req.clone(),
        Some(expected_response.clone()),
    )
    .await;

    let result = test_context
        .client
        .actions()
        .create_org_jit_runner_config(
            ORG,
            jit_config_req.name,
            jit_config_req.runner_group_id,
            jit_config_req.labels,
        )
        .send()
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let config = result.unwrap();
    assert_eq!(config, expected_response);
}

#[tokio::test]
async fn should_return_repo_jit_config() {
    let jit_config_req = JitConfigBody {
        name: "jit_config".into(),
        runner_group_id: 20.into(),
        labels: vec!["label-1".into(), "label-2".into()],
    };
    let expected_response = jit_config_req.expected_response();

    let test_context = test_context_with_request_body(
        Scope::Repo,
        "POST",
        "/actions/runners/generate-jitconfig",
        StatusCode::CREATED,
        jit_config_req.clone(),
        Some(expected_response.clone()),
    )
    .await;

    let result = test_context
        .client
        .actions()
        .create_repo_jit_runner_config(
            OWNER,
            REPO,
            jit_config_req.name,
            jit_config_req.runner_group_id,
            jit_config_req.labels,
        )
        .send()
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let config = result.unwrap();
    assert_eq!(config, expected_response);
}

#[tokio::test]
async fn should_return_org_registration_token() {
    let expected_token: SelfHostedRunnerToken =
        serde_json::from_str(include_str!("resources/self_hosted_runner_token.json")).unwrap();
    let test_context = test_context(
        Scope::Org,
        "POST",
        "/actions/runners/registration-token",
        StatusCode::CREATED,
        Some(expected_token.clone()),
    )
    .await;
    let result = test_context
        .client
        .actions()
        .create_org_runner_registration_token(ORG)
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let token = result.unwrap();
    assert_eq!(token, expected_token);
}

#[tokio::test]
async fn should_return_repo_registration_token() {
    let expected_token: SelfHostedRunnerToken =
        serde_json::from_str(include_str!("resources/self_hosted_runner_token.json")).unwrap();
    let test_context = test_context(
        Scope::Repo,
        "POST",
        "/actions/runners/registration-token",
        StatusCode::CREATED,
        Some(expected_token.clone()),
    )
    .await;
    let result = test_context
        .client
        .actions()
        .create_repo_runner_registration_token(OWNER, REPO)
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let token = result.unwrap();
    assert_eq!(token, expected_token);
}

#[tokio::test]
async fn should_return_org_remove_token() {
    let expected_token: SelfHostedRunnerToken =
        serde_json::from_str(include_str!("resources/self_hosted_runner_token.json")).unwrap();
    let test_context = test_context(
        Scope::Org,
        "POST",
        "/actions/runners/remove-token",
        StatusCode::CREATED,
        Some(expected_token.clone()),
    )
    .await;
    let result = test_context
        .client
        .actions()
        .create_org_runner_remove_token(ORG)
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let token = result.unwrap();
    assert_eq!(token, expected_token);
}

#[tokio::test]
async fn should_return_repo_remove_token() {
    let expected_token: SelfHostedRunnerToken =
        serde_json::from_str(include_str!("resources/self_hosted_runner_token.json")).unwrap();
    let test_context = test_context(
        Scope::Repo,
        "POST",
        "/actions/runners/remove-token",
        StatusCode::CREATED,
        Some(expected_token.clone()),
    )
    .await;
    let result = test_context
        .client
        .actions()
        .create_repo_runner_remove_token(OWNER, REPO)
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let token = result.unwrap();
    assert_eq!(token, expected_token);
}
#[tokio::test]
async fn should_return_single_org_self_hosted_runner() {
    const RUNNER_ID: u64 = 30;

    let expected_runner: SelfHostedRunner =
        serde_json::from_str(include_str!("resources/self_hosted_runner.json")).unwrap();
    let test_context = test_context(
        Scope::Org,
        "GET",
        &format!("/actions/runners/{RUNNER_ID}"),
        StatusCode::OK,
        Some(expected_runner.clone()),
    )
    .await;
    let result = test_context
        .client
        .actions()
        .get_org_runner(ORG, RUNNER_ID.into())
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let runner = result.unwrap();
    assert_eq!(runner, expected_runner);
}

#[tokio::test]
async fn should_return_single_repo_self_hosted_runner() {
    const RUNNER_ID: u64 = 30;

    let expected_runner: SelfHostedRunner =
        serde_json::from_str(include_str!("resources/self_hosted_runner.json")).unwrap();
    let test_context = test_context(
        Scope::Repo,
        "GET",
        &format!("/actions/runners/{RUNNER_ID}"),
        StatusCode::OK,
        Some(expected_runner.clone()),
    )
    .await;
    let result = test_context
        .client
        .actions()
        .get_repo_runner(OWNER, REPO, RUNNER_ID.into())
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let runner = result.unwrap();
    assert_eq!(runner, expected_runner);
}

#[tokio::test]
async fn should_return_no_content_deleting_org_runner() {
    const RUNNER_ID: u64 = 30;

    let test_context = test_context(
        Scope::Org,
        "DELETE",
        &format!("/actions/runners/{RUNNER_ID}"),
        StatusCode::NO_CONTENT,
        None::<()>,
    )
    .await;
    let result = test_context
        .client
        .actions()
        .delete_org_runner(ORG, RUNNER_ID.into())
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_return_no_content_deleting_repo_runner() {
    const RUNNER_ID: u64 = 30;

    let test_context = test_context(
        Scope::Repo,
        "DELETE",
        &format!("/actions/runners/{RUNNER_ID}"),
        StatusCode::NO_CONTENT,
        None::<()>,
    )
    .await;
    let result = test_context
        .client
        .actions()
        .delete_repo_runner(OWNER, REPO, RUNNER_ID.into())
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}
