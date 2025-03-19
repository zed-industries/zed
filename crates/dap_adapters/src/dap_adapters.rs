mod custom;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod gdb;
mod go;
mod javascript;
mod lldb;
mod php;
mod python;

use std::{collections::HashMap, sync::Arc};

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
use sysinfo::{Pid, Process};
use task::{CustomArgs, DebugAdapterConfig, DebugAdapterKind, DebugConnectionType, TCPHost};

pub async fn build_adapter(kind: &DebugAdapterKind) -> Result<Arc<dyn DebugAdapter>> {
    match kind {
        DebugAdapterKind::Custom(start_args) => {
            Ok(Arc::new(CustomDebugAdapter::new(start_args.clone()).await?))
        }
        DebugAdapterKind::Python(host) => Ok(Arc::new(PythonDebugAdapter::new(host).await?)),
        DebugAdapterKind::Php(host) => Ok(Arc::new(PhpDebugAdapter::new(host.clone()).await?)),
        DebugAdapterKind::Javascript(host) => {
            Ok(Arc::new(JsDebugAdapter::new(host.clone()).await?))
        }
        DebugAdapterKind::Lldb => Ok(Arc::new(LldbDebugAdapter::new())),
        DebugAdapterKind::Go(host) => Ok(Arc::new(GoDebugAdapter::new(host).await?)),
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        DebugAdapterKind::Gdb => Ok(Arc::new(GdbDebugAdapter::new())),
        #[cfg(any(test, feature = "test-support"))]
        DebugAdapterKind::Fake(_) => Ok(Arc::new(dap::adapters::FakeAdapter::new())),
        #[cfg(not(any(test, feature = "test-support")))]
        #[allow(unreachable_patterns)]
        _ => unreachable!("Fake variant only exists with test-support feature"),
    }
}

pub fn attach_processes<'a>(
    kind: &DebugAdapterKind,
    processes: &'a HashMap<Pid, Process>,
) -> Vec<(&'a Pid, &'a Process)> {
    match kind {
        #[cfg(any(test, feature = "test-support"))]
        DebugAdapterKind::Fake(_) => processes
            .iter()
            .filter(|(pid, _)| pid.as_u32() == std::process::id())
            .collect::<Vec<_>>(),
        DebugAdapterKind::Custom(_) => CustomDebugAdapter::attach_processes(processes),
        DebugAdapterKind::Javascript(_) => JsDebugAdapter::attach_processes(processes),
        DebugAdapterKind::Lldb => LldbDebugAdapter::attach_processes(processes),
        _ => processes.iter().collect::<Vec<_>>(),
    }
}
