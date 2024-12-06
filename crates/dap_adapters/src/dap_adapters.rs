mod custom;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod gdb;
mod go;
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
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use gdb::GdbDebugAdapter;
use go::GoDebugAdapter;
use javascript::JsDebugAdapter;
use lldb::LldbDebugAdapter;
use php::PhpDebugAdapter;
use python::PythonDebugAdapter;
use serde_json::{json, Value};
use task::{CustomArgs, DebugAdapterConfig, DebugAdapterKind, DebugConnectionType, TCPHost};

pub async fn build_adapter(kind: &DebugAdapterKind) -> Result<Box<dyn DebugAdapter>> {
    match &kind {
        DebugAdapterKind::Custom(start_args) => {
            Ok(Box::new(CustomDebugAdapter::new(start_args.clone()).await?))
        }
        DebugAdapterKind::Python(host) => Ok(Box::new(PythonDebugAdapter::new(host).await?)),
        DebugAdapterKind::Php(host) => Ok(Box::new(PhpDebugAdapter::new(host.clone()).await?)),
        DebugAdapterKind::Javascript(host) => {
            Ok(Box::new(JsDebugAdapter::new(host.clone()).await?))
        }
        DebugAdapterKind::Lldb => Ok(Box::new(LldbDebugAdapter::new())),
        DebugAdapterKind::Go(host) => Ok(Box::new(GoDebugAdapter::new(host).await?)),
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        DebugAdapterKind::Gdb => Ok(Box::new(GdbDebugAdapter::new())),
    }
}
