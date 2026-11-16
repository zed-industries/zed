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
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};

    use sandbox_linux::{
        Access::{self, Allow, Deny},
        Decision, Denial, DomainPattern, FsRule, Handler, NetHost, NetRule, NetTarget, Permission,
        PortSet, Protocol, RequestedAccess, Rule, Sandbox, SandboxContext, SandboxPolicy,
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
                    .filter_map(|item| match item {
                        RequestedAccess::Fs { path, .. } => {
                            Some(path.file_name().unwrap().to_string_lossy().into_owned())
                        }
                        RequestedAccess::Net { .. } => None,
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
        let policy = |rules: Vec<FsRule>| SandboxPolicy {
            fs: rules,
            net: Vec::new(),
        };

        let context = SandboxContext::new().expect("starting supervisor");
        let sandbox = context
            .create_sandbox(policy(base_rules.clone()))
            .expect("creating sandbox");
        let denials = Arc::new(Mutex::new(Vec::<Denial>::new()));
        sandbox.on_denial({
            let denials = denials.clone();
            move |denial| {
                if let Ok(mut denials) = denials.lock() {
                    denials.push(denial);
                }
            }
        });

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
                "datagram unix socket",
                "python3 -c 'import socket; socket.socket(socket.AF_UNIX, socket.SOCK_DGRAM)'",
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
        let reported = |permission: Permission, file: &Path| {
            let request = RequestedAccess::Fs {
                permission,
                path: file.to_path_buf(),
            };
            denials.lock().is_ok_and(|denials| {
                denials
                    .iter()
                    .any(|denial| denial.requests.contains(&request))
            })
        };
        host_check(
            "denials were reported",
            reported(Permission::Read, &Path::new(&secret).join("key"))
                && reported(Permission::Write, &proj.join(".git/config"))
                && reported(Permission::Write, &Path::new(&ask).join("no.txt")),
            &mut failures,
        );

        check_policy_update(&sandbox, &proj, &base_rules, &policy, &mut failures);
        check_stop(&sandbox, &proj, &mut failures);
        drop(sandbox);
        check_network(&context, &work, &mut failures);
        check_early_connect(&context, &work, &mut failures);

        drop(context);
        if !failures.is_empty() {
            eprintln!("\n{} failure(s):\n{}", failures.len(), failures.join("\n"));
            std::process::exit(1);
        }
        println!("\nall checks passed");
    }

    /// Client side of the network checks, run in the sandbox. Each command
    /// exits 0 if what it checks happened.
    const NET_CLIENT: &str = r#"
import asyncio, errno, ipaddress, random, socket, struct, sys

FAKE = ipaddress.ip_network("198.18.0.0/15")

def echo(stream):
    stream.sendall(b"ping")
    return stream.recv(4) == b"ping"

def dns(name, record_type=1):
    query_id = random.randrange(65536)
    query = struct.pack(">HHHHHH", query_id, 0x0100, 1, 0, 0, 0)
    for label in name.split("."):
        query += bytes([len(label)]) + label.encode()
    query += b"\0" + struct.pack(">HH", record_type, 1)
    # Any server: the sandbox answers every address.
    udp = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    udp.settimeout(5)
    udp.sendto(query, ("8.8.8.8", 53))
    response, server = udp.recvfrom(4096)
    assert server == ("8.8.8.8", 53), server
    assert struct.unpack(">H", response[:2])[0] == query_id
    code = response[3] & 0xf
    answers = struct.unpack(">H", response[6:8])[0]
    address = socket.inet_ntoa(response[-4:]) if answers and record_type == 1 else None
    return code, address

def main(command, *args):
    if command == "tcp":
        host, port = args[0], int(args[1])
        stream = socket.create_connection((host, port), timeout=10)
        return echo(stream) and stream.getpeername()[1] == port
    if command == "tcp-error":
        host, port, expected = args[0], int(args[1]), getattr(errno, args[2])
        try:
            # Without a timeout, so the socket stays blocking: Python makes
            # sockets with timeouts non-blocking, and the sandbox reports
            # failures to connect those as resets.
            socket.create_connection((host, port))
        except OSError as error:
            print(error, file=sys.stderr)
            return error.errno == expected
        return False
    if command == "tcp-name":
        code, address = dns(args[0])
        stream = socket.create_connection((address, int(args[1])), timeout=10)
        return echo(stream)
    if command == "asyncio":
        async def run():
            reader, writer = await asyncio.open_connection(args[0], int(args[1]))
            writer.write(b"ping")
            return await reader.readexactly(4) == b"ping"
        return asyncio.run(run())
    if command == "asyncio-refused":
        # Check the two stages independently: a refused connect must not
        # satisfy this test in place of the later reset.
        import select
        stream = socket.socket()
        stream.setblocking(False)
        assert stream.connect_ex((args[0], int(args[1]))) == 0
        assert select.select([stream], [], [], 5)[0]
        try:
            stream.recv(1)
        except ConnectionResetError:
            return True
        return False
    if command == "local-server":
        server = socket.create_server(("127.0.0.1", 0))
        client = socket.create_connection(server.getsockname(), timeout=5)
        connection, _ = server.accept()
        client.sendall(b"hi")
        return connection.recv(2) == b"hi"
    if command == "udp":
        host, port, mode = args[0], int(args[1]), args[2]
        udp = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        udp.settimeout(3)
        if mode == "connect":
            udp.connect((host, port))
            udp.send(b"ping")
            return udp.recv(4) == b"ping"
        udp.sendto(b"ping", (host, port))
        data, source = udp.recvfrom(4)
        return data == b"ping" and source == (host, port)
    if command == "dns-a":
        code, address = dns(args[0])
        return code == 0 and ipaddress.ip_address(address) in FAKE
    if command == "dns-nxdomain":
        return dns(args[0])[0] == 3
    if command == "dns-aaaa-empty":
        return dns(args[0], 28) == (0, None)
    if command == "getaddrinfo":
        return ipaddress.ip_address(socket.gethostbyname(args[0])) in FAKE
    if command == "ping-socket":
        socket.socket(socket.AF_INET, socket.SOCK_DGRAM, socket.IPPROTO_ICMP)
        return True
    if command == "unix-echo":
        stream = socket.socket(socket.AF_UNIX)
        stream.settimeout(5)
        stream.connect(args[0])
        return echo(stream)
    if command == "unix-error":
        stream = socket.socket(socket.AF_UNIX)
        try:
            stream.connect(args[0])
        except OSError as error:
            print(error, file=sys.stderr)
            return error.errno == getattr(errno, args[1])
        return False
    if command == "unix-server":
        # A path, or a name starting with @ for an abstract address.
        address = "\0" + args[0][1:] if args[0].startswith("@") else args[0]
        server = socket.socket(socket.AF_UNIX)
        server.bind(address)
        server.listen()
        client = socket.socket(socket.AF_UNIX)
        client.settimeout(5)
        client.connect(address)
        connection, _ = server.accept()
        client.sendall(b"hi")
        return connection.recv(2) == b"hi"
    if command == "unix-bind":
        socket.socket(socket.AF_UNIX).bind(args[0])
        return True
    if command == "socketpair":
        kind = getattr(socket, args[0])
        first, second = socket.socketpair(socket.AF_UNIX, kind)
        first.send(b"hi")
        return second.recv(2) == b"hi"
    raise ValueError(command)

sys.exit(0 if main(*sys.argv[1:]) else 1)
"#;

    fn tcp_echo_server() -> u16 {
        use std::io::{Read as _, Write as _};
        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("binding");
        let port = listener.local_addr().unwrap().port();
        std::thread::spawn(move || {
            for mut stream in listener.incoming().flatten() {
                std::thread::spawn(move || {
                    let mut buffer = [0; 1024];
                    while let Ok(length @ 1..) = stream.read(&mut buffer) {
                        if stream.write_all(&buffer[..length]).is_err() {
                            break;
                        }
                    }
                });
            }
        });
        port
    }

    fn unix_echo_server(path: &Path) {
        use std::io::{Read as _, Write as _};
        let listener = std::os::unix::net::UnixListener::bind(path).expect("binding");
        std::thread::spawn(move || {
            for mut stream in listener.incoming().flatten() {
                std::thread::spawn(move || {
                    let mut buffer = [0; 1024];
                    while let Ok(length @ 1..) = stream.read(&mut buffer) {
                        if stream.write_all(&buffer[..length]).is_err() {
                            break;
                        }
                    }
                });
            }
        });
    }

    fn udp_echo_server() -> u16 {
        let socket = std::net::UdpSocket::bind("127.0.0.1:0").expect("binding");
        let port = socket.local_addr().unwrap().port();
        std::thread::spawn(move || {
            let mut buffer = [0; 1024];
            while let Ok((length, source)) = socket.recv_from(&mut buffer) {
                socket.send_to(&buffer[..length], source).ok();
            }
        });
        port
    }

    /// Hold the rebinding prompt open to deterministically stall the host
    /// connection. The app must get its local connection (and run another
    /// event-loop task) before we approve the host leg. No network latency
    /// assumptions or privileged netem setup are needed.
    fn check_early_connect(context: &SandboxContext, work: &Path, failures: &mut Vec<String>) {
        use std::io::{BufRead as _, BufReader};
        use std::sync::mpsc;

        let port = tcp_echo_server();
        for cancel in [false, true] {
            let (prompts, pending) = mpsc::channel();
            let handler = Handler::new(move |request, responder| {
                assert!(request.items.iter().all(|item| matches!(
                    item,
                    RequestedAccess::Net {
                        host: NetHost::Ip(_),
                        ..
                    }
                )));
                if let Err(error) = prompts.send(responder) {
                    error.0.respond(Decision::deny());
                }
            });
            let sandbox = context
                .create_sandbox(SandboxPolicy {
                    fs: vec![FsRule::new("/").unwrap().read(Allow)],
                    net: vec![
                        NetRule::new(
                            NetTarget::Domain(DomainPattern::new("localhost").unwrap()),
                            Allow,
                        )
                        .tcp_only(),
                        NetRule::new(NetTarget::Loopback, Access::Ask(handler)).tcp_only(),
                        NetRule::new(NetTarget::Ip("::1".parse().unwrap()), Allow).tcp_only(),
                    ],
                })
                .unwrap();
            // Use the sandbox's raw DNS query helper: libc resolves
            // localhost from /etc/hosts, which would bypass fake-IP mapping.
            let dns_helpers = NET_CLIENT
                .split("def main(command, *args):")
                .next()
                .unwrap();
            let script = format!(
                r#"python3 - <<'PY'
{dns_helpers}
code, address = dns('localhost')
assert code == 0
async def main():
    reader, writer = await asyncio.open_connection(address, {port})
    await asyncio.sleep(0)
    print('connected and responsive', flush=True)
    writer.write(b'ping')
    assert await reader.readexactly(4) == b'ping'
    writer.close()
    await writer.wait_closed()
asyncio.run(main())
PY"#
            );
            let mut prepared = sandbox.prepare(&command(&script, work)).unwrap();
            prepared
                .command_mut()
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());
            let mut helper = prepared.spawn().unwrap();
            let stdout = helper.stdout.take().unwrap();
            let (lines, output) = mpsc::channel();
            let reader = std::thread::spawn(move || {
                for line in BufReader::new(stdout).lines() {
                    if lines.send(line).is_err() {
                        break;
                    }
                }
            });
            let responder = pending.recv_timeout(Duration::from_secs(5)).unwrap();
            let responsive = output
                .recv_timeout(Duration::from_secs(5))
                .is_ok_and(|line| line.is_ok_and(|line| line == "connected and responsive"));
            if cancel {
                // The prompt belongs to the background host connection,
                // not a still-blocked seccomp notification.
                unsafe { libc::kill(helper.id() as i32, libc::SIGTERM) };
                let status = helper.wait().unwrap();
                let deadline = Instant::now() + Duration::from_secs(5);
                while !responder.is_cancelled() && Instant::now() < deadline {
                    std::thread::sleep(Duration::from_millis(20));
                }
                host_check(
                    "early connect is cancelled when its execution ends",
                    responsive && status.code() == Some(143) && responder.is_cancelled(),
                    failures,
                );
                responder.respond(Decision::deny());
            } else {
                responder.respond(Decision::allow());
                let output = helper.wait_with_output().unwrap();
                host_check(
                    "event loop progresses before the host connection is approved",
                    responsive && output.status.success(),
                    failures,
                );
            }
            reader.join().unwrap();
        }
    }

    /// Connections to echo servers on the host's loopback, by address and by
    /// name, through rules, a prompt, and DNS.
    fn check_network(context: &SandboxContext, work: &Path, failures: &mut Vec<String>) {
        let allowed = tcp_echo_server();
        let rebinding = tcp_echo_server();
        let asked = tcp_echo_server();
        let unlisted = tcp_echo_server();
        let udp_allowed = udp_echo_server();
        let udp_unlisted = udp_echo_server();
        // Allowed, but nothing listens there.
        let closed = std::net::TcpListener::bind("127.0.0.1:0")
            .and_then(|listener| listener.local_addr())
            .expect("finding a free port")
            .port();
        let sockets = work.join("sockets");
        std::fs::create_dir_all(&sockets).unwrap();
        let allowed_socket = sockets.join("allowed.sock");
        let denied_socket = sockets.join("denied.sock");
        unix_echo_server(&allowed_socket);
        unix_echo_server(&denied_socket);

        let net_prompts = Arc::new(Mutex::new(Vec::<RequestedAccess>::new()));
        let handler = Handler::new({
            let net_prompts = net_prompts.clone();
            move |request, responder| {
                if let Ok(mut prompts) = net_prompts.lock() {
                    prompts.extend(request.items.iter().cloned());
                }
                let allow = request.items.iter().all(
                    |item| matches!(item, RequestedAccess::Net { port, .. } if *port == asked),
                );
                responder.respond(if allow {
                    Decision::allow()
                } else {
                    Decision::deny()
                });
            }
        });
        let localhost = || NetTarget::Domain(DomainPattern::new("localhost").unwrap());
        let host_loopback = || NetTarget::Ip("127.0.0.1".parse().unwrap());
        let ports =
            |ports: &[u16]| PortSet::Ranges(ports.iter().map(|port| *port..=*port).collect());
        let policy = SandboxPolicy {
            fs: vec![
                FsRule::new("/").unwrap().read(Allow),
                FsRule::new(work.to_str().unwrap()).unwrap().write(Allow),
                FsRule::new(allowed_socket.to_str().unwrap())
                    .unwrap()
                    .connect(Allow),
            ],
            net: vec![
                NetRule::new(host_loopback(), Allow)
                    .ports(ports(&[allowed, closed]))
                    .tcp_only(),
                NetRule::new(host_loopback(), Allow)
                    .ports(PortSet::single(udp_allowed))
                    .udp_only(),
                // Names resolving to loopback also need the address allowed;
                // only `allowed` and `asked` have that.
                NetRule::new(localhost(), Allow)
                    .ports(ports(&[allowed, rebinding]))
                    .tcp_only(),
                NetRule::new(localhost(), Access::Ask(handler.clone()))
                    .ports(PortSet::single(asked))
                    .tcp_only(),
                NetRule::new(NetTarget::Loopback, Allow)
                    .ports(PortSet::single(asked))
                    .tcp_only(),
                NetRule::new(
                    NetTarget::Domain(DomainPattern::new("ask.test").unwrap()),
                    Access::Ask(handler),
                ),
            ],
        };
        let sandbox = context
            .create_sandbox(policy)
            .expect("creating network sandbox");
        std::fs::write(work.join("net.py"), NET_CLIENT).unwrap();
        let client = |arguments: &str| format!("python3 net.py {arguments}");
        let cases = vec![
            case(
                "tcp to an allowed host port",
                client(&format!("tcp 127.0.0.1 {allowed}")),
                true,
            ),
            case(
                "non-blocking tcp to an allowed host port",
                client(&format!("asyncio 127.0.0.1 {allowed}")),
                true,
            ),
            case(
                "blocking tcp to an allowed closed port is refused",
                client(&format!("tcp-error 127.0.0.1 {closed} ECONNREFUSED")),
                true,
            ),
            case(
                "non-blocking tcp to an allowed closed port is reset",
                client(&format!("asyncio-refused 127.0.0.1 {closed}")),
                true,
            ),
            case(
                "tcp to an unlisted host port is refused with EACCES",
                client(&format!("tcp-error 127.0.0.1 {unlisted} EACCES")),
                true,
            ),
            case(
                "tcp to an unlisted public address is refused with EACCES",
                client("tcp-error 1.1.1.1 80 EACCES"),
                true,
            ),
            case(
                "tcp to an unknown fake address is refused",
                client("tcp-error 198.18.255.254 80 ECONNREFUSED"),
                true,
            ),
            case(
                "tcp within the sandbox needs no rule",
                client("local-server"),
                true,
            ),
            case(
                "tcp to an allowed name",
                client(&format!("tcp-name localhost {allowed}")),
                true,
            ),
            case(
                "tcp to an allowed name resolving to loopback (rebinding)",
                client(&format!("tcp-name localhost {rebinding}")),
                false,
            ),
            case(
                "tcp to an asked name, allowed",
                client(&format!("tcp-name localhost {asked}")),
                true,
            ),
            case(
                "udp to an allowed host port, connected",
                client(&format!("udp 127.0.0.1 {udp_allowed} connect")),
                true,
            ),
            case(
                "udp to an allowed host port, sendto",
                client(&format!("udp 127.0.0.1 {udp_allowed} sendto")),
                true,
            ),
            case(
                "udp to an unlisted host port",
                client(&format!("udp 127.0.0.1 {udp_unlisted} connect")),
                false,
            ),
            case("dns: allowed name", client("dns-a localhost"), true),
            case("dns: asked name", client("dns-a ask.test"), true),
            case(
                "dns: unlisted name",
                client("dns-nxdomain example.com"),
                true,
            ),
            case("dns: no AAAA", client("dns-aaaa-empty localhost"), true),
            case(
                "getaddrinfo uses the sandbox's DNS",
                client("getaddrinfo ask.test"),
                true,
            ),
            case("no ping sockets", client("ping-socket"), false),
            case(
                "unix: connect to an allowed host socket",
                client(&format!("unix-echo {}", allowed_socket.display())),
                true,
            ),
            case(
                "unix: connect to another host socket is refused with EACCES",
                client(&format!("unix-error {} EACCES", denied_socket.display())),
                true,
            ),
            case(
                "unix: connect through a symlink to a refused socket",
                format!(
                    "ln -s {} link.sock && {}",
                    denied_socket.display(),
                    client("unix-error link.sock EACCES")
                ),
                true,
            ),
            case(
                "unix: server in TMPDIR",
                client("unix-server \"$TMPDIR/server.sock\""),
                true,
            ),
            case(
                "unix: bind in a writable directory",
                client("unix-bind bound.sock"),
                true,
            ),
            case(
                "unix: bind outside writable directories",
                client("unix-bind /tmp/sandbox-escape-test-bound.sock"),
                false,
            ),
            case(
                "unix: server on an abstract address",
                client("unix-server @zed-sandbox-test"),
                true,
            ),
            case(
                "unix: stream socketpair",
                client("socketpair SOCK_STREAM"),
                true,
            ),
            case(
                "unix: datagram socketpair",
                client("socketpair SOCK_DGRAM"),
                false,
            ),
        ];
        check(&sandbox, work, &cases, failures);
        host_check(
            "unix socket bound in the sandbox exists on the host",
            std::fs::symlink_metadata(work.join("bound.sock")).is_ok_and(|metadata| {
                std::os::unix::fs::FileTypeExt::is_socket(&metadata.file_type())
            }) && !Path::new("/tmp/sandbox-escape-test-bound.sock").exists(),
            failures,
        );
        let prompts = net_prompts
            .lock()
            .map(|prompts| prompts.clone())
            .unwrap_or_default();
        host_check(
            "one prompt, for the asked connection",
            prompts
                == [RequestedAccess::Net {
                    host: NetHost::Domain("localhost".to_string()),
                    port: asked,
                    protocol: Protocol::Tcp,
                }],
            failures,
        );
    }

    /// `SIGTERM` to the helper stops the command and everything it spawned.
    fn check_stop(sandbox: &Sandbox, proj: &Path, failures: &mut Vec<String>) {
        let pid_file = sandbox.temp_dir().join("background.pid");
        let script = format!("sleep 300 & echo $! > {}; wait", pid_file.display());
        let mut prepared = sandbox
            .prepare(&command(&script, proj))
            .expect("preparing command");
        prepared.command_mut().stdin(Stdio::null());
        let mut helper = prepared.spawn().expect("spawning command");
        let deadline = Instant::now() + Duration::from_secs(10);
        let background = loop {
            if let Some(pid) = std::fs::read_to_string(&pid_file)
                .ok()
                .and_then(|pid| pid.trim().parse::<i32>().ok())
            {
                break Some(pid);
            }
            if Instant::now() > deadline {
                break None;
            }
            std::thread::sleep(Duration::from_millis(20));
        };
        // SAFETY: plain kill of our own, not yet reaped, child.
        unsafe { libc::kill(helper.id() as libc::pid_t, libc::SIGTERM) };
        let deadline = Instant::now() + Duration::from_secs(5);
        let status = loop {
            match helper.try_wait() {
                Ok(Some(status)) => break Some(status),
                Ok(None) if Instant::now() < deadline => {
                    std::thread::sleep(Duration::from_millis(20))
                }
                _ => break None,
            }
        };
        if status.is_none() {
            helper.kill().ok();
        }
        // SAFETY: signal 0 only checks whether the process exists.
        let background_gone = background.is_some_and(|pid| unsafe { libc::kill(pid, 0) } != 0);
        host_check(
            "stopping kills the command and its children",
            status.and_then(|status| status.code()) == Some(128 + libc::SIGTERM) && background_gone,
            failures,
        );
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
