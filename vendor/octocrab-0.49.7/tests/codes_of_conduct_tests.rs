mod mock_error;

use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;
use octocrab::models::codes_of_conduct::CodeOfConduct;
use octocrab::{models, Octocrab};

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

#[tokio::test]
async fn should_respond_to_list_all_codes_of_conduct() {
    let mocked_response: Vec<models::codes_of_conduct::CodeOfConduct> =
        serde_json::from_str(include_str!("resources/codes_of_conduct_list_all.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_mock_http_server("GET", "/codes_of_conduct", template).await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client.codes_of_conduct().list_all_codes_of_conduct().await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.len(), 2);
    assert_eq!(response[0].name, "Citizen Code of Conduct");
    assert_eq!(response[0].html_url, "http://citizencodeofconduct.org/");
}

#[tokio::test]
async fn should_respond_to_get_code_of_conduct() {
    let code_of_conduct_key: String = "contributor_covenant".into();
    let mocked_response: models::codes_of_conduct::CodeOfConduct =
        serde_json::from_str(include_str!("resources/codes_of_conduct_get_code.json")).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
    let mock_server = setup_mock_http_server(
        "GET",
        format!("/codes_of_conduct/{code_of_conduct_key}").as_str(),
        template,
    )
    .await;
    let client = setup_octocrab(&mock_server.uri());
    let result = client
        .codes_of_conduct()
        .get_code_of_conduct(code_of_conduct_key)
        .await;
    assert!(
        result.is_ok(),
        "expected successful result, got error: {:#?}",
        result
    );
    let response = result.unwrap();
    assert_eq!(response.name, "Contributor Covenant");
    assert_eq!(
        response.html_url,
        "http://contributor-covenant.org/version/1/4/"
    );
    assert_eq!(
        response.url,
        "https://api.github.com/codes_of_conduct/contributor_covenant",
    );
}
