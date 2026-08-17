#![allow(
    clippy::match_ref_pats, // looks more optimized with long array
    clippy::needless_borrowed_reference,
)]

use std::os::unix::io::RawFd;
use std::{io, fmt};
use std::num::NonZeroU32;
use std::io::ErrorKind::*;
#[cfg(any(
    target_os="linux", target_os="android",
    target_os="freebsd", target_os="dragonfly", target_vendor="apple",
    target_os="openbsd", target_os="netbsd"
))]
use std::mem;
#[cfg(any(target_os="illumos", target_os="solaris"))]
use std::ptr;

#[cfg(any(
    target_os="linux", target_os="android",
    target_os="freebsd", target_os="dragonfly", target_vendor="apple",
    target_os="openbsd", target_os="netbsd"
))]
use libc::{getsockopt, c_void, socklen_t};
#[cfg(any(target_os="linux", target_os="android"))]
use libc::{pid_t, uid_t, gid_t, getpid, getuid, geteuid, getgid, getegid};
#[cfg(any(target_os="linux", target_os="android"))]
use libc::{ucred, SOL_SOCKET, SO_PEERCRED, SO_PEERSEC};
#[cfg(any(target_os="freebsd", target_os="dragonfly", target_vendor="apple"))]
use libc::{xucred, XUCRED_VERSION, LOCAL_PEERCRED};
#[cfg(target_vendor="apple")]
use libc::SOL_LOCAL; // Apple is for once the one that does the right thing!
#[cfg(target_os="openbsd")]
use libc::{sockpeercred, SOL_SOCKET, SO_PEERCRED};
#[cfg(target_os="netbsd")]
use libc::{unpcbid, LOCAL_PEEREID};
#[cfg(any(target_os="illumos", target_os="solaris"))]
use libc::{getpeerucred, ucred_free, ucred_t};
#[cfg(any(target_os="illumos", target_os="solaris"))]
use libc::{ucred_geteuid, ucred_getegid, ucred_getpid, ucred_getgroups, uid_t, gid_t, pid_t};

/// Credentials to be sent with `send_ancillary()`.
///
/// Only on Linux (& Android) does one need to send credentials, and on other
/// operating systems this struct is ignored.
#[derive(Clone,Copy, PartialEq,Eq, Debug)]
#[allow(unused)] // not used yet
pub enum SendCredentials {
    Effective,
    Real,
    Custom{ pid: u32, uid: u32, gid: u32 }
}
#[cfg(any(target_os="linux", target_os="android"))]
impl SendCredentials {
    pub fn into_raw(self) -> ucred {
        let mut ucred: ucred = unsafe { mem::zeroed() };
        let (pid, uid, gid) = match self {
            SendCredentials::Effective => unsafe { (getpid(), geteuid(), getegid()) },
            SendCredentials::Real => unsafe { (getpid(), getuid(), getgid()) },
            SendCredentials::Custom{pid, uid, gid} => (pid as pid_t, uid as uid_t, gid as gid_t),
        };
        ucred.pid = pid;
        ucred.uid = uid;
        ucred.gid = gid;
        return ucred;
    }
}



#[cfg(any(target_os="linux", target_os="android"))]
pub fn selinux_context(fd: RawFd,  buffer: &mut[u8]) -> Result<usize, io::Error> {
    unsafe {
        let ptr = buffer.as_mut_ptr() as *mut c_void;
        let mut capacity = buffer.len().min(socklen_t::max_value() as usize) as socklen_t;
        match getsockopt(fd, SOL_SOCKET, SO_PEERSEC, ptr, &mut capacity) {
            -1 => Err(io::Error::last_os_error()),
            _ => Ok(capacity as usize),
        }
    }
}

#[cfg(not(any(target_os="linux", target_os="android")))]
pub fn selinux_context(_fd: RawFd,  _buffer: &mut[u8]) -> Result<usize, io::Error> {
    Err(io::Error::new(Other, "not available"))
}



