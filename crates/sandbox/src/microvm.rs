//! The `microvm` backend: every sandboxed command runs inside a persistent
//! smol machine (a libkrun microVM with its own kernel) dedicated to the
//! project, driven in-process through the `smolmachines` SDK.
//!
//! Policy mapping. Writable paths are live, read-write mounts at their host
//! paths, so `cwd` and every path in tool output mean the same thing inside
//! and outside the machine. Protected paths (Git metadata) are mounted
//! read-only on top of them. Everything else is the guest's own disk, so tool
//! installs need no host grant and never touch the host. The network policy
//! applies to the whole guest: no network device, a per-host allow-list the
//! engine enforces for every socket (not only HTTP through a proxy), or open
//! egress.
//!
//! Lifecycle. One machine per set of writable roots, named from a hash of
//! those roots and labelled `zed.sandbox=1`, created on first use, started as
//! a branch source (so `smol machine branch` can fork it), and adopted by name
//! afterwards, including across Zed restarts. The engine pulls the image from
//! inside the guest on the first start, so a new machine boots once with
//! network while only its keep-alive workload runs, is stopped, and is
//! switched to the requested policy before the first command executes.
//!
//! The SDK locates an installed engine (`SMOLVM`, `PATH`, `~/.local/bin`) or
//! fetches the release it was built against into the user's cache. The command
//! Zed spawns in its PTY is that engine's interactive exec, so the guest owns
//! the terminal session: `/bin/sh -i -c` and job-control shells behave
//! normally.

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::Command,
    sync::{Mutex, OnceLock},
};

use sha2::{Digest, Sha256};
use smolmachines::{ConnectOptions, Machine, MachineState, Mount};

use crate::{CommandAndArgs, FsSetup, NetSetup, SandboxError, WrappedCommand};

const MACHINE_PREFIX: &str = "zed-sandbox-";
const SANDBOX_LABEL: (&str, &str) = ("zed.sandbox", "1");
/// Debian so that `/bin/sh` is the same dash Debian and Ubuntu hosts run.
const DEFAULT_IMAGE: &str = "debian:bookworm-slim";
const DEFAULT_CPUS: u8 = 2;
const DEFAULT_MEMORY_MIB: u32 = 2048;
const IMAGE_ENV: &str = "ZED_SANDBOX_MICROVM_IMAGE";
const CPUS_ENV: &str = "ZED_SANDBOX_MICROVM_CPUS";
const MEMORY_ENV: &str = "ZED_SANDBOX_MICROVM_MEMORY_MIB";
/// The guest shell every command runs under. The host's shell binary is not
/// in the guest, and a non-interactive `-c` avoids the job-control setup that
/// interactive dash performs against a foreign process group.
const GUEST_SHELL: &str = "/bin/sh";

/// What the guest may reach, as the engine applies it to the whole machine.
#[derive(Clone, Debug, PartialEq, Eq)]
enum GuestNetwork {
    Blocked,
    Open,
    Hosts(Vec<String>),
}

/// The network policy each machine currently runs with, by machine name.
///
/// The engine records whether a machine has a network device but not its
/// allow-list, so this process remembers what it last applied. A fresh process
/// re-applies the policy once (a stop/start of a warm machine), which also
/// repairs a machine left networked by a crash during its pull boot.
static APPLIED_NETWORK: OnceLock<Mutex<HashMap<String, GuestNetwork>>> = OnceLock::new();
/// Machine preparation is serialized per process: two concurrent commands for
/// one project must not both try to create or restart its machine.
static PREPARE: Mutex<()> = Mutex::new(());

/// Whether this host can run local machines, without booting one.
pub(crate) fn availability() -> Result<(), SandboxError> {
    smolmachines::local_availability().map_err(unavailable)
}

/// Transform `command` into the engine exec that runs it in the project's
/// machine, after creating it, applying its network policy, and starting it.
pub(crate) fn wrap(
    fs: &FsSetup,
    network: &NetSetup,
    command: &CommandAndArgs,
) -> Result<WrappedCommand, SandboxError> {
    let roots = writable_roots(fs, command.cwd.as_deref())?;
    let protected: Vec<PathBuf> = fs
        .protected_paths
        .iter()
        .map(|location| location.linux_canonical_path().to_path_buf())
        .collect();
    let guest_network = match network {
        NetSetup::Unrestricted => GuestNetwork::Open,
        NetSetup::Blocked => GuestNetwork::Blocked,
        NetSetup::Restricted { allowlist } => GuestNetwork::Hosts(
            allowlist
                .patterns()
                .iter()
                .map(|pattern| pattern.to_string())
                .collect(),
        ),
    };
    let name = machine_name(&roots);
    let cli = engine_cli()?;
    {
        let _serialized = PREPARE
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        ensure_machine(&cli, &name, &roots, &protected, &guest_network)?;
    }
    Ok(exec_command(&cli, &name, command))
}

