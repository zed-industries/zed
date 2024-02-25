use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use futures::StreamExt;
use language::{LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::LanguageServerBinary;
use smol::fs::{self, File};
use std::{
    any::Any, 
    path::PathBuf
};
use std::ffi::OsString;
use std::sync::Arc;
use util::async_maybe;
use gpui::{AsyncAppContext, Task};
use util::github::latest_github_release;
use util::{github::GitHubLspBinaryVersion, ResultExt};

pub struct SmithyLspAdapter;

fn server_binary_arguments() -> Vec<OsString> {
    vec!["0".into()] // "0" to use STDIN and STDOUT.
}

#[async_trait]
impl LspAdapter for SmithyLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("smithy-language-server".into())
    }

    fn short_name(&self) -> &'static str {
        "smithy-language-server"
    }

    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        let release =
            latest_github_release("smithy-lang/smithy-language-server", true, false, delegate.http_client()).await?;
        let asset_name = format!("smithy-language-server-{}.zip",release.tag_name);
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| anyhow!("no asset found matching {:?}", asset_name))?;
        let version = GitHubLspBinaryVersion {
            name: release.tag_name,
            url: asset.browser_download_url.clone(),
        };

        Ok(Box::new(version) as Box<_>)
    }

    fn check_if_user_installed(
        &self,
        delegate: &Arc<dyn LspAdapterDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Option<Task<Option<LanguageServerBinary>>> {
        let delegate = delegate.clone();

        Some(cx.spawn(|cx| async move {
            match cx.update(|cx| delegate.which_command(OsString::from("smithy-language-server"), cx)) {
                Ok(task) => task.await.map(|(path, env)| LanguageServerBinary {
                    path,
                    arguments: server_binary_arguments(),
                    env: Some(env),
                }),
                Err(_) => None,
            }
        }))
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let version = version.downcast::<GitHubLspBinaryVersion>().unwrap();
        let zip_path = container_dir.join(format!("smithy-language-server-{}.zip", version.name));
        let version_dir = container_dir.join(format!("smithy-language-server-{}", version.name));
        let binary_path: PathBuf = version_dir.join("bin/smithy-language-server");

        
        if fs::metadata(&binary_path).await.is_err() {
            let mut response = delegate
                .http_client()
                .get(&version.url, Default::default(), true)
                .await
                .context("error downloading smithy-language-server release")?;
            let mut file = File::create(&zip_path).await?;
            if !response.status().is_success() {
                Err(anyhow!(
                    "smithy-language-server download failed with status {}",
                    response.status().to_string()
                ))?;
            }
            futures::io::copy(response.body_mut(), &mut file).await?;

            let unzip_status = smol::process::Command::new("unzip")
                .current_dir(&container_dir)
                .arg(&zip_path)
                .output()
                .await?
                .status;
            if !unzip_status.success() {
                Err(anyhow!("failed to unzip smithy-language-server archive"))?;
            }
        }
        Ok(LanguageServerBinary {
            path: binary_path.clone(),
            env: None,
            arguments: server_binary_arguments(),
        })
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        get_cached_server_binary(container_dir).await
    }

    async fn installation_test_binary(
        &self,
        container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        get_cached_server_binary(container_dir)
            .await
            .map(|mut binary| {
                binary.arguments = vec!["--help".into()];
                binary
            })
    }
}

async fn get_cached_server_binary(container_dir: PathBuf) -> Option<LanguageServerBinary> {
    async_maybe!({
        let mut last_smithy_dir = None;
        let mut entries = fs::read_dir(&container_dir).await?;
        while let Some(entry) = entries.next().await {
            let entry = entry?;
            if entry.file_type().await?.is_dir() {
                last_smithy_dir = Some(entry.path());
            }
        }
        let smithy_dir = last_smithy_dir.ok_or_else(|| anyhow!("no cached binary"))?;
        let smithy_bin = smithy_dir.join("bin/smithy-language-server");
        if smithy_bin.exists() {
            Ok(LanguageServerBinary {
                path: smithy_bin.clone(),
                env: None,
                arguments: server_binary_arguments(), 
            })
        } else {
            Err(anyhow!(
                "missing smithy-language-server binary in directory {:?}",
                smithy_dir
            ))
        }
    })
    .await
    .log_err()
}