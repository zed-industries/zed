use http::header::ACCEPT;
use octocrab::models::repos::secret_scanning_alert::UpdateSecretScanningAlert;
use octocrab::Octocrab;

const OWNER: &str = "org";
const REPO: &str = "some-repo";

#[tokio::main]
async fn main() {
    // example for Secret Scanning alerts API with OAuth GitHub App
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
    // Get all Secret Scanning alerts for a repo
    let a = octocrab
        .repos(OWNER, REPO)
        .secrets_scanning()
        .direction("asc")
        .get_alerts()
        .await
        .unwrap();
    println!("{a:?}");
    // Get a single Secret Scanning alert
    let single_alert = octocrab
        .repos(OWNER, REPO)
        .secrets_scanning()
        .get_alert(5)
        .await
        .unwrap();
    println!("{single_alert:?}");
    // Update (dismiss) a Secret Scanning alert
    let updated_alert = octocrab
        .repos(OWNER, REPO)
        .secrets_scanning()
        .update_alert(
            5,
            Some(&UpdateSecretScanningAlert {
                state: "resolved",
                resolution: Some("used_in_tests"),
                resolution_comment: Some("Mock value that is used in tests"),
            }),
        )
        .await
        .unwrap();
    println!("{updated_alert:?}");

    // Get alert locations
    let alert_location = octocrab
        .repos("owner", "repo")
        .secrets_scanning()
        .get_alert_locations(5)
        .await
        .unwrap();
    println!("{alert_location:?}");
}
