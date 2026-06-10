//! macOS Seatbelt sandbox integration.
//!
//! This module is specifically about Apple's Seatbelt sandbox API — the
//! macOS-only kernel-level sandboxing framework, accessed via the
//! `sandbox-exec(1)` command-line tool and a Seatbelt-specific config
//! file (a Scheme-like policy language documented in Apple's
//! `sandbox.h` and the `sandbox-exec` man page).
//!
//! The integration wraps a shell invocation by:
//!
//! 1. Generating a Seatbelt config file (a string of Scheme-like rules)
//!    from the requested [`SandboxPermissions`].
//! 2. Writing it to a temporary file on disk (a [`SeatbeltConfigFile`],
//!    which cleans itself up when dropped).
//! 3. Returning the program/args needed to launch the original command
//!    under `sandbox-exec -f <config-path>`.
//!
//! Reads are permitted by default; writes are restricted to a caller-
//! provided list of directories; IP network access and unrestricted writes
//! must be opted into per command. Callers may separately allow specific
//! Unix domain sockets for local IPC; those do not permit sending packets to
//! other machines.

use std::path::Path;
use std::{io::Write, path::PathBuf};

use anyhow::{Context, Result};
use tempfile::NamedTempFile;

/// Per-command relaxations of the default Seatbelt sandbox.
///
/// All-false is the default, fully-sandboxed run. Setting any field
/// requires user approval before the command is launched.
///
/// There are some baseline OS operations (e.g. arbitrary hardware access)
/// that are disallowed by Seatbelt's baseline policy regardless of these
/// flags; even with everything `true` here those operations stay denied.
/// The only way to allow them is to skip the sandbox entirely (which this
/// module deliberately doesn't expose).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SandboxPermissions {
    /// Allow IP network access for the command.
    pub allow_network: bool,
    /// Allow unrestricted filesystem writes.
    pub allow_fs_write: bool,
}

/// A Seatbelt config file written to a temporary path on disk, suitable
/// for `sandbox-exec -f <path>`. The file is deleted when this is dropped.
///
/// The config-file content is the Scheme-like Seatbelt policy language
/// (see `sandbox-exec(1)` and the comments in macOS's `sandbox.h`); it's
/// generated from a [`SandboxPermissions`] by [`generate_seatbelt_config`].
pub struct SeatbeltConfigFile {
    /// The temporary file containing the Seatbelt config.
    /// Kept alive so the file exists for the duration of the command.
    _file: NamedTempFile,
    /// Path to the temporary config file on disk.
    path: PathBuf,
}

impl SeatbeltConfigFile {
    /// Generate a Seatbelt config from `permissions` and write it to a
    /// fresh temporary file.
    ///
    /// `writable_directories` lists every directory subtree where the
    /// command is allowed to write when `permissions.allow_fs_write` is
    /// false. Pass the project's worktree paths here — not the working
    /// directory of the command, since that is model-controlled and would
    /// let the model widen its own writable scope.
    ///
    /// `protected_paths` lists paths whose file data reads and writes should
    /// be blocked even if they fall under a readable or writable directory.
    /// File metadata remains readable.
    ///
    /// `allowed_unix_socket_paths` lists Unix domain sockets the command may
    /// connect to for local IPC even when IP network access is otherwise
    /// disabled. This does not permit sending packets to other machines.
    pub fn new(
        writable_directories: &[&Path],
        protected_paths: &[&Path],
        allowed_unix_socket_paths: &[&Path],
        permissions: SandboxPermissions,
    ) -> Result<Self> {
        let mut file =
            NamedTempFile::new().context("failed to create temporary Seatbelt config file")?;

        let config = generate_seatbelt_config(
            writable_directories,
            protected_paths,
            allowed_unix_socket_paths,
            permissions,
        )?;
        file.write_all(config.as_bytes())
            .context("failed to write Seatbelt config")?;
        file.flush().context("failed to flush Seatbelt config")?;

        let path = file.path().to_path_buf();

        Ok(Self { _file: file, path })
    }
}

