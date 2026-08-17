use crate::OpenError;
use std::ffi::OsStr;
use std::io;
use std::io::Write;
use std::process::{Child, Command, Stdio};

const XDG_OPEN_SCRIPT: &[u8] = include_bytes!("xdg-open");

pub(crate) fn open(path: &OsStr) -> Result<(), OpenError> {
    if crate::is_wsl() {
        wsl_open(path)
    } else {
        non_wsl_open(path)
    }
}

#[cfg(all(feature = "reveal", target_os = "linux"))]
pub(crate) fn reveal(path: &std::path::Path) -> Result<(), OpenError> {
    if crate::is_wsl() {
        reveal_in_windows_explorer(path)
    } else {
        crate::freedesktop::reveal_with_dbus(path).or_else(|_| reveal_fallback(path))
    }
}

#[cfg(all(feature = "reveal", not(target_os = "linux")))]
pub(crate) fn reveal(path: &std::path::Path) -> Result<(), OpenError> {
    reveal_fallback(path)
}

#[cfg(feature = "reveal")]
fn reveal_fallback(path: &std::path::Path) -> Result<(), OpenError> {
    let path = path.canonicalize().map_err(OpenError::Io)?;
    let parent = path.parent().unwrap_or(std::path::Path::new("/"));
    open(parent.as_os_str())
}

fn wsl_open(path: &OsStr) -> Result<(), OpenError> {
    let result = open_with_wslview(path);
    if let Ok(mut child) = result {
        return crate::wait_child(&mut child, "wslview");
    }

    open_with_system_xdg_open(path).map_err(|err| OpenError::Spawn {
        cmds: "wslview, xdg-open".into(),
        source: err,
    })?;

    Ok(())
}

fn non_wsl_open(path: &OsStr) -> Result<(), OpenError> {
    if open_with_system_xdg_open(path).is_err() {
        open_with_internal_xdg_open(path)?;
    }

    Ok(())
}

fn open_with_wslview(path: &OsStr) -> io::Result<Child> {
    Command::new("wslview")
        .arg(path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
}

fn open_with_system_xdg_open(path: &OsStr) -> io::Result<Child> {
    Command::new("xdg-open")
        .arg(path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
}

fn open_with_internal_xdg_open(path: &OsStr) -> Result<Child, OpenError> {
    let mut sh = Command::new("sh")
        .arg("-s")
        .arg(path)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|err| OpenError::Spawn {
            cmds: "sh".into(),
            source: err,
        })?;

    sh.stdin
        .as_mut()
        .unwrap()
        .write_all(XDG_OPEN_SCRIPT)
        .map_err(OpenError::Io)?;

    Ok(sh)
}

#[cfg(all(feature = "reveal", target_os = "linux"))]
fn reveal_in_windows_explorer(path: &std::path::Path) -> Result<(), OpenError> {
    let converted_path = crate::wsl_to_windows_path(path.as_os_str());
    let converted_path = converted_path.as_deref();
    let path = match converted_path {
        None => path,
        Some(x) => std::path::Path::new(x),
    };
    Command::new("explorer.exe")
        .arg("/select,")
        .arg(path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|err| OpenError::Spawn {
            cmds: "explorer.exe".into(),
            source: err,
        })?;
    Ok(())
}

#[cfg(target_os = "linux")]
pub(crate) fn is_wsl() -> bool {
    if is_docker() {
        return false;
    }

    if let Ok(true) = std::fs::read_to_string("/proc/sys/kernel/osrelease")
        .map(|osrelease| osrelease.to_ascii_lowercase().contains("microsoft"))
    {
        return true;
    }

    if let Ok(true) = std::fs::read_to_string("/proc/version")
        .map(|version| version.to_ascii_lowercase().contains("microsoft"))
    {
        return true;
    }

    false
}

#[cfg(target_os = "linux")]
fn is_docker() -> bool {
    let has_docker_env = std::fs::metadata("/.dockerenv").is_ok();

    let has_docker_cgroup = std::fs::read_to_string("/proc/self/cgroup")
        .map(|cgroup| cgroup.to_ascii_lowercase().contains("docker"))
        .unwrap_or(false);

    has_docker_env || has_docker_cgroup
}
