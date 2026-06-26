//! Behavior test helper for the Windows WSL Bubblewrap sandbox — the Windows
//! analog of `bwrap_test_helper`.
//!
//! Where the Linux helper is the sandboxed process itself (it re-execs under
//! the launcher), here the sandboxed process is a *Linux* program inside WSL
//! while this helper runs on Windows. So instead of a status channel and a
//! launcher, the helper drives the real [`sandbox::Sandbox`] (`new` + `wrap`),
//! spawns the command line it produces, and inspects exit codes and
//! host-side filesystem effects to confirm every grant the sandbox makes and
//! every restriction it imposes actually holds — including the Windows-specific
//! one: that a sandboxed process cannot escape via WSL interop by exec'ing a
//! Windows binary.
//!
//! It targets the **default** WSL distro (matching real Zed usage for native
//! Windows paths); provision that distro before running (see
//! `script/test-wsl-sandbox.ps1`). Like the Linux helper, it **skips** (rather
//! than fails) the enforcement assertions when the environment can't actually
//! enforce a sandbox, so a misconfigured WSL doesn't masquerade as a sandbox
//! regression. Set `ZED_TEST_SANDBOX_REQUIRE_ENFORCED=1` to turn that skip into
//! a failure once you've provisioned an environment that *should* enforce.
//!
//! Run it with `cargo xtask wsl-sandbox-tests` or `script/test-wsl-sandbox.ps1`.

#![allow(
    clippy::disallowed_methods,
    reason = "a single-threaded test helper that intentionally blocks on child processes"
)]

#[cfg(not(target_os = "windows"))]
fn main() {
    eprintln!("wsl_sandbox_test_helper is only supported on Windows");
    std::process::exit(1);
}

#[cfg(target_os = "windows")]
fn main() {
    imp::main();
}

#[cfg(target_os = "windows")]
mod imp {
    use std::collections::HashMap;
    use std::ffi::OsStr;
    use std::net::TcpListener;
    use std::os::windows::process::CommandExt as _;
    use std::path::{Path, PathBuf};
    use std::process::{Command, Output};

    use anyhow::{Context as _, Result, bail, ensure};
    use sandbox::{
        CommandAndArgs, GitSandboxPolicy, Sandbox, SandboxError, SandboxFsPolicy, SandboxNetPolicy,
        SandboxPolicy,
    };

    /// Network access for a helper run, translated into a `SandboxNetPolicy` in
    /// `drive_sandbox`. Only the all-or-nothing cases the helper exercises are
    /// represented.
    #[derive(Clone, Copy, Default)]
    enum NetworkAccess {
        #[default]
        None,
        All,
    }

    /// The per-run permission knobs the helper varies, translated into a
    /// `SandboxPolicy` in `drive_sandbox`.
    #[derive(Clone, Copy, Default)]
    struct SandboxPermissions {
        network: NetworkAccess,
        allow_fs_write: bool,
    }

    /// Tag prefixed to every result line, matching `bwrap_test_helper` so both
    /// helpers' output reads the same.
    const RESULT_TAG: &str = "[sandbox_test]:";

    /// `CREATE_NO_WINDOW`: keep `wsl.exe` (a console-subsystem binary) from
    /// flashing a console window when spawned.
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;

    pub fn main() {
        if let Err(error) = run() {
            eprintln!("{RESULT_TAG} FAILED: {error:#}");
            std::process::exit(1);
        }
    }

    fn run() -> Result<()> {
        let require_enforced = env_flag("ZED_TEST_SANDBOX_REQUIRE_ENFORCED");
        let wsl = Wsl::detect();
        println!("{RESULT_TAG} starting (require_enforced={require_enforced})");

        // `wrap_invocation` performs the real environment probe (locate `bwrap`,
        // reject setuid-root, smoke-test the exact namespaces) before it builds
        // any command. So a default-permissions run of `true` doubles as our
        // enforcement probe: `Ok` means the sandbox is enforceable here, an
        // `Unavailable` error means it is not.
        let probe = run_in_sandbox("true", &[], SandboxPermissions::default())?;
        match &probe {
            Outcome::Ran {
                command_succeeded: true,
                ..
            } => {}
            Outcome::Unavailable(message) => return not_enforced(require_enforced, message),
            other => {
                return not_enforced(
                    require_enforced,
                    &format!("the sandbox probe did not run cleanly: {other:?}"),
                );
            }
        }

        run_enforced(&wsl)
    }

