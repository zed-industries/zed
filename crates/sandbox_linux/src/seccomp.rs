//! The seccomp filter installed on the untrusted command.
//!
//! It sends to the supervisor (`SECCOMP_RET_USER_NOTIF`):
//! - every open except `O_PATH` (read rules need the supervisor to decide);
//! - write-intent filesystem syscalls, decided from register arguments;
//! - syscalls that act on another process by pid;
//! - `connect` and `bind`.
//!
//! It denies syscalls the sandbox never needs (non-IP sockets, new user
//! namespaces, io_uring, ptrace, ...) and returns `ENOSYS` for every syscall
//! newer than the ones listed here, so a future kernel's additions are
//! unavailable rather than unreviewed.

use std::io;
use std::os::fd::{FromRawFd as _, OwnedFd, RawFd};

use crate::sys::cvt;

const SECCOMP_FILTER_FLAG_NEW_LISTENER: libc::c_ulong = 1 << 3;
const SECCOMP_FILTER_FLAG_WAIT_KILLABLE_RECV: libc::c_ulong = 1 << 5;
const SECCOMP_RET_USER_NOTIF: u32 = 0x7fc0_0000;
const SECCOMP_RET_KILL_PROCESS: u32 = 0x8000_0000;

// Syscalls added after the per-arch tables were unified share one number.
const SYS_PIDFD_OPEN: i64 = 434;
const SYS_CLONE3: i64 = 435;
const SYS_OPENAT2: i64 = 437;
const SYS_FACCESSAT2: i64 = 439;
const SYS_PROCESS_MADVISE: i64 = 440;
const SYS_FCHMODAT2: i64 = 452;
/// The first syscall number this filter hasn't been reviewed against
/// (`setxattrat`, Linux 6.13).
const FIRST_UNREVIEWED_SYSCALL: u32 = 463;

#[cfg(target_arch = "x86_64")]
const AUDIT_ARCH: u32 = 0xc000_003e;
#[cfg(target_arch = "aarch64")]
const AUDIT_ARCH: u32 = 0xc000_00b7;
#[cfg(target_arch = "x86_64")]
const X32_SYSCALL_BIT: u32 = 0x4000_0000;

/// The syscalls the supervisor handles. Anything else that writes (e.g.
/// `fchownat`, `setxattr`, `mknodat`) isn't trapped yet and fails against the
/// read-only mounts. TODO: broker every write-capable path syscall.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Brokered {
    OpenAt,
    #[cfg(target_arch = "x86_64")]
    Open,
    #[cfg(target_arch = "x86_64")]
    Creat,
    MkdirAt,
    #[cfg(target_arch = "x86_64")]
    Mkdir,
    UnlinkAt,
    #[cfg(target_arch = "x86_64")]
    Unlink,
    #[cfg(target_arch = "x86_64")]
    Rmdir,
    #[cfg(target_arch = "x86_64")]
    RenameAt,
    RenameAt2,
    #[cfg(target_arch = "x86_64")]
    Rename,
    LinkAt,
    #[cfg(target_arch = "x86_64")]
    Link,
    SymlinkAt,
    #[cfg(target_arch = "x86_64")]
    Symlink,
    FchmodAt,
    #[cfg(target_arch = "x86_64")]
    Chmod,
    Truncate,
    FaccessAt,
    FaccessAt2,
    #[cfg(target_arch = "x86_64")]
    Access,
    UtimensAt,
    Kill,
    Tkill,
    Tgkill,
    RtSigqueueinfo,
    RtTgsigqueueinfo,
    PidfdOpen,
    // Argument 0 is a pid, with 0 meaning the caller.
    Prlimit64,
    SchedSetaffinity,
    SchedSetscheduler,
    SchedSetparam,
    SchedSetattr,
    Setpriority,
    IoprioSet,
    Connect,
    Bind,
}

