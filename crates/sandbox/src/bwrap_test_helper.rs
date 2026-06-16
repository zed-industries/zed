//! Test binary used exclusively in the NixOS integration test for bwrap at
//! `nix/tests/sandboxing`. See the comment there.

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
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::time::Duration;

    use anyhow::{Context as _, Result, bail};
    use sandbox::SandboxPermissions;
    use sandbox::linux_bubblewrap::{LauncherStatus, StatusChannel, wrap_invocation};

    /// Internal subcommand: attempt an outbound TCP connection to the given
    /// `host:port`, exiting 0 on success and 1 on any failure. Run *inside* the
    /// sandbox to check whether the network is reachable.
    const SUBCOMMAND_NET_CHECK: &str = "__net_check";
    /// Internal subcommand: create an `AF_UNIX` socket, exiting 0 on success. Used
    /// to confirm local IPC still works while IP networking is denied.
    const SUBCOMMAND_UNIX_CHECK: &str = "__unix_check";

    pub fn main() {
        // If we were re-exec'd as the sandbox launcher, this sets up the sandbox and
        // execs the wrapped command without returning.
        sandbox::run_sandbox_launcher_if_invoked();

        let args: Vec<String> = std::env::args().collect();
        let result = match args.get(1).map(String::as_str) {
            Some(SUBCOMMAND_NET_CHECK) => run_net_check(args.get(2).map(String::as_str)),
            Some(SUBCOMMAND_UNIX_CHECK) => run_unix_check(),
            _ => run_tests(),
        };

        if let Err(error) = result {
            eprintln!("[sandbox_test]: FAILED: {error:#}");
            std::process::exit(1);
        }
    }

    /// Inner command: try to open a TCP connection. Inside the sandbox's own
    /// network namespace there is no route out, so this exits non-zero.
    ///
    /// A host like `echo:7000` can resolve to several addresses (e.g. IPv6 and
    /// IPv4); we must try all of them and only fail if none connect. Trying just
    /// the first would spuriously "fail" the allowed-network case whenever the
    /// first address happens to be one with no route (commonly IPv6).
    fn run_net_check(address: Option<&str>) -> Result<()> {
        use std::net::ToSocketAddrs as _;
        let address = address.context("net check requires a host:port argument")?;
        let resolved = address
            .to_socket_addrs()
            .with_context(|| format!("failed to resolve {address:?}"))?;
        let mut last_error = None;
        for socket_address in resolved {
            match std::net::TcpStream::connect_timeout(&socket_address, Duration::from_secs(5)) {
                Ok(_) => return Ok(()),
                Err(error) => last_error = Some(error),
            }
        }
        match last_error {
            Some(error) => Err(error).with_context(|| format!("failed to connect to {address}")),
            None => bail!("{address:?} resolved to no socket addresses"),
        }
    }

    /// Inner command: create an `AF_UNIX` socket. Local IPC is not categorically
    /// blocked by the sandbox, so this must succeed even when IP networking is
    /// denied.
    fn run_unix_check() -> Result<()> {
        std::os::unix::net::UnixDatagram::unbound().context("failed to create AF_UNIX socket")?;
        Ok(())
    }

    /// The outcome of driving the launcher once.
    struct LaunchResult {
        /// The status the launcher reported back, if any.
        status: Option<LauncherStatus>,
        /// Whether the launched command exited successfully.
        command_succeeded: bool,
    }

    /// Drive the sandbox launcher the same way Zed's terminal integration does:
    /// bind a status channel, build the launcher invocation, spawn it, and collect
    /// both the reported status and the command's exit result.
    fn drive_launcher(
        program: &str,
        args: &[String],
        writable_dirs: &[&Path],
        cwd: Option<&Path>,
        permissions: SandboxPermissions,
    ) -> Result<LaunchResult> {
        let launcher = std::env::current_exe().context("failed to resolve current executable")?;
        let launcher = launcher
            .to_str()
            .context("current executable path is not valid UTF-8")?;

        let channel = StatusChannel::bind().context("failed to bind status channel")?;
        let (launcher_program, launcher_args) = wrap_invocation(
            launcher,
            Some(channel.name()),
            permissions,
            writable_dirs,
            cwd,
            program,
            args,
        );

        let mut child = Command::new(&launcher_program)
            .args(&launcher_args)
            .spawn()
            .context("failed to spawn sandbox launcher")?;

        // The launcher connects and reports before it execs, so read the status
        // while the command runs, then wait for the command to finish.
        let status = channel.recv(Duration::from_secs(30));
        let exit = child.wait().context("failed to wait for launcher")?;

        Ok(LaunchResult {
            status,
            command_succeeded: exit.success(),
        })
    }

    /// Run `sh -c <script>` inside the sandbox and report whether it succeeded.
    fn run_in_sandbox(
        script: &str,
        writable_dirs: &[&Path],
        permissions: SandboxPermissions,
    ) -> Result<LaunchResult> {
        drive_launcher(
            "sh",
            &["-c".to_string(), script.to_string()],
            writable_dirs,
            None,
            permissions,
        )
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

    fn run_tests() -> Result<()> {
        let expect_enforced = std::env::var("ZED_TEST_SANDBOX_ENFORCED").unwrap_or_default() == "1";
        let echo_address = std::env::var("ZED_TEST_ECHO_ADDR").ok();
        let expected_degrade_status = std::env::var("ZED_TEST_EXPECTED_DEGRADE_STATUS").ok();

        println!(
            "[sandbox_test]: starting (expect_enforced={expect_enforced}, echo={echo_address:?}, \
             expected_degrade={expected_degrade_status:?})"
        );

        // A host scratch tree the sandbox tests write into. Kept outside `/tmp`
        // because the sandbox overlays a fresh tmpfs there, which would mask
        // whether writes really hit (or were blocked from hitting) the host.
        let base = PathBuf::from(format!("/sandbox-test-{}", std::process::id()));
        let writable = base.join("writable");
        let forbidden = base.join("forbidden");
        let readable = base.join("readable");
        std::fs::create_dir_all(&writable).context("failed to create writable scratch dir")?;
        std::fs::create_dir_all(&forbidden).context("failed to create forbidden scratch dir")?;
        std::fs::create_dir_all(&readable).context("failed to create readable scratch dir")?;
        let _cleanup = Cleanup(base);

        if expect_enforced {
            run_enforced_tests(&writable, &forbidden, &readable, echo_address.as_deref())
        } else {
            // Outside the enforced scenario, the sandbox could not be set up. The
            // scenario tells us *why* so we can assert the exact reported status.
            let expected = match expected_degrade_status.as_deref() {
                Some("bwrap_not_found") => LauncherStatus::BwrapNotFound,
                Some("probe_failed") => LauncherStatus::SandboxProbeFailed,
                other => bail!(
                    "ZED_TEST_EXPECTED_DEGRADE_STATUS must be `bwrap_not_found` or \
                     `probe_failed`, got {other:?}"
                ),
            };
            run_degraded_tests(&forbidden, expected)
        }
    }

    /// Enforced scenario: unprivileged user namespaces are available and `bwrap`
    /// is present, so the sandbox must actually be enforced. We assert, for every
    /// access the sandbox is supposed to *grant*, that it really is granted, and
    /// for every restriction it is supposed to *impose*, that it really holds.
    fn run_enforced_tests(
        writable: &Path,
        forbidden: &Path,
        readable: &Path,
        echo_address: Option<&str>,
    ) -> Result<()> {
        // First confirm this environment can enforce a sandbox at all. Nested
        // virtualization quirks can leave a VM unable to set up user namespaces
        // even when we expected it to; in that case skip (rather than fail) the
        // enforcement assertions, which would otherwise be testing nothing.
        let probe = drive_launcher("true", &[], &[], None, SandboxPermissions::default())?;
        if probe.status != Some(LauncherStatus::Success) {
            println!(
                "[sandbox_test]: SKIP: this environment cannot enforce a bwrap sandbox \
             (probe status: {:?})",
                probe.status
            );
            return Ok(());
        }

        let default = SandboxPermissions::default();
        let fs_write_all = SandboxPermissions {
            allow_network: false,
            allow_fs_write: true,
        };
        let network_allowed = SandboxPermissions {
            allow_network: true,
            allow_fs_write: false,
        };
        let mut checks = Checks::new();

        // GRANT: writing into a writable bind succeeds and lands on the host file.
        let writable_file = writable.join("from-sandbox.txt");
        let write_writable = run_in_sandbox(
            &format!("echo zed > {}", shell_quote(&writable_file)),
            &[writable],
            default,
        )?;
        checks.check(
            "GRANT: write into a writable dir succeeds",
            write_writable.command_succeeded,
        );
        checks.check(
            "GRANT: write into a writable dir lands on the host",
            writable_file.exists(),
        );
        checks.check(
            "default run reports Success",
            write_writable.status == Some(LauncherStatus::Success),
        );

        // RESTRICT: writing outside any writable bind is denied by the read-only
        // root, and must not leak to the host.
        let forbidden_file = forbidden.join("escaped.txt");
        let write_forbidden = run_in_sandbox(
            &format!("echo zed > {}", shell_quote(&forbidden_file)),
            &[],
            default,
        )?;
        checks.check(
            "RESTRICT: write outside writable dirs is denied",
            !write_forbidden.command_succeeded,
        );
        checks.check(
            "RESTRICT: denied write did not leak to the host",
            !forbidden_file.exists(),
        );

        // GRANT: the whole filesystem is readable (root is bound read-only), so a
        // host file outside every writable dir can still be read.
        let readable_file = readable.join("host-data.txt");
        std::fs::write(&readable_file, "host data").context("failed to seed readable file")?;
        let read_host = run_in_sandbox(
            &format!("cat {}", shell_quote(&readable_file)),
            &[],
            default,
        )?;
        checks.check(
            "GRANT: host files outside writable dirs are still readable",
            read_host.command_succeeded,
        );

        // GRANT + RESTRICT: `/tmp` is a writable tmpfs, but it is ephemeral and
        // must not leak to the host's real `/tmp`.
        let tmp_path = PathBuf::from(format!("/tmp/zed-sandbox-ephemeral-{}", std::process::id()));
        let write_tmp = run_in_sandbox(
            &format!("echo zed > {}", shell_quote(&tmp_path)),
            &[],
            default,
        )?;
        checks.check(
            "GRANT: writing to /tmp succeeds",
            write_tmp.command_succeeded,
        );
        checks.check(
            "RESTRICT: /tmp writes are ephemeral (do not leak to the host)",
            !tmp_path.exists(),
        );

        // RESTRICT + GRANT: outbound TCP is denied by the network namespace, but
        // works when network access is explicitly granted (the latter also proves
        // the peer is reachable, so the denial is the sandbox's doing).
        if let Some(echo_address) = echo_address {
            let net_denied = drive_launcher(
                &current_exe_str()?,
                &[SUBCOMMAND_NET_CHECK.to_string(), echo_address.to_string()],
                &[],
                None,
                default,
            )?;
            checks.check(
                "RESTRICT: outbound TCP is blocked when network is denied",
                !net_denied.command_succeeded,
            );

            let net_allowed = drive_launcher(
                &current_exe_str()?,
                &[SUBCOMMAND_NET_CHECK.to_string(), echo_address.to_string()],
                &[],
                None,
                network_allowed,
            )?;
            checks.check(
                "GRANT: outbound TCP works when network is allowed",
                net_allowed.command_succeeded,
            );
        } else {
            println!("[sandbox_test]: SKIP: no ZED_TEST_ECHO_ADDR; skipping network checks");
        }

        // GRANT: local AF_UNIX IPC keeps working even while IP networking is
        // denied.
        let unix_ok = drive_launcher(
            &current_exe_str()?,
            &[SUBCOMMAND_UNIX_CHECK.to_string()],
            &[],
            None,
            default,
        )?;
        checks.check(
            "GRANT: AF_UNIX sockets still work while network is denied",
            unix_ok.command_succeeded,
        );

        // GRANT (escape hatch): `allow_fs_write` lets the command write anywhere,
        // and the write reaches the host.
        let escape_file = forbidden.join("escape-hatch.txt");
        let write_escape = run_in_sandbox(
            &format!("echo zed > {}", shell_quote(&escape_file)),
            &[],
            fs_write_all,
        )?;
        checks.check(
            "GRANT: allow_fs_write lets the command write outside writable dirs",
            write_escape.command_succeeded,
        );
        checks.check(
            "GRANT: allow_fs_write write lands on the host",
            escape_file.exists(),
        );
        checks.check(
            "allow_fs_write run reports Success",
            write_escape.status == Some(LauncherStatus::Success),
        );

        // RESTRICT: permissions are independent — granting filesystem writes must
        // not also grant network access.
        if let Some(echo_address) = echo_address {
            let net_with_fs_write = drive_launcher(
                &current_exe_str()?,
                &[SUBCOMMAND_NET_CHECK.to_string(), echo_address.to_string()],
                &[],
                None,
                fs_write_all,
            )?;
            checks.check(
                "RESTRICT: allow_fs_write does not also grant network access",
                !net_with_fs_write.command_succeeded,
            );
        }

        // RESTRICT: a setuid-root bwrap must be refused, and the launcher must
        // abort (not run the command). We run as root in the VM, so we can build
        // one.
        check_setuid_rejected(&mut checks)?;

        checks.finish()
    }

    /// Degraded scenario: the sandbox could not be set up (user namespaces
    /// disabled, or no `bwrap` present). The launcher must report the specific
    /// reason *and abort* — it never runs the command unsandboxed. (Falling back
    /// to an unsandboxed run is the consumer's choice; the launcher, and these
    /// tests, fail closed.)
    fn run_degraded_tests(forbidden: &Path, expected_status: LauncherStatus) -> Result<()> {
        let mut checks = Checks::new();

        let forbidden_file = forbidden.join("degraded.txt");
        let result = run_in_sandbox(
            &format!("echo zed > {}", shell_quote(&forbidden_file)),
            &[],
            SandboxPermissions::default(),
        )?;

        checks.check(
            &format!("reports {expected_status:?} when the sandbox can't be created"),
            result.status == Some(expected_status),
        );
        // Fail closed: the launcher aborted, so the command did not run and the
        // otherwise-forbidden write never happened.
        checks.check(
            "command does not run when the sandbox can't be created (fail-closed)",
            !result.command_succeeded && !forbidden_file.exists(),
        );

        checks.finish()
    }

    /// Build a setuid-root copy of `bwrap`, put it alone on `PATH`, and assert the
    /// launcher refuses to run it (reporting `SetuidRejected`) and aborts without
    /// running the command.
    fn check_setuid_rejected(checks: &mut Checks) -> Result<()> {
        let Some(real_bwrap) = find_on_path("bwrap") else {
            println!("[sandbox_test]: SKIP: no bwrap on PATH; skipping setuid rejection check");
            return Ok(());
        };

        let dir = PathBuf::from(format!("/sandbox-test-setuid-{}", std::process::id()));
        std::fs::create_dir_all(&dir).context("failed to create setuid test dir")?;
        let _cleanup = Cleanup(dir.clone());
        let fake_bwrap = dir.join("bwrap");
        std::fs::copy(&real_bwrap, &fake_bwrap).context("failed to copy bwrap")?;

        // chown root (we are root) and set the setuid bit: 04755.
        use std::os::unix::fs::PermissionsExt as _;
        std::fs::set_permissions(&fake_bwrap, std::fs::Permissions::from_mode(0o4755))
            .context("failed to set setuid bit on fake bwrap")?;

        // Make the setuid copy the *only* bwrap visible, so the launcher can't fall
        // back to a different one.
        let previous_path = std::env::var_os("PATH");
        // SAFETY: the helper is single-threaded here.
        unsafe {
            std::env::set_var("PATH", &dir);
        }
        let result = drive_launcher("true", &[], &[], None, SandboxPermissions::default());
        // SAFETY: still single-threaded; restore PATH regardless of the outcome.
        unsafe {
            match previous_path {
                Some(path) => std::env::set_var("PATH", path),
                None => std::env::remove_var("PATH"),
            }
        }
        let result = result?;

        checks.check(
            "a setuid-root bwrap is rejected",
            result.status == Some(LauncherStatus::SetuidRejected),
        );
        checks.check(
            "command does not run after rejecting setuid bwrap (fail-closed)",
            !result.command_succeeded,
        );

        Ok(())
    }

    fn current_exe_str() -> Result<String> {
        Ok(std::env::current_exe()
            .context("failed to resolve current executable")?
            .to_str()
            .context("current executable path is not valid UTF-8")?
            .to_string())
    }

    /// Find an executable named `name` by walking `PATH`.
    fn find_on_path(name: &str) -> Option<PathBuf> {
        let path = std::env::var_os("PATH")?;
        std::env::split_paths(&path)
            .map(|directory| directory.join(name))
            .find(|candidate| candidate.is_file())
    }

    /// Single-quote a path for safe interpolation into an `sh -c` script.
    fn shell_quote(path: &Path) -> String {
        let raw = path.to_string_lossy();
        format!("'{}'", raw.replace('\'', "'\\''"))
    }

    /// Removes a directory tree on drop, so scratch dirs don't pile up across runs.
    struct Cleanup(PathBuf);

    impl Drop for Cleanup {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
}