    /// The environment can't enforce a sandbox. Skip the enforcement checks
    /// unless the caller asserted (via `ZED_TEST_SANDBOX_REQUIRE_ENFORCED`) that
    /// it should be able to, in which case this is a real failure.
    fn not_enforced(require_enforced: bool, reason: &str) -> Result<()> {
        if require_enforced {
            bail!(
                "ZED_TEST_SANDBOX_REQUIRE_ENFORCED is set, but the WSL sandbox could not be \
                 enforced: {reason}"
            );
        }
        println!(
            "{RESULT_TAG} SKIP: this environment cannot enforce a WSL bwrap sandbox: {reason}"
        );
        Ok(())
    }

    /// Enforced scenario: `bwrap` is present and a sandbox can be set up, so the
    /// sandbox must actually be enforced. Assert every grant and every
    /// restriction end-to-end against the real WSL distro.
    fn run_enforced(wsl: &Wsl) -> Result<()> {
        let mut checks = Checks::new();
        let pid = std::process::id();

        // The core filesystem checks use a scratch tree on the WSL distro's own
        // rootfs (under `/var/tmp`, which the sandbox leaves read-only rather
        // than overlaying like `/tmp`). This mirrors the Linux helper and is
        // robust: it doesn't depend on how drvfs `/mnt/<drive>` submounts behave
        // under bwrap's recursive root bind. The Windows-drive translation path
        // (the realistic Zed-on-`C:` case) gets its own dedicated check below.
        let root_base = format!("/var/tmp/zed-wsl-sandbox-test-{pid}");
        let writable_wsl = format!("{root_base}/writable");
        let forbidden_wsl = format!("{root_base}/forbidden");
        let readable_wsl = format!("{root_base}/readable");
        let mkdir = wsl.run_sh(&format!(
            "mkdir -p {} {} {}",
            shell_quote(&writable_wsl),
            shell_quote(&forbidden_wsl),
            shell_quote(&readable_wsl),
        ))?;
        ensure!(
            mkdir.status.success(),
            "failed to create the WSL scratch tree{}",
            failure_details(&mkdir)
        );
        let _root_cleanup = WslCleanup {
            exe: wsl.exe.clone(),
            path: root_base,
        };

        let default = SandboxPermissions::default();
        let fs_write_all = SandboxPermissions {
            network: NetworkAccess::None,
            allow_fs_write: true,
        };
        let network_allowed = SandboxPermissions {
            network: NetworkAccess::All,
            allow_fs_write: false,
        };

        // GRANT: writing into a writable bind succeeds and lands on the host.
        let writable_file = format!("{writable_wsl}/from-sandbox.txt");
        let write_writable = run_in_sandbox(
            &format!("echo zed > {}", shell_quote(&writable_file)),
            &[PathBuf::from(writable_wsl)],
            default,
        )?;
        checks.expect_succeeded("GRANT: write into a writable dir succeeds", &write_writable);
        checks.expect(
            "GRANT: write into a writable dir lands on the host",
            wsl.exists(&writable_file)?,
        );

        // RESTRICT: writing outside any writable bind is denied by the read-only
        // root, and must not leak to the host.
        let forbidden_file = format!("{forbidden_wsl}/escaped.txt");
        let write_forbidden = run_in_sandbox(
            &format!("echo zed > {}", shell_quote(&forbidden_file)),
            &[],
            default,
        )?;
        checks.expect_blocked(
            "RESTRICT: write outside writable dirs is denied",
            &write_forbidden,
        );
        checks.expect(
            "RESTRICT: denied write did not leak to the host",
            !wsl.exists(&forbidden_file)?,
        );

        // GRANT: the whole filesystem is readable (root is bound read-only), so
        // a host file outside every writable dir can still be read.
        let readable_file = format!("{readable_wsl}/host-data.txt");
        let seed = wsl.run_sh(&format!(
            "printf 'host data' > {}",
            shell_quote(&readable_file)
        ))?;
        ensure!(
            seed.status.success(),
            "failed to seed the readable file{}",
            failure_details(&seed)
        );
        let read_host = run_in_sandbox(
            &format!("cat {}", shell_quote(&readable_file)),
            &[],
            default,
        )?;
        checks.expect_succeeded(
            "GRANT: host files outside writable dirs are still readable",
            &read_host,
        );

        // GRANT + RESTRICT: `/tmp` is a writable tmpfs, but ephemeral — it must
        // not leak to the WSL distro's real `/tmp`.
        let tmp_path = format!("/tmp/zed-sandbox-ephemeral-{pid}");
        let write_tmp = run_in_sandbox(
            &format!("echo zed > {}", shell_quote(&tmp_path)),
            &[],
            default,
        )?;
        checks.expect_succeeded("GRANT: writing to /tmp succeeds", &write_tmp);
        checks.expect(
            "RESTRICT: /tmp writes are ephemeral (do not leak to the WSL host /tmp)",
            !wsl.exists(&tmp_path)?,
        );

        // RESTRICT + GRANT: outbound TCP is denied by the network namespace, but
        // works when network access is explicitly granted. We discover a peer
        // reachable from WSL first; that same reachability check proves the
        // denial below is the sandbox's doing.
        match discover_peer(wsl)? {
            Some(peer) => {
                let connect = connect_script(&peer);
                let net_denied = run_in_sandbox(&connect, &[], default)?;
                checks.expect_blocked(
                    "RESTRICT: outbound TCP is blocked when network is denied",
                    &net_denied,
                );
                let net_allowed = run_in_sandbox(&connect, &[], network_allowed)?;
                checks.expect_succeeded(
                    "GRANT: outbound TCP works when network is allowed",
                    &net_allowed,
                );
                // RESTRICT: permissions are independent — granting filesystem
                // writes must not also grant network access.
                let net_with_fs_write = run_in_sandbox(&connect, &[], fs_write_all)?;
                checks.expect_blocked(
                    "RESTRICT: allow_fs_write does not also grant network access",
                    &net_with_fs_write,
                );
            }
            None => println!(
                "{RESULT_TAG} SKIP: no TCP peer reachable from WSL; skipping network checks"
            ),
        }

        // GRANT: local AF_UNIX IPC keeps working even while IP networking is
        // denied. Needs python3 to create the socket; skip if it isn't present.
        if wsl.has_program("python3")? {
            let unix_ok = run_in_sandbox(
                "python3 -c 'import socket; socket.socket(socket.AF_UNIX, socket.SOCK_DGRAM)'",
                &[],
                default,
            )?;
            checks.expect_succeeded(
                "GRANT: AF_UNIX sockets still work while network is denied",
                &unix_ok,
            );
        } else {
            println!("{RESULT_TAG} SKIP: no python3 in WSL; skipping AF_UNIX check");
        }

        // GRANT (escape hatch): `allow_fs_write` lets the command write
        // anywhere, and the write reaches the host.
        let escape_file = format!("{forbidden_wsl}/escape-hatch.txt");
        let write_escape = run_in_sandbox(
            &format!("echo zed > {}", shell_quote(&escape_file)),
            &[],
            fs_write_all,
        )?;
        checks.expect_succeeded(
            "GRANT: allow_fs_write lets the command write outside writable dirs",
            &write_escape,
        );
        checks.expect(
            "GRANT: allow_fs_write write lands on the host",
            wsl.exists(&escape_file)?,
        );

        // GRANT + RESTRICT (Windows-specific): a writable directory given as a
        // native `C:\` path is translated into WSL and bound read-write, and the
        // write lands back on the Windows filesystem.
        check_windows_drive_writable(wsl, &mut checks)?;

        // RESTRICT (Windows-specific): WSL interop must be blocked, so a
        // sandboxed process can't exec a Windows binary and escape bwrap.
        check_interop_blocked(wsl, &mut checks)?;

        // GRANT + RESTRICT: the caller's environment is forwarded into the
        // command, but Windows-specific values like PATH are not (which would
        // otherwise shadow WSL's PATH and break the shell).
        check_env_forwarding(&mut checks)?;

        // Degraded (bad request): a non-existent writable path is the model's
        // mistake, not a broken sandbox environment, so it must be reported
        // *without* the "sandboxing is unavailable" marker (which would wrongly
        // prompt the user to disable sandboxing globally).
        let missing = std::env::temp_dir().join(format!("zed-wsl-missing-{pid}"));
        let bad_request = drive_sandbox(
            "true",
            &[],
            std::slice::from_ref(&missing),
            default,
            &HashMap::new(),
        )?;
        checks.expect(
            "a non-existent writable path is a bad request, not an unavailable-environment error",
            matches!(bad_request, Outcome::BadRequest(_)),
        );
        if !matches!(bad_request, Outcome::BadRequest(_)) {
            println!("{RESULT_TAG}   (got {bad_request:?})");
        }

        checks.finish()
    }