/// What the filter does with one syscall number.
#[derive(Clone, Copy)]
enum Action {
    Return(u32),
    /// Do `matched` if argument `arg`, masked with `mask`, equals one of
    /// `values`, else do `otherwise`.
    ArgumentIn {
        arg: usize,
        mask: u32,
        values: &'static [u32],
        matched: &'static Action,
        otherwise: &'static Action,
    },
    /// Return `set` if argument `arg` has any bit of `mask` set.
    ArgumentHasBits {
        arg: usize,
        mask: u32,
        set: u32,
        clear: u32,
    },
    /// Return `non_null` if the 64-bit argument `arg` is non-zero.
    ArgumentNonNull {
        arg: usize,
        non_null: u32,
        null: u32,
    },
}

const ALLOW: u32 = libc::SECCOMP_RET_ALLOW;
const NOTIFY: u32 = SECCOMP_RET_USER_NOTIF;

const fn errno(code: i32) -> u32 {
    libc::SECCOMP_RET_ERRNO | (code as u32 & libc::SECCOMP_RET_DATA)
}

/// `O_PATH` opens grant no access to contents; every other open is brokered.
const fn opens_contents(arg: usize) -> Action {
    Action::ArgumentHasBits {
        arg,
        mask: libc::O_PATH as u32,
        set: ALLOW,
        clear: NOTIFY,
    }
}

const fn checks_write_access(arg: usize) -> Action {
    Action::ArgumentHasBits {
        arg,
        mask: libc::W_OK as u32,
        set: NOTIFY,
        clear: ALLOW,
    }
}

const ALWAYS_NOTIFY: Action = Action::Return(NOTIFY);

/// Syscalls the supervisor handles, with when to send them to it.
const BROKERED: &[(i64, Brokered, Action)] = {
    use Brokered::*;
    let always = ALWAYS_NOTIFY;
    &[
        (libc::SYS_openat, OpenAt, opens_contents(2)),
        (libc::SYS_mkdirat, MkdirAt, always),
        (libc::SYS_unlinkat, UnlinkAt, always),
        (libc::SYS_renameat2, RenameAt2, always),
        (libc::SYS_linkat, LinkAt, always),
        (libc::SYS_symlinkat, SymlinkAt, always),
        (libc::SYS_fchmodat, FchmodAt, always),
        (libc::SYS_truncate, Truncate, always),
        (libc::SYS_faccessat, FaccessAt, checks_write_access(2)),
        (SYS_FACCESSAT2, FaccessAt2, checks_write_access(2)),
        // `utimensat(fd, NULL, ...)` is `futimens` on an already-open fd.
        (
            libc::SYS_utimensat,
            UtimensAt,
            Action::ArgumentNonNull {
                arg: 1,
                non_null: NOTIFY,
                null: ALLOW,
            },
        ),
        (libc::SYS_kill, Kill, always),
        (libc::SYS_tkill, Tkill, always),
        (libc::SYS_tgkill, Tgkill, always),
        (libc::SYS_rt_sigqueueinfo, RtSigqueueinfo, always),
        (libc::SYS_rt_tgsigqueueinfo, RtTgsigqueueinfo, always),
        (SYS_PIDFD_OPEN, PidfdOpen, always),
        (libc::SYS_prlimit64, Prlimit64, always),
        (libc::SYS_sched_setaffinity, SchedSetaffinity, always),
        (libc::SYS_sched_setscheduler, SchedSetscheduler, always),
        (libc::SYS_sched_setparam, SchedSetparam, always),
        (libc::SYS_sched_setattr, SchedSetattr, always),
        (libc::SYS_setpriority, Setpriority, always),
        (libc::SYS_ioprio_set, IoprioSet, always),
        (libc::SYS_connect, Connect, always),
        (libc::SYS_bind, Bind, always),
    ]
};

/// The pre-`*at` path syscalls that only x86_64 still has.
#[cfg(target_arch = "x86_64")]
const BROKERED_LEGACY: &[(i64, Brokered, Action)] = {
    use Brokered::*;
    let always = ALWAYS_NOTIFY;
    &[
        (libc::SYS_open, Open, opens_contents(1)),
        (libc::SYS_creat, Creat, always),
        (libc::SYS_mkdir, Mkdir, always),
        (libc::SYS_unlink, Unlink, always),
        (libc::SYS_rmdir, Rmdir, always),
        (libc::SYS_renameat, RenameAt, always),
        (libc::SYS_rename, Rename, always),
        (libc::SYS_link, Link, always),
        (libc::SYS_symlink, Symlink, always),
        (libc::SYS_chmod, Chmod, always),
        (libc::SYS_access, Access, checks_write_access(1)),
    ]
};

