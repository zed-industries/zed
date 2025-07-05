use std::path::Path;

use dap_adapters::JsDebugAdapter;
use gpui::background_executor;
use tempfile::TempDir;

fn main() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let output_dir = Path::new("crates/dap_adapters/schemas");
    let executor = background_executor();
    JsDebugAdapter::get_schema(&temp_dir, output_dir, executor.clone())?;
    Ok(())
}
