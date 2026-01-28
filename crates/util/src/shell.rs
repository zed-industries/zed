use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt, path::Path, sync::LazyLock};

/// Shell configuration to open the terminal with.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, JsonSchema, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Shell {
    /// Use the system's default terminal configuration in /etc/passwd
    #[default]
    System,
    /// Use a specific program with no arguments.
    Program(String),
    /// Use a specific program with arguments.
    WithArguments {
        /// The program to run.
        program: String,
        /// The arguments to pass to the program.
        args: Vec<String>,
        /// An optional string to override the title of the terminal tab
        title_override: Option<String>,
    },
}

impl Shell {
    pub fn program(&self) -> String {
        match self {
            Shell::Program(program) => program.clone(),
            Shell::WithArguments { program, .. } => program.clone(),
            Shell::System => get_system_shell(),
        }
    }

    pub fn program_and_args(&self) -> (String, &[String]) {
        match self {
            Shell::Program(program) => (program.clone(), &[]),
            Shell::WithArguments { program, args, .. } => (program.clone(), args),
            Shell::System => (get_system_shell(), &[]),
        }
    }

    pub fn shell_kind(&self, is_windows: bool) -> ShellKind {
        match self {
            Shell::Program(program) => ShellKind::new(program, is_windows),
            Shell::WithArguments { program, .. } => ShellKind::new(program, is_windows),
            Shell::System => ShellKind::system(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShellKind {
    #[default]
    Posix,
    Csh,
    Tcsh,
    Rc,
    Fish,
    /// Pre-installed "legacy" powershell for windows
    PowerShell,
    /// PowerShell 7.x
    Pwsh,
    Nushell,
    Cmd,
    Xonsh,
    Elvish,
}

pub fn get_system_shell() -> String {
    if cfg!(windows) {
        get_windows_system_shell()
    } else {
        std::env::var("SHELL").unwrap_or("/bin/sh".to_string())
    }
}

pub fn get_default_system_shell() -> String {
    if cfg!(windows) {
        get_windows_system_shell()
    } else {
        "/bin/sh".to_string()
    }
}

/// Get the default system shell, preferring bash on Windows.
pub fn get_default_system_shell_preferring_bash() -> String {
    if cfg!(windows) {
        get_windows_bash().unwrap_or_else(|| get_windows_system_shell())
    } else {
        "/bin/sh".to_string()
    }
}

pub fn get_windows_bash() -> Option<String> {
    use std::path::PathBuf;

    fn find_bash_in_scoop() -> Option<PathBuf> {
        let bash_exe =
            PathBuf::from(std::env::var_os("USERPROFILE")?).join("scoop\\shims\\bash.exe");
        bash_exe.exists().then_some(bash_exe)
    }

    fn find_bash_in_git() -> Option<PathBuf> {
        // /path/to/git/cmd/git.exe/../../bin/bash.exe
        let git = which::which("git").ok()?;
        let git_bash = git.parent()?.parent()?.join("bin").join("bash.exe");
        git_bash.exists().then_some(git_bash)
    }

    static BASH: LazyLock<Option<String>> = LazyLock::new(|| {
        let bash = find_bash_in_scoop()
            .or_else(|| find_bash_in_git())
            .map(|p| p.to_string_lossy().into_owned());
        if let Some(ref path) = bash {
            log::info!("Found bash at {}", path);
        }
        bash
    });

    (*BASH).clone()
}

pub fn get_windows_system_shell() -> String {
    use std::path::PathBuf;

    fn find_pwsh_in_programfiles(find_alternate: bool, find_preview: bool) -> Option<PathBuf> {
        #[cfg(target_pointer_width = "64")]
        let env_var = if find_alternate {
            "ProgramFiles(x86)"
        } else {
            "ProgramFiles"
        };

        #[cfg(target_pointer_width = "32")]
        let env_var = if find_alternate {
            "ProgramW6432"
        } else {
            "ProgramFiles"
        };

        let install_base_dir = PathBuf::from(std::env::var_os(env_var)?).join("PowerShell");
        install_base_dir
            .read_dir()
            .ok()?
            .filter_map(Result::ok)
            .filter(|entry| matches!(entry.file_type(), Ok(ft) if ft.is_dir()))
            .filter_map(|entry| {
                let dir_name = entry.file_name();
                let dir_name = dir_name.to_string_lossy();

                let version = if find_preview {
                    let dash_index = dir_name.find('-')?;
                    if &dir_name[dash_index + 1..] != "preview" {
                        return None;
                    };
                    dir_name[..dash_index].parse::<u32>().ok()?
                } else {
                    dir_name.parse::<u32>().ok()?
                };

                let exe_path = entry.path().join("pwsh.exe");
                if exe_path.exists() {
                    Some((version, exe_path))
                } else {
                    None
                }
            })
            .max_by_key(|(version, _)| *version)
            .map(|(_, path)| path)
    }

    fn find_pwsh_in_msix(find_preview: bool) -> Option<PathBuf> {
        let msix_app_dir =
            PathBuf::from(std::env::var_os("LOCALAPPDATA")?).join("Microsoft\\WindowsApps");
        if !msix_app_dir.exists() {
            return None;
        }

        let prefix = if find_preview {
            "Microsoft.PowerShellPreview_"
        } else {
            "Microsoft.PowerShell_"
        };
        msix_app_dir
            .read_dir()
            .ok()?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                if !matches!(entry.file_type(), Ok(ft) if ft.is_dir()) {
                    return None;
                }

                if !entry.file_name().to_string_lossy().starts_with(prefix) {
                    return None;
                }

                let exe_path = entry.path().join("pwsh.exe");
                exe_path.exists().then_some(exe_path)
            })
            .next()
    }

    fn find_pwsh_in_scoop() -> Option<PathBuf> {
        let pwsh_exe =
            PathBuf::from(std::env::var_os("USERPROFILE")?).join("scoop\\shims\\pwsh.exe");
        pwsh_exe.exists().then_some(pwsh_exe)
    }

    static SYSTEM_SHELL: LazyLock<String> = LazyLock::new(|| {
        let locations = [
            || find_pwsh_in_programfiles(false, false),
            || find_pwsh_in_programfiles(true, false),
            || find_pwsh_in_msix(false),
            || find_pwsh_in_programfiles(false, true),
            || find_pwsh_in_msix(true),
            || find_pwsh_in_programfiles(true, true),
            || find_pwsh_in_scoop(),
            || which::which_global("pwsh.exe").ok(),
            || which::which_global("powershell.exe").ok(),
        ];

        locations
            .into_iter()
            .find_map(|f| f())
            .map(|p| p.to_string_lossy().trim().to_owned())
            .inspect(|shell| log::info!("Found powershell in: {}", shell))
            .unwrap_or_else(|| {
                log::warn!("Powershell not found, falling back to `cmd`");
                "cmd.exe".to_string()
            })
    });

    (*SYSTEM_SHELL).clone()
}

impl fmt::Display for ShellKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShellKind::Posix => write!(f, "sh"),
            ShellKind::Csh => write!(f, "csh"),
            ShellKind::Tcsh => write!(f, "tcsh"),
            ShellKind::Fish => write!(f, "fish"),
            ShellKind::PowerShell => write!(f, "powershell"),
            ShellKind::Pwsh => write!(f, "pwsh"),
            ShellKind::Nushell => write!(f, "nu"),
            ShellKind::Cmd => write!(f, "cmd"),
            ShellKind::Rc => write!(f, "rc"),
            ShellKind::Xonsh => write!(f, "xonsh"),
            ShellKind::Elvish => write!(f, "elvish"),
        }
    }
}

