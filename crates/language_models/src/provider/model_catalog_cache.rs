//! Fetches public model catalogs and keeps a copy of each on disk, so that a
//! provider has its models before the network answers.

use anyhow::{Context as _, Result, bail};
use fs::Fs;
use futures::{AsyncReadExt, FutureExt};
use gpui::BackgroundExecutor;
use http_client::{AsyncBody, HttpClient, Method, Request};
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
    let mut body = String::new();
    response
        .body_mut()
        .take(RESPONSE_LIMIT_BYTES)
        .read_to_string(&mut body)
        .await?;
    Ok(body)
}

/// The catalog stored at `path`, and whether it is recent enough to use
/// without fetching it again.
pub(crate) async fn load_cached(fs: &dyn Fs, path: &Path) -> Result<Option<(String, bool)>> {
    let Some(metadata) = fs.metadata(path).await? else {
        return Ok(None);
    };
    let catalog = fs.load(path).await?;
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
