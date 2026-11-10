//! Run a command in a `sandbox_linux` sandbox.
//!
//! ```sh
//! cargo run -p sandbox_linux --example run -- \
//!     --write /tmp/sandbox-escape-test-x/proj --protect /tmp/sandbox-escape-test-x/proj/.git \
//!     --hide '/**/*.env' --ask /tmp/sandbox-escape-test-x/ask -- sh
//! ```
//!
//! Everything is readable and nothing writable, except:
//! - `--write PATTERN` allows writes, `--protect PATTERN` denies them;
//! - `--hide PATTERN` denies reads;
//! - `--ask PATTERN` asks on the terminal before each write.
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
        Access, Decision, FsRule, Handler, RequestedAccess, Rule, SandboxContext, SandboxPolicy,
        UntrustedCommand,
    };

    sandbox_linux::run_hidden_entrypoint_if_invoked();

    // Asks on the terminal. `a` allows and remembers the exact path for the
    // rest of the run.
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
                    let RequestedAccess::Fs { permission, path } = item;
                    writeln!(writer, "  {permission:?} {}", path.display())?;
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
                        .filter_map(|item| {
                            let RequestedAccess::Fs { permission, path } = item;
                            let rule = FsRule::new(path.to_str()?).ok()?;
                            Some(Rule::Fs(match permission {
                                sandbox_linux::Permission::Read => rule.read(Access::Allow),
                                sandbox_linux::Permission::Write => rule.write(Access::Allow),
                            }))
                        })
                        .collect(),
                },
                _ => Decision::deny(),
            };
            responder.respond(decision);
        });
    });

    let mut rules = vec![FsRule::new("/")?.read(Access::Allow)];
    let mut arguments = std::env::args().skip(1);
    let mut command = Vec::new();
    while let Some(argument) = arguments.next() {
        let mut pattern = || arguments.next().context("flag needs a path pattern");
        match argument.as_str() {
            "--write" => rules.push(FsRule::new(pattern()?)?.write(Access::Allow)),
            "--protect" => rules.push(FsRule::new(pattern()?)?.write(Access::Deny)),
            "--hide" => rules.push(FsRule::new(pattern()?)?.read(Access::Deny)),
            "--ask" => rules.push(FsRule::new(pattern()?)?.write(Access::Ask(ask.clone()))),
            "--" => command.extend(arguments.by_ref()),
            other => bail!("unexpected argument {other}"),
        }
    }
    let Some((cmd, args)) = command.split_first() else {
        bail!("usage: run [--write P] [--protect P] [--hide P] [--ask P] -- COMMAND...");
    };

    let context = SandboxContext::new()?;
    let sandbox = context.create_sandbox(SandboxPolicy { fs: rules })?;
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