impl ShellKind {
    pub fn system() -> Self {
        Self::new(&get_system_shell(), cfg!(windows))
    }

    pub fn new(program: impl AsRef<Path>, is_windows: bool) -> Self {
        let program = program.as_ref();
        let program = program
            .file_stem()
            .unwrap_or_else(|| program.as_os_str())
            .to_string_lossy();

        match &*program {
            "powershell" => ShellKind::PowerShell,
            "pwsh" => ShellKind::Pwsh,
            "cmd" => ShellKind::Cmd,
            "nu" => ShellKind::Nushell,
            "fish" => ShellKind::Fish,
            "csh" => ShellKind::Csh,
            "tcsh" => ShellKind::Tcsh,
            "rc" => ShellKind::Rc,
            "xonsh" => ShellKind::Xonsh,
            "elvish" => ShellKind::Elvish,
            "sh" | "bash" | "zsh" => ShellKind::Posix,
            _ if is_windows => ShellKind::PowerShell,
            // Some other shell detected, the user might install and use a
            // unix-like shell.
            _ => ShellKind::Posix,
        }
    }

    pub fn to_shell_variable(self, input: &str) -> String {
        match self {
            Self::PowerShell | Self::Pwsh => Self::to_powershell_variable(input),
            Self::Cmd => Self::to_cmd_variable(input),
            Self::Posix => input.to_owned(),
            Self::Fish => input.to_owned(),
            Self::Csh => input.to_owned(),
            Self::Tcsh => input.to_owned(),
            Self::Rc => input.to_owned(),
            Self::Nushell => Self::to_nushell_variable(input),
            Self::Xonsh => input.to_owned(),
            Self::Elvish => input.to_owned(),
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

    pub fn args_for_shell(&self, interactive: bool, combined_command: String) -> Vec<String> {
        match self {
            ShellKind::PowerShell | ShellKind::Pwsh => vec!["-C".to_owned(), combined_command],
            ShellKind::Cmd => vec![
                "/S".to_owned(),
                "/C".to_owned(),
                format!("\"{combined_command}\""),
            ],
            ShellKind::Posix
            | ShellKind::Nushell
            | ShellKind::Fish
            | ShellKind::Csh
            | ShellKind::Tcsh
            | ShellKind::Rc
            | ShellKind::Xonsh
            | ShellKind::Elvish => interactive
                .then(|| "-i".to_owned())
                .into_iter()
                .chain(["-c".to_owned(), combined_command])
                .collect(),
        }
    }

    pub const fn command_prefix(&self) -> Option<char> {
        match self {
            ShellKind::PowerShell | ShellKind::Pwsh => Some('&'),
            ShellKind::Nushell => Some('^'),
            ShellKind::Posix
            | ShellKind::Csh
            | ShellKind::Tcsh
            | ShellKind::Rc
            | ShellKind::Fish
            | ShellKind::Cmd
            | ShellKind::Xonsh
            | ShellKind::Elvish => None,
        }
    }

    pub fn prepend_command_prefix<'a>(&self, command: &'a str) -> Cow<'a, str> {
        match self.command_prefix() {
            Some(prefix) if !command.starts_with(prefix) => {
                Cow::Owned(format!("{prefix}{command}"))
            }
            _ => Cow::Borrowed(command),
        }
    }

