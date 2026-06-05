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
    // In some shells, file descriptors greater than 2 cannot be used in interactive mode,
    // so file descriptor 0 (stdin) is used instead. This impacts zsh, old bash; perhaps others.
    // See: https://github.com/zed-industries/zed/pull/32136#issuecomment-2999645482
    const FD_STDIN: std::os::fd::RawFd = 0;
    const FD_STDOUT: std::os::fd::RawFd = 1;
    const FD_STDERR: std::os::fd::RawFd = 2;

    let (fd_num, redir) = match shell_kind {
        ShellKind::Rc => (FD_STDIN, format!(">[1={}]", FD_STDIN)), // `[1=0]`
        ShellKind::Nushell | ShellKind::Tcsh => (FD_STDOUT, "".to_string()),
        // xonsh doesn't support redirecting to stdin, and control sequences are printed to
        // stdout on startup
        ShellKind::Xonsh => (FD_STDERR, "o>e".to_string()),
        ShellKind::PowerShell => (FD_STDIN, format!(">{}", FD_STDIN)),
        _ => (FD_STDIN, format!(">&{}", FD_STDIN)), // `>&0`
    };

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
