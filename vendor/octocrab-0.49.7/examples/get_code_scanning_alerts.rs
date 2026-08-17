use http::header::ACCEPT;
use octocrab::params::AlertState;
use octocrab::Octocrab;

const OWNER: &str = "org";
const REPO: &str = "some-repo";

#[tokio::main]
async fn main() {
    // example for Code Scanning alerts API with OAuth GitHub App
    let client_id = secrecy::SecretString::from(std::env::var("GITHUB_CLIENT_ID").unwrap());
    let crab = octocrab::Octocrab::builder()
        .base_uri("https://github.com")
        .unwrap()
        .add_header(ACCEPT, "application/json".to_string())
        .build()
        .unwrap();

    let codes = crab
        .authenticate_as_device(&client_id, ["security_events"])
        .await
        .unwrap();
    println!(
        "Go to {} and enter code {}",
        codes.verification_uri, codes.user_code
    );
    let auth = codes.poll_until_available(&crab, &client_id).await.unwrap();
    println!(
        "Auth: scope {:?}; token type {}",
        auth.scope, auth.token_type
    );
    let octocrab = Octocrab::builder()
        .oauth(auth)
        .add_header(ACCEPT, "application/vnd.github+json".to_string())
        .build()
        .unwrap();
    // Get all Code Scanning alerts for a repo
    let a = octocrab
        .code_scannings(OWNER.to_owned(), REPO.to_owned())
        .list()
        .send()
        .await
        .unwrap();
    println!("{a:?}");
    // Get a single Code Scanning alert
    let single_alert = octocrab
        .code_scannings(OWNER.to_owned(), REPO.to_owned())
        .get(1)
        .await
        .unwrap();
    println!("{single_alert:?}");
    // Update (Open) a Code Scanning alert
    let updated_alert = octocrab
        .code_scannings(OWNER.to_owned(), REPO.to_owned())
        .update(1)
        .state(AlertState::Open)
        .send()
        .await
        .unwrap();
    println!("{updated_alert:?}");
}
