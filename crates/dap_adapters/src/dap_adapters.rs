mod custom;
mod javascript;
mod lldb;
mod php;
mod python;

use custom::CustomDebugAdapter;
use lldb::LldbDebugAdapter;
use php::PhpDebugAdapter;
use python::PythonDebugAdapter;

use anyhow::{anyhow, bail, Context, Result};
use async_trait::async_trait;
use dap::{
    adapters::{
        create_stdio_client, create_tcp_client, DapDelegate, DebugAdapter, DebugAdapterBinary,
        DebugAdapterName,
    },
    client::TransportParams,
};
use gpui::AsyncAppContext;
use http_client::github::latest_github_release;
use serde_json::{json, Value};
use smol::{
    fs::{self, File},
    process,
};
use std::{fmt::Debug, process::Stdio};
use task::{CustomArgs, DebugAdapterConfig, DebugAdapterKind, DebugConnectionType, TCPHost};

pub fn build_adapter(adapter_config: &DebugAdapterConfig) -> Result<Box<dyn DebugAdapter>> {
    match &adapter_config.kind {
        DebugAdapterKind::Custom(start_args) => Ok(Box::new(CustomDebugAdapter::new(
            adapter_config,
            start_args.clone(),
        ))),
        DebugAdapterKind::Python => Ok(Box::new(PythonDebugAdapter::new(adapter_config))),
        DebugAdapterKind::PHP => Ok(Box::new(PhpDebugAdapter::new(adapter_config))),
        DebugAdapterKind::Lldb => Ok(Box::new(LldbDebugAdapter::new(adapter_config))),
    }
}
