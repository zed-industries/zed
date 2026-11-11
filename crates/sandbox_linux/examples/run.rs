//! Run a command in a `sandbox_linux` sandbox.
//!
//! ```sh
//! cargo run -p sandbox_linux --example run -- \
//!     --write /tmp/sandbox-escape-test-x/proj --protect /tmp/sandbox-escape-test-x/proj/.git \
//!     --hide '/**/*.env' --ask /tmp/sandbox-escape-test-x/ask \
//!     --allow-net '*.github.com:443' --ask-net '*' -- sh
//! ```
//!
//! Everything is readable and nothing writable, and nothing on the network
//! is reachable, except:
//! - `--write PATTERN` allows writes, `--protect PATTERN` denies them;
//! - `--hide PATTERN` denies reads;
//! - `--ask PATTERN` asks on the terminal before each write;
//! - `--connect PATTERN` allows connecting to unix sockets;
//! - `--allow-net`, `--deny-net` and `--ask-net DESTINATION` for
//!   connections, with destinations as `NetRule::parse` reads them, e.g.
//!   `tcp/*.github.com:22,443`, `udp/10.0.0.0/8:53`, `loopback:5432`.
//!
//! Rules apply in the order given, last match wins.

#[cfg(not(target_os = "linux"))]
fn main() {}

#[cfg(target_os = "linux")]
#[allow(
    clippy::disallowed_methods,
    reason = "a manual test driver that blocks on the child"
)]
fn main() -> anyhow::Result<()> {
    use std::io::{BufRead as _, Write as _};

    use anyhow::{Context as _, bail};
    use sandbox_linux::{
        Access, Decision, FsRule, Handler, NetRule, PortSet, RequestedAccess, Rule, SandboxContext,
        SandboxPolicy, UntrustedCommand,
    };

    sandbox_linux::run_hidden_entrypoint_if_invoked();

    // Asks on the terminal. `a` allows and remembers the exact path (or host
    // and port) for the rest of the run.
    let ask = Handler::new(|request, responder| {
        // The handler must not block, so ask on another thread.
        std::thread::spawn(move || {
            let answer = (|| -> std::io::Result<String> {
                let tty = std::fs::OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open("/dev/tty")?;
                let mut writer = &tty;
                writeln!(writer, "\n[sandbox] {:?} wants:", request.executable)?;
                for item in &request.items {
                    match item {
                        RequestedAccess::Fs { permission, path } => {
                            writeln!(writer, "  {permission:?} {}", path.display())?
                        }
                        RequestedAccess::Net {
                            host,
                            port,
                            protocol,
                        } => writeln!(writer, "  {protocol:?} connection to {host}:{port}")?,
                    }
                }
                write!(writer, "allow? [y/a(lways)/N] ")?;
                let mut answer = String::new();
                std::io::BufReader::new(&tty).read_line(&mut answer)?;
                Ok(answer.trim().to_lowercase())
            })()
            .unwrap_or_default();
            let decision = match answer.as_str() {
                "y" => Decision::allow(),
                "a" => Decision {
                    allow: true,
                    remember: request
                        .items
                        .iter()
                        .filter_map(|item| match item {
                            RequestedAccess::Fs { permission, path } => {
                                let rule = FsRule::new(path.to_str()?).ok()?;
                                Some(Rule::Fs(match permission {
                                    sandbox_linux::Permission::Read => rule.read(Access::Allow),
                                    sandbox_linux::Permission::Write => rule.write(Access::Allow),
                                    sandbox_linux::Permission::Connect => {
                                        rule.connect(Access::Allow)
                                    }
                                }))
                            }
                            RequestedAccess::Net {
                                host,
                                port,
                                protocol,
                            } => {
                                let target = match host {
                                    sandbox_linux::NetHost::Domain(name) => {
                                        sandbox_linux::NetTarget::Domain(
                                            sandbox_linux::DomainPattern::new(name.as_str())
                                                .ok()?,
                                        )
                                    }
                                    sandbox_linux::NetHost::Ip(address) => {
                                        sandbox_linux::NetTarget::Ip(
                                            address.to_string().parse().ok()?,
                                        )
                                    }
                                };
                                let rule = NetRule::new(target, Access::Allow)
                                    .ports(PortSet::single(*port));
                                Some(Rule::Net(match protocol {
                                    sandbox_linux::Protocol::Tcp => rule.tcp_only(),
                                    sandbox_linux::Protocol::Udp => rule.udp_only(),
                                }))
                            }
                        })
                        .collect(),
                },
                _ => Decision::deny(),
            };
            responder.respond(decision);
        });
    });

    let mut rules = vec![FsRule::new("/")?.read(Access::Allow)];
    let mut net = Vec::new();
    let mut arguments = std::env::args().skip(1);
    let mut command = Vec::new();
    while let Some(argument) = arguments.next() {
        let mut value = || arguments.next().context("flag needs a value");
        match argument.as_str() {
            "--write" => rules.push(FsRule::new(value()?)?.write(Access::Allow)),
            "--protect" => rules.push(FsRule::new(value()?)?.write(Access::Deny)),
            "--hide" => rules.push(FsRule::new(value()?)?.read(Access::Deny)),
            "--ask" => rules.push(FsRule::new(value()?)?.write(Access::Ask(ask.clone()))),
            "--connect" => rules.push(FsRule::new(value()?)?.connect(Access::Allow)),
            "--allow-net" => net.push(NetRule::parse(&value()?, Access::Allow)?),
            "--deny-net" => net.push(NetRule::parse(&value()?, Access::Deny)?),
            "--ask-net" => net.push(NetRule::parse(&value()?, Access::Ask(ask.clone()))?),
            "--" => command.extend(arguments.by_ref()),
            other => bail!("unexpected argument {other}"),
        }
    }
    let Some((cmd, args)) = command.split_first() else {
        bail!(
            "usage: run [--write P] [--protect P] [--hide P] [--ask P] [--connect P] [--allow-net D] \
             [--deny-net D] [--ask-net D] -- COMMAND..."
        );
    };

    let context = SandboxContext::new()?;
    let sandbox = context.create_sandbox(SandboxPolicy { fs: rules, net })?;
    let status = sandbox
        .execute(&UntrustedCommand {
            cmd: cmd.clone(),
            args: args.to_vec(),
            env: std::env::vars().collect(),
            cwd: std::env::current_dir()?,
        })?
        .wait()?;
    drop(sandbox);
    drop(context);
    std::process::exit(status.code().unwrap_or(1));
}