/// Wrap a process invocation so it runs under macOS's `sandbox-exec(1)`
/// with a Seatbelt config built from `permissions`.
///
/// Returns the new program and arguments to execute, along with a
/// [`SeatbeltConfigFile`] that **must** be kept alive for the duration of
/// the command (the file is deleted when dropped, and `sandbox-exec` reads
/// it lazily when the child process starts up).
///
/// # Arguments
/// * `program` - The program to invoke (typically a shell, e.g. `"/bin/sh"`,
///   but anything that takes its arguments via `argv` works).
/// * `args` - The full argument list that would have been passed to
///   `program`.
/// * `writable_directories` - Directory subtrees where the command is
///   allowed to write when `permissions.allow_fs_write` is false. Pass
///   the project's worktree paths here, not the working directory of the
///   command (the working directory is model-controlled, and using it as
///   the writable scope would let the model write outside the project).
/// * `protected_paths` - Paths whose file data reads and writes should be
///   denied even if they fall under a readable or writable directory. File
///   metadata remains readable.
/// * `allowed_unix_socket_paths` - Unix domain sockets the command may
///   connect to for local IPC even when IP network access is otherwise
///   disabled. This does not permit sending packets to other machines.
/// * `permissions` - Sandbox relaxations requested for this command.
///
/// # Returns
/// A tuple of `(program, args, config_file)` where `config_file` must be
/// kept alive.
pub fn wrap_invocation(
    program: &str,
    args: &[String],
    writable_directories: &[&Path],
    protected_paths: &[&Path],
    allowed_unix_socket_paths: &[&Path],
    permissions: SandboxPermissions,
) -> Result<(String, Vec<String>, SeatbeltConfigFile)> {
    let config_file = SeatbeltConfigFile::new(
        writable_directories,
        protected_paths,
        allowed_unix_socket_paths,
        permissions,
    )?;

    let mut wrapped_args = vec![
        "-f".to_string(),
        config_file
            .path
            .to_str()
            .with_context(|| {
                format!(
                    "Seatbelt config file path contains invalid UTF-8: {}",
                    config_file.path.display()
                )
            })?
            .to_string(),
        program.to_string(),
    ];
    wrapped_args.extend(args.iter().cloned());

    Ok((
        "/usr/bin/sandbox-exec".to_string(),
        wrapped_args,
        config_file,
    ))
}

