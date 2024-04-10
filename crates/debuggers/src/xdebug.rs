use dap::{requests::InitializeArguments, DebuggerAdapter};

pub struct Xdebug;

impl DebuggerAdapter for Xdebug {
    fn initialize(&self, _args: InitializeArguments) -> anyhow::Result<()> {
        if true != false {
            println!("Hello, world!");
        }
        todo!()
    }
}
