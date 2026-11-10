//! End-to-end checks against a real sandbox. Each case runs a shell snippet
//! inside one and asserts whether it succeeds; the host side is then checked
//! for leaks. Uses a custom harness because the sandbox re-executes this
//! binary as its supervisor and helper.
//!
//! Fixtures live under `/tmp/sandbox-escape-test-*`, so a sandbox bug can't
//! write anywhere that matters.

#[cfg(not(target_os = "linux"))]
fn main() {}

#[cfg(target_os = "linux")]
fn main() {
    sandbox_linux::run_hidden_entrypoint_if_invoked();
    imp::main();
}

#[cfg(target_os = "linux")]
#[allow(
    clippy::disallowed_methods,
    reason = "a standalone test binary that blocks on sandboxed commands"
)]
mod imp {
    use std::path::Path;
    use std::process::{Output, Stdio};
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{Duration, Instant};

    use sandbox_linux::{
        Access::{self, Allow, Deny},
        Decision, FsRule, Handler, RequestedAccess, Rule, Sandbox, SandboxContext, SandboxPolicy,
        UntrustedCommand,
    };

    struct Case {
        name: &'static str,
        script: String,
        succeeds: bool,
    }

    fn case(name: &'static str, script: impl Into<String>, succeeds: bool) -> Case {
        Case {
            name,
            script: script.into(),
            succeeds,
        }
    }

    fn command(script: &str, cwd: &Path) -> UntrustedCommand {
        UntrustedCommand {
            cmd: "sh".into(),
            args: vec!["-c".into(), script.into()],
            env: std::env::vars()
                .filter(|(name, _)| name == "PATH" || name == "HOME")
                .chain([("HOST_PID".to_string(), std::process::id().to_string())])
                .collect(),
            cwd: cwd.to_path_buf(),
        }
    }

    fn run(sandbox: &Sandbox, script: &str, cwd: &Path) -> Output {
        let mut prepared = sandbox
            .prepare(&command(script, cwd))
            .expect("preparing command");
        prepared.command_mut().stdin(Stdio::null());
        prepared.output().expect("running sandbox helper")
    }

    fn check(sandbox: &Sandbox, cwd: &Path, cases: &[Case], failures: &mut Vec<String>) {
        for case in cases {
            let output = run(sandbox, &case.script, cwd);
            let succeeded = output.status.success();
            let passed = succeeded == case.succeeds;
            println!("{:4} {}", if passed { "ok" } else { "FAIL" }, case.name);
            if !passed {
                failures.push(format!(
                    "{}: expected {}, got {} (stderr: {})",
                    case.name,
                    if case.succeeds { "success" } else { "failure" },
                    output.status,
                    String::from_utf8_lossy(&output.stderr).trim()
                ));
            }
        }
    }

    fn host_check(name: &str, passed: bool, failures: &mut Vec<String>) {
        println!("{:4} host: {name}", if passed { "ok" } else { "FAIL" });
        if !passed {
            failures.push(format!("host: {name}"));
        }
    }

