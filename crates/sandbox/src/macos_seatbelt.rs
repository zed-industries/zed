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
    /// Network access policy for the command.
    pub network: NetworkAccess,
    /// Allow unrestricted filesystem writes.
    pub allow_fs_write: bool,
}

/// Network-access setting for a sandboxed command.
///
/// The default ([`NetworkAccess::None`]) blocks all outbound network at the
/// Seatbelt layer. [`NetworkAccess::LocalhostPort`] confines the command to a
/// single loopback port — used to force all egress through the in-process
/// HTTP/HTTPS proxy that enforces a hostname allowlist (see the `http_proxy`
/// crate). [`NetworkAccess::All`] lifts the restriction entirely.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum NetworkAccess {
    /// All outbound network blocked.
    #[default]
    None,
    /// Outbound TCP allowed only to `localhost:<port>`. Used to confine
    /// sandboxed commands to the in-process HTTP/HTTPS proxy.
    LocalhostPort(u16),
    /// All outbound network allowed.
    All,
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
    /// `protected_paths` lists paths whose writes should be blocked even if they
    /// fall under a writable directory. Contents remain readable.
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
/// * `protected_paths` - Paths whose writes should be denied even if they fall
///   under a writable directory. Contents remain readable.
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
/// default. Writes to paths in `protected_paths` are denied even when they
/// would otherwise be writable; contents remain readable. Unix domain socket paths in
/// `allowed_unix_socket_paths` are reachable for local IPC even when IP
/// network access is otherwise blocked. This does not permit sending packets
/// to other machines.
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
    // These paths are already the canonical identities captured once, at
    // validation time, inside each `HostFilesystemLocation` (resolving symlinks
    // and, for a not-yet-created leaf, its existing parent). We deliberately do
    // NOT re-canonicalize here: re-resolving a path at profile-generation time
    // is the time-of-check-to-time-of-use hole this design closes. Use them
    // verbatim as Seatbelt rule literals.
    let canonical_writable_directories: Vec<PathBuf> = writable_directories
        .iter()
        .map(|path| path.to_path_buf())
        .collect();
    let canonical_protected_paths: Vec<PathBuf> = protected_paths
        .iter()
        .map(|path| path.to_path_buf())
        .collect();
    // Unlike file paths, Unix socket literals are emitted verbatim: it isn't
    // guaranteed whether Seatbelt resolves symlinks before matching a
    // `remote unix-socket` literal, so the caller passes both the path the
    // child connects to and its canonical form, and we keep them as given.

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

; Mach service lookups. This is an ALLOWLIST, not a blanket `(allow mach-lookup)`.
; An unrestricted mach-lookup lets a sandboxed command reach LaunchServices /
; launchd and have a process spawned *outside* the sandbox (e.g. `open -a
; Terminal`, or opening a crafted `.app`) — the launched process does not inherit
; this profile, so that is a full sandbox escape. So we allow only the services
; ordinary dev tooling needs and deliberately EXCLUDE the LaunchServices/launchd
; endpoints (closing that escape), the pasteboard (silent clipboard theft), and
; audio (mic/privacy). Curated from Codex's and Chromium's Seatbelt policies; add
; an entry here (with a comment) if a legitimate toolchain needs another service.
; A non-existent name is simply never matched, so erring toward including
; plausible infrastructure services is safe.
(allow mach-lookup
    ; identity: user & group resolution (getpwuid, id, whoami, perm checks)
    (global-name "com.apple.system.opendirectoryd.libinfo")
    (global-name "com.apple.system.opendirectoryd.membership")
    (global-name "com.apple.system.DirectoryService.libinfo_v1")
    ; per-user temp/cache dir resolution ($TMPDIR, /var/folders/...)
    (global-name "com.apple.bsd.dirhelper")
    ; CFPreferences (pervasive in Apple frameworks linked by dev tools).
    ; Chromium denies cfprefsd.daemon to force in-process prefs; we follow Codex
    ; and allow it since dev commands legitimately use many prefs domains — it's
    ; a prefs read/write, not an escape.
    (global-name "com.apple.cfprefsd.daemon")
    (global-name "com.apple.cfprefsd.agent")
    (local-name "com.apple.cfprefsd.agent")
    ; logging / diagnostics (os_log, ASL, Darwin notifications)
    (global-name "com.apple.logd")
    (global-name "com.apple.logd.events")
    (global-name "com.apple.system.logger")
    (global-name "com.apple.diagnosticd")
    (global-name "com.apple.system.notification_center")
    ; Apple telemetry (data goes to Apple only; harmless, avoids init latency)
    (global-name "com.apple.analyticsd")
    (global-name "com.apple.analyticsd.messagetracer")
    ; power assertions (caffeinate / prevent idle sleep during long builds)
    (global-name "com.apple.PowerManagement.control")
    ; developer-tools automation-mode flag (our workload is dev tooling)
    (global-name "com.apple.dt.automationmode.reader"))

