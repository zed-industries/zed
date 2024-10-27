mod custom;
mod javascript;
mod lldb;
mod php;
mod python;

use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use custom::CustomDebugAdapter;
use dap::adapters::{
    self, AdapterVersion, DapDelegate, DebugAdapter, DebugAdapterBinary, DebugAdapterName,
    GithubRepo,
};
use javascript::JsDebugAdapter;
use lldb::LldbDebugAdapter;
use php::PhpDebugAdapter;
use python::PythonDebugAdapter;
use serde_json::{json, Value};
use task::{CustomArgs, DebugAdapterConfig, DebugAdapterKind, DebugConnectionType, TCPHost};

pub async fn build_adapter(adapter_config: &DebugAdapterConfig) -> Result<Box<dyn DebugAdapter>> {
    match &adapter_config.kind {
        DebugAdapterKind::Custom(start_args) => {
            Ok(Box::new(CustomDebugAdapter::new(start_args.clone()).await?))
        }
        DebugAdapterKind::Python => Ok(Box::new(PythonDebugAdapter::new())),
        DebugAdapterKind::PHP(host) => Ok(Box::new(PhpDebugAdapter::new(host.clone()).await?)),
        DebugAdapterKind::Javascript(host) => {
            Ok(Box::new(JsDebugAdapter::new(host.clone()).await?))
        }
        DebugAdapterKind::Lldb => Ok(Box::new(LldbDebugAdapter::new())),
    }
}