/// Credentials of the peer process when it called `connect()`, `accept()` or `pair()`.
///
/// User and group IDs can be misleading if the peer side of the socket
/// has been transfered to another process or the peer has changed privileges.  
/// PID is almost impossible to use correctly, as the peer might have
/// terminated and the pid reused, or as for euid, the socket has been sent
/// to another process (via fd-passing or forking).
///
/// What information is received varies from OS to OS:
///
/// * Linux, OpenBSD and NetBSD provides process ID, effective user ID
///   and effective group id.
/// * macOS, FreeBSD and DragonFly BSD provides effective user ID
///   and group memberships. (The first group is also the effective group ID.)
///   [FreeBSD 13+ will also provide process ID](https://www.freebsd.org/cgi/man.cgi?query=unix&sektion=0&manpath=FreeBSD+13-current&format=html).
/// * Illumos and Solaris provide [more than one could possibly want](https://illumos.org/man/3C/ucred)
///   (the `LinuxLike` variant is most likely returned).
///
/// Current limitations of this crate:
///
/// * Illumos and Solaris provides enough information to fill out
///   both variants, but obviously only one can be returned.
/// * FreeBSD 13 will also provide pid, but this crate doesn't detect that.
#[derive(Clone,Copy, PartialEq,Eq)]
pub enum ConnCredentials {
    LinuxLike{ pid: NonZeroU32, euid: u32, egid: u32 },
    MacOsLike{ euid: u32, number_of_groups: u8, groups: [u32; 16/*what libc uses for all OSes*/] },
}
impl ConnCredentials {
    /// Get the process ID of the initial peer of a connection.
    ///
    /// This is currently only available on Linux & Android,
    /// but will in the future also be available on OpenBSD and NetBSD,
    /// and possibly also FreeBSD and Solaris.
    pub fn pid(&self) -> Option<NonZeroU32> {
        match self {
            &ConnCredentials::LinuxLike{ pid, .. } => Some(pid),
            &ConnCredentials::MacOsLike{ .. } => None,
        }
    }
    /// Get the effective user ID of the initial peer of a connection.
    ///
    /// This is provided by any supported OS.
    pub fn euid(&self) -> u32 {
        match self {
            &ConnCredentials::LinuxLike{ euid, .. } => euid,
            &ConnCredentials::MacOsLike{ euid, .. } => euid,
        }
    }
    /// Get the effective group ID of the initial peer of a connection.
    ///
    /// * On Linux, Android, OpenBSD and NetBSD,
    ///   `egid` from the `LinuxLike` variant is returned.
    /// * On FreeBSD, DragonFly BSD, macOS & other Apple platforms,
    ///   `groups[0]` from the `MacOsLike` variant is returned
    ///   (except in the unlikely case that `number_of_groups` is zero).
    // Sources for that the first group is egid: `<sys/ucred.h>` for
    // [macOS](https://github.com/apple/darwin-xnu/blob/cc0ca6d1af34cf5daee3673d1b0d770538f19ca5/bsd/sys/ucred.h#L140),
    // [FreeBSD](https://svnweb.freebsd.org/base/stable/11/sys/sys/ucred.h?revision=331722&view=markup#l93),
    // [DragonFly BSD](http://gitweb.dragonflybsd.org/dragonfly.git/blob/91dc43dd1215cf13344c65a8f9478bfd31b95814:/sys/sys/ucred.h#l77).
    // Used by the implementation of `getpeereid()` for
    // [FreeBSD](https://svnweb.freebsd.org/base/head/lib/libc/gen/getpeereid.c?view=markup),
    // [DragonFly BSD](http://gitweb.dragonflybsd.org/dragonfly.git/blob/HEAD:/lib/libc/gen/getpeereid.c#l77),
    // [macOS](https://opensource.apple.com/source/Libc/Libc-1082.50.1/gen/FreeBSD/getpeereid.c.auto.html)
    // TODO remove None case before 0.2
    pub fn egid(&self) -> Option<u32> {
        match self {
            &ConnCredentials::LinuxLike{ egid, .. } => Some(egid),
            &ConnCredentials::MacOsLike{ number_of_groups: 1..=255, groups, .. } => Some(groups[0]),
            &ConnCredentials::MacOsLike{ number_of_groups: 0, .. } => None,
        }
    }
    /// Get the groups that the initial peer of a connection was a mamber of.
    ///
    /// This is only available on FreeBSD and macOS (in the future also
    /// DragonFly BSD), and an empty slice is returned on other OSes.
    pub fn groups(&self) -> &[u32] {
        match self {
            &ConnCredentials::LinuxLike{ .. } => &[],
            &ConnCredentials::MacOsLike{ number_of_groups: n @ 0..=15, ref groups, .. } => {
                &groups[..(n as usize)]
            },
            &ConnCredentials::MacOsLike{ number_of_groups: 16..=255, ref groups, .. } => groups,
        }
    }
}
impl fmt::Debug for ConnCredentials {
    fn fmt(&self,  fmtr: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut repr = fmtr.debug_struct("ConnCredentials");
        match self {
            &ConnCredentials::LinuxLike{ ref pid, ref euid, ref egid } => {
                repr.field("pid", pid);
                repr.field("euid", euid);
                repr.field("egid", egid);
            }
            &ConnCredentials::MacOsLike{ ref euid, number_of_groups, ref groups } => {
                repr.field("euid", euid);
                let number_of_groups = (number_of_groups as usize).min(groups.len());
                repr.field("groups", &&groups[..number_of_groups]);
            }
        }
        repr.finish()
    }
}