/// The host directories mounted read-write: the policy's writable paths, or
/// the command's working directory when the policy allows writes everywhere
/// (the user approved unrestricted writes, so the directory the model chose is
/// inside that grant).
fn writable_roots(fs: &FsSetup, cwd: Option<&Path>) -> Result<Vec<PathBuf>, SandboxError> {
    let mut roots: Vec<PathBuf> = fs
        .writable_paths
        .iter()
        .map(|location| location.linux_canonical_path().to_path_buf())
        .collect();
    if roots.is_empty() && fs.allow_fs_write {
        if let Some(cwd) = cwd {
            roots.push(
                std::fs::canonicalize(cwd)
                    .map_err(|error| SandboxError::Io(format!("{}: {error}", cwd.display())))?,
            );
        }
    }
    if roots.is_empty() {
        return Err(SandboxError::UnsupportedPolicy(
            "the microVM sandbox needs at least one writable project path to mount".to_string(),
        ));
    }
    roots.sort();
    roots.dedup();
    Ok(roots)
}

/// One machine per set of writable roots, stable across restarts.
fn machine_name(roots: &[PathBuf]) -> String {
    let mut roots: Vec<&PathBuf> = roots.iter().collect();
    roots.sort();
    roots.dedup();
    let mut hasher = Sha256::new();
    for root in roots {
        hasher.update(root.as_os_str().as_encoded_bytes());
        hasher.update([0]);
    }
    let digest = hasher.finalize();
    let hex: String = digest
        .iter()
        .take(6)
        .map(|byte| format!("{byte:02x}"))
        .collect();
    format!("{MACHINE_PREFIX}{hex}")
}

/// The engine binary the command runs through: the one `SMOLVM` names, else
/// the release the SDK was built against (fetched into the user's cache on
/// first use). The SDK resolves its own engine the same way.
fn engine_cli() -> Result<PathBuf, SandboxError> {
    if let Some(explicit) = std::env::var_os("SMOLVM") {
        let path = PathBuf::from(explicit);
        if path.is_file() {
            return Ok(path);
        }
        return Err(SandboxError::MicrovmUnavailable(format!(
            "SMOLVM must name an existing engine binary: {}",
            path.display()
        )));
    }
    smolmachines::bootstrap::ensure_engine().map_err(unavailable)
}

fn ensure_machine(
    cli: &Path,
    name: &str,
    roots: &[PathBuf],
    protected: &[PathBuf],
    network: &GuestNetwork,
) -> Result<(), SandboxError> {
    let connect = ConnectOptions::local();
    let existing = Machine::list(&connect, &[SANDBOX_LABEL])
        .map_err(unavailable)?
        .into_iter()
        .find(|machine| machine.name == name);

    let (machine, mut state) = match existing {
        Some(summary) => (Machine::attach(name).map_err(unavailable)?, summary.state),
        None => {
            let mut builder = Machine::builder(name)
                .image(image())
                .cpus(env_number(CPUS_ENV, DEFAULT_CPUS))
                .memory_mib(env_number(MEMORY_ENV, DEFAULT_MEMORY_MIB))
                // The guest pulls its own image on the first start.
                .network(true)
                .persistent(true)
                .branchable(true)
                .label(SANDBOX_LABEL.0, SANDBOX_LABEL.1)
                // Only keeps the guest alive; every tool call is an exec.
                .command(["tail", "-f", "/dev/null"]);
            for root in roots {
                let path = utf8(root)?;
                builder = builder.mount(Mount::new(path, path));
            }
            for path in protected {
                let path = utf8(path)?;
                builder = builder.mount(Mount::new(path, path).read_only());
            }
            let machine = builder.create_with(&connect).map_err(unavailable)?;
            // Pull boot: nothing but the keep-alive workload runs while the
            // machine has unrestricted network. Stop before any command can.
            machine.start().map_err(unavailable)?;
            machine.stop().map_err(unavailable)?;
            (machine, MachineState::Stopped)
        }
    };

    let applied = applied_network().get(name).cloned();
    if applied.as_ref() != Some(network) {
        if matches!(
            state,
            MachineState::Running | MachineState::Started | MachineState::Starting
        ) {
            machine.stop().map_err(unavailable)?;
        }
        apply_network(cli, name, network)?;
        state = MachineState::Stopped;
    }
    if state != MachineState::Running {
        machine.start_branchable().map_err(unavailable)?;
    }
    applied_network().insert(name.to_string(), network.clone());
    Ok(())
}

