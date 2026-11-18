//! End-to-end checks against a real sandbox, driven by a table of scenarios.
//!
//! Each [`Scenario`] in the table below is data: the fixtures it needs on the
//! host, the policy it runs under, one action the untrusted command attempts,
//! and what must happen — including how long the action must be blocked on an
//! `Ask` prompt, how many prompts may be shown, which denial must be reported,
//! and what must hold on the host afterwards. Every scenario runs in a fresh
//! directory under a fresh sandbox, so each policy states exactly what it
//! allows and scenarios can't interact.
//!
//! Behaviour that is inherently a sequence — narrowing the policy while a file
//! is held open, stopping a running command, a prompt cancelled by the command
//! ending — is kept as dedicated checks after the table.
//!
//! Declared as an example gated behind `__end_to_end_harness`, not a test
//! target: CI's `cargo nextest run --workspace` never builds or runs it. The
//! sandbox re-executes this binary as its own supervisor and helper, so it
//! needs its own `main()`; the NixOS VM tests build it with Crane and run it
//! as `sandbox-linux-test-helper` (nix/tests/sandboxing/default.nix).
//!
//! The actions need Python 3, `unshare` and the usual coreutils on PATH,
//! checked before running. Echo peers run in-process on the host, so no
//! network services are needed. Fixtures live under
//! `/tmp/sandbox-escape-test-*`, so a sandbox bug can't write anywhere that
//! matters.

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
    use std::fmt::Write as _;
    use std::os::unix::fs::FileTypeExt as _;
    use std::path::{Path, PathBuf};
    use std::process::{Output, Stdio};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};

    use sandbox_linux::{
        Access::{self, Allow, Deny},
        Decision, Denial, DomainPattern, FsRule, Handler, NetHost, NetRule, NetTarget, Permission,
        PortSet, Protocol, RequestedAccess, Rule, SandboxContext, SandboxPolicy, UntrustedCommand,
    };

    /// A delayed `Ask` answer takes this long, so scenarios that block on it
    /// can assert the syscall was genuinely held, not failed straight away.
    const ASK_DELAY: Duration = Duration::from_millis(600);
    /// Slack when asserting a syscall was blocked, so scheduler jitter under a
    /// loaded machine can't flake the timing check.
    const TIMING_SLACK: Duration = Duration::from_millis(150);

    /// The host's own namespaces, to prove the sandboxed command left them.
    struct Ctx {
        namespaces: Vec<(&'static str, String)>,
    }

    /// The echo peers and free ports scenarios connect to.
    struct World {
        /// Kept bound, never listened: `tcp_closed` stays deterministically refused.
        _closed: std::os::fd::OwnedFd,
        tcp_allowed: u16,
        tcp_asked: u16,
        tcp_unlisted: u16,
        /// Allowed by policy, but nothing listens there.
        tcp_closed: u16,
        tcp_rebinding: u16,
        udp_allowed: u16,
        udp_unlisted: u16,
        unix_allowed: PathBuf,
        unix_denied: PathBuf,
        unix_denied_bind: PathBuf,
    }

    /// One scenario: fixtures, policy, action, expectation.
    struct Scenario {
        name: &'static str,
        /// This scenario's own directory: its cwd, and the root its fixtures
        /// and policy are expressed from.
        home: PathBuf,
        fixtures: Vec<Fixture>,
        policy: SandboxPolicy,
        action: Action,
        expected: Expected,
        /// The action must take at least this long: it was held on a prompt
        /// rather than failing straight away.
        blocked_for: Option<Duration>,
        /// How many prompts the scenario's handler must have answered.
        expected_prompts: Option<usize>,
        /// A denial the harness must see reported through `Sandbox::on_denial`.
        reported: Option<RequestedAccess>,
        /// What must hold on the host afterwards.
        host_checks: Vec<HostCheck>,
        prompts: Arc<AtomicUsize>,
        denials: Arc<Mutex<Vec<Denial>>>,
    }

    /// What exists on the host before the sandbox runs.
    enum Fixture {
        File {
            path: PathBuf,
            contents: &'static str,
        },
        Directory {
            path: PathBuf,
        },
        Symlink {
            target: PathBuf,
            link: PathBuf,
        },
    }

    /// What the untrusted command attempts.
    enum Action {
        // Filesystem.
        Read {
            path: PathBuf,
        },
        Write {
            path: PathBuf,
        },
        Append {
            path: PathBuf,
        },
        CreateDirectory {
            path: PathBuf,
        },
        Delete {
            path: PathBuf,
        },
        Rename {
            from: PathBuf,
            to: PathBuf,
        },
        Symlink {
            target: PathBuf,
            link: PathBuf,
        },
        HardLink {
            target: PathBuf,
            link: PathBuf,
        },
        List {
            path: PathBuf,
        },
        Stat {
            path: PathBuf,
        },
        /// `open(2)` through the client, asserting the errno on denial.
        /// `r`: `O_RDONLY`; `w`: `O_WRONLY | O_TRUNC`; `wc`: plus `O_CREAT`.
        Open {
            path: PathBuf,
            mode: &'static str,
        },
        /// Write (and read back) a file under the sandbox's `TMPDIR`.
        WriteTemp {
            file: &'static str,
        },
        /// `cat /proc/$HOST_PID/environ`: another process's environment.
        ReadHostEnvironment,
        /// `kill -0` a process outside the execution.
        SignalHost,
        /// `unshare -r`: a new user namespace.
        NewUserNamespace,
        /// `cat /proc/self/status` and `/proc/$$/cmdline`.
        ReadOwnProc,
        /// Assert the sandbox's namespaces differ from the host's.
        AssertNamespaces,
        /// Assert the command's capability sets are empty.
        AssertNoCapabilities,
        /// A composite shell snippet, for steps that only mean something
        /// together (`chmod` then `test -w`, signalling a group, ...).
        Shell(String),

        // Unix sockets.
        ConnectUnix {
            path: PathBuf,
        },
        BindUnix {
            path: PathBuf,
        },
        /// Serve on a unix socket in the sandbox's `TMPDIR`, then connect.
        UnixServer {
            file: &'static str,
        },
        /// Serve on an abstract (namespace-local) name, then connect.
        AbstractUnixServer {
            name: &'static str,
        },
        /// `socketpair` of this kind (`SOCK_*`).
        SocketPair {
            kind: &'static str,
        },
        /// A datagram unix socket: `socket()` or a `socketpair`.
        DatagramUnix,

        // Network.
        Connect {
            host: String,
            port: u16,
        },
        ConnectNonBlocking {
            host: String,
            port: u16,
        },
        /// Resolve a name with the sandbox's DNS, then connect to it.
        ConnectByName {
            name: &'static str,
            port: u16,
        },
        /// Serve and connect on TCP inside the sandbox; no rule needed.
        LocalServer,
        Udp {
            host: String,
            port: u16,
            connected: bool,
        },
        /// A raw DNS query for a name.
        Dns {
            name: &'static str,
        },
        /// `getaddrinfo` a name.
        Resolve {
            name: &'static str,
        },
        /// A ping (`ICMP`) socket.
        Ping,
    }

    /// What must happen.
    #[derive(Debug)]
    enum Expected {
        /// The command exits 0.
        Allowed,
        /// The command exits non-zero, any way it likes.
        Denied,
        /// The client asserts the failing syscall returned one of these errnos
        /// (or, for `open`, one of `EACCES`/`EROFS`) and exits 0 when it does.
        Errno(&'static [i32]),
        /// A non-blocking connect succeeds locally, then the connection is
        /// reset: the host leg was refused.
        Reset,
        /// The name resolves, to a fake address from the sandbox's DNS.
        FakeAddress,
        /// The name does not resolve (NXDOMAIN).
        NxDomain,
        /// The name resolves, but has no record of the type asked.
        EmptyRecord,
    }

    impl Expected {
        fn holds(&self, output: &Output) -> bool {
            match self {
                Expected::Denied => !output.status.success(),
                _ => output.status.success(),
            }
        }
    }

    /// What must hold on the host after a scenario ran.
    enum HostCheck {
        Contents { path: PathBuf, equals: &'static str },
        Absent(PathBuf),
        Socket(PathBuf),
    }

    /// How an `Ask` handler answers.
    #[derive(Clone)]
    enum Ask {
        Immediate(Answer),
        /// Answer after a delay, from another thread, so the syscall (and
        /// with it the command) is genuinely blocked meanwhile.
        Delayed(Duration, Answer),
    }

    #[derive(Clone)]
    enum Answer {
        Allow,
        Deny,
        /// Allow, and remember a rule letting later writes through.
        Remember(String),
        Panic,
    }

    impl Action {
        /// The command the sandbox runs for this action, as an `sh -c` script.
        fn script(&self, ctx: &Ctx, expected: &Expected) -> String {
            match self {
                Action::Read { path } => format!("cat {}", shell(path)),
                Action::Write { path } => format!("echo content > {}", shell(path)),
                Action::Append { path } => format!("echo content >> {}", shell(path)),
                Action::CreateDirectory { path } => format!("mkdir -p {}", shell(path)),
                Action::Delete { path } => format!("rm -r {}", shell(path)),
                Action::Rename { from, to } => {
                    format!("mv {} {}", shell(from), shell(to))
                }
                Action::Symlink { target, link } => {
                    format!("ln -s {} {}", shell(target), shell(link))
                }
                Action::HardLink { target, link } => {
                    format!("ln {} {}", shell(target), shell(link))
                }
                Action::List { path } => format!("ls {}", shell(path)),
                Action::Stat { path } => format!("stat {}", shell(path)),
                Action::Open { path, mode } => py_open(path, mode),
                Action::WriteTemp { file } => {
                    format!("echo x > \"$TMPDIR/{file}\" && cat \"$TMPDIR/{file}\"")
                }
                Action::ReadHostEnvironment => "cat /proc/$HOST_PID/environ".to_string(),
                Action::SignalHost => "kill -0 $HOST_PID".to_string(),
                Action::NewUserNamespace => "unshare -r true".to_string(),
                Action::ReadOwnProc => {
                    "cat /proc/self/status >/dev/null && cat /proc/$$/cmdline >/dev/null"
                        .to_string()
                }
                Action::AssertNamespaces => {
                    let mut script = String::from("python3 - <<'PY'\nimport os\n");
                    for (namespace, host) in &ctx.namespaces {
                        let _ = writeln!(
                            script,
                            "assert os.readlink('/proc/self/ns/{namespace}') != {host:?}, \
                             '{namespace} namespace is the host''s'"
                        );
                    }
                    format!("{script}PY")
                }
                Action::AssertNoCapabilities => {
                    "python3 -c 'status = open(\"/proc/self/status\").read(); \
                     assert \"CapEff:\t0000000000000000\" in status, status'"
                        .to_string()
                }
                Action::Shell(script) => script.clone(),
                Action::ConnectUnix { path } => {
                    let expected = match expected {
                        Expected::Errno(errnos) => net_errnos(errnos),
                        _ => String::new(),
                    };
                    net(&format!("unix-connect {} {expected}", shell(path)))
                }
                Action::BindUnix { path } => {
                    let expected = match expected {
                        Expected::Errno(errnos) => net_errnos(errnos),
                        _ => String::new(),
                    };
                    net(&format!("unix-bind {} {expected}", shell(path)))
                }
                Action::UnixServer { file } => net(&format!("unix-server \"$TMPDIR/{file}\"")),
                Action::AbstractUnixServer { name } => net(&format!("unix-server @{name}")),
                Action::SocketPair { kind } => net(&format!("socketpair {kind}")),
                Action::DatagramUnix => net("datagram-unix"),
                Action::Connect { host, port } => {
                    let expected = match expected {
                        Expected::Errno(errnos) => net_errnos(errnos),
                        _ => String::new(),
                    };
                    net(&format!("tcp {host} {port} {expected}"))
                }
                Action::ConnectNonBlocking { host, port } => {
                    let expected = match expected {
                        Expected::Reset => "reset".to_string(),
                        Expected::Errno(errnos) => net_errnos(errnos),
                        _ => String::new(),
                    };
                    net(&format!("tcp-nonblocking {host} {port} {expected}"))
                }
                Action::ConnectByName { name, port } => {
                    let expected = match expected {
                        Expected::Errno(errnos) => net_errnos(errnos),
                        _ => String::new(),
                    };
                    net(&format!("tcp-name {name} {port} {expected}"))
                }
                Action::LocalServer => net("local-server"),
                Action::Udp {
                    host,
                    port,
                    connected,
                } => net(&format!(
                    "udp {host} {port} {}",
                    if *connected { "connect" } else { "sendto" }
                )),
                Action::Dns { name } => {
                    let verdict = match expected {
                        Expected::FakeAddress => "fake",
                        Expected::NxDomain => "nxdomain",
                        Expected::EmptyRecord => "empty",
                        _ => "fake",
                    };
                    let record = match expected {
                        Expected::EmptyRecord => "28",
                        _ => "1",
                    };
                    net(&format!("dns {name} {verdict} {record}"))
                }
                Action::Resolve { name } => net(&format!("resolve {name}")),
                Action::Ping => {
                    let expected = match expected {
                        Expected::Errno(errnos) => net_errnos(errnos),
                        _ => String::new(),
                    };
                    net(&format!("ping-socket {expected}"))
                }
            }
        }
    }

    /// Quote a path for a shell script.
    fn shell(path: &Path) -> String {
        let text = path.to_string_lossy();
        if text.chars().any(|character| {
            character.is_whitespace() || "[]*?{}()<>|&;\"'`\\$#".contains(character)
        }) {
            format!("'{}'", text.replace('\'', "'\\''"))
        } else {
            text.into_owned()
        }
    }

    /// The client script (`net.py`) is written into each scenario's home,
    /// which is also the command's cwd, so scripts refer to it relatively.
    fn net(arguments: &str) -> String {
        format!("python3 net.py {arguments}")
    }

    /// The errno names the net client asserts (Python `errno` attributes).
    fn net_errnos(errnos: &'static [i32]) -> String {
        errnos
            .iter()
            .map(|errno| match *errno {
                libc::EACCES => "EACCES",
                libc::EPERM => "EPERM",
                libc::ECONNREFUSED => "ECONNREFUSED",
                libc::EROFS => "EROFS",
                _ => unreachable!("unmapped errno in the scenario table"),
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Assert `open(path, mode)` fails with a permission error (`EACCES`,
    /// or `EROFS` for writes: the message tools expect for a read-only
    /// mount).
    fn py_open(path: &Path, mode: &str) -> String {
        let flags = match mode {
            "r" => "os.O_RDONLY",
            "w" => "os.O_WRONLY | os.O_TRUNC",
            "wc" => "os.O_WRONLY | os.O_CREAT | os.O_TRUNC",
            _ => unreachable!("unknown open mode"),
        };
        let lines = [
            "import errno, os".to_string(),
            format!("path = {:?}", shell(path)),
            format!("flags = {flags}"),
            "try:".to_string(),
            "    descriptor = os.open(path, flags)".to_string(),
            "except OSError as error:".to_string(),
            "    assert error.errno in (errno.EACCES, errno.EROFS), error".to_string(),
            "else:".to_string(),
            "    os.close(descriptor)".to_string(),
            "    raise AssertionError('denied open succeeded: ' + path)".to_string(),
        ];
        let script = lines.join("\n");
        format!("python3 - <<'PY'\n{script}\nPY")
    }

    /// The `Ask` handler for one scenario: counts its prompts and answers
    /// them as told.
    fn ask_handler(prompts: Arc<AtomicUsize>, ask: Ask) -> Handler {
        Handler::new(move |_, responder| {
            prompts.fetch_add(1, Ordering::SeqCst);
            match ask.clone() {
                Ask::Immediate(answer) => answer_the(responder, answer),
                Ask::Delayed(delay, answer) => {
                    std::thread::spawn(move || {
                        std::thread::sleep(delay);
                        answer_the(responder, answer);
                    });
                }
            }
        })
    }

    fn answer_the(responder: sandbox_linux::Responder, answer: Answer) {
        match answer {
            Answer::Allow => responder.respond(Decision::allow()),
            Answer::Deny => responder.respond(Decision::deny()),
            Answer::Remember(pattern) => responder.respond(Decision {
                allow: true,
                remember: vec![Rule::Fs(FsRule::new(pattern).unwrap().write(Allow))],
            }),
            Answer::Panic => panic!("test handler panics on purpose"),
        }
    }

    /// Builds the scenario table: each scenario gets its own numbered
    /// directory under `root`, so scenarios are independent.
    struct Table {
        root: PathBuf,
        scenarios: Vec<Scenario>,
    }

    impl Table {
        fn new(root: PathBuf) -> Self {
            Self {
                root,
                scenarios: Vec::new(),
            }
        }

        /// Add a scenario: fixtures, policy, action, expectation. The policy
        /// is built from the handler its `Ask` rules must use, and the
        /// scenario's home (its fixtures and relative paths live there).
        fn push(
            &mut self,
            name: &'static str,
            fixtures: Vec<Fixture>,
            ask: Ask,
            action: Action,
            expected: Expected,
            policy: impl FnOnce(Handler, &Path) -> SandboxPolicy,
        ) {
            let home = self
                .root
                .join("scenarios")
                .join(format!("{:03}", self.scenarios.len()));
            let prompts = Arc::new(AtomicUsize::new(0));
            let denials = Arc::new(Mutex::new(Vec::new()));
            let ask = match ask {
                Ask::Immediate(Answer::Remember(pattern)) => Ask::Immediate(Answer::Remember(
                    format!("{}/ask/{pattern}", home.display()),
                )),
                Ask::Delayed(delay, Answer::Remember(pattern)) => Ask::Delayed(
                    delay,
                    Answer::Remember(format!("{}/ask/{pattern}", home.display())),
                ),
                other => other,
            };
            let handler = ask_handler(prompts.clone(), ask);
            let policy = policy(handler, &home);
            self.scenarios.push(Scenario {
                name,
                home,
                fixtures,
                policy,
                action,
                expected,
                blocked_for: None,
                expected_prompts: None,
                reported: None,
                host_checks: Vec::new(),
                prompts,
                denials,
            });
        }

        /// The action must be held on its prompt for at least `at_least`.
        fn blocked_for(&mut self, at_least: Duration) {
            self.scenarios.last_mut().unwrap().blocked_for = Some(at_least);
        }

        fn expected_prompts(&mut self, count: usize) {
            self.scenarios.last_mut().unwrap().expected_prompts = Some(count);
        }

        fn reported(&mut self, mut request: RequestedAccess) {
            // Denials carry absolute paths; the table writes relative ones.
            match &mut request {
                RequestedAccess::Fs { path, .. } => {
                    let home = self.scenarios.last().unwrap().home.clone();
                    if path.is_relative() {
                        *path = home.join(&*path);
                    }
                }
                RequestedAccess::Net { .. } => {}
            }
            self.scenarios.last_mut().unwrap().reported = Some(request);
        }

        fn after(&mut self, checks: Vec<HostCheck>) {
            self.scenarios.last_mut().unwrap().host_checks = checks;
        }
    }

    fn file(path: &str, contents: &'static str) -> Fixture {
        Fixture::File {
            path: PathBuf::from(path),
            contents,
        }
    }
    fn directory(path: &str) -> Fixture {
        Fixture::Directory {
            path: PathBuf::from(path),
        }
    }
    fn symlink(target: &str, link: &str) -> Fixture {
        Fixture::Symlink {
            target: PathBuf::from(target),
            link: PathBuf::from(link),
        }
    }

    /// The full scenario table. `root` is the fixture root (every scenario's
    /// home, and every peer socket, lives under it), `world` the peers and
    /// free ports. Scenario-relative paths (fixtures, actions, host checks)
    /// resolve inside each scenario's own home.
    fn table(root: &Path, world: &World) -> Vec<Scenario> {
        let mut t = Table::new(root.to_path_buf());

        // Policies, as closures over (handler, home).
        let read_all = |_: Handler, _: &Path| SandboxPolicy {
            fs: vec![FsRule::new("/").unwrap().read(Allow)],
            net: Vec::new(),
        };
        let writable = |_: Handler, home: &Path| SandboxPolicy {
            fs: vec![
                FsRule::new("/").unwrap().read(Allow),
                FsRule::new(home.to_string_lossy()).unwrap().write(Allow),
            ],
            net: Vec::new(),
        };
        let proj_writable_secret_denied = |_: Handler, home: &Path| SandboxPolicy {
            fs: vec![
                FsRule::new("/").unwrap().read(Allow),
                FsRule::new(home.join("proj").to_string_lossy())
                    .unwrap()
                    .write(Allow),
                FsRule::new(home.join("proj/.git").to_string_lossy())
                    .unwrap()
                    .write(Deny),
                FsRule::new(home.join("secret").to_string_lossy())
                    .unwrap()
                    .read(Deny),
                FsRule::new("/**/*.env").unwrap().read(Deny),
            ],
            net: Vec::new(),
        };
        let ask_write = |handler: Handler, home: &Path| SandboxPolicy {
            fs: vec![
                FsRule::new("/").unwrap().read(Allow),
                FsRule::new(home.join("ask").to_string_lossy())
                    .unwrap()
                    .write(Access::Ask(handler)),
            ],
            net: Vec::new(),
        };
        let unix_policy = |_: Handler, home: &Path| SandboxPolicy {
            fs: vec![
                FsRule::new("/").unwrap().read(Allow),
                FsRule::new(home.to_string_lossy()).unwrap().write(Allow),
                FsRule::new(world.unix_allowed.to_string_lossy())
                    .unwrap()
                    .connect(Allow),
            ],
            net: Vec::new(),
        };
        let net_policy = |handler: Handler, _: &Path| SandboxPolicy {
            fs: vec![FsRule::new("/").unwrap().read(Allow)],
            net: vec![
                // No `tcp_rebinding` here: the name rule allows that port,
                // and the rebinding check must then deny the address it
                // resolves to.
                NetRule::new(NetTarget::Ip("127.0.0.1".parse().unwrap()), Allow)
                    .ports(PortSet::Ranges(vec![
                        world.tcp_allowed..=world.tcp_allowed,
                        world.tcp_closed..=world.tcp_closed,
                    ]))
                    .tcp_only(),
                NetRule::new(NetTarget::Ip("127.0.0.1".parse().unwrap()), Allow)
                    .ports(PortSet::Ranges(vec![world.udp_allowed..=world.udp_allowed]))
                    .udp_only(),
                NetRule::new(
                    NetTarget::Domain(DomainPattern::new("localhost").unwrap()),
                    Allow,
                )
                .ports(PortSet::Ranges(vec![
                    world.tcp_allowed..=world.tcp_allowed,
                    world.tcp_rebinding..=world.tcp_rebinding,
                ]))
                .tcp_only(),
                NetRule::new(NetTarget::Loopback, Access::Ask(handler.clone()))
                    .ports(PortSet::Ranges(vec![world.tcp_asked..=world.tcp_asked]))
                    .tcp_only(),
                NetRule::new(
                    NetTarget::Domain(DomainPattern::new("ask.test").unwrap()),
                    Access::Ask(handler),
                ),
            ],
        };
        // Enough for the loader and `sh` to run, nothing for the target.
        let loader_only = |_: Handler, _: &Path| SandboxPolicy {
            fs: vec![
                FsRule::new("/nix").unwrap().read(Allow),
                FsRule::new("/run/current-system").unwrap().read(Allow),
                FsRule::new("/dev/null").unwrap().read(Access::Allow),
            ],
            net: Vec::new(),
        };

        let allow = Ask::Immediate(Answer::Allow);

        // -- Filesystem: the sandbox holds up its side of the bargain --

        t.push(
            "read an allowed file",
            vec![file("proj/app.txt", "app")],
            allow.clone(),
            Action::Read {
                path: PathBuf::from("proj/app.txt"),
            },
            Expected::Allowed,
            read_all,
        );
        t.push(
            "read a file no rule allows",
            vec![file("proj/app.txt", "app")],
            allow.clone(),
            Action::Read {
                path: PathBuf::from("proj/app.txt"),
            },
            Expected::Denied,
            loader_only,
        );
        t.push(
            "write into a writable grant",
            vec![directory("proj")],
            allow.clone(),
            Action::Write {
                path: PathBuf::from("proj/new.txt"),
            },
            Expected::Allowed,
            writable,
        );
        t.after(vec![HostCheck::Contents {
            path: PathBuf::from("proj/new.txt"),
            equals: "content\n",
        }]);
        t.push(
            "a write-only policy still denies reads",
            vec![file("proj/app.txt", "app")],
            allow.clone(),
            Action::Read {
                path: PathBuf::from("proj/app.txt"),
            },
            Expected::Denied,
            loader_only,
        );
        t.push(
            "create directories, rename, link, delete inside a grant",
            vec![],
            allow.clone(),
            Action::Shell(
                "mkdir -p a/b/c && echo hi > a/f.txt && mv a/f.txt a/g.txt \
                 && ln -s g.txt a/link && echo x >> a/link && ln a/g.txt a/hard \
                 && rm -r a/b && cat a/g.txt a/hard && ls a && stat a/g.txt"
                    .to_string(),
            ),
            Expected::Allowed,
            writable,
        );
        t.push(
            "TMPDIR is writable",
            vec![],
            allow.clone(),
            Action::WriteTemp { file: "f.txt" },
            Expected::Allowed,
            writable,
        );
        t.push(
            "/dev/null, /dev/stderr and pipes are usable",
            vec![],
            allow.clone(),
            Action::Shell(
                "echo x > /dev/null && echo x > /dev/stderr && echo x > /dev/stdout".to_string(),
            ),
            Expected::Allowed,
            read_all,
        );
        t.push(
            "the command's own /proc entries are readable",
            vec![],
            allow.clone(),
            Action::ReadOwnProc,
            Expected::Allowed,
            read_all,
        );
        t.push(
            "the command has no capabilities",
            vec![],
            allow.clone(),
            Action::AssertNoCapabilities,
            Expected::Allowed,
            read_all,
        );
        t.push(
            "the command's namespaces are private",
            vec![],
            allow.clone(),
            Action::AssertNamespaces,
            Expected::Allowed,
            read_all,
        );
        t.push(
            "append to a file in a grant",
            vec![file("proj/log.txt", "line one\n")],
            allow.clone(),
            Action::Append {
                path: PathBuf::from("proj/log.txt"),
            },
            Expected::Allowed,
            writable,
        );
        t.after(vec![HostCheck::Contents {
            path: PathBuf::from("proj/log.txt"),
            equals: "line one\ncontent\n",
        }]);
        t.push(
            "create a directory in a grant",
            vec![],
            allow.clone(),
            Action::CreateDirectory {
                path: PathBuf::from("proj/a/b/c"),
            },
            Expected::Allowed,
            writable,
        );
        t.push(
            "list a denied directory is still allowed (listing is not reading)",
            vec![directory("secret")],
            allow.clone(),
            Action::List {
                path: PathBuf::from("secret"),
            },
            Expected::Allowed,
            proj_writable_secret_denied,
        );
        t.push(
            "stat a denied file is still allowed",
            vec![file("proj/.git/config", "original")],
            allow.clone(),
            Action::Stat {
                path: PathBuf::from("proj/.git/config"),
            },
            Expected::Allowed,
            proj_writable_secret_denied,
        );
        t.push(
            "symlink a denied target is allowed (writing the link, not reading it)",
            vec![file("secret/key", "password"), directory("proj")],
            allow.clone(),
            Action::Symlink {
                target: PathBuf::from("secret/key"),
                link: PathBuf::from("proj/link"),
            },
            Expected::Allowed,
            proj_writable_secret_denied,
        );
        t.push(
            "a symlink into a grant lets a write through",
            vec![
                directory("outside"),
                directory("proj"),
                symlink("proj", "outside/qux"),
            ],
            allow.clone(),
            Action::Write {
                path: PathBuf::from("outside/qux/via.txt"),
            },
            Expected::Allowed,
            proj_writable_secret_denied,
        );
        t.after(vec![HostCheck::Contents {
            path: PathBuf::from("proj/via.txt"),
            equals: "content\n",
        }]);

        // -- Filesystem: escapes must fail --

        t.push(
            "read a denied file",
            vec![file("secret/key", "password")],
            allow.clone(),
            Action::Open {
                path: PathBuf::from("secret/key"),
                mode: "r",
            },
            Expected::Errno(&[libc::EACCES]),
            proj_writable_secret_denied,
        );
        t.reported(RequestedAccess::Fs {
            permission: Permission::Read,
            path: PathBuf::from("secret/key"),
        });
        t.push(
            "write a protected file",
            vec![file("proj/.git/config", "original")],
            allow.clone(),
            Action::Open {
                path: PathBuf::from("proj/.git/config"),
                mode: "w",
            },
            Expected::Errno(&[libc::EACCES, libc::EROFS]),
            proj_writable_secret_denied,
        );
        t.reported(RequestedAccess::Fs {
            permission: Permission::Write,
            path: PathBuf::from("proj/.git/config"),
        });
        t.after(vec![HostCheck::Contents {
            path: PathBuf::from("proj/.git/config"),
            equals: "original",
        }]);
        t.push(
            "delete a protected file",
            vec![file("proj/.git/config", "original")],
            allow.clone(),
            Action::Delete {
                path: PathBuf::from("proj/.git/config"),
            },
            Expected::Denied,
            proj_writable_secret_denied,
        );
        t.push(
            "rename a protected directory",
            vec![directory("proj/.git")],
            allow.clone(),
            Action::Rename {
                from: PathBuf::from("proj/.git"),
                to: PathBuf::from("proj/gitx"),
            },
            Expected::Denied,
            proj_writable_secret_denied,
        );
        t.push(
            "read a .env file (glob-deny)",
            vec![file("proj/app.env", "API_KEY=hunter2")],
            allow.clone(),
            Action::Read {
                path: PathBuf::from("proj/app.env"),
            },
            Expected::Denied,
            proj_writable_secret_denied,
        );
        t.push(
            "read a .env through a symlink (glob-deny)",
            vec![file("proj/app.env", "API_KEY=hunter2")],
            allow.clone(),
            Action::Shell("ln -sf app.env sym.txt && cat sym.txt".to_string()),
            Expected::Denied,
            proj_writable_secret_denied,
        );
        t.push(
            "copy a .env file",
            vec![file("proj/app.env", "API_KEY=hunter2")],
            allow.clone(),
            Action::Shell("cp app.env copy.txt".to_string()),
            Expected::Denied,
            proj_writable_secret_denied,
        );
        t.push(
            "rename a .env file",
            vec![file("proj/app.env", "API_KEY=hunter2")],
            allow.clone(),
            Action::Rename {
                from: PathBuf::from("proj/app.env"),
                to: PathBuf::from("proj/app.txt"),
            },
            Expected::Denied,
            proj_writable_secret_denied,
        );
        t.push(
            "hard link a .env file out of its rule",
            vec![file("proj/app.env", "API_KEY=hunter2")],
            allow.clone(),
            Action::HardLink {
                target: PathBuf::from("proj/app.env"),
                link: PathBuf::from("proj/hard.txt"),
            },
            Expected::Denied,
            proj_writable_secret_denied,
        );
        t.push(
            "write outside the grant",
            vec![directory("outside")],
            allow.clone(),
            Action::Open {
                path: PathBuf::from("outside/new.txt"),
                mode: "wc",
            },
            Expected::Errno(&[libc::EACCES, libc::EROFS]),
            proj_writable_secret_denied,
        );
        t.reported(RequestedAccess::Fs {
            permission: Permission::Write,
            path: PathBuf::from("outside/new.txt"),
        });
        t.push(
            "write through .. to a sibling",
            vec![],
            allow.clone(),
            Action::Shell("mkdir -p sub && echo x > sub/../../outside/h".to_string()),
            Expected::Denied,
            proj_writable_secret_denied,
        );
        t.push(
            "read a host process's environment",
            vec![],
            allow.clone(),
            Action::ReadHostEnvironment,
            Expected::Denied,
            read_all,
        );
        t.push(
            "signal a host process",
            vec![],
            allow.clone(),
            Action::SignalHost,
            Expected::Denied,
            read_all,
        );
        t.push(
            "create a new user namespace",
            vec![],
            allow.clone(),
            Action::NewUserNamespace,
            Expected::Denied,
            read_all,
        );

        // -- Ask: the syscall blocks until answered --

        t.push(
            "an asked write, answered late, still succeeds",
            vec![directory("ask")],
            Ask::Delayed(ASK_DELAY, Answer::Allow),
            Action::Write {
                path: PathBuf::from("ask/slow.txt"),
            },
            Expected::Allowed,
            ask_write,
        );
        t.expected_prompts(1);
        t.blocked_for(ASK_DELAY - TIMING_SLACK);
        t.after(vec![HostCheck::Contents {
            path: PathBuf::from("ask/slow.txt"),
            equals: "content\n",
        }]);

        t.push(
            "an asked write, answered late with a denial, fails",
            vec![directory("ask")],
            Ask::Delayed(ASK_DELAY, Answer::Deny),
            Action::Write {
                path: PathBuf::from("ask/no.txt"),
            },
            Expected::Denied,
            ask_write,
        );
        t.expected_prompts(1);
        t.blocked_for(ASK_DELAY - TIMING_SLACK);
        t.reported(RequestedAccess::Fs {
            permission: Permission::Write,
            path: PathBuf::from("ask/no.txt"),
        });
        t.after(vec![HostCheck::Absent(PathBuf::from("ask/no.txt"))]);

        t.push(
            "a remembered rule covers a later write",
            vec![directory("ask")],
            Ask::Delayed(ASK_DELAY, Answer::Remember("remember*".to_string())),
            Action::Shell("echo x > ask/remember-a.txt && echo x > ask/remember-b.txt".to_string()),
            Expected::Allowed,
            ask_write,
        );
        t.expected_prompts(1);
        t.blocked_for(ASK_DELAY - TIMING_SLACK);
        t.after(vec![
            HostCheck::Contents {
                path: PathBuf::from("ask/remember-a.txt"),
                equals: "x\n",
            },
            HostCheck::Contents {
                path: PathBuf::from("ask/remember-b.txt"),
                equals: "x\n",
            },
        ]);

        t.push(
            "a panicking handler denies",
            vec![directory("ask")],
            Ask::Immediate(Answer::Panic),
            Action::Write {
                path: PathBuf::from("ask/panic.txt"),
            },
            Expected::Denied,
            ask_write,
        );
        t.expected_prompts(1);
        t.after(vec![HostCheck::Absent(PathBuf::from("ask/panic.txt"))]);

        // -- Unix sockets --

        t.push(
            "connect to an allowed unix socket",
            vec![],
            allow.clone(),
            Action::ConnectUnix {
                path: world.unix_allowed.clone(),
            },
            Expected::Allowed,
            unix_policy,
        );
        t.push(
            "connect to another unix socket is refused",
            vec![],
            allow.clone(),
            Action::ConnectUnix {
                path: world.unix_denied.clone(),
            },
            Expected::Errno(&[libc::EACCES]),
            unix_policy,
        );
        t.push(
            "connect through a symlink to a refused socket",
            vec![],
            allow.clone(),
            Action::Shell(format!(
                "ln -s {} link.sock && python3 net.py unix-connect link.sock EACCES",
                shell(&world.unix_denied)
            )),
            Expected::Allowed,
            unix_policy,
        );
        t.push(
            "serve on a unix socket in TMPDIR",
            vec![],
            allow.clone(),
            Action::UnixServer {
                file: "server.sock",
            },
            Expected::Allowed,
            unix_policy,
        );
        t.push(
            "bind a unix socket in a writable directory",
            vec![],
            allow.clone(),
            Action::BindUnix {
                path: PathBuf::from("bound.sock"),
            },
            Expected::Allowed,
            unix_policy,
        );
        t.after(vec![HostCheck::Socket(PathBuf::from("bound.sock"))]);
        t.push(
            "bind a unix socket outside the grant is refused",
            vec![],
            allow.clone(),
            Action::BindUnix {
                path: world.unix_denied_bind.clone(),
            },
            Expected::Errno(&[libc::EACCES, libc::EROFS]),
            unix_policy,
        );
        t.push(
            "serve on an abstract unix socket (namespace-local)",
            vec![],
            allow.clone(),
            Action::AbstractUnixServer {
                name: "zed-sandbox-test",
            },
            Expected::Allowed,
            unix_policy,
        );
        t.push(
            "stream socketpairs work",
            vec![],
            allow.clone(),
            Action::SocketPair {
                kind: "SOCK_STREAM",
            },
            Expected::Allowed,
            unix_policy,
        );
        t.push(
            "seqpacket socketpairs work",
            vec![],
            allow.clone(),
            Action::SocketPair {
                kind: "SOCK_SEQPACKET",
            },
            Expected::Allowed,
            unix_policy,
        );
        t.push(
            "datagram unix sockets fail with EPERM",
            vec![],
            allow.clone(),
            Action::DatagramUnix,
            Expected::Errno(&[libc::EPERM]),
            unix_policy,
        );

        // -- Network --

        t.push(
            "tcp to an allowed port",
            vec![],
            allow.clone(),
            Action::Connect {
                host: "127.0.0.1".to_string(),
                port: world.tcp_allowed,
            },
            Expected::Allowed,
            net_policy,
        );
        t.push(
            "non-blocking tcp to an allowed port",
            vec![],
            allow.clone(),
            Action::ConnectNonBlocking {
                host: "127.0.0.1".to_string(),
                port: world.tcp_allowed,
            },
            Expected::Allowed,
            net_policy,
        );
        t.push(
            "blocking tcp to an allowed closed port is refused",
            vec![],
            allow.clone(),
            Action::Connect {
                host: "127.0.0.1".to_string(),
                port: world.tcp_closed,
            },
            Expected::Errno(&[libc::ECONNREFUSED]),
            net_policy,
        );
        t.push(
            "non-blocking tcp to an allowed closed port is reset",
            vec![],
            allow.clone(),
            Action::ConnectNonBlocking {
                host: "127.0.0.1".to_string(),
                port: world.tcp_closed,
            },
            Expected::Reset,
            net_policy,
        );
        t.push(
            "tcp to an unlisted port is refused with EACCES",
            vec![],
            allow.clone(),
            Action::Connect {
                host: "127.0.0.1".to_string(),
                port: world.tcp_unlisted,
            },
            Expected::Errno(&[libc::EACCES]),
            net_policy,
        );
        t.reported(RequestedAccess::Net {
            host: NetHost::Ip("127.0.0.1".parse().unwrap()),
            port: world.tcp_unlisted,
            protocol: Protocol::Tcp,
        });
        t.push(
            "tcp to a public address is refused with EACCES",
            vec![],
            allow.clone(),
            Action::Connect {
                host: "1.1.1.1".to_string(),
                port: 80,
            },
            Expected::Errno(&[libc::EACCES]),
            net_policy,
        );
        t.reported(RequestedAccess::Net {
            host: NetHost::Ip("1.1.1.1".parse().unwrap()),
            port: 80,
            protocol: Protocol::Tcp,
        });
        t.push(
            "tcp to an unknown fake address is refused",
            vec![],
            allow.clone(),
            Action::Connect {
                host: "198.18.255.254".to_string(),
                port: 80,
            },
            Expected::Errno(&[libc::ECONNREFUSED]),
            net_policy,
        );
        t.push(
            "tcp within the sandbox needs no rule",
            vec![],
            allow.clone(),
            Action::LocalServer,
            Expected::Allowed,
            net_policy,
        );
        t.push(
            "tcp to an allowed name",
            vec![],
            allow.clone(),
            Action::ConnectByName {
                name: "localhost",
                port: world.tcp_allowed,
            },
            Expected::Allowed,
            net_policy,
        );
        t.push(
            "tcp to an allowed name resolving to an ip:port no rule allows (rebinding)",
            vec![],
            allow.clone(),
            Action::ConnectByName {
                name: "localhost",
                port: world.tcp_rebinding,
            },
            Expected::Errno(&[libc::EACCES]),
            net_policy,
        );
        t.reported(RequestedAccess::Net {
            host: NetHost::Ip("127.0.0.1".parse().unwrap()),
            port: world.tcp_rebinding,
            protocol: Protocol::Tcp,
        });
        let asked_name_policy = |handler: Handler, _: &Path| SandboxPolicy {
            fs: vec![FsRule::new("/").unwrap().read(Allow)],
            net: vec![
                // The fake IP maps back to the name, so the Ask rule must be a
                // Domain rule (names only match Domain/Any targets), and last,
                // so it wins on the asked port.
                NetRule::new(
                    NetTarget::Domain(DomainPattern::new("localhost").unwrap()),
                    Allow,
                )
                .ports(PortSet::Ranges(vec![
                    world.tcp_allowed..=world.tcp_allowed,
                    world.tcp_rebinding..=world.tcp_rebinding,
                ]))
                .tcp_only(),
                NetRule::new(
                    NetTarget::Domain(DomainPattern::new("localhost").unwrap()),
                    Access::Ask(handler),
                )
                .ports(PortSet::Ranges(vec![world.tcp_asked..=world.tcp_asked]))
                .tcp_only(),
                // `localhost` resolves to loopback, which the rebinding check
                // requires its own rule for.
                NetRule::new(NetTarget::Ip("127.0.0.1".parse().unwrap()), Allow).tcp_only(),
            ],
        };
        t.push(
            "an asked name, answered late, connects",
            vec![],
            Ask::Delayed(ASK_DELAY, Answer::Allow),
            Action::ConnectByName {
                name: "localhost",
                port: world.tcp_asked,
            },
            Expected::Allowed,
            asked_name_policy,
        );
        t.expected_prompts(1);
        t.blocked_for(ASK_DELAY - TIMING_SLACK);

        t.push(
            "udp to an allowed port, connected",
            vec![],
            allow.clone(),
            Action::Udp {
                host: "127.0.0.1".to_string(),
                port: world.udp_allowed,
                connected: true,
            },
            Expected::Allowed,
            net_policy,
        );
        t.push(
            "udp to an allowed port, sendto",
            vec![],
            allow.clone(),
            Action::Udp {
                host: "127.0.0.1".to_string(),
                port: world.udp_allowed,
                connected: false,
            },
            Expected::Allowed,
            net_policy,
        );
        t.push(
            "udp to an unlisted port is blocked",
            vec![],
            allow.clone(),
            Action::Udp {
                host: "127.0.0.1".to_string(),
                port: world.udp_unlisted,
                connected: true,
            },
            Expected::Denied,
            net_policy,
        );
        t.reported(RequestedAccess::Net {
            host: NetHost::Ip("127.0.0.1".parse().unwrap()),
            port: world.udp_unlisted,
            protocol: Protocol::Udp,
        });

        t.push(
            "dns: an allowed name resolves to a fake address",
            vec![],
            allow.clone(),
            Action::Dns { name: "localhost" },
            Expected::FakeAddress,
            net_policy,
        );
        t.push(
            "dns: an asked name resolves",
            vec![],
            Ask::Immediate(Answer::Allow),
            Action::Dns { name: "ask.test" },
            Expected::FakeAddress,
            net_policy,
        );
        t.push(
            "dns: an unlisted name does not resolve",
            vec![],
            allow.clone(),
            Action::Dns {
                name: "unlisted.test",
            },
            Expected::NxDomain,
            net_policy,
        );
        t.push(
            "dns: AAAA answers are empty",
            vec![],
            allow.clone(),
            Action::Dns { name: "localhost" },
            Expected::EmptyRecord,
            net_policy,
        );
        t.push(
            "getaddrinfo goes through the sandbox's DNS",
            vec![],
            allow.clone(),
            Action::Resolve { name: "ask.test" },
            Expected::FakeAddress,
            net_policy,
        );
        t.push(
            "ping sockets fail",
            vec![],
            allow,
            Action::Ping,
            Expected::Errno(&[libc::EACCES]),
            net_policy,
        );

        t.scenarios
    }

    /// Run one scenario and report whether it held.
    fn run_scenario(
        context: &SandboxContext,
        scenario: &Scenario,
        ctx: &Ctx,
        failures: &mut Vec<String>,
    ) {
        // Fixtures, in their scenario's fresh directory.
        std::fs::create_dir_all(&scenario.home).expect("creating scenario home");
        for fixture in &scenario.fixtures {
            match fixture {
                Fixture::File { path, contents } => {
                    let path = scenario.home.join(path);
                    std::fs::create_dir_all(path.parent().expect("a parent")).unwrap();
                    std::fs::write(&path, contents)
                        .unwrap_or_else(|error| panic!("writing {}: {error}", path.display()));
                }
                Fixture::Directory { path } => {
                    std::fs::create_dir_all(scenario.home.join(path)).unwrap();
                }
                Fixture::Symlink { target, link } => {
                    let link = scenario.home.join(link);
                    std::fs::create_dir_all(link.parent().expect("a parent")).unwrap();
                    std::os::unix::fs::symlink(scenario.home.join(target), link)
                        .unwrap_or_else(|error| panic!("linking: {error}"));
                }
            }
        }

        let sandbox = context
            .create_sandbox(scenario.policy.clone())
            .expect("creating the scenario's sandbox");
        sandbox.on_denial({
            let denials = scenario.denials.clone();
            move |denial| denials.lock().unwrap().push(denial)
        });

        std::fs::write(scenario.home.join("net.py"), NET_CLIENT).unwrap();
        let script = scenario.action.script(ctx, &scenario.expected);
        let started = Instant::now();
        let mut prepared = sandbox
            .prepare(&command(&script, &scenario.home))
            .expect("preparing command");
        prepared.command_mut().stdin(Stdio::null());
        let output = prepared.output().expect("running the scenario");
        let elapsed = started.elapsed();

        let mut notes = Vec::new();
        if !scenario.expected.holds(&output) {
            notes.push(format!(
                "expected {}, got {} (stderr: {})",
                match &scenario.expected {
                    Expected::Allowed | Expected::FakeAddress | Expected::EmptyRecord => {
                        "success".to_string()
                    }
                    other => format!("{other:?}"),
                },
                output.status,
                String::from_utf8_lossy(&output.stderr).trim(),
            ));
        }
        if let Some(at_least) = scenario.blocked_for
            && elapsed < at_least
        {
            notes.push(format!(
                "expected the action to block for at least {at_least:?}, it took {elapsed:?}"
            ));
        }
        let prompts = scenario.prompts.load(Ordering::SeqCst);
        if let Some(expected) = scenario.expected_prompts
            && prompts != expected
        {
            notes.push(format!("expected {expected} prompt(s), saw {prompts}"));
        }
        if let Some(reported) = &scenario.reported {
            let found = scenario
                .denials
                .lock()
                .unwrap()
                .iter()
                .any(|denial| denial.requests.contains(reported));
            if !found {
                notes.push(format!("no denial reported for {reported:?}"));
            }
        }
        for check in &scenario.host_checks {
            let path = scenario.home.join(check_path(check));
            let held = match check {
                HostCheck::Contents { equals, .. } => std::fs::read_to_string(&path)
                    .map(|contents| contents == *equals)
                    .unwrap_or(false),
                HostCheck::Absent(_) => !path.exists(),
                HostCheck::Socket(_) => path
                    .symlink_metadata()
                    .map(|metadata| metadata.file_type().is_socket())
                    .unwrap_or(false),
            };
            if !held {
                notes.push(format!(
                    "host check failed: {}",
                    check_path(check).display()
                ));
            }
        }

        drop(sandbox);
        let passed = notes.is_empty();
        println!("{:4} {}", if passed { "ok" } else { "FAIL" }, scenario.name);
        if !passed {
            failures.push(format!("{}: {}", scenario.name, notes.join("; ")));
        }
    }

    fn check_path(check: &HostCheck) -> PathBuf {
        match check {
            HostCheck::Contents { path, .. } => path.clone(),
            HostCheck::Absent(path) | HostCheck::Socket(path) => path.clone(),
        }
    }

    fn command(script: &str, cwd: &Path) -> UntrustedCommand {
        UntrustedCommand {
            cmd: "sh".into(),
            args: vec!["-c".into(), script.into()],
            env: std::env::vars()
                // The untrusted command needs `sh` and its loader chain to
                // run at all; everything else is its own business.
                .filter(|(name, _)| {
                    matches!(
                        name.as_str(),
                        "PATH"
                            | "HOME"
                            | "LD_LIBRARY_PATH"
                            | "LD_PRELOAD"
                            | "NIX_LD"
                            | "NIX_LD_LIBRARY_PATH"
                    )
                })
                .chain([("HOST_PID".to_string(), std::process::id().to_string())])
                .collect(),
            cwd: cwd.to_path_buf(),
        }
    }

    /// The client script the network and socket actions run.
    const NET_CLIENT: &str = r#"
import asyncio, errno, ipaddress, os, random, select, socket, struct, sys

FAKE = ipaddress.ip_network("198.18.0.0/15")

def echo(stream):
    stream.sendall(b"ping")
    try:
        return stream.recv(4) == b"ping"
    except BlockingIOError:
        # A non-blocking socket: wait for the reply first.
        assert select.select([stream], [], [], 10)[0], "no echo reply"
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
    address = (
        socket.inet_ntoa(response[-4:])
        if answers and record_type == 1
        else None
    )
    return code, address

def main(command, *args):
    if command == "tcp":
        host, port = args[0], int(args[1])
        expected = [getattr(errno, name) for name in args[2:]]
        # Blocking: the sandbox reports the host leg's exact failure to a
        # blocking connect, and the echo reply arrives in its own time.
        stream = socket.socket()
        try:
            stream.connect((host, port))
        except OSError as error:
            assert error.errno in expected, error
            return True
        assert not expected, "connect succeeded: expected " + ", ".join(args[2:])
        return echo(stream) and stream.getpeername()[1] == port
    if command == "tcp-nonblocking":
        host, port = args[0], int(args[1])
        if "reset" in args[2:]:
            # Succeed locally, then the host leg's refusal shows as a reset.
            stream = socket.socket()
            stream.setblocking(False)
            result = stream.connect_ex((host, port))
            assert result == 0, os.strerror(result)
            assert select.select([stream], [], [], 5)[0]
            try:
                stream.recv(1)
            except ConnectionResetError:
                return True
            return False
        expected = [getattr(errno, name) for name in args[2:]]
        stream = socket.socket()
        stream.setblocking(False)
        result = stream.connect_ex((host, port))
        if result != 0:
            assert result in expected, os.strerror(result)
            return True
        assert not expected, "connect succeeded: expected " + ", ".join(args[2:])
        return echo(stream)
    if command == "tcp-name":
        name, port = args[0], int(args[1])
        expected = [getattr(errno, name) for name in args[2:]]
        code, address = dns(name)
        assert code == 0, code
        stream = socket.socket()
        try:
            stream.connect((address, port))
        except OSError as error:
            assert error.errno in expected, error
            return True
        assert not expected, "connect succeeded: expected " + ", ".join(args[2:])
        return echo(stream)
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
    if command == "dns":
        name, verdict, record = args[0], args[1], int(args[2])
        code, address = dns(name, record)
        if verdict == "nxdomain":
            return code == 3
        if verdict == "empty":
            return code == 0 and address is None
        return (
            code == 0
            and ipaddress.ip_address(address) in FAKE
        )
    if command == "resolve":
        # getaddrinfo: the sandbox's DNS answers.
        return ipaddress.ip_address(socket.gethostbyname(args[0])) in FAKE
    if command == "ping-socket":
        expected = [getattr(errno, name) for name in args]
        try:
            socket.socket(socket.AF_INET, socket.SOCK_DGRAM, socket.IPPROTO_ICMP)
        except OSError as error:
            assert error.errno in expected, error
            return True
        raise AssertionError("ping socket allowed")
    if command == "unix-connect":
        path = args[0]
        expected = [getattr(errno, name) for name in args[1:]]
        stream = socket.socket(socket.AF_UNIX)
        stream.settimeout(5)
        try:
            stream.connect(path)
        except OSError as error:
            assert error.errno in expected, error
            return True
        assert not expected, "connect succeeded: expected " + ", ".join(args[1:])
        return echo(stream)
    if command == "unix-bind":
        path = args[0]
        expected = [getattr(errno, name) for name in args[1:]]
        try:
            socket.socket(socket.AF_UNIX).bind(path)
        except OSError as error:
            assert error.errno in expected, error
            return True
        assert not expected, "bind succeeded: expected " + ", ".join(args[1:])
        return True
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
    if command == "socketpair":
        kind = getattr(socket, args[0])
        first, second = socket.socketpair(socket.AF_UNIX, kind)
        first.send(b"hi")
        return second.recv(2) == b"hi"
    if command == "datagram-unix":
        # Both creation paths must fail with EPERM.
        for operation in (
            lambda: socket.socket(socket.AF_UNIX, socket.SOCK_DGRAM),
            lambda: socket.socketpair(socket.AF_UNIX, socket.SOCK_DGRAM),
        ):
            try:
                operation()
            except OSError as error:
                assert error.errno == errno.EPERM, error
            else:
                raise AssertionError("datagram unix socket allowed")
        return True
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

    /// A TCP socket bound to a free loopback port but never listened on:
    /// connecting to it is deterministically refused.
    fn bound_not_listening() -> (u16, std::os::fd::OwnedFd) {
        use std::os::fd::{AsRawFd as _, FromRawFd as _};
        let mut address = libc::sockaddr_in {
            sin_family: libc::AF_INET as libc::sa_family_t,
            sin_port: 0,
            sin_addr: libc::in_addr {
                // `INADDR_LOOPBACK` is defined in host byte order.
                s_addr: libc::htonl(libc::INADDR_LOOPBACK),
            },
            sin_zero: [0; 8],
        };
        // SAFETY: plain socket call; the fresh descriptor is owned below.
        let descriptor =
            unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM | libc::SOCK_CLOEXEC, 0) };
        assert!(descriptor >= 0, "socket for the closed port");
        let socket = unsafe { std::os::fd::OwnedFd::from_raw_fd(descriptor) };
        // SAFETY: a sockaddr_in we own, with its length.
        let bound = unsafe {
            libc::bind(
                socket.as_raw_fd(),
                (&mut address as *mut libc::sockaddr_in).cast(),
                std::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t,
            )
        };
        assert!(
            bound >= 0,
            "binding the closed port: {}",
            std::io::Error::last_os_error()
        );
        // SAFETY: getsockname fills the same sockaddr_in.
        let mut length = std::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t;
        let named = unsafe {
            libc::getsockname(
                socket.as_raw_fd(),
                (&mut address as *mut libc::sockaddr_in).cast(),
                &mut length,
            )
        };
        assert!(named >= 0, "naming the closed port");
        (u16::from_be(address.sin_port), socket)
    }

    /// Build the world of peers and free ports under `root`.
    fn world(root: &Path) -> World {
        // A socket bound but never listened on: connecting is deterministically
        // refused. (`TcpListener::bind` listens, so its backlog would accept.)
        let (closed_port, closed) = bound_not_listening();
        std::fs::create_dir_all(root).unwrap();
        let sockets = root.join("sockets");
        std::fs::create_dir_all(&sockets).unwrap();
        let unix_allowed = sockets.join("allowed.sock");
        let unix_denied = sockets.join("denied.sock");
        let unix_denied_bind = root.parent().unwrap().join("denied-bind.sock");
        unix_echo_server(&unix_allowed);
        unix_echo_server(&unix_denied);
        World {
            _closed: closed,
            tcp_allowed: tcp_echo_server(),
            tcp_asked: tcp_echo_server(),
            tcp_unlisted: tcp_echo_server(),
            tcp_closed: closed_port,
            tcp_rebinding: tcp_echo_server(),
            udp_allowed: udp_echo_server(),
            udp_unlisted: udp_echo_server(),
            unix_allowed,
            unix_denied,
            unix_denied_bind,
        }
    }

    /// This harness's single test, as `cargo nextest` lists and runs it.
    const TEST_NAME: &str = "end_to_end";

    pub fn main() {
        let arguments: Vec<_> = std::env::args().skip(1).collect();
        // `cargo nextest` drives `harness = false` binaries with its own
        // protocol: `--list --format terse [--ignored]` to discover tests, then
        // `<test-name> --exact [--nocapture]` to run each one. This harness is
        // a single unit: one test, none ignored. `cargo test` and the NixOS
        // checks instead run it bare, or with the flags below.
        if arguments.iter().any(|argument| argument == "--list") {
            if !arguments.iter().any(|argument| argument == "--ignored") {
                println!("{TEST_NAME}: test");
            }
            return;
        }
        let (mut require_sandbox, mut expect_userns_disabled) = (false, false);
        for argument in &arguments {
            match argument.as_str() {
                "--require-sandbox" => require_sandbox = true,
                "--expect-userns-disabled" => expect_userns_disabled = true,
                TEST_NAME | "--nocapture" | "--exact" | "--bench" => {}
                other => panic!("unexpected argument {other}"),
            }
        }
        let (require_sandbox, expect_userns_disabled) = (require_sandbox, expect_userns_disabled);
        let fixtures = tempfile::Builder::new()
            .prefix("sandbox-escape-test-")
            .tempdir_in("/tmp")
            .expect("creating work dir");
        let root = fixtures.path().to_path_buf();

        let context = SandboxContext::new().expect("starting supervisor");
        let ctx = Ctx {
            namespaces: ["user", "mnt", "net", "ipc"]
                .into_iter()
                .map(|namespace| {
                    (
                        namespace,
                        std::fs::read_link(format!("/proc/self/ns/{namespace}"))
                            .expect("reading a namespace")
                            .to_string_lossy()
                            .into_owned(),
                    )
                })
                .collect(),
        };
        assert!(
            std::process::Command::new("sh")
                .args([
                    "-c",
                    "for tool in python3 unshare cat chmod cp grep kill ln ls mkdir mv rm sleep stat truncate; do command -v \"$tool\" >/dev/null || exit 1; done",
                ])
                .status()
                .expect("checking test dependencies")
                .success(),
            "missing test dependencies"
        );

        let world = world(&root);
        let scenarios = table(&root, &world);
        let mut failures = Vec::new();

        // A probe first: whether the sandbox can start decides the run.
        std::fs::create_dir_all(root.join("probe")).unwrap();
        let probe = {
            let sandbox = context
                .create_sandbox(SandboxPolicy {
                    fs: vec![FsRule::new("/").unwrap().read(Allow)],
                    net: Vec::new(),
                })
                .expect("creating the probe sandbox");
            let mut prepared = sandbox
                .prepare(&command("true", &root.join("probe")))
                .expect("preparing the probe");
            prepared.command_mut().stdin(Stdio::null());
            prepared.output().expect("running the probe")
        };
        if expect_userns_disabled {
            assert!(!probe.status.success(), "sandbox unexpectedly started");
            assert!(
                String::from_utf8_lossy(&probe.stderr)
                    .contains("creating sandbox namespaces failed"),
                "expected namespace setup failure, got: {probe:?}"
            );
            println!("ok   unavailable user namespaces fail closed");
            return;
        }
        if !probe.status.success() {
            let stderr = String::from_utf8_lossy(&probe.stderr);
            if !require_sandbox && stderr.contains("creating sandbox namespaces failed") {
                eprintln!("skipping sandbox_linux end-to-end tests: {}", stderr.trim());
                return;
            }
            panic!("sandbox probe failed: {stderr}");
        }

        for scenario in &scenarios {
            run_scenario(&context, scenario, &ctx, &mut failures);
        }

        // -- Sequence checks, which orchestrate rather than act --

        check_stop(&context, &root, &mut failures);
        check_policy_update(&context, &root, &mut failures);
        check_early_connect(&context, &root, &mut failures);

        drop(context);
        if !failures.is_empty() {
            eprintln!("\n{} failure(s):\n{}", failures.len(), failures.join("\n"));
            panic!("sandbox isolation checks failed");
        }
        println!("\nall checks passed");
    }

    /// `SIGTERM` to the helper stops the command and everything it spawned.
    fn check_stop(context: &SandboxContext, root: &Path, failures: &mut Vec<String>) {
        let home = root.join("sequence").join("stop");
        std::fs::create_dir_all(&home).unwrap();
        let sandbox = context
            .create_sandbox(SandboxPolicy {
                fs: vec![
                    FsRule::new("/").unwrap().read(Allow),
                    FsRule::new("/").unwrap().write(Allow),
                ],
                net: Vec::new(),
            })
            .unwrap();
        let pid_file = sandbox.temp_dir().join("background.pid");
        let script = format!("sleep 300 & echo $! > {}; wait", pid_file.display());
        let mut prepared = sandbox
            .prepare(&command(&script, &home))
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
        let passed =
            status.and_then(|status| status.code()) == Some(128 + libc::SIGTERM) && background_gone;
        println!(
            "{:4} stopping kills the command and its children",
            if passed { "ok" } else { "FAIL" }
        );
        if !passed {
            failures.push("stopping kills the command and its children".to_string());
        }
    }

    /// Narrow the policy while a command holds a file open for writing: new
    /// writes fail, and the open file is reported.
    fn check_policy_update(context: &SandboxContext, root: &Path, failures: &mut Vec<String>) {
        let home = root.join("sequence").join("policy");
        std::fs::create_dir_all(&home).unwrap();
        let base = SandboxPolicy {
            fs: vec![
                FsRule::new("/").unwrap().read(Allow),
                FsRule::new("/").unwrap().write(Allow),
            ],
            net: Vec::new(),
        };
        let sandbox = context.create_sandbox(base.clone()).unwrap();
        let held = home.join("held.txt");
        let mut prepared = sandbox
            .prepare(&command("exec 3>>held.txt; read line", &home))
            .expect("preparing command");
        prepared.command_mut().stdin(Stdio::piped());
        let mut holder = prepared.spawn().expect("spawning holder");
        let deadline = Instant::now() + Duration::from_secs(10);
        while !held.exists() && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(20));
        }

        let mut narrowed = base.clone();
        narrowed.fs.push(FsRule::new("/").unwrap().write(Deny));
        let still_open = sandbox.update_policy(&narrowed).expect("updating policy");
        let narrowed_reported = still_open
            .iter()
            .any(|open| open.path == held && open.write && !open.read);

        let mut check = |name: &'static str, script: &str, expected: bool| {
            let mut prepared = sandbox
                .prepare(&command(script, &home))
                .expect("preparing command");
            prepared.command_mut().stdin(Stdio::null());
            let output = prepared.output().expect("running the check");
            let passed = output.status.success() == expected;
            println!("{:4} {name}", if passed { "ok" } else { "FAIL" });
            if !passed {
                failures.push(format!(
                    "{name}: expected {}, got {} (stderr: {})",
                    if expected { "success" } else { "failure" },
                    output.status,
                    String::from_utf8_lossy(&output.stderr).trim(),
                ));
            }
        };
        check(
            "write after narrowing",
            "echo x > after-narrowing.txt",
            false,
        );
        drop(holder.stdin.take());
        holder.wait().ok();
        sandbox.update_policy(&base).expect("restoring policy");
        check(
            "write after restoring",
            "echo x > after-restoring.txt",
            true,
        );

        let passed = narrowed_reported;
        println!(
            "{:4} narrowing reports the held file",
            if passed { "ok" } else { "FAIL" }
        );
        if !passed {
            failures.push("narrowing reports the held file".to_string());
        }
    }

    /// Hold the host-leg prompt open: the app must get its local connection
    /// (and run another event-loop task) before we approve the host leg, and
    /// ending the execution cancels the in-flight request.
    fn check_early_connect(context: &SandboxContext, root: &Path, failures: &mut Vec<String>) {
        use std::io::{BufRead as _, BufReader};
        use std::sync::mpsc;

        let home = root.join("sequence").join("early-connect");
        std::fs::create_dir_all(&home).unwrap();
        let port = tcp_echo_server();
        for cancelled in [false, true] {
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
            // Use the sandbox's raw DNS so the connect goes through the
            // host-leg prompt (`localhost` from /etc/hosts wouldn't).
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
            let mut prepared = sandbox
                .prepare(&command(&script, &home))
                .expect("preparing command");
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
            let name = if cancelled {
                "early connect is cancelled when its execution ends"
            } else {
                "event loop progresses before the host connection is approved"
            };
            if cancelled {
                // SAFETY: plain kill of our own, not yet reaped, child.
                unsafe { libc::kill(helper.id() as i32, libc::SIGTERM) };
                let status = helper.wait().unwrap();
                let deadline = Instant::now() + Duration::from_secs(5);
                while !responder.is_cancelled() && Instant::now() < deadline {
                    std::thread::sleep(Duration::from_millis(20));
                }
                let passed = responsive && status.code() == Some(143) && responder.is_cancelled();
                println!("{:4} {name}", if passed { "ok" } else { "FAIL" });
                if !passed {
                    failures.push(name.to_string());
                }
                responder.respond(Decision::deny());
            } else {
                responder.respond(Decision::allow());
                let result = helper.wait_with_output().unwrap();
                let passed = responsive && result.status.success();
                println!("{:4} {name}", if passed { "ok" } else { "FAIL" });
                if !passed {
                    failures.push(name.to_string());
                }
            }
            reader.join().unwrap();
        }
    }
}
