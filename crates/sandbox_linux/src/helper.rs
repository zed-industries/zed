//! The helper (`zed --zed-sandbox-linux-helper`), which becomes the parent of
//! the sandboxed command.
//!
//! Spawned by Zed with two inherited sockets: [`SETUP_FD`] carries the
//! [`HelperSetup`], and [`ATTACH_FD`] reaches the supervisor. It then:
//!
//! 1. creates user, mount, network and IPC namespaces. The mount namespace
//!    is a copy of the host's, made read-only: the kernel-enforced backstop
//!    for any write syscall the supervisor doesn't handle. Paths are the
//!    same inside and out. In the network namespace every address routes
//!    to loopback, and DNS sockets are created for the supervisor
//!    ([`crate::net`]).
//! 2. becomes a child subreaper, so every process the command spawns stays
//!    its descendant (that's how the supervisor tells them apart);
//! 3. forks the command, which drops all capabilities, installs the seccomp
//!    filter, hands the listener (and DNS sockets) to the supervisor and
//!    execs;
//! 4. serves the supervisor's requests for sockets in the namespace, while
//!    it waits for the command; then kills whatever it left running, and exits with
//!    its status.

#![allow(
    clippy::disallowed_methods,
    reason = "the helper is a standalone process that must block on its command"
)]

use std::os::fd::{AsFd as _, AsRawFd as _, BorrowedFd, FromRawFd as _, OwnedFd, RawFd};
use std::os::unix::process::{CommandExt as _, ExitStatusExt as _};
use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::{Context as _, Result};

use crate::protocol::{AttachInfo, HelperSetup, receive_message, send_message};
use crate::sys::{self, cvt};

pub(crate) const HELPER_FLAG: &str = "--zed-sandbox-linux-helper";
pub(crate) const SETUP_FD: RawFd = 3;
pub(crate) const ATTACH_FD: RawFd = 4;

/// Exit status for failures before the command runs (as with `env`/`bwrap`).
const SETUP_FAILED: i32 = 126;

pub(crate) fn run() -> ! {
    match run_helper() {
        Ok(code) => std::process::exit(code),
        Err(error) => {
            eprintln!("zed sandbox: {error:#}");
            std::process::exit(SETUP_FAILED);
        }
    }
}

fn run_helper() -> Result<i32> {
    sys::set_parent_death_signal()?;
    // SAFETY: Zed spawns the helper with exactly these two descriptors.
    let (setup_socket, attach_socket) = unsafe {
        (
            sys::inherited_fd(SETUP_FD).context("missing setup socket")?,
            sys::inherited_fd(ATTACH_FD).context("missing attach socket")?,
        )
    };
    let (setup, _) = receive_message::<HelperSetup>(setup_socket.as_fd())?
        .context("Zed closed the setup channel")?;
    drop(setup_socket);

    // SAFETY: getuid/getgid can't fail.
    let (uid, gid) = unsafe { (libc::getuid(), libc::getgid()) };
    let namespaces =
        libc::CLONE_NEWUSER | libc::CLONE_NEWNS | libc::CLONE_NEWNET | libc::CLONE_NEWIPC;
    // SAFETY: plain unshare; we're single-threaded.
    if let Err(error) = cvt(unsafe { libc::unshare(namespaces) }) {
        anyhow::bail!(
            "creating sandbox namespaces failed: {error}. Unprivileged user namespaces may be \
             disabled, or (on Ubuntu 23.10+) restricted by AppArmor"
        );
    }
    // Keep our own uid/gid inside. Capabilities come from having created the
    // user namespace, and the command drops them.
    std::fs::write("/proc/self/setgroups", "deny").context("writing setgroups")?;
    std::fs::write("/proc/self/uid_map", format!("{uid} {uid} 1")).context("writing uid_map")?;
    std::fs::write("/proc/self/gid_map", format!("{gid} {gid} 1")).context("writing gid_map")?;
    bring_up_loopback()?;
    crate::net::configure_namespace()?;
    let dns_sockets = crate::net::create_dns_sockets()?;

    // Nothing done here may propagate back to the host.
    sys::mount(
        None,
        Path::new("/"),
        None,
        libc::MS_REC | libc::MS_PRIVATE,
        None,
    )
    .context("making mounts private")?;
    sys::mount_setattr(Path::new("/"), sys::MOUNT_ATTR_RDONLY, true)
        .context("making the filesystem read-only")?;
    // SAFETY: plain prctl.
    cvt(unsafe { libc::prctl(libc::PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0) })
        .context("becoming a subreaper")?;

    install_terminate_handler()?;
    let command = spawn_command(&setup, &attach_socket, dns_sockets)?;
    start_factory(attach_socket)?;
    let status = wait_for_command(command)?;
    kill_remaining();
    Ok(status)
}

