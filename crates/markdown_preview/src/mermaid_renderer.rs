use anyhow::{Context, Result, anyhow};
use gpui::SharedString;
use node_runtime::{NodeRuntime, VersionStrategy};
use smol::fs;
use std::path::PathBuf;
use std::sync::Arc;
use util::ResultExt;

pub async fn render_mermaid_diagram(
    contents: SharedString,
    diagram_id: u64,
    scale: u32,
) -> Result<PathBuf> {
    // let node_path = self
    //     .node_runtime
    //     .binary_path()
    //     .await
    //     .context("failed to get node binary path")?;

    // let input_file = self.mermaid_dir.join(format!("input_{}.mmd", diagram_id));
    // let output_file = self.mermaid_dir.join(format!("diagram_{}.png", diagram_id));

    // if output_file.exists() {
    //     fs::remove_file(&output_file)
    //         .await
    //         .context("failed to remove old output file")?;
    // }

    // fs::write(&input_file, mermaid_source)
    //     .await
    //     .context("failed to write mermaid source to file")?;

    // let mmdc_path = self
    //     .mermaid_dir
    //     .join("node_modules")
    //     .join(".bin")
    //     .join(if cfg!(windows) { "mmdc.cmd" } else { "mmdc" });

    // let scale_value = ((scale as f32 / 100.0) * 2.0).clamp(1.0, 10.0) as u32;

    // let output = util::command::new_smol_command(&node_path)
    //     .arg(&mmdc_path)
    //     .arg("-i")
    //     .arg(&input_file)
    //     .arg("-o")
    //     .arg(&output_file)
    //     .arg("--theme")
    //     .arg("neutral")
    //     .arg("--backgroundColor")
    //     .arg("transparent")
    //     .arg("--scale")
    //     .arg(scale_value.to_string())
    //     .arg("--quiet")
    //     .current_dir(&self.mermaid_dir)
    //     .output()
    //     .await
    //     .context("failed to execute mermaid-cli")?;

    // if !output.status.success() {
    //     let stderr = String::from_utf8_lossy(&output.stderr);
    //     return Err(anyhow!("mermaid-cli failed: {}", stderr));
    // }

    // if !output_file.exists() {
    //     return Err(anyhow!("mermaid-cli did not generate output file"));
    // }

    // fs::remove_file(&input_file).await.log_err();

    // Ok(output_file)
    todo!()
}
