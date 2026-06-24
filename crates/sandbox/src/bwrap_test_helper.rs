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
    use std::path::{Path, PathBuf};
    use std::time::Duration;

    use anyhow::{Context as _, Result, bail};
    use sandbox::{
        CommandAndArgs, Sandbox, SandboxError, SandboxFsPolicy, SandboxNetPolicy, SandboxPolicy,
    };
    use serde::Deserialize;

    /// Internal subcommand: round-trip a byte through the echo server at the
    /// given `host:port`, honoring `HTTP_PROXY` when set (the restricted-network
    /// case routes through the sandbox proxy via HTTP CONNECT). Exits 0 on a
    /// successful round-trip, non-zero otherwise. Run *inside* the sandbox.
    const SUBCOMMAND_ECHO_CHECK: &str = "__echo_check";

    /// Default port for echo targets given as a bare hostname (e.g. `echo1`).
    const DEFAULT_ECHO_PORT: &str = "7000";

    pub fn main() {
        // If we were re-exec'd as the restricted-network bridge, this starts the
        // bridge and execs the wrapped command without returning.
        sandbox::run_sandbox_launcher_if_invoked();

        let args: Vec<String> = std::env::args().collect();
        let result = match args.get(1).map(String::as_str) {
            Some(SUBCOMMAND_ECHO_CHECK) => run_echo_check(args.get(2).map(String::as_str)),
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
    /// Exactly one operation field (`read`, `write`, `network`, or `canCreate`)
    /// must be set. Policy fields default to the most-confined policy
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

    fn policy_of(check: &Check) -> SandboxPolicy {
        let fs = match check.fs {
            FsMode::Unrestricted => SandboxFsPolicy::Unrestricted,
            FsMode::Restricted => SandboxFsPolicy::Restricted {
                writable_paths: check.writable_paths.iter().map(PathBuf::from).collect(),
            },
        };
        let network = match check.network_access {
            NetMode::Unrestricted => SandboxNetPolicy::Unrestricted,
            NetMode::Blocked => SandboxNetPolicy::Blocked,
            NetMode::Restricted => SandboxNetPolicy::Restricted {
                allowed_domains: check.allowed_domains.clone(),
            },
        };
        SandboxPolicy {
            fs,
            network,
            git: sandbox::GitSandboxPolicy::default(),
        }
    }

    fn describe(check: &Check) -> String {
        if let Some(name) = &check.name {
            return name.clone();
        }
        let policy = format!("fs={:?},net={:?}", check.fs, check.network_access);
        let op = if let Some(path) = &check.read {
            format!("read {path}")
        } else if let Some(path) = &check.write {
            format!("write {path}")
        } else if let Some(host) = &check.network {
            format!("network {host}")
        } else if let Some(expected) = check.can_create {
            format!("can_create == {expected}")
        } else {
            "<no operation>".to_string()
        };
        format!("[{policy}] {op}")
    }

    fn run_check(check: &Check, echo_port: &str, checks: &mut Checks) -> Result<()> {
        let label = describe(check);
        let policy = policy_of(check);

        if let Some(expect_can_create) = check.can_create {
            let outcome = Sandbox::can_create(&policy, None);
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
            run_read(&policy, path)?
        } else if let Some(path) = &check.write {
            run_write(&policy, path)?
        } else if let Some(host) = &check.network {
            run_network(&policy, host, echo_port)?
        } else {
            bail!("check {label:?} has no operation");
        };

        checks.check(&label, actual == succeeds);
        Ok(())
    }

    /// Seed a host file, then `cat` it from inside the sandbox. Reads are always
    /// granted (root is bound read-only), so this proves the sandbox doesn't
    /// *block* reads of existing host files.
    fn run_read(policy: &SandboxPolicy, path: &str) -> Result<bool> {
        let path = Path::new(path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("failed to create parent of {}", path.display()))?;
        }
        std::fs::write(path, b"sandbox-test\n")
            .with_context(|| format!("failed to seed readable file {}", path.display()))?;

        let mut sandbox = Sandbox::new(policy.clone()).map_err(sandbox_err)?;
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
    fn run_write(policy: &SandboxPolicy, path: &str) -> Result<bool> {
        let path = Path::new(path);
        if let Some(parent) = path.parent() {
            // Create the parent on the host so the only thing under test is the
            // sandbox's write permission, not a missing directory.
            std::fs::create_dir_all(parent)
                .with_context(|| format!("failed to create parent of {}", path.display()))?;
        }
        // Start from a clean slate so `exists()` afterwards is meaningful.
        let _ = std::fs::remove_file(path);

        let mut sandbox = Sandbox::new(policy.clone()).map_err(sandbox_err)?;
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
    fn run_network(policy: &SandboxPolicy, host: &str, echo_port: &str) -> Result<bool> {
        let target = if host.contains(':') {
            host.to_string()
        } else {
            format!("{host}:{echo_port}")
        };
        let exe = current_exe_str()?;
        let mut sandbox = Sandbox::new(policy.clone()).map_err(sandbox_err)?;
        run_command(&mut sandbox, &exe, &[SUBCOMMAND_ECHO_CHECK, &target])
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
