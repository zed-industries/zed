//! Handling of one trapped syscall.
//!
//! - Filesystem syscalls are never continued: the supervisor resolves the
//!   path itself ([`crate::resolve`]), checks the rules against the object it
//!   ends up holding, and performs the operation on that object.
//!   - Reads resolve in the sandbox's own root, so the fd we hand back lives
//!     on its read-only mounts (a read fd can't be used to `fchmod`).
//!   - Writes resolve from the host's root, on its writable mounts.
//! - Syscalls acting on another process by pid are checked (the pid is a
//!   register, so it can't change under us) and then continued. A pid reused
//!   between the check and the call is a remaining race.
//! - `connect` on IP sockets is decided by [`crate::net`]. Continuing it is
//!   always safe: nothing outside the sandbox's network namespace can be
//!   reached by address from inside it.
//! - `connect` and `bind` on unix sockets are checked against the
//!   filesystem rules (sockets are found by path) and performed by us.

use std::cell::OnceCell;
use std::collections::HashSet;
use std::ffi::{CString, OsStr};
use std::io;
use std::os::fd::{AsFd as _, AsRawFd as _, BorrowedFd, OwnedFd};
use std::os::unix::ffi::OsStrExt as _;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::net::{self, Caller, TcpOutcome};
use crate::policy::{HandlerId, Permission, RequestedAccess, WireAccess, WirePolicy};
use crate::process::{self, proc_path_pid};
use crate::resolve::{Mode, Resolved, Resolver};
use crate::seccomp::{Brokered, brokered_syscall};
use crate::supervisor::{AskOutcome, Execution, Shared};
use crate::sys::{self, cvt, errno, path_cstring};

// From <linux/seccomp.h> (generic ioctl encoding, shared by x86_64/aarch64).
const SECCOMP_IOCTL_NOTIF_RECV: libc::c_ulong = 0xc050_2100;
const SECCOMP_IOCTL_NOTIF_SEND: libc::c_ulong = 0xc018_2101;
const SECCOMP_IOCTL_NOTIF_ID_VALID: libc::c_ulong = 0x4008_2102;
const SECCOMP_IOCTL_NOTIF_ADDFD: libc::c_ulong = 0x4018_2103;
const SECCOMP_ADDFD_FLAG_SEND: u32 = 1 << 1;
const SECCOMP_USER_NOTIF_FLAG_CONTINUE: u32 = 1;

const PRIO_PROCESS: i32 = 0;
const PRIO_PGRP: i32 = 1;
const IOPRIO_WHO_PROCESS: i32 = 1;
const IOPRIO_WHO_PGRP: i32 = 2;

#[repr(C)]
#[derive(Default)]
struct NotificationResponse {
    id: u64,
    val: i64,
    error: i32,
    flags: u32,
}

#[repr(C)]
struct AddFd {
    id: u64,
    flags: u32,
    srcfd: u32,
    newfd: u32,
    newfd_flags: u32,
}

enum Response {
    /// Run the original syscall in the kernel.
    Continue,
    Errno(i32),
    Success,
    /// Install `fd` in the target and return its number.
    InstallFd {
        fd: OwnedFd,
        cloexec: bool,
    },
}

/// Receive one pending notification (main thread).
pub(crate) fn receive(listener: BorrowedFd<'_>) -> Option<libc::seccomp_notif> {
    // SAFETY: plain old data; the kernel requires it zeroed before RECV.
    let mut notification: libc::seccomp_notif = unsafe { std::mem::zeroed() };
    // SAFETY: RECV fills in the struct above.
    let received = unsafe {
        libc::ioctl(
            listener.as_raw_fd(),
            SECCOMP_IOCTL_NOTIF_RECV as _,
            &mut notification as *mut libc::seccomp_notif,
        )
    };
    // ENOENT: the target was interrupted before we picked it up.
    (received == 0).then_some(notification)
}

/// Handle and answer one notification (worker thread).
pub(crate) fn handle(
    shared: &Arc<Shared>,
    execution: &Arc<Execution>,
    notification: libc::seccomp_notif,
) {
    let request = Request {
        listener: execution.listener.as_fd(),
        id: notification.id,
        // The UAPI field is named `pid`, but it is the trapping thread's id.
        tid: notification.pid as i32,
        number: notification.data.nr as i64,
        args: notification.data.args,
        status: OnceCell::new(),
    };
    let policy = execution.sandbox.policy();
    let broker = Broker {
        shared,
        execution,
        request: &request,
        policy: &policy,
    };
    let response = match brokered_syscall(request.number) {
        Some(syscall) => broker
            .handle(syscall)
            .unwrap_or_else(|error| Response::Errno(error.raw_os_error().unwrap_or(libc::EIO))),
        None => Response::Continue,
    };
    send_response(&request, response);
}

/// A trapped syscall, plus access to the target thread.
pub(crate) struct Request<'a> {
    listener: BorrowedFd<'a>,
    id: u64,
    tid: i32,
    number: i64,
    args: [u64; 6],
    /// The thread's `/proc/<tid>/status` fields, read at most once.
    status: OnceCell<Option<ThreadStatus>>,
}

#[derive(Clone, Copy)]
struct ThreadStatus {
    tgid: i32,
    umask: u32,
}

