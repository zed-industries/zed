//! Some definitions from the kernel headers

#![allow(non_camel_case_types)]

use cfg_if::cfg_if;

// const SNDRV_PCM_MMAP_OFFSET_DATA: c_uint = 0x00000000;
pub const SNDRV_PCM_MMAP_OFFSET_STATUS: libc::c_uint = 0x80000000;
pub const SNDRV_PCM_MMAP_OFFSET_CONTROL: libc::c_uint = 0x81000000;

pub const SNDRV_PCM_SYNC_PTR_HWSYNC: libc::c_uint = 1;
pub const SNDRV_PCM_SYNC_PTR_APPL: libc::c_uint = 2;
pub const SNDRV_PCM_SYNC_PTR_AVAIL_MIN: libc::c_uint = 4;

// #[repr(C)]
#[allow(non_camel_case_types)]
pub type snd_pcm_state_t = libc::c_int;

// #[repr(C)]
#[allow(non_camel_case_types)]
pub type snd_pcm_uframes_t = libc::c_ulong;

// I think?! Not sure how this will work with X32 ABI?!
#[allow(non_camel_case_types)]
pub type __kernel_off_t = libc::c_long;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_mmap_status {
    pub state: snd_pcm_state_t,    /* RO: state - SNDRV_PCM_STATE_XXXX */
    pub pad1: libc::c_int,         /* Needed for 64 bit alignment */
    pub hw_ptr: snd_pcm_uframes_t, /* RO: hw ptr (0...boundary-1) */
    pub tstamp: libc::timespec,    /* Timestamp */
    pub suspended_state: snd_pcm_state_t, /* RO: suspended stream state */
    pub audio_tstamp: libc::timespec, /* from sample counter or wall clock */
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct snd_pcm_mmap_control {
    pub appl_ptr: snd_pcm_uframes_t,  /* RW: appl ptr (0...boundary-1) */
    pub avail_min: snd_pcm_uframes_t, /* RW: min available frames for wakeup */
}

#[repr(C)]
#[derive(Debug)]
pub struct snd_pcm_channel_info {
    pub channel: libc::c_uint,
    pub offset: __kernel_off_t, /* mmap offset */
    pub first: libc::c_uint,    /* offset to first sample in bits */
    pub step: libc::c_uint,     /* samples distance in bits */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union snd_pcm_mmap_status_r {
    pub status: snd_pcm_mmap_status,
    pub reserved: [libc::c_uchar; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union snd_pcm_mmap_control_r {
    pub control: snd_pcm_mmap_control,
    pub reserved: [libc::c_uchar; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_sync_ptr {
    pub flags: libc::c_uint,
    pub s: snd_pcm_mmap_status_r,
    pub c: snd_pcm_mmap_control_r,
}

cfg_if! {
    if #[cfg(any(target_os = "linux", target_os = "android"))] {
        // See <https://github.com/nix-rust/nix/blob/197f55b3ccbce3273bf6ce119d1a8541b5df5d66/src/sys/ioctl/linux.rs>

        cfg_if! {
            if #[cfg(any(target_os = "android", target_env = "musl"))] {
                pub(super) type ioctl_num_type = libc::c_int;
            } else {
                pub(super) type ioctl_num_type = libc::c_ulong;
            }
        }

        // The READ dir is consistent across arches
        pub(super) const READ: ioctl_num_type = 2;

        // But WRITE is not, as well as having a different number of bits for the SIZEBITS
        cfg_if!{
            if #[cfg(any(
                target_arch = "mips",
                target_arch = "mips32r6",
                target_arch = "mips64",
                target_arch = "mips64r6",
                target_arch = "powerpc",
                target_arch = "powerpc64",
                target_arch = "sparc64"
            ))] {
                pub(super) const WRITE: ioctl_num_type = 4;
                const SIZEBITS: ioctl_num_type = 13;
            } else {
                pub(super) const WRITE: ioctl_num_type = 1;
                const SIZEBITS: ioctl_num_type = 14;
            }
        }

        const NRSHIFT: ioctl_num_type = 0;
        const NRBITS: ioctl_num_type = 8;
        const TYPEBITS: ioctl_num_type = 8;
        const TYPESHIFT: ioctl_num_type = NRSHIFT + NRBITS;
        const SIZESHIFT: ioctl_num_type = TYPESHIFT + TYPEBITS;
        const DIRSHIFT: ioctl_num_type = SIZESHIFT + SIZEBITS;

        /// Replication of the [`nix::ioc!`](https://github.com/nix-rust/nix/blob/197f55b3ccbce3273bf6ce119d1a8541b5df5d66/src/sys/ioctl/linux.rs#L78-L96)
        pub(super) const fn make_request(
            dir: ioctl_num_type,
            typ: u8,
            nr: u8,
            size: usize,
        ) -> ioctl_num_type {
            dir << DIRSHIFT
                | (typ as ioctl_num_type) << TYPESHIFT
                | (nr as ioctl_num_type) << NRSHIFT
                | (size as ioctl_num_type) << SIZESHIFT
        }
    } else if #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "solaris",
        target_os = "illumos"
    ))] {
        // See <https://github.com/nix-rust/nix/blob/197f55b3ccbce3273bf6ce119d1a8541b5df5d66/src/sys/ioctl/bsd.rs>

        cfg_if! {
            if #[cfg(not(any(target_os = "illumos", target_os = "solaris")))] {
                pub(super) type ioctl_num_type = libc::c_ulong;
            } else {
                pub(super) type ioctl_num_type = libc::c_int;
            }
        }

        #[allow(overflowing_literals)]
        pub(super) const READ: ioctl_num_type = 0x4000_0000;
        #[allow(overflowing_literals)]
        pub(super) const WRITE: ioctl_num_type = 0x8000_0000;

        const IOCPARM_MASK: ioctl_num_type = 0x1fff;

        /// Replication of [`nix::ioc!`](https://github.com/nix-rust/nix/blob/197f55b3ccbce3273bf6ce119d1a8541b5df5d66/src/sys/ioctl/bsd.rs#L31-L42)
        pub(super) const fn make_request(
            dir: ioctl_num_type,
            typ: u8,
            nr: u8,
            size: usize,
        ) -> ioctl_num_type {
            dir | ((size as ioctl_num_type) & IOCPARM_MASK) << 16
                | (typ as ioctl_num_type) << 8
                | nr as ioctl_num_type
        }
    } else {
        compile_error!("unknown target platform");
    }
}

