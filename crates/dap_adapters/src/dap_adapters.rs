mod gdb;
mod go;
mod javascript;
mod lldb;
mod php;
mod python;

use std::{collections::HashMap, sync::Arc};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use dap::adapters::{
    self, AdapterVersion, DapDelegate, DebugAdapter, DebugAdapterBinary, DebugAdapterName,
    GithubRepo,
};
use gdb::GdbDebugAdapter;
use go::GoDebugAdapter;
use javascript::JsDebugAdapter;
use lldb::LldbDebugAdapter;
use php::PhpDebugAdapter;
use python::PythonDebugAdapter;
use serde_json::{json, Value};
use sysinfo::{Pid, Process};
use task::{DebugAdapterConfig, DebugAdapterKind, TCPHost};

pub async fn build_adapter(kind: &DebugAdapterKind) -> Result<Arc<dyn DebugAdapter>> {
    match kind {
        DebugAdapterKind::Python(host) => Ok(Arc::new(PythonDebugAdapter::new(host).await?)),
        DebugAdapterKind::Php(host) => Ok(Arc::new(PhpDebugAdapter::new(host.clone()).await?)),
        DebugAdapterKind::Javascript(host) => {
            Ok(Arc::new(JsDebugAdapter::new(host.clone()).await?))
        }
        DebugAdapterKind::Lldb => Ok(Arc::new(LldbDebugAdapter::new())),
        DebugAdapterKind::Go(host) => Ok(Arc::new(GoDebugAdapter::new(host).await?)),
        DebugAdapterKind::Gdb => Ok(Arc::new(GdbDebugAdapter::new())),
    }
}

pub fn attach_processes<'a>(
    kind: &DebugAdapterKind,
    processes: &'a HashMap<Pid, Process>,
) -> Vec<(&'a Pid, &'a Process)> {
    match kind {
        DebugAdapterKind::Javascript(_) => JsDebugAdapter::attach_processes(processes),
        DebugAdapterKind::Lldb => LldbDebugAdapter::attach_processes(processes),
        _ => processes.iter().collect::<Vec<_>>(),
    }
}
