use std::{path::Path, process::Command};

use dap::adapters::DapDelegate as _;
use dap_adapters::{
    CodeLldbDebugAdapter, GoDebugAdapter, JsDebugAdapter, PythonDebugAdapter,
    UpdateSchemasDapDelegate,
};
use tempfile::TempDir;

fn main() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let output_dir = Path::new("crates/dap_adapters/schemas");
    let delegate = UpdateSchemasDapDelegate::new();

    let schema = JsDebugAdapter::get_schema(&temp_dir, delegate.clone())?;
    std::fs::write(
        &output_dir
            .join(JsDebugAdapter::ADAPTER_NAME)
            .with_extension("json"),
        serde_json::to_string(&schema)?,
    )?;

    let schema = PythonDebugAdapter::get_schema(&temp_dir, delegate.clone())?;
    std::fs::write(
        &output_dir
            .join(PythonDebugAdapter::ADAPTER_NAME)
            .with_extension("json"),
        serde_json::to_string(&schema)?,
    )?;

    let schema = GoDebugAdapter::get_schema(&temp_dir, delegate.clone())?;
    std::fs::write(
        &output_dir
            .join(GoDebugAdapter::ADAPTER_NAME)
            .with_extension("json"),
        serde_json::to_string(&schema)?,
    )?;

    let schema = CodeLldbDebugAdapter::get_schema(&temp_dir, delegate.clone())?;
    std::fs::write(
        &output_dir
            .join(CodeLldbDebugAdapter::ADAPTER_NAME)
            .with_extension("json"),
        serde_json::to_string(&schema)?,
    )?;

    delegate.output_to_console("Formatting schemas with prettier...".into());
    Command::new("npx")
        .arg("prettier")
        .arg("--write")
        .arg(output_dir.join("*"))
        .status()?;
    Ok(())
}
