use serde_json::json;
use wiremock::{
    matchers::{method, path_regex},
    Mock, MockServer, ResponseTemplate,
};

// Sets up a handler on the mock server which will return a 500 with the given message. This
// will be mapped internally into a GitHub json error, making it much easier to identify the cause
// of these test failures.
//
// This handler should always come after your real expectations as it will match any GET request.
pub async fn setup_error_handler(mock_server: &MockServer, message: &str) {
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(500).set_body_json(json!( {
            "documentation_url": "",
            "errors": None::<Vec<serde_json::Value>>,
            "message": message,
        })))
        .mount(mock_server)
        .await;
}