    /// Windows-specific GRANT: a writable directory passed as a native `C:\`
    /// path is translated into WSL with `wslpath`, bound read-write, and a write
    /// inside the sandbox lands back on the Windows filesystem. Exercises the
    /// native-drive path translation end-to-end (the realistic case of Zed on
    /// Windows sandboxing a command in a project under `C:\`).
    fn check_windows_drive_writable(wsl: &Wsl, checks: &mut Checks) -> Result<()> {
        let base =
            std::env::temp_dir().join(format!("zed-wsl-sandbox-drive-{}", std::process::id()));
        let writable = base.join("writable");
        std::fs::create_dir_all(&writable)
            .with_context(|| format!("failed to create scratch dir `{}`", writable.display()))?;
        let _cleanup = Cleanup(base);

        let mapped = wsl.wsl_paths(&[&writable]).context(
            "failed to translate the scratch dir into a WSL path (is the C: drive automounted?)",
        )?;
        let Some(writable_wsl) = mapped.into_iter().next() else {
            bail!("wslpath returned no result for the scratch dir");
        };

        let write = run_in_sandbox(
            &format!(
                "echo zed > {}",
                shell_quote(&format!("{writable_wsl}/from-sandbox.txt"))
            ),
            std::slice::from_ref(&writable),
            SandboxPermissions::default(),
        )?;
        checks.expect_succeeded(
            "GRANT: write into a writable C:\\ dir succeeds (native path translated into WSL)",
            &write,
        );
        checks.expect(
            "GRANT: write into a writable C:\\ dir lands on the Windows host",
            writable.join("from-sandbox.txt").exists(),
        );
        Ok(())
    }

