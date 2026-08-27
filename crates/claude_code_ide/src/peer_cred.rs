//! The pid of a connected Unix-socket client, read from the kernel
//! (`LOCAL_PEERPID` on macOS, `SO_PEERCRED` on Linux) so a client cannot forge it.
//!
//! Used only to know whether the process that pushed a selection is still alive, so
//! its region can be reaped when it dies. It is deliberately NOT used to key
//! regions: it resolves only while the peer is alive, and these clients close the
//! socket the moment they have written, so it is not reliable enough to name things
//! by.
//!
//! The socket already sits under a `0700` directory, so only the owning user can
//! connect. [`peer_pid`] also checks the peer's effective uid against ours, keeping
//! that property explicit at the read boundary rather than resting on the mode.

#[cfg(unix)]
use std::os::unix::io::AsRawFd as _;

/// The connected client's pid, or `None` when it cannot be read or the peer belongs
/// to another user. Always `None` on non-unix, where the caller keeps every region.
#[cfg(unix)]
pub fn peer_pid(stream: &net::async_net::UnixStream) -> Option<u32> {
    peer_pid_for_fd(stream.as_raw_fd())
}

/// The fd-level half, split out so a test can drive it with a plain std socket pair
/// rather than building an async stream.
#[cfg(unix)]
fn peer_pid_for_fd(fd: std::os::unix::io::RawFd) -> Option<u32> {
    let Some((euid, pid)) = peer_euid_and_pid(fd) else {
        log::warn!("claude_code_ide: could not read selection socket peer credentials");
        return None;
    };
    // SAFETY: `geteuid` reads this process's own id and cannot fail.
    let ours = unsafe { libc::geteuid() };
    if euid != ours {
        log::warn!("claude_code_ide: selection socket peer euid {euid} != ours {ours}, refusing");
        return None;
    }
    Some(pid)
}

#[cfg(not(unix))]
pub fn peer_pid(_stream: &net::async_net::UnixStream) -> Option<u32> {
    None
}

/// The peer's (euid, pid) as one read, so the two platform paths appear once each.
///
/// macOS needs two calls (`getpeereid` for the uid, `LOCAL_PEERPID` for the pid).
/// Linux gets both from one `SO_PEERCRED`, and the libc crate does not declare
/// `getpeereid` for linux-gnu. Other unix targets have no mechanism wired, so they
/// report nothing and every region is kept.
#[cfg(target_os = "macos")]
fn peer_euid_and_pid(fd: std::os::unix::io::RawFd) -> Option<(u32, u32)> {
    let mut euid: libc::uid_t = 0;
    let mut egid: libc::gid_t = 0;
    let mut pid: libc::pid_t = 0;
    let mut len = std::mem::size_of::<libc::pid_t>() as libc::socklen_t;
    // SAFETY: `fd` is a live connected Unix-socket fd owned by the caller's stream
    // for the duration of these calls, and every out-param is valid and correctly
    // sized. Neither call transfers ownership of the fd.
    let (uid_ret, pid_ret) = unsafe {
        (
            libc::getpeereid(fd, &mut euid, &mut egid),
            libc::getsockopt(
                fd,
                libc::SOL_LOCAL,
                libc::LOCAL_PEERPID,
                &mut pid as *mut libc::pid_t as *mut libc::c_void,
                &mut len,
            ),
        )
    };
    (uid_ret == 0 && pid_ret == 0 && pid > 0).then_some((euid, pid as u32))
}

#[cfg(target_os = "linux")]
fn peer_euid_and_pid(fd: std::os::unix::io::RawFd) -> Option<(u32, u32)> {
    let mut cred = libc::ucred {
        pid: 0,
        uid: 0,
        gid: 0,
    };
    let mut len = std::mem::size_of::<libc::ucred>() as libc::socklen_t;
    // SAFETY: as above; `cred`/`len` are valid out-params sized for a `ucred`.
    let ret = unsafe {
        libc::getsockopt(
            fd,
            libc::SOL_SOCKET,
            libc::SO_PEERCRED,
            &mut cred as *mut libc::ucred as *mut libc::c_void,
            &mut len,
        )
    };
    (ret == 0 && cred.pid > 0).then_some((cred.uid, cred.pid as u32))
}

#[cfg(all(unix, not(target_os = "macos"), not(target_os = "linux")))]
fn peer_euid_and_pid(_fd: std::os::unix::io::RawFd) -> Option<(u32, u32)> {
    None
}

/// Other unix targets (BSDs) that are neither macOS nor Linux.
#[cfg(all(unix, not(target_os = "macos"), not(target_os = "linux")))]
fn peer_pid_raw(_fd: std::os::unix::io::RawFd) -> Option<u32> {
    None
}

#[cfg(all(test, any(target_os = "macos", target_os = "linux")))]
mod tests {
    use super::*;

    /// The kernel must report the connecting process's pid for an accepted
    /// connection. The selection socket's region key depends on this, and a
    /// `None` here silently downgrades it to per-file keying, so it is worth
    /// asserting against the real syscall rather than trusting it.
    #[test]
    fn reports_the_connecting_process_pid() {
        let dir = tempfile::tempdir().expect("temp dir");
        let path = dir.path().join("peer.sock");

        let listener = std::os::unix::net::UnixListener::bind(&path).expect("bind");
        // Keep the client alive: the pid is only readable while the peer exists.
        let _client = std::os::unix::net::UnixStream::connect(&path).expect("connect");
        let (accepted, _addr) = listener.accept().expect("accept");

        let observed = peer_pid_for_fd(accepted.as_raw_fd());
        log::info!("observed peer pid: {observed:?}, ours: {}", std::process::id());
        assert_eq!(
            observed,
            Some(std::process::id()),
            "the peer of a self-connection is this process"
        );
    }
}
