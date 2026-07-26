//! Test binary used exclusively by the NixOS integration tests for the
//! Bubblewrap sandbox at `nix/tests/sandboxing`. See the comment there.
//!
//! It drives the sandbox crate's *public* API only (`Sandbox`, `SandboxPolicy`,
//! …) — never platform internals — so it doubles as a check that the public API
//! is sufficient to express and enforce the policies the agent needs.
//!
//! The list of checks to run is *data*, declared by the Nix test and handed to
//! this binary as a JSON file (path in `ZED_SANDBOX_CHECKS`). Each check
//! describes a sandbox policy, an operation to attempt under that policy, and
//! the expected outcome; this binary executes each and asserts the result.

#![allow(
    clippy::disallowed_methods,
    reason = "a single-threaded test helper that intentionally blocks on child processes"
)]

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("bwrap_test_helper is only supported on Linux");
    std::process::exit(1);
}

#[cfg(target_os = "linux")]
fn main() {
    imp::main();
}

#[cfg(target_os = "linux")]
mod imp {
    use std::io::{Read as _, Write as _};
    use std::net::TcpStream;
    use std::path::Path;
    use std::time::Duration;

    use anyhow::{Context as _, Result, bail};
    use sandbox::{
        CommandAndArgs, HostFilesystemLocation, Sandbox, SandboxError, SandboxFsPolicy,
        SandboxNetPolicy, SandboxPolicy,
    };
    use serde::Deserialize;

    /// Internal subcommand: round-trip a byte through the echo server at the
    /// given `host:port`, honoring `HTTP_PROXY` when set (the restricted-network
    /// case routes through the sandbox proxy via HTTP CONNECT). Exits 0 on a
    /// successful round-trip, non-zero otherwise. Run *inside* the sandbox.
    const SUBCOMMAND_ECHO_CHECK: &str = "__echo_check";

    /// Internal subcommand: connect to the unix-domain socket at the given path
    /// and round-trip a byte through it. Exits 0 on a successful round-trip,
    /// non-zero on any failure (including `socket(AF_UNIX)` being blocked once
    /// the seccomp guard lands). Run *inside* the sandbox.
    const SUBCOMMAND_UNIX_CONNECT_CHECK: &str = "__unix_connect_check";

    /// Default port for echo targets given as a bare hostname (e.g. `echo1`).
    const DEFAULT_ECHO_PORT: &str = "7000";

    pub fn main() {
        // If we were re-exec'd as the restricted-network bridge, this starts the
        // bridge and execs the wrapped command without returning.
        sandbox::run_sandbox_launcher_if_invoked();

        let args: Vec<String> = std::env::args().collect();
        let result = match args.get(1).map(String::as_str) {
            Some(SUBCOMMAND_ECHO_CHECK) => run_echo_check(args.get(2).map(String::as_str)),
            Some(SUBCOMMAND_UNIX_CONNECT_CHECK) => {
                run_unix_connect_check(args.get(2).map(String::as_str))
            }
            _ => run_checks(),
        };

        if let Err(error) = result {
            eprintln!("[sandbox_test]: FAILED: {error:#}");
            std::process::exit(1);
        }
    }

    /// Filesystem policy as declared in a check (`fs` field).
    #[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq)]
    #[serde(rename_all = "lowercase")]
    enum FsMode {
        /// Reads allowed everywhere; writes confined to `writablePaths`.
        #[default]
        Restricted,
        /// Writes allowed anywhere.
        Unrestricted,
    }

    /// Network policy as declared in a check (`networkAccess` field).
    #[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq)]
    #[serde(rename_all = "lowercase")]
    enum NetMode {
        /// All outbound network blocked.
        #[default]
        Blocked,
        /// All outbound network allowed.
        Unrestricted,
        /// Outbound HTTP(S) allowed only to `allowedDomains`.
        Restricted,
    }

    /// One declarative check: a sandbox policy, an operation, and the expected
    /// result. Deserialized from the JSON the Nix test produces.
    ///
    /// Exactly one operation field (`read`, `write`, `network`, `socketPath`, or
    /// `canCreate`) must be set. Policy fields default to the most-confined policy
    /// (restricted filesystem with no writable paths, blocked network).
    #[derive(Debug, Default, Deserialize)]
    #[serde(rename_all = "camelCase", deny_unknown_fields)]
    struct Check {
        /// Optional human label; falls back to an auto-generated description.
        #[serde(default)]
        name: Option<String>,

