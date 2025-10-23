use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt, path::Path, sync::LazyLock};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShellKind {
    #[default]
    Posix,
    Csh,
    Tcsh,
    Rc,
    Fish,
    PowerShell,
    Nushell,
    Cmd,
    Xonsh,
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

/// Get the default system shell, preferring git-bash on Windows.
pub fn get_default_system_shell_preferring_bash() -> String {
    if cfg!(windows) {
        get_windows_git_bash().unwrap_or_else(|| get_windows_system_shell())
    } else {
        "/bin/sh".to_string()
    }
}

pub fn get_windows_git_bash() -> Option<String> {
    static GIT_BASH: LazyLock<Option<String>> = LazyLock::new(|| {
        // /path/to/git/cmd/git.exe/../../bin/bash.exe
        let git = which::which("git").ok()?;
        let git_bash = git.parent()?.parent()?.join("bin").join("bash.exe");
        if git_bash.is_file() {
            log::info!("Found git-bash at {}", git_bash.display());
            Some(git_bash.to_string_lossy().to_string())
        } else {
            None
        }
    });

    (*GIT_BASH).clone()
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
        find_pwsh_in_programfiles(false, false)
            .or_else(|| find_pwsh_in_programfiles(true, false))
            .or_else(|| find_pwsh_in_msix(false))
            .or_else(|| find_pwsh_in_programfiles(false, true))
            .or_else(|| find_pwsh_in_msix(true))
            .or_else(|| find_pwsh_in_programfiles(true, true))
            .or_else(find_pwsh_in_scoop)
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or("powershell.exe".to_string())
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
            ShellKind::Nushell => write!(f, "nu"),
            ShellKind::Cmd => write!(f, "cmd"),
            ShellKind::Rc => write!(f, "rc"),
            ShellKind::Xonsh => write!(f, "xonsh"),
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

        if program == "powershell" || program == "pwsh" {
            ShellKind::PowerShell
        } else if program == "cmd" {
            ShellKind::Cmd
        } else if program == "nu" {
            ShellKind::Nushell
        } else if program == "fish" {
            ShellKind::Fish
        } else if program == "csh" {
            ShellKind::Csh
        } else if program == "tcsh" {
            ShellKind::Tcsh
        } else if program == "rc" {
            ShellKind::Rc
        } else if program == "xonsh" {
            ShellKind::Xonsh
        } else if program == "sh" || program == "bash" {
            ShellKind::Posix
        } else {
            if is_windows {
                ShellKind::PowerShell
            } else {
                // Some other shell detected, the user might install and use a
                // unix-like shell.
                ShellKind::Posix
            }
        }
    }

    pub fn to_shell_variable(self, input: &str) -> String {
        match self {
            Self::PowerShell => Self::to_powershell_variable(input),
            Self::Cmd => Self::to_cmd_variable(input),
            Self::Posix => input.to_owned(),
            Self::Fish => input.to_owned(),
            Self::Csh => input.to_owned(),
            Self::Tcsh => input.to_owned(),
            Self::Rc => input.to_owned(),
            Self::Nushell => Self::to_nushell_variable(input),
            Self::Xonsh => input.to_owned(),
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
            ShellKind::PowerShell => vec!["-C".to_owned(), combined_command],
            ShellKind::Cmd => vec!["/C".to_owned(), combined_command],
            ShellKind::Posix
            | ShellKind::Nushell
            | ShellKind::Fish
            | ShellKind::Csh
            | ShellKind::Tcsh
            | ShellKind::Rc
            | ShellKind::Xonsh => interactive
                .then(|| "-i".to_owned())
                .into_iter()
                .chain(["-c".to_owned(), combined_command])
                .collect(),
        }
    }

    pub const fn command_prefix(&self) -> Option<char> {
        match self {
            ShellKind::PowerShell => Some('&'),
            ShellKind::Nushell => Some('^'),
            _ => None,
        }
    }

    pub const fn sequential_commands_separator(&self) -> char {
        match self {
            ShellKind::Cmd => '&',
            _ => ';',
        }
    }

    pub fn try_quote<'a>(&self, arg: &'a str) -> Option<Cow<'a, str>> {
        shlex::try_quote(arg).ok().map(|arg| match self {
            // If we are running in PowerShell, we want to take extra care when escaping strings.
            // In particular, we want to escape strings with a backtick (`) rather than a backslash (\).
            // TODO double escaping backslashes is not necessary in PowerShell and probably CMD
            ShellKind::PowerShell => Cow::Owned(arg.replace("\\\"", "`\"")),
            _ => arg,
        })
    }

    pub const fn activate_keyword(&self) -> &'static str {
        match self {
            ShellKind::Cmd => "",
            ShellKind::Nushell => "overlay use",
            ShellKind::PowerShell => ".",
            ShellKind::Fish => "source",
            ShellKind::Csh => "source",
            ShellKind::Tcsh => "source",
            ShellKind::Posix | ShellKind::Rc => "source",
            ShellKind::Xonsh => "source",
        }
    }

    pub const fn clear_screen_command(&self) -> &'static str {
        match self {
            ShellKind::Cmd => "cls",
            _ => "clear",
        }
    }
}
