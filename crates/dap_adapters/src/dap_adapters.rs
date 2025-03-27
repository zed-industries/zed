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
use task::{DebugAdapterConfig, DebugAdapterKind};

pub fn build_adapter(kind: &DebugAdapterKind) -> Arc<dyn DebugAdapter> {
    match kind {
        DebugAdapterKind::Python => Arc::new(PythonDebugAdapter::default()),
        DebugAdapterKind::Php => Arc::new(PhpDebugAdapter::default()),
        DebugAdapterKind::Javascript => Arc::new(JsDebugAdapter::default()),
        DebugAdapterKind::Lldb => Arc::new(LldbDebugAdapter::default()),
        DebugAdapterKind::Go => Arc::new(GoDebugAdapter::default()),
        DebugAdapterKind::Gdb => Arc::new(GdbDebugAdapter::default()),
    }
}

pub fn attach_processes<'a>(
    kind: &DebugAdapterKind,
    processes: &'a HashMap<Pid, Process>,
) -> Vec<(&'a Pid, &'a Process)> {
    match kind {
        DebugAdapterKind::Javascript => JsDebugAdapter::attach_processes(processes),
        DebugAdapterKind::Lldb => LldbDebugAdapter::attach_processes(processes),
        _ => processes.iter().collect::<Vec<_>>(),
    }
}
