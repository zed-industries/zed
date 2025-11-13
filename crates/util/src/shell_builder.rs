use crate::shell::get_system_shell;
use crate::shell::{Shell, ShellKind};
use std::borrow::Cow;

/// ShellBuilder is used to turn a user-requested task into a
/// program that can be executed by the shell.
pub struct ShellBuilder {
    /// The shell to run
    program: String,
    args: Vec<String>,
    interactive: bool,
    /// Whether to redirect stdin to /dev/null for the spawned command as a subshell.
    redirect_stdin: bool,
    kind: ShellKind,
}

impl ShellBuilder {
    /// Create a new ShellBuilder as configured.
    pub fn new(shell: &Shell, is_windows: bool) -> Self {
        let (program, args) = match shell {
            Shell::System => (get_system_shell(), Vec::new()),
            Shell::Program(shell) => (shell.clone(), Vec::new()),
            Shell::WithArguments { program, args, .. } => (program.clone(), args.clone()),
        };

        let kind = ShellKind::new(&program, is_windows);
        Self {
            program,
            args,
            interactive: true,
            kind,
            redirect_stdin: false,
        }
    }
    pub fn non_interactive(mut self) -> Self {
        self.interactive = false;
        self
    }

    /// Returns the label to show in the terminal tab
    pub fn command_label(&self, command_to_use_in_label: &str) -> String {
        if command_to_use_in_label.trim().is_empty() {
            self.program.clone()
        } else {
            match self.kind {
                ShellKind::PowerShell => {
                    format!("{} -C '{}'", self.program, command_to_use_in_label)
                }
                ShellKind::Cmd => {
                    format!("{} /C \"{}\"", self.program, command_to_use_in_label)
                }
                ShellKind::Posix
                | ShellKind::Nushell
                | ShellKind::Fish
                | ShellKind::Csh
                | ShellKind::Tcsh
                | ShellKind::Rc
                | ShellKind::Xonsh
                | ShellKind::Elvish => {
                    let interactivity = self.interactive.then_some("-i ").unwrap_or_default();
                    format!(
                        "{PROGRAM} {interactivity}-c '{command_to_use_in_label}'",
                        PROGRAM = self.program
                    )
                }
            }
        }
    }

    pub fn redirect_stdin_to_dev_null(mut self) -> Self {
        self.redirect_stdin = true;
        self
    }

    /// Returns the program and arguments to run this task in a shell.
    pub fn build(
        mut self,
        task_command: Option<String>,
        task_args: &[String],
    ) -> (String, Vec<String>) {
        if let Some(task_command) = task_command {
            let mut combined_command = self
                .kind
                .try_quote_prefix_aware(&task_command)
                .map(Cow::into_owned)
                .unwrap_or(task_command);

            combined_command = task_args.iter().fold(combined_command, |mut command, arg| {
                command.push(' ');
                let substituted = self.kind.to_shell_variable(arg);
                let quoted = self
                    .kind
                    .try_quote(&substituted)
                    .map(Cow::into_owned)
                    .unwrap_or(substituted);
                command.push_str(&quoted);
                command
            });
            if self.redirect_stdin {
                match self.kind {
                    ShellKind::Fish => {
                        combined_command.insert_str(0, "begin; ");
                        combined_command.push_str("; end </dev/null");
                    }
                    ShellKind::Posix
                    | ShellKind::Nushell
                    | ShellKind::Csh
                    | ShellKind::Tcsh
                    | ShellKind::Rc
                    | ShellKind::Xonsh
                    | ShellKind::Elvish => {
                        combined_command.insert(0, '(');
                        combined_command.push_str(") </dev/null");
                    }
                    ShellKind::PowerShell => {
                        combined_command.insert_str(0, "$null | & {");
                        combined_command.push_str("}");
                    }
                    ShellKind::Cmd => {
                        combined_command.push_str("< NUL");
                    }
                }
            }

            self.args
                .extend(self.kind.args_for_shell(self.interactive, combined_command));
        }

        (self.program, self.args)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_nu_shell_variable_substitution() {
        let shell = Shell::Program("nu".to_owned());
        let shell_builder = ShellBuilder::new(&shell, false);

        let (program, args) = shell_builder.build(
            Some("echo".into()),
            &[
                "${hello}".to_string(),
                "$world".to_string(),
                "nothing".to_string(),
                "--$something".to_string(),
                "$".to_string(),
                "${test".to_string(),
            ],
        );

        assert_eq!(program, "nu");
        assert_eq!(
            args,
            vec![
                "-i",
                "-c",
                "echo $env.hello $env.world nothing --($env.something) $ ${test"
            ]
        );
    }

    #[test]
    fn redirect_stdin_to_dev_null_precedence() {
        let shell = Shell::Program("nu".to_owned());
        let shell_builder = ShellBuilder::new(&shell, false);

        let (program, args) = shell_builder
            .redirect_stdin_to_dev_null()
            .build(Some("echo".into()), &["nothing".to_string()]);

        assert_eq!(program, "nu");
        assert_eq!(args, vec!["-i", "-c", "(echo nothing) </dev/null"]);
    }

    #[test]
    fn redirect_stdin_to_dev_null_fish() {
        let shell = Shell::Program("fish".to_owned());
        let shell_builder = ShellBuilder::new(&shell, false);

        let (program, args) = shell_builder
            .redirect_stdin_to_dev_null()
            .build(Some("echo".into()), &["test".to_string()]);

        assert_eq!(program, "fish");
        assert_eq!(args, vec!["-i", "-c", "begin; echo test; end </dev/null"]);
    }

    #[test]
    fn quotes_special_characters_in_args() {
        let shell = Shell::Program("bash".to_owned());
        let shell_builder = ShellBuilder::new(&shell, false);

        let (program, args) = shell_builder.build(
            Some("/usr/bin/sandbox-exec".into()),
            &[
                "-p".to_string(),
                "(version 1)(allow default)".to_string(),
                "--".to_string(),
                "/bin/bash".to_string(),
                "-l".to_string(),
                "-c".to_string(),
                "ls -la".to_string(),
            ],
        );

        assert_eq!(program, "bash");
        assert_eq!(args[0], "-i");
        assert_eq!(args[1], "-c");
        assert!(
            args[2].contains("-p '(version 1)(allow default)'"),
            "combined command should quote policy argument: {}",
            args[2]
        );
        assert!(
            args[2].ends_with("-c 'ls -la'"),
            "combined command should quote trailing argument: {}",
            args[2]
        );
    }
}
