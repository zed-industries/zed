use anyhow::{Context as _, Result, anyhow, ensure};
use http_client::{
    HttpClient, HttpRequestExt as _, RedirectPolicy, Request, StatusCode,
    http::{
        HeaderMap, HeaderValue,
        header::{
            ACCEPT_ENCODING, CONTENT_ENCODING, CONTENT_LENGTH, CONTENT_RANGE, ETAG, IF_RANGE, RANGE,
        },
    },
};
use smol::{
    fs::{self, File},
    io::{AsyncReadExt as _, AsyncSeekExt as _, AsyncWriteExt as _},
};
use std::{
    future::Future,
    io::{self, SeekFrom},
    path::Path,
    time::Duration,
};

const RETRY_DELAYS: [u64; 4] = [1, 2, 4, 8];

pub(super) async fn download<Delay: Future<Output = ()>>(
    client: &dyn HttpClient,
    url: &str,
    target: &Path,
    wait: impl Fn(Duration) -> Delay,
    mut report_progress: impl FnMut(u64, Option<u64>),
) -> Result<()> {
    let mut transfer = Transfer::default();
    let result = async {
        let directory = target
            .parent()
            .context("missing remote server cache directory")?;
        let (file, path) = tempfile::NamedTempFile::new_in(directory)?.into_parts();
        let mut file = File::from(file);
        let mut delays = RETRY_DELAYS.into_iter();

        loop {
            match transfer
                .attempt(client, url, &mut file, &mut report_progress)
                .await?
            {
                Attempt::Complete => break,
                Attempt::Retry(error) => {
                    let Some(delay) = delays.next() else {
                        return Err(error).context("Remote server download retries exhausted");
                    };
                    log::warn!(
                        "Remote server download interrupted at {} bytes: {error:#}",
                        transfer.written
                    );
                    wait(Duration::from_secs(delay)).await;
                }
            }
        }

        file.flush().await?;
        drop(file);
        fs::rename(&path, target).await?;
        Ok(())
    }
    .await;

    result.with_context(|| match transfer.total {
        Some(total) => format!(
            "Downloaded {} of {total} bytes from {url}",
            transfer.written
        ),
        None => format!("Downloaded {} bytes from {url}", transfer.written),
    })
}

enum Attempt {
    Complete,
    Retry(anyhow::Error),
}

#[derive(Default)]
struct Transfer {
    written: u64,
    total: Option<u64>,
    etag: Option<HeaderValue>,
}

impl Transfer {
    async fn attempt(
        &mut self,
        client: &dyn HttpClient,
        url: &str,
        file: &mut File,
        report_progress: &mut impl FnMut(u64, Option<u64>),
    ) -> Result<Attempt> {
        let mut request = Request::get(url)
            .follow_redirects(RedirectPolicy::FollowAll)
            .timeout(Duration::from_secs(5 * 60))
            .header(ACCEPT_ENCODING, "identity");
        // A strong validator keeps two responses from different archives from being joined.
        let resume = self.etag.as_ref().filter(|_| self.written > 0);
        if let Some(etag) = resume {
            request = request
                .header(RANGE, format!("bytes={}-", self.written))
                .header(IF_RANGE, etag);
        }

        let mut response = match client.send(request.body(Default::default())?).await {
            Ok(response) => response,
            Err(error) => return Ok(Attempt::Retry(error)),
        };
        let status = response.status();
        if status.is_server_error()
            || matches!(
                status,
                StatusCode::REQUEST_TIMEOUT | StatusCode::TOO_MANY_REQUESTS
            )
        {
            return Ok(Attempt::Retry(anyhow!("HTTP {status}")));
        }

        let headers = response.headers();
        if matches!(status, StatusCode::OK | StatusCode::PARTIAL_CONTENT) {
            ensure!(
                headers
                    .get(CONTENT_ENCODING)
                    .is_none_or(|encoding| encoding.as_bytes().eq_ignore_ascii_case(b"identity")),
                "Remote server archive used an unexpected Content-Encoding"
            );
        }
        let end = match status {
            StatusCode::OK => {
                ensure!(
                    !headers.contains_key(CONTENT_RANGE),
                    "Unexpected Content-Range in a full remote server response"
                );
                let total = headers
                    .get(CONTENT_LENGTH)
                    .map(|length| -> Result<u64> { Ok(length.to_str()?.parse()?) })
                    .transpose()
                    .context("Invalid Content-Length for remote server archive")?;
                if self.written > 0 {
                    file.set_len(0).await?;
                    file.seek(SeekFrom::Start(0)).await?;
                }
                self.written = 0;
                self.total = total;
                self.etag = headers
                    .get(ETAG)
                    .filter(|etag| {
                        let etag = etag.as_bytes();
                        etag.len() >= 2 && etag.starts_with(b"\"") && etag.ends_with(b"\"")
                    })
                    .cloned();
                total
            }
            StatusCode::PARTIAL_CONTENT if resume.is_some() => {
                let (start, end, total) = parse_content_range(headers)
                    .context("Invalid Content-Range for remote server archive")?;
                ensure!(
                    start == self.written,
                    "Expected range to start at byte {}, got {start}",
                    self.written
                );
                ensure!(
                    self.total.is_none_or(|previous| previous == total),
                    "Remote server archive size changed"
                );
                ensure!(
                    headers.get(ETAG).is_none_or(|etag| Some(etag) == resume),
                    "Remote server archive ETag changed"
                );
                self.total = Some(total);
                Some(end + 1)
            }
            StatusCode::RANGE_NOT_SATISFIABLE if resume.is_some() => {
                self.etag = None;
                return Ok(Attempt::Retry(anyhow!("HTTP {status}")));
            }
            _ => return Err(anyhow!("HTTP {status}")),
        };

        report_progress(self.written, self.total);
        let mut last_reported = self.written;
        let mut buffer = vec![0; 64 * 1024];
        loop {
            let count = match response.body_mut().read(&mut buffer).await {
                Ok(0) => break,
                Ok(count) => count,
                Err(error) => return Ok(Attempt::Retry(error.into())),
            };
            let written = self.written + count as u64;
            ensure!(
                end.is_none_or(|end| written <= end),
                "Remote server response exceeded its declared length"
            );
            file.write_all(&buffer[..count]).await?;
            self.written = written;
            if self.written - last_reported >= 1024 * 1024 {
                report_progress(self.written, self.total);
                last_reported = self.written;
            }
        }

        if self.total.is_some_and(|total| self.written < total) {
            return Ok(Attempt::Retry(
                io::Error::from(io::ErrorKind::UnexpectedEof).into(),
            ));
        }
        ensure!(self.written > 0, "Remote server archive was empty");
        Ok(Attempt::Complete)
    }
}

