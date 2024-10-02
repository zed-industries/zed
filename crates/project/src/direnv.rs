use std::process::ExitStatus;
#[cfg(not(any(test, feature = "test-support")))]
use {collections::HashMap, std::path::Path, util::ResultExt};

pub struct DirenvError {
    pub status: ExitStatus,
    pub stderr: Vec<u8>,
}

#[cfg(not(any(test, feature = "test-support")))]
pub async fn load_direnv_environment(
    dir: &Path,
) -> (Option<HashMap<String, String>>, Option<DirenvError>) {
    let Ok(direnv_path) = which::which("direnv") else {
        return (None, None);
    };

    let Some(direnv_output) = smol::process::Command::new(direnv_path)
        .args(["export", "json"])
        .env("TERM", "dumb")
        .current_dir(dir)
        .output()
        .await
        .log_err()
    else {
        return (None, None);
    };

    if !direnv_output.status.success() {
        log::error!(
            "Loading direnv environment failed ({}), stderr: {}",
            direnv_output.status,
            String::from_utf8_lossy(&direnv_output.stderr)
        );
        return (
            None,
            Some(DirenvError {
                status: direnv_output.status,
                stderr: direnv_output.stderr,
            }),
        );
    }

    let output = String::from_utf8_lossy(&direnv_output.stdout);
    if output.is_empty() {
        return (None, None);
    }

    let Some(env) = serde_json::from_str(&output).log_err() else {
        return (None, None);
    };

    (env, None)
}
