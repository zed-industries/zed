use dap::requests::InitializeArguments;
pub use dap::*;

pub trait DebuggerAdapter {
    fn initialize(&self, args: InitializeArguments) -> anyhow::Result<()>;
}

// impl dyn DebuggerAdapter {}
