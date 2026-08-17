// Implementation derived from `copy` in Rust's
// library/std/src/sys/unix/fs.rs at revision
// 108e90ca78f052c0c1c49c42a22c85620be19712.

use crate::fs::{open, OpenOptions};
#[cfg(any(target_os = "android", target_os = "linux"))]
use rustix::fs::copy_file_range;
#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "watchos",
    target_os = "visionos",
))]
use rustix::fs::{
    copyfile_state_alloc, copyfile_state_free, copyfile_state_get_copied, copyfile_state_t,
    fclonefileat, fcopyfile, CloneFlags, CopyfileFlags,
};
use std::path::Path;
use std::{fs, io};

fn open_from(start: &fs::File, path: &Path) -> io::Result<(fs::File, fs::Metadata)> {
    let reader = open(start, path, OpenOptions::new().read(true))?;
    let metadata = reader.metadata()?;
    if !metadata.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "the source path is not an existing regular file",
        ));
    }
    Ok((reader, metadata))
}

#[cfg(not(target_os = "wasi"))]
fn open_to_and_set_permissions(
    start: &fs::File,
    path: &Path,
    reader_metadata: fs::Metadata,
) -> io::Result<(fs::File, fs::Metadata)> {
    use crate::fs::OpenOptionsExt;
    use std::os::unix::fs::PermissionsExt;

    let perm = reader_metadata.permissions();
    let writer = open(
        start,
        path,
        OpenOptions::new()
            // create the file with the correct mode right away
            .mode(perm.mode())
            .write(true)
            .create(true)
            .truncate(true),
    )?;
    let writer_metadata = writer.metadata()?;
    if writer_metadata.is_file() {
        // Set the correct file permissions, in case the file already existed.
        // Don't set the permissions on already existing non-files like
        // pipes/FIFOs or device nodes.
        writer.set_permissions(perm)?;
    }
    Ok((writer, writer_metadata))
}

#[cfg(target_os = "wasi")]
fn open_to_and_set_permissions(
    start: &fs::File,
    path: &Path,
    reader_metadata: fs::Metadata,
) -> io::Result<(fs::File, fs::Metadata)> {
    let writer = open(
        start,
        path,
        OpenOptions::new()
            // create the file with the correct mode right away
            .write(true)
            .create(true)
            .truncate(true),
    )?;
    let writer_metadata = writer.metadata()?;
    Ok((writer, writer_metadata))
}

#[cfg(not(any(
    target_os = "linux",
    target_os = "android",
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "watchos",
    target_os = "visionos",
)))]
pub(crate) fn copy_impl(
    from_start: &fs::File,
    from_path: &Path,
    to_start: &fs::File,
    to_path: &Path,
) -> io::Result<u64> {
    let (mut reader, reader_metadata) = open_from(from_start, from_path)?;
    let (mut writer, _) = open_to_and_set_permissions(to_start, to_path, reader_metadata)?;

    io::copy(&mut reader, &mut writer)
}