pub(crate) unsafe fn sndrv_pcm_ioctl_channel_info(
    fd: libc::c_int,
    data: *mut snd_pcm_channel_info,
) -> Result<(), crate::Error> {
    const REQUEST: ioctl_num_type = make_request(
        READ,
        b'A',
        0x32,
        std::mem::size_of::<snd_pcm_channel_info>(),
    );

    unsafe {
        if libc::ioctl(fd, REQUEST, data) == -1 {
            Err(crate::Error::last("SNDRV_PCM_IOCTL_CHANNEL_INFO"))
        } else {
            Ok(())
        }
    }
}

pub(crate) unsafe fn sndrv_pcm_ioctl_sync_ptr(
    fd: libc::c_int,
    data: *mut snd_pcm_sync_ptr,
) -> Result<(), crate::Error> {
    const REQUEST: ioctl_num_type = make_request(
        READ | WRITE,
        b'A',
        0x23,
        std::mem::size_of::<snd_pcm_sync_ptr>(),
    );

    unsafe {
        if libc::ioctl(fd, REQUEST, data) == -1 {
            Err(crate::Error::last("SNDRV_PCM_IOCTL_SYNC_PTR"))
        } else {
            Ok(())
        }
    }
}

pub fn pagesize() -> usize {
    unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize }
}
