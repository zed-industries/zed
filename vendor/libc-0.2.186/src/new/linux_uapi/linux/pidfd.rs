//! Header: `uapi/linux/pidfd.h`

use crate::prelude::*;
use crate::{
    Ioctl,
    _IO,
    _IOWR,
};

/* Flags for pidfd_open().  */
pub const PIDFD_NONBLOCK: c_uint = crate::O_NONBLOCK as c_uint;
pub const PIDFD_THREAD: c_uint = crate::O_EXCL as c_uint;

/* Flags for pidfd_send_signal(). */
pub const PIDFD_SIGNAL_THREAD: c_uint = 1 << 0;
pub const PIDFD_SIGNAL_THREAD_GROUP: c_uint = 1 << 1;
pub const PIDFD_SIGNAL_PROCESS_GROUP: c_uint = 1 << 2;

/* Flags for pidfd_info. */
pub const PIDFD_INFO_PID: c_uint = 1 << 0;
pub const PIDFD_INFO_CREDS: c_uint = 1 << 1;
pub const PIDFD_INFO_CGROUPID: c_uint = 1 << 2;
pub const PIDFD_INFO_EXIT: c_uint = 1 << 3;

pub const PIDFD_INFO_SIZE_VER0: c_uint = 64;

s! {
    #[non_exhaustive]
    pub struct pidfd_info {
        pub mask: crate::__u64,
        pub cgroupid: crate::__u64,
        pub pid: crate::__u32,
        pub tgid: crate::__u32,
        pub ppid: crate::__u32,
        pub ruid: crate::__u32,
        pub rgid: crate::__u32,
        pub euid: crate::__u32,
        pub egid: crate::__u32,
        pub suid: crate::__u32,
        pub sgid: crate::__u32,
        pub fsuid: crate::__u32,
        pub fsgid: crate::__u32,
        pub exit_code: crate::__s32,
    }
}

const PIDFS_IOCTL_MAGIC: c_uint = 0xFF;

pub const PIDFD_GET_CGROUP_NAMESPACE: Ioctl = _IO(PIDFS_IOCTL_MAGIC, 1);
pub const PIDFD_GET_IPC_NAMESPACE: Ioctl = _IO(PIDFS_IOCTL_MAGIC, 2);
pub const PIDFD_GET_MNT_NAMESPACE: Ioctl = _IO(PIDFS_IOCTL_MAGIC, 3);
pub const PIDFD_GET_NET_NAMESPACE: Ioctl = _IO(PIDFS_IOCTL_MAGIC, 4);
pub const PIDFD_GET_PID_NAMESPACE: Ioctl = _IO(PIDFS_IOCTL_MAGIC, 5);
pub const PIDFD_GET_PID_FOR_CHILDREN_NAMESPACE: Ioctl = _IO(PIDFS_IOCTL_MAGIC, 6);
pub const PIDFD_GET_TIME_NAMESPACE: Ioctl = _IO(PIDFS_IOCTL_MAGIC, 7);
pub const PIDFD_GET_TIME_FOR_CHILDREN_NAMESPACE: Ioctl = _IO(PIDFS_IOCTL_MAGIC, 8);
pub const PIDFD_GET_USER_NAMESPACE: Ioctl = _IO(PIDFS_IOCTL_MAGIC, 9);
pub const PIDFD_GET_UTS_NAMESPACE: Ioctl = _IO(PIDFS_IOCTL_MAGIC, 10);
pub const PIDFD_GET_INFO: Ioctl = _IOWR::<pidfd_info>(PIDFS_IOCTL_MAGIC, 11);
