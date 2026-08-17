mod mock_error;

use mock_error::setup_error_handler;
use octocrab::Octocrab;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

async fn setup_get_api(template: ResponseTemplate) -> MockServer {
    let gist_id: &str = "12c55a94bd03166ff33ed0596263b4c6";

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/gists/{gist_id}/star")))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("GET on /gists/{gist_id}/star was not received"),
    )
    .await;
    mock_server
}

async fn setup_delete_star_api(template: ResponseTemplate) -> MockServer {
    let gist_id: &str = "12c55a94bd03166ff33ed0596263b4c6";

    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(format!("/gists/{gist_id}/star")))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("DELETE on /gists/{gist_id}/star was not received"),
    )
    .await;
    mock_server
}

async fn setup_delete_gist_api(template: ResponseTemplate) -> MockServer {
    let gist_id: &str = "12c55a94bd03166ff33ed0596263b4c6";

    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(format!("/gists/{gist_id}")))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("DELETE on /gists/{gist_id} was not received"),
    )
    .await;
    mock_server
}

async fn setup_put_api(template: ResponseTemplate) -> MockServer {
    let gist_id: &str = "12c55a94bd03166ff33ed0596263b4c6";

    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path(format!("/gists/{gist_id}/star")))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("PUT on /gists/{gist_id}/star was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

const GIST_ID: &str = "12c55a94bd03166ff33ed0596263b4c6";

#[tokio::test]
async fn test_get_gists_star_204() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_get_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().is_starred(GIST_ID.to_owned()).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let result = result.unwrap();
    assert!(result, "expected the result to be true: {}", result);
}

#[tokio::test]
async fn test_get_gists_star_404() {
    let template = ResponseTemplate::new(404);
    let mock_server = setup_get_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().is_starred(GIST_ID.to_owned()).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let result = result.unwrap();
    assert!(!result, "expected the result to be false: {}", result);
}

#[tokio::test]
async fn test_get_gists_star_500() {
    let template = ResponseTemplate::new(500);
    let mock_server = setup_get_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().is_starred(GIST_ID.to_owned()).await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_put_gists_star_204() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_put_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().star(GIST_ID.to_owned()).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_put_gists_star_404() {
    let template = ResponseTemplate::new(404);
    let mock_server = setup_put_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().star(GIST_ID.to_owned()).await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_put_gists_star_500() {
    let template = ResponseTemplate::new(500);
    let mock_server = setup_put_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().star(GIST_ID.to_owned()).await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_delete_gists_star_204() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_delete_star_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().unstar(GIST_ID.to_owned()).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_delete_gists_star_304() {
    let template = ResponseTemplate::new(304);
    let mock_server = setup_delete_star_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().unstar(GIST_ID.to_owned()).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_delete_gists_star_404() {
    let template = ResponseTemplate::new(404);
    let mock_server = setup_delete_star_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().unstar(GIST_ID.to_owned()).await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_delete_gists_star_500() {
    let template = ResponseTemplate::new(500);
    let mock_server = setup_delete_star_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().unstar(GIST_ID.to_owned()).await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_delete_gist_204() {
    let template = ResponseTemplate::new(204);
    let mock_server = setup_delete_gist_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().delete(GIST_ID.to_owned()).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_delete_gist_304() {
    let template = ResponseTemplate::new(304);
    let mock_server = setup_delete_gist_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().delete(GIST_ID.to_owned()).await;

    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_delete_gist_404() {
    let template = ResponseTemplate::new(404);
    let mock_server = setup_delete_gist_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().delete(GIST_ID.to_owned()).await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}

#[tokio::test]
async fn test_delete_gist_500() {
    let template = ResponseTemplate::new(500);
    let mock_server = setup_delete_gist_api(template).await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client.gists().delete(GIST_ID.to_owned()).await;

    assert!(
        result.is_err(),
        "expected error result, got success: {:#?}",
        result
    );
}
