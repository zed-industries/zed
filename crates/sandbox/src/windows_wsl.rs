//! Windows sandbox integration via WSL.
//!
//! This deliberately only supports worktrees opened through WSL UNC paths such
//! as `\\wsl.localhost\Ubuntu\home\me\project` or `\\wsl$\Ubuntu\...`.
//! Commands are launched with `wsl.exe` into that distro, then executed under
//! Bubblewrap inside Linux. Native Windows paths and native Windows processes are
//! not sandboxed by this module.

use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result, bail, ensure};

use crate::SandboxPermissions;

#[derive(Clone, Debug, Eq, PartialEq)]
struct WslPath {
    distro: String,
    path: String,
}

/// Wrap a Linux process invocation so it runs under Bubblewrap inside WSL.
///
/// `program` and `args` must name a Linux executable and Linux argv, not a
/// Windows executable. The caller is expected to convert the model's command
/// into a Linux shell invocation (typically `/bin/sh -c ...`) before calling
/// this function.
///
/// All writable directories and the cwd must be WSL UNC paths in the same
/// distro. If any path is native Windows, missing, or in a different distro,
/// this fails closed rather than running unsandboxed.
pub fn wrap_invocation(
    program: &str,
    args: &[String],
    writable_directories: &[&Path],
    permissions: SandboxPermissions,
    cwd: Option<&Path>,
) -> Result<(String, Vec<String>)> {
    let cwd = match cwd {
        Some(cwd) => Some(path_to_wsl(cwd).with_context(|| {
            format!(
                "Windows sandboxing requires the terminal cwd to be a WSL path: {}",
                cwd.display()
            )
        })?),
        None => None,
    };

    let writable_directories = writable_directories
        .iter()
        .map(|directory| {
            path_to_wsl(directory).with_context(|| {
                format!(
                    "Windows sandboxing requires writable directories to be WSL paths: {}",
                    directory.display()
                )
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let Some(distro) = cwd.as_ref().map(|path| path.distro.as_str()).or_else(|| {
        writable_directories
            .first()
            .map(|path| path.distro.as_str())
    }) else {
        bail!("Windows sandboxing via WSL requires a WSL cwd or writable directory");
    };

    for path in &writable_directories {
        ensure!(
            path.distro == distro,
            "Windows sandboxing via WSL cannot mix WSL distros: expected `{}`, got `{}`",
            distro,
            path.distro
        );
    }

    if let Some(cwd) = &cwd {
        ensure!(
            cwd.distro == distro,
            "Windows sandboxing via WSL cannot mix WSL distros: expected `{}`, got `{}`",
            distro,
            cwd.distro
        );
    }

    let wsl_exe = wsl_exe_path();
    ensure!(
        wsl_exe.is_file(),
        "Windows sandboxing requires WSL (`wsl.exe`) to be installed"
    );

    let mut wsl_args = vec!["-d".to_string(), distro.to_string()];
    if let Some(cwd) = &cwd {
        wsl_args.extend(["--cd".to_string(), cwd.path.clone()]);
    }
    wsl_args.extend(["--exec".to_string(), "bwrap".to_string()]);
    wsl_args.extend(build_bwrap_args(
        &writable_directories,
        permissions,
        cwd.as_ref().map(|path| path.path.as_str()),
    ));
    wsl_args.push("--".to_string());
    wsl_args.push(program.to_string());
    wsl_args.extend(args.iter().cloned());

    Ok((wsl_exe.to_string_lossy().into_owned(), wsl_args))
}

fn wsl_exe_path() -> PathBuf {
    std::env::var_os("SystemRoot")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(r"C:\Windows"))
        .join("System32")
        .join("wsl.exe")
}

fn build_bwrap_args(
    writable_directories: &[WslPath],
    permissions: SandboxPermissions,
    cwd: Option<&str>,
) -> Vec<String> {
    let mut args = Vec::new();

    if permissions.allow_fs_write {
        push_bind(&mut args, "--bind", "/", "/");
    } else {
        push_bind(&mut args, "--ro-bind", "/", "/");
        args.extend(["--tmpfs".to_string(), "/tmp".to_string()]);
        for directory in writable_directories {
            push_bind(&mut args, "--bind", &directory.path, &directory.path);
        }
    }

    args.extend([
        "--dev".to_string(),
        "/dev".to_string(),
        "--proc".to_string(),
        "/proc".to_string(),
    ]);

    if !permissions.allow_network {
        args.push("--unshare-net".to_string());
    }

    args.extend([
        "--unshare-user".to_string(),
        "--unshare-ipc".to_string(),
        "--unshare-uts".to_string(),
        "--unshare-pid".to_string(),
        "--unshare-cgroup-try".to_string(),
        "--die-with-parent".to_string(),
    ]);

    if let Some(cwd) = cwd {
        args.extend(["--chdir".to_string(), cwd.to_string()]);
    }

    args
}

fn push_bind(args: &mut Vec<String>, flag: &str, source: &str, destination: &str) {
    args.extend([
        flag.to_string(),
        source.to_string(),
        destination.to_string(),
    ]);
}

fn path_to_wsl(path: &Path) -> Result<WslPath> {
    ensure!(
        path.is_dir(),
        "Windows sandboxing via WSL can only grant existing directories: {}",
        path.display()
    );
    parse_wsl_unc_path(&path.to_string_lossy())
}

fn parse_wsl_unc_path(path: &str) -> Result<WslPath> {
    let path = path.replace('/', "\\");
    let remainder = path
        .strip_prefix("\\\\wsl.localhost\\")
        .or_else(|| path.strip_prefix("\\\\wsl$\\"))
        .or_else(|| path.strip_prefix("\\\\?\\UNC\\wsl.localhost\\"))
        .or_else(|| path.strip_prefix("\\\\?\\UNC\\wsl$\\"))
        .with_context(|| format!("path is not a WSL UNC path: {path}"))?;

    let (distro, rest) = remainder
        .split_once('\\')
        .map(|(distro, rest)| (distro, Some(rest)))
        .unwrap_or((remainder, None));
    ensure!(
        !distro.is_empty(),
        "WSL UNC path is missing a distro name: {path}"
    );

    let linux_path = match rest {
        Some(rest) if !rest.is_empty() => format!("/{}", rest.replace('\\', "/")),
        _ => "/".to_string(),
    };

    Ok(WslPath {
        distro: distro.to_string(),
        path: linux_path,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_wsl_localhost_path() {
        let path = parse_wsl_unc_path(r"\\wsl.localhost\Ubuntu\home\me\project").unwrap();
        assert_eq!(path.distro, "Ubuntu");
        assert_eq!(path.path, "/home/me/project");
    }

    #[test]
    fn parse_wsl_dollar_path() {
        let path = parse_wsl_unc_path(r"\\wsl$\Debian\tmp").unwrap();
        assert_eq!(path.distro, "Debian");
        assert_eq!(path.path, "/tmp");
    }

    #[test]
    fn rejects_native_windows_path() {
        assert!(parse_wsl_unc_path(r"C:\Users\me\project").is_err());
    }

    #[test]
    fn bwrap_denies_network_by_default() {
        let args = build_bwrap_args(
            &[WslPath {
                distro: "Ubuntu".to_string(),
                path: "/home/me/project".to_string(),
            }],
            SandboxPermissions::default(),
            Some("/home/me/project"),
        );
        assert!(args.iter().any(|arg| arg == "--unshare-net"));
        assert!(
            args.windows(3)
                .any(|window| window == ["--bind", "/home/me/project", "/home/me/project"])
        );
    }

    #[test]
    fn bwrap_allows_network_when_requested() {
        let args = build_bwrap_args(
            &[],
            SandboxPermissions {
                allow_network: true,
                allow_fs_write: false,
            },
            None,
        );
        assert!(!args.iter().any(|arg| arg == "--unshare-net"));
    }
}
