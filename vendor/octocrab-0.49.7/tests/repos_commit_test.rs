mod mock_error;

use mock_error::setup_error_handler;
use octocrab::{models::repos::RepoCommit, Octocrab};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

async fn setup_repos_commits_api(template: ResponseTemplate) -> MockServer {
    let owner: &str = "org";
    let repo: &str = "some-repo";

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/repos/{owner}/{repo}/commits")))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("POST on /repos/{owner}/{repo}/commits was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

const OWNER: &str = "org";
const REPO: &str = "some-repo";

#[tokio::test]
async fn should_return_list_of_commits() {
    let repos_list_commits_json: Vec<RepoCommit> =
        serde_json::from_str(include_str!("resources/repos_list_commits.json")).unwrap();
    let template = ResponseTemplate::new(201).set_body_json(&repos_list_commits_json);
    let mock_server = setup_repos_commits_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .repos(OWNER.to_owned(), REPO.to_owned())
        .list_commits()
        .send()
        .await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );

    let result = result.unwrap();
    let result = result.items;

    assert!(
        result.len() == 1,
        "expected '1' for len(), got {:#?}",
        result.len()
    );

    result.iter().for_each(|commit| {
        println!("Commit = {commit:#?}");
        assert!(
            commit.sha == "24606b5f326a1356f031dd06431cfb0beddd475f",
            "expected '24606b5f326a1356f031dd06431cfb0beddd475f' value, got {:#?}",
            commit.sha
        );

        assert!(
            commit
                .commit
                .author
                .as_ref()
                .unwrap()
                .date
                .unwrap()
                .to_rfc3339()
                == "2024-04-30T08:54:10+00:00",
            "expected '2024-04-30T08:54:10+00:00' value, got {:#?}",
            commit
                .commit
                .author
                .as_ref()
                .unwrap()
                .date
                .unwrap()
                .to_rfc3339()
        );
    });
}
