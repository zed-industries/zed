mod codelldb;
mod gdb;
mod go;
mod javascript;
mod php;
mod python;

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use codelldb::CodeLldbDebugAdapter;
use dap::{
    DapRegistry,
    adapters::{
        self, AdapterVersion, DapDelegate, DebugAdapter, DebugAdapterBinary, DebugAdapterName,
        GithubRepo,
    },
    configure_tcp_connection,
};
use gdb::GdbDebugAdapter;
use go::GoDebugAdapter;
use gpui::{App, BorrowAppContext};
pub use javascript::JsDebugAdapter;
use php::PhpDebugAdapter;
use python::PythonDebugAdapter;
use serde_json::json;
use task::{DebugScenario, ZedDebugConfig};

pub fn init(cx: &mut App) {
    cx.update_default_global(|registry: &mut DapRegistry, _cx| {
        registry.add_adapter(Arc::from(CodeLldbDebugAdapter::default()));
        registry.add_adapter(Arc::from(PythonDebugAdapter::default()));
        registry.add_adapter(Arc::from(PhpDebugAdapter::default()));
        registry.add_adapter(Arc::from(JsDebugAdapter::default()));
        registry.add_adapter(Arc::from(GoDebugAdapter::default()));
        registry.add_adapter(Arc::from(GdbDebugAdapter));

        #[cfg(any(test, feature = "test-support"))]
        {
            registry.add_adapter(Arc::from(dap::FakeAdapter {}));
        }
    })
}

#[cfg(feature = "update-schemas")]
struct UpdateSchemasDapDelegate {
    client: std::sync::Arc<reqwest_client::ReqwestClient>,
    fs: std::sync::Arc<fs::RealFs>,
}

#[cfg(feature = "update-schemas")]
impl UpdateSchemasDapDelegate {
    fn new(executor: gpui::BackgroundExecutor) -> Self {
        // FIXME
        let client = Arc::new(reqwest_client::ReqwestClient::user_agent("Cole").unwrap());
        let fs = Arc::new(fs::RealFs::new(None, executor.clone()));
        Self { client, fs }
    }
}

#[cfg(feature = "update-schemas")]
#[async_trait]
impl dap::adapters::DapDelegate for UpdateSchemasDapDelegate {
    fn worktree_id(&self) -> settings::WorktreeId {
        unreachable!()
    }
    fn worktree_root_path(&self) -> &std::path::Path {
        unreachable!()
    }
    fn http_client(&self) -> Arc<dyn dap::adapters::HttpClient> {
        self.client.clone()
    }
    fn node_runtime(&self) -> node_runtime::NodeRuntime {
        unreachable!()
    }
    fn toolchain_store(&self) -> Arc<dyn language::LanguageToolchainStore> {
        unreachable!()
    }
    fn fs(&self) -> Arc<dyn fs::Fs> {
        self.fs.clone()
    }
    fn output_to_console(&self, _msg: String) {}
    async fn which(&self, _command: &std::ffi::OsStr) -> Option<std::path::PathBuf> {
        unreachable!()
    }
    async fn read_text_file(&self, _path: std::path::PathBuf) -> Result<String> {
        unreachable!()
    }
    async fn shell_env(&self) -> collections::HashMap<String, String> {
        unreachable!()
    }
}
