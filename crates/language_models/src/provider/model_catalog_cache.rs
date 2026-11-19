//! Fetches public model catalogs and keeps a copy of each on disk, so that a
//! provider has its models before the network answers.

use anyhow::{Context as _, Result, bail};
use fs::Fs;
use futures::{AsyncReadExt, FutureExt};
use gpui::BackgroundExecutor;
use http_client::{AsyncBody, HttpClient, Method, Request};
use std::io::Read as _;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, SystemTime},
};

const RESPONSE_LIMIT_BYTES: u64 = 8 * 1024 * 1024;
const FETCH_TIMEOUT: Duration = Duration::from_secs(15);
const DISK_FRESHNESS: Duration = Duration::from_secs(5 * 60);
pub(crate) const REFRESH_INTERVAL: Duration = Duration::from_secs(60 * 60);

pub(crate) async fn fetch(
    client: Arc<dyn HttpClient>,
    executor: BackgroundExecutor,
    url: &str,
) -> Result<String> {
    let fetch = fetch_resource(client.as_ref(), url).fuse();
    let timeout = executor.timer(FETCH_TIMEOUT).fuse();
    futures::pin_mut!(fetch, timeout);
    futures::select_biased! {
        result = fetch => result,
        _ = timeout => bail!("model catalog request to {url} timed out"),
    }
}

async fn fetch_resource(client: &dyn HttpClient, url: &str) -> Result<String> {
    // Catalogs are public; completion credentials and custom headers must not be sent here.
    let request = Request::builder().method(Method::GET).uri(url);
    let mut response = client
        .send(request.body(AsyncBody::empty())?)
        .await
        .with_context(|| format!("requesting model catalog at {url}"))?;
    if !response.status().is_success() {
        bail!(
            "model catalog request to {url} returned {}",
            response.status()
        );
    }
    let mut body = Vec::new();
    response
        .body_mut()
        .take(RESPONSE_LIMIT_BYTES + 1)
        .read_to_end(&mut body)
        .await?;
    anyhow::ensure!(
        body.len() as u64 <= RESPONSE_LIMIT_BYTES,
        "model catalog response exceeds the {RESPONSE_LIMIT_BYTES}-byte limit"
    );
    String::from_utf8(body).map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "stream did not contain valid UTF-8",
        )
        .into()
    })
}

/// The catalog stored at `path`, and whether it is recent enough to use
/// without fetching it again.
pub(crate) async fn load_cached(
    fs: Arc<dyn Fs>,
    path: &Path,
    executor: &BackgroundExecutor,
) -> Result<Option<(String, bool)>> {
    let Some(metadata) = fs.metadata(path).await? else {
        return Ok(None);
    };
    let path = path.to_path_buf();
    let catalog = executor
        .spawn(async move {
            let reader = fs
                .open_sync(&path)
                .await
                .with_context(|| format!("Failed to read file {}", path.display()))?;
            let mut catalog = Vec::new();
            std::io::Read::take(reader, RESPONSE_LIMIT_BYTES + 1)
                .read_to_end(&mut catalog)
                .with_context(|| format!("Failed to read file {}", path.display()))?;
            anyhow::ensure!(
                catalog.len() as u64 <= RESPONSE_LIMIT_BYTES,
                "cached model catalog exceeds the {RESPONSE_LIMIT_BYTES}-byte limit"
            );
            String::from_utf8(catalog)
                .map_err(|_| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "stream did not contain valid UTF-8",
                    )
                })
                .with_context(|| format!("Failed to read file {}", path.display()))
        })
        .await?;
    let is_fresh = SystemTime::now()
        .duration_since(metadata.mtime.timestamp_for_user())
        .is_ok_and(|age| age < DISK_FRESHNESS);
    Ok(Some((catalog, is_fresh)))
}

pub(crate) async fn store_cached(fs: &dyn Fs, path: PathBuf, catalog: String) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs.create_dir(parent).await?;
    }
    fs.atomic_write(path, catalog).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[gpui::test]
    async fn response_and_disk_reads_report_the_size_limit(cx: &mut gpui::TestAppContext) {
        let scenario = async {
            let fs = fs::FakeFs::new(cx.background_executor.clone());
            let path = PathBuf::from("/cache/catalog.json");
            let limit = usize::try_from(RESPONSE_LIMIT_BYTES)?;
            for size in [limit, limit + 1] {
                // A complete JSON value followed by whitespace also tests that
                // exceeding the limit cannot be mistaken for a valid prefix.
                let body = format!("{{}}{}", " ".repeat(size - 2));
                let client = http_client::FakeHttpClient::create({
                    let body = body.clone();
                    move |_| {
                        let body = body.clone();
                        async move { Ok(http_client::Response::builder().body(AsyncBody::from(body))?) }
                    }
                });
                let response = fetch(
                    client,
                    cx.background_executor.clone(),
                    "https://catalog.example.com",
                )
                .await;
                store_cached(fs.as_ref(), path.clone(), body).await?;
                let cached = load_cached(fs.clone(), &path, &cx.background_executor).await;
                if size == limit {
                    assert_eq!(response?.len(), size);
                    assert_eq!(cached?.context("cached catalog")?.0.len(), size);
                } else {
                    for (result, source) in [
                        (response.map(|_| ()), "response"),
                        (cached.map(|_| ()), "cached"),
                    ] {
                        let error = result.expect_err("oversized input").to_string();
                        assert!(
                            error.contains(source) && error.contains("8388608-byte limit"),
                            "{error}"
                        );
                    }
                }
            }
            Ok::<_, anyhow::Error>(())
        };
        scenario.await.expect("size limit scenario should complete");
    }
}
