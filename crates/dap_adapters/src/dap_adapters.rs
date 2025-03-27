mod gdb;
mod go;
mod javascript;
mod lldb;
mod php;
mod python;

use std::{collections::HashMap, sync::Arc};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use dap::{
    adapters::{
        self, AdapterVersion, DapDelegate, DebugAdapter, DebugAdapterBinary, DebugAdapterName,
        GithubRepo,
    },
    DapRegistry,
};
use gdb::GdbDebugAdapter;
use go::GoDebugAdapter;
use javascript::JsDebugAdapter;
use lldb::LldbDebugAdapter;
use php::PhpDebugAdapter;
use python::PythonDebugAdapter;
use serde_json::{json, Value};
use sysinfo::{Pid, Process};
use task::DebugAdapterConfig;

pub fn init(registry: Arc<DapRegistry>) {
    registry.add_adapter(Arc::from(PythonDebugAdapter::default()));
    registry.add_adapter(Arc::from(PhpDebugAdapter::default()));
    registry.add_adapter(Arc::from(JsDebugAdapter::default()));
    registry.add_adapter(Arc::from(LldbDebugAdapter::default()));
    registry.add_adapter(Arc::from(GoDebugAdapter::default()));
    registry.add_adapter(Arc::from(GdbDebugAdapter::default()));
}