impl Request<'_> {
    fn int(&self, index: usize) -> i32 {
        self.args[index] as i32
    }

    /// Copy up to `buffer.len()` bytes out of the target, stopping early at
    /// an unmapped page. The copy is ours; the target can't touch it
    /// afterwards (it may race the copy itself, yielding torn bytes, which
    /// are still just bytes).
    fn read_memory(&self, address: u64, buffer: &mut [u8]) -> io::Result<usize> {
        let local = libc::iovec {
            iov_base: buffer.as_mut_ptr().cast(),
            iov_len: buffer.len(),
        };
        let remote = libc::iovec {
            iov_base: address as *mut libc::c_void,
            iov_len: buffer.len(),
        };
        // SAFETY: `local` describes our buffer; the kernel checks `remote`.
        let read = unsafe { libc::process_vm_readv(self.tid, &local, 1, &remote, 1, 0) };
        if read < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(read as usize)
    }

    fn read_bytes(&self, address: u64, length: usize) -> io::Result<Vec<u8>> {
        let mut buffer = vec![0u8; length];
        if self.read_memory(address, &mut buffer)? != length {
            return Err(errno(libc::EFAULT));
        }
        Ok(buffer)
    }

    /// Copy the NUL-terminated string at argument `index` out of the target.
    fn read_string(&self, index: usize) -> io::Result<Vec<u8>> {
        let mut address = self.args[index];
        if address == 0 {
            return Err(errno(libc::EFAULT));
        }
        let mut result = Vec::new();
        let mut chunk = [0u8; 4096];
        while result.len() <= libc::PATH_MAX as usize {
            // Never read across a page boundary in one go, so an unmapped page
            // after the string doesn't fail the read.
            let chunk_end = (address | 0xfff) + 1;
            let length = (chunk_end - address) as usize;
            let read = self.read_memory(address, &mut chunk[..length])?;
            if read == 0 {
                return Err(errno(libc::EFAULT));
            }
            if let Some(nul) = chunk[..read].iter().position(|byte| *byte == 0) {
                result.extend_from_slice(&chunk[..nul]);
                return Ok(result);
            }
            result.extend_from_slice(&chunk[..read]);
            address += read as u64;
        }
        Err(errno(libc::ENAMETOOLONG))
    }

    fn status(&self) -> Option<ThreadStatus> {
        *self.status.get_or_init(|| {
            let status = process::read_status_head(self.tid)?;
            let field = |name: &str| {
                status
                    .lines()
                    .find_map(|line| line.strip_prefix(name))
                    .map(str::trim)
            };
            Some(ThreadStatus {
                tgid: field("Tgid:")?.parse().ok()?,
                umask: u32::from_str_radix(field("Umask:")?, 8).ok()?,
            })
        })
    }

    fn umask(&self) -> u32 {
        self.status().map_or(0o022, |status| status.umask)
    }

    /// Whether the target is still blocked in this syscall. Checked after
    /// reading from `/proc/<tid>` and before acting, so a recycled tid can't
    /// feed us another process's data.
    pub fn still_valid(&self) -> bool {
        let mut id = self.id;
        // SAFETY: ioctl with a pointer to a live u64.
        unsafe {
            libc::ioctl(
                self.listener.as_raw_fd(),
                SECCOMP_IOCTL_NOTIF_ID_VALID as _,
                &mut id as *mut u64,
            ) == 0
        }
    }
}

#[derive(Clone, Copy)]
enum View {
    /// The sandbox's mount namespace (read-only mounts).
    Sandbox,
    /// The supervisor's own (the host's writable mounts).
    Host,
}

#[derive(Clone, Copy)]
enum UnixCall {
    Connect,
    Bind,
}

struct Broker<'a> {
    shared: &'a Arc<Shared>,
    execution: &'a Arc<Execution>,
    request: &'a Request<'a>,
    policy: &'a WirePolicy,
}

