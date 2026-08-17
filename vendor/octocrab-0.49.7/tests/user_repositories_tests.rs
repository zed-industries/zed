/// Tests API calls related to check runs of a specific commit.
mod mock_error;

use mock_error::setup_error_handler;
use octocrab::models::{Repository, RepositoryId};
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

async fn setup_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    let mocked_path = "/users/some-user/repos";

    Mock::given(method("GET"))
        .and(path(mocked_path))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on {mocked_path} was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_return_repositories_for_user() {
    let mocked_response: Vec<Repository> =
        serde_json::from_str(include_str!("resources/user_repositories.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client.users("some-user").repos().send().await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let response = result.unwrap();
    let items = response.items;

    assert_eq!(items.len(), 2);

    {
        let item = &items[0];

        assert_eq!(RepositoryId(566109822), item.id);
        assert_eq!("actix-examples", item.name);
        assert_eq!("Apache-2.0", item.license.as_ref().unwrap().spdx_id);
    }

    {
        let item = &items[1];

        assert_eq!(RepositoryId(292435601), item.id);
        assert_eq!("amazon-sqs-java-temporary-queues-client", item.name);
        assert_eq!("Apache-2.0", item.license.as_ref().unwrap().spdx_id);
    }
}

#[tokio::test]
async fn should_fail_when_not_found() {
    let mocked_response = json!({
        "documentation_url": json!("rtm"),
        "errors": Value::Null,
        "message": json!("Its gone")
    });

    let template = ResponseTemplate::new(404).set_body_json(&mocked_response);
    let mock_server = setup_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client.users("some-user").repos().send().await;

    match result.unwrap_err() {
        Error::GitHub { source, .. } => {
            assert_eq!("Its gone", source.message)
        }
        other => panic!("Unexpected error: {:?}", other),
    }
}