        // ---- policy ----
        #[serde(default)]
        fs: FsMode,
        #[serde(default)]
        writable_paths: Vec<String>,
        #[serde(default)]
        network_access: NetMode,
        #[serde(default)]
        allowed_domains: Vec<String>,
        /// Paths to protect from writes even if they fall under a writable path.
        #[serde(default)]
        protected_paths: Vec<String>,

        // ---- operation (exactly one) ----
        /// Read this host path from inside the sandbox.
        #[serde(default)]
        read: Option<String>,
        /// Write to this host path from inside the sandbox.
        #[serde(default)]
        write: Option<String>,
        /// Connect to this echo host (`hostname` or `hostname:port`) from inside
        /// the sandbox.
        #[serde(default)]
        network: Option<String>,
        /// Connect to this unix-domain socket path from inside the sandbox.
        #[serde(default)]
        socket_path: Option<String>,
        /// Assert that `Sandbox::can_create` for this policy matches the value:
        /// `true` => the sandbox can be created, `false` => it cannot.
        #[serde(default)]
        can_create: Option<bool>,

        // ---- expectation ----
        /// Expected outcome for `read` / `write` / `network`.
        #[serde(default)]
        succeeds: Option<bool>,
        /// Expected error kind for `canCreate = false`
        /// (`bwrap_not_found` | `setuid_rejected` | `probe_failed`). When
        /// omitted, any creation failure is accepted.
        #[serde(default)]
        error: Option<String>,
    }

    fn run_checks() -> Result<()> {
        // The restricted-network proxy runs in *this* process and the echo
        // servers live on the VM's private network, which the proxy's
        // DNS-rebinding protection would otherwise reject. The proxy only honors
        // this var when built with `http_proxy/nixos-integration-tests` (pulled
        // in by `sandbox/nixos-test`, which this binary requires), so it has no
        // effect in a real Zed build.
        // SAFETY: single-threaded at this point.
        unsafe {
            std::env::set_var("ZED_SANDBOX_PROXY_ALLOW_LOCAL_IPS", "1");
        }

        let checks_path =
            std::env::var("ZED_SANDBOX_CHECKS").context("ZED_SANDBOX_CHECKS must be set")?;
        let raw = std::fs::read_to_string(&checks_path)
            .with_context(|| format!("failed to read checks file {checks_path}"))?;
        let specs: Vec<Check> =
            serde_json::from_str(&raw).context("failed to parse checks JSON")?;
        let echo_port =
            std::env::var("ZED_TEST_ECHO_PORT").unwrap_or_else(|_| DEFAULT_ECHO_PORT.to_string());

        println!("[sandbox_test]: running {} check(s)", specs.len());

        let mut checks = Checks::new();
        for spec in &specs {
            run_check(spec, &echo_port, &mut checks)?;
        }
        checks.finish()
    }

    fn policy_of(check: &Check) -> Result<SandboxPolicy> {
        let fs = match check.fs {
            FsMode::Unrestricted => SandboxFsPolicy::Unrestricted {
                protected_paths: capture_protected_paths(&check.protected_paths),
            },
            FsMode::Restricted => {
                let mut writable_paths = Vec::new();
                for path in &check.writable_paths {
                    // Mirror production (`acp_thread::SandboxWrap::to_policy`):
                    // the directory must exist before its inode can be pinned, so
                    // create it up front, then capture it.
                    std::fs::create_dir_all(path)
                        .with_context(|| format!("failed to create writable path {path}"))?;
                    writable_paths.push(
                        HostFilesystemLocation::new(path)
                            .with_context(|| format!("failed to capture writable path {path}"))?,
                    );
                }
                let protected_paths = capture_protected_paths(&check.protected_paths);
                SandboxFsPolicy::Restricted {
                    writable_paths,
                    protected_paths,
                }
            }
        };
        let network = match check.network_access {
            NetMode::Unrestricted => SandboxNetPolicy::Unrestricted,
            NetMode::Blocked => SandboxNetPolicy::Blocked,
            NetMode::Restricted => SandboxNetPolicy::Restricted {
                allowed_domains: check.allowed_domains.clone(),
            },
        };
        Ok(SandboxPolicy { fs, network })
    }