    /// Assert that a sandboxed process cannot reach the Windows host through WSL
    /// interop. Without the sandbox's interop block, a command could exec a
    /// Windows binary (e.g. `cmd.exe`), which the WSL binfmt handler runs on the
    /// host — fully outside bwrap.
    fn check_interop_blocked(wsl: &Wsl, checks: &mut Checks) -> Result<()> {
        // `$WSL_INTEROP` must be unset inside the sandbox (the variable `/init`
        // uses to find the interop socket).
        let interop_env = run_in_sandbox(
            "[ -z \"$WSL_INTEROP\" ]",
            &[],
            SandboxPermissions::default(),
        )?;
        checks.expect_succeeded(
            "RESTRICT: $WSL_INTEROP is unset inside the sandbox",
            &interop_env,
        );

        // Resolve cmd.exe's path inside WSL; skip the exec check if we can't
        // (e.g. a non-standard automount root).
        let cmd = match wsl.wsl_paths(&[Path::new(r"C:\Windows\System32\cmd.exe")]) {
            Ok(mut paths) => paths.pop(),
            Err(_) => None,
        };
        let Some(cmd) = cmd else {
            println!(
                "{RESULT_TAG} SKIP: could not resolve cmd.exe inside WSL; skipping interop exec check"
            );
            return Ok(());
        };

        // Control: unsandboxed, interop should let WSL exec a Windows binary. If
        // even this fails, interop isn't available here, so the sandboxed denial
        // below would prove nothing — skip.
        let control = wsl.run(&cmd, ["/C", "exit"])?;
        if !control.status.success() {
            println!(
                "{RESULT_TAG} SKIP: WSL interop is unavailable in this environment (the unsandboxed \
                 control run failed); skipping interop exec check"
            );
            return Ok(());
        }

        // Sandboxed: exec'ing the same Windows binary must fail, because interop
        // is blocked.
        let escape = run_in_sandbox(
            &format!("{} /C exit", shell_quote(&cmd)),
            &[],
            SandboxPermissions::default(),
        )?;
        checks.expect_blocked(
            "RESTRICT: cannot exec a Windows binary via WSL interop (sandbox escape blocked)",
            &escape,
        );
        Ok(())
    }

