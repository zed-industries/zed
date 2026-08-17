use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use octocrab::models::pulls::{Review, ReviewAction, ReviewComment};
use octocrab::Octocrab;

use crate::mock_error::setup_error_handler;

/// Unit test for calls to the `/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}` endpoint
mod mock_error;

const OWNER: &str = "XAMPPRocky";
const REPO: &str = "octocrab";
const PULL_NUMBER: u64 = 42;
const REVIEW_ID: u64 = 42;
const COMMENT_ID: u64 = 42;

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_work_with_specific_review() {
    let review_ops_response: Review =
        serde_json::from_str(include_str!("resources/get_pull_request_review.json")).unwrap();
    let review_comments_response: Vec<ReviewComment> = serde_json::from_str(include_str!(
        "resources/get_pull_request_review_comments.json"
    ))
    .unwrap();
    let pr_comment_response: ReviewComment =
        serde_json::from_str(include_str!("resources/pull_request_review_comment.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&review_ops_response);
    let comments_template = ResponseTemplate::new(200).set_body_json(&review_comments_response);
    let pr_comment_template = ResponseTemplate::new(200).set_body_json(&pr_comment_response);
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/pulls/{PULL_NUMBER}/reviews/{REVIEW_ID}"
        )))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;
    Mock::given(method("PATCH"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/pulls/{PULL_NUMBER}/reviews/{REVIEW_ID}"
        )))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;
    Mock::given(method("DELETE"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/pulls/{PULL_NUMBER}/reviews/{REVIEW_ID}"
        )))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;
    Mock::given(method("POST"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/pulls/{PULL_NUMBER}/reviews/{REVIEW_ID}/events"
        )))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;
    Mock::given(method("PUT"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/pulls/{PULL_NUMBER}/reviews/{REVIEW_ID}/dismissals"
        )))
        .respond_with(template.clone())
        .mount(&mock_server)
        .await;
    Mock::given(method("GET"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/pulls/{PULL_NUMBER}/reviews/{REVIEW_ID}/comments"
        )))
        .respond_with(comments_template.clone())
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path(format!(
            "/repos/{OWNER}/{REPO}/pulls/{PULL_NUMBER}/comments/{COMMENT_ID}/replies"
        )))
        .respond_with(pr_comment_template.clone())
        .mount(&mock_server)
        .await;

    setup_error_handler(
        &mock_server,
        &format!("request on /repos/{OWNER}/{REPO}/pulls/{PULL_NUMBER}/reviews/{REVIEW_ID} was not received"),
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());

    let result = client
        .pulls(OWNER, REPO)
        .pr_review_actions(PULL_NUMBER, REVIEW_ID)
        .get()
        .await;
    assert_eq!(result.unwrap(), review_ops_response);
    let result = client
        .pulls(OWNER, REPO)
        .pr_review_actions(PULL_NUMBER, REVIEW_ID)
        .update("test")
        .await;
    assert_eq!(result.unwrap(), review_ops_response);
    let result = client
        .pulls(OWNER, REPO)
        .pr_review_actions(PULL_NUMBER, REVIEW_ID)
        .delete_pending()
        .await;
    assert_eq!(result.unwrap(), review_ops_response);
    let result = client
        .pulls(OWNER, REPO)
        .pr_review_actions(PULL_NUMBER, REVIEW_ID)
        .submit(ReviewAction::Comment, "test")
        .await;
    assert_eq!(result.unwrap(), review_ops_response);
    let result = client
        .pulls(OWNER, REPO)
        .pr_review_actions(PULL_NUMBER, REVIEW_ID)
        .dismiss("test")
        .await;
    assert_eq!(result.unwrap(), review_ops_response);

    let result = client
        .pulls(OWNER, REPO)
        .pr_review_actions(PULL_NUMBER, REVIEW_ID)
        .list_comments()
        .per_page(15)
        .send()
        .await;
    let result_items = result.unwrap();
    assert_eq!(result_items.items, review_comments_response);

    let result = client
        .pulls(OWNER, REPO)
        .reply_to_comment(PULL_NUMBER, COMMENT_ID.into(), "test")
        .await;
    assert_eq!(result.unwrap(), pr_comment_response);
}