static TERMINATE_REQUESTED: AtomicBool = AtomicBool::new(false);

extern "C" fn on_terminate(_signal: libc::c_int) {
    TERMINATE_REQUESTED.store(true, Ordering::SeqCst);
}

/// `SIGTERM` stops the whole execution: the command and everything it
/// spawned (which `SIGKILL`ing the helper alone would orphan). The handler
/// is installed without `SA_RESTART`, so it interrupts `waitpid`. The
/// command gets default dispositions back when it execs.
fn install_terminate_handler() -> Result<()> {
    // SAFETY: plain old data.
    let mut action: libc::sigaction = unsafe { std::mem::zeroed() };
    action.sa_sigaction = on_terminate as extern "C" fn(libc::c_int) as libc::sighandler_t;
    action.sa_flags = 0;
    // SAFETY: valid pointers to a sigaction we own.
    unsafe {
        libc::sigemptyset(&mut action.sa_mask);
        cvt(libc::sigaction(
            libc::SIGTERM,
            &action,
            std::ptr::null_mut(),
        ))
        .context("installing the SIGTERM handler")?;
    }
    Ok(())
}

fn wait_for_command(command: libc::pid_t) -> Result<i32> {
    loop {
        if TERMINATE_REQUESTED.load(Ordering::SeqCst) {
            return Ok(128 + libc::SIGTERM);
        }
        let mut status = 0;
        // SAFETY: plain waitpid.
        let pid = unsafe { libc::waitpid(-1, &mut status, 0) };
        if pid == command {
            return Ok(exit_code(status));
        }
        if pid < 0 {
            let error = std::io::Error::last_os_error();
            if error.raw_os_error() != Some(libc::EINTR) {
                return Err(error).context("waiting for the command");
            }
        }
    }
}

/// Kill everything the command left behind. As a subreaper we inherit each
/// orphan, so repeat until there are no children left.
fn kill_remaining() {
    // SAFETY: getpid can't fail.
    let helper = unsafe { libc::getpid() };
    loop {
        for process in crate::process::all_processes() {
            if process.parent == helper {
                // SAFETY: plain kill.
                unsafe { libc::kill(process.pid, libc::SIGKILL) };
            }
        }
        let mut status = 0;
        // SAFETY: plain waitpid.
        let pid = unsafe { libc::waitpid(-1, &mut status, 0) };
        if pid < 0 && std::io::Error::last_os_error().raw_os_error() == Some(libc::ECHILD) {
            return;
        }
    }
}

fn exit_code(status: i32) -> i32 {
    let status = std::process::ExitStatus::from_raw(status);
    status
        .code()
        .unwrap_or_else(|| 128 + status.signal().unwrap_or(0))
}

fn bring_up_loopback() -> Result<()> {
    // SAFETY: plain socket call.
    let socket =
        cvt(unsafe { libc::socket(libc::AF_INET, libc::SOCK_DGRAM | libc::SOCK_CLOEXEC, 0) })?;
    // SAFETY: freshly created, owned here.
    let socket = unsafe { OwnedFd::from_raw_fd(socket) };
    // SAFETY: `ifreq` is plain old data.
    let mut request: libc::ifreq = unsafe { std::mem::zeroed() };
    for (slot, byte) in request.ifr_name.iter_mut().zip(b"lo") {
        *slot = *byte as libc::c_char;
    }
    // SAFETY: SIOC[GS]IFFLAGS with a valid `ifreq`.
    unsafe {
        cvt(libc::ioctl(
            socket.as_raw_fd(),
            libc::SIOCGIFFLAGS,
            &mut request,
        ))?;
        request.ifr_ifru.ifru_flags |= libc::IFF_UP as libc::c_short;
        cvt(libc::ioctl(
            socket.as_raw_fd(),
            libc::SIOCSIFFLAGS,
            &request,
        ))
        .context("bringing up loopback")?;
    }
    Ok(())
}

