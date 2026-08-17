use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::{Author, Contributor};
use octocrab::Octocrab;

/// Unit test for calls to the `/repos/OWNER/REPO/contributors` endpoint
mod mock_error;

const OWNER: &str = "XAMPPRocky";
const REPO: &str = "octocrab";

async fn setup_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/repos/{OWNER}/{REPO}/contributors")))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        "GET on /repos/OWNER/REPO/contributors not called",
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_return_repo_contributors() {
    let repo_contributors_response: Vec<Contributor> =
        serde_json::from_str(include_str!("resources/repo_contributors.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&repo_contributors_response);
    let mock_server = setup_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER, REPO)
        .list_contributors()
        .anon(true)
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let contributors = result.unwrap();

    assert!(!contributors.items.is_empty());

    let Contributor {
        author: Author { login, .. },
        contributions,
        ..
    } = contributors.items.first().unwrap();

    {
        assert_eq!(login, "XAMPPRocky");
        assert!(*contributions > 0);
    }
}
