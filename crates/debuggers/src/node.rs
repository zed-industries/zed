use dap::DebuggerAdapter;

pub struct JsAdapter;

impl DebuggerAdapter for JsAdapter {
    fn initialize(&self, args: dap::requests::InitializeArguments) -> anyhow::Result<()> {
        todo!()
    }
}
