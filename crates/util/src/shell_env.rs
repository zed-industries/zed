use std::path::Path;

use anyhow::{Context as _, Result};
use collections::HashMap;

use crate::shell::ShellKind;

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

#[cfg(unix)]
async fn capture_unix(
    shell_path: &Path,
    args: &[String],
    directory: &Path,
) -> Result<collections::HashMap<String, String>> {
    use std::os::unix::process::CommandExt;
    use std::process::Stdio;

    let zed_path = super::get_shell_safe_zed_path()?;
    let shell_kind = ShellKind::new(shell_path);

    let mut command_string = String::new();
    let mut command = std::process::Command::new(shell_path);
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
        _ => (FD_STDIN, format!(">&{}", FD_STDIN)), // `>&0`
    };
    command.stdin(Stdio::null());
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

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
    // cd into the directory, triggering directory specific side-effects (asdf, direnv, etc)
    command_string.push_str(&format!("cd '{}';", directory.display()));
    if let Some(prefix) = shell_kind.command_prefix() {
        command_string.push(prefix);
    }
    command_string.push_str(&format!("{} --printenv {}", zed_path, redir));
    command.args(["-i", "-c", &command_string]);

    super::set_pre_exec_to_start_new_session(&mut command);

    let (env_output, process_output) = spawn_and_read_fd(command, fd_num).await?;
    let env_output = String::from_utf8_lossy(&env_output);

    anyhow::ensure!(
        process_output.status.success(),
        "login shell exited with {}. stdout: {:?}, stderr: {:?}",
        process_output.status,
        String::from_utf8_lossy(&process_output.stdout),
        String::from_utf8_lossy(&process_output.stderr),
    );

    // Parse the JSON output from zed --printenv
    let env_map: collections::HashMap<String, String> = serde_json::from_str(&env_output)
        .with_context(|| "Failed to deserialize environment variables from json")?;
    Ok(env_map)
}

#[cfg(unix)]
async fn spawn_and_read_fd(
    mut command: std::process::Command,
    child_fd: std::os::fd::RawFd,
) -> anyhow::Result<(Vec<u8>, std::process::Output)> {
    use command_fds::{CommandFdExt, FdMapping};
    use std::io::Read;

    let (mut reader, writer) = std::io::pipe()?;

    command.fd_mappings(vec![FdMapping {
        parent_fd: writer.into(),
        child_fd,
    }])?;

    let process = smol::process::Command::from(command).spawn()?;

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    Ok((buffer, process.output().await?))
}

#[cfg(windows)]
async fn capture_windows(
    shell_path: &Path,
    _args: &[String],
    directory: &Path,
) -> Result<collections::HashMap<String, String>> {
    use std::process::Stdio;

    let zed_path =
        std::env::current_exe().context("Failed to determine current zed executable path.")?;

    let shell_kind = ShellKind::new(shell_path);
    let env_output = match shell_kind {
        ShellKind::Posix
        | ShellKind::Csh
        | ShellKind::Tcsh
        | ShellKind::Rc
        | ShellKind::Fish
        | ShellKind::Xonsh => {
            return Err(anyhow::anyhow!("unsupported shell kind"));
        }
        ShellKind::PowerShell => {
            let output = crate::command::new_smol_command(shell_path)
                .args([
                    "-NonInteractive",
                    "-NoProfile",
                    "-Command",
                    &format!(
                        "Set-Location '{}'; & '{}' --printenv",
                        directory.display(),
                        zed_path.display()
                    ),
                ])
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .await?;

            anyhow::ensure!(
                output.status.success(),
                "PowerShell command failed with {}. stdout: {:?}, stderr: {:?}",
                output.status,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr),
            );
            output
        }
        ShellKind::Nushell => {
            let output = crate::command::new_smol_command(shell_path)
                .args([
                    "-c",
                    &format!(
                        "cd '{}'; {} --printenv",
                        directory.display(),
                        zed_path.display()
                    ),
                ])
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .await?;

            anyhow::ensure!(
                output.status.success(),
                "Nushell command failed with {}. stdout: {:?}, stderr: {:?}",
                output.status,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr),
            );
            output
        }
        ShellKind::Cmd => {
            let output = crate::command::new_smol_command(shell_path)
                .args([
                    "/c",
                    &format!(
                        "cd '{}'; {} --printenv",
                        directory.display(),
                        zed_path.display()
                    ),
                ])
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .await?;

            anyhow::ensure!(
                output.status.success(),
                "Cmd command failed with {}. stdout: {:?}, stderr: {:?}",
                output.status,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr),
            );
            output
        }
    };

    let env_output = String::from_utf8_lossy(&env_output.stdout);

    // Parse the JSON output from zed --printenv
    serde_json::from_str(&env_output)
        .with_context(|| "Failed to deserialize environment variables from json")
}
