mod releases;

pub use releases::{
    AssetKind, GitHubLspBinaryVersion, GithubRelease, GithubReleaseAsset, build_asset_url,
    get_release_by_tag_name, latest_github_release,
};

use std::{path::Path, pin::Pin, sync::Arc, task::Poll};

use anyhow::{Context, Result};
use async_compression::futures::bufread::{BzDecoder, GzipDecoder};
use futures::{AsyncRead, AsyncSeekExt, AsyncWrite, io::BufReader};
use gpui::{BackgroundExecutor, Task};
use http_client::HttpClient;
use sha2::{Digest, Sha256};

fn sha256_matches(actual: &str, expected: &str) -> bool {
    actual.eq_ignore_ascii_case(expected)
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct GithubBinaryMetadata {
    pub metadata_version: u64,
    pub digest: Option<String>,
}

impl GithubBinaryMetadata {
    pub async fn read_from_file(metadata_path: &Path) -> Result<GithubBinaryMetadata> {
        let metadata_content = async_fs::read_to_string(metadata_path)
            .await
            .with_context(|| format!("reading metadata file at {metadata_path:?}"))?;
        serde_json::from_str(&metadata_content)
            .with_context(|| format!("parsing metadata file at {metadata_path:?}"))
    }

    pub async fn write_to_file(&self, metadata_path: &Path) -> Result<()> {
        let metadata_content = serde_json::to_string(self)
            .with_context(|| format!("serializing metadata for {metadata_path:?}"))?;
        async_fs::write(metadata_path, metadata_content.as_bytes())
            .await
            .with_context(|| format!("writing metadata file at {metadata_path:?}"))?;
        Ok(())
    }
}

pub fn download_server_binary(
    executor: &BackgroundExecutor,
    http_client: Arc<dyn HttpClient>,
    url: &str,
    digest: Option<&str>,
    destination_path: &Path,
    asset_kind: AssetKind,
) -> Task<Result<()>> {
    spawn_download(
        executor,
        http_client,
        url,
        digest,
        destination_path,
        DownloadedAsset::Archive(asset_kind),
    )
}

pub fn download_server_raw_binary(
    executor: &BackgroundExecutor,
    http_client: Arc<dyn HttpClient>,
    url: &str,
    digest: Option<&str>,
    destination_path: &Path,
    binary_file_name: &str,
) -> Task<Result<()>> {
    spawn_download(
        executor,
        http_client,
        url,
        digest,
        destination_path,
        DownloadedAsset::RawBinary(binary_file_name.to_owned()),
    )
}

enum DownloadedAsset {
    Archive(AssetKind),
    RawBinary(String),
}

fn spawn_download(
    executor: &BackgroundExecutor,
    http_client: Arc<dyn HttpClient>,
    url: &str,
    digest: Option<&str>,
    destination_path: &Path,
    downloaded_asset: DownloadedAsset,
) -> Task<Result<()>> {
    let url = url.to_owned();
    let digest = digest.map(ToOwned::to_owned);
    let destination_path = destination_path.to_owned();
    executor.spawn(async move {
        download_server_asset(
            &*http_client,
            &url,
            digest.as_deref(),
            &destination_path,
            downloaded_asset,
        )
        .await
    })
}

async fn download_server_asset(
    http_client: &dyn HttpClient,
    url: &str,
    digest: Option<&str>,
    destination_path: &Path,
    downloaded_asset: DownloadedAsset,
) -> Result<()> {
    log::info!("downloading {url} into {destination_path:?}");
    let mut response = http_client
        .get(url, Default::default(), true)
        .await
        .with_context(|| format!("downloading release from {url}"))?;
    anyhow::ensure!(
        response.status().is_success(),
        "downloading {url} failed with status {status}",
        status = response.status()
    );

    let destination_parent = destination_path
        .parent()
        .with_context(|| format!("destination path has no parent: {destination_path:?}"))?;
    let staging_directory = tempfile::Builder::new()
        .prefix(".tmp-github-download-")
        .tempdir_in(destination_parent)
        .with_context(|| format!("creating staging directory in {destination_parent:?}"))?;
    let staged_path = staging_directory.path().join("download");

    match downloaded_asset {
        DownloadedAsset::Archive(asset_kind) => {
            download_and_extract(
                response.body_mut(),
                digest,
                url,
                staging_directory.path(),
                &staged_path,
                asset_kind,
            )
            .await?
        }
        DownloadedAsset::RawBinary(binary_file_name) => {
            async_fs::create_dir(&staged_path)
                .await
                .with_context(|| format!("creating directory {staged_path:?}"))?;
            let binary_path = staged_path.join(binary_file_name);
            let mut binary_file = async_fs::File::create(&binary_path)
                .await
                .with_context(|| format!("creating a file {binary_path:?} for {url}"))?;
            download_to_writer(response.body_mut(), &mut binary_file, digest, url).await?;
            util::fs::make_file_executable(&binary_path)
                .await
                .with_context(|| format!("marking {binary_path:?} as executable"))?;
        }
    }

    // `remove_dir_all` removes a symlink itself without following it, including dangling ones.
    if async_fs::symlink_metadata(destination_path)
        .await
        .is_ok_and(|metadata| metadata.is_dir() || metadata.is_symlink())
    {
        async_fs::remove_dir_all(destination_path)
            .await
            .with_context(|| format!("removing existing {destination_path:?}"))?;
    }
    async_fs::rename(&staged_path, destination_path)
        .await
        .with_context(|| format!("renaming {staged_path:?} to {destination_path:?}"))
}

async fn download_and_extract(
    body: impl AsyncRead + Unpin,
    digest: Option<&str>,
    url: &str,
    staging_directory: &Path,
    staged_path: &Path,
    asset_kind: AssetKind,
) -> Result<()> {
    // Buffer on the destination's volume rather than a possibly RAM-backed system temp dir.
    let mut asset_file =
        async_fs::File::from(tempfile::tempfile_in(staging_directory).with_context(|| {
            format!("creating a temporary file for {url} in {staging_directory:?}")
        })?);
    download_to_writer(body, &mut asset_file, digest, url).await?;
    asset_file
        .seek(std::io::SeekFrom::Start(0))
        .await
        .with_context(|| format!("seeking to the start of the temporary file for {url}"))?;

    if asset_kind != AssetKind::Gz {
        async_fs::create_dir(staged_path)
            .await
            .with_context(|| format!("creating directory {staged_path:?}"))?;
    }
    stream_file_archive(asset_file, url, staged_path, asset_kind)
        .await
        .with_context(|| format!("extracting {url} into {staged_path:?}"))
}

async fn download_to_writer(
    body: impl AsyncRead + Unpin,
    writer: &mut (impl AsyncWrite + Unpin),
    digest: Option<&str>,
    url: &str,
) -> Result<()> {
    let mut writer = HashingWriter {
        writer,
        hasher: Sha256::new(),
    };
    futures::io::copy(&mut BufReader::new(body), &mut writer)
        .await
        .with_context(|| format!("saving downloaded contents from {url}"))?;

    if let Some(expected_sha_256) = digest {
        let asset_sha_256 = format!("{:x}", writer.hasher.finalize());
        anyhow::ensure!(
            sha256_matches(&asset_sha_256, expected_sha_256),
            "{url} asset got SHA-256 mismatch. Expected: {expected_sha_256}, Got: {asset_sha_256}",
        );
    }

    Ok(())
}

async fn stream_file_archive(
    file_archive: async_fs::File,
    url: &str,
    destination_path: &Path,
    asset_kind: AssetKind,
) -> Result<()> {
    match asset_kind {
        AssetKind::TarGz => extract_tar_gz(destination_path, url, file_archive).await?,
        AssetKind::TarBz2 => extract_tar_bz2(destination_path, url, file_archive).await?,
        AssetKind::Gz => extract_gz(destination_path, url, file_archive).await?,
        #[cfg(not(windows))]
        AssetKind::Zip => {
            util::archive::extract_seekable_zip(destination_path, file_archive).await?;
        }
        #[cfg(windows)]
        AssetKind::Zip => {
            util::archive::extract_zip(destination_path, file_archive).await?;
        }
    };
    Ok(())
}

async fn extract_tar_gz(
    destination_path: &Path,
    url: &str,
    from: impl AsyncRead + Unpin,
) -> Result<(), anyhow::Error> {
    let decompressed_bytes = GzipDecoder::new(BufReader::new(from));
    unpack_tar_archive(destination_path, url, decompressed_bytes).await?;
    Ok(())
}

async fn extract_tar_bz2(
    destination_path: &Path,
    url: &str,
    from: impl AsyncRead + Unpin,
) -> Result<(), anyhow::Error> {
    let decompressed_bytes = BzDecoder::new(BufReader::new(from));
    unpack_tar_archive(destination_path, url, decompressed_bytes).await?;
    Ok(())
}

async fn unpack_tar_archive(
    destination_path: &Path,
    url: &str,
    archive_bytes: impl AsyncRead + Unpin,
) -> Result<(), anyhow::Error> {
    // We don't need to set the modified time. It's irrelevant to downloaded
    // archive verification, and some filesystems return errors when asked to
    // apply it after extraction.
    let archive = async_tar::ArchiveBuilder::new(archive_bytes)
        .set_preserve_mtime(false)
        .build();
    archive
        .unpack(&destination_path)
        .await
        .with_context(|| format!("extracting {url} to {destination_path:?}"))?;
    Ok(())
}

async fn extract_gz(
    destination_path: &Path,
    url: &str,
    from: impl AsyncRead + Unpin,
) -> Result<(), anyhow::Error> {
    let mut decompressed_bytes = GzipDecoder::new(BufReader::new(from));
    let mut file = async_fs::File::create(&destination_path)
        .await
        .with_context(|| {
            format!("creating a file {destination_path:?} for a download from {url}")
        })?;
    futures::io::copy(&mut decompressed_bytes, &mut file)
        .await
        .with_context(|| format!("extracting {url} to {destination_path:?}"))?;
    Ok(())
}

struct HashingWriter<W: AsyncWrite + Unpin> {
    writer: W,
    hasher: Sha256,
}

impl<W: AsyncWrite + Unpin> AsyncWrite for HashingWriter<W> {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<std::result::Result<usize, std::io::Error>> {
        match Pin::new(&mut self.writer).poll_write(cx, buf) {
            Poll::Ready(Ok(n)) => {
                self.hasher.update(&buf[..n]);
                Poll::Ready(Ok(n))
            }
            other => other,
        }
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.writer).poll_flush(cx)
    }

    fn poll_close(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<std::result::Result<(), std::io::Error>> {
        Pin::new(&mut self.writer).poll_close(cx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::future::BoxFuture;
    use http_client::{AsyncBody, Response, Url, http::HeaderValue};

    struct StaticResponseClient {
        status: u16,
        body: Vec<u8>,
    }

    impl HttpClient for StaticResponseClient {
        fn send(
            &self,
            _req: http_client::Request<AsyncBody>,
        ) -> BoxFuture<'static, anyhow::Result<Response<AsyncBody>>> {
            let status = self.status;
            let body = self.body.clone();
            Box::pin(async move {
                Ok(Response::builder()
                    .status(status)
                    .body(AsyncBody::from(body))
                    .unwrap())
            })
        }

        fn user_agent(&self) -> Option<&HeaderValue> {
            None
        }

        fn proxy(&self) -> Option<&Url> {
            None
        }
    }

    #[gpui::test]
    async fn downloads_raw_binary_with_uppercase_digest_into_destination_dir(
        executor: BackgroundExecutor,
    ) {
        executor.allow_parking();
        let temp_dir = tempfile::tempdir().unwrap();
        let destination_path = temp_dir.path().join("v_1");
        let contents = b"#!/bin/sh\necho hello\n".to_vec();
        let expected_sha_256 = format!("{:X}", Sha256::digest(&contents));
        let client = StaticResponseClient {
            status: 200,
            body: contents,
        };

        download_server_raw_binary(
            &executor,
            Arc::new(client),
            "https://example.com/agent-binary",
            Some(&expected_sha_256),
            &destination_path,
            "agent-binary",
        )
        .await
        .unwrap();

        let binary_path = destination_path.join("agent-binary");
        assert_eq!(
            std::fs::read(&binary_path).unwrap(),
            b"#!/bin/sh\necho hello\n"
        );
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(&binary_path)
                .unwrap()
                .permissions()
                .mode();
            assert_eq!(mode & 0o111, 0o111, "binary should be executable");
        }
    }

    #[gpui::test]
    async fn raw_binary_without_digest_rejects_failed_response(executor: BackgroundExecutor) {
        executor.allow_parking();
        let temp_dir = tempfile::tempdir().unwrap();
        let destination_path = temp_dir.path().join("v_1");
        let client = StaticResponseClient {
            status: 403,
            body: b"rate limit exceeded".to_vec(),
        };

        let error = download_server_raw_binary(
            &executor,
            Arc::new(client),
            "https://example.com/agent-binary",
            None,
            &destination_path,
            "agent-binary",
        )
        .await
        .unwrap_err();

        assert!(error.to_string().contains("403"));
        assert!(!destination_path.exists());
        let leftover_entries = std::fs::read_dir(temp_dir.path()).unwrap().count();
        assert_eq!(
            leftover_entries, 0,
            "no staging directory should be created"
        );
    }

    #[gpui::test]
    async fn raw_binary_digest_mismatch_cleans_up_staging(executor: BackgroundExecutor) {
        executor.allow_parking();
        let temp_dir = tempfile::tempdir().unwrap();
        let destination_path = temp_dir.path().join("v_1");
        let client = StaticResponseClient {
            status: 200,
            body: b"some binary".to_vec(),
        };

        let error = download_server_raw_binary(
            &executor,
            Arc::new(client),
            "https://example.com/agent-binary",
            Some("0000000000000000000000000000000000000000000000000000000000000000"),
            &destination_path,
            "agent-binary",
        )
        .await
        .unwrap_err();

        assert!(error.to_string().contains("SHA-256 mismatch"));
        assert!(!destination_path.exists());
        let leftover_entries = std::fs::read_dir(temp_dir.path()).unwrap().count();
        assert_eq!(leftover_entries, 0, "staging directory should be removed");
    }

    #[gpui::test]
    async fn downloads_archive_with_uppercase_digest_and_extracts_contents(
        executor: BackgroundExecutor,
    ) {
        executor.allow_parking();
        let archive = vec![
            0x50, 0x4b, 0x03, 0x04, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x21, 0x00,
            0x86, 0xa6, 0x10, 0x36, 0x05, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x05, 0x00,
            0x00, 0x00, 0x61, 0x67, 0x65, 0x6e, 0x74, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x50, 0x4b,
            0x01, 0x02, 0x14, 0x03, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x21, 0x00,
            0x86, 0xa6, 0x10, 0x36, 0x05, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x05, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x01, 0x00, 0x00,
            0x00, 0x00, 0x61, 0x67, 0x65, 0x6e, 0x74, 0x50, 0x4b, 0x05, 0x06, 0x00, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x01, 0x00, 0x33, 0x00, 0x00, 0x00, 0x28, 0x00, 0x00, 0x00, 0x00,
            0x00,
        ];
        let expected_sha_256 = format!("{:X}", Sha256::digest(&archive));
        let client = StaticResponseClient {
            status: 200,
            body: archive,
        };
        let temp_dir = tempfile::tempdir().unwrap();
        let destination_path = temp_dir.path().join("v_1");

        download_server_binary(
            &executor,
            Arc::new(client),
            "https://example.com/agent.zip",
            Some(&expected_sha_256),
            &destination_path,
            AssetKind::Zip,
        )
        .await
        .unwrap();

        assert_eq!(
            std::fs::read(destination_path.join("agent")).unwrap(),
            b"hello"
        );
    }

    #[cfg(unix)]
    #[gpui::test]
    async fn replaces_symlinked_destination_without_touching_its_target(
        executor: BackgroundExecutor,
    ) {
        executor.allow_parking();
        let temp_dir = tempfile::tempdir().unwrap();
        let symlink_target = temp_dir.path().join("target");
        std::fs::create_dir(&symlink_target).unwrap();
        std::fs::write(symlink_target.join("existing"), b"keep").unwrap();
        let destination_path = temp_dir.path().join("v_1");
        std::os::unix::fs::symlink(&symlink_target, &destination_path).unwrap();
        let client = StaticResponseClient {
            status: 200,
            body: b"binary".to_vec(),
        };

        download_server_raw_binary(
            &executor,
            Arc::new(client),
            "https://example.com/agent-binary",
            None,
            &destination_path,
            "agent-binary",
        )
        .await
        .unwrap();

        assert!(
            !std::fs::symlink_metadata(&destination_path)
                .unwrap()
                .is_symlink()
        );
        assert_eq!(
            std::fs::read(destination_path.join("agent-binary")).unwrap(),
            b"binary"
        );
        assert_eq!(
            std::fs::read(symlink_target.join("existing")).unwrap(),
            b"keep"
        );
    }

    #[gpui::test]
    async fn archive_digest_mismatch_prevents_extraction_and_cleans_up_staging(
        executor: BackgroundExecutor,
    ) {
        executor.allow_parking();
        let temp_dir = tempfile::tempdir().unwrap();
        let destination_path = temp_dir.path().join("v_1");
        let client = StaticResponseClient {
            status: 200,
            body: b"not an archive".to_vec(),
        };

        let error = download_server_binary(
            &executor,
            Arc::new(client),
            "https://example.com/agent.zip",
            Some("0000000000000000000000000000000000000000000000000000000000000000"),
            &destination_path,
            AssetKind::Zip,
        )
        .await
        .unwrap_err();

        assert!(error.to_string().contains("SHA-256 mismatch"));
        assert!(!destination_path.exists());
        let leftover_entries = std::fs::read_dir(temp_dir.path()).unwrap().count();
        assert_eq!(leftover_entries, 0, "staging directory should be removed");
    }
}