#[cfg(not(target_arch = "x86_64"))]
const BROKERED_LEGACY: &[(i64, Brokered, Action)] = &[];

fn brokered_entries() -> impl Iterator<Item = &'static (i64, Brokered, Action)> {
    BROKERED.iter().chain(BROKERED_LEGACY)
}

pub(crate) fn brokered_syscall(number: i64) -> Option<Brokered> {
    brokered_entries()
        .find_map(|&(candidate, brokered, _)| (candidate == number).then_some(brokered))
}

const DENIED_WITH_EPERM: Action = Action::Return(errno(libc::EPERM));
const DENIED_WITH_ENOSYS: Action = Action::Return(errno(libc::ENOSYS));

/// Syscalls the sandbox never needs, with how they're refused.
const DENIED: &[(i64, Action)] = {
    const IP_FAMILIES: &[u32] = &[
        libc::AF_INET as u32,
        libc::AF_INET6 as u32,
        libc::AF_NETLINK as u32,
    ];
    const UNIX_FAMILY: &[u32] = &[libc::AF_UNIX as u32];
    const ICMP_PROTOCOLS: &[u32] = &[libc::IPPROTO_ICMP as u32, libc::IPPROTO_ICMPV6 as u32];
    const DATAGRAM: &[u32] = &[libc::SOCK_DGRAM as u32];
    /// The type argument without `SOCK_NONBLOCK` / `SOCK_CLOEXEC`.
    const SOCKET_TYPE_MASK: u32 = 0xf;
    const ALLOWED: &Action = &Action::Return(ALLOW);
    const REFUSED: &Action = &DENIED_WITH_EPERM;
    // A datagram unix socket can send to any socket by path with
    // `sendmsg`, whose address sits behind a pointer; trapping every
    // `sendmsg` would slow all UDP. Stream and seqpacket sockets only reach
    // a path through `connect`, which is brokered.
    const UNIX_UNLESS_DATAGRAM: &Action = &Action::ArgumentIn {
        arg: 1,
        mask: SOCKET_TYPE_MASK,
        values: DATAGRAM,
        matched: REFUSED,
        otherwise: ALLOWED,
    };
    const NEW_USER_NAMESPACE: Action = Action::ArgumentHasBits {
        arg: 0,
        mask: libc::CLONE_NEWUSER as u32,
        set: errno(libc::EPERM),
        clear: ALLOW,
    };
    &[
        (
            libc::SYS_socket,
            Action::ArgumentIn {
                // Ping sockets: every address is local in the sandbox, so
                // every ping would get an answer.
                arg: 2,
                mask: u32::MAX,
                values: ICMP_PROTOCOLS,
                matched: &Action::Return(errno(libc::EACCES)),
                otherwise: &Action::ArgumentIn {
                    arg: 0,
                    mask: u32::MAX,
                    values: IP_FAMILIES,
                    matched: ALLOWED,
                    otherwise: &Action::ArgumentIn {
                        arg: 0,
                        mask: u32::MAX,
                        values: UNIX_FAMILY,
                        matched: UNIX_UNLESS_DATAGRAM,
                        otherwise: REFUSED,
                    },
                },
            },
        ),
        (
            libc::SYS_socketpair,
            Action::ArgumentIn {
                arg: 0,
                mask: u32::MAX,
                values: UNIX_FAMILY,
                matched: UNIX_UNLESS_DATAGRAM,
                otherwise: REFUSED,
            },
        ),
        // A nested user namespace could mount, creating new names for files
        // the read rules deny.
        (libc::SYS_unshare, NEW_USER_NAMESPACE),
        (libc::SYS_clone, NEW_USER_NAMESPACE),
        // Flags live behind a pointer; libc falls back to `clone`.
        (SYS_CLONE3, DENIED_WITH_ENOSYS),
        // Flags/modes live behind pointers; libc falls back to `openat` /
        // `fchmodat`, which we can inspect.
        (SYS_OPENAT2, DENIED_WITH_ENOSYS),
        (SYS_FCHMODAT2, DENIED_WITH_ENOSYS),
        (libc::SYS_io_uring_setup, DENIED_WITH_EPERM),
        (libc::SYS_io_uring_enter, DENIED_WITH_EPERM),
        (libc::SYS_io_uring_register, DENIED_WITH_EPERM),
        (libc::SYS_ptrace, DENIED_WITH_EPERM),
        (libc::SYS_process_vm_readv, DENIED_WITH_EPERM),
        (libc::SYS_process_vm_writev, DENIED_WITH_EPERM),
        (libc::SYS_setns, DENIED_WITH_EPERM),
        (libc::SYS_perf_event_open, DENIED_WITH_EPERM),
        (libc::SYS_kcmp, DENIED_WITH_EPERM),
        (libc::SYS_migrate_pages, DENIED_WITH_EPERM),
        (libc::SYS_move_pages, DENIED_WITH_EPERM),
        (SYS_PROCESS_MADVISE, DENIED_WITH_EPERM),
    ]
};

