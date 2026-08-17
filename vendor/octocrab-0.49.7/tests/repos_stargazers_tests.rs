// Tests for calls to the /repos/{owner}/{repo}/stargazers API.
mod mock_error;

use mock_error::setup_error_handler;
use octocrab::{models::StarGazer, Octocrab, Page};
use serde::{Deserialize, Serialize};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[derive(Serialize, Deserialize)]
struct FakePage<T> {
    items: Vec<T>,
}

async fn setup_api(template: ResponseTemplate) -> MockServer {
    let owner = "owner";
    let repo = "repo";
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(format!("/repos/{owner}/{repo}/stargazers")))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on /repo/{owner}/{repo}/stargazers was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

const OWNER: &str = "owner";
const REPO: &str = "repo";

#[tokio::test]
async fn should_return_page_with_users() {
    let star_gazers: Vec<StarGazer> =
        serde_json::from_str(include_str!("resources/stargazers.json")).unwrap();
    let login1: String = star_gazers[0].user.as_ref().unwrap().login.clone();
    let page_response = FakePage { items: star_gazers };
    let template = ResponseTemplate::new(200).set_body_json(&page_response);
    let mock_server = setup_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let repos = client.repos(OWNER.to_owned(), REPO.to_owned());
    let result = repos.list_stargazers().send().await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let Page { items, .. } = result.unwrap();
    {
        assert_eq!(items.len(), 3);
        assert_eq!(items[0].user.as_ref().unwrap().login, login1);
    }
}

#[tokio::test]
async fn should_return_page_with_all_users() {
    let star_gazers: Vec<StarGazer> =
        serde_json::from_str(include_str!("resources/stargazers.json")).unwrap();
    let login1: String = star_gazers[0].user.as_ref().unwrap().login.clone();
    let login2: String = star_gazers[1].user.as_ref().unwrap().login.clone();
    let page_response = FakePage { items: star_gazers };
    let template = ResponseTemplate::new(200).set_body_json(&page_response);
    let mock_server = setup_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let page = client
        .repos(OWNER.to_owned(), REPO.to_owned())
        .list_stargazers()
        .per_page(100)
        .send()
        .await
        .unwrap();

    let result = client.all_pages(page).await.unwrap();
    assert_eq!(result.len(), 3);
    assert_eq!(result[0].user.as_ref().unwrap().login, login1);
    assert_eq!(result[1].user.as_ref().unwrap().login, login2);
    assert_eq!(result[2].user, None);
}
