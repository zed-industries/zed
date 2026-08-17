// Tests for calls to the /repos/{owner}/members API.
mod mock_error;

use mock_error::setup_error_handler;
use octocrab::{models::Author, Octocrab, Page};
use serde::{Deserialize, Serialize};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[derive(Serialize, Deserialize)]
struct FakePage<T> {
    items: Vec<T>,
}

async fn setup_list_api(template: ResponseTemplate) -> MockServer {
    let org = "org";
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/orgs/{org}/members")))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on /orgs/{org}/members was not received"),
    )
    .await;
    mock_server
}

//
async fn setup_check_membership_api(template: ResponseTemplate) -> MockServer {
    let org = "org";
    let mock_server = MockServer::start().await;
    let username = "mona";

    Mock::given(method("GET"))
        .and(path(format!("/orgs/{org}/members/{username}")))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on /orgs/{org}/members/{username} was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

const ORG: &str = "org";
const USERNAME: &str = "mona";

#[tokio::test]
async fn should_return_page_with_users() {
    let org_members: Author =
        serde_json::from_str(include_str!("resources/org_members.json")).unwrap();
    let login: String = org_members.login.clone();
    let page_response = FakePage {
        items: vec![org_members],
    };
    let template = ResponseTemplate::new(200).set_body_json(&page_response);
    let mock_server = setup_list_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs(ORG.to_owned());
    let result = org.list_members().send().await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let Page { items, .. } = result.unwrap();
    {
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].login, login);
    }
}

#[tokio::test]
async fn org_check_membership_204() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_check_membership_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs(ORG.to_owned());
    let result = org.check_membership(USERNAME.to_owned()).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    assert!(
        result.as_ref().unwrap(),
        "expected _true_ to be returned, but got {:#?}",
        result
    );
}

#[tokio::test]
async fn org_check_membership_404() {
    let template = ResponseTemplate::new(404);
    let mock_server = setup_check_membership_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs(ORG.to_owned());
    let result = org.check_membership(USERNAME.to_owned()).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    assert!(
        !result.as_ref().unwrap(),
        "expected false to be returned, but got {:#?}",
        result
    );
}

#[tokio::test]
async fn org_check_membership_500() {
    let template = ResponseTemplate::new(500);
    let mock_server = setup_check_membership_api(template).await;
    let client = setup_octocrab(&mock_server.uri());
    let org = client.orgs(ORG.to_owned());
    let result = org.check_membership(USERNAME.to_owned()).await;

    assert!(
        result.is_err(),
        "expected error result, got success somehow: {:#?}",
        result
    );
}
