//! Fetches public model catalogs and keeps a copy of each on disk, so that a
//! provider has its models before the network answers.

use anyhow::{Context as _, Result, bail};
use async_compression::futures::bufread::GzipDecoder;
use fs::Fs;
use futures::{AsyncRead, AsyncReadExt, FutureExt, io::BufReader};
use gpui::BackgroundExecutor;
use http_client::{
    AsyncBody, HttpClient, Method, Request,
    http::header::{ACCEPT_ENCODING, CONTENT_ENCODING},
};
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
    client: &dyn HttpClient,
    executor: &BackgroundExecutor,
    url: &str,
) -> Result<String> {
    let fetch = fetch_resource(client, url).fuse();
    let timeout = executor.timer(FETCH_TIMEOUT).fuse();
    futures::pin_mut!(fetch, timeout);
    futures::select_biased! {
        result = fetch => result,
        _ = timeout => bail!("model catalog request to {url} timed out"),
    }
}

async fn fetch_resource(client: &dyn HttpClient, url: &str) -> Result<String> {
    // Catalogs are public; completion credentials and custom headers must not be sent here.
    let request = Request::builder()
        .method(Method::GET)
        .uri(url)
        .header(ACCEPT_ENCODING, "gzip");
    let response = client
        .send(request.body(AsyncBody::empty())?)
        .await
        .with_context(|| format!("requesting model catalog at {url}"))?;
    if !response.status().is_success() {
        bail!(
            "model catalog request to {url} returned {}",
            response.status()
        );
    }
    let encoding = response
        .headers()
        .get(CONTENT_ENCODING)
        .map(|encoding| encoding.to_str())
        .transpose()?
        .unwrap_or("identity");
    if encoding.eq_ignore_ascii_case("gzip") {
        let mut decoder = GzipDecoder::new(BufReader::new(response.into_body()));
        decoder.multiple_members(true);
        read_catalog_response(decoder).await
    } else if encoding.eq_ignore_ascii_case("identity") {
        read_catalog_response(response.into_body()).await
    } else {
        bail!("unsupported model catalog content encoding: {encoding}");
    }
}

async fn read_catalog_response(reader: impl AsyncRead + Unpin) -> Result<String> {
    let mut body = Vec::new();
    reader
        .take(RESPONSE_LIMIT_BYTES + 1)
        .read_to_end(&mut body)
        .await?;
    anyhow::ensure!(
        body.len() as u64 <= RESPONSE_LIMIT_BYTES,
        "model catalog response exceeds the {RESPONSE_LIMIT_BYTES}-byte limit"
    );
    String::from_utf8(body).context("model catalog response is not valid UTF-8")
}

/// The catalog stored at `path`, and whether it is recent enough to use
/// without fetching it again.
pub(crate) async fn load_cached(
    fs: Arc<dyn Fs>,
    path: &Path,
    executor: &BackgroundExecutor,
) -> Result<Option<(String, bool)>> {
    load_cached_with(fs, path, executor, Ok).await
}

pub(crate) async fn load_cached_with<T: Send + 'static>(
    fs: Arc<dyn Fs>,
    path: &Path,
    executor: &BackgroundExecutor,
    decode: impl FnOnce(String) -> Result<T> + Send + 'static,
) -> Result<Option<(T, bool)>> {
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
            let text = String::from_utf8(catalog)
                .with_context(|| format!("Failed to read file {}", path.display()))?;
            decode(text)
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
    async fn responses_are_limited_to_the_decoded_size(cx: &mut gpui::TestAppContext) {
        use async_compression::futures::bufread::GzipEncoder;
        use futures::io::Cursor;

        let limit = usize::try_from(RESPONSE_LIMIT_BYTES).expect("response limit fits usize");
        for size in [limit, limit + 1] {
            let catalog = catalog_of_size(size);
            let mut compressed = Vec::new();
            GzipEncoder::new(Cursor::new(catalog.as_bytes()))
                .read_to_end(&mut compressed)
                .await
                .expect("gzip fixture should encode");
            for (encoding, bytes) in [(None, catalog.into_bytes()), (Some("gzip"), compressed)] {
                let client = http_client::FakeHttpClient::create(move |request| {
                    assert_eq!(request.headers()[ACCEPT_ENCODING], "gzip");
                    let body = AsyncBody::from(bytes.clone());
                    async move {
                        let mut response = http_client::Response::builder();
                        if let Some(encoding) = encoding {
                            response = response.header(CONTENT_ENCODING, encoding);
                        }
                        Ok(response.body(body)?)
                    }
                });
                let result = fetch(
                    client.as_ref(),
                    &cx.background_executor,
                    "https://catalog.example.com",
                )
                .await;
                assert_catalog_size(result, size, "response");
            }
        }
    }

    #[gpui::test]
    async fn disk_reads_report_the_size_limit(cx: &mut gpui::TestAppContext) {
        let fs = fs::FakeFs::new(cx.background_executor.clone());
        let path = PathBuf::from("/cache/catalog.json");
        let limit = usize::try_from(RESPONSE_LIMIT_BYTES).expect("response limit fits usize");
        for size in [limit, limit + 1] {
            store_cached(fs.as_ref(), path.clone(), catalog_of_size(size))
                .await
                .expect("write cached catalog");
            let result = load_cached(fs.clone(), &path, &cx.background_executor)
                .await
                .and_then(|catalog| catalog.context("missing cached catalog"))
                .map(|(catalog, _)| catalog);
            assert_catalog_size(result, size, "cached");
        }
    }

    fn catalog_of_size(size: usize) -> String {
        // A valid JSON prefix must not hide an oversized response.
        format!("{{}}{}", " ".repeat(size - 2))
    }

    fn assert_catalog_size(result: Result<String>, size: usize, source: &str) {
        if size as u64 <= RESPONSE_LIMIT_BYTES {
            let catalog = result.expect("catalog should be readable");
            assert_eq!(catalog.len(), size);
            assert_eq!(catalog.trim_end(), "{}");
        } else {
            let error = result.expect_err("oversized catalog").to_string();
            assert!(
                error.contains(source) && error.contains("8388608-byte limit"),
                "{error}"
            );
        }
    }
}
