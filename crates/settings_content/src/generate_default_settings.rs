//! Writes the generated `assets/settings/default.json` to disk.
//!
//! Run via `script/generate-default-settings`.

use std::path::Path;

fn main() -> anyhow::Result<()> {
    let generated = settings_content::default_settings_json::generate_default_settings_json()?;
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/settings/default.json");
    std::fs::write(&path, generated)?;
    println!("Wrote {}", path.display());
    Ok(())
}
