use std::{path::Path, process::ExitStatus};

use collections::HashMap;

pub enum DirenvLoadError {
    Io(std::io::Error),
    NonZeroResult { status: ExitStatus, stderr: Vec<u8> },
    Serde(serde_json::Error),
}

#[derive(Debug, Clone, Default)]
pub struct DirenvWarning {
    warning: Option<String>,
}

impl From<String> for DirenvWarning {
    fn from(value: String) -> Self {
        Self {
            warning: Some(value),
        }
    }
}

impl DirenvWarning {
    pub fn take(&mut self) -> Option<String> {
        self.warning.take()
    }
}

impl Drop for DirenvWarning {
    fn drop(&mut self) {
        if let Some(ref warning) = self.warning {
            log::warn!("{warning}");
        }
    }
}

pub async fn load_direnv_environment(
    dir: &Path,
) -> Result<Option<HashMap<String, String>>, DirenvLoadError> {
    let Ok(direnv_path) = which::which("direnv") else {
        return Ok(None);
    };

    let direnv_output = match smol::process::Command::new(direnv_path)
        .args(["export", "json"])
        .current_dir(dir)
        .output()
        .await
    {
        Ok(output) => output,
        Err(io_err) => return Err(DirenvLoadError::Io(io_err)),
    };

    if !direnv_output.status.success() {
        return Err(DirenvLoadError::NonZeroResult {
            status: direnv_output.status,
            stderr: direnv_output.stderr,
        });
    }

    let output = String::from_utf8_lossy(&direnv_output.stdout);
    if output.is_empty() {
        return Ok(None);
    }

    Ok(Some(match serde_json::from_str(&output) {
        Ok(env) => env,
        Err(err) => return Err(DirenvLoadError::Serde(err)),
    }))
}