    pub const fn sequential_commands_separator(&self) -> char {
        match self {
            ShellKind::Cmd => '&',
            ShellKind::Posix
            | ShellKind::Csh
            | ShellKind::Tcsh
            | ShellKind::Rc
            | ShellKind::Fish
            | ShellKind::PowerShell
            | ShellKind::Pwsh
            | ShellKind::Nushell
            | ShellKind::Xonsh
            | ShellKind::Elvish => ';',
        }
    }

    pub const fn sequential_and_commands_separator(&self) -> &'static str {
        match self {
            ShellKind::Cmd
            | ShellKind::Posix
            | ShellKind::Csh
            | ShellKind::Tcsh
            | ShellKind::Rc
            | ShellKind::Fish
            | ShellKind::Pwsh
            | ShellKind::Xonsh => "&&",
            ShellKind::PowerShell | ShellKind::Nushell | ShellKind::Elvish => ";",
        }
    }

    pub fn try_quote<'a>(&self, arg: &'a str) -> Option<Cow<'a, str>> {
        match self {
            ShellKind::PowerShell => Some(Self::quote_powershell(arg)),
            ShellKind::Pwsh => Some(Self::quote_pwsh(arg)),
            ShellKind::Cmd => Some(Self::quote_cmd(arg)),
            ShellKind::Posix
            | ShellKind::Csh
            | ShellKind::Tcsh
            | ShellKind::Rc
            | ShellKind::Fish
            | ShellKind::Nushell
            | ShellKind::Xonsh
            | ShellKind::Elvish => shlex::try_quote(arg).ok(),
        }
    }

    fn quote_windows(arg: &str, enclose: bool) -> Cow<'_, str> {
        if arg.is_empty() {
            return Cow::Borrowed("\"\"");
        }

        let needs_quoting = arg.chars().any(|c| c == ' ' || c == '\t' || c == '"');
        if !needs_quoting {
            return Cow::Borrowed(arg);
        }

        let mut result = String::with_capacity(arg.len() + 2);

        if enclose {
            result.push('"');
        }

        let chars: Vec<char> = arg.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            if chars[i] == '\\' {
                let mut num_backslashes = 0;
                while i < chars.len() && chars[i] == '\\' {
                    num_backslashes += 1;
                    i += 1;
                }

                if i < chars.len() && chars[i] == '"' {
                    // Backslashes followed by quote: double the backslashes and escape the quote
                    for _ in 0..(num_backslashes * 2 + 1) {
                        result.push('\\');
                    }
                    result.push('"');
                    i += 1;
                } else if i >= chars.len() {
                    // Trailing backslashes: double them (they precede the closing quote)
                    for _ in 0..(num_backslashes * 2) {
                        result.push('\\');
                    }
                } else {
                    // Backslashes not followed by quote: output as-is
                    for _ in 0..num_backslashes {
                        result.push('\\');
                    }
                }
            } else if chars[i] == '"' {
                // Quote not preceded by backslash: escape it
                result.push('\\');
                result.push('"');
                i += 1;
            } else {
                result.push(chars[i]);
                i += 1;
            }
        }

        if enclose {
            result.push('"');
        }
        Cow::Owned(result)
    }

    fn needs_quoting_powershell(s: &str) -> bool {
        s.is_empty()
            || s.chars().any(|c| {
                c.is_whitespace()
                    || matches!(
                        c,
                        '"' | '`'
                            | '$'
                            | '&'
                            | '|'
                            | '<'
                            | '>'
                            | ';'
                            | '('
                            | ')'
                            | '['
                            | ']'
                            | '{'
                            | '}'
                            | ','
                            | '\''
                            | '@'
                    )
            })
    }

    fn need_quotes_powershell(arg: &str) -> bool {
        let mut quote_count = 0;
        for c in arg.chars() {
            if c == '"' {
                quote_count += 1;
            } else if c.is_whitespace() && (quote_count % 2 == 0) {
                return true;
            }
        }
        false
    }

    fn escape_powershell_quotes(s: &str) -> String {
        let mut result = String::with_capacity(s.len() + 4);
        result.push('\'');
        for c in s.chars() {
            if c == '\'' {
                result.push('\'');
            }
            result.push(c);
        }
        result.push('\'');
        result
    }

    pub fn quote_powershell(arg: &str) -> Cow<'_, str> {
        let ps_will_quote = Self::need_quotes_powershell(arg);
        let crt_quoted = Self::quote_windows(arg, !ps_will_quote);

        if !Self::needs_quoting_powershell(arg) {
            return crt_quoted;
        }

        Cow::Owned(Self::escape_powershell_quotes(&crt_quoted))
    }

    pub fn quote_pwsh(arg: &str) -> Cow<'_, str> {
        if arg.is_empty() {
            return Cow::Borrowed("''");
        }

        if !Self::needs_quoting_powershell(arg) {
            return Cow::Borrowed(arg);
        }

        Cow::Owned(Self::escape_powershell_quotes(arg))
    }

    pub fn quote_cmd(arg: &str) -> Cow<'_, str> {
        let crt_quoted = Self::quote_windows(arg, true);

        let needs_cmd_escaping = crt_quoted.contains(['"', '%', '^', '<', '>', '&', '|', '(', ')']);

        if !needs_cmd_escaping {
            return crt_quoted;
        }

        let mut result = String::with_capacity(crt_quoted.len() * 2);
        for c in crt_quoted.chars() {
            match c {
                '^' | '"' | '<' | '>' | '&' | '|' | '(' | ')' => {
                    result.push('^');
                    result.push(c);
                }
                '%' => {
                    result.push_str("%%cd:~,%");
                }
                _ => result.push(c),
            }
        }
        Cow::Owned(result)
    }

    /// Quotes the given argument if necessary, taking into account the command prefix.
    ///
    /// In other words, this will consider quoting arg without its command prefix to not break the command.
    /// You should use this over `try_quote` when you want to quote a shell command.
    pub fn try_quote_prefix_aware<'a>(&self, arg: &'a str) -> Option<Cow<'a, str>> {
        if let Some(char) = self.command_prefix() {
            if let Some(arg) = arg.strip_prefix(char) {
                // we have a command that is prefixed
                for quote in ['\'', '"'] {
                    if let Some(arg) = arg
                        .strip_prefix(quote)
                        .and_then(|arg| arg.strip_suffix(quote))
                    {
                        // and the command itself is wrapped as a literal, that
                        // means the prefix exists to interpret a literal as a
                        // command. So strip the quotes, quote the command, and
                        // re-add the quotes if they are missing after requoting
                        let quoted = self.try_quote(arg)?;
                        return Some(if quoted.starts_with(['\'', '"']) {
                            Cow::Owned(self.prepend_command_prefix(&quoted).into_owned())
                        } else {
                            Cow::Owned(
                                self.prepend_command_prefix(&format!("{quote}{quoted}{quote}"))
                                    .into_owned(),
                            )
                        });
                    }
                }
                return self
                    .try_quote(arg)
                    .map(|quoted| Cow::Owned(self.prepend_command_prefix(&quoted).into_owned()));
            }
        }
        self.try_quote(arg).map(|quoted| match quoted {
            unquoted @ Cow::Borrowed(_) => unquoted,
            Cow::Owned(quoted) => Cow::Owned(self.prepend_command_prefix(&quoted).into_owned()),
        })
    }

    pub fn split(&self, input: &str) -> Option<Vec<String>> {
        shlex::split(input)
    }

    pub const fn activate_keyword(&self) -> &'static str {
        match self {
            ShellKind::Cmd => "",
            ShellKind::Nushell => "overlay use",
            ShellKind::PowerShell | ShellKind::Pwsh => ".",
            ShellKind::Fish
            | ShellKind::Csh
            | ShellKind::Tcsh
            | ShellKind::Posix
            | ShellKind::Rc
            | ShellKind::Xonsh
            | ShellKind::Elvish => "source",
        }
    }

    pub const fn clear_screen_command(&self) -> &'static str {
        match self {
            ShellKind::Cmd => "cls",
            ShellKind::Posix
            | ShellKind::Csh
            | ShellKind::Tcsh
            | ShellKind::Rc
            | ShellKind::Fish
            | ShellKind::PowerShell
            | ShellKind::Pwsh
            | ShellKind::Nushell
            | ShellKind::Xonsh
            | ShellKind::Elvish => "clear",
        }
    }

    #[cfg(windows)]
    /// We do not want to escape arguments if we are using CMD as our shell.
    /// If we do we end up with too many quotes/escaped quotes for CMD to handle.
    pub const fn tty_escape_args(&self) -> bool {
        match self {
            ShellKind::Cmd => false,
            ShellKind::Posix
            | ShellKind::Csh
            | ShellKind::Tcsh
            | ShellKind::Rc
            | ShellKind::Fish
            | ShellKind::PowerShell
            | ShellKind::Pwsh
            | ShellKind::Nushell
            | ShellKind::Xonsh
            | ShellKind::Elvish => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Examples
    // WSL
    // wsl.exe --distribution NixOS --cd /home/user -- /usr/bin/zsh -c "echo hello"
    // wsl.exe --distribution NixOS --cd /home/user -- /usr/bin/zsh -c "\"echo hello\"" | grep hello"
    // wsl.exe --distribution NixOS --cd ~ env RUST_LOG=info,remote=debug .zed_wsl_server/zed-remote-server-dev-build proxy --identifier dev-workspace-53
    // PowerShell from Nushell
    // nu -c overlay use "C:\Users\kubko\dev\python\39007\tests\.venv\Scripts\activate.nu"; ^"C:\Program Files\PowerShell\7\pwsh.exe" -C "C:\Users\kubko\dev\python\39007\tests\.venv\Scripts\python.exe -m pytest \"test_foo.py::test_foo\""
    // PowerShell from CMD
    // cmd /C \" \"C:\\\\Users\\\\kubko\\\\dev\\\\python\\\\39007\\\\tests\\\\.venv\\\\Scripts\\\\activate.bat\"& \"C:\\\\Program Files\\\\PowerShell\\\\7\\\\pwsh.exe\" -C \"C:\\\\Users\\\\kubko\\\\dev\\\\python\\\\39007\\\\tests\\\\.venv\\\\Scripts\\\\python.exe -m pytest \\\"test_foo.py::test_foo\\\"\"\"

    #[test]
    fn test_try_quote_powershell() {
        let shell_kind = ShellKind::PowerShell;
        assert_eq!(
            shell_kind
                .try_quote("C:\\Users\\johndoe\\dev\\python\\39007\\tests\\.venv\\Scripts\\python.exe -m pytest \"test_foo.py::test_foo\"")
                .unwrap()
                .into_owned(),
            "'C:\\Users\\johndoe\\dev\\python\\39007\\tests\\.venv\\Scripts\\python.exe -m pytest \\\"test_foo.py::test_foo\\\"'".to_string()
        );
    }

    #[test]
    fn test_try_quote_cmd() {
        let shell_kind = ShellKind::Cmd;
        assert_eq!(
            shell_kind
                .try_quote("C:\\Users\\johndoe\\dev\\python\\39007\\tests\\.venv\\Scripts\\python.exe -m pytest \"test_foo.py::test_foo\"")
                .unwrap()
                .into_owned(),
            "^\"C:\\Users\\johndoe\\dev\\python\\39007\\tests\\.venv\\Scripts\\python.exe -m pytest \\^\"test_foo.py::test_foo\\^\"^\"".to_string()
        );
    }

    #[test]
    fn test_try_quote_powershell_edge_cases() {
        let shell_kind = ShellKind::PowerShell;

        // Empty string
        assert_eq!(
            shell_kind.try_quote("").unwrap().into_owned(),
            "'\"\"'".to_string()
        );

        // String without special characters (no quoting needed)
        assert_eq!(shell_kind.try_quote("simple").unwrap(), "simple");

        // String with spaces
        assert_eq!(
            shell_kind.try_quote("hello world").unwrap().into_owned(),
            "'hello world'".to_string()
        );

        // String with dollar signs
        assert_eq!(
            shell_kind.try_quote("$variable").unwrap().into_owned(),
            "'$variable'".to_string()
        );

        // String with backticks
        assert_eq!(
            shell_kind.try_quote("test`command").unwrap().into_owned(),
            "'test`command'".to_string()
        );

        // String with multiple special characters
        assert_eq!(
            shell_kind
                .try_quote("test `\"$var`\" end")
                .unwrap()
                .into_owned(),
            "'test `\\\"$var`\\\" end'".to_string()
        );

        // String with backslashes and colon (path without spaces doesn't need quoting)
        assert_eq!(
            shell_kind.try_quote("C:\\path\\to\\file").unwrap(),
            "C:\\path\\to\\file"
        );
    }

    #[test]
    fn test_try_quote_cmd_edge_cases() {
        let shell_kind = ShellKind::Cmd;

        // Empty string
        assert_eq!(
            shell_kind.try_quote("").unwrap().into_owned(),
            "^\"^\"".to_string()
        );

        // String without special characters (no quoting needed)
        assert_eq!(shell_kind.try_quote("simple").unwrap(), "simple");

        // String with spaces
        assert_eq!(
            shell_kind.try_quote("hello world").unwrap().into_owned(),
            "^\"hello world^\"".to_string()
        );

        // String with space and backslash (backslash not at end, so not doubled)
        assert_eq!(
            shell_kind.try_quote("path\\ test").unwrap().into_owned(),
            "^\"path\\ test^\"".to_string()
        );

        // String ending with backslash (must be doubled before closing quote)
        assert_eq!(
            shell_kind.try_quote("test path\\").unwrap().into_owned(),
            "^\"test path\\\\^\"".to_string()
        );

        // String ending with multiple backslashes (all doubled before closing quote)
        assert_eq!(
            shell_kind.try_quote("test path\\\\").unwrap().into_owned(),
            "^\"test path\\\\\\\\^\"".to_string()
        );

        // String with embedded quote (quote is escaped, backslash before it is doubled)
        assert_eq!(
            shell_kind.try_quote("test\\\"quote").unwrap().into_owned(),
            "^\"test\\\\\\^\"quote^\"".to_string()
        );

        // String with multiple backslashes before embedded quote (all doubled)
        assert_eq!(
            shell_kind
                .try_quote("test\\\\\"quote")
                .unwrap()
                .into_owned(),
            "^\"test\\\\\\\\\\^\"quote^\"".to_string()
        );

        // String with backslashes not before quotes (path without spaces doesn't need quoting)
        assert_eq!(
            shell_kind.try_quote("C:\\path\\to\\file").unwrap(),
            "C:\\path\\to\\file"
        );
    }

    #[test]
    fn test_try_quote_nu_command() {
        let shell_kind = ShellKind::Nushell;
        assert_eq!(
            shell_kind.try_quote("'uname'").unwrap().into_owned(),
            "\"'uname'\"".to_string()
        );
        assert_eq!(
            shell_kind
                .try_quote_prefix_aware("'uname'")
                .unwrap()
                .into_owned(),
            "^\"'uname'\"".to_string()
        );
        assert_eq!(
            shell_kind.try_quote("^uname").unwrap().into_owned(),
            "'^uname'".to_string()
        );
        assert_eq!(
            shell_kind
                .try_quote_prefix_aware("^uname")
                .unwrap()
                .into_owned(),
            "^uname".to_string()
        );
        assert_eq!(
            shell_kind.try_quote("^'uname'").unwrap().into_owned(),
            "'^'\"'uname\'\"".to_string()
        );
        assert_eq!(
            shell_kind
                .try_quote_prefix_aware("^'uname'")
                .unwrap()
                .into_owned(),
            "^'uname'".to_string()
        );
        assert_eq!(
            shell_kind.try_quote("'uname a'").unwrap().into_owned(),
            "\"'uname a'\"".to_string()
        );
        assert_eq!(
            shell_kind
                .try_quote_prefix_aware("'uname a'")
                .unwrap()
                .into_owned(),
            "^\"'uname a'\"".to_string()
        );
        assert_eq!(
            shell_kind.try_quote("^'uname a'").unwrap().into_owned(),
            "'^'\"'uname a'\"".to_string()
        );
        assert_eq!(
            shell_kind
                .try_quote_prefix_aware("^'uname a'")
                .unwrap()
                .into_owned(),
            "^'uname a'".to_string()
        );
        assert_eq!(
            shell_kind.try_quote("uname").unwrap().into_owned(),
            "uname".to_string()
        );
        assert_eq!(
            shell_kind
                .try_quote_prefix_aware("uname")
                .unwrap()
                .into_owned(),
            "uname".to_string()
        );
    }
}
