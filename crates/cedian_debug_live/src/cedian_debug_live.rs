//! S1's live evidence (ARCHITECTURE §22): the Command Line Tools `lldb-dap`
//! as a Zed debug adapter, so `cedian_debug` can be proven on a real
//! adapter. Test-only: the product ships Zed's own adapters (CodeLLDB and
//! the rest), never this one. Its own crate because Zed's `dap` swaps every
//! transport for a fake one when its `test-support` feature is on, which
//! `cedian_panel`'s tests turn on.

use anyhow::{Context as _, Result};
use async_trait::async_trait;
use collections::HashMap;
use dap::StartDebuggingRequestArguments;
use dap::adapters::{
    DapDelegate, DebugAdapter, DebugAdapterBinary, DebugAdapterName, DebugTaskDefinition,
};
use gpui::{App, AsyncApp};
use std::path::PathBuf;
use std::sync::Arc;
use task::{DebugScenario, ZedDebugConfig};

pub const ADAPTER: &str = "lldb-dap";

/// Where the Command Line Tools keep `lldb-dap`, through `xcrun`. Called
/// once, before any app runs.
#[expect(clippy::disallowed_methods, reason = "once, before the app starts")]
pub fn lldb_dap() -> Option<PathBuf> {
    let out = std::process::Command::new("xcrun")
        .args(["-f", "lldb-dap"])
        .output()
        .ok()?;
    let path = PathBuf::from(String::from_utf8(out.stdout).ok()?.trim());
    (out.status.success() && path.is_file()).then_some(path)
}

/// Whether macOS lets this user debug without an authorization dialog
/// (Developer mode). Without it `lldb-dap` never answers `launch` in an
/// unattended run.
#[expect(clippy::disallowed_methods, reason = "once, before the app starts")]
pub fn debugger_authorized() -> bool {
    std::process::Command::new("DevToolsSecurity")
        .arg("-status")
        .output()
        .is_ok_and(|out| String::from_utf8_lossy(&out.stdout).contains("enabled"))
}

/// Register the adapter in Zed's registry.
pub fn register(binary: PathBuf, cx: &mut App) {
    dap::DapRegistry::global(cx).add_adapter(Arc::new(LldbDap { binary }));
}

struct LldbDap {
    binary: PathBuf,
}

#[async_trait(?Send)]
impl DebugAdapter for LldbDap {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(ADAPTER.into())
    }

    async fn config_from_zed_format(&self, zed: ZedDebugConfig) -> Result<DebugScenario> {
        let dap::DebugRequest::Launch(launch) = zed.request else {
            anyhow::bail!("{ADAPTER}: only launch");
        };
        Ok(DebugScenario {
            adapter: zed.adapter,
            label: zed.label,
            build: None,
            config: serde_json::json!({"request": "launch", "program": launch.program}),
            tcp_connection: None,
        })
    }

    async fn get_binary(
        &self,
        delegate: &Arc<dyn DapDelegate>,
        config: &DebugTaskDefinition,
        _: Option<PathBuf>,
        _: Option<Vec<String>>,
        _: Option<HashMap<String, String>>,
        _: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        let command = self.binary.to_str().context("lldb-dap path")?.to_string();
        Ok(DebugAdapterBinary {
            command: Some(command),
            arguments: Vec::new(),
            envs: HashMap::default(),
            cwd: Some(delegate.worktree_root_path().to_path_buf()),
            connection: None,
            request_args: StartDebuggingRequestArguments {
                request: self.request_kind(&config.config).await?,
                configuration: config.config.clone(),
            },
        })
    }

    fn dap_schema(&self) -> serde_json::Value {
        serde_json::json!({})
    }
}
