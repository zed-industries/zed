mod codelldb;
mod gdb;
mod go;
mod javascript;
mod php;
mod python;
mod ruby;

use std::sync::Arc;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use codelldb::CodeLldbDebugAdapter;
use dap::{
    DapRegistry, DebugRequest,
    adapters::{
        self, AdapterVersion, DapDelegate, DebugAdapter, DebugAdapterBinary, DebugAdapterName,
        GithubRepo,
    },
    configure_tcp_connection,
    inline_value::{PythonInlineValueProvider, RustInlineValueProvider},
};
use gdb::GdbDebugAdapter;
use go::GoDebugAdapter;
use gpui::{App, BorrowAppContext};
use javascript::JsDebugAdapter;
use php::PhpDebugAdapter;
use python::PythonDebugAdapter;
use ruby::RubyDebugAdapter;
use serde_json::{Value, json};

pub fn init(cx: &mut App) {
    cx.update_default_global(|registry: &mut DapRegistry, _cx| {
        registry.add_adapter(Arc::from(CodeLldbDebugAdapter::default()));
        registry.add_adapter(Arc::from(PythonDebugAdapter::default()));
        registry.add_adapter(Arc::from(PhpDebugAdapter::default()));
        registry.add_adapter(Arc::from(JsDebugAdapter::default()));
        registry.add_adapter(Arc::from(RubyDebugAdapter));
        registry.add_adapter(Arc::from(GoDebugAdapter));
        registry.add_adapter(Arc::from(GdbDebugAdapter));

        registry.add_inline_value_provider("Rust".to_string(), Arc::from(RustInlineValueProvider));
        registry
            .add_inline_value_provider("Python".to_string(), Arc::from(PythonInlineValueProvider));
    })
}

trait ToDap {
    fn to_dap(&self) -> dap::StartDebuggingRequestArgumentsRequest;
}

impl ToDap for DebugRequest {
    fn to_dap(&self) -> dap::StartDebuggingRequestArgumentsRequest {
        match self {
            Self::Launch(_) => dap::StartDebuggingRequestArgumentsRequest::Launch,
            Self::Attach(_) => dap::StartDebuggingRequestArgumentsRequest::Attach,
        }
    }
}