impl Broker<'_> {
    fn handle(&self, syscall: Brokered) -> io::Result<Response> {
        let request = self.request;
        let int = |index| request.int(index);
        let arg = |index: usize| request.args[index];
        const CWD: i32 = libc::AT_FDCWD;
        match syscall {
            Brokered::OpenAt => self.open(int(0), 1, int(2), arg(3) as u32),
            #[cfg(target_arch = "x86_64")]
            Brokered::Open => self.open(CWD, 0, int(1), arg(2) as u32),
            #[cfg(target_arch = "x86_64")]
            Brokered::Creat => self.open(
                CWD,
                0,
                libc::O_CREAT | libc::O_WRONLY | libc::O_TRUNC,
                arg(1) as u32,
            ),
            Brokered::MkdirAt => self.mkdir(int(0), 1, arg(2) as u32),
            #[cfg(target_arch = "x86_64")]
            Brokered::Mkdir => self.mkdir(CWD, 0, arg(1) as u32),
            Brokered::UnlinkAt => self.unlink(int(0), 1, int(2)),
            #[cfg(target_arch = "x86_64")]
            Brokered::Unlink => self.unlink(CWD, 0, 0),
            #[cfg(target_arch = "x86_64")]
            Brokered::Rmdir => self.unlink(CWD, 0, libc::AT_REMOVEDIR),
            #[cfg(target_arch = "x86_64")]
            Brokered::RenameAt => self.rename((int(0), 1), (int(2), 3), 0),
            Brokered::RenameAt2 => self.rename((int(0), 1), (int(2), 3), arg(4) as u32),
            #[cfg(target_arch = "x86_64")]
            Brokered::Rename => self.rename((CWD, 0), (CWD, 1), 0),
            Brokered::LinkAt => self.link((int(0), 1), (int(2), 3), int(4)),
            #[cfg(target_arch = "x86_64")]
            Brokered::Link => self.link((CWD, 0), (CWD, 1), 0),
            Brokered::SymlinkAt => self.symlink(0, (int(1), 2)),
            #[cfg(target_arch = "x86_64")]
            Brokered::Symlink => self.symlink(0, (CWD, 1)),
            Brokered::FchmodAt => self.chmod(int(0), 1, arg(2) as u32),
            #[cfg(target_arch = "x86_64")]
            Brokered::Chmod => self.chmod(CWD, 0, arg(1) as u32),
            Brokered::Truncate => self.truncate(arg(1) as i64),
            Brokered::FaccessAt => self.access(int(0), 1, int(2), 0),
            Brokered::FaccessAt2 => self.access(int(0), 1, int(2), int(3)),
            #[cfg(target_arch = "x86_64")]
            Brokered::Access => self.access(CWD, 0, int(1), 0),
            Brokered::UtimensAt => self.utimens(),
            Brokered::Kill => self.kill(int(0), int(1)),
            Brokered::Tkill => {
                let group = process::thread_group(int(0)).ok_or_else(|| errno(libc::ESRCH))?;
                self.continue_if_member(group, libc::EPERM)
            }
            Brokered::Tgkill | Brokered::RtSigqueueinfo | Brokered::RtTgsigqueueinfo => {
                self.continue_if_member(int(0), libc::EPERM)
            }
            Brokered::PidfdOpen => self.continue_if_member(int(0), libc::ESRCH),
            Brokered::Prlimit64
            | Brokered::SchedSetaffinity
            | Brokered::SchedSetscheduler
            | Brokered::SchedSetparam
            | Brokered::SchedSetattr => match int(0) {
                0 => Ok(Response::Continue),
                pid => self.continue_if_member(pid, libc::EPERM),
            },
            Brokered::Setpriority => self.process_or_group(int(0), int(1), PRIO_PROCESS, PRIO_PGRP),
            Brokered::IoprioSet => {
                self.process_or_group(int(0), int(1), IOPRIO_WHO_PROCESS, IOPRIO_WHO_PGRP)
            }
            Brokered::Connect => self.connect(int(0), arg(1), arg(2) as u32),
            Brokered::Bind => self.bind(int(0), arg(1), arg(2) as u32),
        }
    }

    /// The target's socket `fd` and its address family, or `None` if `fd`
    /// isn't an open socket (the kernel reports that itself).
    fn target_socket(&self, fd: i32) -> io::Result<Option<(OwnedFd, i32)>> {
        // Through the helper, which can fetch the descriptor where we, the
        // command's uncle, may be denied the ptrace access it takes.
        let socket = match self.execution.network.fetch_fd(self.tgid()?, fd) {
            Ok(socket) => socket,
            Err(error) if error.raw_os_error() == Some(libc::EBADF) => return Ok(None),
            Err(error) => return Err(error),
        };
        if !self.request.still_valid() {
            return Err(errno(libc::ESRCH));
        }
        Ok(
            sys::socket_option(socket.as_fd(), libc::SOL_SOCKET, libc::SO_DOMAIN)
                .ok()
                .map(|domain| (socket, domain)),
        )
    }

    fn bind(&self, fd: i32, address: u64, length: u32) -> io::Result<Response> {
        match self.target_socket(fd)? {
            Some((socket, libc::AF_UNIX)) => {
                self.unix_address_call(&socket, address, length, UnixCall::Bind)
            }
            // Addresses of other families only mean something inside the
            // sandbox's network namespace.
            _ => Ok(Response::Continue),
        }
    }

    fn connect(&self, fd: i32, address: u64, length: u32) -> io::Result<Response> {
        let Some((socket, domain)) = self.target_socket(fd)? else {
            return Ok(Response::Continue);
        };
        match domain {
            libc::AF_INET | libc::AF_INET6 => {}
            libc::AF_UNIX => {
                return self.unix_address_call(&socket, address, length, UnixCall::Connect);
            }
            // Netlink: within the namespace.
            _ => return Ok(Response::Continue),
        }
        let socket = socket.as_fd();
        let length = (length as usize).min(size_of::<libc::sockaddr_storage>());
        if address == 0 || length == 0 {
            return Ok(Response::Continue);
        }
        let Some(destination) = net::decode_address(&self.request.read_bytes(address, length)?)
        else {
            // `AF_UNSPEC` (dissolving a UDP association), or nonsense the
            // kernel rejects.
            return Ok(Response::Continue);
        };
        let socket_type = sys::socket_option(socket, libc::SOL_SOCKET, libc::SO_TYPE)?;
        let protocol = sys::socket_option(socket, libc::SOL_SOCKET, libc::SO_PROTOCOL)?;
        match (socket_type, protocol) {
            (libc::SOCK_STREAM, libc::IPPROTO_TCP | libc::IPPROTO_MPTCP) => {
                let still_valid = || self.request.still_valid();
                let caller = Caller {
                    tid: self.request.tid,
                    still_valid: &still_valid,
                };
                match net::connect_tcp(self.shared, self.execution, socket, destination, &caller)? {
                    TcpOutcome::Continue => Ok(Response::Continue),
                    TcpOutcome::Relayed => Ok(Response::Success),
                }
            }
            (libc::SOCK_DGRAM, libc::IPPROTO_UDP) => {
                net::connect_udp(self.shared, self.execution, destination);
                Ok(Response::Continue)
            }
            _ => Ok(Response::Continue),
        }
    }

    /// `connect` or `bind` of a unix socket. Never continued: the kernel
    /// would read the address again, after the target had a chance to
    /// change it. We make the call on our copy of the socket instead.
    ///
    /// - Abstract and unnamed addresses are looked up in the socket's own
    ///   network namespace (the sandbox's), so they're used as they are.
    /// - A path is resolved like any other: `connect` needs `Connect` on the
    ///   socket it finds, `bind` needs `Write` on the entry it creates. The
    ///   call then goes through `/proc/self/fd`, reaching exactly what was
    ///   checked. (So `getsockname` on a bound socket shows that path.)
    fn unix_address_call(
        &self,
        socket: &OwnedFd,
        address: u64,
        length: u32,
        call: UnixCall,
    ) -> io::Result<Response> {
        let length = length as usize;
        if address == 0 {
            return Err(errno(libc::EFAULT));
        }
        if !(size_of::<libc::sa_family_t>()..=size_of::<libc::sockaddr_un>()).contains(&length) {
            return Err(errno(libc::EINVAL));
        }
        let bytes = self.request.read_bytes(address, length)?;
        if i32::from(u16::from_ne_bytes([bytes[0], bytes[1]])) != libc::AF_UNIX {
            return Err(errno(libc::EINVAL));
        }
        let name = &bytes[2..];
        let mut _held = None;
        let address = match name.first() {
            None | Some(0) => bytes.clone(),
            Some(_) => {
                let end = name
                    .iter()
                    .position(|byte| *byte == 0)
                    .unwrap_or(name.len());
                let path = &name[..end];
                let through = match call {
                    UnixCall::Connect => {
                        let Resolved::Object(object) = self.resolve_bytes(
                            View::Sandbox,
                            libc::AT_FDCWD,
                            path,
                            Mode::Object { follow: true },
                        )?
                        else {
                            return Err(errno(libc::ENOENT));
                        };
                        if sys::fstat(object.as_fd())?.st_mode & libc::S_IFMT != libc::S_IFSOCK {
                            return Err(errno(libc::ECONNREFUSED));
                        }
                        let path = self
                            .object_path(&object)?
                            .ok_or_else(|| errno(libc::ECONNREFUSED))?;
                        self.authorize(&[(Permission::Connect, &path)])?;
                        let through = sys::proc_self_fd(object.as_fd());
                        _held = Some(object);
                        through
                    }
                    UnixCall::Bind => {
                        let (parent, name, path) =
                            self.entry_bytes(libc::AT_FDCWD, path, libc::EADDRINUSE)?;
                        self.authorize(&[(Permission::Write, &path)])?;
                        let through = sys::proc_self_fd(parent.as_fd())
                            .join(OsStr::from_bytes(name.as_bytes()));
                        _held = Some(parent);
                        through
                    }
                };
                let through = through.as_os_str().as_bytes();
                if through.len() >= size_of::<libc::sockaddr_un>() - 2 {
                    return Err(errno(libc::ENAMETOOLONG));
                }
                let mut address = bytes[..2].to_vec();
                address.extend_from_slice(through);
                address.push(0);
                address
            }
        };
        let (fd, pointer, length) = (
            socket.as_raw_fd(),
            address.as_ptr().cast(),
            address.len() as libc::socklen_t,
        );
        // SAFETY: a sockaddr_un we built, with its length.
        cvt(unsafe {
            match call {
                UnixCall::Connect => libc::connect(fd, pointer, length),
                UnixCall::Bind => libc::bind(fd, pointer, length),
            }
        })?;
        Ok(Response::Success)
    }

    /// The root of the execution's mount namespace. Every process in it has
    /// the same one: none can `chroot` or `pivot_root` without capabilities.
    fn sandbox_root(&self) -> io::Result<BorrowedFd<'_>> {
        if let Some(root) = self.execution.root.get() {
            return Ok(root.as_fd());
        }
        let root = sys::open(
            Path::new(&format!("/proc/{}/root", self.request.tid)),
            libc::O_PATH | libc::O_DIRECTORY,
        )?;
        // The tid could have been recycled since the notification; don't
        // keep another process's root.
        if !self.request.still_valid() {
            return Err(errno(libc::ESRCH));
        }
        Ok(self.execution.root.get_or_init(|| root).as_fd())
    }

    fn tgid(&self) -> io::Result<i32> {
        self.request
            .status()
            .map(|status| status.tgid)
            .ok_or_else(|| errno(libc::ESRCH))
    }

    /// The absolute path (in the sandbox's view) the target means by
    /// `(dirfd, path argument)`.
    fn absolute_path(&self, dirfd: i32, raw: &[u8]) -> io::Result<PathBuf> {
        if raw.is_empty() {
            return Err(errno(libc::ENOENT));
        }
        let raw = Path::new(OsStr::from_bytes(raw));
        if raw.is_absolute() {
            return Ok(raw.to_path_buf());
        }
        let link = if dirfd == libc::AT_FDCWD {
            format!("/proc/{}/cwd", self.request.tid)
        } else {
            format!("/proc/{}/fd/{dirfd}", self.request.tid)
        };
        // Read from outside, these render in the sandbox's view.
        let base = std::fs::read_link(link).map_err(|_| errno(libc::EBADF))?;
        if is_deleted(&base) {
            return Err(errno(libc::ENOENT));
        }
        if !base.is_absolute() {
            return Err(errno(libc::ENOTDIR));
        }
        Ok(base.join(raw))
    }

    fn resolve(&self, view: View, dirfd: i32, path_arg: usize, mode: Mode) -> io::Result<Resolved> {
        let raw = self.request.read_string(path_arg)?;
        self.resolve_bytes(view, dirfd, &raw, mode)
    }

    fn resolve_bytes(
        &self,
        view: View,
        dirfd: i32,
        raw: &[u8],
        mode: Mode,
    ) -> io::Result<Resolved> {
        let path = self.absolute_path(dirfd, raw)?;
        let root = match view {
            View::Sandbox => self.sandbox_root()?,
            View::Host => self.shared.host_root(),
        };
        let resolver = Resolver {
            root,
            tid: self.request.tid,
            tgid: &|| self.tgid(),
            tree: self.execution.tree,
        };
        let resolved = resolver.resolve(&path, mode)?;
        if !self.request.still_valid() {
            return Err(errno(libc::ESRCH));
        }
        Ok(resolved)
    }

    /// The path of an object we hold, for checking the rules. `None` for
    /// objects without one (pipes and sockets reached through the target's
    /// own `/proc/<pid>/fd`).
    fn object_path(&self, object: &OwnedFd) -> io::Result<Option<PathBuf>> {
        let path = sys::fd_path(object.as_fd())?;
        if !path.is_absolute() {
            return Ok(None);
        }
        if is_deleted(&path) {
            return Err(errno(libc::ENOENT));
        }
        self.check_process_path(&path)?;
        Ok(Some(path))
    }

    fn entry_path(&self, parent: &OwnedFd, name: &CString) -> io::Result<PathBuf> {
        let parent = sys::fd_path(parent.as_fd())?;
        if is_deleted(&parent) || !parent.is_absolute() {
            return Err(errno(libc::ENOENT));
        }
        let path = parent.join(OsStr::from_bytes(name.as_bytes()));
        self.check_process_path(&path)?;
        Ok(path)
    }

    /// Until process rules exist, `/proc/<pid>` of anything outside this
    /// execution is off limits (it would expose e.g. Zed's environment).
    fn check_process_path(&self, path: &Path) -> io::Result<()> {
        match proc_path_pid(path) {
            Some(pid) if !self.execution.tree.contains(pid) => Err(errno(libc::EACCES)),
            _ => Ok(()),
        }
    }

    fn authorize(&self, checks: &[(Permission, &Path)]) -> io::Result<()> {
        let denied = |permission| match permission {
            Permission::Read | Permission::Connect => errno(libc::EACCES),
            // Tools treat `EACCES` as "write-protected" and may prompt (e.g.
            // `rm`), but give up on `EROFS`.
            Permission::Write => errno(libc::EROFS),
        };
        let sandbox_id = self.execution.sandbox.id;
        // Granted once, for this syscall only.
        let mut granted: Vec<RequestedAccess> = Vec::new();
        // Re-evaluated after every prompt, against the live policy: a handler
        // may have remembered rules that now decide the rest.
        loop {
            let policy = self.execution.sandbox.policy();
            // Requests that need asking, grouped by handler; each handler
            // that's involved decides everything it covers in one go.
            let mut asks: Vec<(HandlerId, Permission, Vec<RequestedAccess>)> = Vec::new();
            for &(permission, path) in checks {
                let request = RequestedAccess::Fs {
                    permission,
                    path: path.to_path_buf(),
                };
                if granted.contains(&request) {
                    continue;
                }
                match policy.fs_access(path, permission) {
                    WireAccess::Allow => {}
                    WireAccess::Deny => {
                        self.shared
                            .report_denial(sandbox_id, vec![request], self.request.tid);
                        return Err(denied(permission));
                    }
                    WireAccess::Ask(handler) => {
                        match asks.iter_mut().find(|(id, _, _)| *id == handler) {
                            Some((_, _, requests)) => requests.push(request),
                            None => asks.push((handler, permission, vec![request])),
                        }
                    }
                }
            }
            let Some((handler, first, requests)) = asks.into_iter().next() else {
                return Ok(());
            };
            let still_valid = || self.request.still_valid();
            let sandbox = &self.execution.sandbox;
            match self.shared.ask(
                sandbox,
                handler,
                requests.clone(),
                self.request.tid,
                still_valid,
            ) {
                AskOutcome::Decided(true) => granted.extend(requests),
                AskOutcome::Decided(false) => {
                    self.shared
                        .report_denial(sandbox_id, requests, self.request.tid);
                    return Err(denied(first));
                }
                AskOutcome::Retry => {}
            }
        }
    }

    fn open(&self, dirfd: i32, path_arg: usize, flags: i32, mode: u32) -> io::Result<Response> {
        let access_mode = flags & libc::O_ACCMODE;
        let reads = access_mode != libc::O_WRONLY;
        let writes = access_mode != libc::O_RDONLY || flags & (libc::O_CREAT | libc::O_TRUNC) != 0;
        let exclusive = flags & (libc::O_CREAT | libc::O_EXCL) == libc::O_CREAT | libc::O_EXCL;
        // `O_CREAT | O_EXCL` never follows a final symlink.
        let follow = flags & libc::O_NOFOLLOW == 0 && !exclusive;
        let cloexec = flags & libc::O_CLOEXEC != 0;
        let view = if writes { View::Host } else { View::Sandbox };

        match self.resolve(view, dirfd, path_arg, Mode::Object { follow })? {
            Resolved::Object(object) => {
                if exclusive {
                    return Err(errno(libc::EEXIST));
                }
                let stat = sys::fstat(object.as_fd())?;
                if stat.st_mode & libc::S_IFMT == libc::S_IFLNK {
                    return Err(errno(libc::ELOOP));
                }
                let Some(path) = self.object_path(&object)? else {
                    return Ok(Response::InstallFd {
                        fd: reopen(&object, flags)?,
                        cloexec,
                    });
                };
                let mut checks = Vec::new();
                if writes {
                    checks.push((Permission::Write, path.as_path()));
                }
                // Listing a directory isn't reading contents.
                if reads && stat.st_mode & libc::S_IFMT != libc::S_IFDIR {
                    checks.push((Permission::Read, path.as_path()));
                }
                self.authorize(&checks)?;
                let fd = if is_controlling_terminal_device(&stat) {
                    self.open_controlling_terminal(flags)?
                } else {
                    reopen(&object, flags)?
                };
                Ok(Response::InstallFd { fd, cloexec })
            }
            Resolved::Entry { parent, name } => {
                if flags & libc::O_CREAT == 0 {
                    return Err(errno(libc::ENOENT));
                }
                let path = self.entry_path(&parent, &name)?;
                let mut checks = vec![(Permission::Write, path.as_path())];
                if reads {
                    checks.push((Permission::Read, path.as_path()));
                }
                self.authorize(&checks)?;
                let mode = mode & 0o7777 & !self.request.umask();
                // O_EXCL | O_NOFOLLOW: create exactly the entry we checked.
                match sys::openat(
                    parent.as_raw_fd(),
                    &name,
                    flags | libc::O_EXCL | libc::O_NOFOLLOW | libc::O_NOCTTY,
                    mode,
                ) {
                    Ok(fd) => {
                        // Our own umask applied too; restore the target's intent.
                        // SAFETY: fchmod on an fd we own.
                        cvt(unsafe { libc::fchmod(fd.as_raw_fd(), mode) })?;
                        Ok(Response::InstallFd { fd, cloexec })
                    }
                    // Someone created it since we looked (e.g. a concurrent
                    // create that shared our prompt). Without O_EXCL the kernel
                    // would open it, so open it too: the same entry we checked,
                    // never through a symlink.
                    Err(error) if error.raw_os_error() == Some(libc::EEXIST) && !exclusive => {
                        let fd = sys::openat(
                            parent.as_raw_fd(),
                            &name,
                            flags & !libc::O_CREAT | libc::O_NOFOLLOW | libc::O_NOCTTY,
                            0,
                        )?;
                        Ok(Response::InstallFd { fd, cloexec })
                    }
                    Err(error) => Err(error),
                }
            }
        }
    }

    /// `/dev/tty` means "my controlling terminal": opened by us it would be
    /// ours, so open the target's instead.
    fn open_controlling_terminal(&self, flags: i32) -> io::Result<OwnedFd> {
        let device = process::controlling_terminal(self.tgid()?).unwrap_or(0);
        let (major, minor) = (libc::major(device), libc::minor(device));
        let path = match major {
            0 => return Err(errno(libc::ENXIO)),
            136..=143 => format!("/dev/pts/{}", (major - 136) * 256 + minor),
            4 if minor < 64 => format!("/dev/tty{minor}"),
            _ => return Err(errno(libc::ENXIO)),
        };
        sys::open(
            Path::new(&path),
            flags & !(libc::O_CREAT | libc::O_EXCL | libc::O_TRUNC) | libc::O_NOCTTY,
        )
    }

    fn entry(&self, dirfd: i32, path_arg: usize) -> io::Result<(OwnedFd, CString, PathBuf)> {
        self.entry_bytes(dirfd, &self.request.read_string(path_arg)?, libc::EINVAL)
    }

    /// A directory entry to create or remove, as `(parent, name, path)`.
    /// Fails with `exists` if `raw` names an existing object.
    fn entry_bytes(
        &self,
        dirfd: i32,
        raw: &[u8],
        exists: i32,
    ) -> io::Result<(OwnedFd, CString, PathBuf)> {
        match self.resolve_bytes(View::Host, dirfd, raw, Mode::Entry)? {
            Resolved::Entry { parent, name } => {
                let path = self.entry_path(&parent, &name)?;
                Ok((parent, name, path))
            }
            Resolved::Object(_) => Err(errno(exists)),
        }
    }

    fn mkdir(&self, dirfd: i32, path_arg: usize, mode: u32) -> io::Result<Response> {
        let (parent, name, path) = self.entry(dirfd, path_arg)?;
        self.authorize(&[(Permission::Write, &path)])?;
        let mode = mode & 0o7777 & !self.request.umask();
        // SAFETY: *at calls on a directory fd we own with a single name.
        unsafe {
            cvt(libc::mkdirat(parent.as_raw_fd(), name.as_ptr(), mode))?;
            cvt(libc::fchmodat(parent.as_raw_fd(), name.as_ptr(), mode, 0))?;
        }
        Ok(Response::Success)
    }

    fn unlink(&self, dirfd: i32, path_arg: usize, flags: i32) -> io::Result<Response> {
        let (parent, name, path) = self.entry(dirfd, path_arg)?;
        self.authorize(&[(Permission::Write, &path)])?;
        // SAFETY: unlinkat on a directory fd we own with a single name.
        cvt(unsafe { libc::unlinkat(parent.as_raw_fd(), name.as_ptr(), flags) })?;
        Ok(Response::Success)
    }

    /// Whether moving the entry at `from` to `to` keeps every rule's verdict
    /// for the moved contents from widening.
    fn check_move(
        &self,
        parent: &OwnedFd,
        name: &CString,
        from: &Path,
        to: &Path,
    ) -> io::Result<()> {
        let fs = self.policy;
        let allowed = if is_directory_entry(parent, name)? {
            Permission::ALL
                .iter()
                .all(|permission| fs.same_beneath(from, to, *permission))
        } else {
            !Permission::ALL
                .iter()
                .any(|permission| fs.widens(from, to, *permission))
        };
        if allowed {
            Ok(())
        } else {
            Err(errno(libc::EACCES))
        }
    }

    fn rename(&self, old: (i32, usize), new: (i32, usize), flags: u32) -> io::Result<Response> {
        let (old_parent, old_name, old_path) = self.entry(old.0, old.1)?;
        let (new_parent, new_name, new_path) = self.entry(new.0, new.1)?;
        self.authorize(&[
            (Permission::Write, &old_path),
            (Permission::Write, &new_path),
        ])?;
        self.check_move(&old_parent, &old_name, &old_path, &new_path)?;
        if flags & libc::RENAME_EXCHANGE != 0 {
            self.check_move(&new_parent, &new_name, &new_path, &old_path)?;
        }
        // SAFETY: renameat2 on directory fds we own with single names.
        let result = unsafe {
            libc::syscall(
                libc::SYS_renameat2,
                old_parent.as_raw_fd(),
                old_name.as_ptr(),
                new_parent.as_raw_fd(),
                new_name.as_ptr(),
                flags,
            )
        };
        cvt(result as i32)?;
        Ok(Response::Success)
    }

    fn link(&self, old: (i32, usize), new: (i32, usize), flags: i32) -> io::Result<Response> {
        if flags & libc::AT_EMPTY_PATH != 0 {
            return Err(errno(libc::EPERM));
        }
        let (new_parent, new_name, new_path) = self.entry(new.0, new.1)?;
        // A new name must not reach the file with access its old one lacked.
        let check_source = |source: &Path| {
            self.authorize(&[(Permission::Write, &new_path)])?;
            let fs = self.policy;
            if Permission::ALL
                .iter()
                .any(|permission| fs.widens(source, &new_path, *permission))
            {
                return Err(errno(libc::EACCES));
            }
            Ok(())
        };
        let result = if flags & libc::AT_SYMLINK_FOLLOW != 0 {
            let Resolved::Object(object) =
                self.resolve(View::Host, old.0, old.1, Mode::Object { follow: true })?
            else {
                return Err(errno(libc::ENOENT));
            };
            let source = self
                .object_path(&object)?
                .ok_or_else(|| errno(libc::ENOENT))?;
            check_source(&source)?;
            // SAFETY: linkat through our own fd's magic link.
            unsafe {
                libc::linkat(
                    libc::AT_FDCWD,
                    path_cstring(&sys::proc_self_fd(object.as_fd()))?.as_ptr(),
                    new_parent.as_raw_fd(),
                    new_name.as_ptr(),
                    libc::AT_SYMLINK_FOLLOW,
                )
            }
        } else {
            let (old_parent, old_name, source) = self.entry(old.0, old.1)?;
            check_source(&source)?;
            // SAFETY: linkat on directory fds we own with single names.
            unsafe {
                libc::linkat(
                    old_parent.as_raw_fd(),
                    old_name.as_ptr(),
                    new_parent.as_raw_fd(),
                    new_name.as_ptr(),
                    0,
                )
            }
        };
        cvt(result)?;
        Ok(Response::Success)
    }

    fn symlink(&self, content_arg: usize, link: (i32, usize)) -> io::Result<Response> {
        // The link's content is never resolved by us.
        let content = sys::cstring(&self.request.read_string(content_arg)?)?;
        let (parent, name, path) = self.entry(link.0, link.1)?;
        self.authorize(&[(Permission::Write, &path)])?;
        // SAFETY: symlinkat on a directory fd we own with a single name.
        cvt(unsafe { libc::symlinkat(content.as_ptr(), parent.as_raw_fd(), name.as_ptr()) })?;
        Ok(Response::Success)
    }

    /// Shared shape of chmod/truncate/access/utimens: act on an existing
    /// object through its `/proc/self/fd` magic link.
    fn with_object(
        &self,
        dirfd: i32,
        path_arg: usize,
        follow: bool,
        act: impl FnOnce(&CString) -> libc::c_int,
    ) -> io::Result<Response> {
        let Resolved::Object(object) =
            self.resolve(View::Host, dirfd, path_arg, Mode::Object { follow })?
        else {
            return Err(errno(libc::ENOENT));
        };
        let path = self
            .object_path(&object)?
            .ok_or_else(|| errno(libc::EPERM))?;
        self.authorize(&[(Permission::Write, &path)])?;
        cvt(act(&path_cstring(&sys::proc_self_fd(object.as_fd()))?))?;
        Ok(Response::Success)
    }

    fn chmod(&self, dirfd: i32, path_arg: usize, mode: u32) -> io::Result<Response> {
        // SAFETY: fchmodat on a NUL-terminated path.
        self.with_object(dirfd, path_arg, true, |path| unsafe {
            libc::fchmodat(libc::AT_FDCWD, path.as_ptr(), mode, 0)
        })
    }

    fn truncate(&self, length: i64) -> io::Result<Response> {
        // SAFETY: truncate on a NUL-terminated path.
        self.with_object(libc::AT_FDCWD, 0, true, |path| unsafe {
            libc::truncate(path.as_ptr(), length)
        })
    }

    fn access(&self, dirfd: i32, path_arg: usize, mode: i32, flags: i32) -> io::Result<Response> {
        let follow = flags & libc::AT_SYMLINK_NOFOLLOW == 0;
        // SAFETY: faccessat on a NUL-terminated path.
        self.with_object(dirfd, path_arg, follow, |path| unsafe {
            libc::faccessat(
                libc::AT_FDCWD,
                path.as_ptr(),
                mode,
                flags & libc::AT_EACCESS,
            )
        })
    }

    fn utimens(&self) -> io::Result<Response> {
        let request = self.request;
        let times = match request.args[2] {
            0 => None,
            address => {
                let bytes = request.read_bytes(address, 32)?;
                // Decode field by field; never transmute target bytes.
                let field = |offset: usize| -> i64 {
                    let mut value = [0u8; 8];
                    value.copy_from_slice(&bytes[offset..offset + 8]);
                    i64::from_ne_bytes(value)
                };
                Some([
                    libc::timespec {
                        tv_sec: field(0),
                        tv_nsec: field(8),
                    },
                    libc::timespec {
                        tv_sec: field(16),
                        tv_nsec: field(24),
                    },
                ])
            }
        };
        let follow = request.int(3) & libc::AT_SYMLINK_NOFOLLOW == 0;
        self.with_object(request.int(0), 1, follow, |path| {
            let times = times
                .as_ref()
                .map_or(std::ptr::null(), |times| times.as_ptr());
            // SAFETY: NUL-terminated path; null or two valid timespecs.
            unsafe { libc::utimensat(libc::AT_FDCWD, path.as_ptr(), times, 0) }
        })
    }

    fn continue_if_member(&self, pid: i32, error: i32) -> io::Result<Response> {
        if self.execution.tree.contains(pid) {
            Ok(Response::Continue)
        } else {
            Err(errno(error))
        }
    }

    /// `kill`. Process groups and `-1` may include processes outside the
    /// sandbox (e.g. Zed itself, if the sandbox inherited its group), so
    /// those are emulated by signalling only the members.
    fn kill(&self, pid: i32, signal: i32) -> io::Result<Response> {
        if pid > 0 {
            return self.continue_if_member(pid, libc::EPERM);
        }
        let caller = self.tgid()?;
        let group = match pid {
            0 => {
                process::process_info(caller)
                    .ok_or_else(|| errno(libc::ESRCH))?
                    .group
            }
            -1 => 0,
            _ => pid.checked_neg().ok_or_else(|| errno(libc::EINVAL))?,
        };
        let in_scope: Vec<i32> = process::all_processes()
            .into_iter()
            .filter(|process| pid == -1 || process.group == group)
            .map(|process| process.pid)
            .collect();
        let members: HashSet<i32> = self
            .execution
            .tree
            .members()
            .into_iter()
            .map(|process| process.pid)
            .collect();
        if pid != -1 && !in_scope.is_empty() && in_scope.iter().all(|pid| members.contains(pid)) {
            return Ok(Response::Continue);
        }
        let targets: Vec<i32> = in_scope
            .into_iter()
            .filter(|target| members.contains(target) && !(pid == -1 && *target == caller))
            .collect();
        if targets.is_empty() {
            return Err(errno(libc::ESRCH));
        }
        if signal != 0 {
            for target in targets {
                // SAFETY: plain kill.
                unsafe { libc::kill(target, signal) };
            }
        }
        Ok(Response::Success)
    }

    /// `setpriority` / `ioprio_set`: a process, a process group (all of whose
    /// members must be ours), or a user (never: it covers every process of
    /// the user).
    fn process_or_group(
        &self,
        which: i32,
        who: i32,
        process: i32,
        group: i32,
    ) -> io::Result<Response> {
        if which == process {
            return match who {
                0 => Ok(Response::Continue),
                pid => self.continue_if_member(pid, libc::EPERM),
            };
        }
        if which != group {
            return Err(errno(libc::EPERM));
        }
        let group = match who {
            0 => {
                process::process_info(self.tgid()?)
                    .ok_or_else(|| errno(libc::ESRCH))?
                    .group
            }
            group => group,
        };
        let tree = self.execution.tree;
        let all_members = process::all_processes()
            .into_iter()
            .filter(|process| process.group == group)
            .all(|process| tree.contains(process.pid));
        if all_members {
            Ok(Response::Continue)
        } else {
            Err(errno(libc::EPERM))
        }
    }
}

