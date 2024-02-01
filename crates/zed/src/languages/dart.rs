use anyhow::{anyhow, Result};
use async_trait::async_trait;
use language::{LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::LanguageServerBinary;
use std::{any::Any, path::PathBuf};
// use util::ResultExt;
use futures::{StreamExt};

pub struct DartLanguageServer;

#[async_trait]
impl LspAdapter for DartLanguageServer {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("dart".into())
    }

    fn short_name(&self) -> &'static str {
        "dart"
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        Ok(Box::new(()))
    }

    async fn fetch_server_binary(
        &self,
        _: Box<dyn 'static + Send + Any>,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        Err(anyhow!("dart must me installed from dart.dev/get-dart"))
    }

    async fn cached_server_binary(
        &self,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        // get_cached_server_binary(container_dir).await

        // println!("{:?}", container_dir);
        Some(LanguageServerBinary {
            path: "dart".into(),
            arguments: vec!["language-server".into(), "--protocol=lsp".into()],
        })
        // Some(LanguageServerBinary {
        //     path: "typescript-language-server".into(),
        //     arguments: vec!["--stdio".into()],
        // })
    }

    fn can_be_reinstalled(&self) -> bool {
        false
    }

    async fn installation_test_binary(&self, _: PathBuf) -> Option<LanguageServerBinary> {
        None
        //     get_cached_server_binary(container_dir)
        //         .await
        //         .map(|mut binary| {
        //             binary.arguments = vec!["--help".into()];
        //             binary
        //         })
    }
}

// async fn get_cached_server_binary(container_dir: PathBuf) -> Option<LanguageServerBinary> {
//     (|| async move {
//         let mut last = None;
//         let mut entries = smol::fs::read_dir(&container_dir).await?;
//         while let Some(entry) = entries.next().await {
//             last = Some(entry?.path());
//         }
//
//         anyhow::Ok(LanguageServerBinary {
//             path: last.ok_or_else(|| anyhow!("no cached binary"))?,
//             arguments: Default::default(),
//         })
//     })()
//         .await
//         .log_err()
// }

