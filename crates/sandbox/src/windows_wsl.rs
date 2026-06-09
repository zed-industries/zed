//! Windows sandbox integration via WSL.
//!
//! Sandboxed Windows terminal commands are routed through WSL and then executed
//! under Bubblewrap inside Linux. Projects may be opened either from native
//! Windows paths (`C:\Users\...`) or WSL UNC paths
//! (`\\wsl.localhost\Ubuntu\home\...`). Native drive-letter paths are mapped to
//! WSL's `/mnt/<drive>/...` view and use the user's default WSL distro unless a
//! WSL UNC path in the request pins a specific distro.

use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use anyhow::{Context as _, Result, bail, ensure};

use crate::SandboxPermissions;

const WSL_SANDBOX_ERROR_PREFIX: &str = "Windows sandboxing via WSL is unavailable";

#[derive(Clone, Debug, Eq, PartialEq)]
struct WslPath {
    distro: Option<String>,
    path: String,
}

/// Whether an error came from the Windows WSL sandbox setup path.
pub fn is_wsl_sandbox_error(error: &anyhow::Error) -> bool {
    error.to_string().contains(WSL_SANDBOX_ERROR_PREFIX)
}

/// Wrap a Linux process invocation so it runs under Bubblewrap inside WSL.
///
/// `program` and `args` must name a Linux executable and Linux argv, not a
/// Windows executable. The caller is expected to convert the model's command
/// into a Linux shell invocation (typically `/bin/sh -c ...`) before calling
/// this function.
///
/// All writable directories and the cwd must be directories that can be mapped
/// into WSL. WSL UNC paths may specify a distro; native drive-letter paths map
/// to `/mnt/<drive>/...` and use either that distro or the default distro.
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
                "{WSL_SANDBOX_ERROR_PREFIX}: failed to map terminal cwd `{}` into WSL",
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
                    "{WSL_SANDBOX_ERROR_PREFIX}: failed to map writable directory `{}` into WSL",
                    directory.display()
                )
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let distro = select_distro(cwd.as_ref(), &writable_directories)?;
    let wsl_exe = wsl_exe_path();
    ensure!(
        wsl_exe.is_file(),
        "{WSL_SANDBOX_ERROR_PREFIX}: WSL (`wsl.exe`) is not installed"
    );
    ensure!(
        wsl_command_succeeds(
            &wsl_exe,
            distro.as_deref(),
            &["--exec", "sh", "-lc", "true"]
        ),
        "{WSL_SANDBOX_ERROR_PREFIX}: WSL is installed, but no usable default distro was found"
    );
    ensure!(
        wsl_command_succeeds(
            &wsl_exe,
            distro.as_deref(),
            &["--exec", "sh", "-lc", "command -v bwrap >/dev/null"]
        ),
        "{WSL_SANDBOX_ERROR_PREFIX}: Bubblewrap (`bwrap`) is not installed in WSL"
    );

    let mut wsl_args = Vec::new();
    if let Some(distro) = distro.as_deref() {
        wsl_args.extend(["-d".to_string(), distro.to_string()]);
    }
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

fn select_distro(
    cwd: Option<&WslPath>,
    writable_directories: &[WslPath],
) -> Result<Option<String>> {
    let mut distro = cwd.and_then(|path| path.distro.clone());
    for path in writable_directories {
        let Some(path_distro) = path.distro.as_ref() else {
            continue;
        };
        match distro.as_deref() {
            Some(distro) => ensure!(
                distro == path_distro,
                "{WSL_SANDBOX_ERROR_PREFIX}: cannot mix WSL distros `{}` and `{}`",
                distro,
                path_distro
            ),
            None => distro = Some(path_distro.clone()),
        }
    }
    Ok(distro)
}

fn wsl_command_succeeds(wsl_exe: &Path, distro: Option<&str>, args: &[&str]) -> bool {
    let mut command = Command::new(wsl_exe);
    if let Some(distro) = distro {
        command.args(["-d", distro]);
    }
    command
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
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
    let path = path.to_string_lossy();
    parse_wsl_unc_path(&path).or_else(|_| parse_native_drive_path(&path))
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
        distro: Some(distro.to_string()),
        path: linux_path,
    })
}

fn parse_native_drive_path(path: &str) -> Result<WslPath> {
    let path = path
        .strip_prefix("\\\\?\\")
        .unwrap_or(path)
        .replace('\\', "/");
    let mut chars = path.chars();
    let Some(drive) = chars.next().filter(|drive| drive.is_ascii_alphabetic()) else {
        bail!("path is not a drive-letter Windows path: {path}");
    };
    ensure!(chars.next() == Some(':'), "path is not absolute: {path}");
    let rest = chars.as_str().trim_start_matches('/');
    let drive = drive.to_ascii_lowercase();
    let linux_path = if rest.is_empty() {
        format!("/mnt/{drive}")
    } else {
        format!("/mnt/{drive}/{rest}")
    };
    Ok(WslPath {
        distro: None,
        path: linux_path,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_wsl_localhost_path() {
        let path = parse_wsl_unc_path(r"\\wsl.localhost\Ubuntu\home\me\project").unwrap();
        assert_eq!(path.distro.as_deref(), Some("Ubuntu"));
        assert_eq!(path.path, "/home/me/project");
    }

    #[test]
    fn parse_wsl_dollar_path() {
        let path = parse_wsl_unc_path(r"\\wsl$\Debian\tmp").unwrap();
        assert_eq!(path.distro.as_deref(), Some("Debian"));
        assert_eq!(path.path, "/tmp");
    }

    #[test]
    fn parse_native_windows_path() {
        let path = parse_native_drive_path(r"C:\Users\me\project").unwrap();
        assert_eq!(path.distro, None);
        assert_eq!(path.path, "/mnt/c/Users/me/project");
    }

    #[test]
    fn parse_verbatim_native_windows_path() {
        let path = parse_native_drive_path(r"\\?\D:\workspace").unwrap();
        assert_eq!(path.distro, None);
        assert_eq!(path.path, "/mnt/d/workspace");
    }

    #[test]
    fn rejects_unc_non_wsl_path() {
        assert!(parse_native_drive_path(r"\\server\share\project").is_err());
    }

    #[test]
    fn bwrap_denies_network_by_default() {
        let args = build_bwrap_args(
            &[WslPath {
                distro: Some("Ubuntu".to_string()),
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

    #[test]
    fn select_distro_uses_wsl_distro_when_present() {
        let distro = select_distro(
            None,
            &[
                WslPath {
                    distro: None,
                    path: "/mnt/c/project".to_string(),
                },
                WslPath {
                    distro: Some("Ubuntu".to_string()),
                    path: "/home/me/project".to_string(),
                },
            ],
        )
        .unwrap();
        assert_eq!(distro.as_deref(), Some("Ubuntu"));
    }
}
