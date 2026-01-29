use std::borrow::Cow;

use crate::shell::get_system_shell;
use crate::shell::{Shell, ShellKind};

/// ShellBuilder is used to turn a user-requested task into a
/// program that can be executed by the shell.
pub struct ShellBuilder {
    /// The shell to run
    program: String,
    args: Vec<String>,
    interactive: bool,
    /// Whether to redirect stdin to /dev/null for the spawned command as a subshell.
    redirect_stdin: bool,
    kind: Option<ShellKind>,
}

impl ShellBuilder {
    /// Create a new ShellBuilder as configured.
    pub fn new(shell: &Shell, _is_windows: bool) -> Self {
        let (program, args) = match shell {
            Shell::System => (get_system_shell(), Vec::new()),
            Shell::Program(shell) => (shell.clone(), Vec::new()),
            Shell::WithArguments { program, args, .. } => (program.clone(), args.clone()),
        };

        let kind = ShellKind::new(&program);
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
                Some(ShellKind::PowerShell) | Some(ShellKind::Pwsh) => {
                    format!("{} -C '{}'", self.program, command_to_use_in_label)
                }
                #[cfg(windows)]
                None => {
                    format!("{} -C '{}'", self.program, command_to_use_in_label)
                }
                Some(ShellKind::Cmd) => {
                    format!("{} /C \"{}\"", self.program, command_to_use_in_label)
                }
                Some(
                    ShellKind::Posix(_)
                    | ShellKind::Nushell
                    | ShellKind::Fish
                    | ShellKind::Csh
                    | ShellKind::Tcsh
                    | ShellKind::Rc
                    | ShellKind::Xonsh
                    | ShellKind::Elvish,
                ) => {
                    let interactivity = self.interactive.then_some("-i ").unwrap_or_default();
                    format!(
                        "{PROGRAM} {interactivity}-c '{command_to_use_in_label}'",
                        PROGRAM = self.program
                    )
                }
                #[cfg(unix)]
                None => {
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

    fn try_quote_prefix_aware<'a>(&self, arg: &'a str) -> Option<Cow<'a, str>> {
        match self.kind {
            Some(ref kind) => kind.try_quote_prefix_aware(arg),
            #[cfg(windows)]
            None => Some(ShellKind::quote_powershell(arg)),
            #[cfg(unix)]
            None => shlex::try_quote(arg).ok(),
        }
    }

    fn to_shell_variable(&self, input: &str) -> String {
        match self.kind {
            Some(ref kind) => kind.to_shell_variable(input),
            #[cfg(windows)]
            None => ShellKind::to_powershell_variable(input),
            #[cfg(unix)]
            None => input.to_owned(),
        }
    }