fn is_deleted(path: &Path) -> bool {
    path.as_os_str().as_bytes().ends_with(b" (deleted)")
}

fn is_controlling_terminal_device(stat: &libc::stat) -> bool {
    stat.st_mode & libc::S_IFMT == libc::S_IFCHR && stat.st_rdev == libc::makedev(5, 0)
}

fn is_directory_entry(parent: &OwnedFd, name: &CString) -> io::Result<bool> {
    // SAFETY: plain old data, filled in by the kernel.
    let mut stat: libc::stat = unsafe { std::mem::zeroed() };
    // SAFETY: valid directory fd, NUL-terminated name and stat pointer.
    cvt(unsafe {
        libc::fstatat(
            parent.as_raw_fd(),
            name.as_ptr(),
            &mut stat,
            libc::AT_SYMLINK_NOFOLLOW,
        )
    })?;
    Ok(stat.st_mode & libc::S_IFMT == libc::S_IFDIR)
}

/// Open the object `object` refers to with the target's flags. Through the
/// magic link, the new fd uses the mount `object` was found on.
fn reopen(object: &OwnedFd, flags: i32) -> io::Result<OwnedFd> {
    sys::open(
        &sys::proc_self_fd(object.as_fd()),
        flags & !(libc::O_CREAT | libc::O_EXCL | libc::O_NOFOLLOW) | libc::O_NOCTTY,
    )
}

