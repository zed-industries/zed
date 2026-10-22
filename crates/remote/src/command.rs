use anyhow::{Context as _, Result, ensure};
use collections::HashMap;
use serde::{Deserialize, Serialize};
use std::io::Read;
use util::shell::ShellKind;

#[derive(Serialize, Deserialize)]
pub struct RemoteCommand {
    pub program: String,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
    pub working_dir: Option<String>,
}

impl RemoteCommand {
    pub fn retain_valid_env(&mut self) {
        self.env.retain(|name, _| {
            let valid = !name.is_empty() && !name.contains(['=', '\0']);
            if !valid {
                log::warn!("Skipping environment variable with invalid name");
            }
            valid
        });
    }

    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut buffer = vec![0; size_of::<u32>()];
        serde_json::to_writer(&mut buffer, self)?;
        let len = buffer.len() - size_of::<u32>();
        ensure!(len <= MAX_COMMAND_SIZE, "Remote command exceeds 16 MiB");
        buffer[..size_of::<u32>()].copy_from_slice(&(len as u32).to_le_bytes());
        Ok(buffer)
    }

    pub fn read(reader: &mut impl Read) -> Result<Self> {
        let mut len = [0; size_of::<u32>()];
        reader
            .read_exact(&mut len)
            .context("Failed to read remote command length")?;
        let len = u32::from_le_bytes(len) as usize;
        ensure!(len <= MAX_COMMAND_SIZE, "Remote command exceeds 16 MiB");
        let mut buffer = vec![0; len];
        reader
            .read_exact(&mut buffer)
            .context("Failed to read remote command")?;
        serde_json::from_slice(&buffer).map_err(|_| anyhow::anyhow!("Invalid remote command"))
    }
}

pub fn stdio_launcher_command(shell_kind: ShellKind, remote_binary_path: &str) -> Result<String> {
    let helper = shell_kind
        .try_quote(remote_binary_path)
        .context("shell quoting")?;
    Ok(format!("exec {helper} exec"))
}

pub fn home_stdio_launcher_command(
    shell_kind: ShellKind,
    home_dir: Option<&str>,
    relative_binary_path: &str,
) -> Result<String> {
    match home_dir {
        Some(home_dir) => stdio_launcher_command(
            shell_kind,
            &format!("{}/{relative_binary_path}", home_dir.trim_end_matches('/')),
        ),
        None => Ok(format!(
            "exec {} exec",
            home_relative_path(shell_kind, Some(relative_binary_path))?
        )),
    }
}

pub fn home_relative_path(shell_kind: ShellKind, relative_path: Option<&str>) -> Result<String> {
    let quoted = relative_path
        .map(|relative_path| shell_kind.try_quote(relative_path).context("shell quoting"))
        .transpose()?;
    Ok(match (shell_kind, quoted) {
        (ShellKind::Nushell, Some(quoted)) => format!("($env.HOME | path join {quoted})"),
        (ShellKind::Nushell, None) => String::from("$env.HOME"),
        (_, Some(quoted)) => format!("\"$HOME\"/{quoted}"),
        (_, None) => String::from("\"$HOME\""),
    })
}

