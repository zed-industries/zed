use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub fn get_or_create_id(path: &Path) -> Result<String> {
    if let Ok(id) = fs::read_to_string(path) {
        let trimmed = id.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_string());
        }
    }
    let new_id = Uuid::new_v4().to_string();
    fs::write(path, &new_id)?;
    Ok(new_id)
}

pub fn eval_system_id_path() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("zed-eval-system-id")
}

pub fn eval_installation_id_path() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("zed-eval-installation-id")
}