    /// Capture each already-existing protected path, mirroring production's
    /// fail-closed `filter_map(HostFilesystemLocation::new(..).ok())`: a path
    /// that does not yet exist can't be pinned and is simply skipped. Unlike
    /// writable paths, these are never created here — whether one exists is
    /// exactly what several checks turn on.
    fn capture_protected_paths(paths: &[String]) -> Vec<HostFilesystemLocation> {
        paths
            .iter()
            .filter_map(|path| HostFilesystemLocation::new(path).ok())
            .collect()
    }

    fn describe(check: &Check) -> String {
        if let Some(name) = &check.name {
            return name.clone();
        }
        let protected = if check.protected_paths.is_empty() {
            String::new()
        } else {
            format!(",protected_paths={:?}", check.protected_paths)
        };
        let policy = format!(
            "fs={:?},net={:?}{protected}",
            check.fs, check.network_access
        );
        let op = if let Some(path) = &check.read {
            format!("read {path}")
        } else if let Some(path) = &check.write {
            format!("write {path}")
        } else if let Some(host) = &check.network {
            format!("network {host}")
        } else if let Some(path) = &check.socket_path {
            format!("socket_connect {path}")
        } else if let Some(expected) = check.can_create {
            format!("can_create == {expected}")
        } else {
            "<no operation>".to_string()
        };
        format!("[{policy}] {op}")
    }

    fn run_check(check: &Check, echo_port: &str, checks: &mut Checks) -> Result<()> {
        let label = describe(check);

        if let Some(expect_can_create) = check.can_create {
            let policy = policy_of(check)?;
            let outcome = Sandbox::can_create(&policy);
            let passed = match (&outcome, expect_can_create) {
                (Ok(()), true) => true,
                (Ok(()), false) => false,
                (Err(_), true) => false,
                (Err(error), false) => check
                    .error
                    .as_deref()
                    .map(|expected| error_matches(error, expected))
                    .unwrap_or(true),
            };
            checks.check(&format!("{label} (got {outcome:?})"), passed);
            return Ok(());
        }

        let succeeds = check
            .succeeds
            .with_context(|| format!("check {label:?} has an operation but no `succeeds`"))?;

        let actual = if let Some(path) = &check.read {
            run_read(check, path)?
        } else if let Some(path) = &check.write {
            run_write(check, path)?
        } else if let Some(host) = &check.network {
            run_network(check, host, echo_port)?
        } else if let Some(path) = &check.socket_path {
            run_socket_connect(check, path)?
        } else {
            bail!("check {label:?} has no operation");
        };

        checks.check(&label, actual == succeeds);
        Ok(())
    }

    /// Seed a host file, then `cat` it from inside the sandbox. Reads are always
    /// granted (root is bound read-only), so this proves the sandbox doesn't
    /// *block* reads of existing host files.
    fn run_read(check: &Check, path: &str) -> Result<bool> {
        let path = Path::new(path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("failed to create parent of {}", path.display()))?;
        }
        std::fs::write(path, b"sandbox-test\n")
            .with_context(|| format!("failed to seed readable file {}", path.display()))?;