fn parse_content_range(headers: &HeaderMap) -> Option<(u64, u64, u64)> {
    let range = headers.get(CONTENT_RANGE)?.to_str().ok()?;
    let (unit, range) = range.split_once(' ')?;
    if !unit.eq_ignore_ascii_case("bytes") {
        return None;
    }
    let (range, total) = range.split_once('/')?;
    let (start, end) = range.split_once('-')?;
    let (start, end, total) = (start.parse().ok()?, end.parse().ok()?, total.parse().ok()?);
    (start <= end && end < total).then_some((start, end, total))
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::TryStreamExt as _;
    use http_client::{AsyncBody, FakeHttpClient, HttpClientWithUrl, Response};
    use std::{
        collections::VecDeque,
        future,
        sync::{Arc, Mutex},
    };

    const URL: &str = "https://example.com/zed-remote-server.gz";
    const ETAG_VALUE: &str = "\"archive\"";

    fn interrupted_body() -> AsyncBody {
        let chunks = futures::stream::iter([
            Ok(&b"abc"[..]),
            Err(io::Error::from(io::ErrorKind::ConnectionReset)),
        ]);
        AsyncBody::from_reader(chunks.into_async_read())
    }

    fn response(status: StatusCode, body: impl Into<AsyncBody>) -> Response<AsyncBody> {
        Response::builder()
            .status(status)
            .header(ETAG, ETAG_VALUE)
            .body(body.into())
            .unwrap()
    }

    fn full_response(body: impl Into<AsyncBody>) -> Response<AsyncBody> {
        let mut response = response(StatusCode::OK, body);
        response
            .headers_mut()
            .insert(CONTENT_LENGTH, HeaderValue::from_static("6"));
        response
    }

    fn partial_response(range: &'static str) -> Response<AsyncBody> {
        let mut response = response(StatusCode::PARTIAL_CONTENT, "def");
        response
            .headers_mut()
            .insert(CONTENT_RANGE, HeaderValue::from_static(range));
        response
    }

    fn client(
        responses: impl IntoIterator<Item = (Option<&'static str>, Response<AsyncBody>)>,
    ) -> Arc<HttpClientWithUrl> {
        let responses = Mutex::new(responses.into_iter().collect::<VecDeque<_>>());
        FakeHttpClient::create(move |request| {
            let (expected_range, response) = responses
                .lock()
                .unwrap()
                .pop_front()
                .expect("unexpected request");
            assert_eq!(request.uri(), URL);
            assert_eq!(request.headers()[ACCEPT_ENCODING], "identity");
            assert_eq!(
                request
                    .headers()
                    .get(RANGE)
                    .map(|range| range.to_str().unwrap()),
                expected_range
            );
            assert_eq!(
                request
                    .headers()
                    .get(IF_RANGE)
                    .map(|etag| etag.to_str().unwrap()),
                expected_range.map(|_| ETAG_VALUE)
            );
            future::ready(Ok(response))
        })
    }

    #[test]
    fn resumes_after_a_connection_reset() {
        let directory = tempfile::tempdir().unwrap();
        let target = directory.path().join("server.gz");
        let client = client([
            (None, full_response(interrupted_body())),
            (Some("bytes=3-"), partial_response("bytes 3-5/6")),
        ]);
        let mut progress = Vec::new();
        smol::block_on(download(
            client.as_ref(),
            URL,
            &target,
            |_| future::ready(()),
            |written, total| {
                progress.push((written, total));
            },
        ))
        .unwrap();
        assert_eq!(std::fs::read(target).unwrap(), b"abcdef");
        assert_eq!(progress, [(0, Some(6)), (3, Some(6))]);
    }

    #[test]
    fn retries_a_connection_timeout() {
        use std::sync::atomic::{AtomicBool, Ordering};

        let directory = tempfile::tempdir().unwrap();
        let target = directory.path().join("server.gz");
        let timed_out = AtomicBool::new(false);
        let client = FakeHttpClient::create(move |request| {
            assert!(!request.headers().contains_key(RANGE));
            let response = if timed_out.swap(true, Ordering::Relaxed) {
                Ok(full_response("abcdef"))
            } else {
                Err(io::Error::from(io::ErrorKind::TimedOut).into())
            };
            future::ready(response)
        });
        smol::block_on(download(
            client.as_ref(),
            URL,
            &target,
            |_| future::ready(()),
            |_, _| {},
        ))
        .unwrap();
        assert_eq!(std::fs::read(target).unwrap(), b"abcdef");
    }

    #[test]
    fn restarts_after_the_server_returns_a_full_response() {
        let directory = tempfile::tempdir().unwrap();
        let target = directory.path().join("server.gz");
        let client = client([
            (None, full_response("abc")),
            (Some("bytes=3-"), full_response("123456")),
        ]);
        smol::block_on(download(
            client.as_ref(),
            URL,
            &target,
            |_| future::ready(()),
            |_, _| {},
        ))
        .unwrap();
        assert_eq!(std::fs::read(target).unwrap(), b"123456");
    }

    #[test]
    fn restarts_if_the_partial_file_has_no_strong_etag() {
        let directory = tempfile::tempdir().unwrap();
        let target = directory.path().join("server.gz");
        let mut initial = full_response("abc");
        initial
            .headers_mut()
            .insert(ETAG, HeaderValue::from_static("W/\"archive\""));
        let client = client([(None, initial), (None, full_response("123456"))]);
        smol::block_on(download(
            client.as_ref(),
            URL,
            &target,
            |_| future::ready(()),
            |_, _| {},
        ))
        .unwrap();
        assert_eq!(std::fs::read(target).unwrap(), b"123456");
    }

    #[test]
    fn rejects_a_partial_response_starting_at_the_wrong_offset() {
        let directory = tempfile::tempdir().unwrap();
        let target = directory.path().join("server.gz");
        let client = client([
            (None, full_response("abc")),
            (Some("bytes=3-"), partial_response("bytes 2-5/6")),
        ]);
        let error = smol::block_on(download(
            client.as_ref(),
            URL,
            &target,
            |_| future::ready(()),
            |_, _| {},
        ))
        .unwrap_err();
        assert!(format!("{error:#}").contains("Expected range to start at byte 3"));
        assert_eq!(std::fs::read_dir(directory.path()).unwrap().count(), 0);
    }

    #[test]
    fn reports_the_download_url_and_partial_size_after_retries() {
        let directory = tempfile::tempdir().unwrap();
        let target = directory.path().join("server.gz");
        let client = client(
            [(None, full_response("abc"))]
                .into_iter()
                .chain(RETRY_DELAYS.map(|_| {
                    (
                        Some("bytes=3-"),
                        response(StatusCode::SERVICE_UNAVAILABLE, ""),
                    )
                })),
        );
        let delays = std::cell::RefCell::new(Vec::new());
        let error = smol::block_on(download(
            client.as_ref(),
            URL,
            &target,
            |delay| {
                delays.borrow_mut().push(delay.as_secs());
                future::ready(())
            },
            |_, _| {},
        ))
        .unwrap_err();
        assert_eq!(*delays.borrow(), RETRY_DELAYS);
        assert_eq!(
            error.to_string(),
            format!("Downloaded 3 of 6 bytes from {URL}")
        );
        assert_eq!(std::fs::read_dir(directory.path()).unwrap().count(), 0);
    }
}
