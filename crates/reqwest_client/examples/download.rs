use std::time::Instant;

use anyhow::{Context as _, Result, anyhow};
use futures::AsyncReadExt as _;
use http_client::{AsyncBody, HttpClient as _};
use reqwest_client::ReqwestClient;

fn main() -> Result<()> {
    let url = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("usage: cargo run -p reqwest_client --example download -- <url>"))?;

    let client = ReqwestClient::user_agent("zed-reqwest-client-download-example")?;

    futures::executor::block_on(async move {
        let started_at = Instant::now();
        let mut response = client
            .get(&url, AsyncBody::empty(), true)
            .await
            .with_context(|| format!("requesting {url}"))?;
        let headers_elapsed = started_at.elapsed();

        println!("status: {}", response.status());
        println!("version: {:?}", response.version());
        if let Some(content_length) = response
            .headers()
            .get(http_client::http::header::CONTENT_LENGTH)
            && let Ok(content_length) = content_length.to_str()
        {
            println!("content-length: {content_length}");
        }
        println!("time-to-headers: {:.3}s", headers_elapsed.as_secs_f64());

        let mut buffer = vec![0; 1024 * 1024];
        let mut bytes_downloaded = 0usize;
        let body_started_at = Instant::now();

        loop {
            let bytes_read = response
                .body_mut()
                .read(&mut buffer)
                .await
                .context("reading response body")?;
            if bytes_read == 0 {
                break;
            }
            bytes_downloaded += bytes_read;
        }

        let body_elapsed = body_started_at.elapsed();
        let total_elapsed = started_at.elapsed();
        let mebibytes = bytes_downloaded as f64 / 1024.0 / 1024.0;
        let body_seconds = body_elapsed.as_secs_f64();
        let total_seconds = total_elapsed.as_secs_f64();

        println!("downloaded-bytes: {bytes_downloaded}");
        println!("downloaded-mib: {mebibytes:.3}");
        println!("body-time: {body_seconds:.3}s");
        println!("total-time: {total_seconds:.3}s");
        if body_seconds > 0.0 {
            println!("body-throughput: {:.3} MiB/s", mebibytes / body_seconds);
        }
        if total_seconds > 0.0 {
            println!("total-throughput: {:.3} MiB/s", mebibytes / total_seconds);
        }

        anyhow::ensure!(
            response.status().is_success(),
            "download completed with unsuccessful status {}",
            response.status()
        );

        Ok(())
    })
}