#[cfg(any(target_os="linux", target_os="android"))]
pub fn peer_credentials(conn: RawFd) -> Result<ConnCredentials, io::Error> {
    let mut ucred: ucred = unsafe { mem::zeroed() };
    unsafe {
        let ptr = &mut ucred as *mut ucred as *mut c_void;
        let mut size = mem::size_of::<ucred>() as socklen_t;
        if getsockopt(conn, SOL_SOCKET, SO_PEERCRED, ptr, &mut size) == -1 {
            Err(io::Error::last_os_error())
        } else if let Some(pid) = NonZeroU32::new(ucred.pid as u32) {
            Ok(ConnCredentials::LinuxLike{ pid, euid: ucred.uid as u32, egid: ucred.gid as u32 })
        } else {
            Err(io::Error::new(NotConnected, "socket is not a connection"))
        }
    }
}

#[cfg(any(target_os="freebsd", target_os="dragonfly", target_vendor="apple"))]
pub fn peer_credentials(conn: RawFd) -> Result<ConnCredentials, io::Error> {
    let mut xucred: xucred = unsafe { mem::zeroed() };
    xucred.cr_version = XUCRED_VERSION;
    xucred.cr_ngroups = xucred.cr_groups.len() as _;
    // initialize to values that don't signify root, to reduce severity of bugs
    xucred.cr_uid = !0;
    for group_slot in &mut xucred.cr_groups {
        *group_slot = !0;
    }
    #[cfg(any(target_os="freebsd", target_os="dragonfly"))]
    const PEERCRED_SOCKET_LEVEL: i32 = 0; // yes literal zero: not SOL_SOCKET, and SOL_LOCAL is not a thing
    #[cfg(target_vendor="apple")]
    use SOL_LOCAL as PEERCRED_SOCKET_LEVEL;
    unsafe {
        let ptr = &mut xucred as *mut xucred as *mut c_void;
        let mut size = mem::size_of::<xucred>() as socklen_t;
        match getsockopt(conn, PEERCRED_SOCKET_LEVEL, LOCAL_PEERCRED, ptr, &mut size) {
            -1 => Err(io::Error::last_os_error()),
            _ if xucred.cr_version != XUCRED_VERSION => {
                Err(io::Error::new(InvalidData, "unknown version of peer credentials"))
            },
            _ => {
                let mut groups = [u32::max_value(); 16]; // set all unused group slots to ~0
                let filled_groups = xucred.cr_groups.iter().take(xucred.cr_ngroups as usize);
                for (&src, dst) in filled_groups.zip(&mut groups) {
                    *dst = src.into();
                }
                Ok(ConnCredentials::MacOsLike {
                    euid: xucred.cr_uid.into(),
                    number_of_groups: xucred.cr_ngroups as u8,
                    groups: groups,
                })
            }
        }
    }
}

#[cfg(target_os="openbsd")]
pub fn peer_credentials(conn: RawFd) -> Result<ConnCredentials, io::Error> {
    let mut sockpeercred: sockpeercred = unsafe { mem::zeroed() };
    unsafe {
        let ptr = &mut sockpeercred as *mut sockpeercred as *mut c_void;
        let mut size = mem::size_of::<sockpeercred>() as socklen_t;
        if getsockopt(conn, SOL_SOCKET, SO_PEERCRED, ptr, &mut size) == -1 {
            Err(io::Error::last_os_error())
        } else if let Some(pid) = NonZeroU32::new(sockpeercred.pid as u32) {
            Ok(ConnCredentials::LinuxLike {
                pid,
                euid: sockpeercred.uid as u32,
                egid: sockpeercred.gid as u32,
            })
        } else {
            Err(io::Error::new(InvalidData, "the returned pid is zero"))
        }
    }
}

#[cfg(target_os="netbsd")]
pub fn peer_credentials(conn: RawFd) -> Result<ConnCredentials, io::Error> {
    let mut unpcbid: unpcbid = unsafe { mem::zeroed() };
    unsafe {
        let ptr = &mut unpcbid as *mut unpcbid as *mut c_void;
        let mut size = mem::size_of::<unpcbid>() as socklen_t;
        // `man unix` describes it as a socket-level option, but 0 is what works
        if getsockopt(conn, 0, LOCAL_PEEREID, ptr, &mut size) == -1 {
            Err(io::Error::last_os_error())
        } else if let Some(pid) = NonZeroU32::new(unpcbid.unp_pid as u32) {
            Ok(ConnCredentials::LinuxLike {
                pid,
                euid: unpcbid.unp_euid as u32,
                egid: unpcbid.unp_egid as u32,
            })
        } else {
            Err(io::Error::new(InvalidData, "the returned pid is zero"))
        }
    }
}

