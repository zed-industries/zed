use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use octocrab::models::pulls::Comment;
use octocrab::models::CommentId;
use octocrab::Octocrab;

use crate::mock_error::setup_error_handler;

/// Unit test for calls to the `/repos/{owner}/{repo}/pulls/comments/{comment_id}` endpoint
mod mock_error;

const OWNER: &str = "XAMPPRocky";
const REPO: &str = "octocrab";
const COMMENT_ID: u64 = 42;

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_work_with_review_comment() {
    let review_comment_response: Comment =
        serde_json::from_str(include_str!("resources/pull_request_review_comment.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&review_comment_response);
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/pulls/comments/{COMMENT_ID}"
        )))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;
    Mock::given(method("PATCH"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/pulls/comments/{COMMENT_ID}"
        )))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;
    Mock::given(method("DELETE"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/pulls/comments/{COMMENT_ID}"
        )))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("request on /repos/{OWNER}/{REPO}/pulls/comments/{COMMENT_ID} was not received"),
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .pulls(OWNER, REPO)
        .comment(CommentId(COMMENT_ID))
        .get()
        .await;
    assert_eq!(result.unwrap(), review_comment_response);
    let result = client
        .pulls(OWNER, REPO)
        .comment(CommentId(COMMENT_ID))
        .update("test")
        .await;
    assert_eq!(result.unwrap(), review_comment_response);
    let result = client
        .pulls(OWNER, REPO)
        .comment(CommentId(COMMENT_ID))
        .delete()
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
}
