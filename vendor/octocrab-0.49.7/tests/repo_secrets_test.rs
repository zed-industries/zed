// Tests for calls to the /repos/{owner}/{repo}/actions/secrets API.
mod mock_error;

use chrono::DateTime;
use mock_error::setup_error_handler;
use octocrab::{
    models::{
        repos::secrets::{
            CreateRepositorySecret, CreateRepositorySecretResponse, RepositorySecret,
            RepositorySecrets,
        },
        PublicKey,
    },
    Octocrab,
};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

const OWNER: &str = "owner";
const REPO: &str = "repo";

async fn setup_get_api(template: ResponseTemplate, secrets_path: &str) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/actions/secrets{secrets_path}"
        )))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on /repos/{OWNER}/{REPO}/actions/secrets{secrets_path} was not received"),
    )
    .await;
    mock_server
}

async fn setup_put_api(template: ResponseTemplate, secrets_path: &str) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/actions/secrets{secrets_path}"
        )))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("PUT on /repos/{OWNER}/{REPO}/actions/secrets{secrets_path} was not received"),
    )
    .await;
    mock_server
}

async fn setup_delete_api(template: ResponseTemplate, secrets_path: &str) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/actions/secrets{secrets_path}"
        )))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("DELETE on /repos/{OWNER}/{REPO}/actions/secrets{secrets_path} was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_return_repo_secrets() {
    let repo_secrets: RepositorySecrets =
        serde_json::from_str(include_str!("resources/repo_secrets.json")).unwrap();

    let template = ResponseTemplate::new(200).set_body_json(&repo_secrets);
    let mock_server = setup_get_api(template, "").await;
    let result = setup_octocrab(&mock_server.uri())
        .repos(OWNER.to_owned(), REPO.to_owned())
        .secrets()
        .get_secrets()
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let item = result.unwrap();

    assert_eq!(item.total_count, 2);
    assert_eq!(
        item.secrets,
        vec![
            RepositorySecret {
                name: String::from("GH_TOKEN"),
                created_at: DateTime::parse_from_rfc3339("2019-08-10T14:59:22Z")
                    .unwrap()
                    .into(),
                updated_at: DateTime::parse_from_rfc3339("2020-01-10T14:59:22Z")
                    .unwrap()
                    .into(),
            },
            RepositorySecret {
                name: String::from("GIST_ID"),
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
async fn should_return_repo_public_key() {
    let repo_secrets: PublicKey =
        serde_json::from_str(include_str!("resources/repo_public_key.json")).unwrap();

    let template = ResponseTemplate::new(200).set_body_json(&repo_secrets);
    let mock_server = setup_get_api(template, "/public-key").await;
    let result = setup_octocrab(&mock_server.uri())
        .repos(OWNER.to_owned(), REPO.to_owned())
        .secrets()
        .get_public_key()
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let item = result.unwrap();

    assert_eq!(item.key_id, String::from("012345678912345678"));
    assert_eq!(
        item.key,
        String::from("2Sg8iYjAxxmI2LvUXpJjkYrMxURPc8r+dB7TJyvv1234")
    );
}

#[tokio::test]
async fn should_return_repo_secret() {
    let repo_secrets: RepositorySecret =
        serde_json::from_str(include_str!("resources/repo_secret.json")).unwrap();

    let template = ResponseTemplate::new(200).set_body_json(&repo_secrets);
    let mock_server = setup_get_api(template, "/GH_TOKEN").await;
    let result = setup_octocrab(&mock_server.uri())
        .repos(OWNER.to_owned(), REPO.to_owned())
        .secrets()
        .get_secret("GH_TOKEN")
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let item = result.unwrap();
    assert_eq!(
        item,
        RepositorySecret {
            name: String::from("GH_TOKEN"),
            created_at: DateTime::parse_from_rfc3339("2019-08-10T14:59:22Z")
                .unwrap()
                .into(),
            updated_at: DateTime::parse_from_rfc3339("2020-01-10T14:59:22Z")
                .unwrap()
                .into(),
        }
    );
}

#[tokio::test]
async fn should_add_secret() {
    let template = ResponseTemplate::new(201);
    let mock_server = setup_put_api(template, "/GH_TOKEN").await;
    let result = setup_octocrab(&mock_server.uri())
        .repos(OWNER.to_owned(), REPO.to_owned())
        .secrets()
        .create_or_update_secret(
            "GH_TOKEN",
            &CreateRepositorySecret {
                key_id: "123456",
                encrypted_value: "some-b64-string",
            },
        )
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let item = result.unwrap();
    assert_eq!(item, CreateRepositorySecretResponse::Created);
}

#[tokio::test]
async fn should_update_secret_204() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_put_api(template, "/GH_TOKEN").await;
    let result = setup_octocrab(&mock_server.uri())
        .repos(OWNER.to_owned(), REPO.to_owned())
        .secrets()
        .create_or_update_secret(
            "GH_TOKEN",
            &CreateRepositorySecret {
                key_id: "123456",
                encrypted_value: "some-b64-string",
            },
        )
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let item = result.unwrap();
    assert_eq!(item, CreateRepositorySecretResponse::Updated);
}

#[tokio::test]
async fn should_delete_secret() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_delete_api(template, "/GH_TOKEN").await;
    let result = setup_octocrab(&mock_server.uri())
        .repos(OWNER.to_owned(), REPO.to_owned())
        .secrets()
        .delete_secret("GH_TOKEN")
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn should_noop_secret_500() {
    let template = ResponseTemplate::new(500);
    let mock_server = setup_delete_api(template, "/GH_TOKEN").await;
    let result = setup_octocrab(&mock_server.uri())
        .repos(OWNER.to_owned(), REPO.to_owned())
        .secrets()
        .delete_secret("GH_TOKEN")
        .await;

    assert!(
        result.is_err(),
        "expected error result, got success somehow: {:#?}",
        result
    );
}