const MAX_COMMAND_SIZE: usize = 16 * 1024 * 1024;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_remote_command_frame() -> Result<()> {
        let json = br#"{"program":"agent","args":["","--acp"],"env":{"TOKEN":"'\"\n\r\t\\$()=secret\n"},"working_dir":"~/project with spaces"}"#;
        let mut input = (json.len() as u32).to_le_bytes().to_vec();
        input.extend_from_slice(json);
        let remaining = b"{\"jsonrpc\":\"2.0\",\"id\":1}\n\0\xff";
        input.extend_from_slice(remaining);
        let mut input = Cursor::new(input);
        let command = RemoteCommand::read(&mut input)?;
        assert_eq!(command.program, "agent");
        assert_eq!(command.args, ["", "--acp"]);
        assert_eq!(
            command.env,
            HashMap::from_iter([(
                String::from("TOKEN"),
                String::from("'\"\n\r\t\\$()=secret\n")
            )])
        );
        assert_eq!(
            command.working_dir.as_deref(),
            Some("~/project with spaces")
        );
        let mut output = Vec::new();
        input.read_to_end(&mut output)?;
        assert_eq!(output, remaining);
        let encoded = command.encode()?;
        assert_eq!(&encoded[4..], json);
        assert_eq!(&encoded[..4], &(json.len() as u32).to_le_bytes());
        Ok(())
    }

    #[test]
    fn test_remote_command_size_limit() -> Result<()> {
        let mut command = RemoteCommand {
            program: String::from("agent"),
            args: Vec::new(),
            env: HashMap::default(),
            working_dir: None,
        };
        let overhead = command.encode()?.len() - size_of::<u32>();
        command
            .program
            .push_str(&"a".repeat(MAX_COMMAND_SIZE - overhead));
        let encoded = command.encode()?;
        assert_eq!(encoded.len(), MAX_COMMAND_SIZE + size_of::<u32>());
        assert_eq!(
            RemoteCommand::read(&mut &encoded[..])?.program,
            command.program
        );
        command.program.push('a');
        assert_eq!(
            command.encode().unwrap_err().to_string(),
            "Remote command exceeds 16 MiB"
        );
        let oversized = (MAX_COMMAND_SIZE as u32 + 1).to_le_bytes();
        let error = RemoteCommand::read(&mut &oversized[..]).err().unwrap();
        assert_eq!(error.to_string(), "Remote command exceeds 16 MiB");
        Ok(())
    }

    #[test]
    fn test_retain_valid_env() {
        let mut command = RemoteCommand {
            program: String::from("agent"),
            args: Vec::new(),
            env: HashMap::from_iter(
                [
                    ("", "empty"),
                    ("NAME=VALUE", "equals"),
                    ("NAME\0NUL", "nul"),
                    ("LINE\nBREAK", "kept"),
                    ("TAB\tKEY", "kept"),
                    ("KEY-WITH-DASH", "kept"),
                    ("GH_TOKEN", "kept"),
                ]
                .map(|(name, value)| (name.to_owned(), value.to_owned())),
            ),
            working_dir: None,
        };
        command.retain_valid_env();
        assert_eq!(
            command.env,
            HashMap::from_iter([
                (String::from("LINE\nBREAK"), String::from("kept")),
                (String::from("TAB\tKEY"), String::from("kept")),
                (String::from("KEY-WITH-DASH"), String::from("kept")),
                (String::from("GH_TOKEN"), String::from("kept")),
            ])
        );
    }

    #[test]
    fn test_home_relative_path() -> Result<()> {
        assert_eq!(home_relative_path(ShellKind::Posix, None)?, "\"$HOME\"");
        assert_eq!(
            home_relative_path(ShellKind::Posix, Some("a b/c"))?,
            "\"$HOME\"/'a b/c'"
        );
        assert_eq!(
            home_relative_path(ShellKind::Tcsh, Some("bang!"))?,
            "\"$HOME\"/'bang'\\!"
        );
        assert_eq!(home_relative_path(ShellKind::Nushell, None)?, "$env.HOME");
        assert_eq!(
            home_relative_path(ShellKind::Nushell, Some("a b/O'Brien"))?,
            "($env.HOME | path join \"a b/O'Brien\")"
        );
        assert_eq!(
            home_stdio_launcher_command(ShellKind::Nushell, None, ".zed_server/server")?,
            "exec ($env.HOME | path join .zed_server/server) exec"
        );
        assert_eq!(
            home_stdio_launcher_command(ShellKind::Nushell, Some("/home/u/"), "x y")?,
            "exec '/home/u/x y' exec"
        );
        assert_eq!(
            home_stdio_launcher_command(ShellKind::Posix, None, "x y")?,
            "exec \"$HOME\"/'x y' exec"
        );
        Ok(())
    }

    #[test]
    fn test_remote_command_invalid_input() {
        for input in [&b""[..], &[1, 0, 0], &[1, 0, 0, 0]] {
            assert!(RemoteCommand::read(&mut &input[..]).is_err());
        }
        let json = br#"{"program":"agent","args":"secret"}"#;
        let mut input = (json.len() as u32).to_le_bytes().to_vec();
        input.extend_from_slice(json);
        let error = RemoteCommand::read(&mut &input[..]).err().unwrap();
        assert_eq!(error.to_string(), "Invalid remote command");
    }
}