#[cfg(any(target_os="illumos", target_os="solaris"))]
pub fn peer_credentials(conn: RawFd) -> Result<ConnCredentials, io::Error> {
    struct UcredAlloc(*mut ucred_t);
    impl Drop for UcredAlloc {
        fn drop(&mut self) {
            unsafe {
                if self.0 != ptr::null_mut() {
                    ucred_free(self.0);
                }
            }
        }
    }
    unsafe {
        let mut ucred = UcredAlloc(ptr::null_mut());
        if getpeerucred(conn, &mut ucred.0) == -1 {
            Err(io::Error::last_os_error())
        } else if ucred.0 == ptr::null_mut() {
            Err(io::Error::new(NotConnected, "socket is not a connection"))
        } else {
            let euid = ucred_geteuid(ucred.0 as *const _);
            let egid = ucred_getegid(ucred.0 as *const _);
            let pid = ucred_getpid(ucred.0 as *const _);
            let mut groups_ptr: *const gid_t = ptr::null_mut();
            let ngroups = ucred_getgroups(ucred.0 as *const _, &mut groups_ptr);
            // https://illumos.org/man/3C/ucred says -1 is returned on error,
            // but the types in libc are u32
            if euid != -1i32 as uid_t  &&  egid != -1i32 as gid_t
            &&  pid != -1i32 as pid_t  &&  pid != 0 {
                Ok(ConnCredentials::LinuxLike {
                    pid: NonZeroU32::new(pid as u32).unwrap(), // already checked
                    euid: euid as u32,
                    egid: egid as u32,
                })
            } else if euid != -1i32 as uid_t  &&  ngroups > 0  &&  groups_ptr != ptr::null() {
                let mut groups = [u32::max_value(); 16];
                let number_of_groups = ngroups.min(16) as u8;
                for i in 0..number_of_groups {
                    groups[i as usize] = *groups_ptr.offset(i as isize);
                }
                Ok(ConnCredentials::MacOsLike {
                    euid: euid as u32,
                    number_of_groups,
                    groups,
                })
            } else if euid != -1i32 as uid_t  &&  egid != -1i32 as gid_t {
                let mut groups = [u32::max_value(); 16];
                groups[0] = egid as u32;
                Ok(ConnCredentials::MacOsLike {
                    euid: euid as u32,
                    number_of_groups: 1,
                    groups,
                })
            } else {
                Err(io::Error::new(Other, "Not enough information was available"))
            }
        }
    }
}

#[cfg(not(any(
    target_os="linux", target_os="android",
    target_os="freebsd", target_os="dragonfly", target_vendor="apple",
    target_os="openbsd", target_os="netbsd",
    target_os="illumos", target_os="solaris",
)))]
pub fn peer_credentials(_: RawFd) -> Result<ConnCredentials, io::Error> {
    Err(io::Error::new(Other, "Not available"))
}



#[cfg(any(target_os="linux", target_os="android"))]
pub type RawReceivedCredentials = libc::ucred;


/// Process credentials received through `recv_ancillary()`.
///
/// What information is returned varies from OS to OS:
///
/// * On Linux (& Android) the information has to be explicitly sent by the
///   peer through `send_ancillary()` or `sendmsg()`, but is validated by the
///   kernel.  
///   Peer chooses whether to send effective or real uid or gid, unless root
///   in which case it can send whatever it wants.
/// * On FreeBSD, NetBSD, DragonFly BSD, Illumos and likely macOS it is provided
///   by the OS automatically when the socket option is set.
/// * OpenBSD doesn't appear to support receiving credentials.
#[derive(Clone,Copy, PartialEq,Eq,Hash, Debug)]
pub struct ReceivedCredentials {
    #[cfg(any(target_os="linux", target_os="android", target_os="dragonfly"))]
    pid: u32,
    #[cfg(any(target_os="linux", target_os="android"))]
    uid: u32,
    #[cfg(any(target_os="linux", target_os="android"))]
    gid: u32,

