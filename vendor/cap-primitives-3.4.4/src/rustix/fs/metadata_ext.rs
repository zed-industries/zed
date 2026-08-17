#![allow(clippy::useless_conversion)]

use crate::fs::{ImplFileTypeExt, ImplPermissionsExt, Metadata};
use crate::time::{Duration, SystemClock, SystemTime};
#[cfg(target_os = "linux")]
use rustix::fs::{makedev, Statx, StatxFlags};
use rustix::fs::{RawMode, Stat};
use std::{fs, io};

#[derive(Debug, Clone)]
pub(crate) struct ImplMetadataExt {
    dev: u64,
    ino: u64,
    #[cfg(not(target_os = "wasi"))]
    mode: u32,
    nlink: u64,
    #[cfg(not(target_os = "wasi"))]
    uid: u32,
    #[cfg(not(target_os = "wasi"))]
    gid: u32,
    #[cfg(not(target_os = "wasi"))]
    rdev: u64,
    size: u64,
    #[cfg(not(target_os = "wasi"))]
    atime: i64,
    #[cfg(not(target_os = "wasi"))]
    atime_nsec: i64,
    #[cfg(not(target_os = "wasi"))]
    mtime: i64,
    #[cfg(not(target_os = "wasi"))]
    mtime_nsec: i64,
    #[cfg(not(target_os = "wasi"))]
    ctime: i64,
    #[cfg(not(target_os = "wasi"))]
    ctime_nsec: i64,
    #[cfg(not(target_os = "wasi"))]
    blksize: u64,
    #[cfg(not(target_os = "wasi"))]
    blocks: u64,
    #[cfg(target_os = "wasi")]
    atim: u64,
    #[cfg(target_os = "wasi")]
    mtim: u64,
    #[cfg(target_os = "wasi")]
    ctim: u64,
}

impl ImplMetadataExt {
    /// Constructs a new instance of `Self` from the given [`std::fs::File`]
    /// and [`std::fs::Metadata`].
    #[inline]
    #[allow(clippy::unnecessary_wraps)]
    pub(crate) fn from(_file: &fs::File, std: &fs::Metadata) -> io::Result<Self> {
        // On `rustix`-style platforms, the `Metadata` has everything we need.
        Ok(Self::from_just_metadata(std))
    }

    /// Constructs a new instance of `Self` from the given
    /// [`std::fs::Metadata`].
    #[inline]
    pub(crate) fn from_just_metadata(std: &fs::Metadata) -> Self {
        use rustix::fs::MetadataExt;
        Self {
            dev: std.dev(),
            ino: std.ino(),
            #[cfg(not(target_os = "wasi"))]
            mode: std.mode(),
            nlink: std.nlink(),
            #[cfg(not(target_os = "wasi"))]
            uid: std.uid(),
            #[cfg(not(target_os = "wasi"))]
            gid: std.gid(),
            #[cfg(not(target_os = "wasi"))]
            rdev: std.rdev(),
            size: std.size(),
            #[cfg(not(target_os = "wasi"))]
            atime: std.atime(),
            #[cfg(not(target_os = "wasi"))]
            atime_nsec: std.atime_nsec(),
            #[cfg(not(target_os = "wasi"))]
            mtime: std.mtime(),
            #[cfg(not(target_os = "wasi"))]
            mtime_nsec: std.mtime_nsec(),
            #[cfg(not(target_os = "wasi"))]
            ctime: std.ctime(),
            #[cfg(not(target_os = "wasi"))]
            ctime_nsec: std.ctime_nsec(),
            #[cfg(not(target_os = "wasi"))]
            blksize: std.blksize(),
            #[cfg(not(target_os = "wasi"))]
            blocks: std.blocks(),
            #[cfg(target_os = "wasi")]
            atim: std.atim(),
            #[cfg(target_os = "wasi")]
            mtim: std.mtim(),
            #[cfg(target_os = "wasi")]
            ctim: std.ctim(),
        }
    }