fn statement(code: u32, k: u32) -> libc::sock_filter {
    libc::sock_filter {
        code: code as u16,
        jt: 0,
        jf: 0,
        k,
    }
}

fn jump(code: u32, k: u32, jt: u8, jf: u8) -> libc::sock_filter {
    libc::sock_filter {
        code: code as u16,
        jt,
        jf,
        k,
    }
}

// Classic BPF opcodes. The filter is a tiny program over one register (the
// accumulator); each opcode ORs together an instruction class, an operand
// size and an addressing mode.

/// `A = *(u32 *)(seccomp_data + k)`: load a 32-bit field of the syscall.
const LOAD_OPCODE: u32 = libc::BPF_LD | libc::BPF_W | libc::BPF_ABS;
/// `if A == k` jump `jt` instructions forward, else `jf`.
const JUMP_EQUAL_OPCODE: u32 = libc::BPF_JMP | libc::BPF_JEQ | libc::BPF_K;
/// `if A >= k` (unsigned) jump `jt`, else `jf`.
const JUMP_AT_LEAST_OPCODE: u32 = libc::BPF_JMP | libc::BPF_JGE | libc::BPF_K;
/// `if A & k != 0` jump `jt`, else `jf`.
const JUMP_ANY_BITS_OPCODE: u32 = libc::BPF_JMP | libc::BPF_JSET | libc::BPF_K;
/// `A &= k`.
const AND_OPCODE: u32 = libc::BPF_ALU | libc::BPF_AND | libc::BPF_K;
/// Stop and return `k`: the seccomp verdict (allow, errno, notify, kill).
const RETURN_OPCODE: u32 = libc::BPF_RET | libc::BPF_K;

// Field offsets in `struct seccomp_data` (the program's input).
const NR_OFFSET: u32 = 0;
const ARCH_OFFSET: u32 = 4;

/// Offset of the low 32 bits of argument `index` in `seccomp_data` (both
/// supported architectures are little-endian).
fn argument_offset(index: usize) -> u32 {
    16 + 8 * index as u32
}

