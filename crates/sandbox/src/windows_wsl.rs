//! Windows sandbox integration via WSL.
//!
//! Sandboxed Windows terminal commands are routed through WSL and then executed
//! under Bubblewrap inside Linux. Projects may be opened either from native
//! Windows paths (`C:\Users\...`) or WSL UNC paths
//! (`\\wsl.localhost\Ubuntu\home\...`). Native drive-letter paths are
//! translated into the distro's filesystem view with `wslpath` (falling back
//! to the conventional `/mnt/<drive>/...` mapping if that fails) and use the
//! user's default WSL distro unless a WSL UNC path in the request pins a
//! specific distro.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use smol::process::{Command, Stdio};

use anyhow::{Context as _, Result, bail, ensure};

use crate::SandboxPermissions;

const WSL_SANDBOX_ERROR_PREFIX: &str = "Windows sandboxing via WSL is unavailable";

#[derive(Clone, Debug, Eq, PartialEq)]
struct WslPath {
    distro: Option<String>,
    path: String,
}

/// A path mapped for use inside WSL.
///
/// WSL UNC and WSL-absolute paths can be mapped structurally up front. Native
/// drive-letter paths depend on the distro's automount configuration
/// (`/etc/wsl.conf` can move the `/mnt` root), so they are translated with
/// `wslpath` inside the distro — but a distro can only be chosen after every
/// path has been parsed (WSL UNC paths pin one), hence this two-stage shape:
/// parse structurally first, then resolve native paths via
/// [`resolve_path_mapping`] once the distro is known.
#[derive(Clone, Debug, Eq, PartialEq)]
enum PathMapping {
    Wsl(WslPath),
    NativeDrive {
        /// The `\\?\`-stripped, forward-slashed form that `wslpath -u`
        /// accepts (`wslpath` is a Linux binary and doesn't understand
        /// backslash separators).
        windows_path: String,
        /// The conventional `/mnt/<drive>/...` mapping, used when `wslpath`
        /// translation fails.
        fallback: WslPath,
    },
}