    /// Assert the caller's environment is forwarded into the sandbox, while
    /// Windows-specific values like PATH are dropped rather than overriding
    /// WSL's own.
    fn check_env_forwarding(checks: &mut Checks) -> Result<()> {
        let mut env = HashMap::new();
        env.insert("ZED_TEST_FORWARDED".to_string(), "yes".to_string());
        // If PATH were forwarded it would replace WSL's PATH with this bogus
        // value; it must not be.
        env.insert(
            "PATH".to_string(),
            "/zed-sentinel-should-not-win".to_string(),
        );
        let outcome = drive_sandbox(
            "/bin/sh",
            &[
                "-c",
                "[ \"$ZED_TEST_FORWARDED\" = yes ] && [ \"$PATH\" != /zed-sentinel-should-not-win ]",
            ],
            &[],
            SandboxPermissions::default(),
            &env,
        )?;
        checks.expect_succeeded(
            "GRANT: caller env is forwarded into the sandbox; RESTRICT: PATH is not overridden",
            &outcome,
        );
        Ok(())
    }

    /// The outcome of asking the sandbox to run a command.
    #[derive(Debug)]
    enum Outcome {
        /// `wrap_invocation` succeeded and the wrapped `wsl.exe` command ran;
        /// `command_succeeded` is its exit success.
        Ran {
            command_succeeded: bool,
            stdout: String,
            stderr: String,
        },
        /// `wrap_invocation` reported the sandbox *environment* as unavailable
        /// (carried the shared unavailable-prefix marker).
        Unavailable(String),
        /// `wrap_invocation` reported a bad request (a mappable-path / distro
        /// problem); no unavailable-prefix marker.
        BadRequest(String),
    }

    /// Run `/bin/sh -c <script>` under the sandbox with the given writable paths
    /// and permissions.
    fn run_in_sandbox(
        script: &str,
        writable_paths: &[PathBuf],
        permissions: SandboxPermissions,
    ) -> Result<Outcome> {
        drive_sandbox(
            "/bin/sh",
            &["-c", script],
            writable_paths,
            permissions,
            &HashMap::new(),
        )
    }

