use http::header::USER_AGENT;
use http::Uri;
use hyper_rustls::HttpsConnectorBuilder;

use octocrab::service::middleware::base_uri::BaseUriLayer;
use octocrab::service::middleware::extra_headers::ExtraHeadersLayer;
use octocrab::{AuthState, OctocrabBuilder};
use std::sync::Arc;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let connector = HttpsConnectorBuilder::new()
        .with_native_roots() // enabled the `rustls-native-certs` feature in hyper-rustls
        .unwrap()
        .https_only()
        .enable_http1()
        .build();

    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build(connector);
    let octocrab = OctocrabBuilder::new_empty()
        .with_service(client)
        .with_layer(&BaseUriLayer::new(Uri::from_static(
            "https://api.github.com",
        )))
        .with_layer(&ExtraHeadersLayer::new(Arc::new(vec![(
            USER_AGENT,
            "octocrab".parse().unwrap(),
        )])))
        .with_auth(AuthState::None)
        .build()
        .unwrap();

    let repo = octocrab.repos("rust-lang", "rust").get().await?;

    let repo_metrics = octocrab
        .repos("rust-lang", "rust")
        .get_community_profile_metrics()
        .await?;

    println!(
        "{} has {} stars and {}% health percentage",
        repo.full_name.unwrap(),
        repo.stargazers_count.unwrap_or(0),
        repo_metrics.health_percentage
    );

    Ok(())
}
