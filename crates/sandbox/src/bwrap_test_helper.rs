//! Test binary used exclusively by the NixOS integration tests for the
//! Bubblewrap sandbox at `nix/tests/sandboxing`. See the comment there.
//!
//! It drives the sandbox crate's *public* API only (`Sandbox`, `SandboxPolicy`,
//! …) — never platform internals — so it doubles as a check that the public API
//! is sufficient to express and enforce the policies the agent needs.

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

    /// Internal subcommand: round-trip a byte through the echo server at the
    /// given `host:port`, honoring `HTTP_PROXY` when set (the restricted-network
    /// case routes through the sandbox proxy via HTTP CONNECT). Exits 0 on a
    /// successful round-trip, non-zero otherwise. Run *inside* the sandbox.
    const SUBCOMMAND_ECHO_CHECK: &str = "__echo_check";

    pub fn main() {
        // If we were re-exec'd as the restricted-network bridge, this starts the
        // bridge and execs the wrapped command without returning.
        sandbox::run_sandbox_launcher_if_invoked();

        let args: Vec<String> = std::env::args().collect();
        let result = match args.get(1).map(String::as_str) {
            Some(SUBCOMMAND_ECHO_CHECK) => run_echo_check(args.get(2).map(String::as_str)),
            _ => run_tests(),
        };

        if let Err(error) = result {
            eprintln!("[sandbox_test]: FAILED: {error:#}");
            std::process::exit(1);
        }
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

    /// What the machine under test is expected to do when asked to sandbox.
    enum Expectation {
        /// The sandbox is enforceable; run the full policy matrix.
        Enforced,
        /// The sandbox can't be created; `Sandbox::can_create` must report this
        /// error, and the consumer must fail closed.
        CannotCreate(SandboxError),
    }

    fn run_tests() -> Result<()> {
        // The proxy runs in *this* process for restricted-network policies, and
        // the echo servers live on the VM's private network, which the proxy's
        // DNS-rebinding protection would otherwise reject. This escape hatch is
        // test-only (see `http_proxy`).
        // SAFETY: single-threaded at this point.
        unsafe {
            std::env::set_var("ZED_SANDBOX_PROXY_ALLOW_LOCAL_IPS", "1");
        }

        let expect = match std::env::var("ZED_TEST_EXPECT").as_deref() {
            Ok("enforced") => Expectation::Enforced,
            Ok("bwrap_not_found") => Expectation::CannotCreate(SandboxError::BwrapNotFound),
            Ok("setuid_rejected") => Expectation::CannotCreate(SandboxError::BwrapSetuidRejected),
            Ok("probe_failed") => Expectation::CannotCreate(SandboxError::SandboxProbeFailed),
            other => bail!(
                "ZED_TEST_EXPECT must be `enforced`, `bwrap_not_found`, `setuid_rejected`, or \
                 `probe_failed`, got {other:?}"
            ),
        };

        let echo1 = std::env::var("ZED_TEST_ECHO1").context("ZED_TEST_ECHO1 must be set")?;
        let echo2 = std::env::var("ZED_TEST_ECHO2").context("ZED_TEST_ECHO2 must be set")?;
        println!("[sandbox_test]: starting (echo1={echo1}, echo2={echo2})");

        // A host scratch tree the tests write into. Kept outside `/tmp` because
        // the sandbox overlays a fresh tmpfs there, which would mask whether
        // writes really hit (or were blocked from hitting) the host.
        let base = PathBuf::from(format!("/sandbox-test-{}", std::process::id()));
        let writable = base.join("writable");
        let forbidden = base.join("forbidden");
        let readable = base.join("readable");
        std::fs::create_dir_all(&writable).context("failed to create writable scratch dir")?;
        std::fs::create_dir_all(&forbidden).context("failed to create forbidden scratch dir")?;
        std::fs::create_dir_all(&readable).context("failed to create readable scratch dir")?;
        let _cleanup = Cleanup(base);

        match expect {
            Expectation::Enforced => {
                run_enforced_matrix(&writable, &forbidden, &readable, &echo1, &echo2)
            }
            Expectation::CannotCreate(expected) => {
                run_cannot_create(&writable, &forbidden, expected)
            }
        }
    }

    #[derive(Clone, Copy)]
    enum FsMode {
        Unrestricted,
        Restricted,
    }

    #[derive(Clone, Copy)]
    enum NetMode {
        Unrestricted,
        Blocked,
        Restricted,
    }

    impl FsMode {
        fn label(self) -> &'static str {
            match self {
                FsMode::Unrestricted => "fs=unrestricted",
                FsMode::Restricted => "fs=restricted",
            }
        }
    }

    impl NetMode {
        fn label(self) -> &'static str {
            match self {
                NetMode::Unrestricted => "net=unrestricted",
                NetMode::Blocked => "net=blocked",
                NetMode::Restricted => "net=restricted",
            }
        }
    }

    /// On the enforced machine, run every fs × network combination and verify
    /// that allowed operations succeed and blocked operations fail.
    fn run_enforced_matrix(
        writable: &Path,
        forbidden: &Path,
        readable: &Path,
        echo1: &str,
        echo2: &str,
    ) -> Result<()> {
        // Nested-virtualization quirks can leave a VM unable to set up user
        // namespaces even when we expected enforcement; skip rather than fail.
        let probe_policy = SandboxPolicy {
            fs: SandboxFsPolicy::Restricted {
                writable_paths: Vec::new(),
            },
            network: SandboxNetPolicy::Blocked,
        };
        if let Err(error) = Sandbox::can_create(&probe_policy, None) {
            println!(
                "[sandbox_test]: SKIP: this environment cannot enforce a bwrap sandbox ({error})"
            );
            return Ok(());
        }

        // Seed a host file outside every writable dir to prove reads are allowed
        // everywhere regardless of the write policy.
        let readable_file = readable.join("host-data.txt");
        std::fs::write(&readable_file, "host data").context("failed to seed readable file")?;

        let echo1_host = host_of(echo1);

        let mut checks = Checks::new();
        for fs in [FsMode::Unrestricted, FsMode::Restricted] {
            for net in [NetMode::Unrestricted, NetMode::Blocked, NetMode::Restricted] {
                run_combo(
                    &mut checks,
                    fs,
                    net,
                    writable,
                    forbidden,
                    &readable_file,
                    echo1,
                    echo2,
                    &echo1_host,
                )?;
            }
        }
        checks.finish()
    }

    #[allow(clippy::too_many_arguments)]
    fn run_combo(
        checks: &mut Checks,
        fs: FsMode,
        net: NetMode,
        writable: &Path,
        forbidden: &Path,
        readable_file: &Path,
        echo1: &str,
        echo2: &str,
        echo1_host: &str,
    ) -> Result<()> {
        let fs_policy = match fs {
            FsMode::Unrestricted => SandboxFsPolicy::Unrestricted,
            FsMode::Restricted => SandboxFsPolicy::Restricted {
                writable_paths: vec![writable.to_path_buf()],
            },
        };
        let net_policy = match net {
            NetMode::Unrestricted => SandboxNetPolicy::Unrestricted,
            NetMode::Blocked => SandboxNetPolicy::Blocked,
            NetMode::Restricted => SandboxNetPolicy::Restricted {
                allowed_domains: vec![echo1_host.to_string()],
            },
        };
        let mut sandbox = Sandbox::new(SandboxPolicy {
            fs: fs_policy,
            network: net_policy,
        })
        .with_context(|| {
            format!(
                "failed to create sandbox for {} {}",
                fs.label(),
                net.label()
            )
        })?;

        let scope = format!("[{} {}]", fs.label(), net.label());

        // Reads are always allowed (root is bound read-only).
        let read_host = run_command(
            &mut sandbox,
            "sh",
            &["-c", &format!("cat {}", shell_quote(readable_file))],
        )?;
        checks.check(
            &format!("{scope} read of a host file is allowed"),
            read_host,
        );

        // Writing into the writable dir: allowed under both fs modes (it's
        // either unrestricted, or explicitly granted).
        let writable_file = writable.join(format!("w-{}-{}.txt", fs.label(), net.label()));
        let _ = std::fs::remove_file(&writable_file);
        let wrote_writable = run_command(
            &mut sandbox,
            "sh",
            &["-c", &format!("echo zed > {}", shell_quote(&writable_file))],
        )?;
        checks.check(
            &format!("{scope} write into the writable dir is allowed"),
            wrote_writable && writable_file.exists(),
        );

        // Writing outside any writable dir: allowed only when fs is unrestricted.
        let forbidden_file = forbidden.join(format!("f-{}-{}.txt", fs.label(), net.label()));
        let _ = std::fs::remove_file(&forbidden_file);
        let wrote_forbidden = run_command(
            &mut sandbox,
            "sh",
            &[
                "-c",
                &format!("echo zed > {}", shell_quote(&forbidden_file)),
            ],
        )?;
        match fs {
            FsMode::Unrestricted => checks.check(
                &format!("{scope} write outside writable dirs is allowed"),
                wrote_forbidden && forbidden_file.exists(),
            ),
            FsMode::Restricted => checks.check(
                &format!("{scope} write outside writable dirs is blocked"),
                !wrote_forbidden && !forbidden_file.exists(),
            ),
        }

        // Network reachability of echo1 / echo2 depends only on the net policy.
        let exe = current_exe_str()?;
        let reach_echo1 = run_command(&mut sandbox, &exe, &[SUBCOMMAND_ECHO_CHECK, echo1])?;
        let reach_echo2 = run_command(&mut sandbox, &exe, &[SUBCOMMAND_ECHO_CHECK, echo2])?;
        match net {
            NetMode::Unrestricted => {
                checks.check(&format!("{scope} echo1 reachable"), reach_echo1);
                checks.check(&format!("{scope} echo2 reachable"), reach_echo2);
            }
            NetMode::Blocked => {
                checks.check(&format!("{scope} echo1 blocked"), !reach_echo1);
                checks.check(&format!("{scope} echo2 blocked"), !reach_echo2);
            }
            NetMode::Restricted => {
                checks.check(
                    &format!("{scope} allowed domain echo1 reachable"),
                    reach_echo1,
                );
                checks.check(
                    &format!("{scope} disallowed domain echo2 blocked"),
                    !reach_echo2,
                );
            }
        }

        Ok(())
    }

    /// On a degraded machine, `Sandbox::can_create` must report the specific
    /// failure for every policy, and we must fail closed (never run the command).
    fn run_cannot_create(writable: &Path, forbidden: &Path, expected: SandboxError) -> Result<()> {
        let mut checks = Checks::new();
        let policies = [
            SandboxPolicy {
                fs: SandboxFsPolicy::Unrestricted,
                network: SandboxNetPolicy::Unrestricted,
            },
            SandboxPolicy {
                fs: SandboxFsPolicy::Restricted {
                    writable_paths: vec![writable.to_path_buf()],
                },
                network: SandboxNetPolicy::Blocked,
            },
        ];

        for policy in policies {
            match Sandbox::can_create(&policy, None) {
                Ok(()) => checks.check(&format!("can_create reports {expected:?} (got Ok)"), false),
                Err(error) => checks.check(
                    &format!("can_create reports {expected:?} (got {error:?})"),
                    error == expected,
                ),
            }
        }

        // Fail-closed: a consumer that respects `can_create` never runs the
        // command, so the otherwise-forbidden write never happens.
        let forbidden_file = forbidden.join("degraded.txt");
        checks.check(
            "degraded machine performs no forbidden write",
            !forbidden_file.exists(),
        );

        checks.finish()
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

    /// The hostname portion of a `host:port` string.
    fn host_of(host_port: &str) -> String {
        host_port
            .rsplit_once(':')
            .map(|(host, _)| host.to_string())
            .unwrap_or_else(|| host_port.to_string())
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

    /// Removes a directory tree on drop, so scratch dirs don't pile up.
    struct Cleanup(PathBuf);

    impl Drop for Cleanup {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
}
