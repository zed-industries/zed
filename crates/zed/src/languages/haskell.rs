use anyhow::Result;
use async_trait::async_trait;
use language::{LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::LanguageServerBinary;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::Settings;
use std::ops::Deref;
use std::{any::Any, path::PathBuf};

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct HaskellSettings {
    pub lsp: HaskellLspSetting,
}

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HaskellLspSetting {
    None,
    Local {
        path: String,
        arguments: Vec<String>,
    },
}

#[derive(Clone, Serialize, Default, Deserialize, JsonSchema)]
pub struct HaskellSettingsContent {
    lsp: Option<HaskellLspSetting>,
}

impl Settings for HaskellSettings {
    const KEY: Option<&'static str> = Some("haskell");

    type FileContent = HaskellSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &mut gpui::AppContext,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        Self::load_via_json_merge(default_value, user_values)
    }
}

pub struct LocalLspAdapter {
    pub path: String,
    pub arguments: Vec<String>,
}

#[async_trait]
impl LspAdapter for LocalLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("local-hls".into())
    }

    fn short_name(&self) -> &'static str {
        "local-hls"
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        Ok(Box::new(()) as Box<_>)
    }

    async fn fetch_server_binary(
        &self,
        _: Box<dyn 'static + Send + Any>,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let path = shellexpand::full(&self.path)?;
        Ok(LanguageServerBinary {
            path: PathBuf::from(path.deref()),
            arguments: self.arguments.iter().map(|arg| arg.into()).collect(),
        })
    }

    async fn cached_server_binary(
        &self,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        let path = shellexpand::full(&self.path).ok()?;
        Some(LanguageServerBinary {
            path: PathBuf::from(path.deref()),
            arguments: self.arguments.iter().map(|arg| arg.into()).collect(),
        })
    }

    async fn installation_test_binary(&self, _: PathBuf) -> Option<LanguageServerBinary> {
        let path = shellexpand::full(&self.path).ok()?;
        Some(LanguageServerBinary {
            path: PathBuf::from(path.deref()),
            arguments: self.arguments.iter().map(|arg| arg.into()).collect(),
        })
    }
}
