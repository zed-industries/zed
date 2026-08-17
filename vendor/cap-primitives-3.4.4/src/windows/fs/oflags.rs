use crate::fs::{FollowSymlinks, OpenOptions, OpenOptionsExt};
use std::fs;
use windows_sys::Win32::Storage::FileSystem::{
    FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OPEN_REPARSE_POINT, FILE_FLAG_WRITE_THROUGH,
    FILE_SHARE_DELETE,
};

/// Adjust an `OpenOptions` after all the flags are set, in preparation
/// for the to call a Windows API `open` function. Also return a bool
/// indicating that the `trunc` flag was requested but could not be set,
/// so the file should be truncated manually after opening.
pub(in super::super) fn prepare_open_options_for_open(opts: &mut OpenOptions) -> bool {
    let mut trunc = opts.truncate;
    let mut manually_trunc = false;

    let mut custom_flags = match opts.follow {
        FollowSymlinks::Yes => opts.ext.custom_flags,
        FollowSymlinks::No => {
            if trunc && !opts.create_new && !opts.append && opts.write {
                // On Windows, truncating overwrites a symlink with a
                // non-symlink.
                manually_trunc = true;
                trunc = false;
            }
            opts.ext.custom_flags | FILE_FLAG_OPEN_REPARSE_POINT
        }
    };
    let mut share_mode = opts.ext.share_mode;
    if opts.maybe_dir {
        custom_flags |= FILE_FLAG_BACKUP_SEMANTICS;

        // Only allow `FILE_SHARE_READ` and `FILE_SHARE_WRITE`; this mirrors
        // the values in `dir_options()` and is done to prevent directories
        // from being deleted or renamed underneath cap-std's sandboxed path
        // lookups on Windows.
        share_mode &= !FILE_SHARE_DELETE;
    }
    // This matches system-interface's `set_fd_flags` interpretation of these
    // flags on Windows.
    if opts.sync || opts.dsync {
        custom_flags |= FILE_FLAG_WRITE_THROUGH;
    }

    opts.truncate(trunc)
        .share_mode(share_mode)
        .custom_flags(custom_flags);

    manually_trunc
}

/// Translate the given `cap_std` into `std` options. Also return a bool
/// indicating that the `trunc` flag was requested but could not be set,
/// so the file should be truncated manually after opening.
pub(in super::super) fn open_options_to_std(opts: &OpenOptions) -> (fs::OpenOptions, bool) {
    use std::os::windows::fs::OpenOptionsExt;

    let mut opts = opts.clone();
    let manually_trunc = prepare_open_options_for_open(&mut opts);

    let mut std_opts = fs::OpenOptions::new();
    std_opts
        .read(opts.read)
        .write(opts.write)
        .append(opts.append)
        .truncate(opts.truncate)
        .create(opts.create)
        .create_new(opts.create_new)
        .share_mode(opts.ext.share_mode)
        .custom_flags(opts.ext.custom_flags)
        .attributes(opts.ext.attributes);

    // Calling `sequence_qos_flags` with a value of 0 has the side effect
    // of setting `SECURITY_SQOS_PRESENT`, so don't call it if we don't
    // have any flags.
    if opts.ext.security_qos_flags != 0 {
        std_opts.security_qos_flags(opts.ext.security_qos_flags);
    }

    if let Some(access_mode) = opts.ext.access_mode {
        std_opts.access_mode(access_mode);
    }

    (std_opts, manually_trunc)
}