/// Serve the supervisor's requests for sockets in our namespace, on a thread
/// that leaves `SIGTERM` to the main one (where it interrupts `waitpid`).
fn start_factory(attach_socket: OwnedFd) -> Result<()> {
    // SAFETY: plain old data, initialized by sigemptyset.
    let mut terminate: libc::sigset_t = unsafe { std::mem::zeroed() };
    // SAFETY: valid signal set pointers.
    unsafe {
        libc::sigemptyset(&mut terminate);
        libc::sigaddset(&mut terminate, libc::SIGTERM);
        libc::pthread_sigmask(libc::SIG_BLOCK, &terminate, std::ptr::null_mut());
    }
    // The new thread inherits the blocked mask.
    let spawned = std::thread::Builder::new()
        .name("zed-sandbox-factory".to_string())
        .spawn(move || crate::net::serve_factory(attach_socket));
    // SAFETY: as above.
    unsafe { libc::pthread_sigmask(libc::SIG_UNBLOCK, &terminate, std::ptr::null_mut()) };
    spawned.context("starting the socket factory")?;
    Ok(())
}

fn spawn_command(
    setup: &HelperSetup,
    attach_socket: &OwnedFd,
    dns_sockets: Vec<OwnedFd>,
) -> Result<libc::pid_t> {
    let filter = crate::seccomp::build_filter();
    let mut command = Command::new(&setup.command.cmd);
    command
        .args(&setup.command.args)
        .env_clear()
        .envs(&setup.command.env)
        .current_dir(&setup.command.cwd);
    let attach = attach_socket.as_raw_fd();
    let dns: Vec<RawFd> = dns_sockets
        .iter()
        .map(|socket| socket.as_raw_fd())
        .collect();
    // SAFETY: runs in the forked child before exec. The helper is
    // single-threaded, so no lock can be held across the fork.
    unsafe {
        command.pre_exec(move || {
            sys::set_parent_death_signal()?;
            drop_capabilities()?;
            let helper_pid = libc::getppid();
            let listener = crate::seccomp::install(&filter)?;
            // From here on, opens go to the supervisor, which can't answer
            // until it has the listener: send it before anything else.
            // SAFETY: `attach` stays open in the child until exec (CLOEXEC).
            let attach = BorrowedFd::borrow_raw(attach);
            let mut fds = vec![listener.as_raw_fd()];
            fds.extend(&dns);
            send_message(attach, &AttachInfo { helper_pid }, &fds)
                .map_err(std::io::Error::other)?;
            Ok(())
        });
    }
    let child = command
        .spawn()
        .with_context(|| format!("running {}", setup.command.cmd))?;
    // The supervisor has them now (they were close-on-exec in the command).
    drop(dns_sockets);
    Ok(child.id() as libc::pid_t)
}

/// Clear every capability set. `exec` as a non-root uid already does this;
/// being explicit also covers a user running Zed as root.
fn drop_capabilities() -> std::io::Result<()> {
    #[repr(C)]
    struct CapabilityHeader {
        version: u32,
        pid: i32,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    struct CapabilityData {
        effective: u32,
        permitted: u32,
        inheritable: u32,
    }
    const LINUX_CAPABILITY_VERSION_3: u32 = 0x2008_0522;

    for capability in 0..64 {
        // SAFETY: plain prctl; EINVAL past the last capability is expected.
        unsafe { libc::prctl(libc::PR_CAPBSET_DROP, capability, 0, 0, 0) };
    }
    // SAFETY: plain prctl.
    cvt(unsafe {
        libc::prctl(
            libc::PR_CAP_AMBIENT,
            libc::PR_CAP_AMBIENT_CLEAR_ALL,
            0,
            0,
            0,
        )
    })?;
    let header = CapabilityHeader {
        version: LINUX_CAPABILITY_VERSION_3,
        pid: 0,
    };
    let data = [CapabilityData {
        effective: 0,
        permitted: 0,
        inheritable: 0,
    }; 2];
    // SAFETY: capset with a v3 header and two data words.
    cvt(unsafe { libc::syscall(libc::SYS_capset, &header, data.as_ptr()) as i32 })?;
    Ok(())
}
