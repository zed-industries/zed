use std::path::Path;

use anyhow::{Context as _, Result};
use collections::HashMap;
use serde::Deserialize;

use crate::shell::ShellKind;

fn parse_env_map_from_noisy_output(output: &str) -> Result<collections::HashMap<String, String>> {
    for (position, _) in output.match_indices('{') {
        let candidate = &output[position..];
        let mut deserializer = serde_json::Deserializer::from_str(candidate);
        if let Ok(env_map) = HashMap::<String, String>::deserialize(&mut deserializer) {
            return Ok(env_map);
        }
    }
    anyhow::bail!("Failed to find JSON in shell output: {output}")
}

pub fn print_env() {
    let env_vars: HashMap<String, String> = std::env::vars().collect();
    let json = serde_json::to_string_pretty(&env_vars).unwrap_or_else(|err| {
        eprintln!("Error serializing environment variables: {}", err);
        std::process::exit(1);
    });
    println!("{}", json);
}

/// Capture all environment variables from the login shell in the given directory.
pub async fn capture(
    shell_path: impl AsRef<Path>,
    args: &[String],
    directory: impl AsRef<Path>,
) -> Result<collections::HashMap<String, String>> {
    #[cfg(windows)]
    return capture_windows(shell_path.as_ref(), args, directory.as_ref()).await;
    #[cfg(unix)]
    return capture_unix(shell_path.as_ref(), args, directory.as_ref()).await;
}

/// The current `Path` as recorded in the Windows registry, or `None` when it can't be read
/// (and on all other platforms).
///
/// A process keeps the environment block it was started with, so tools installed after Zed
/// launched are missing from this process's `Path`, even though shells opened from Explorer
/// find them. Reading the registry gives us the view such a shell would have.
///
/// See https://github.com/zed-industries/zed/issues/51349
#[cfg(windows)]
pub fn registry_path() -> Option<String> {
    use windows_registry::{CURRENT_USER, LOCAL_MACHINE};

    // `CURRENT_USER` and `LOCAL_MACHINE` are `&'static Key`, so they are passed by reference.
    fn read(hive: &windows_registry::Key, subkey: &str) -> Option<String> {
        match hive.open(subkey).and_then(|key| key.get_string("Path")) {
            Ok(path) => Some(expand_env_vars(&path)),
            Err(error) => {
                log::debug!("failed to read Path from {subkey}: {error}");
                None
            }
        }
    }

    let mut path = read(
        LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment",
    )
    .unwrap_or_default();
    let user_path = read(CURRENT_USER, "Environment").unwrap_or_default();
    if !user_path.is_empty() {
        if !path.is_empty() {
            path.push(';');
        }
        path.push_str(&user_path);
    }

    (!path.is_empty()).then_some(path)
}

#[cfg(not(windows))]
pub fn registry_path() -> Option<String> {
    None
}

/// Expands `%NAME%` references in a `REG_EXPAND_SZ` registry value.
#[cfg(windows)]
fn expand_env_vars(value: &str) -> String {
    expand_env_vars_with(value, |name| std::env::var(name).ok())
}

/// Expands `%NAME%` references using `lookup`, leaving names it can't resolve intact.
#[cfg(any(windows, test))]
fn expand_env_vars_with(value: &str, lookup: impl Fn(&str) -> Option<String>) -> String {
    let mut expanded = String::with_capacity(value.len());
    let mut rest = value;

    while let Some(start) = rest.find('%') {
        let after_start = &rest[start + 1..];
        let Some(end) = after_start.find('%') else {
            break;
        };
        let name = &after_start[..end];
        match lookup(name).filter(|_| !name.is_empty()) {
            Some(replacement) => {
                expanded.push_str(&rest[..start]);
                expanded.push_str(&replacement);
            }
            None => expanded.push_str(&rest[..start + 1 + end + 1]),
        }
        rest = &after_start[end + 1..];
    }

    expanded.push_str(rest);
    expanded
}

