use http_body_util::BodyExt;
use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

    let octocrab = Octocrab::builder()
        .personal_token(token)
        .cache(octocrab::service::middleware::cache::mem::InMemoryCache::default())
        .build()?;

    let url = "https://api.github.com/repos/rust-lang/rust";
    let rate_limit_header = "x-ratelimit-remaining";

    // Use raw HTTP request to be able to access rate limit header value.
    let response = octocrab._get(url).await?;
    let remaining_first = response.headers().get(rate_limit_header).unwrap().clone();

    // Simulate reading the response body so it can be cached.
    response.into_body().collect().await?;

    let response = octocrab._get(url).await?;
    let remaining_second = response.headers().get(rate_limit_header).unwrap().clone();

    // Rate limit remaining count didn't change after the second request.
    println!("{rate_limit_header} after the first request: {remaining_first:?}");
    println!("{rate_limit_header} after the second request: {remaining_second:?}");
    assert_eq!(remaining_first, remaining_second);

    Ok(())
}