    /// Constructs a new instance of `Metadata` from the given `Stat`.
    #[inline]
    #[allow(unused_comparisons)] // NB: rust-lang/rust#115823 requires this here instead of on `st_dev` processing below
    pub(crate) fn from_rustix(stat: Stat) -> Metadata {
        Metadata {
            file_type: ImplFileTypeExt::from_raw_mode(stat.st_mode as RawMode),
            len: u64::try_from(stat.st_size).unwrap(),
            #[cfg(not(target_os = "wasi"))]
            permissions: ImplPermissionsExt::from_raw_mode(stat.st_mode as RawMode),
            #[cfg(target_os = "wasi")]
            permissions: ImplPermissionsExt::default(),

            #[cfg(not(target_os = "wasi"))]
            modified: system_time_from_rustix(
                stat.st_mtime.try_into().unwrap(),
                stat.st_mtime_nsec as _,
            ),
            #[cfg(not(target_os = "wasi"))]
            accessed: system_time_from_rustix(
                stat.st_atime.try_into().unwrap(),
                stat.st_atime_nsec as _,
            ),

            #[cfg(target_os = "wasi")]
            modified: system_time_from_rustix(stat.st_mtim.tv_sec, stat.st_mtim.tv_nsec as _),
            #[cfg(target_os = "wasi")]
            accessed: system_time_from_rustix(stat.st_atim.tv_sec, stat.st_atim.tv_nsec as _),

            #[cfg(any(
                target_os = "freebsd",
                target_os = "openbsd",
                target_os = "netbsd",
                target_os = "macos",
                target_os = "ios",
                target_os = "tvos",
                target_os = "watchos",
                target_os = "visionos",
            ))]
            created: system_time_from_rustix(
                stat.st_birthtime.try_into().unwrap(),
                stat.st_birthtime_nsec as _,
            ),

            // `stat.st_ctime` is the latest status change; we want the creation.
            #[cfg(not(any(
                target_os = "freebsd",
                target_os = "openbsd",
                target_os = "macos",
                target_os = "ios",
                target_os = "tvos",
                target_os = "watchos",
                target_os = "visionos",
                target_os = "netbsd"
            )))]
            created: None,

            ext: Self {
                // The type of `st_dev` is `dev_t` which is signed on some
                // platforms and unsigned on other platforms. A `u64` is enough
                // to work for all unsigned platforms, and for signed platforms
                // perform a sign extension to `i64` and then view that as an
                // unsigned 64-bit number instead.
                //
                // Note that the `unused_comparisons` is ignored here for
                // platforms where it's unsigned since the first branch here
                // will never be taken.
                dev: if stat.st_dev < 0 {
                    i64::try_from(stat.st_dev).unwrap() as u64
                } else {
                    u64::try_from(stat.st_dev).unwrap()
                },
                ino: stat.st_ino.into(),
                #[cfg(not(target_os = "wasi"))]
                mode: u32::from(stat.st_mode),
                nlink: u64::from(stat.st_nlink),
                #[cfg(not(target_os = "wasi"))]
                uid: stat.st_uid,
                #[cfg(not(target_os = "wasi"))]
                gid: stat.st_gid,
                #[cfg(not(target_os = "wasi"))]
                rdev: u64::try_from(stat.st_rdev).unwrap(),
                size: u64::try_from(stat.st_size).unwrap(),
                #[cfg(not(target_os = "wasi"))]
                atime: i64::try_from(stat.st_atime).unwrap(),
                #[cfg(not(target_os = "wasi"))]
                atime_nsec: stat.st_atime_nsec as _,
                #[cfg(not(target_os = "wasi"))]
                mtime: i64::try_from(stat.st_mtime).unwrap(),
                #[cfg(not(target_os = "wasi"))]
                mtime_nsec: stat.st_mtime_nsec as _,
                #[cfg(not(target_os = "wasi"))]
                ctime: i64::try_from(stat.st_ctime).unwrap(),
                #[cfg(not(target_os = "wasi"))]
                ctime_nsec: stat.st_ctime_nsec as _,
                #[cfg(not(target_os = "wasi"))]
                blksize: u64::try_from(stat.st_blksize).unwrap(),
                #[cfg(not(target_os = "wasi"))]
                blocks: u64::try_from(stat.st_blocks).unwrap(),
                #[cfg(target_os = "wasi")]
                atim: u64::try_from(
                    stat.st_atim.tv_sec as u64 * 1000000000 + stat.st_atim.tv_nsec as u64,
                )
                .unwrap(),
                #[cfg(target_os = "wasi")]
                mtim: u64::try_from(
                    stat.st_mtim.tv_sec as u64 * 1000000000 + stat.st_mtim.tv_nsec as u64,
                )
                .unwrap(),
                #[cfg(target_os = "wasi")]
                ctim: u64::try_from(
                    stat.st_ctim.tv_sec as u64 * 1000000000 + stat.st_ctim.tv_nsec as u64,
                )
                .unwrap(),
            },
        }
    }

    /// Constructs a new instance of `Metadata` from the given `Statx`.
    #[cfg(target_os = "linux")]
    #[inline]
    pub(crate) fn from_rustix_statx(statx: Statx) -> Metadata {
        Metadata {
            file_type: ImplFileTypeExt::from_raw_mode(RawMode::from(statx.stx_mode)),
            len: u64::try_from(statx.stx_size).unwrap(),
            permissions: ImplPermissionsExt::from_raw_mode(RawMode::from(statx.stx_mode)),
            modified: if statx.stx_mask & StatxFlags::MTIME.bits() != 0 {
                system_time_from_rustix(statx.stx_mtime.tv_sec, statx.stx_mtime.tv_nsec as _)
            } else {
                None
            },
            accessed: if statx.stx_mask & StatxFlags::ATIME.bits() != 0 {
                system_time_from_rustix(statx.stx_atime.tv_sec, statx.stx_atime.tv_nsec as _)
            } else {
                None
            },
            created: if statx.stx_mask & StatxFlags::BTIME.bits() != 0 {
                system_time_from_rustix(statx.stx_btime.tv_sec, statx.stx_btime.tv_nsec as _)
            } else {
                None
            },

            ext: Self {
                dev: makedev(statx.stx_dev_major, statx.stx_dev_minor),
                ino: statx.stx_ino.into(),
                mode: u32::from(statx.stx_mode),
                nlink: u64::from(statx.stx_nlink),
                uid: statx.stx_uid,
                gid: statx.stx_gid,
                rdev: makedev(statx.stx_rdev_major, statx.stx_rdev_minor),
                size: statx.stx_size,
                atime: i64::from(statx.stx_atime.tv_sec),
                atime_nsec: statx.stx_atime.tv_nsec as _,
                mtime: i64::from(statx.stx_mtime.tv_sec),
                mtime_nsec: statx.stx_mtime.tv_nsec as _,
                ctime: i64::from(statx.stx_ctime.tv_sec),
                ctime_nsec: statx.stx_ctime.tv_nsec as _,
                blksize: u64::from(statx.stx_blksize),
                blocks: statx.stx_blocks,
            },
        }
    }

    /// Determine if `self` and `other` refer to the same inode on the same
    /// device.
    pub(crate) const fn is_same_file(&self, other: &Self) -> bool {
        self.dev == other.dev && self.ino == other.ino
    }
}