/// Try to parse the environment output before checking the exit status.
/// The user's shell rc files may contain commands that fail (e.g. editor
/// integrations that call posix_spawnp outside a real PTY), causing a
/// non-zero exit status even though `zed --printenv` ran successfully and
/// produced valid output on its separate fd.
fn parse_env_output(
    env_output: &str,
    status: &std::process::ExitStatus,
    successful_capture_warning: impl FnOnce() -> String,
    failed_capture_error: impl FnOnce() -> String,
) -> Result<collections::HashMap<String, String>> {
    match parse_env_map_from_noisy_output(env_output) {
        Ok(env_map) => {
            if !status.success() {
                log::warn!("{}", successful_capture_warning());
            }
            Ok(env_map)
        }
        Err(parse_error) => {
            if !status.success() {
                anyhow::bail!(
                    "{}. Failed to deserialize environment variables from json: {parse_error}. output: {env_output}",
                    failed_capture_error(),
                );
            }

            anyhow::bail!(
                "Failed to deserialize environment variables from json: {parse_error}. output: {env_output}"
            );
        }
    }
}

#[cfg(unix)]
async fn capture_unix(
    shell_path: &Path,
    args: &[String],
    directory: &Path,
) -> Result<collections::HashMap<String, String>> {
    use std::os::unix::process::CommandExt;

    use crate::command::new_std_command;

    let shell_kind = ShellKind::new(shell_path, false);
    let quoted_zed_path = super::get_shell_safe_zed_path(shell_kind)?;

    let mut command_string = String::new();
    let mut command = new_std_command(shell_path);
    command.args(args);
    let (fd_num, redir) = capture_output_redirection(shell_kind);

    match shell_kind {
        ShellKind::Csh | ShellKind::Tcsh => {
            // For csh/tcsh, login shell requires passing `-` as 0th argument (instead of `-l`)
            command.arg0("-");
        }
        ShellKind::Fish => {
            // in fish, asdf, direnv attach to the `fish_prompt` event
            command_string.push_str("emit fish_prompt;");
            command.arg("-l");
        }
        _ => {
            command.arg("-l");
        }
    }

    match shell_kind {
        // Nushell does not allow non-interactive login shells.
        // Instead of doing "-l -i -c '<command>'"
        // use "-l -e '<command>; exit'" instead
        ShellKind::Nushell => command.arg("-e"),
        _ => command.args(["-i", "-c"]),
    };

    // Prefix with "./" if the path starts with "-" to prevent cd from interpreting it as a flag
    let dir_str = directory.to_string_lossy();
    let dir_str = if dir_str.starts_with('-') {
        format!("./{dir_str}").into()
    } else {
        dir_str
    };
    let quoted_dir = shell_kind
        .try_quote(&dir_str)
        .context("unexpected null in directory name")?;

    // cd into the directory, triggering directory specific side-effects (asdf, direnv, etc)
    command_string.push_str(&format!("cd {};", quoted_dir));
    if let Some(prefix) = shell_kind.command_prefix() {
        command_string.push(prefix);
    }
    command_string.push_str(&format!("{} --printenv {}", quoted_zed_path, redir));

    if let ShellKind::Nushell = shell_kind {
        command_string.push_str("; exit");
    }

    command.arg(&command_string);

    super::set_pre_exec_to_start_new_session(&mut command);

    let (env_output, process_output) = spawn_and_read_fd(command, fd_num).await?;
    let env_output = String::from_utf8_lossy(&env_output);

    parse_env_output(
        &env_output,
        &process_output.status,
        || {
            format!(
                "login shell exited with {} but environment was captured successfully. stderr: {:?}",
                process_output.status,
                String::from_utf8_lossy(&process_output.stderr),
            )
        },
        || {
            format!(
                "login shell exited with {}. stdout: {:?}, stderr: {:?}",
                process_output.status,
                String::from_utf8_lossy(&process_output.stdout),
                String::from_utf8_lossy(&process_output.stderr),
            )
        },
    )
}