impl PathMapping {
    fn distro(&self) -> Option<&str> {
        match self {
            PathMapping::Wsl(path) => path.distro.as_deref(),
            PathMapping::NativeDrive { .. } => None,
        }
    }
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
/// All writable paths and the cwd must be paths that can be mapped into WSL.
/// WSL UNC paths may specify a distro; native drive-letter paths are
/// translated with `wslpath` inside either that distro or the default distro
/// (falling back to `/mnt/<drive>/...` if translation fails).
///
/// `env` is forwarded into the sandboxed command via `bwrap --setenv` rather
/// than being set on the `wsl.exe` process. Windows environment variables
/// don't cross the WSL boundary unless they're listed in `WSLENV`, so without
/// this the command would lose `PAGER` (used to stop `git` from paging into
/// the PTY) and the rest of the project environment. Variables whose Windows
/// values are meaningless or harmful inside Linux are dropped (see
/// [`is_forwardable_env_var`]).
pub fn wrap_invocation<S: std::hash::BuildHasher>(
    program: &str,
    args: &[String],
    writable_paths: &[&Path],
    permissions: SandboxPermissions,
    cwd: Option<&Path>,
    env: &HashMap<String, String, S>,
) -> Result<(String, Vec<String>)> {
    let cwd_mapping = match cwd {
        Some(cwd) => Some(directory_to_wsl(cwd).with_context(|| {
            format!(
                "{WSL_SANDBOX_ERROR_PREFIX}: failed to map terminal cwd `{}` into WSL",
                cwd.display()
            )
        })?),
        None => None,
    };

    let writable_mappings = writable_paths
        .iter()
        .map(|path| {
            path_to_wsl(path).with_context(|| {
                format!(
                    "{WSL_SANDBOX_ERROR_PREFIX}: failed to map writable path `{}` into WSL",
                    path.display()
                )
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let distro = select_distro(cwd_mapping.as_ref(), &writable_mappings)?;
    let wsl_exe = wsl_exe_path();
    ensure!(
        wsl_exe.is_file(),
        "{WSL_SANDBOX_ERROR_PREFIX}: WSL (`wsl.exe`) was not found at `{}`",
        wsl_exe.display()
    );
    check_wsl_command(
        &wsl_exe,
        distro.as_deref(),
        &["--exec", "sh", "-lc", "true"],
        "start a shell in WSL",
    )?;
    // Locate `bwrap` and, in the same round-trip, report whether the WSL
    // interop socket directory exists so we know whether masking it is
    // possible (see `build_bwrap_args`). `printf` on stdout is the success
    // signal; a missing `bwrap` exits non-zero and surfaces the
    // "not installed" error below.
    let probe = check_wsl_command_capture(
        &wsl_exe,
        distro.as_deref(),
        &[
            "--exec",
            "sh",
            "-lc",
            "command -v bwrap >/dev/null || exit 1; [ -d /run/WSL ] && printf interop",
        ],
        "find Bubblewrap (`bwrap`) in WSL",
    )
    .with_context(|| {
        format!(
            "{WSL_SANDBOX_ERROR_PREFIX}: Bubblewrap (`bwrap`) is not installed in {}",
            wsl_distro_label(distro.as_deref())
        )
    })?;
    let mask_interop_dir = probe.contains("interop");

    // Translate native drive-letter paths with `wslpath` now that the distro
    // is known and WSL is confirmed to work.
    let cwd = cwd_mapping.map(|mapping| resolve_path_mapping(&wsl_exe, distro.as_deref(), mapping));
    let writable_paths = writable_mappings
        .into_iter()
        .map(|mapping| resolve_path_mapping(&wsl_exe, distro.as_deref(), mapping))
        .collect::<Vec<_>>();

    if let Some(cwd) = &cwd {
        check_wsl_path_exists(&wsl_exe, distro.as_deref(), cwd, "terminal cwd")?;
    }
    for path in &writable_paths {
        check_wsl_path_exists(&wsl_exe, distro.as_deref(), path, "writable path")?;
    }

    let mut wsl_args = Vec::new();
    if let Some(distro) = distro.as_deref() {
        wsl_args.extend(["-d".to_string(), distro.to_string()]);
    }
    if let Some(cwd) = &cwd {
        wsl_args.extend(["--cd".to_string(), cwd.path.clone()]);
    }
    wsl_args.extend(["--exec".to_string(), "bwrap".to_string()]);
    wsl_args.extend(build_bwrap_args(
        &writable_paths,
        permissions,
        cwd.as_ref().map(|path| path.path.as_str()),
        mask_interop_dir,
        env,
    ));
    wsl_args.push("--".to_string());
    wsl_args.push(program.to_string());
    wsl_args.extend(args.iter().cloned());

    Ok((wsl_exe.to_string_lossy().into_owned(), wsl_args))
}

fn select_distro(
    cwd: Option<&PathMapping>,
    writable_paths: &[PathMapping],
) -> Result<Option<String>> {
    let mut distro = cwd.and_then(|mapping| mapping.distro().map(str::to_string));
    for mapping in writable_paths {
        let Some(path_distro) = mapping.distro() else {
            continue;
        };
        match distro.as_deref() {
            Some(distro) => ensure!(
                distro == path_distro,
                "{WSL_SANDBOX_ERROR_PREFIX}: cannot mix WSL distros `{}` and `{}`",
                distro,
                path_distro
            ),
            None => distro = Some(path_distro.to_string()),
        }
    }
    Ok(distro)
}

fn check_wsl_command(
    wsl_exe: &Path,
    distro: Option<&str>,
    args: &[&str],
    description: &str,
) -> Result<()> {
    let mut command = Command::new(wsl_exe);
    if let Some(distro) = distro {
        command.args(["-d", distro]);
    }
    command
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null());

    let output = smol::block_on(command.output()).with_context(|| {
        format!("{WSL_SANDBOX_ERROR_PREFIX}: failed to invoke WSL while trying to {description}")
    })?;
    ensure!(
        output.status.success(),
        "{WSL_SANDBOX_ERROR_PREFIX}: failed to {description} in {}{}",
        wsl_distro_label(distro),
        command_failure_details(output.status.code(), &output.stderr)
    );
    Ok(())
}

/// Like [`check_wsl_command`], but returns the command's stdout on success.
///
/// stdout is decoded as UTF-8 (lossily). This is only used for `--exec`'d
/// programs whose output we control, not for `wsl.exe`'s own diagnostics
/// (which are UTF-16LE).
fn check_wsl_command_capture(
    wsl_exe: &Path,
    distro: Option<&str>,
    args: &[&str],
    description: &str,
) -> Result<String> {
    let mut command = Command::new(wsl_exe);
    if let Some(distro) = distro {
        command.args(["-d", distro]);
    }
    command.args(args).stdin(Stdio::null());

    let output = smol::block_on(command.output()).with_context(|| {
        format!("{WSL_SANDBOX_ERROR_PREFIX}: failed to invoke WSL while trying to {description}")
    })?;
    ensure!(
        output.status.success(),
        "{WSL_SANDBOX_ERROR_PREFIX}: failed to {description} in {}{}",
        wsl_distro_label(distro),
        command_failure_details(output.status.code(), &output.stderr)
    );
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn check_wsl_path_exists(
    wsl_exe: &Path,
    distro: Option<&str>,
    path: &WslPath,
    description: &str,
) -> Result<()> {
    let mut command = Command::new(wsl_exe);
    if let Some(distro) = distro {
        command.args(["-d", distro]);
    }
    command
        .args(["--exec", "test", "-e", &path.path])
        .stdin(Stdio::null())
        .stdout(Stdio::null());

    let output = smol::block_on(command.output()).with_context(|| {
        format!(
            "{WSL_SANDBOX_ERROR_PREFIX}: failed to check {description} `{}` in {}",
            path.path,
            wsl_distro_label(distro)
        )
    })?;
    ensure!(
        output.status.success(),
        "{WSL_SANDBOX_ERROR_PREFIX}: mapped {description} `{}` does not exist in {}{}",
        path.path,
        wsl_distro_label(distro),
        command_failure_details(output.status.code(), &output.stderr)
    );
    Ok(())
}

fn command_failure_details(exit_code: Option<i32>, stderr: &[u8]) -> String {
    let stderr = String::from_utf8_lossy(stderr);
    let stderr = stderr.trim();
    let exit_status = match exit_code {
        Some(code) => format!("exit code {code}"),
        None => "terminated by signal".to_string(),
    };
    if stderr.is_empty() {
        format!(" ({exit_status})")
    } else {
        format!(" ({exit_status}; stderr: {stderr})")
    }
}

fn wsl_distro_label(distro: Option<&str>) -> String {
    match distro {
        Some(distro) => format!("WSL distro `{distro}`"),
        None => "the default WSL distro".to_string(),
    }
}

fn wsl_exe_path() -> PathBuf {
    std::env::var_os("SystemRoot")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(r"C:\Windows"))
        .join("System32")
        .join("wsl.exe")
}

fn build_bwrap_args<S: std::hash::BuildHasher>(
    writable_paths: &[WslPath],
    permissions: SandboxPermissions,
    cwd: Option<&str>,
    mask_interop_dir: bool,
    env: &HashMap<String, String, S>,
) -> Vec<String> {
    let mut args = Vec::new();

    if permissions.allow_fs_write {
        push_bind(&mut args, "--bind", "/", "/");
    } else {
        push_bind(&mut args, "--ro-bind", "/", "/");
        args.extend(["--tmpfs".to_string(), "/tmp".to_string()]);
        for path in writable_paths {
            push_bind(&mut args, "--bind", &path.path, &path.path);
        }
    }

    // Block WSL's Windows interop, regardless of the requested permissions.
    // Without this, a sandboxed process can exec a Windows binary (e.g.
    // /mnt/c/Windows/System32/cmd.exe), which the kernel's binfmt handler
    // (`/init`) hands off to the Windows host over an AF_UNIX socket — running
    // fully outside bwrap and defeating both the filesystem and the network
    // restrictions. `/init` locates that socket via the $WSL_INTEROP
    // environment variable, so we drop it; and we mask the socket directory
    // (when it exists) so the value can't be rediscovered by listing
    // /run/WSL and re-exporting it. Both steps are required: unsetting the
    // variable alone is bypassable, and masking alone leaves the inherited
    // variable usable.
    args.extend(["--unsetenv".to_string(), "WSL_INTEROP".to_string()]);
    if mask_interop_dir {
        args.extend(["--tmpfs".to_string(), "/run/WSL".to_string()]);
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

    // Forward the caller-provided environment into the command. Windows env
    // set on the `wsl.exe` process doesn't reach the Linux command, so we
    // re-apply it here on the sandbox's child instead.
    for (name, value) in env {
        if is_forwardable_env_var(name) {
            args.extend(["--setenv".to_string(), name.clone(), value.clone()]);
        }
    }

    if let Some(cwd) = cwd {
        args.extend(["--chdir".to_string(), cwd.to_string()]);
    }

    args
}

/// Whether an environment variable should be forwarded into the Linux sandbox.
///
/// `bwrap --setenv` calls `setenv(3)`, which rejects names that are empty or
/// contain `=`. Windows process environments include such entries — most
/// notably the per-drive current-directory pseudo-variables (`=C:`, `=D:`,
/// ...) Windows keeps in the environment block — so they must be skipped or
/// bwrap aborts with "setenv failed".
///
/// Beyond that, a few variables hold Windows-specific values that would be
/// meaningless or actively break the command inside WSL: `PATH` would shadow
/// WSL's own `PATH` and stop the shell from finding Linux executables, and the
/// temp-dir variables point at Windows paths that don't exist in WSL (bwrap
/// provides a fresh tmpfs `/tmp` instead). Matched case-insensitively because
/// Windows environment variable names are.
fn is_forwardable_env_var(name: &str) -> bool {
    if name.is_empty() || name.contains('=') {
        return false;
    }
    const BLOCKED: [&str; 4] = ["PATH", "TMPDIR", "TMP", "TEMP"];
    !BLOCKED
        .iter()
        .any(|blocked| name.eq_ignore_ascii_case(blocked))
}

fn push_bind(args: &mut Vec<String>, flag: &str, source: &str, destination: &str) {
    args.extend([
        flag.to_string(),
        source.to_string(),
        destination.to_string(),
    ]);
}

/// Resolve a [`PathMapping`] into the final WSL path, translating native
/// drive-letter paths with `wslpath -u` inside the chosen distro so the
/// distro's actual automount configuration is honored. Falls back to the
/// structural `/mnt/<drive>/...` mapping when translation fails (e.g. a
/// distro without `wslpath`); a wrong fallback is still caught by the
/// existence check that follows.
fn resolve_path_mapping(wsl_exe: &Path, distro: Option<&str>, mapping: PathMapping) -> WslPath {
    match mapping {
        PathMapping::Wsl(path) => path,
        PathMapping::NativeDrive {
            windows_path,
            fallback,
        } => match translate_windows_path(wsl_exe, distro, &windows_path) {
            Ok(path) => WslPath { distro: None, path },
            Err(error) => {
                log::warn!(
                    "failed to translate `{windows_path}` with wslpath in {}; \
                     falling back to `{}`: {error:#}",
                    wsl_distro_label(distro),
                    fallback.path
                );
                fallback
            }
        },
    }
}

/// Translate a forward-slashed Windows path into the distro's view with
/// `wslpath -u` (the same technique as
/// `remote::transport::wsl::WslConnectionOptions::abs_windows_path_to_wsl_path`).
fn translate_windows_path(
    wsl_exe: &Path,
    distro: Option<&str>,
    windows_path: &str,
) -> Result<String> {
    let output = check_wsl_command_capture(
        wsl_exe,
        distro,
        &["--exec", "wslpath", "-u", windows_path],
        "translate a Windows path with `wslpath`",
    )?;
    let path = output.trim_end_matches(['\r', '\n']);
    ensure!(path.starts_with('/'), "unexpected wslpath output: {path:?}");
    Ok(path.to_string())
}

fn directory_to_wsl(path: &Path) -> Result<PathMapping> {
    ensure!(
        path.is_dir(),
        "Windows sandboxing via WSL can only use an existing directory as cwd: {}",
        path.display()
    );
    map_path_to_wsl(path)
}

fn path_to_wsl(path: &Path) -> Result<PathMapping> {
    let path_string = path.to_string_lossy();
    if let Ok(path) = parse_wsl_absolute_path(&path_string) {
        return Ok(PathMapping::Wsl(path));
    }

    ensure!(
        path.is_dir() || path.is_file(),
        "Windows sandboxing via WSL can only grant existing files or directories: {}",
        path.display()
    );
    map_path_to_wsl(path)
}

fn map_path_to_wsl(path: &Path) -> Result<PathMapping> {
    let path_string = path.to_string_lossy();
    if let Ok(path) = parse_wsl_unc_path(&path_string) {
        return Ok(PathMapping::Wsl(path));
    }
    let fallback = parse_native_drive_path(&path_string)?;
    let windows_path = path_string
        .strip_prefix(r"\\?\")
        .unwrap_or(&path_string)
        .replace('\\', "/");
    Ok(PathMapping::NativeDrive {
        windows_path,
        fallback,
    })
}

fn parse_wsl_absolute_path(path: &str) -> Result<WslPath> {
    let path = path.replace('\\', "/");
    ensure!(
        path.starts_with('/') && !path.starts_with("//"),
        "path is not a WSL absolute path: {path}"
    );
    Ok(WslPath { distro: None, path })
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
    fn parse_wsl_absolute_path_keeps_linux_path() {
        let path = parse_wsl_absolute_path("/home/me").unwrap();
        assert_eq!(path.distro, None);
        assert_eq!(path.path, "/home/me");
    }

    #[test]
    fn parse_wsl_absolute_path_rejects_unc_paths() {
        assert!(parse_wsl_absolute_path(r"\\server\share").is_err());
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
            true,
            &HashMap::new(),
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
            true,
            &HashMap::new(),
        );
        assert!(!args.iter().any(|arg| arg == "--unshare-net"));
    }

    #[test]
    fn bwrap_binds_explicit_writable_file_paths() {
        let args = build_bwrap_args(
            &[WslPath {
                distro: None,
                path: "/mnt/c/Users/me/AppData/Roaming/Zed/AGENTS.md".to_string(),
            }],
            SandboxPermissions::default(),
            None,
            true,
            &HashMap::new(),
        );
        assert!(args.windows(3).any(|window| window
            == [
                "--bind",
                "/mnt/c/Users/me/AppData/Roaming/Zed/AGENTS.md",
                "/mnt/c/Users/me/AppData/Roaming/Zed/AGENTS.md"
            ]));
    }

    #[test]
    fn bwrap_blocks_wsl_interop_by_default() {
        let args = build_bwrap_args(
            &[WslPath {
                distro: Some("Ubuntu".to_string()),
                path: "/home/me/project".to_string(),
            }],
            SandboxPermissions::default(),
            Some("/home/me/project"),
            true,
            &HashMap::new(),
        );
        assert!(
            args.windows(2)
                .any(|window| window == ["--unsetenv", "WSL_INTEROP"])
        );
        assert!(
            args.windows(2)
                .any(|window| window == ["--tmpfs", "/run/WSL"])
        );
    }

    #[test]
    fn bwrap_blocks_wsl_interop_even_with_fs_write() {
        let args = build_bwrap_args(
            &[],
            SandboxPermissions {
                allow_network: true,
                allow_fs_write: true,
            },
            None,
            true,
            &HashMap::new(),
        );
        // Interop is host code execution, not just a filesystem write, so it
        // stays blocked even when the user has granted unrestricted writes
        // and network.
        assert!(
            args.windows(2)
                .any(|window| window == ["--unsetenv", "WSL_INTEROP"])
        );
        assert!(
            args.windows(2)
                .any(|window| window == ["--tmpfs", "/run/WSL"])
        );
    }

    #[test]
    fn bwrap_skips_interop_dir_mask_when_absent() {
        // When the interop socket directory doesn't exist (interop disabled),
        // there's nothing to mask and a `--tmpfs /run/WSL` would abort bwrap,
        // so the mount must be omitted. Unsetting the variable is harmless and
        // stays.
        let args = build_bwrap_args(
            &[],
            SandboxPermissions::default(),
            None,
            false,
            &HashMap::new(),
        );
        assert!(
            args.windows(2)
                .any(|window| window == ["--unsetenv", "WSL_INTEROP"])
        );
        assert!(!args.iter().any(|arg| arg == "/run/WSL"));
    }

    #[test]
    fn bwrap_forwards_env_via_setenv() {
        let env = HashMap::from([
            ("PAGER".to_string(), String::new()),
            ("CARGO_TERM_COLOR".to_string(), "always".to_string()),
        ]);
        let args = build_bwrap_args(&[], SandboxPermissions::default(), None, false, &env);
        assert!(
            args.windows(3)
                .any(|window| window == ["--setenv", "PAGER", ""])
        );
        assert!(
            args.windows(3)
                .any(|window| window == ["--setenv", "CARGO_TERM_COLOR", "always"])
        );
    }

    #[test]
    fn bwrap_does_not_forward_windows_specific_env() {
        // These hold Windows paths/values that would break or be meaningless
        // inside WSL, so they must never cross the boundary. Names are matched
        // case-insensitively, as Windows env var names are.
        let env = HashMap::from([
            ("Path".to_string(), r"C:\Windows\System32".to_string()),
            (
                "TEMP".to_string(),
                r"C:\Users\me\AppData\Local\Temp".to_string(),
            ),
            (
                "Tmp".to_string(),
                r"C:\Users\me\AppData\Local\Temp".to_string(),
            ),
            ("TMPDIR".to_string(), r"C:\tmp".to_string()),
        ]);
        let args = build_bwrap_args(&[], SandboxPermissions::default(), None, false, &env);
        assert!(!args.iter().any(|arg| arg == "--setenv"));
    }

    #[test]
    fn bwrap_skips_env_names_setenv_would_reject() {
        // bwrap's `--setenv` calls `setenv(3)`, which rejects empty names and
        // names containing `=`. Windows environments include the per-drive
        // current-directory pseudo-variables (`=C:`, ...); forwarding them
        // would abort bwrap with "setenv failed".
        let env = HashMap::from([
            ("=C:".to_string(), r"C:\Users\me".to_string()),
            (String::new(), "value".to_string()),
            ("OK".to_string(), "value".to_string()),
        ]);
        let args = build_bwrap_args(&[], SandboxPermissions::default(), None, false, &env);
        assert!(
            args.windows(3)
                .any(|window| window == ["--setenv", "OK", "value"])
        );
        assert_eq!(args.iter().filter(|arg| *arg == "--setenv").count(), 1);
    }

    #[test]
    fn select_distro_uses_wsl_distro_when_present() {
        let distro = select_distro(
            None,
            &[
                PathMapping::NativeDrive {
                    windows_path: "C:/project".to_string(),
                    fallback: WslPath {
                        distro: None,
                        path: "/mnt/c/project".to_string(),
                    },
                },
                PathMapping::Wsl(WslPath {
                    distro: Some("Ubuntu".to_string()),
                    path: "/home/me/project".to_string(),
                }),
            ],
        )
        .unwrap();
        assert_eq!(distro.as_deref(), Some("Ubuntu"));
    }

    #[test]
    fn map_path_to_wsl_keeps_unc_paths_structural() {
        let mapping = map_path_to_wsl(Path::new(r"\\wsl.localhost\Ubuntu\home\me")).unwrap();
        assert_eq!(
            mapping,
            PathMapping::Wsl(WslPath {
                distro: Some("Ubuntu".to_string()),
                path: "/home/me".to_string(),
            })
        );
    }

    #[test]
    fn map_path_to_wsl_defers_native_paths_to_wslpath() {
        let mapping = map_path_to_wsl(Path::new(r"C:\Users\me\project")).unwrap();
        assert_eq!(
            mapping,
            PathMapping::NativeDrive {
                windows_path: "C:/Users/me/project".to_string(),
                fallback: WslPath {
                    distro: None,
                    path: "/mnt/c/Users/me/project".to_string(),
                },
            }
        );
    }

    #[test]
    fn map_path_to_wsl_strips_verbatim_prefix_for_wslpath() {
        let mapping = map_path_to_wsl(Path::new(r"\\?\D:\workspace")).unwrap();
        assert_eq!(
            mapping,
            PathMapping::NativeDrive {
                windows_path: "D:/workspace".to_string(),
                fallback: WslPath {
                    distro: None,
                    path: "/mnt/d/workspace".to_string(),
                },
            }
        );
    }
}