    #[cfg(any(
        target_os="freebsd", target_os="netbsd", target_os="dragonfly",
        target_os="illumos", target_os="solaris", target_os="macos",
    ))]
    real_uid: u32,
    #[cfg(any(
        target_os="freebsd", target_os="netbsd", target_os="dragonfly",
        target_os="illumos", target_os="solaris", target_os="macos",
    ))]
    effective_uid: u32,
    #[cfg(any(
        target_os="freebsd", target_os="netbsd", target_os="dragonfly",
        target_os="illumos", target_os="solaris", target_os="macos",
    ))]
    real_gid: u32,
    #[cfg(any(
        target_os="freebsd", target_os="netbsd",
        target_os="illumos", target_os="solaris", target_os="macos",
    ))]
    effective_gid: u32,
    #[cfg(any(
        target_os="freebsd", target_os="netbsd", target_os="dragonfly",
        target_os="illumos", target_os="solaris", target_os="macos",
    ))]
    groups: [u32; 5],
}

#[allow(unused)] // TODO
impl ReceivedCredentials {
    #[cfg(any(target_os="linux", target_os="android"))]
    pub(crate) fn from_raw(creds: libc::ucred) -> Self {
        ReceivedCredentials {
            pid: creds.pid as u32,
            uid: creds.uid as u32,
            gid: creds.gid as u32,
        }
    }

    /// The pid of the peer.
    ///
    /// This information is only available on Linux, Android and DragonFly BSD.
    pub fn pid(&self) -> Option<u32> {
        #[cfg(any(target_os="linux", target_os="android", target_os="dragonfly"))] {
            Some(self.pid)
        }
        #[cfg(not(any(target_os="linux", target_os="android", target_os="dragonfly")))] {
            None
        }
    }
    pub fn effective_or_sent_uid(&self) -> u32 {
        #[cfg(any(target_os="linux", target_os="android"))] {
            self.uid
        }
        #[cfg(any(
            target_os="freebsd", target_os="netbsd", target_os="dragonfly",
            target_os="illumos", target_os="solaris", target_os="macos",
        ))] {
            self.effective_uid
        }
        #[cfg(not(any(
            target_os="linux", target_os="android",
            target_os="freebsd", target_os="netbsd", target_os="dragonfly",
            target_os="illumos", target_os="solaris", target_os="macos",
        )))] {
            unreachable!("struct cannot be created on unsupported OSes")
        }
    }
    pub fn real_or_sent_uid(&self) -> u32 {
        #[cfg(any(target_os="linux", target_os="android"))] {
            self.uid
        }
        #[cfg(any(
            target_os="freebsd", target_os="netbsd", target_os="dragonfly",
            target_os="illumos", target_os="solaris", target_os="macos",
        ))] {
            self.real_uid
        }
        #[cfg(not(any(
            target_os="linux", target_os="android",
            target_os="freebsd", target_os="netbsd", target_os="dragonfly",
            target_os="illumos", target_os="solaris", target_os="macos",
        )))] {
            unreachable!("struct cannot be created on unsupported OSes")
        }
    }
    pub fn effective_or_sent_gid(&self) -> Option<u32> {
        #[cfg(any(target_os="linux", target_os="android"))] {
            Some(self.gid)
        }
        #[cfg(any(
            target_os="freebsd", target_os="netbsd",
            target_os="illumos", target_os="solaris", target_os="macos",
        ))] {
            Some(self.effective_gid)
        }
        #[cfg(not(any(
            target_os="linux", target_os="android",
            target_os="freebsd", target_os="netbsd",
            target_os="illumos", target_os="solaris", target_os="macos",
        )))] {
            None
        }
    }
    pub fn real_or_sent_gid(&self) -> u32 {
        #[cfg(any(target_os="linux", target_os="android"))] {
            self.gid
        }
        #[cfg(any(
            target_os="freebsd", target_os="netbsd", target_os="dragonfly",
            target_os="illumos", target_os="solaris", target_os="macos",
        ))] {
            self.real_gid
        }
        #[cfg(not(any(
            target_os="linux", target_os="android",
            target_os="freebsd", target_os="netbsd", target_os="dragonfly",
            target_os="illumos", target_os="solaris", target_os="macos",
        )))] {
            unreachable!("struct cannot be created on unsupported OSes")
        }
    }
    /// Get the peer's group memberships.
    ///
    /// This information is only available on macOS, the BSDs and and Illumos.
    /// On other operating systems an empty slice is returned.
    pub fn groups(&self) -> &[u32] {
        #[cfg(any(
            target_os="freebsd", target_os="netbsd", target_os="dragonfly",
            target_os="illumos", target_os="solaris", target_os="macos",
        ))] {
            &self.groups[..]
        }
        #[cfg(not(any(
            target_os="freebsd", target_os="netbsd", target_os="dragonfly",
            target_os="illumos", target_os="solaris", target_os="macos",
        )))] {
            &[]
        }
    }
}