/// Generate a Seatbelt config string that reads everywhere by default.
/// Writes to each entry in `writable_directories` (typically the project's
/// worktree paths plus any per-command scratch directory the caller wants
/// allowed) and the standard `/dev/*` write targets are also allowed by
/// default. File data reads and writes to paths in `protected_paths` are
/// denied even when they would otherwise be readable or writable; file
/// metadata remains readable. Unix domain socket paths in
/// `allowed_unix_socket_paths` are reachable for local IPC even when IP
/// network access is otherwise blocked; callers use this for trusted sockets
/// inherited from the process
/// environment, such as `SSH_AUTH_SOCK`. This does not permit sending
/// packets to other machines.
///
/// Network access and unrestricted filesystem writes must be requested via
/// [`SandboxPermissions`].
///
/// The returned string is the textual content to write to the
/// [`SeatbeltConfigFile`] passed to `sandbox-exec -f`.
fn generate_seatbelt_config(
    writable_directories: &[&Path],
    protected_paths: &[&Path],
    allowed_unix_socket_paths: &[&Path],
    permissions: SandboxPermissions,
) -> Result<String> {
    // Canonicalize each writable path to resolve symlinks (e.g.,
    // /var -> /private/var on macOS). Fall back to the original path if
    // canonicalization fails.
    let canonical_writable_directories: Vec<PathBuf> = writable_directories
        .iter()
        .map(|path| path.canonicalize().unwrap_or_else(|_| path.to_path_buf()))
        .collect();
    let canonical_protected_paths: Vec<PathBuf> = protected_paths
        .iter()
        .map(|path| path.canonicalize().unwrap_or_else(|_| path.to_path_buf()))
        .collect();
    let canonical_unix_socket_paths: Vec<PathBuf> = allowed_unix_socket_paths
        .iter()
        .map(|path| path.canonicalize().unwrap_or_else(|_| path.to_path_buf()))
        .collect();

    let mut config = r#"(version 1)

; Start by denying everything
(deny default)

; Allow reading from the entire filesystem
(allow file-read*)

; Allow process execution
(allow process-exec*)
(allow process-fork)

; Allow signal handling
(allow signal)

; Allow sysctl reads (needed for many system calls)
(allow sysctl-read)

; Allow mach lookups (needed for IPC)
(allow mach-lookup)

; Allow pseudo-terminal operations
(allow pseudo-tty)
(allow file-read* file-write* file-ioctl
    (literal "/dev/ptmx"))
(allow file-read* file-write*
    (require-all
        (regex #"^/dev/ttys[0-9]+")
        (extension "com.apple.sandbox.pty")))

; PTYs created before entering Seatbelt may lack the extension. Allow ioctls
; on those slave TTYs so interactive shells and signing prompts can manipulate
; terminal state.
(allow file-ioctl
    (regex #"^/dev/ttys[0-9]+"))
"#
    .to_string();

    if permissions.allow_fs_write {
        config.push_str(
            r#"
; Allow unrestricted filesystem writes
(allow file-write*)
"#,
        );
    } else {
        for canonical_path in &canonical_writable_directories {
            let escaped_path = escape_sandbox_path(canonical_path)?;
            config.push_str(&format!(
                r#"
; Allow writing to a permitted directory
(allow file-write*
    (subpath "{escaped_path}"))
"#
            ));
        }

        config.push_str(
            r#"
; Allow writing to common /dev paths (needed for redirections like 2>/dev/null)
(allow file-write*
    (literal "/dev/null")
    (literal "/dev/zero")
    (literal "/dev/tty")
    (literal "/dev/stdin")
    (literal "/dev/stdout")
    (literal "/dev/stderr")
    (subpath "/dev/fd"))
"#,
        );
    }

    for protected_path in &canonical_protected_paths {
        let escaped_path = escape_sandbox_path(protected_path)?;
        config.push_str(&format!(
            r#"
; Block Git metadata content access unless Git access is approved
(deny file-read-data file-write*
    (literal "{escaped_path}")
    (subpath "{escaped_path}"))
"#
        ));
    }

    if permissions.allow_network {
        config.push_str(
            r#"
; Allow network access
(allow network*)
"#,
        );
    }

    if !canonical_unix_socket_paths.is_empty() {
        config.push_str(
            r#"
; Allow local IPC to inherited Unix domain sockets. Seatbelt models this as
; network-outbound, but this does not permit IP networking or sending packets
; to other machines.
(allow system-socket
    (socket-domain AF_UNIX))
"#,
        );

        for socket_path in &canonical_unix_socket_paths {
            let escaped_path = escape_sandbox_path(socket_path)?;
            config.push_str(&format!(
                r#"(allow network-outbound
    (remote unix-socket
        (literal "{escaped_path}")))
"#
            ));
        }
    }

    Ok(config)
}

/// Escape a path for use in a Seatbelt config string.
///
/// Seatbelt configs use a Scheme-like syntax where certain characters need
/// to be handled carefully.
fn escape_sandbox_path(path: &Path) -> Result<String> {
    let path_str = path
        .to_str()
        .with_context(|| format!("path contains invalid UTF-8: {}", path.display()))?;
    Ok(path_str.replace('\\', "\\\\").replace('"', "\\\""))
}

#[cfg(test)]
#[allow(
    clippy::disallowed_methods,
    reason = "tests run sandbox-exec synchronously to verify the generated Seatbelt config"
)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_generate_seatbelt_config_contains_read_and_project_write_permissions_by_default() {
        let dir = PathBuf::from("/Users/test/projects/myproject");
        let config =
            generate_seatbelt_config(&[dir.as_path()], &[], &[], SandboxPermissions::default())
                .unwrap();

        assert!(config.contains("(allow file-read*)"));
        assert!(config.contains("/Users/test/projects/myproject"));
        assert!(config.contains("(allow file-write*"));
        assert!(!config.contains("; Allow unrestricted filesystem writes"));
        assert!(!config.contains("(allow network*)"));
    }

    #[test]
    fn test_generate_seatbelt_config_allows_unrestricted_writes_when_fs_writes_allowed() {
        let dir = PathBuf::from("/Users/test/projects/myproject");
        let config = generate_seatbelt_config(
            &[dir.as_path()],
            &[],
            &[],
            SandboxPermissions {
                allow_network: false,
                allow_fs_write: true,
            },
        )
        .unwrap();

        assert!(config.contains("(allow file-read*)"));
        assert!(config.contains("; Allow unrestricted filesystem writes"));
        assert!(config.contains("(allow file-write*)"));
        assert!(!config.contains("/Users/test/projects/myproject"));
        assert!(!config.contains("(allow network*)"));
    }

    #[test]
    fn test_generate_seatbelt_config_allows_terminal_ioctls_by_default() {
        let dir = PathBuf::from("/Users/test/projects/myproject");
        let config =
            generate_seatbelt_config(&[dir.as_path()], &[], &[], SandboxPermissions::default())
                .unwrap();

        assert!(config.contains("(allow file-ioctl"));
        assert!(config.contains("/dev/ptmx"));
        assert!(config.contains("^/dev/ttys[0-9]+"));
    }

    #[test]
    fn test_generate_seatbelt_config_allows_unix_socket_paths_without_network() {
        let dir = PathBuf::from("/Users/test/projects/myproject");
        let ssh_auth_socket = PathBuf::from("/private/tmp/com.apple.launchd.test/Listeners");
        let config = generate_seatbelt_config(
            &[dir.as_path()],
            &[],
            &[ssh_auth_socket.as_path()],
            SandboxPermissions::default(),
        )
        .unwrap();

        assert!(config.contains("(allow system-socket"));
        assert!(config.contains("AF_UNIX"));
        assert!(config.contains("(allow network-outbound"));
        assert!(config.contains("remote unix-socket"));
        assert!(config.contains("(literal \"/private/tmp/com.apple.launchd.test/Listeners\")"));
        assert!(!config.contains("(subpath \"/private/tmp/com.apple.launchd.test/Listeners\")"));
        assert!(!config.contains("(allow network*)"));
    }

    #[test]
    fn test_generate_seatbelt_config_denies_protected_path_data_and_writes() {
        let dir = PathBuf::from("/Users/test/projects/myproject");
        let protected = dir.join(".gitignore");
        let config = generate_seatbelt_config(
            &[dir.as_path()],
            &[protected.as_path()],
            &[],
            SandboxPermissions::default(),
        )
        .unwrap();

        assert!(config.contains("(deny file-read-data file-write*"));
        assert!(config.contains("/Users/test/projects/myproject/.gitignore"));
        assert!(config.contains("(literal \"/Users/test/projects/myproject/.gitignore\")"));
        assert!(config.contains("(subpath \"/Users/test/projects/myproject/.gitignore\")"));
    }

    #[test]
    fn test_sandbox_blocks_protected_path_contents_but_allows_metadata() {
        use std::process::Command;

        let temp_dir = tempfile::tempdir().unwrap();
        let protected_file = temp_dir.path().join(".gitignore");
        std::fs::write(&protected_file, "target\n").unwrap();

        let (program, args, _config_file) = wrap_invocation(
            "/bin/sh",
            &[
                "-c".to_string(),
                format!(
                    "test -e '{}' && ! cat '{}' >/dev/null 2>&1 && ! sh -c 'echo changed > {}'",
                    protected_file.display(),
                    protected_file.display(),
                    protected_file.display(),
                ),
            ],
            &[temp_dir.path()],
            &[protected_file.as_path()],
            &[],
            SandboxPermissions::default(),
        )
        .unwrap();

        let output = Command::new(&program)
            .args(&args)
            .output()
            .expect("failed to execute sandbox-exec");

        assert!(
            output.status.success(),
            "sandbox should allow metadata but deny protected data reads and writes: stderr={} stdout={}",
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout),
        );
        assert_eq!(
            std::fs::read_to_string(&protected_file).unwrap(),
            "target\n"
        );
    }

    #[test]
    fn test_sandbox_blocks_protected_paths_even_when_fs_writes_allowed() {
        use std::process::Command;

        let temp_dir = tempfile::tempdir().unwrap();
        let protected_file = temp_dir.path().join(".gitignore");
        std::fs::write(&protected_file, "target\n").unwrap();

        let (program, args, _config_file) = wrap_invocation(
            "/bin/sh",
            &[
                "-c".to_string(),
                format!(
                    "! cat '{}' >/dev/null 2>&1 && ! sh -c 'echo changed > {}'",
                    protected_file.display(),
                    protected_file.display(),
                ),
            ],
            &[temp_dir.path()],
            &[protected_file.as_path()],
            &[],
            SandboxPermissions {
                allow_network: false,
                allow_fs_write: true,
            },
        )
        .unwrap();

        let output = Command::new(&program)
            .args(&args)
            .output()
            .expect("failed to execute sandbox-exec");

        assert!(
            output.status.success(),
            "protected paths should stay blocked even with unrestricted writes: stderr={} stdout={}",
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout),
        );
        assert_eq!(
            std::fs::read_to_string(&protected_file).unwrap(),
            "target\n"
        );
    }

    #[test]
    fn test_generate_seatbelt_config_contains_network_when_allowed() {
        let dir = PathBuf::from("/Users/test/projects/myproject");
        let config = generate_seatbelt_config(
            &[dir.as_path()],
            &[],
            &[],
            SandboxPermissions {
                allow_network: true,
                allow_fs_write: false,
            },
        )
        .unwrap();

        assert!(config.contains("(allow network*)"));
        assert!(config.contains("/Users/test/projects/myproject"));
        assert!(config.contains("(allow file-write*"));
        assert!(!config.contains("; Allow unrestricted filesystem writes"));
    }

    #[test]
    fn test_generate_seatbelt_config_emits_one_subpath_per_writable_directory() {
        let project_dir = PathBuf::from("/Users/test/projects/myproject");
        let scratch_dir = PathBuf::from("/private/tmp/zed-agent-command");
        let config = generate_seatbelt_config(
            &[project_dir.as_path(), scratch_dir.as_path()],
            &[],
            &[],
            SandboxPermissions::default(),
        )
        .unwrap();

        assert!(config.contains("/Users/test/projects/myproject"));
        assert!(config.contains("/private/tmp/zed-agent-command"));
        assert!(!config.contains("; Allow unrestricted filesystem writes"));
        assert!(!config.contains("(allow network*)"));
    }

    #[test]
    fn test_escape_sandbox_path_handles_special_chars() {
        let path = PathBuf::from("/path/with\"quotes");
        let escaped = escape_sandbox_path(&path).unwrap();
        assert_eq!(escaped, "/path/with\\\"quotes");
    }

    #[cfg(unix)]
    #[test]
    fn test_escape_sandbox_path_rejects_invalid_utf8() {
        use std::{ffi::OsString, os::unix::ffi::OsStringExt};

        let path = PathBuf::from(OsString::from_vec(b"/path/with/invalid/\xFF".to_vec()));
        let error = escape_sandbox_path(&path).unwrap_err();

        assert!(error.to_string().contains("invalid UTF-8"));
    }

    #[test]
    fn test_wrap_invocation_structure() {
        let temp_dir = tempfile::tempdir().unwrap();
        let (program, args, _config_file) = wrap_invocation(
            "/bin/sh",
            &["-c".to_string(), "echo hello".to_string()],
            &[temp_dir.path()],
            &[],
            &[],
            SandboxPermissions::default(),
        )
        .unwrap();

        assert_eq!(program, "/usr/bin/sandbox-exec");
        assert_eq!(args[0], "-f");
        // args[1] is the temp file path
        assert_eq!(args[2], "/bin/sh");
        assert_eq!(args[3], "-c");
        assert_eq!(args[4], "echo hello");
    }

    #[test]
    fn test_sandbox_allows_read_everywhere() {
        use std::process::Command;

        let temp_dir = tempfile::tempdir().unwrap();
        let (program, args, _config_file) = wrap_invocation(
            "/bin/sh",
            &["-c".to_string(), "cat /etc/hosts".to_string()],
            &[temp_dir.path()],
            &[],
            &[],
            SandboxPermissions::default(),
        )
        .unwrap();

        let output = Command::new(&program)
            .args(&args)
            .output()
            .expect("failed to execute sandbox-exec");

        assert!(
            output.status.success(),
            "sandbox should allow reading /etc/hosts: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[test]
    fn test_sandbox_allows_dev_null_redirection_by_default() {
        use std::process::Command;

        let temp_dir = tempfile::tempdir().unwrap();
        let (program, args, _config_file) = wrap_invocation(
            "/bin/sh",
            &["-c".to_string(), "echo test 2>/dev/null".to_string()],
            &[temp_dir.path()],
            &[],
            &[],
            SandboxPermissions::default(),
        )
        .unwrap();

        let output = Command::new(&program)
            .args(&args)
            .output()
            .expect("failed to execute sandbox-exec");

        assert!(
            output.status.success(),
            "sandbox should allow redirecting to /dev/null by default: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[test]
    fn test_sandbox_allows_dev_null_redirection_when_fs_writes_allowed() {
        use std::process::Command;

        let temp_dir = tempfile::tempdir().unwrap();
        let (program, args, _config_file) = wrap_invocation(
            "/bin/sh",
            &["-c".to_string(), "echo test 2>/dev/null".to_string()],
            &[temp_dir.path()],
            &[],
            &[],
            SandboxPermissions {
                allow_network: false,
                allow_fs_write: true,
            },
        )
        .unwrap();

        let output = Command::new(&program)
            .args(&args)
            .output()
            .expect("failed to execute sandbox-exec");

        assert!(
            output.status.success(),
            "sandbox should allow redirecting to /dev/null: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[test]
    fn test_sandbox_allows_write_to_project_directory_when_fs_writes_allowed() {
        use std::process::Command;

        let temp_dir = tempfile::tempdir().unwrap();
        let test_file = temp_dir.path().join("test_write.txt");

        let (program, args, _config_file) = wrap_invocation(
            "/bin/sh",
            &[
                "-c".to_string(),
                format!("echo 'hello' > '{}'", test_file.display()),
            ],
            &[temp_dir.path()],
            &[],
            &[],
            SandboxPermissions {
                allow_network: false,
                allow_fs_write: true,
            },
        )
        .unwrap();

        let output = Command::new(&program)
            .args(&args)
            .output()
            .expect("failed to execute sandbox-exec");

        assert!(
            output.status.success(),
            "sandbox should allow writing to project dir: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(test_file.exists(), "file should have been created");
    }

    #[test]
    fn test_sandbox_allows_write_to_any_listed_writable_directory() {
        use std::process::Command;

        let project_dir = tempfile::tempdir().unwrap();
        let scratch_dir = tempfile::tempdir().unwrap();
        let test_file = scratch_dir.path().join("test_write.txt");

        let (program, args, _config_file) = wrap_invocation(
            "/bin/sh",
            &[
                "-c".to_string(),
                format!("echo 'hello' > '{}'", test_file.display()),
            ],
            &[project_dir.path(), scratch_dir.path()],
            &[],
            &[],
            SandboxPermissions::default(),
        )
        .unwrap();

        let output = Command::new(&program)
            .args(&args)
            .output()
            .expect("failed to execute sandbox-exec");

        assert!(
            output.status.success(),
            "sandbox should allow writing to a non-first writable directory: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(test_file.exists(), "file should have been created");
    }

    #[test]
    fn test_sandbox_allows_write_to_project_directory_by_default() {
        use std::process::Command;

        let temp_dir = tempfile::tempdir().unwrap();
        let test_file = temp_dir.path().join("test_write.txt");

        let (program, args, _config_file) = wrap_invocation(
            "/bin/sh",
            &[
                "-c".to_string(),
                format!("echo 'hello' > '{}'", test_file.display()),
            ],
            &[temp_dir.path()],
            &[],
            &[],
            SandboxPermissions::default(),
        )
        .unwrap();

        let output = Command::new(&program)
            .args(&args)
            .output()
            .expect("failed to execute sandbox-exec");

        assert!(
            output.status.success(),
            "sandbox should allow writing to project dir by default: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(test_file.exists(), "file should have been created");
    }

    #[test]
    fn test_sandbox_allows_write_to_system_tmp_when_fs_writes_allowed() {
        use std::process::Command;

        let project_dir = tempfile::tempdir().unwrap();
        let test_file = PathBuf::from("/tmp/zed-sandbox-write-test");
        let _ = std::fs::remove_file(&test_file);

        let (program, args, _config_file) = wrap_invocation(
            "/bin/sh",
            &[
                "-c".to_string(),
                format!("echo 'hello' > '{}'", test_file.display()),
            ],
            &[project_dir.path()],
            &[],
            &[],
            SandboxPermissions {
                allow_network: false,
                allow_fs_write: true,
            },
        )
        .unwrap();

        let output = Command::new(&program)
            .args(&args)
            .output()
            .expect("failed to execute sandbox-exec");

        assert!(
            output.status.success(),
            "sandbox should allow writing to system tmp when filesystem writes are allowed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(test_file.exists(), "file should have been created");
        let _ = std::fs::remove_file(&test_file);
    }

    #[test]
    fn test_sandbox_denies_write_outside_project_directory_by_default() {
        use std::process::Command;

        let project_dir = tempfile::tempdir().unwrap();
        let forbidden_file = std::env::home_dir()
            .unwrap()
            .join(".zed-sandbox-forbidden-write-test");
        let _ = std::fs::remove_file(&forbidden_file);

        let (program, args, _config_file) = wrap_invocation(
            "/bin/sh",
            &[
                "-c".to_string(),
                format!("echo 'hello' > '{}'", forbidden_file.display()),
            ],
            &[project_dir.path()],
            &[],
            &[],
            SandboxPermissions::default(),
        )
        .unwrap();

        let output = Command::new(&program)
            .args(&args)
            .output()
            .expect("failed to execute sandbox-exec");

        assert!(
            !output.status.success(),
            "sandbox should deny writing outside project dir when filesystem writes are not allowed"
        );
        assert!(
            !forbidden_file.exists(),
            "file should not have been created"
        );
    }
}
