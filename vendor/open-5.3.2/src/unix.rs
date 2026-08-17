use std::{
    env,
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
    process::Command,
};

pub fn commands<T: AsRef<OsStr>>(path: T) -> Vec<Command> {
    let path = path.as_ref();
    let mut commands: Vec<(&str, Vec<&OsStr>)> = vec![];

    let wsl_path = wsl_path(path);
    if is_wsl::is_wsl() {
        commands.push(("wslview", vec![&wsl_path]));
    }

    commands.extend_from_slice(&[
        ("xdg-open", vec![&path]),
        ("gio", vec![OsStr::new("open"), path]),
        ("gnome-open", vec![path]),
        ("kde-open", vec![path]),
    ]);

    commands
        .iter()
        .map(|(command, args)| {
            let mut cmd = Command::new(command);
            cmd.args(args);
            cmd
        })
        .collect()
}

pub fn with_command<T: AsRef<OsStr>>(path: T, app: impl Into<String>) -> Command {
    let mut cmd = Command::new(app.into());
    cmd.arg(path.as_ref());
    cmd
}

// Polyfill to workaround absolute path bug in wslu(wslview). In versions before
// v3.1.1, wslview is unable to find absolute paths. `wsl_path` converts an
// absolute path into a relative path starting from the current directory. If
// the path is already a relative path or the conversion fails the original path
// is returned.
fn wsl_path<T: AsRef<OsStr>>(path: T) -> OsString {
    fn path_relative_to_current_dir<T: AsRef<OsStr>>(path: T) -> Option<PathBuf> {
        let path = Path::new(&path);

        if path.is_relative() {
            return None;
        }

        let base = env::current_dir().ok()?;
        pathdiff::diff_paths(path, base)
    }

    match path_relative_to_current_dir(&path) {
        None => OsString::from(&path),
        Some(relative) => OsString::from(relative),
    }
}