#[allow(clippy::similar_names)]
fn system_time_from_rustix(sec: i64, nsec: u64) -> Option<SystemTime> {
    if sec >= 0 {
        SystemClock::UNIX_EPOCH.checked_add(Duration::new(u64::try_from(sec).unwrap(), nsec as _))
    } else {
        SystemClock::UNIX_EPOCH
            .checked_sub(Duration::new(u64::try_from(-sec).unwrap(), 0))
            .map(|t| t.checked_add(Duration::new(0, nsec as u32)))
            .flatten()
    }
}

impl crate::fs::MetadataExt for ImplMetadataExt {
    #[inline]
    fn dev(&self) -> u64 {
        self.dev
    }

    #[inline]
    fn ino(&self) -> u64 {
        self.ino
    }

    #[cfg(not(target_os = "wasi"))]
    #[inline]
    fn mode(&self) -> u32 {
        self.mode
    }

    #[inline]
    fn nlink(&self) -> u64 {
        self.nlink
    }

    #[cfg(not(target_os = "wasi"))]
    #[inline]
    fn uid(&self) -> u32 {
        self.uid
    }

    #[cfg(not(target_os = "wasi"))]
    #[inline]
    fn gid(&self) -> u32 {
        self.gid
    }

    #[cfg(not(target_os = "wasi"))]
    #[inline]
    fn rdev(&self) -> u64 {
        self.rdev
    }

    #[inline]
    fn size(&self) -> u64 {
        self.size
    }

    #[cfg(not(target_os = "wasi"))]
    #[inline]
    fn atime(&self) -> i64 {
        self.atime
    }

    #[cfg(not(target_os = "wasi"))]
    #[inline]
    fn atime_nsec(&self) -> i64 {
        self.atime_nsec
    }

    #[cfg(not(target_os = "wasi"))]
    #[inline]
    fn mtime(&self) -> i64 {
        self.mtime
    }

    #[cfg(not(target_os = "wasi"))]
    #[inline]
    fn mtime_nsec(&self) -> i64 {
        self.mtime_nsec
    }

    #[cfg(not(target_os = "wasi"))]
    #[inline]
    fn ctime(&self) -> i64 {
        self.ctime
    }

    #[cfg(not(target_os = "wasi"))]
    #[inline]
    fn ctime_nsec(&self) -> i64 {
        self.ctime_nsec
    }

    #[cfg(not(target_os = "wasi"))]
    #[inline]
    fn blksize(&self) -> u64 {
        self.blksize
    }

    #[cfg(not(target_os = "wasi"))]
    #[inline]
    fn blocks(&self) -> u64 {
        self.blocks
    }

    #[cfg(target_os = "wasi")]
    fn atim(&self) -> u64 {
        self.atim
    }

    #[cfg(target_os = "wasi")]
    fn mtim(&self) -> u64 {
        self.mtim
    }

    #[cfg(target_os = "wasi")]
    fn ctim(&self) -> u64 {
        self.ctim
    }
}

/// It should be possible to represent times before the Epoch.
/// https://github.com/bytecodealliance/cap-std/issues/328
#[test]
fn negative_time() {
    let system_time = system_time_from_rustix(-1, 1).unwrap();
    let d = SystemClock::UNIX_EPOCH.duration_since(system_time).unwrap();
    assert_eq!(d.as_secs(), 0);
    if !cfg!(emulate_second_only_system) {
        assert_eq!(d.subsec_nanos(), 999999999);
    }
}
