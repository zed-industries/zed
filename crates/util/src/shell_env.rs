#![cfg_attr(not(unix), allow(unused))]

use anyhow::{Context as _, Result};
use collections::HashMap;

/// Capture all environment variables from the login shell.
#[cfg(unix)]
pub fn capture(directory: &std::path::Path) -> Result<collections::HashMap<String, String>> {
    use std::os::unix::process::CommandExt;
    use std::process::Stdio;

    let zed_path = super::get_shell_safe_zed_path()?;
    let shell_path = std::env::var("SHELL").map(std::path::PathBuf::from)?;
    let shell_name = shell_path.file_name().and_then(std::ffi::OsStr::to_str);

    let mut command_string = String::new();
    let mut command = std::process::Command::new(&shell_path);
    // In some shells, file descriptors greater than 2 cannot be used in interactive mode,
    // so file descriptor 0 (stdin) is used instead. [Citation Needed]
    const ENV_OUTPUT_FD: std::os::fd::RawFd = 0;
    command.stdin(Stdio::null());
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    match shell_name {
        Some("tcsh" | "csh") => {
            // For csh/tcsh, login shell requires passing `-` as 0th argument (instead of `-l`)
            command.arg0("-");
        }
        Some("fish") => {
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
    command_string.push_str(&format!(
        "sh -c \"{} --printenv >&{}\";",
        zed_path, ENV_OUTPUT_FD
    ));
    command.args(["-i", "-c", &command_string]);

    super::set_pre_exec_to_start_new_session(&mut command);

    let (env_output, process_output) = spawn_and_read_fd(command, ENV_OUTPUT_FD)?;
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
fn spawn_and_read_fd(
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

    let process = command.spawn()?;
    drop(command);

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    Ok((buffer, process.wait_with_output()?))
}

pub fn print_env() {
    let env_vars: HashMap<String, String> = std::env::vars().collect();
    let json = serde_json::to_string_pretty(&env_vars).unwrap_or_else(|err| {
        eprintln!("Error serializing environment variables: {}", err);
        std::process::exit(1);
    });
    println!("{}", json);
    std::process::exit(0);
}