#[cfg(unix)]
fn capture_output_redirection(shell_kind: ShellKind) -> (std::os::fd::RawFd, String) {
    // In some shells, file descriptors greater than 2 cannot be used in interactive mode,
    // so file descriptor 0 (stdin) is used instead. This impacts zsh, old bash; perhaps others.
    // See: https://github.com/zed-industries/zed/pull/32136#issuecomment-2999645482
    const FD_STDIN: std::os::fd::RawFd = 0;
    const FD_STDOUT: std::os::fd::RawFd = 1;
    const FD_STDERR: std::os::fd::RawFd = 2;

    match shell_kind {
        ShellKind::Rc => (FD_STDIN, format!(">[1={FD_STDIN}]")), // `[1=0]`
        ShellKind::Nushell | ShellKind::Tcsh => (FD_STDOUT, "".to_string()),
        // xonsh doesn't support redirecting to stdin, and control sequences are printed to
        // stdout on startup
        ShellKind::Xonsh => (FD_STDERR, "o>e".to_string()),
        ShellKind::PowerShell | ShellKind::Pwsh => (FD_STDIN, format!("> /dev/fd/{FD_STDIN}")),
        _ => (FD_STDIN, format!(">&{FD_STDIN}")), // `>&0`
    }
}

#[cfg(unix)]
async fn spawn_and_read_fd(
    mut command: std::process::Command,
    child_fd: std::os::fd::RawFd,
) -> anyhow::Result<(Vec<u8>, std::process::Output)> {
    use command_fds::{CommandFdExt, FdMapping};
    use std::{io::Read, process::Stdio};

    let (mut reader, writer) = std::io::pipe()?;

    command.fd_mappings(vec![FdMapping {
        parent_fd: writer.into(),
        child_fd,
    }])?;

    let process = smol::process::Command::from(command)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    Ok((buffer, process.output().await?))
}

#[cfg(windows)]
async fn capture_windows(
    shell_path: &Path,
    args: &[String],
    directory: &Path,
) -> Result<collections::HashMap<String, String>> {
    use std::process::Stdio;

    let zed_path =
        std::env::current_exe().context("Failed to determine current zed executable path.")?;

    let shell_kind = ShellKind::new(shell_path, true);
    // Prefix with "./" if the path starts with "-" to prevent cd from interpreting it as a flag
    let directory_string = directory.display().to_string();
    let directory_string = if directory_string.starts_with('-') {
        format!("./{directory_string}")
    } else {
        directory_string
    };
    let zed_path_string = zed_path.display().to_string();
    let quote_for_shell = |value: &str| {
        shell_kind
            .try_quote(value)
            .map(|quoted| quoted.into_owned())
            .context("unexpected null in directory name")
    };
    let mut cmd = crate::command::new_command(shell_path);
    cmd.args(args);
    let quoted_directory = quote_for_shell(&directory_string)?;
    let quoted_zed_path = quote_for_shell(&zed_path_string)?;
    let cmd = match shell_kind {
        ShellKind::Csh
        | ShellKind::Tcsh
        | ShellKind::Rc
        | ShellKind::Fish
        | ShellKind::Xonsh
        | ShellKind::Posix => cmd.args([
            "-l",
            "-i",
            "-c",
            &format!("cd {}; {} --printenv", quoted_directory, quoted_zed_path),
        ]),
        ShellKind::PowerShell | ShellKind::Pwsh => cmd.args([
            "-NonInteractive",
            "-NoProfile",
            "-Command",
            &format!(
                "Set-Location {}; & {} --printenv",
                quoted_directory, quoted_zed_path
            ),
        ]),
        ShellKind::Elvish => cmd.args([
            "-c",
            &format!("cd {}; {} --printenv", quoted_directory, quoted_zed_path),
        ]),
        ShellKind::Nushell => {
            let zed_command = shell_kind
                .prepend_command_prefix(&quoted_zed_path)
                .into_owned();
            cmd.args([
                "-c",
                &format!("cd {}; {} --printenv", quoted_directory, zed_command),
            ])
        }
        ShellKind::Cmd => {
            let dir = directory_string.trim_end_matches('\\');
            cmd.args(["/d", "/c", "cd", dir, "&&", &zed_path_string, "--printenv"])
        }
    }
    .stdin(Stdio::null())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped());
    // Newly installed tools append themselves to `Path` in the registry and broadcast
    // `WM_SETTINGCHANGE`, but that never reaches our own environment block, so a shell we
    // spawn here would inherit a stale `Path`. Read it from the registry instead, so the
    // captured environment matches what a shell launched from Explorer would see.
    if let Some(path) = registry_path() {
        cmd.env("PATH", path);
    }

    let output = cmd
        .output()
        .await
        .with_context(|| format!("command {cmd:?}"))?;
    let env_output = String::from_utf8_lossy(&output.stdout);

    parse_env_output(
        &env_output,
        &output.status,
        || {
            format!(
                "Command {cmd:?} exited with {} but environment was captured successfully. stderr: {:?}",
                output.status,
                String::from_utf8_lossy(&output.stderr),
            )
        },
        || {
            format!(
                "Command {cmd:?} failed with {}. stdout: {:?}, stderr: {:?}",
                output.status,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr),
            )
        },
    )
}

