use anyhow::{anyhow, ensure, Result};
use async_trait::async_trait;
use futures::StreamExt;
pub use language::*;
use lsp::LanguageServerBinary;
use node_runtime::NodeRuntime;
use parking_lot::Mutex;
use serde_json::Value;
use smol::fs::{self};
use std::{
    any::Any,
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::ResultExt;

pub struct AngularLspVersion {
    angular_version: String,
    ts_version: String,
}

pub struct AngularLspAdapter {
    node: Arc<dyn NodeRuntime>,
    typescript_install_path: Mutex<Option<PathBuf>>,
}

impl AngularLspAdapter {
    const SERVER_PATH: &'static str = "node_modules/@angular/language-server/index.js";
    // TODO: this can't be hardcoded, yet we have to figure out how to pass it in initialization_options.
    const TYPESCRIPT_PATH: &'static str = "node_modules/typescript/lib";
    pub fn new(node: Arc<dyn NodeRuntime>) -> Self {
        let typescript_install_path = Mutex::new(None);
        Self {
            node,
            typescript_install_path,
        }
    }
}

#[async_trait]
impl super::LspAdapter for AngularLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("angular-language-server".into())
    }

    fn short_name(&self) -> &'static str {
        "angular"
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        Ok(Box::new(AngularLspVersion {
            angular_version: self
                .node
                .npm_package_latest_version("@angular/language-server")
                .await?,
            ts_version: self.node.npm_package_latest_version("typescript").await?,
        }) as Box<_>)
    }
    fn initialization_options(&self) -> Option<Value> {
        let typescript_sdk_path = self.typescript_install_path.lock();
        let typescript_sdk_path = typescript_sdk_path
            .as_ref()
            .expect("initialization_options called without a container_dir for typescript");

        let mut service_path = typescript_sdk_path.clone();
        service_path.pop();
        service_path.pop();
        service_path.push("@angular/language-service");

        Some(serde_json::json!({
            "typescript": {
                "tsdk": typescript_sdk_path,
            },
            "tsProbeLocations": typescript_sdk_path,
            "ngProbeLocations":service_path,

        }))
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let version = version.downcast::<AngularLspVersion>().unwrap();
        let server_path = container_dir.join(Self::SERVER_PATH);
        let ts_path = container_dir.join(Self::TYPESCRIPT_PATH);

        if fs::metadata(&server_path).await.is_err() {
            self.node
                .npm_install_packages(
                    &container_dir,
                    &[("@angular/language-server", version.angular_version.as_str())],
                )
                .await?;
        }

        ensure!(
            fs::metadata(&server_path).await.is_ok(),
            "@angular/language-server package installation failed"
        );

        if fs::metadata(&ts_path).await.is_err() {
            self.node
                .npm_install_packages(
                    &container_dir,
                    &[("typescript", version.ts_version.as_str())],
                )
                .await?;
        }

        ensure!(
            fs::metadata(&ts_path).await.is_ok(),
            "typescript for Angular package installation failed"
        );
        *self.typescript_install_path.lock() = Some(ts_path.clone());

        Ok(LanguageServerBinary {
            path: self.node.binary_path().await?,
            arguments: angular_server_binary_arguments(&server_path, &ts_path),
        })
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        let (server, ts_path) = get_cached_server_binary(container_dir, self.node.clone()).await?;
        *self.typescript_install_path.lock() = Some(ts_path);
        Some(server)
    }

    async fn installation_test_binary(
        &self,
        container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        let (server, ts_path) = get_cached_server_binary(container_dir, self.node.clone())
            .await
            .map(|(mut binary, ts_path)| {
                let mut service_path = ts_path.clone();
                service_path.pop();
                service_path.pop();
                service_path.push("@angular/language-service");

                binary.arguments = vec![
                    "--stdio".into(),
                    "--tsProbeLocations".into(),
                    ts_path.clone().into(),
                    "--ngProbeLocations".into(),
                    service_path.into(),
                ];
                (binary, ts_path)
            })?;
        *self.typescript_install_path.lock() = Some(ts_path);
        Some(server)
    }
}

fn angular_server_binary_arguments(server_path: &Path, ts_path: &Path) -> Vec<OsString> {
    //TODO this is very hacky way to get the servie directory
    let mut service_path = ts_path.to_path_buf();
    service_path.pop();
    service_path.pop();
    service_path.push("@angular/language-service");
    vec![
        server_path.into(),
        "--stdio".into(),
        "--tsProbeLocations".into(),
        ts_path.into(),
        "--ngProbeLocations".into(),
        service_path.into(),
    ]
}

type TypescriptPath = PathBuf;
async fn get_cached_server_binary(
    container_dir: PathBuf,
    node: Arc<dyn NodeRuntime>,
) -> Option<(LanguageServerBinary, TypescriptPath)> {
    (|| async move {
        let mut last_version_dir = None;
        let mut entries = fs::read_dir(&container_dir).await?;
        while let Some(entry) = entries.next().await {
            let entry = entry?;
            if entry.file_type().await?.is_dir() {
                last_version_dir = Some(entry.path());
            }
        }
        let last_version_dir = last_version_dir.ok_or_else(|| anyhow!("no cached binary"))?;
        let server_path = last_version_dir.join(AngularLspAdapter::SERVER_PATH);
        let typescript_path = last_version_dir.join(AngularLspAdapter::TYPESCRIPT_PATH);
        if server_path.exists() && typescript_path.exists() {
            Ok((
                LanguageServerBinary {
                    path: node.binary_path().await?,
                    arguments: angular_server_binary_arguments(&server_path, &typescript_path),
                },
                typescript_path,
            ))
        } else {
            Err(anyhow!(
                "missing executable in directory {:?}",
                last_version_dir
            ))
        }
    })()
    .await
    .log_err()
}