fn compile_action(action: Action) -> Vec<libc::sock_filter> {
    match action {
        Action::Return(value) => vec![statement(RETURN_OPCODE, value)],
        Action::ArgumentIn {
            arg,
            mask,
            values,
            matched,
            otherwise,
        } => {
            // Comparisons, then `otherwise` (reached by falling through
            // them all), then `matched`. Both end in a return.
            let otherwise = compile_action(*otherwise);
            let mut program = vec![statement(LOAD_OPCODE, argument_offset(arg))];
            if mask != u32::MAX {
                program.push(statement(AND_OPCODE, mask));
            }
            let count = values.len();
            for (index, value) in values.iter().enumerate() {
                // Jump over the remaining comparisons and `otherwise`.
                let skip = count - index - 1 + otherwise.len();
                program.push(jump(JUMP_EQUAL_OPCODE, *value, skip as u8, 0));
            }
            program.extend(otherwise);
            program.extend(compile_action(*matched));
            program
        }
        Action::ArgumentHasBits {
            arg,
            mask,
            set,
            clear,
        } => vec![
            statement(LOAD_OPCODE, argument_offset(arg)),
            jump(JUMP_ANY_BITS_OPCODE, mask, 0, 1),
            statement(RETURN_OPCODE, set),
            statement(RETURN_OPCODE, clear),
        ],
        Action::ArgumentNonNull {
            arg,
            non_null,
            null,
        } => vec![
            statement(LOAD_OPCODE, argument_offset(arg)),
            jump(JUMP_EQUAL_OPCODE, 0, 0, 2),
            statement(LOAD_OPCODE, argument_offset(arg) + 4),
            jump(JUMP_EQUAL_OPCODE, 0, 1, 0),
            statement(RETURN_OPCODE, non_null),
            statement(RETURN_OPCODE, null),
        ],
    }
}

pub(crate) fn build_filter() -> Vec<libc::sock_filter> {
    let mut program = vec![
        statement(LOAD_OPCODE, ARCH_OFFSET),
        jump(JUMP_EQUAL_OPCODE, AUDIT_ARCH, 1, 0),
        statement(RETURN_OPCODE, SECCOMP_RET_KILL_PROCESS),
        statement(LOAD_OPCODE, NR_OFFSET),
    ];
    #[cfg(target_arch = "x86_64")]
    program.extend([
        jump(JUMP_ANY_BITS_OPCODE, X32_SYSCALL_BIT, 0, 1),
        statement(RETURN_OPCODE, SECCOMP_RET_KILL_PROCESS),
    ]);
    program.extend([
        jump(JUMP_AT_LEAST_OPCODE, FIRST_UNREVIEWED_SYSCALL, 0, 1),
        statement(RETURN_OPCODE, errno(libc::ENOSYS)),
    ]);

    let entries = DENIED
        .iter()
        .copied()
        .chain(brokered_entries().map(|&(number, _, action)| (number, action)));
    for (number, action) in entries {
        // Every compiled action ends in a return, so the accumulator (the
        // syscall number) is intact on the fall-through path.
        let body = compile_action(action);
        program.push(jump(JUMP_EQUAL_OPCODE, number as u32, 0, body.len() as u8));
        program.extend(body);
    }
    program.push(statement(RETURN_OPCODE, ALLOW));
    program
}

/// Install `filter` on the calling thread and return the listener fd. Called
/// after `fork`, before `exec`.
pub(crate) fn install(filter: &[libc::sock_filter]) -> io::Result<OwnedFd> {
    let program = libc::sock_fprog {
        len: filter.len() as u16,
        filter: filter.as_ptr().cast_mut(),
    };
    // SAFETY: plain prctl.
    cvt(unsafe { libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) })?;
    let install = |flags: libc::c_ulong| {
        // SAFETY: `program` points at `filter`, which outlives the call.
        unsafe {
            libc::syscall(
                libc::SYS_seccomp,
                libc::SECCOMP_SET_MODE_FILTER,
                flags,
                &program as *const libc::sock_fprog,
            )
        }
    };
    // WAIT_KILLABLE_RECV (5.19): once the supervisor has picked up a
    // notification, only a fatal signal can interrupt it. Otherwise a signal
    // (e.g. Go's SIGURG preemption) arriving while we perform a mkdir or
    // rename, or while a prompt is open, makes the target retry it.
    let mut listener =
        install(SECCOMP_FILTER_FLAG_NEW_LISTENER | SECCOMP_FILTER_FLAG_WAIT_KILLABLE_RECV);
    if listener < 0 && io::Error::last_os_error().raw_os_error() == Some(libc::EINVAL) {
        listener = install(SECCOMP_FILTER_FLAG_NEW_LISTENER);
    }
    if listener < 0 {
        return Err(io::Error::last_os_error());
    }
    // SAFETY: a freshly returned descriptor that nothing else owns.
    Ok(unsafe { OwnedFd::from_raw_fd(listener as RawFd) })
}