fn send_response(request: &Request, response: Response) {
    let listener = request.listener.as_raw_fd();
    let mut reply = NotificationResponse {
        id: request.id,
        ..Default::default()
    };
    match response {
        Response::Continue => reply.flags = SECCOMP_USER_NOTIF_FLAG_CONTINUE,
        Response::Errno(code) => reply.error = -code,
        Response::Success => {}
        Response::InstallFd { fd, cloexec } => {
            let mut add = AddFd {
                id: request.id,
                flags: SECCOMP_ADDFD_FLAG_SEND,
                srcfd: fd.as_raw_fd() as u32,
                newfd: 0,
                newfd_flags: if cloexec { libc::O_CLOEXEC as u32 } else { 0 },
            };
            // SAFETY: ADDFD with a correctly laid out struct. With
            // ADDFD_FLAG_SEND (5.14) it also answers the syscall atomically.
            let result = unsafe {
                libc::ioctl(
                    listener,
                    SECCOMP_IOCTL_NOTIF_ADDFD as _,
                    &mut add as *mut AddFd,
                )
            };
            if result >= 0 || io::Error::last_os_error().raw_os_error() != Some(libc::EINVAL) {
                // Done, or the target is gone (ENOENT) and needs no answer.
                return;
            }
            // Pre-5.14: install, then answer separately.
            add.flags = 0;
            // SAFETY: as above.
            let installed = unsafe {
                libc::ioctl(
                    listener,
                    SECCOMP_IOCTL_NOTIF_ADDFD as _,
                    &mut add as *mut AddFd,
                )
            };
            if installed < 0 {
                reply.error = -io::Error::last_os_error()
                    .raw_os_error()
                    .unwrap_or(libc::EIO);
            } else {
                reply.val = installed as i64;
            }
        }
    }
    // SAFETY: SEND with a correctly laid out struct. Failure means the target
    // was interrupted or died, which needs no handling.
    unsafe {
        libc::ioctl(
            listener,
            SECCOMP_IOCTL_NOTIF_SEND as _,
            &mut reply as *mut NotificationResponse,
        );
    }
}