    /// Drive the real sandbox the way Zed's terminal integration does: wrap the
    /// invocation, then spawn the resulting `wsl.exe` command and collect its
    /// result. `wrap_invocation` errors are classified into [`Outcome`] by the
    /// shared unavailable-prefix marker rather than bubbling up, so callers can
    /// assert on the classification.
    fn drive_sandbox(
        program: &str,
        args: &[&str],
        writable_paths: &[PathBuf],
        permissions: SandboxPermissions,
        env: &HashMap<String, String>,
    ) -> Result<Outcome> {
        let policy = SandboxPolicy {
            fs: if permissions.allow_fs_write {
                SandboxFsPolicy::Unrestricted
            } else {
                SandboxFsPolicy::Restricted {
                    writable_paths: writable_paths.to_vec(),
                }
            },
            network: match permissions.network {
                NetworkAccess::None => SandboxNetPolicy::Blocked,
                NetworkAccess::All => SandboxNetPolicy::Unrestricted,
            },
            git: GitSandboxPolicy::default(),
        };
        let command = CommandAndArgs {
            program: program.to_string(),
            args: args.iter().map(|arg| arg.to_string()).collect(),
            env: env.clone(),
            cwd: None,
        };
        let prepared =
            Sandbox::new(policy).and_then(|mut sandbox| smol::block_on(sandbox.wrap(&command)));

        let prepared = match prepared {
            Ok(prepared) => prepared,
            Err(error) => {
                let message = error.to_string();
                return Ok(match error {
                    SandboxError::WslUnavailable(_) => Outcome::Unavailable(message),
                    _ => Outcome::BadRequest(message),
                });
            }
        };

        let output = Command::new(&prepared.program)
            .args(&prepared.args)
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .context("failed to spawn the wrapped wsl.exe sandbox command")?;
        Ok(Outcome::Ran {
            command_succeeded: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }

    /// Find a TCP peer reachable from inside WSL so the network checks have a
    /// real endpoint. Honors `ZED_TEST_ECHO_ADDR` (a `host:port`) if set;
    /// otherwise binds a local listener on the Windows side and finds an address
    /// WSL can use to reach the Windows host. Returns `None` (so the caller
    /// skips, rather than fails, the network checks) when nothing is reachable.
    fn discover_peer(wsl: &Wsl) -> Result<Option<String>> {
        if let Some(address) = std::env::var("ZED_TEST_ECHO_ADDR")
            .ok()
            .filter(|address| !address.is_empty())
        {
            ensure_host_port(&address)?;
            return Ok(Some(address));
        }

        // A listener on the Windows side that accepts and immediately drops
        // connections, so an in-WSL connect succeeds. It lives for the rest of
        // the process (the thread owns it and never closes it).
        let listener =
            TcpListener::bind(("0.0.0.0", 0)).context("failed to bind a TCP listener")?;
        let port = listener
            .local_addr()
            .context("local listener has no address")?
            .port();
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => drop(stream),
                    Err(_) => break,
                }
            }
        });

        // Addresses WSL might use to reach the Windows host: the default
        // gateway (WSL2 NAT mode) and loopback (mirrored networking mode).
        let mut candidates = Vec::new();
        if let Ok(output) =
            wsl.run_sh("ip route show default 2>/dev/null | awk '/default/{print $3; exit}'")
        {
            let gateway = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !gateway.is_empty() {
                candidates.push(gateway);
            }
        }
        candidates.push("127.0.0.1".to_string());

        for host in candidates {
            let address = format!("{host}:{port}");
            if wsl.run_sh(&connect_script(&address))?.status.success() {
                return Ok(Some(address));
            }
        }
        Ok(None)
    }

    /// A shell snippet that attempts an outbound TCP connection to `host:port`,
    /// exiting 0 on success. Uses bash's `/dev/tcp` (the most widely available
    /// option in default WSL distros) under `timeout` so a blocked connect can't
    /// hang. If bash or timeout is missing the snippet simply fails, which the
    /// caller treats as "unreachable" and skips.
    fn connect_script(address: &str) -> String {
        format!(
            "timeout 5 bash -c 'exec 3<>/dev/tcp/{}'",
            address.replace(':', "/")
        )
    }

    fn ensure_host_port(address: &str) -> Result<()> {
        let (host, port) = address
            .rsplit_once(':')
            .with_context(|| format!("ZED_TEST_ECHO_ADDR must be host:port, got {address:?}"))?;
        ensure!(
            !host.is_empty() && port.parse::<u16>().is_ok(),
            "ZED_TEST_ECHO_ADDR must be host:port, got {address:?}"
        );
        Ok(())
    }

    /// Thin wrapper over `wsl.exe` for the helper's own (unsandboxed) WSL calls:
    /// path translation, the interop control run, and peer discovery. Targets
    /// the default distro, matching `wrap_invocation`.
    struct Wsl {
        exe: PathBuf,
    }

    impl Wsl {
        fn detect() -> Self {
            let exe = std::env::var_os("SystemRoot")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(r"C:\Windows"))
                .join("System32")
                .join("wsl.exe");
            Self { exe }
        }

        /// Run a Linux `program` (with `args`) unsandboxed via `wsl.exe --exec`.
        fn run<I, S>(&self, program: &str, args: I) -> Result<Output>
        where
            I: IntoIterator<Item = S>,
            S: AsRef<OsStr>,
        {
            Command::new(&self.exe)
                .arg("--exec")
                .arg(program)
                .args(args)
                .creation_flags(CREATE_NO_WINDOW)
                .output()
                .with_context(|| format!("failed to run `wsl.exe --exec {program}`"))
        }

        /// Run `/bin/sh -c <script>` unsandboxed.
        fn run_sh(&self, script: &str) -> Result<Output> {
            self.run("/bin/sh", ["-c", script])
        }

        /// Whether `name` resolves to a program inside WSL.
        fn has_program(&self, name: &str) -> Result<bool> {
            Ok(self.run_sh(&format!("command -v {name}"))?.status.success())
        }

        /// Whether `linux_path` exists in the (unsandboxed) WSL distro — used to
        /// confirm that sandboxed writes did or didn't reach the host.
        fn exists(&self, linux_path: &str) -> Result<bool> {
            Ok(self
                .run_sh(&format!("[ -e {} ]", shell_quote(linux_path)))?
                .status
                .success())
        }

        /// Translate Windows paths to their WSL paths with `wslpath -u`, in one
        /// round-trip. Paths are passed as argv (not interpolated) so quoting is
        /// never a concern.
        fn wsl_paths(&self, paths: &[&Path]) -> Result<Vec<String>> {
            let mut args: Vec<std::ffi::OsString> = vec![
                "-c".into(),
                "for path; do wslpath -u \"$path\" || exit 9; done".into(),
                "zed-wslpath".into(),
            ];
            args.extend(paths.iter().map(|path| path.as_os_str().to_os_string()));

            let output = self.run("/bin/sh", args)?;
            ensure!(
                output.status.success(),
                "wslpath translation failed{}",
                failure_details(&output)
            );
            let stdout = String::from_utf8_lossy(&output.stdout);
            let resolved: Vec<String> = stdout
                .lines()
                .map(|line| line.trim_end_matches('\r').to_string())
                .collect();
            ensure!(
                resolved.len() == paths.len(),
                "expected {} wslpath results, got {}: {stdout:?}",
                paths.len(),
                resolved.len()
            );
            Ok(resolved)
        }
    }

    /// Tracks assertion results so the helper can report a summary and a single
    /// exit code, mirroring `bwrap_test_helper`.
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

        fn expect(&mut self, description: &str, condition: bool) {
            if condition {
                self.passed += 1;
                println!("{RESULT_TAG} PASS: {description}");
            } else {
                self.failed += 1;
                println!("{RESULT_TAG} FAIL: {description}");
            }
        }

        /// Expect the wrapped command to have run and succeeded.
        fn expect_succeeded(&mut self, description: &str, outcome: &Outcome) {
            let ok = matches!(
                outcome,
                Outcome::Ran {
                    command_succeeded: true,
                    ..
                }
            );
            self.report(description, ok, outcome);
        }

        /// Expect the wrapped command to have run but been blocked (non-zero
        /// exit) — the sandbox imposed a restriction.
        fn expect_blocked(&mut self, description: &str, outcome: &Outcome) {
            let ok = matches!(
                outcome,
                Outcome::Ran {
                    command_succeeded: false,
                    ..
                }
            );
            self.report(description, ok, outcome);
        }

        fn report(&mut self, description: &str, ok: bool, outcome: &Outcome) {
            if ok {
                self.passed += 1;
                println!("{RESULT_TAG} PASS: {description}");
            } else {
                self.failed += 1;
                println!("{RESULT_TAG} FAIL: {description}");
                match outcome {
                    Outcome::Ran {
                        command_succeeded,
                        stdout,
                        stderr,
                    } => println!(
                        "{RESULT_TAG}   (command {}, stdout: {:?}, stderr: {:?})",
                        if *command_succeeded {
                            "succeeded"
                        } else {
                            "failed"
                        },
                        truncate(stdout),
                        truncate(stderr),
                    ),
                    Outcome::Unavailable(message) => {
                        println!("{RESULT_TAG}   (sandbox reported unavailable: {message})");
                    }
                    Outcome::BadRequest(message) => {
                        println!("{RESULT_TAG}   (sandbox reported bad request: {message})");
                    }
                }
            }
        }

        fn finish(self) -> Result<()> {
            println!(
                "{RESULT_TAG} summary: {} passed, {} failed",
                self.passed, self.failed
            );
            if self.failed > 0 {
                bail!("{} sandbox assertion(s) failed", self.failed);
            }
            Ok(())
        }
    }

    fn failure_details(output: &Output) -> String {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stderr = stderr.trim();
        let status = match output.status.code() {
            Some(code) => format!("exit code {code}"),
            None => "terminated by signal".to_string(),
        };
        if stderr.is_empty() {
            format!(" ({status})")
        } else {
            format!(" ({status}; stderr: {stderr})")
        }
    }

    /// Single-quote a value for safe interpolation into an `sh -c` script.
    fn shell_quote(value: &str) -> String {
        format!("'{}'", value.replace('\'', "'\\''"))
    }

    fn truncate(value: &str) -> String {
        const LIMIT: usize = 200;
        let trimmed = value.trim();
        match trimmed.char_indices().nth(LIMIT) {
            // `index` is a char boundary, so this slice can't split a code point.
            Some((index, _)) => format!("{}…", &trimmed[..index]),
            None => trimmed.to_string(),
        }
    }

    fn env_flag(name: &str) -> bool {
        std::env::var(name)
            .map(|value| value == "1")
            .unwrap_or(false)
    }

    /// Removes the Windows-side scratch tree on drop, so it doesn't pile up
    /// across runs.
    struct Cleanup(PathBuf);

    impl Drop for Cleanup {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// Removes a WSL-side scratch tree on drop (best effort).
    struct WslCleanup {
        exe: PathBuf,
        path: String,
    }

    impl Drop for WslCleanup {
        fn drop(&mut self) {
            let _ = Command::new(&self.exe)
                .arg("--exec")
                .arg("rm")
                .arg("-rf")
                .arg(&self.path)
                .creation_flags(CREATE_NO_WINDOW)
                .status();
        }
    }
}
