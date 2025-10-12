use crate::environment::EnvironmentErrorMessage;
use std::process::ExitStatus;

use {collections::HashMap, std::path::Path, util::ResultExt};

#[derive(Clone)]
pub enum DirenvError {
    NotFound,
    FailedRun,
    NonZeroExit(ExitStatus, Vec<u8>),
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
            DirenvError::InvalidJson => Some(EnvironmentErrorMessage(String::from(
                "Direnv returned invalid json. See logs for more info",
            ))),
        }
    }
}

pub async fn load_direnv_environment(
    env: &HashMap<String, String>,
    dir: &Path,
) -> Result<HashMap<String, Option<String>>, DirenvError> {
    let Ok(direnv_path) = which::which("direnv") else {
        return Err(DirenvError::NotFound);
    };

    let args = &["export", "json"];
    let Some(direnv_output) = smol::process::Command::new(&direnv_path)
        .args(args)
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
        // direnv outputs nothing when it has no changes to apply to environment variables
        return Ok(HashMap::default());
    }

    match serde_json::from_str(&output) {
        Ok(env) => Ok(env),
        Err(err) => {
            log::error!(
                "json parse error {}, while parsing output of `{} {}`:\n{}",
                err,
                direnv_path.display(),
                args.join(" "),
                output
            );
            Err(DirenvError::InvalidJson)
        }
    }
}
