use std::path::Path;

use collections::HashMap;
use util::ResultExt;

pub async fn load_direnv_environment(
    dir: &Path,
) -> (Option<HashMap<String, String>>, Option<String>) {
    let Ok(direnv_path) = which::which("direnv") else {
        return (None, None);
    };

    let Some(direnv_output) = smol::process::Command::new(direnv_path)
        .args(["export", "json"])
        .current_dir(dir)
        .output()
        .await
        .log_err()
    else {
        return (None, None);
    };

    if !direnv_output.status.success() {
        let error = format!(
            "Loading direnv environment failed (exit code {})\nStderr:\n{}",
            direnv_output.status,
            String::from_utf8_lossy(&direnv_output.stderr)
        );
        log::error!("{error}");
        return (None, Some(error));
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