    pub fn main() {
        let work = tempfile::Builder::new()
            .prefix("sandbox-escape-test-")
            .tempdir_in("/tmp")
            .expect("creating work dir");
        let work = work.path().canonicalize().expect("canonicalizing work dir");
        let proj = work.join("proj");
        let outside = work.join("outside");
        let secret = work.join("secret");
        let ask = work.join("ask");
        for directory in [
            proj.join(".git"),
            outside.clone(),
            secret.clone(),
            ask.clone(),
        ] {
            std::fs::create_dir_all(directory).expect("creating fixture dir");
        }
        std::fs::write(proj.join(".git/config"), "original").unwrap();
        std::fs::write(proj.join("app.env"), "API_KEY=hunter2").unwrap();
        std::fs::write(outside.join("f"), "keep").unwrap();
        std::fs::write(secret.join("key"), "password").unwrap();
        std::os::unix::fs::symlink(&proj, outside.join("qux")).unwrap();
        let host_tmp_marker = work.with_extension("marker");

        let path = |path: &Path| path.to_str().unwrap().to_string();
        let rule = |pattern: &str| FsRule::new(pattern).unwrap();

        // Decides by file name: `yes*` allowed once, `remember*` allowed and
        // every `remember*` sibling allowed from then on, `panic*` panics,
        // anything else denied. `slow*` and `once*` answer after a delay
        // (from another thread), so concurrent requests overlap: `slow*`
        // remembers every `slow*` sibling, `once*` allows just once.
        let prompts = Arc::new(AtomicUsize::new(0));
        let handler = Handler::new({
            let prompts = prompts.clone();
            let ask = ask.clone();
            move |request, responder| {
                prompts.fetch_add(1, Ordering::SeqCst);
                let names: Vec<String> = request
                    .items
                    .iter()
                    .map(|item| {
                        let RequestedAccess::Fs { path, .. } = item;
                        path.file_name().unwrap().to_string_lossy().into_owned()
                    })
                    .collect();
                let all = |prefix: &str| names.iter().all(|name| name.starts_with(prefix));
                if all("slow") || all("once") {
                    let remember = if all("slow") {
                        let pattern = format!("{}/slow*", ask.display());
                        vec![Rule::Fs(FsRule::new(pattern).unwrap().write(Allow))]
                    } else {
                        Vec::new()
                    };
                    std::thread::spawn(move || {
                        std::thread::sleep(Duration::from_millis(300));
                        responder.respond(Decision {
                            allow: true,
                            remember,
                        });
                    });
                    return;
                }
                let decision = if names.iter().all(|name| name.starts_with("yes")) {
                    Decision::allow()
                } else if names.iter().all(|name| name.starts_with("remember")) {
                    let pattern = format!("{}/remember*", ask.display());
                    Decision {
                        allow: true,
                        remember: vec![Rule::Fs(FsRule::new(pattern).unwrap().write(Allow))],
                    }
                } else if names.iter().any(|name| name.starts_with("panic")) {
                    panic!("test handler panics on purpose");
                } else {
                    Decision::deny()
                };
                responder.respond(decision);
            }
        });
        let base_rules = vec![
            rule("/").read(Allow),
            rule(&path(&proj)).write(Allow),
            rule(&path(&proj.join(".git"))).write(Deny),
            rule(&path(&secret)).read(Deny),
            rule("/**/*.env").read(Deny),
            rule(&path(&ask)).write(Access::Ask(handler)),
        ];
        let policy = |rules: Vec<FsRule>| SandboxPolicy { fs: rules };

        let context = SandboxContext::new().expect("starting supervisor");
        let sandbox = context
            .create_sandbox(policy(base_rules.clone()))
            .expect("creating sandbox");

        let probe = run(&sandbox, "true", &proj);
        if !probe.status.success() {
            let stderr = String::from_utf8_lossy(&probe.stderr);
            if stderr.contains("creating sandbox namespaces failed") {
                eprintln!("skipping sandbox_linux end-to-end tests: {}", stderr.trim());
                return;
            }
            panic!("sandbox probe failed: {stderr}");
        }

        let outside = path(&outside);
        let secret = path(&secret);
        let ask = path(&ask);
        let marker = path(&host_tmp_marker);
        let cases = vec![
            case("create file", "echo hi > new.txt", true),
            case("append", "echo more >> new.txt", true),
            case("mkdir -p", "mkdir -p a/b/c", true),
            case("rename", "mv new.txt a/moved.txt", true),
            case(
                "symlink and write through it",
                "ln -s moved.txt a/link && echo x >> a/link",
                true,
            ),
            case("hard link inside grant", "ln a/moved.txt a/hard", true),
            case(
                "chmod, truncate, access",
                "chmod 600 a/moved.txt && truncate -s 1 a/moved.txt && test -w a/moved.txt",
                true,
            ),
            case("recursive delete", "rm -r a/b", true),
            case("rename a directory", "mkdir d && mv d d2", true),
            case(
                "TMPDIR is writable",
                "echo x > \"$TMPDIR/f\" && cat \"$TMPDIR/f\"",
                true,
            ),
            case(
                "/dev/null and /dev/stderr",
                "echo x > /dev/null && echo x > /dev/stderr",
                true,
            ),
            case(
                "write to /dev/stdout (a pipe)",
                "echo x > /dev/stdout",
                true,
            ),
            case("read outside grant", format!("cat {outside}/f"), true),
            case(
                "write through outside symlink into grant",
                format!("echo x > {outside}/qux/via-qux"),
                true,
            ),
            case(
                "own /proc entries",
                "cat /proc/self/status >/dev/null && cat /proc/$$/cmdline >/dev/null",
                true,
            ),
            case("signal own process", "sleep 10 & kill $!", true),
            // The sandbox shares this test's process group, so a group signal
            // that leaked out would kill the test.
            case(
                "signal own process group",
                "trap '' TERM; kill -TERM 0",
                true,
            ),
            case("list a read-denied dir", format!("ls {secret}"), true),
            case("stat a read-denied file", "stat app.env", true),
            case("write a new .env file", "echo A=1 > new.env", true),
            case(
                "asked write, allowed",
                format!("echo x > {ask}/yes.txt"),
                true,
            ),
            // One prompt; the second write is covered by the remembered rule.
            case(
                "asked write, remembered",
                format!("echo x > {ask}/remember-a.txt && echo x > {ask}/remember-b.txt"),
                true,
            ),
            // One prompt: the second write waits for the first's, then is
            // allowed by the rule it remembered.
            case(
                "concurrent asked writes, remembered",
                format!(
                    "echo x > {ask}/slow-a.txt & a=$!; echo x > {ask}/slow-b.txt & b=$!; \
                     wait $a && wait $b"
                ),
                true,
            ),
            // One prompt: the identical request shares the open prompt's answer.
            case(
                "concurrent identical asked writes",
                format!(
                    "echo x > {ask}/once.txt & a=$!; echo y > {ask}/once.txt & b=$!; \
                     wait $a && wait $b"
                ),
                true,
            ),
            case(
                "capabilities are empty",
                "grep -q 'CapEff:\t0000000000000000' /proc/self/status",
                true,
            ),
            case(
                "read a read-denied file",
                format!("cat {secret}/key"),
                false,
            ),
            case("read a .env file", "cat app.env", false),
            case("read a new .env file", "cat new.env", false),
            case(
                "read .env through a symlink",
                "ln -sf app.env sym.txt && cat sym.txt",
                false,
            ),
            case("copy a .env file", "cp app.env copy.txt", false),
            case("rename a .env file", "mv app.env app.txt", false),
            case("hard link a .env file", "ln app.env hard.txt", false),
            case(
                "asked write, denied",
                format!("echo x > {ask}/no.txt"),
                false,
            ),
            case(
                "asked write, handler panics",
                format!("echo x > {ask}/panic.txt"),
                false,
            ),
            case("write protected file", "echo pwn > .git/config", false),
            case("delete protected file", "rm -f .git/config", false),
            case("rename protected dir", "mv .git gitx", false),
            case(
                "write outside grant",
                format!("echo x > {outside}/g"),
                false,
            ),
            case(
                "overwrite outside grant",
                format!("echo x > {outside}/f"),
                false,
            ),
            case(
                "write via symlink to protected file",
                "ln -sf .git/config evil && echo pwn > evil",
                false,
            ),
            case(
                "hard link outside file into grant",
                format!("ln {outside}/f hardout"),
                false,
            ),
            case(
                "escape with ..",
                "mkdir -p sub && echo x > sub/../../outside/h",
                false,
            ),
            case("write host /tmp", format!("echo x > {marker}"), false),
            case(
                "read a host process's environment",
                "cat /proc/$HOST_PID/environ",
                false,
            ),
            case("signal a host process", "kill -0 $HOST_PID", false),
            case("new user namespace", "unshare -r true", false),
            case(
                "unix socket",
                "python3 -c 'import socket; socket.socket(socket.AF_UNIX)'",
                false,
            ),
        ];
        let mut failures = Vec::new();
        check(&sandbox, &proj, &cases, &mut failures);

        let outside = Path::new(&outside);
        host_check(
            "protected file unchanged",
            std::fs::read_to_string(proj.join(".git/config"))
                .ok()
                .as_deref()
                == Some("original"),
            &mut failures,
        );
        host_check(
            ".env file unchanged",
            std::fs::read_to_string(proj.join("app.env"))
                .ok()
                .as_deref()
                == Some("API_KEY=hunter2")
                && !proj.join("copy.txt").exists()
                && !proj.join("hard.txt").exists(),
            &mut failures,
        );
        host_check(
            "outside file unchanged",
            std::fs::read_to_string(outside.join("f")).ok().as_deref() == Some("keep"),
            &mut failures,
        );
        host_check(
            "nothing created outside",
            !outside.join("g").exists() && !outside.join("h").exists(),
            &mut failures,
        );
        host_check(
            "host /tmp untouched",
            !host_tmp_marker.exists(),
            &mut failures,
        );
        host_check(
            "asked writes followed the answers",
            [
                "yes.txt",
                "remember-a.txt",
                "remember-b.txt",
                "slow-a.txt",
                "slow-b.txt",
                "once.txt",
            ]
            .iter()
            .all(|name| Path::new(&ask).join(name).exists())
                && ["no.txt", "panic.txt"]
                    .iter()
                    .all(|name| !Path::new(&ask).join(name).exists()),
            &mut failures,
        );
        host_check(
            "one prompt per undecided syscall",
            prompts.load(Ordering::SeqCst) == 6,
            &mut failures,
        );

        check_policy_update(&sandbox, &proj, &base_rules, &policy, &mut failures);

        drop(sandbox);
        drop(context);
        if !failures.is_empty() {
            eprintln!("\n{} failure(s):\n{}", failures.len(), failures.join("\n"));
            std::process::exit(1);
        }
        println!("\nall checks passed");
    }

