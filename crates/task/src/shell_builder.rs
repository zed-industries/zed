use crate::Shell;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShellKind {
    #[default]
    Posix,
    Csh,
    Fish,
    Powershell,
    Nushell,
    Cmd,
}

impl ShellKind {
    pub fn system() -> Self {
        Self::new(&system_shell())
    }

    pub fn new(program: &str) -> Self {
        #[cfg(windows)]
        let (_, program) = program.rsplit_once('\\').unwrap_or(("", program));
        #[cfg(not(windows))]
        let (_, program) = program.rsplit_once('/').unwrap_or(("", program));
        if program == "powershell"
            || program == "powershell.exe"
            || program == "pwsh"
            || program == "pwsh.exe"
        {
            ShellKind::Powershell
        } else if program == "cmd" || program == "cmd.exe" {
            ShellKind::Cmd
        } else if program == "nu" {
            ShellKind::Nushell
        } else if program == "fish" {
            ShellKind::Fish
        } else if program == "csh" {
            ShellKind::Csh
        } else {
            // Someother shell detected, the user might install and use a
            // unix-like shell.
            ShellKind::Posix
        }
    }

    fn to_shell_variable(self, input: &str) -> String {
        match self {
            Self::Powershell => Self::to_powershell_variable(input),
            Self::Cmd => Self::to_cmd_variable(input),
            Self::Posix => input.to_owned(),
            Self::Fish => input.to_owned(),
            Self::Csh => input.to_owned(),
            Self::Nushell => Self::to_nushell_variable(input),
        }
    }

    fn to_cmd_variable(input: &str) -> String {
        if let Some(var_str) = input.strip_prefix("${") {
            if var_str.find(':').is_none() {
                // If the input starts with "${", remove the trailing "}"
                format!("%{}%", &var_str[..var_str.len() - 1])
            } else {
                // `${SOME_VAR:-SOME_DEFAULT}`, we currently do not handle this situation,
                // which will result in the task failing to run in such cases.
                input.into()
            }
        } else if let Some(var_str) = input.strip_prefix('$') {
            // If the input starts with "$", directly append to "$env:"
            format!("%{}%", var_str)
        } else {
            // If no prefix is found, return the input as is
            input.into()
        }
    }
    fn to_powershell_variable(input: &str) -> String {
        if let Some(var_str) = input.strip_prefix("${") {
            if var_str.find(':').is_none() {
                // If the input starts with "${", remove the trailing "}"
                format!("$env:{}", &var_str[..var_str.len() - 1])
            } else {
                // `${SOME_VAR:-SOME_DEFAULT}`, we currently do not handle this situation,
                // which will result in the task failing to run in such cases.
                input.into()
            }
        } else if let Some(var_str) = input.strip_prefix('$') {
            // If the input starts with "$", directly append to "$env:"
            format!("$env:{}", var_str)
        } else {
            // If no prefix is found, return the input as is
            input.into()
        }
    }

    fn to_nushell_variable(input: &str) -> String {
        let mut result = String::new();
        let mut source = input;
        let mut is_start = true;

        loop {
            match source.chars().next() {
                None => return result,
                Some('$') => {
                    source = Self::parse_nushell_var(&source[1..], &mut result, is_start);
                    is_start = false;
                }
                Some(_) => {
                    is_start = false;
                    let chunk_end = source.find('$').unwrap_or(source.len());
                    let (chunk, rest) = source.split_at(chunk_end);
                    result.push_str(chunk);
                    source = rest;
                }
            }
        }
    }

