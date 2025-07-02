use crate::Shell;

#[cfg(target_os = "windows")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WindowsShellType {
    Powershell,
    Cmd,
    Other,
}

/// ShellBuilder is used to turn a user-requested task into a
/// program that can be executed by the shell.
pub struct ShellBuilder {
    /// The shell to run
    program: String,
    args: Vec<String>,
    interactive: bool,
}

pub static DEFAULT_REMOTE_SHELL: &str = "\"${SHELL:-sh}\"";

impl ShellBuilder {
    /// Create a new ShellBuilder as configured.
    pub fn new(is_local: bool, shell: &Shell) -> Self {
        let (program, args) = match shell {
            Shell::System => {
                if is_local {
                    (Self::system_shell(), Vec::new())
                } else {
                    (DEFAULT_REMOTE_SHELL.to_string(), Vec::new())
                }
            }
            Shell::Program(shell) => (shell.clone(), Vec::new()),
            Shell::WithArguments { program, args, .. } => (program.clone(), args.clone()),
        };
        Self {
            program,
            args,
            interactive: true,
        }
    }
    pub fn non_interactive(mut self) -> Self {
        self.interactive = false;
        self
    }
}

#[cfg(not(target_os = "windows"))]
impl ShellBuilder {
    /// Returns the label to show in the terminal tab
    pub fn command_label(&self, command_label: &str) -> String {
        let interactivity = self.interactive.then_some("-i ").unwrap_or_default();
        format!("{} {interactivity}-c '{}'", self.program, command_label)
    }

    /// Returns the program and arguments to run this task in a shell.
    pub fn build(mut self, task_command: String, task_args: &Vec<String>) -> (String, Vec<String>) {
        let combined_command = task_args
            .into_iter()
            .fold(task_command, |mut command, arg| {
                command.push(' ');
                command.push_str(&arg);
                command
            });
        self.args.extend(
            self.interactive
                .then(|| "-i".to_owned())
                .into_iter()
                .chain(["-c".to_owned(), combined_command]),
        );

        (self.program, self.args)
    }

    fn system_shell() -> String {
        std::env::var("SHELL").unwrap_or("/bin/sh".to_string())
    }
}

#[cfg(target_os = "windows")]
impl ShellBuilder {
    /// Returns the label to show in the terminal tab
    pub fn command_label(&self, command_label: &str) -> String {
        match self.windows_shell_type() {
            WindowsShellType::Powershell => {
                format!("{} -C '{}'", self.program, command_label)
            }
            WindowsShellType::Cmd => {
                format!("{} /C '{}'", self.program, command_label)
            }
            WindowsShellType::Other => {
                format!("{} -i -c '{}'", self.program, command_label)
            }
        }
    }

    /// Returns the program and arguments to run this task in a shell.
    pub fn build(mut self, task_command: String, task_args: &Vec<String>) -> (String, Vec<String>) {
        let combined_command = task_args
            .into_iter()
            .fold(task_command, |mut command, arg| {
                command.push(' ');
                command.push_str(&self.to_windows_shell_variable(arg.to_string()));
                command
            });

        match self.windows_shell_type() {
            WindowsShellType::Powershell => self.args.extend(["-C".to_owned(), combined_command]),
            WindowsShellType::Cmd => self.args.extend(["/C".to_owned(), combined_command]),
            WindowsShellType::Other => {
                self.args
                    .extend(["-i".to_owned(), "-c".to_owned(), combined_command])
            }
        }

        (self.program, self.args)
    }
    fn windows_shell_type(&self) -> WindowsShellType {
        if self.program == "powershell"
            || self.program.ends_with("powershell.exe")
            || self.program == "pwsh"
            || self.program.ends_with("pwsh.exe")
        {
            WindowsShellType::Powershell
        } else if self.program == "cmd" || self.program.ends_with("cmd.exe") {
            WindowsShellType::Cmd
        } else {
            // Someother shell detected, the user might install and use a
            // unix-like shell.
            WindowsShellType::Other
        }
    }

    // `alacritty_terminal` uses this as default on Windows. See:
    // https://github.com/alacritty/alacritty/blob/0d4ab7bca43213d96ddfe40048fc0f922543c6f8/alacritty_terminal/src/tty/windows/mod.rs#L130
    // We could use `util::get_windows_system_shell()` here, but we are running tasks here, so leave it to `powershell.exe`
    // should be okay.
    fn system_shell() -> String {
        "powershell.exe".to_string()
    }

    fn to_windows_shell_variable(&self, input: String) -> String {
        match self.windows_shell_type() {
            WindowsShellType::Powershell => Self::to_powershell_variable(input),
            WindowsShellType::Cmd => Self::to_cmd_variable(input),
            WindowsShellType::Other => input,
        }
    }

    fn to_cmd_variable(input: String) -> String {
        if let Some(var_str) = input.strip_prefix("${") {
            if var_str.find(':').is_none() {
                // If the input starts with "${", remove the trailing "}"
                format!("%{}%", &var_str[..var_str.len() - 1])
            } else {
                // `${SOME_VAR:-SOME_DEFAULT}`, we currently do not handle this situation,
                // which will result in the task failing to run in such cases.
                input
            }
        } else if let Some(var_str) = input.strip_prefix('$') {
            // If the input starts with "$", directly append to "$env:"
            format!("%{}%", var_str)
        } else {
            // If no prefix is found, return the input as is
            input
        }
    }

    fn to_powershell_variable(input: String) -> String {
        if let Some(var_str) = input.strip_prefix("${") {
            if var_str.find(':').is_none() {
                // If the input starts with "${", remove the trailing "}"
                format!("$env:{}", &var_str[..var_str.len() - 1])
            } else {
                // `${SOME_VAR:-SOME_DEFAULT}`, we currently do not handle this situation,
                // which will result in the task failing to run in such cases.
                input
            }
        } else if let Some(var_str) = input.strip_prefix('$') {
            // If the input starts with "$", directly append to "$env:"
            format!("$env:{}", var_str)
        } else {
            // If no prefix is found, return the input as is
            input
        }
    }
}