    fn try_quote<'a>(&self, arg: &'a str) -> Option<Cow<'a, str>> {
        match self.kind {
            Some(ref kind) => kind.try_quote(arg),
            #[cfg(windows)]
            None => Some(ShellKind::quote_powershell(arg)),
            #[cfg(unix)]
            None => shlex::try_quote(arg).ok(),
        }
    }

    fn args_for_shell(&self, interactive: bool, combined_command: String) -> Vec<String> {
        match self.kind {
            Some(ref kind) => kind.args_for_shell(interactive, combined_command),
            #[cfg(windows)]
            None => vec!["-C".to_owned(), combined_command],
            #[cfg(unix)]
            None => interactive
                .then(|| "-i".to_owned())
                .into_iter()
                .chain(["-c".to_owned(), combined_command])
                .collect(),
        }
    }

    /// Returns the program and arguments to run this task in a shell.
    pub fn build(
        mut self,
        task_command: Option<String>,
        task_args: &[String],
    ) -> (String, Vec<String>) {
        if let Some(task_command) = task_command {
            let task_command = if !task_args.is_empty() {
                match self.try_quote_prefix_aware(&task_command) {
                    Some(task_command) => task_command.into_owned(),
                    None => task_command,
                }
            } else {
                task_command
            };
            let mut combined_command = task_args.iter().fold(task_command, |mut command, arg| {
                command.push(' ');
                let shell_variable = self.to_shell_variable(arg);
                command.push_str(&match self.try_quote(&shell_variable) {
                    Some(shell_variable) => shell_variable,
                    None => Cow::Owned(shell_variable),
                });
                command
            });
            if self.redirect_stdin {
                match self.kind {
                    Some(ShellKind::Fish) => {
                        combined_command.insert_str(0, "begin; ");
                        combined_command.push_str("; end </dev/null");
                    }
                    Some(
                        ShellKind::Posix(_)
                        | ShellKind::Nushell
                        | ShellKind::Csh
                        | ShellKind::Tcsh
                        | ShellKind::Rc
                        | ShellKind::Xonsh
                        | ShellKind::Elvish,
                    ) => {
                        combined_command.insert(0, '(');
                        combined_command.push_str(") </dev/null");
                    }
                    #[cfg(unix)]
                    None => {
                        combined_command.insert(0, '(');
                        combined_command.push_str(") </dev/null");
                    }
                    Some(ShellKind::PowerShell) | Some(ShellKind::Pwsh) => {
                        combined_command.insert_str(0, "$null | & {");
                        combined_command.push_str("}");
                    }
                    #[cfg(windows)]
                    None => {
                        combined_command.insert_str(0, "$null | & {");
                        combined_command.push_str("}");
                    }
                    Some(ShellKind::Cmd) => {
                        combined_command.push_str("< NUL");
                    }
                }
            }

            self.args
                .extend(self.args_for_shell(self.interactive, combined_command));
        }

        (self.program, self.args)
    }

    // This should not exist, but our task infra is broken beyond repair right now
    #[doc(hidden)]
    pub fn build_no_quote(
        mut self,
        task_command: Option<String>,
        task_args: &[String],
    ) -> (String, Vec<String>) {
        if let Some(task_command) = task_command {
            let mut combined_command = task_args.iter().fold(task_command, |mut command, arg| {
                command.push(' ');
                command.push_str(&self.to_shell_variable(arg));
                command
            });
            if self.redirect_stdin {
                match self.kind {
                    Some(ShellKind::Fish) => {
                        combined_command.insert_str(0, "begin; ");
                        combined_command.push_str("; end </dev/null");
                    }
                    Some(
                        ShellKind::Posix(_)
                        | ShellKind::Nushell
                        | ShellKind::Csh
                        | ShellKind::Tcsh
                        | ShellKind::Rc
                        | ShellKind::Xonsh
                        | ShellKind::Elvish,
                    ) => {
                        combined_command.insert(0, '(');
                        combined_command.push_str(") </dev/null");
                    }
                    #[cfg(unix)]
                    None => {
                        combined_command.insert(0, '(');
                        combined_command.push_str(") </dev/null");
                    }
                    Some(ShellKind::PowerShell) | Some(ShellKind::Pwsh) => {
                        combined_command.insert_str(0, "$null | & {");
                        combined_command.push_str("}");
                    }
                    #[cfg(windows)]
                    None => {
                        combined_command.insert_str(0, "$null | & {");
                        combined_command.push_str("}");
                    }
                    Some(ShellKind::Cmd) => {
                        combined_command.push_str("< NUL");
                    }
                }
            }

            self.args
                .extend(self.args_for_shell(self.interactive, combined_command));
        }

        (self.program, self.args)
    }

    /// Builds a `smol::process::Command` with the given task command and arguments.
    ///
    /// Prefer this over manually constructing a command with the output of `Self::build`,
    /// as this method handles `cmd` weirdness on windows correctly.
    pub fn build_smol_command(
        self,
        task_command: Option<String>,
        task_args: &[String],
    ) -> smol::process::Command {
        smol::process::Command::from(self.build_std_command(task_command, task_args))
    }

    /// Builds a `std::process::Command` with the given task command and arguments.
    ///
    /// Prefer this over manually constructing a command with the output of `Self::build`,
    /// as this method handles `cmd` weirdness on windows correctly.
    pub fn build_std_command(
        self,
        mut task_command: Option<String>,
        task_args: &[String],
    ) -> std::process::Command {
        #[cfg(windows)]
        let kind = self.kind;
        if task_args.is_empty() {
            task_command = task_command
                .as_ref()
                .map(|cmd| self.try_quote_prefix_aware(cmd).map(Cow::into_owned))
                .unwrap_or(task_command);
        }
        let (program, args) = self.build(task_command, task_args);

        let mut child = crate::command::new_std_command(program);

        #[cfg(windows)]
        if kind == Some(ShellKind::Cmd) {
            use std::os::windows::process::CommandExt;

            for arg in args {
                child.raw_arg(arg);
            }
        } else {
            child.args(args);
        }

        #[cfg(not(windows))]
        child.args(args);

        child
    }

    pub fn kind(&self) -> Option<ShellKind> {
        self.kind
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
                "echo '$env.hello' '$env.world' nothing '--($env.something)' '$' '${test'"
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
    fn does_not_quote_sole_command_only() {
        let shell = Shell::Program("fish".to_owned());
        let shell_builder = ShellBuilder::new(&shell, false);

        let (program, args) = shell_builder.build(Some("echo".into()), &[]);

        assert_eq!(program, "fish");
        assert_eq!(args, vec!["-i", "-c", "echo"]);

        let shell = Shell::Program("fish".to_owned());
        let shell_builder = ShellBuilder::new(&shell, false);

        let (program, args) = shell_builder.build(Some("echo oo".into()), &[]);

        assert_eq!(program, "fish");
        assert_eq!(args, vec!["-i", "-c", "echo oo"]);
    }
}