    fn parse_nushell_var<'a>(source: &'a str, text: &mut String, is_start: bool) -> &'a str {
        if source.starts_with("env.") {
            text.push('$');
            return source;
        }

        match source.chars().next() {
            Some('{') => {
                let source = &source[1..];
                if let Some(end) = source.find('}') {
                    let var_name = &source[..end];
                    if !var_name.is_empty() {
                        if !is_start {
                            text.push_str("(");
                        }
                        text.push_str("$env.");
                        text.push_str(var_name);
                        if !is_start {
                            text.push_str(")");
                        }
                        &source[end + 1..]
                    } else {
                        text.push_str("${}");
                        &source[end + 1..]
                    }
                } else {
                    text.push_str("${");
                    source
                }
            }
            Some(c) if c.is_alphabetic() || c == '_' => {
                let end = source
                    .find(|c: char| !c.is_alphanumeric() && c != '_')
                    .unwrap_or(source.len());
                let var_name = &source[..end];
                if !is_start {
                    text.push_str("(");
                }
                text.push_str("$env.");
                text.push_str(var_name);
                if !is_start {
                    text.push_str(")");
                }
                &source[end..]
            }
            _ => {
                text.push('$');
                source
            }
        }
    }

    fn args_for_shell(&self, interactive: bool, combined_command: String) -> Vec<String> {
        match self {
            ShellKind::Powershell => vec!["-C".to_owned(), combined_command],
            ShellKind::Cmd => vec!["/C".to_owned(), combined_command],
            ShellKind::Posix | ShellKind::Nushell | ShellKind::Fish | ShellKind::Csh => interactive
                .then(|| "-i".to_owned())
                .into_iter()
                .chain(["-c".to_owned(), combined_command])
                .collect(),
        }
    }
}

fn system_shell() -> String {
    if cfg!(target_os = "windows") {
        // `alacritty_terminal` uses this as default on Windows. See:
        // https://github.com/alacritty/alacritty/blob/0d4ab7bca43213d96ddfe40048fc0f922543c6f8/alacritty_terminal/src/tty/windows/mod.rs#L130
        // We could use `util::get_windows_system_shell()` here, but we are running tasks here, so leave it to `powershell.exe`
        // should be okay.
        "powershell.exe".to_string()
    } else {
        std::env::var("SHELL").unwrap_or("/bin/sh".to_string())
    }
}

/// ShellBuilder is used to turn a user-requested task into a
/// program that can be executed by the shell.
pub struct ShellBuilder {
    /// The shell to run
    program: String,
    args: Vec<String>,
    interactive: bool,
    kind: ShellKind,
}

impl ShellBuilder {
    /// Create a new ShellBuilder as configured.
    pub fn new(remote_system_shell: Option<&str>, shell: &Shell) -> Self {
        let (program, args) = match shell {
            Shell::System => match remote_system_shell {
                Some(remote_shell) => (remote_shell.to_string(), Vec::new()),
                None => (system_shell(), Vec::new()),
            },
            Shell::Program(shell) => (shell.clone(), Vec::new()),
            Shell::WithArguments { program, args, .. } => (program.clone(), args.clone()),
        };
        let kind = ShellKind::new(&program);
        Self {
            program,
            args,
            interactive: true,
            kind,
        }
    }
    pub fn non_interactive(mut self) -> Self {
        self.interactive = false;
        self
    }

    /// Returns the label to show in the terminal tab
    pub fn command_label(&self, command_label: &str) -> String {
        match self.kind {
            ShellKind::Powershell => {
                format!("{} -C '{}'", self.program, command_label)
            }
            ShellKind::Cmd => {
                format!("{} /C '{}'", self.program, command_label)
            }
            ShellKind::Posix | ShellKind::Nushell | ShellKind::Fish | ShellKind::Csh => {
                let interactivity = self.interactive.then_some("-i ").unwrap_or_default();
                format!(
                    "{} {interactivity}-c '$\"{}\"'",
                    self.program, command_label
                )
            }
        }
    }
    /// Returns the program and arguments to run this task in a shell.
    pub fn build(
        mut self,
        task_command: Option<String>,
        task_args: &[String],
    ) -> (String, Vec<String>) {
        if let Some(task_command) = task_command {
            let combined_command = task_args.iter().fold(task_command, |mut command, arg| {
                command.push(' ');
                command.push_str(&self.kind.to_shell_variable(arg));
                command
            });

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
        let shell_builder = ShellBuilder::new(None, &shell);

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
}
