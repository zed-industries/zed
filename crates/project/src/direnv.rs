use crate::environment::EnvironmentErrorMessage;
use std::process::ExitStatus;

#[cfg(not(any(target_os = "windows", test, feature = "test-support")))]
use {collections::HashMap, std::path::Path, util::ResultExt};

#[derive(Clone)]
pub enum DirenvError {
    NotFound,
    FailedRun,
    NonZeroExit(ExitStatus, Vec<u8>),
    EmptyOutput,
    InvalidJson,
}

impl From<DirenvError> for Option<EnvironmentErrorMessage> {
    fn from(value: DirenvError) -> Self {
        match value {
            DirenvError::NotFound => None,
            DirenvError::FailedRun | DirenvError::NonZeroExit(_, _) => {
                Some(EnvironmentErrorMessage(String::from(
                    "Failed to run direnv. See logs for more info",
                )))
            }
            DirenvError::EmptyOutput => None,
            DirenvError::InvalidJson => Some(EnvironmentErrorMessage(String::from(
                "Direnv returned invalid json. See logs for more info",
            ))),
        }
    }
}

#[cfg(not(any(target_os = "windows", test, feature = "test-support")))]
pub async fn load_direnv_environment(
    env: &HashMap<String, String>,
    dir: &Path,
) -> Result<HashMap<String, String>, DirenvError> {
    let Ok(direnv_path) = which::which("direnv") else {
        return Err(DirenvError::NotFound);
    };

    let Some(direnv_output) = smol::process::Command::new(direnv_path)
        .args(["export", "json"])
        .envs(env)
        .env("TERM", "dumb")
        .current_dir(dir)
        .output()
        .await
        .log_err()
    else {
        return Err(DirenvError::FailedRun);
    };

    if !direnv_output.status.success() {
        log::error!(
            "Loading direnv environment failed ({}), stderr: {}",
            direnv_output.status,
            String::from_utf8_lossy(&direnv_output.stderr)
        );
        return Err(DirenvError::NonZeroExit(
            direnv_output.status,
            direnv_output.stderr,
        ));
    }

    let output = String::from_utf8_lossy(&direnv_output.stdout);
    if output.is_empty() {
        return Err(DirenvError::EmptyOutput);
    }

    let Some(env) = serde_json::from_str(&output).log_err() else {
        return Err(DirenvError::InvalidJson);
    };

    Ok(env)
}
