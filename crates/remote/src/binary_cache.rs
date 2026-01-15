use anyhow::{Context as _, Result};
use gpui::AsyncApp;
use release_channel::ReleaseChannel;
use semver::Version;
use smol::fs;
use std::path::PathBuf;

use crate::{RemoteClientDelegate, RemotePlatform};

pub(crate) fn expected_remote_server_version(
    release_channel: ReleaseChannel,
    version: &Version,
) -> Option<String> {
    match release_channel {
        ReleaseChannel::Stable | ReleaseChannel::Preview => Some(version.to_string()),
        ReleaseChannel::Nightly | ReleaseChannel::Dev => None,
    }
}

pub(crate) async fn get_or_download_server_binary(
    delegate: &dyn RemoteClientDelegate,
    platform: RemotePlatform,
    release_channel: ReleaseChannel,
    wanted_version: Option<Version>,
    cache_version: &Version,
    cx: &mut AsyncApp,
) -> Result<PathBuf> {
    let cache_dir = paths::remote_server_cache_dir().clone();
    fs::create_dir_all(&cache_dir)
        .await
        .context("creating remote server cache directory")?;

    let cache_path = cache_dir.join(cache_file_name(
        platform,
        release_channel,
        &wanted_version,
        cache_version,
    ));

    if let Ok(metadata) = fs::metadata(&cache_path).await {
        if metadata.len() > 0 {
            return Ok(cache_path);
        }
    }

    let downloaded_path = delegate
        .download_server_binary_locally(platform, release_channel, wanted_version, cx)
        .await
        .context("downloading remote server binary")?;

    if downloaded_path != cache_path {
        fs::copy(&downloaded_path, &cache_path)
            .await
            .context("copying remote server binary to cache")?;
    }

    Ok(cache_path)
}

fn cache_file_name(
    platform: RemotePlatform,
    release_channel: ReleaseChannel,
    wanted_version: &Option<Version>,
    cache_version: &Version,
) -> String {
    let version_tag = wanted_version
        .as_ref()
        .map(|version| version.to_string())
        .unwrap_or_else(|| format!("latest-{}", cache_version));
    format!(
        "zed-remote-server-{}-{}-{}-{}.gz",
        release_channel.dev_name(),
        platform.os.as_str(),
        platform.arch.as_str(),
        version_tag
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_file_name_versioned() {
        let version = Version::new(1, 2, 3);
        let name = cache_file_name(
            RemotePlatform {
                os: crate::RemoteOs::Linux,
                arch: crate::RemoteArch::X86_64,
            },
            ReleaseChannel::Stable,
            &Some(version.clone()),
            &version,
        );

        assert!(name.contains("stable"));
        assert!(name.contains("linux"));
        assert!(name.contains("x86_64"));
        assert!(name.contains("1.2.3"));
    }
}