    /// Narrow the policy while a command holds a file open for writing: new
    /// writes fail, and the open file is reported.
    fn check_policy_update(
        sandbox: &Sandbox,
        proj: &Path,
        base_rules: &[FsRule],
        policy: &dyn Fn(Vec<FsRule>) -> SandboxPolicy,
        failures: &mut Vec<String>,
    ) {
        let held = proj.join("held.txt");
        let mut prepared = sandbox
            .prepare(&command("exec 3>>held.txt; read line", proj))
            .expect("preparing command");
        prepared.command_mut().stdin(Stdio::piped());
        let mut holder = prepared.spawn().expect("spawning holder");
        let deadline = Instant::now() + Duration::from_secs(10);
        while !held.exists() && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(20));
        }

        let mut narrowed = base_rules.to_vec();
        narrowed.push(FsRule::new(proj.to_str().unwrap()).unwrap().write(Deny));
        let still_open = sandbox
            .update_policy(&policy(narrowed))
            .expect("updating policy");
        host_check(
            "narrowing reports the held file",
            still_open
                .iter()
                .any(|open| open.path == held && open.write && !open.read),
            failures,
        );
        check(
            sandbox,
            proj,
            &[case(
                "write after narrowing",
                "echo x > after-narrowing.txt",
                false,
            )],
            failures,
        );
        drop(holder.stdin.take());
        holder.wait().ok();

        sandbox
            .update_policy(&policy(base_rules.to_vec()))
            .expect("restoring policy");
        check(
            sandbox,
            proj,
            &[case(
                "write after restoring",
                "echo x > after-restoring.txt",
                true,
            )],
            failures,
        );
    }
}
