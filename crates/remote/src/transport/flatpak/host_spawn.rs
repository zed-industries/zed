use anyhow::Result;
use collections::HashMap;
use std::path::{Path, PathBuf};

use crate::remote_client::CommandTemplate;

const FLATPAK_SPAWN: &str = "/usr/bin/flatpak-spawn";

/// Builder for commands wrapped with `flatpak-spawn --host`.
pub(super) struct HostCommand {
    program: String,
    remote_server_host_path: Option<String>,
    program_args: Vec<String>,
    current_dir: PathBuf,
    env: HashMap<String, String>,
    pty: bool,
}

impl HostCommand {
    pub(super) fn new(program: &str, remote_server_host_path: Option<&str>) -> Self {
        Self {
            program: program.to_string(),
            remote_server_host_path: remote_server_host_path.map(|p| p.to_string()),
            program_args: Vec::new(),
            current_dir: PathBuf::from("/"),
            env: Default::default(),
            pty: false,
        }
    }

    pub(super) fn arg(mut self, arg: impl AsRef<str>) -> Self {
        self.program_args.push(arg.as_ref().to_string());
        self
    }

    pub(super) fn args<S>(mut self, args: impl IntoIterator<Item = S>) -> Self
    where
        S: AsRef<str>,
    {
        self.program_args
            .extend(args.into_iter().map(|a| a.as_ref().to_string()));
        self
    }

    pub(super) fn env(mut self, var: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        self.env
            .insert(var.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    pub(super) fn envs<K, V>(mut self, envs: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.env.extend(
            envs.into_iter()
                .map(|(k, v)| (k.as_ref().to_string(), v.as_ref().to_string())),
        );
        self
    }

    pub(super) fn pty(mut self, pty: bool) -> Self {
        assert!(
            !pty || self.remote_server_host_path.is_some(),
            "pty is enabled but remote_server_host_path is not set"
        );
        self.pty = pty;
        self
    }

    pub(super) fn current_dir(mut self, current_dir: impl AsRef<Path>) -> Self {
        self.current_dir = current_dir.as_ref().to_owned();
        self
    }

    pub(super) fn build_template(self) -> CommandTemplate {
        let mut args: Vec<String> = vec!["--host".to_string(), "--watch-bus".to_string()];

        // The host command (and, with the wrapper, the process it execs) runs
        // in this directory.
        args.push(format!(
            "--directory={}",
            self.current_dir.to_string_lossy()
        ));

        // `flatpak-spawn` sets these variables in the host command's
        // environment, from where they are inherited by the wrapped process.
        for (key, value) in &self.env {
            args.push(format!("--env={key}={value}"));
        }

        args.push("--".to_string());

        // Wrap with a local PTY if requested
        if self.pty {
            args.push(
                self.remote_server_host_path
                    .expect("remote_server_host_path not set but pty requested"),
            );
            args.push("--pty-wrapper".to_string());
        }

        args.push(self.program);
        args.extend(self.program_args);

        CommandTemplate {
            program: FLATPAK_SPAWN.to_string(),
            args,
            env: HashMap::default(),
        }
    }

    pub(super) fn build_command(self) -> util::command::Command {
        let template = self.build_template();
        let mut command = util::command::new_command(template.program);
        command.args(template.args).envs(template.env);
        command
    }

    pub(super) async fn output(self) -> Result<String> {
        let mut command = self.build_command();
        let output = command.output().await?;
        log::debug!("{:?}: {:?}", command, output);
        anyhow::ensure!(
            output.status.success(),
            "failed to run host command {command:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interactive_command_requests_a_pty() {
        let template = HostCommand::new("/bin/zsh", Some("/host/libexec/zed-remote-server"))
            .current_dir("/home/user/project")
            .env("TERM", "xterm-256color")
            .pty(true)
            .build_template();

        assert_eq!(template.program, FLATPAK_SPAWN);
        assert_eq!(template.args[0], "--host");
        assert_eq!(template.args[1], "--watch-bus");
        assert!(
            template
                .args
                .contains(&"--directory=/home/user/project".to_string())
        );
        assert!(
            template
                .args
                .contains(&"--env=TERM=xterm-256color".to_string())
        );

        // After the first `--`, the target command must be wrapped in the
        // host-side `pty-wrapper` (delimited by a second `--`).
        let separator = template
            .args
            .iter()
            .position(|arg| arg == "--")
            .expect("expected a `--` separator");
        assert_eq!(
            &template.args[separator..],
            &[
                "--".to_string(),
                "/host/libexec/zed-remote-server".to_string(),
                "--pty-wrapper".to_string(),
                "/bin/zsh".to_string(),
            ]
        );
    }

    #[test]
    fn non_interactive_command_does_not_request_a_pty() {
        let template = HostCommand::new("tar", None)
            .arg("-x")
            .arg("-C")
            .arg("/tmp/dest")
            .build_template();

        assert_eq!(template.program, FLATPAK_SPAWN);
        assert!(!template.args.iter().any(|arg| arg == "--pty-wrapper"));

        // Exactly one `--`, immediately preceding the program and its args.
        let separators = template.args.iter().filter(|arg| *arg == "--").count();
        assert_eq!(separators, 1);
        let separator = template.args.iter().position(|arg| arg == "--").unwrap();
        assert_eq!(
            &template.args[separator + 1..],
            &[
                "tar".to_string(),
                "-x".to_string(),
                "-C".to_string(),
                "/tmp/dest".to_string(),
            ]
        );
    }
}
