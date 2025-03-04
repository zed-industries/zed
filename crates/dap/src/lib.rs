pub mod adapters;
pub mod client;
pub mod debugger_settings;
pub mod proto_conversions;
pub mod transport;

pub use dap_types::*;
pub use task::{DebugAdapterConfig, DebugAdapterKind, DebugRequestType};

pub type ScopeId = u64;
pub type VariableReference = u64;
pub type StackFrameId = u64;

#[cfg(any(test, feature = "test-support"))]
pub use adapters::FakeAdapter;

#[cfg(any(test, feature = "test-support"))]
pub fn test_config() -> DebugAdapterConfig {
    DebugAdapterConfig {
        label: "test config".into(),
        kind: DebugAdapterKind::Fake,
        request: DebugRequestType::Launch,
        program: None,
        supports_attach: false,
        cwd: None,
        initialize_args: None,
    }
}
