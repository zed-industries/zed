//! Linux 5.6 and later have a syscall `openat2`, with flags that allow it to
//! enforce the sandboxing property we want. See the [LWN article] for an
//! overview and the [`openat2` documentation] for details.
//!
//! [LWN article]: https://lwn.net/Articles/796868/
//! [`openat2` documentation]: https://man7.org/linux/man-pages/man2/openat2.2.html
//!
//! On older Linux, fall back to `manually::open`.

#[cfg(racy_asserts)]
use crate::fs::is_same_file;
use crate::fs::{manually, OpenOptions};
use std::path::Path;
use std::{fs, io};
#[cfg(target_os = "linux")]
use {
    super::super::super::fs::compute_oflags,
    crate::fs::errors,
    io_lifetimes::FromFd,
    rustix::fs::{openat2, Mode, OFlags, RawMode, ResolveFlags},
    rustix::path::Arg,
    std::sync::atomic::AtomicBool,
    std::sync::atomic::Ordering::Relaxed,
};

/// Call the `openat2` system call, or use a fallback if that's unavailable.
pub(crate) fn open_impl(
    start: &fs::File,
    path: &Path,
    options: &OpenOptions,
) -> io::Result<fs::File> {
    // On regular Linux, attempt to use `openat2` to accelerate sandboxed
    // lookups. On Android, the [seccomp policy] prevents us from even
    // detecting whether `openat2` is supported, so don't even try.
    //
    // [seccomp policy]: https://android-developers.googleblog.com/2017/07/seccomp-filter-in-android-o.html
    #[cfg(target_os = "linux")]
    {
        let result = open_beneath(start, path, options);

        // If we got anything other than a `ENOSYS` error, that's our result.
        match result {
            Err(err) if err.raw_os_error() == Some(rustix::io::Errno::NOSYS.raw_os_error()) => {}
            Err(err) => return Err(err),
            Ok(fd) => return Ok(fd),
        }
    }

    manually::open(start, path, options)
}

/// Call the `openat2` system call with `RESOLVE_BENEATH`. If the syscall is
/// unavailable, mark it so for future calls. If `openat2` is unavailable
/// either permanently or temporarily, return `ENOSYS`.
#[cfg(target_os = "linux")]
pub(crate) fn open_beneath(
    start: &fs::File,
    path: &Path,
    options: &OpenOptions,
) -> io::Result<fs::File> {
    static INVALID: AtomicBool = AtomicBool::new(false);
    if INVALID.load(Relaxed) {
        // `openat2` is permanently unavailable.
        return Err(rustix::io::Errno::NOSYS.into());
    }

    let oflags = compute_oflags(options)?;

    // Do two `contains` checks because `TMPFILE` may be represented with
    // multiple flags and we need to ensure they're all set.
    let mode = if oflags.contains(OFlags::CREATE) || oflags.contains(OFlags::TMPFILE) {
        Mode::from_bits((options.ext.mode & 0o7777) as RawMode).unwrap()
    } else {
        Mode::empty()
    };

    // We know `openat2` needs a `&CStr` internally; to avoid allocating on
    // each iteration of the loop below, allocate the `CString` now.
    path.into_with_c_str(|path_c_str| {
        // `openat2` fails with `EAGAIN` if a rename happens anywhere on the host
        // while it's running, so use a loop to retry it a few times. But not too many
        // times, because there's no limit on how often this can happen. The actual
        // number here is currently an arbitrarily chosen guess.
        for _ in 0..4 {
            match openat2(
                start,
                path_c_str,
                oflags,
                mode,
                ResolveFlags::BENEATH | ResolveFlags::NO_MAGICLINKS,
            ) {
                Ok(file) => {
                    let file = fs::File::from_into_fd(file);

                    #[cfg(racy_asserts)]
                    check_open(start, path, options, &file);

                    return Ok(file);
                }
                Err(err) => match err {
                    // A rename or similar happened. Try again.
                    rustix::io::Errno::AGAIN => continue,

                    // `EPERM` is used by some `seccomp` sandboxes to indicate
                    // that `openat2` is unimplemented:
                    // <https://github.com/systemd/systemd/blob/e2357b1c8a87b610066b8b2a59517bcfb20b832e/src/shared/seccomp-util.c#L2066>
                    //
                    // However, `EPERM` may also indicate a failed `O_NOATIME`
                    // or a file seal prevented the operation, and it's complex
                    // to detect those cases, so exit the loop and use the
                    // fallback.
                    rustix::io::Errno::PERM => break,

                    // `ENOSYS` means `openat2` is permanently unavailable;
                    // mark it so and exit the loop.
                    rustix::io::Errno::NOSYS => {
                        INVALID.store(true, Relaxed);
                        break;
                    }

                    _ => return Err(err),
                },
            }
        }

        Err(rustix::io::Errno::NOSYS)
    })
    .map_err(|err| match err {
        rustix::io::Errno::XDEV => errors::escape_attempt(),
        err => err.into(),
    })
}

#[cfg(racy_asserts)]
fn check_open(start: &fs::File, path: &Path, options: &OpenOptions, file: &fs::File) {
    let check = manually::open(
        start,
        path,
        options
            .clone()
            .create(false)
            .create_new(false)
            .truncate(false),
    )
    .expect("manually::open failed when open_openat2 succeeded");
    assert!(
        is_same_file(file, &check).unwrap(),
        "manually::open should open the same inode as open_openat2"
    );
}