/// Switch a stopped machine's network to `network`. The allow-list is reset
/// through a no-network update first so an earlier, wider grant does not
/// linger beside the current one.
fn apply_network(cli: &Path, name: &str, network: &GuestNetwork) -> Result<(), SandboxError> {
    engine(cli, &["machine", "update", "--name", name, "--no-net"])?;
    match network {
        GuestNetwork::Blocked => Ok(()),
        GuestNetwork::Open => engine(cli, &["machine", "update", "--name", name, "--net"]),
        GuestNetwork::Hosts(hosts) => {
            let mut args = vec!["machine", "update", "--name", name, "--net"];
            for host in hosts {
                args.push("--allow-host-pattern");
                args.push(host);
            }
            engine(cli, &args)
        }
    }
}

/// Run an engine lifecycle command that the SDK has no method for.
#[allow(
    clippy::disallowed_methods,
    reason = "a short blocking engine call made from the background executor, like the SDK's own; the sandbox crate has no async runtime to hand it"
)]
fn engine(cli: &Path, args: &[&str]) -> Result<(), SandboxError> {
    let output = Command::new(cli)
        .args(args)
        .stdin(std::process::Stdio::null())
        .output()
        .map_err(|error| SandboxError::Io(format!("{}: {error}", cli.display())))?;
    if output.status.success() {
        return Ok(());
    }
    let detail = String::from_utf8_lossy(&output.stderr);
    let detail = detail.trim();
    let detail = if detail.is_empty() {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        detail.to_string()
    };
    Err(SandboxError::MicrovmUnavailable(format!(
        "`smolvm {}` failed: {detail}",
        args.join(" ")
    )))
}

/// The engine exec Zed spawns in its PTY: the command's shell script, run by
/// the guest shell in the command's working directory with its environment.
fn exec_command(cli: &Path, name: &str, command: &CommandAndArgs) -> WrappedCommand {
    let mut args = vec![
        "machine".to_string(),
        "exec".to_string(),
        "--name".to_string(),
        name.to_string(),
        "-t".to_string(),
    ];
    if let Some(cwd) = &command.cwd {
        args.push("-w".to_string());
        args.push(cwd.to_string_lossy().into_owned());
    }
    let mut guest_env: Vec<(&String, &String)> = command
        .env
        .iter()
        .filter(|(key, _)| !host_only_env(key))
        .collect();
    guest_env.sort();
    for (key, value) in guest_env {
        args.push("-e".to_string());
        args.push(format!("{key}={value}"));
    }
    args.push("--".to_string());
    match shell_script(command) {
        Some(script) => {
            args.push(GUEST_SHELL.to_string());
            args.push("-lc".to_string());
            args.push(script);
        }
        None => {
            args.push(command.program.clone());
            args.extend(command.args.iter().cloned());
        }
    }
    // The engine process itself runs on the host with the command's
    // environment, completed with what it needs to find its own state.
    let mut env = command.env.clone();
    for key in [
        "HOME",
        "PATH",
        "USER",
        "XDG_CACHE_HOME",
        "XDG_DATA_HOME",
        "SMOLVM",
    ] {
        if !env.contains_key(key) {
            if let Ok(value) = std::env::var(key) {
                env.insert(key.to_string(), value);
            }
        }
    }
    WrappedCommand {
        program: cli.to_string_lossy().into_owned(),
        args,
        env,
        cwd: command.cwd.clone(),
    }
}

/// The script a host shell was asked to run (`<shell> [-i] -c <script>`), if
/// the command has that shape. It is what the guest shell runs instead.
fn shell_script(command: &CommandAndArgs) -> Option<String> {
    let position = command.args.iter().position(|arg| arg == "-c")?;
    command.args.get(position + 1).cloned()
}

/// Host-session variables that would be wrong inside the guest: paths and
/// sockets of the host, and the host shell's identity.
fn host_only_env(key: &str) -> bool {
    matches!(
        key,
        "PATH"
            | "HOME"
            | "USER"
            | "LOGNAME"
            | "SHELL"
            | "PWD"
            | "OLDPWD"
            | "SHLVL"
            | "TMPDIR"
            | "DISPLAY"
            | "WAYLAND_DISPLAY"
            | "DBUS_SESSION_BUS_ADDRESS"
            | "SSH_AUTH_SOCK"
            | "XAUTHORITY"
            | "SMOLVM"
    ) || key.starts_with("XDG_")
}

