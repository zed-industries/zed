use anyhow::{anyhow, Result};
use async_trait::async_trait;
use gpui::AppContext;
use language::{LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::LanguageServerBinary;
use project::project_settings::ProjectSettings;
use serde_json::Value;
use settings::Settings;
use std::{
    any::Any,
    path::{Path, PathBuf},
};

pub struct DartLanguageServer;

#[async_trait]
impl LspAdapter for DartLanguageServer {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("dart".into())
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
        Some(LanguageServerBinary {
            path: "dart".into(),
            env: None,
            arguments: vec!["language-server".into(), "--protocol=lsp".into()],
        })
    }

    fn can_be_reinstalled(&self) -> bool {
        false
    }

    async fn installation_test_binary(&self, _: PathBuf) -> Option<LanguageServerBinary> {
        None
    }

    fn workspace_configuration(&self, _workspace_root: &Path, cx: &mut AppContext) -> Value {
        let settings = ProjectSettings::get_global(cx)
            .lsp
            .get("dart")
            .and_then(|s| s.settings.clone())
            .unwrap_or_default();

        serde_json::json!({
            "dart": settings
        })
    }
}
