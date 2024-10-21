use crate::transport::Transport;
use ::fs::Fs;
use anyhow::Result;
use async_trait::async_trait;
use http_client::HttpClient;
use node_runtime::NodeRuntime;
use serde_json::Value;
use std::{collections::HashMap, ffi::OsString, path::Path, sync::Arc};
use task::DebugAdapterConfig;

pub trait DapDelegate {
    fn http_client(&self) -> Option<Arc<dyn HttpClient>>;
    fn node_runtime(&self) -> Option<NodeRuntime>;
    fn fs(&self) -> Arc<dyn Fs>;
}

pub struct DebugAdapterName(pub Arc<str>);

impl AsRef<Path> for DebugAdapterName {
    fn as_ref(&self) -> &Path {
        Path::new(&*self.0)
    }
}

impl std::fmt::Display for DebugAdapterName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Debug, Clone)]
pub struct DebugAdapterBinary {
    pub command: String,
    pub arguments: Option<Vec<OsString>>,
    pub envs: Option<HashMap<String, String>>,
}

#[async_trait(?Send)]
pub trait DebugAdapter: 'static + Send + Sync {
    fn name(&self) -> DebugAdapterName;

    fn transport(&self) -> Box<dyn Transport>;

    /// Installs the binary for the debug adapter.
    /// This method is called when the adapter binary is not found or needs to be updated.
    /// It should download and install the necessary files for the debug adapter to function.
    async fn install_binary(&self, delegate: &dyn DapDelegate) -> Result<()>;

    async fn fetch_binary(
        &self,
        delegate: &dyn DapDelegate,
        config: &DebugAdapterConfig,
    ) -> Result<DebugAdapterBinary>;

    /// Should return base configuration to make the debug adapter work
    fn request_args(&self, config: &DebugAdapterConfig) -> Value;
}
