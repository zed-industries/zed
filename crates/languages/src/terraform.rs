use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use collections::HashMap;
use futures::StreamExt;
pub use language::*;
use lsp::{CodeActionKind, LanguageServerBinary};
use smol::fs::{self, File};
use std::{any::Any, ffi::OsString, path::PathBuf};
use util::{
    async_maybe,
    fs::remove_matching,
    github::{latest_github_release, GitHubLspBinaryVersion},
    ResultExt,
};

fn terraform_ls_binary_arguments() -> Vec<OsString> {
    vec!["serve".into()]
}

pub struct TerraformLspAdapter;

#[async_trait]
impl LspAdapter for TerraformLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("terraform-ls".into())
    }

    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        // TODO: maybe use release API instead
        // https://api.releases.hashicorp.com/v1/releases/terraform-ls?limit=1
        let release = latest_github_release(
            "hashicorp/terraform-ls",
            false,
            false,
            delegate.http_client(),
        )
        .await?;

        Ok(Box::new(GitHubLspBinaryVersion {
            name: release.tag_name,
            url: Default::default(),
        }))
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let version = version.downcast::<GitHubLspBinaryVersion>().unwrap();
        let zip_path = container_dir.join(format!("terraform-ls_{}.zip", version.name));
        let version_dir = container_dir.join(format!("terraform-ls_{}", version.name));
        let binary_path = version_dir.join("terraform-ls");
        let url = build_download_url(version.name)?;

        if fs::metadata(&binary_path).await.is_err() {
            let mut response = delegate
                .http_client()
                .get(&url, Default::default(), true)
                .await
                .context("error downloading release")?;
            let mut file = File::create(&zip_path).await?;
            if !response.status().is_success() {
                Err(anyhow!(
                    "download failed with status {}",
                    response.status().to_string()
                ))?;
            }
            futures::io::copy(response.body_mut(), &mut file).await?;

            let unzip_status = smol::process::Command::new("unzip")
                .current_dir(&container_dir)
                .arg(&zip_path)
                .arg("-d")
                .arg(&version_dir)
                .output()
                .await?
                .status;
            if !unzip_status.success() {
                Err(anyhow!("failed to unzip Terraform LS archive"))?;
            }

            remove_matching(&container_dir, |entry| entry != version_dir).await;
        }

        Ok(LanguageServerBinary {
            path: binary_path,
            env: None,
            arguments: terraform_ls_binary_arguments(),
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
                binary.arguments = vec!["version".into()];
                binary
            })
    }

    fn code_action_kinds(&self) -> Option<Vec<CodeActionKind>> {
        // TODO: file issue for server supported code actions
        // TODO: reenable default actions / delete override
        Some(vec![])
    }

    fn language_ids(&self) -> HashMap<String, String> {
        HashMap::from_iter([
            ("Terraform".into(), "terraform".into()),
            ("Terraform Vars".into(), "terraform-vars".into()),
        ])
    }
}

fn build_download_url(version: String) -> Result<String> {
    let v = version.strip_prefix('v').unwrap_or(&version);
    let os = match std::env::consts::OS {
        "linux" => "linux",
        "macos" => "darwin",
        "win" => "windows",
        _ => Err(anyhow!("unsupported OS {}", std::env::consts::OS))?,
    }
    .to_string();
    let arch = match std::env::consts::ARCH {
        "x86" => "386",
        "x86_64" => "amd64",
        "arm" => "arm",
        "aarch64" => "arm64",
        _ => Err(anyhow!("unsupported ARCH {}", std::env::consts::ARCH))?,
    }
    .to_string();

    let url = format!(
        "https://releases.hashicorp.com/terraform-ls/{v}/terraform-ls_{v}_{os}_{arch}.zip",
    );

    Ok(url)
}

async fn get_cached_server_binary(container_dir: PathBuf) -> Option<LanguageServerBinary> {
    async_maybe!({
        let mut last = None;
        let mut entries = fs::read_dir(&container_dir).await?;
        while let Some(entry) = entries.next().await {
            last = Some(entry?.path());
        }

        match last {
            Some(path) if path.is_dir() => {
                let binary = path.join("terraform-ls");
                if fs::metadata(&binary).await.is_ok() {
                    return Ok(LanguageServerBinary {
                        path: binary,
                        env: None,
                        arguments: terraform_ls_binary_arguments(),
                    });
                }
            }
            _ => {}
        }

        Err(anyhow!("no cached binary"))
    })
    .await
    .log_err()
}