; Allow pseudo-terminal operations
(allow pseudo-tty)
(allow file-read* file-write* file-ioctl
    (literal "/dev/ptmx"))
(allow file-read* file-write*
    (require-all
        (regex #"^/dev/ttys[0-9]+$")
        (extension "com.apple.sandbox.pty")))

; The command's PTY is allocated after this profile is generated, so its slave
; TTY path isn't known here and may lack the `com.apple.sandbox.pty` extension.
; Allow ioctls on slave TTYs so interactive shells and signing prompts can
; manipulate terminal state. Seatbelt can't filter by ioctl request number, so
; this can't exclude input-injection ioctls (e.g. TIOCSTI) specifically. The
; residual risk is bounded by the kernel, not by this profile: XNU's TIOCSTI
; handler (bsd/kern/tty.c) rejects a non-root caller unless the target TTY is
; the caller's own controlling terminal (EACCES otherwise). Each agent command
; runs in its own dedicated PTY, so the only TTY it can inject into is that
; throwaway PTY, not the user's interactive terminal. The regex is also anchored
; so it matches only `/dev/ttysNNN` device nodes.
(allow file-ioctl
    (regex #"^/dev/ttys[0-9]+$"))
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
        // `subpath` already matches the path itself plus everything beneath it,
        // so no redundant `literal` rule is needed.
        config.push_str(&format!(
            r#"
; Block protected path writes
(deny file-write*
    (subpath "{escaped_path}"))
"#
        ));
    }

    match permissions.network {
        NetworkAccess::None => {}
        NetworkAccess::All => {
            config.push_str(
                r#"
; Allow network access
(allow network*)
"#,
            );
        }
        NetworkAccess::LocalhostPort(port) => {
            // Seatbelt rejects IP literals in `(remote tcp ...)` rules — it
            // only accepts `*` or `localhost` as the host part. The runtime
            // resolves correctly when the sandboxed process connects to
            // `127.0.0.1:<port>`, so this is a syntactic constraint, not a
            // routing one.
            config.push_str(&format!(
                r#"
; Allow outbound network only to the in-process proxy on localhost
(allow network-outbound (remote tcp "localhost:{port}"))
; Network binds (sandboxed process picking its own ephemeral source port)
(allow network-bind (local ip "localhost:*"))
"#,
            ));
        }
    }

    // When outbound network is permitted at all, tools that do their own DNS
    // resolution, TLS trust evaluation, and network-configuration lookups need a
    // few more Mach services. Kept out of the base allowlist so a no-network
    // command can't reach them. Still an allowlist (mirrors Codex's Seatbelt
    // network policy) that excludes LaunchServices/launchd.
    if !matches!(permissions.network, NetworkAccess::None) {
        config.push_str(
            r#"
; Extra Mach services for DNS / TLS-trust / network configuration, needed only
; when outbound network is permitted. Still an allowlist that excludes
; LaunchServices/launchd. If hostname resolution fails, add
; `com.apple.mDNSResponder`; if offline code-signature verification of loaded
; dylibs/plugins fails, move the trust services into the base block above.
(allow mach-lookup
    ; network / DNS configuration
    (global-name "com.apple.SystemConfiguration.configd")
    (global-name "com.apple.SystemConfiguration.DNSConfiguration")
    (global-name "com.apple.networkd")
    ; TLS certificate trust / keychain / revocation
    (global-name "com.apple.SecurityServer")
    (global-name "com.apple.trustd")
    (global-name "com.apple.trustd.agent")
    (global-name "com.apple.ocspd"))
"#,
        );
    }

    if !allowed_unix_socket_paths.is_empty() {
        config.push_str(
            r#"
; Allow local IPC to inherited Unix domain sockets. Seatbelt models this as
; network-outbound, but this does not permit IP networking or sending packets
; to other machines.
;
; `system-socket` only governs the `socket()` syscall (creating an AF_UNIX
; socket), which is harmless on its own. The capability that matters,
; `connect()`, stays gated by the per-path `network-outbound (remote
; unix-socket ...)` rules below, so `(deny default)` still blocks connecting to
; any socket not explicitly allow-listed.
(allow system-socket
    (socket-domain AF_UNIX))
"#,
        );

        for socket_path in allowed_unix_socket_paths {
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

    /// Strip SBPL comment lines (`;`-prefixed) from a generated Seatbelt profile
    /// so assertions match on the actual rules rather than on documentation.
    /// Several comments legitimately mention rule syntax (for example the
    /// blanket `(allow mach-lookup)` form they explain we avoid), which would
    /// otherwise cause a naive substring check to spuriously match.
    fn seatbelt_rules_only(config: &str) -> String {
        config
            .lines()
            .filter(|line| !line.trim_start().starts_with(';'))
            .collect::<Vec<_>>()
            .join("\n")
    }

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
                network: NetworkAccess::None,
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
    fn test_generate_seatbelt_config_scopes_mach_lookup_and_excludes_escape_services() {
        let dir = PathBuf::from("/Users/test/projects/myproject");
        let config =
            generate_seatbelt_config(&[dir.as_path()], &[], &[], SandboxPermissions::default())
                .unwrap();
        // Assert on the rules only: the mach-lookup comment intentionally spells
        // out the blanket `(allow mach-lookup)` form it avoids, which a raw
        // `config.contains` would match.
        let rules = seatbelt_rules_only(&config);

        // A scoped allowlist, never the blanket form — a blanket `(allow
        // mach-lookup)` would let a command reach LaunchServices/launchd and
        // escape the sandbox via `open`.
        assert!(rules.contains("(allow mach-lookup"));
        assert!(!rules.contains("(allow mach-lookup)"));
        assert!(rules.contains("com.apple.cfprefsd.daemon"));

        // The escape/abuse endpoints must fall through to `(deny default)`.
        assert!(!rules.contains("launchservicesd"));
        assert!(!rules.contains("com.apple.lsd"));
        assert!(!rules.contains("com.apple.pasteboard"));

        // Network-only services must not be granted without network.
        assert!(!rules.contains("com.apple.SecurityServer"));
        assert!(!rules.contains("com.apple.SystemConfiguration.configd"));
    }

    #[test]
    fn test_generate_seatbelt_config_adds_network_mach_services_when_network_allowed() {
        let dir = PathBuf::from("/Users/test/projects/myproject");
        let config = generate_seatbelt_config(
            &[dir.as_path()],
            &[],
            &[],
            SandboxPermissions {
                network: NetworkAccess::All,
                allow_fs_write: false,
            },
        )
        .unwrap();

        // DNS / TLS / network-config services appear only when network is allowed.
        assert!(config.contains("com.apple.SystemConfiguration.configd"));
        assert!(config.contains("com.apple.SecurityServer"));
        assert!(config.contains("com.apple.trustd"));
        assert!(config.contains("com.apple.trustd.agent"));
        // ...but never the escape endpoints.
        assert!(!config.contains("launchservicesd"));
    }

    #[test]
    fn test_generate_seatbelt_config_allows_unix_socket_paths_without_network() {
        let dir = PathBuf::from("/Users/test/projects/myproject");
        let socket_path = PathBuf::from("/private/tmp/com.example.test/Listeners");
        let config = generate_seatbelt_config(
            &[dir.as_path()],
            &[],
            &[socket_path.as_path()],
            SandboxPermissions::default(),
        )
        .unwrap();

        assert!(config.contains("(allow system-socket"));
        assert!(config.contains("AF_UNIX"));
        assert!(config.contains("(allow network-outbound"));
        assert!(config.contains("remote unix-socket"));
        assert!(config.contains("(literal \"/private/tmp/com.example.test/Listeners\")"));
        assert!(!config.contains("(subpath \"/private/tmp/com.example.test/Listeners\")"));
        assert!(!config.contains("(allow network*)"));
    }

    #[test]
    fn test_sandbox_allows_connecting_to_allowed_unix_socket() {
        use std::io::ErrorKind;
        use std::os::unix::net::UnixListener;
        use std::process::{Command, Stdio};
        use std::time::{Duration, Instant};

        // Bind under `/tmp` rather than the default temp dir: macOS temp paths
        // (`/var/folders/...`) overflow the `sun_path` limit for Unix sockets.
        // `TempDir` still cleans up on drop, even if the test panics.
        let temp_dir = tempfile::Builder::new()
            .prefix("zed-sock-")
            .tempdir_in("/tmp")
            .unwrap();
        let socket_path = temp_dir.path().join("agent.sock");
        let listener = UnixListener::bind(&socket_path).expect("test socket should bind");
        listener
            .set_nonblocking(true)
            .expect("listener should switch to non-blocking");

        let canonical_socket_path = socket_path
            .canonicalize()
            .expect("bound socket path should canonicalize");

        // Accept (and immediately drop) a single connection, with a bounded wait
        // so the test can't hang if the sandbox blocks the connection instead.
        let accepted = std::thread::spawn(move || {
            let deadline = Instant::now() + Duration::from_secs(10);
            loop {
                match listener.accept() {
                    Ok(_) => return true,
                    Err(error) if error.kind() == ErrorKind::WouldBlock => {
                        if Instant::now() >= deadline {
                            return false;
                        }
                        std::thread::sleep(Duration::from_millis(20));
                    }
                    Err(_) => return false,
                }
            }
        });

        let (program, args, _config_file) = wrap_invocation(
            "/usr/bin/nc",
            &[
                "-U".to_string(),
                "-w".to_string(),
                "5".to_string(),
                socket_path.display().to_string(),
            ],
            &[temp_dir.path()],
            &[],
            &[socket_path.as_path(), canonical_socket_path.as_path()],
            SandboxPermissions::default(),
        )
        .unwrap();

        let output = Command::new(&program)
            .args(&args)
            .stdin(Stdio::null())
            .output()
            .expect("failed to execute sandbox-exec");

        assert!(
            accepted.join().unwrap(),
            "sandbox should allow connecting to the allow-listed unix socket: stderr={} stdout={}",
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout),
        );
    }

    #[test]
    fn test_generate_seatbelt_config_denies_protected_path_writes() {
        let dir = PathBuf::from("/Users/test/projects/myproject");
        let protected = dir.join(".gitignore");
        let config = generate_seatbelt_config(
            &[dir.as_path()],
            &[protected.as_path()],
            &[],
            SandboxPermissions::default(),
        )
        .unwrap();

        assert!(config.contains("(deny file-write*"));
        assert!(!config.contains("file-read-data"));
        assert!(config.contains("(subpath \"/Users/test/projects/myproject/.gitignore\")"));
        // `subpath` already covers the path itself, so no redundant `literal`.
        assert!(!config.contains("(literal \"/Users/test/projects/myproject/.gitignore\")"));
    }

    #[test]
    fn test_sandbox_allows_protected_path_reads_but_blocks_writes() {
        use std::process::Command;

        let temp_dir = tempfile::tempdir().unwrap();
        // Canonicalize so the policy paths resolve the macOS `/var` -> `/private/var`
        // symlink; Seatbelt matches rules against the resolved path, as production
        // does via `HostFilesystemLocation`.
        let dir = std::fs::canonicalize(temp_dir.path()).unwrap();
        let protected_file = dir.join(".gitignore");
        std::fs::write(&protected_file, "target\n").unwrap();

        let (program, args, _config_file) = wrap_invocation(
            "/bin/sh",
            &[
                "-c".to_string(),
                format!(
                    "test -e '{}' && cat '{}' >/dev/null 2>&1 && ! sh -c 'echo changed > {}'",
                    protected_file.display(),
                    protected_file.display(),
                    protected_file.display(),
                ),
            ],
            &[dir.as_path()],
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
            "sandbox should allow protected reads but deny protected writes: stderr={} stdout={}",
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
        // See the sibling protected-path test: canonicalize so policy paths resolve
        // the macOS `/var` -> `/private/var` symlink that Seatbelt matches against.
        let dir = std::fs::canonicalize(temp_dir.path()).unwrap();
        let protected_file = dir.join(".gitignore");
        std::fs::write(&protected_file, "target\n").unwrap();

        let (program, args, _config_file) = wrap_invocation(
            "/bin/sh",
            &[
                "-c".to_string(),
                format!(
                    "cat '{}' >/dev/null 2>&1 && ! sh -c 'echo changed > {}'",
                    protected_file.display(),
                    protected_file.display(),
                ),
            ],
            &[dir.as_path()],
            &[protected_file.as_path()],
            &[],
            SandboxPermissions {
                network: NetworkAccess::None,
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
            "protected paths should stay readable but not writable even with unrestricted writes: stderr={} stdout={}",
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
                network: NetworkAccess::All,
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
    fn test_generate_seatbelt_config_localhost_port_narrows_network() {
        let dir = PathBuf::from("/Users/test/projects/myproject");
        let config = generate_seatbelt_config(
            &[dir.as_path()],
            &[],
            &[],
            SandboxPermissions {
                network: NetworkAccess::LocalhostPort(54321),
                allow_fs_write: false,
            },
        )
        .unwrap();

        // Only the loopback proxy port is reachable; no blanket network rule.
        assert!(config.contains("(allow network-outbound (remote tcp \"localhost:54321\"))"));
        assert!(config.contains("(allow network-bind (local ip \"localhost:*\"))"));
        assert!(!config.contains("(allow network*)"));
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
                network: NetworkAccess::None,
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
                network: NetworkAccess::None,
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
        // Canonicalize so the writable subpaths resolve the macOS `/var` ->
        // `/private/var` symlink that Seatbelt matches against.
        let project_path = std::fs::canonicalize(project_dir.path()).unwrap();
        let scratch_path = std::fs::canonicalize(scratch_dir.path()).unwrap();
        let test_file = scratch_path.join("test_write.txt");

        let (program, args, _config_file) = wrap_invocation(
            "/bin/sh",
            &[
                "-c".to_string(),
                format!("echo 'hello' > '{}'", test_file.display()),
            ],
            &[project_path.as_path(), scratch_path.as_path()],
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
        // Canonicalize so the writable subpath resolves the macOS `/var` ->
        // `/private/var` symlink that Seatbelt matches against.
        let dir = std::fs::canonicalize(temp_dir.path()).unwrap();
        let test_file = dir.join("test_write.txt");

        let (program, args, _config_file) = wrap_invocation(
            "/bin/sh",
            &[
                "-c".to_string(),
                format!("echo 'hello' > '{}'", test_file.display()),
            ],
            &[dir.as_path()],
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
                network: NetworkAccess::None,
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
