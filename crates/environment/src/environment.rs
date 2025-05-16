use anyhow::Result;
use collections::HashMap;
use std::path::Path;
use util::ResultExt;

// Returns the environment that Zed was launched with.
// Prefer `in_home_dir` outside of a project context, or `in_dir` inside of a project context,
// as these will work more like running commands in a shell.
pub fn inherited() -> HashMap<String, String> {
    std::env::vars().collect()
}

// The environment you get if you run a shell in the user's home directory.
pub async fn in_home_dir() -> HashMap<String, String> {
    static HOME_ENV: tokio::sync::OnceCell<HashMap<String, String>> =
        tokio::sync::OnceCell::const_new();

    HOME_ENV
        .get_or_init(|| async {
            match in_dir(paths::home_dir(), false).await.log_err() {
                Some(env) => env,
                None => inherited(),
            }
        })
        .await
        .clone()
}

#[cfg(any(test, feature = "test-support"))]
pub async fn in_dir(_dir: &Path, _load_direnv: bool) -> Result<HashMap<String, String>> {
    let fake_env = [("ZED_FAKE_TEST_ENV".into(), "true".into())]
        .into_iter()
        .collect();
    Ok(fake_env)
}

#[cfg(all(target_os = "windows", not(any(test, feature = "test-support"))))]
pub async fn in_dir(_dir: &Path, _load_direnv: bool) -> Result<HashMap<String, String>> {
    Ok(Default::default())
}

#[cfg(not(any(target_os = "windows", test, feature = "test-support")))]
pub async fn in_dir(dir: &Path, load_direnv: bool) -> Result<HashMap<String, String>> {
    use anyhow::Context;
    use std::path::PathBuf;
    use util::ResultExt;
    use util::parse_env_output;

    const MARKER: &str = "ZED_SHELL_START";
    let shell = util::get_system_shell();
    let shell_path = PathBuf::from(&shell);
    let shell_name = shell_path.file_name().and_then(|f| f.to_str());

    // What we're doing here is to spawn a shell and then `cd` into
    // the project directory to get the env in there as if the user
    // `cd`'d into it. We do that because tools like direnv, asdf, ...
    // hook into `cd` and only set up the env after that.
    //
    // If the user selects `Direct` for direnv, it would set an environment
    // variable that later uses to know that it should not run the hook.
    // We would include in `.envs` call so it is okay to run the hook
    // even if direnv direct mode is enabled.
    //
    // In certain shells we need to execute additional_command in order to
    // trigger the behavior of direnv, etc.

    let command = match shell_name {
        Some("fish") => format!(
            "cd '{}'; emit fish_prompt; printf '%s' {MARKER}; /usr/bin/env;",
            dir.display()
        ),
        _ => format!(
            "cd '{}'; printf '%s' {MARKER}; /usr/bin/env;",
            dir.display()
        ),
    };

    // csh/tcsh only supports `-l` if it's the only flag. So this won't be a login shell.
    // Users must rely on vars from `~/.tcshrc` or `~/.cshrc` and not `.login` as a result.
    let args = match shell_name {
        Some("tcsh") | Some("csh") => vec!["-i".to_string(), "-c".to_string(), command],
        _ => vec![
            "-l".to_string(),
            "-i".to_string(),
            "-c".to_string(),
            command,
        ],
    };

    let output = smol::unblock(move || {
        util::set_pre_exec_to_start_new_session(std::process::Command::new(&shell).args(&args))
            .output()
    })
    .await
    .with_context(|| "Failed to spawn login shell to source login environment variables")?;

    if !output.status.success() {
        anyhow::bail!("Login shell exited with nonzero exit code.");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let Some(env_output_start) = stdout.find(MARKER) else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!(
            "failed to parse output of `env` command in login shell. stdout: {:?}, stderr: {:?}",
            stdout,
            stderr
        );
    };

    let mut parsed_env = HashMap::default();
    let env_output = &stdout[env_output_start + MARKER.len()..];

    parse_env_output(env_output, |key, value| {
        parsed_env.insert(key, value);
    });

    if load_direnv {
        if let Some(direnv) = load_direnv_environment(&parsed_env, dir).await.log_err() {
            for (key, value) in direnv {
                parsed_env.insert(key, value);
            }
        }
    }

    Ok(parsed_env)
}

#[cfg(not(any(target_os = "windows", test, feature = "test-support")))]
async fn load_direnv_environment(
    env: &HashMap<String, String>,
    dir: &Path,
) -> Result<HashMap<String, String>> {
    let Ok(direnv_path) = which::which("direnv") else {
        return Ok(HashMap::default());
    };

    let direnv_output = smol::process::Command::new(direnv_path)
        .args(["export", "json"])
        .envs(env)
        .env("TERM", "dumb")
        .current_dir(dir)
        .output()
        .await?;

    if !direnv_output.status.success() {
        anyhow::bail!(
            "Loading direnv environment failed ({}), stderr: {}",
            direnv_output.status,
            String::from_utf8_lossy(&direnv_output.stderr)
        );
    }

    let output = String::from_utf8_lossy(&direnv_output.stdout);
    if output.is_empty() {
        return Ok(HashMap::default());
    }

    Ok(serde_json::from_str(&output)?)
}