        // Build the policy only after the fixtures exist: capturing a
        // `HostFilesystemLocation` pins the inode, so the path must be present.
        let policy = policy_of(check)?;
        let mut sandbox = Sandbox::new(policy).map_err(sandbox_err)?;
        run_command(
            &mut sandbox,
            "sh",
            &["-c", &format!("cat {}", shell_quote(path))],
        )
    }

    /// Attempt to write a host file from inside the sandbox. "Succeeded" means
    /// the command exited 0 *and* the bytes actually landed on the host file —
    /// a write that only hits the sandbox's ephemeral tmpfs counts as blocked,
    /// since it never escaped the sandbox.
    fn run_write(check: &Check, path: &str) -> Result<bool> {
        let path = Path::new(path);
        if let Some(parent) = path.parent() {
            // Create the parent on the host so the only thing under test is the
            // sandbox's write permission, not a missing directory. This also
            // makes a protected parent exist before the policy captures it.
            std::fs::create_dir_all(parent)
                .with_context(|| format!("failed to create parent of {}", path.display()))?;
        }
        // Start from a clean slate so `exists()` afterwards is meaningful.
        let _ = std::fs::remove_file(path);

        // Build the policy only after the fixtures exist: capturing a
        // `HostFilesystemLocation` pins the inode, so the path must be present.
        let policy = policy_of(check)?;
        let mut sandbox = Sandbox::new(policy).map_err(sandbox_err)?;
        let command_ok = run_command(
            &mut sandbox,
            "sh",
            &[
                "-c",
                &format!("printf sandbox-test > {}", shell_quote(path)),
            ],
        )?;
        Ok(command_ok && path.exists())
    }

    /// Connect to `host` (`hostname` or `hostname:port`) from inside the
    /// sandbox via the `__echo_check` subcommand, which honors `HTTP_PROXY` for
    /// the restricted-network case.
    fn run_network(check: &Check, host: &str, echo_port: &str) -> Result<bool> {
        let target = if host.contains(':') {
            host.to_string()
        } else {
            format!("{host}:{echo_port}")
        };
        let exe = current_exe_str()?;
        let policy = policy_of(check)?;
        let mut sandbox = Sandbox::new(policy).map_err(sandbox_err)?;
        run_command(&mut sandbox, &exe, &[SUBCOMMAND_ECHO_CHECK, &target])
    }

    /// Attempt to connect to the unix-domain socket at `path` from inside the
    /// sandbox via the `__unix_connect_check` subcommand, returning whether the
    /// round-trip succeeded. A read-only bind mount of `/` leaves the socket
    /// reachable, so a sandboxed command can currently `connect()` to a session
    /// IPC socket owned by a process *outside* the sandbox — the escape a
    /// `socket(AF_UNIX)` seccomp filter is meant to block. When that guard lands,
    /// `socket(AF_UNIX)` returns `EPERM`, the subcommand fails, and this returns
    /// `false`.
    fn run_socket_connect(check: &Check, path: &str) -> Result<bool> {
        let exe = current_exe_str()?;
        let policy = policy_of(check)?;
        let mut sandbox = Sandbox::new(policy).map_err(sandbox_err)?;
        run_command(&mut sandbox, &exe, &[SUBCOMMAND_UNIX_CONNECT_CHECK, path])
    }

    fn error_matches(error: &SandboxError, expected: &str) -> bool {
        matches!(
            (error, expected),
            (SandboxError::BwrapNotFound, "bwrap_not_found")
                | (SandboxError::BwrapSetuidRejected, "setuid_rejected")
                | (SandboxError::SandboxProbeFailed, "probe_failed")
        )
    }

    fn sandbox_err(error: SandboxError) -> anyhow::Error {
        anyhow::anyhow!("failed to create sandbox: {error}")
    }

    /// Inner command: prove (or fail to prove) reachability of an echo server.
    ///
    /// With `HTTP_PROXY` set we open an HTTP `CONNECT` tunnel through the proxy
    /// and then echo a byte; a policy denial shows up as a non-200 status. With
    /// no proxy we connect directly. Either way, a clean round-trip means the
    /// destination was reachable under the active network policy.
    fn run_echo_check(target: Option<&str>) -> Result<()> {
        let target = target.context("echo check requires a host:port argument")?;
        let proxy = std::env::var("HTTP_PROXY")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .or_else(|| {
                std::env::var("http_proxy")
                    .ok()
                    .filter(|value| !value.trim().is_empty())
            });

        let mut stream = match proxy {
            Some(proxy_url) => {
                let proxy_addr = proxy_url
                    .trim()
                    .strip_prefix("http://")
                    .unwrap_or(proxy_url.trim())
                    .trim_end_matches('/')
                    .to_string();
                let mut stream = TcpStream::connect(&proxy_addr)
                    .with_context(|| format!("failed to connect to proxy {proxy_addr}"))?;
                stream.set_read_timeout(Some(Duration::from_secs(10)))?;
                write!(
                    stream,
                    "CONNECT {target} HTTP/1.1\r\nHost: {target}\r\n\r\n"
                )
                .context("failed to send CONNECT to proxy")?;
                let status = read_status_line(&mut stream)?;
                if !status.contains(" 200") {
                    bail!("proxy refused CONNECT to {target}: {status:?}");
                }
                stream
            }
            None => {
                let stream = TcpStream::connect(target)
                    .with_context(|| format!("failed to connect to {target}"))?;
                stream.set_read_timeout(Some(Duration::from_secs(10)))?;
                stream
            }
        };

        stream
            .write_all(b"ping\n")
            .context("failed to write to echo server")?;
        let mut buffer = [0u8; 32];
        let read = stream
            .read(&mut buffer)
            .context("failed to read from echo server")?;
        let echoed = String::from_utf8_lossy(&buffer[..read]);
        if echoed.contains("ping") {
            Ok(())
        } else {
            bail!("echo server returned unexpected data: {echoed:?}");
        }
    }

    /// Inner command: connect to the unix-domain socket at `path` and round-trip
    /// a byte through it.
    ///
    /// Any failure — `socket(AF_UNIX)` being denied (how the seccomp guard will
    /// manifest, as `EPERM`), `connect()` failing, or a bad round-trip — exits
    /// non-zero, so the caller reads it as "not connected". A clean round-trip
    /// (exit 0) means the socket outside the sandbox was reachable.
    fn run_unix_connect_check(path: Option<&str>) -> Result<()> {
        use std::os::unix::net::UnixStream;

        let path = path.context("unix connect check requires a socket path argument")?;
        let mut stream = UnixStream::connect(path)
            .with_context(|| format!("failed to connect to unix socket {path}"))?;
        stream.set_read_timeout(Some(Duration::from_secs(10)))?;
        stream
            .write_all(b"ping\n")
            .context("failed to write to unix socket")?;
        let mut buffer = [0u8; 32];
        let read = stream
            .read(&mut buffer)
            .context("failed to read from unix socket")?;
        let echoed = String::from_utf8_lossy(&buffer[..read]);
        if echoed.contains("ping") {
            Ok(())
        } else {
            bail!("unix socket returned unexpected data: {echoed:?}");
        }
    }

    /// Read an HTTP status line (up to the first CRLF), then drain the rest of
    /// the header block (up to the blank line) so the stream is positioned at
    /// the tunneled body.
    fn read_status_line(stream: &mut TcpStream) -> Result<String> {
        let mut header = Vec::new();
        let mut byte = [0u8; 1];
        loop {
            let read = stream
                .read(&mut byte)
                .context("failed reading proxy reply")?;
            if read == 0 {
                break;
            }
            header.push(byte[0]);
            if header.ends_with(b"\r\n\r\n") {
                break;
            }
            if header.len() > 64 * 1024 {
                bail!("proxy reply headers too large");
            }
        }
        let text = String::from_utf8_lossy(&header);
        Ok(text.lines().next().unwrap_or_default().to_string())
    }

    /// Tracks assertion results so we can report a summary and a single exit code.
    struct Checks {
        passed: usize,
        failed: usize,
    }

    impl Checks {
        fn new() -> Self {
            Self {
                passed: 0,
                failed: 0,
            }
        }

        fn check(&mut self, description: &str, condition: bool) {
            if condition {
                self.passed += 1;
                println!("[sandbox_test]: PASS: {description}");
            } else {
                self.failed += 1;
                println!("[sandbox_test]: FAIL: {description}");
            }
        }

        fn finish(self) -> Result<()> {
            println!(
                "[sandbox_test]: summary: {} passed, {} failed",
                self.passed, self.failed
            );
            if self.failed > 0 {
                bail!("{} sandbox assertion(s) failed", self.failed);
            }
            Ok(())
        }
    }

    /// Wrap and run a command inside `sandbox`, returning whether it exited 0.
    fn run_command(sandbox: &mut Sandbox, program: &str, args: &[&str]) -> Result<bool> {
        let command = CommandAndArgs {
            program: program.to_string(),
            args: args.iter().map(|arg| arg.to_string()).collect(),
            env: Default::default(),
            cwd: None,
        };
        let output = futures::executor::block_on(sandbox.execute(&command))
            .map_err(|error| anyhow::anyhow!("failed to run sandboxed command: {error}"))?;
        Ok(output.status.success())
    }

    fn current_exe_str() -> Result<String> {
        Ok(std::env::current_exe()
            .context("failed to resolve current executable")?
            .to_str()
            .context("current executable path is not valid UTF-8")?
            .to_string())
    }

    /// Single-quote a path for safe interpolation into an `sh -c` script.
    fn shell_quote(path: &Path) -> String {
        let raw = path.to_string_lossy();
        format!("'{}'", raw.replace('\'', "'\\''"))
    }
}