#[cfg(test)]
mod tests {
    use std::process::ExitStatus;

    use super::*;
    use crate::path;

    #[cfg(unix)]
    fn exit_status(code: i32) -> ExitStatus {
        use std::os::unix::process::ExitStatusExt;

        ExitStatus::from_raw(code << 8)
    }

    #[cfg(windows)]
    fn exit_status(code: u32) -> ExitStatus {
        use std::os::windows::process::ExitStatusExt;

        ExitStatus::from_raw(code)
    }

    #[cfg(unix)]
    #[test]
    fn capture_powershell_environment_without_startup_output() -> Result<()> {
        for shell in ["pwsh", "powershell"] {
            let (descriptor, redirection) =
                capture_output_redirection(ShellKind::new(shell, false));
            if let Ok(shell_path) = which::which(shell) {
                let directory = tempfile::tempdir()?;
                let script =
                    format!("Write-Output '{{}}'; /bin/echo '{{\"PROBE\":\"ok\"}}' {redirection}");
                let mut command = crate::command::new_std_command(shell_path);
                command
                    .args(["-NoProfile", "-l", "-i", "-c", &script])
                    .current_dir(directory.path());
                let (output, process_output) =
                    smol::block_on(spawn_and_read_fd(command, descriptor))?;
                assert!(process_output.status.success(), "{process_output:?}");
                assert_eq!(output, b"{\"PROBE\":\"ok\"}\n");
                assert_eq!(process_output.stdout, b"{}\n");
                assert!(!directory.path().join("0").exists());
            }
            assert_eq!((descriptor, redirection), (0, String::from("> /dev/fd/0")));
        }
        Ok(())
    }

    #[test]
    fn expand_env_vars_expands_registry_references() {
        let lookup = |name: &str| match name {
            "SystemRoot" => Some(r"C:\Windows".to_string()),
            _ => None,
        };

        assert_eq!(
            expand_env_vars_with(r"%SystemRoot%\system32;%UNKNOWN%;C:\tools", lookup),
            r"C:\Windows\system32;%UNKNOWN%;C:\tools"
        );
    }

    #[test]
    fn parse_env_output_accepts_valid_env_when_shell_exits_nonzero() {
        let env_json = serde_json::json!({
            "PATH": path!("/usr/bin"),
            "SHELL": path!("/bin/zsh"),
        });
        let env_output = format!("shell startup noise\n{env_json}\nshell shutdown noise");

        let env_map = parse_env_output(
            &env_output,
            &exit_status(1),
            || "shell exited with 1 but environment was captured successfully".to_string(),
            || panic!("failed capture error should not be evaluated for valid environment output"),
        )
        .expect("valid environment output should be returned despite non-zero shell exit");
        assert_eq!(
            env_map.get("PATH").map(String::as_str),
            Some(path!("/usr/bin"))
        );
        assert_eq!(
            env_map.get("SHELL").map(String::as_str),
            Some(path!("/bin/zsh"))
        );
    }
}
