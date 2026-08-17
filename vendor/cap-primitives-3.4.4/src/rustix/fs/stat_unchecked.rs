use crate::fs::{FollowSymlinks, ImplMetadataExt, Metadata};
use rustix::fs::{statat, AtFlags};
use std::path::Path;
use std::{fs, io};

#[cfg(target_os = "linux")]
use rustix::fs::{statx, StatxFlags};
#[cfg(target_os = "linux")]
use std::sync::atomic::{AtomicU8, Ordering};

/// *Unsandboxed* function similar to `stat`, but which does not perform
/// sandboxing.
pub(crate) fn stat_unchecked(
    start: &fs::File,
    path: &Path,
    follow: FollowSymlinks,
) -> io::Result<Metadata> {
    let atflags = match follow {
        FollowSymlinks::Yes => AtFlags::empty(),
        FollowSymlinks::No => AtFlags::SYMLINK_NOFOLLOW,
    };

    // `statx` is preferred on regular Linux because it can return creation
    // times. Linux kernels prior to 4.11 don't have `statx` and return
    // `ENOSYS`. Older versions of Docker/seccomp would return `EPERM` for
    // `statx`; see <https://github.com/rust-lang/rust/pull/65685/>. We store
    // the availability in a global to avoid unnecessary syscalls.
    //
    // On Android, the [seccomp policy] prevents us from even
    // detecting whether `statx` is supported, so don't even try.
    //
    // [seccomp policy]: https://android-developers.googleblog.com/2017/07/seccomp-filter-in-android-o.html
    #[cfg(target_os = "linux")]
    {
        // 0: Unknown
        // 1: Not available
        // 2: Available
        static STATX_STATE: AtomicU8 = AtomicU8::new(0);
        let state = STATX_STATE.load(Ordering::Relaxed);

        if state != 1 {
            let statx_result = statx(
                start,
                path,
                atflags,
                StatxFlags::BASIC_STATS | StatxFlags::BTIME,
            );
            match statx_result {
                Ok(statx) => {
                    if state == 0 {
                        STATX_STATE.store(2, Ordering::Relaxed);
                    }
                    return Ok(ImplMetadataExt::from_rustix_statx(statx));
                }
                Err(rustix::io::Errno::NOSYS) => STATX_STATE.store(1, Ordering::Relaxed),
                Err(rustix::io::Errno::PERM) if state == 0 => {
                    // This is an unlikely case, as `statx` doesn't normally
                    // return `PERM` errors. One way this can happen is when
                    // running on old versions of seccomp/Docker. If `statx` on
                    // the current working directory returns a similar error,
                    // then stop using `statx`.
                    if let Err(rustix::io::Errno::PERM) = statx(
                        rustix::fs::CWD,
                        "",
                        AtFlags::EMPTY_PATH,
                        StatxFlags::empty(),
                    ) {
                        STATX_STATE.store(1, Ordering::Relaxed);
                    } else {
                        return Err(rustix::io::Errno::PERM.into());
                    }
                }
                Err(e) => return Err(e.into()),
            }
        }
    }

    Ok(statat(start, path, atflags).map(ImplMetadataExt::from_rustix)?)
}