#[cfg(any(target_os = "android", target_os = "linux"))]
pub(crate) fn copy_impl(
    from_start: &fs::File,
    from_path: &Path,
    to_start: &fs::File,
    to_path: &Path,
) -> io::Result<u64> {
    use std::cmp;
    use std::sync::atomic::{AtomicBool, Ordering};

    // Kernel prior to 4.5 don't have copy_file_range
    // We store the availability in a global to avoid unnecessary syscalls
    static HAS_COPY_FILE_RANGE: AtomicBool = AtomicBool::new(true);

    let (mut reader, reader_metadata) = open_from(from_start, from_path)?;
    let len = reader_metadata.len();
    let (mut writer, _) = open_to_and_set_permissions(to_start, to_path, reader_metadata)?;

    let has_copy_file_range = HAS_COPY_FILE_RANGE.load(Ordering::Relaxed);
    let mut written = 0_u64;
    while written < len {
        let copy_result = if has_copy_file_range {
            let bytes_to_copy = cmp::min(len - written, usize::MAX as u64);

            // `copy_file_range` takes a `usize`; convert with saturation so
            // that we copy as many bytes as we can.
            let bytes_to_copy = usize::try_from(bytes_to_copy).unwrap_or(usize::MAX);

            // We actually don't have to adjust the offsets,
            // because copy_file_range adjusts the file offset automatically
            let copy_result = copy_file_range(&reader, None, &writer, None, bytes_to_copy);
            if let Err(copy_err) = copy_result {
                match copy_err {
                    rustix::io::Errno::NOSYS | rustix::io::Errno::PERM => {
                        HAS_COPY_FILE_RANGE.store(false, Ordering::Relaxed);
                    }
                    _ => {}
                }
            }
            copy_result
        } else {
            Err(rustix::io::Errno::NOSYS)
        };
        match copy_result {
            Ok(ret) => written += ret as u64,
            Err(err) => {
                match err {
                    rustix::io::Errno::NOSYS
                    | rustix::io::Errno::XDEV
                    | rustix::io::Errno::INVAL
                    | rustix::io::Errno::PERM => {
                        // Try fallback io::copy if either:
                        // - Kernel version is < 4.5 (ENOSYS)
                        // - Files are mounted on different fs (EXDEV)
                        // - copy_file_range is disallowed, for example by seccomp (EPERM)
                        // - copy_file_range cannot be used with pipes or device nodes (EINVAL)
                        assert_eq!(written, 0);
                        return io::copy(&mut reader, &mut writer);
                    }
                    _ => return Err(err.into()),
                }
            }
        }
    }
    Ok(written)
}

#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "watchos",
    target_os = "visionos",
))]
#[allow(non_upper_case_globals)]
#[allow(unsafe_code)]
pub(crate) fn copy_impl(
    from_start: &fs::File,
    from_path: &Path,
    to_start: &fs::File,
    to_path: &Path,
) -> io::Result<u64> {
    use std::sync::atomic::{AtomicBool, Ordering};

    struct FreeOnDrop(copyfile_state_t);
    impl Drop for FreeOnDrop {
        fn drop(&mut self) {
            // Safety: This is the only place where we free the state, and we
            // never let it escape.
            unsafe {
                copyfile_state_free(self.0).ok();
            }
        }
    }

    // MacOS prior to 10.12 don't support `fclonefileat`
    // We store the availability in a global to avoid unnecessary syscalls
    static HAS_FCLONEFILEAT: AtomicBool = AtomicBool::new(true);

    let (reader, reader_metadata) = open_from(from_start, from_path)?;

    // Opportunistically attempt to create a copy-on-write clone of `from_path`
    // using `fclonefileat`.
    if HAS_FCLONEFILEAT.load(Ordering::Relaxed) {
        let clonefile_result = fclonefileat(&reader, to_start, to_path, CloneFlags::empty());
        match clonefile_result {
            Ok(_) => return Ok(reader_metadata.len()),
            Err(err) => match err {
                // `fclonefileat` will fail on non-APFS volumes, if the
                // destination already exists, or if the source and destination
                // are on different devices. In all these cases `fcopyfile`
                // should succeed.
                rustix::io::Errno::NOTSUP | rustix::io::Errno::EXIST | rustix::io::Errno::XDEV => {
                    ()
                }
                rustix::io::Errno::NOSYS => HAS_FCLONEFILEAT.store(false, Ordering::Relaxed),
                _ => return Err(err.into()),
            },
        }
    }

    // Fall back to using `fcopyfile` if `fclonefileat` does not succeed.
    let (writer, writer_metadata) =
        open_to_and_set_permissions(to_start, to_path, reader_metadata)?;

    // We ensure that `FreeOnDrop` never contains a null pointer so it is
    // always safe to call `copyfile_state_free`
    let state = {
        let state = copyfile_state_alloc()?;
        FreeOnDrop(state)
    };

    let flags = if writer_metadata.is_file() {
        CopyfileFlags::ALL
    } else {
        CopyfileFlags::DATA
    };

    // Safety: We allocated `state` above so it's still live here.
    unsafe {
        fcopyfile(&reader, &writer, state.0, flags)?;

        Ok(copyfile_state_get_copied(state.0)?)
    }
}
