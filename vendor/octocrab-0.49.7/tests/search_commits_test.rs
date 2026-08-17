mod mock_error;

use serde::{Deserialize, Serialize};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::{
    models::{self, commits::Commit, SimpleUser},
    Octocrab,
};

async fn setup_mock_http_server(
    http_method: &str,
    mocked_path: &str,
    template: ResponseTemplate,
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
#[non_exhaustive]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FakePage<T> {
    pub items: Vec<T>,
    pub incomplete_results: Option<bool>,
    pub total_count: Option<u64>,
}

#[tokio::test]
async fn should_respond_to_search_commits() {
    let mocked_response: FakePage<Commit> =
        serde_json::from_str(include_str!("resources/search_commits.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_mock_http_server("GET", "/search/commits", template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .search()
        .commits("hello world repo:XAMPPRocky/octocrab")
        .sort("author-date")
        .order("desc")
        .send()
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let mut response = result.unwrap();
    let items = response.take_items();

    assert_eq!(items.len(), 1);
    let f = items.first().unwrap().clone().author;
    assert!(f.is_some_and(|a: SimpleUser| a.login == "octocat"),);
}