fn image() -> String {
    std::env::var(IMAGE_ENV)
        .ok()
        .filter(|image| !image.trim().is_empty())
        .unwrap_or_else(|| DEFAULT_IMAGE.to_string())
}

fn env_number<T: std::str::FromStr>(key: &str, default: T) -> T {
    std::env::var(key)
        .ok()
        .and_then(|value| value.trim().parse().ok())
        .unwrap_or(default)
}

fn applied_network() -> std::sync::MutexGuard<'static, HashMap<String, GuestNetwork>> {
    APPLIED_NETWORK
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn utf8(path: &Path) -> Result<&str, SandboxError> {
    path.to_str().ok_or_else(|| {
        SandboxError::UnsupportedPolicy(format!(
            "the microVM sandbox cannot mount a non-UTF-8 path: {}",
            path.display()
        ))
    })
}

fn unavailable(error: smolmachines::Error) -> SandboxError {
    SandboxError::MicrovmUnavailable(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn command(program: &str, args: &[&str], cwd: Option<&str>) -> CommandAndArgs {
        CommandAndArgs {
            program: program.to_string(),
            args: args.iter().map(|arg| arg.to_string()).collect(),
            env: HashMap::from([
                ("TERM".to_string(), "xterm-256color".to_string()),
                ("PATH".to_string(), "/host/bin".to_string()),
                ("HOME".to_string(), "/home/host".to_string()),
                ("XDG_RUNTIME_DIR".to_string(), "/run/user/1000".to_string()),
                ("FOO".to_string(), "bar".to_string()),
            ]),
            cwd: cwd.map(PathBuf::from),
        }
    }

    #[test]
    fn machine_name_is_stable_and_order_independent() {
        let a = machine_name(&[PathBuf::from("/p/a"), PathBuf::from("/p/b")]);
        let b = machine_name(&[PathBuf::from("/p/b"), PathBuf::from("/p/a")]);
        assert_eq!(a, b);
        assert!(a.starts_with(MACHINE_PREFIX));
        assert_eq!(a.len(), MACHINE_PREFIX.len() + 12);
        assert_ne!(a, machine_name(&[PathBuf::from("/p/a")]));
    }

    #[test]
    fn exec_runs_the_shell_script_in_the_guest_shell_with_guest_env_only() {
        let wrapped = exec_command(
            Path::new("/opt/smolvm"),
            "zed-sandbox-abc",
            &command(
                "/usr/bin/zsh",
                &["-i", "-c", "exec </dev/null; printf 'works\\n'"],
                Some("/p/a"),
            ),
        );
        assert_eq!(wrapped.program, "/opt/smolvm");
        assert_eq!(
            wrapped.args,
            vec![
                "machine",
                "exec",
                "--name",
                "zed-sandbox-abc",
                "-t",
                "-w",
                "/p/a",
                "-e",
                "FOO=bar",
                "-e",
                "TERM=xterm-256color",
                "--",
                "/bin/sh",
                "-lc",
                "exec </dev/null; printf 'works\\n'",
            ]
        );
        assert_eq!(
            wrapped.env.get("HOME").map(String::as_str),
            Some("/home/host")
        );
        assert_eq!(wrapped.cwd, Some(PathBuf::from("/p/a")));
    }

    #[test]
    fn exec_passes_a_bare_program_through_when_there_is_no_script() {
        let wrapped = exec_command(
            Path::new("/opt/smolvm"),
            "zed-sandbox-abc",
            &command("/usr/bin/make", &["test"], None),
        );
        assert!(!wrapped.args.contains(&"-w".to_string()));
        let tail: Vec<&str> = wrapped
            .args
            .iter()
            .rev()
            .take(3)
            .rev()
            .map(String::as_str)
            .collect();
        assert_eq!(tail, vec!["--", "/usr/bin/make", "test"]);
    }

    #[test]
    fn host_only_variables_stay_on_the_host() {
        for key in ["PATH", "HOME", "XDG_RUNTIME_DIR", "SSH_AUTH_SOCK", "SMOLVM"] {
            assert!(host_only_env(key), "{key}");
        }
        for key in ["TERM", "LANG", "CARGO_HOME", "FOO"] {
            assert!(!host_only_env(key), "{key}");
        }
    }
}
