use anyhow::{anyhow, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use async_trait::async_trait;
use futures::{future::BoxFuture, FutureExt};
use gpui::AppContext;
use language::{LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::{CodeActionKind, LanguageServerBinary};
use node_runtime::NodeRuntime;
use serde_json::{json, Value};
use smol::{fs, io::BufReader, stream::StreamExt};
use std::{
    any::Any,
    ffi::OsString,
    future,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::ResultExt;

fn intelephense_server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), "--stdio".into()]
}

pub struct IntelephenseVersion(String);

pub struct IntelephenseLspAdapter {
    node: Arc<NodeRuntime>,
}

impl IntelephenseLspAdapter {
    const SERVER_PATH: &'static str = "node_modules/intelephense/lib/intelephense.js";

    #[allow(unused)]
    pub fn new(node: Arc<NodeRuntime>) -> Self {
        Self { node }
    }
}

#[async_trait]
impl LspAdapter for IntelephenseLspAdapter {
    async fn name(&self) -> LanguageServerName {
        LanguageServerName("intelephense".into())
    }

    async fn fetch_latest_server_version(
        &self,
        _delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        // At the time of writing the latest vscode-eslint release was released in 2020 and requires
        // special custom LSP protocol extensions be handled to fully initialize. Download the latest
        // prerelease instead to sidestep this issue
        dbg!("Strarting fetching server binary version");
        Ok(Box::new(IntelephenseVersion(
            self.node.npm_package_latest_version("intelephense").await?,
        )) as Box<_>)
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        _delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        dbg!("Strarting fetching server binary");
        let version = version.downcast::<IntelephenseVersion>().unwrap();
        let server_path = container_dir.join(Self::SERVER_PATH);

        if fs::metadata(&server_path).await.is_err() {
            self.node
                .npm_install_packages(&container_dir, [("intelephense", version.0.as_str())])
                .await?;
        }
        dbg!("Fetched server binary");
        Ok(LanguageServerBinary {
            path: dbg!(self.node.binary_path().await)?,
            arguments: intelephense_server_binary_arguments(&server_path),
        })
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        dbg!("cached_server_binary");
        get_cached_server_binary(container_dir, &self.node).await
    }

    async fn installation_test_binary(
        &self,
        container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        dbg!("installation_test_binary");
        get_cached_server_binary(container_dir, &self.node).await
    }

    async fn label_for_completion(
        &self,
        _item: &lsp::CompletionItem,
        _language: &Arc<language::Language>,
    ) -> Option<language::CodeLabel> {
        dbg!(_item.kind);
        None
    }

    async fn initialization_options(&self) -> Option<serde_json::Value> {
        dbg!("init_options");
        None
    }
}

async fn get_cached_server_binary(
    container_dir: PathBuf,
    node: &NodeRuntime,
) -> Option<LanguageServerBinary> {
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
        let server_path = last_version_dir.join(IntelephenseLspAdapter::SERVER_PATH);
        if server_path.exists() {
            Ok(LanguageServerBinary {
                path: node.binary_path().await?,
                arguments: intelephense_server_binary_arguments(&server_path),
            })
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

#[cfg(test)]
mod tests {
    use gpui::TestAppContext;
    use unindent::Unindent;

    #[gpui::test]
    async fn test_outline(cx: &mut TestAppContext) {
        let language = crate::languages::language("php", tree_sitter_php::language(), None).await;

        /*let text = r#"
            function a() {
              // local variables are omitted
              let a1 = 1;
              // all functions are included
              async function a2() {}
            }
            // top-level variables are included
            let b: C
            function getB() {}
            // exported variables are included
            export const d = e;
        "#
        .unindent();*/
        let text = r#"
            function a() {
              // local variables are omitted
              $a1 = 1;
              // all functions are included
              function a2() {}
            }
            class Foo {}
        "#
        .unindent();
        let buffer =
            cx.add_model(|cx| language::Buffer::new(0, text, cx).with_language(language, cx));
        let outline = buffer.read_with(cx, |buffer, _| buffer.snapshot().outline(None).unwrap());
        panic!(
            "{:?}",
            outline
                .items
                .iter()
                .map(|item| (item.text.as_str(), item.depth))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            outline
                .items
                .iter()
                .map(|item| (item.text.as_str(), item.depth))
                .collect::<Vec<_>>(),
            &[
                ("function a()", 0),
                ("async function a2()", 1),
                ("let b", 0),
                ("function getB()", 0),
                ("const d", 0),
            ]
        );
    }
}
