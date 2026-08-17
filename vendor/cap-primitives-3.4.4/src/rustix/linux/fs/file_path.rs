use super::procfs::get_path_from_proc_self_fd;
use std::fs;
use std::path::PathBuf;

pub(crate) fn file_path(file: &fs::File) -> Option<PathBuf> {
    use std::os::unix::fs::MetadataExt;

    // Ignore paths that don't start with '/', which are things like
    // `socket:[3556564]` or similar.
    let path = get_path_from_proc_self_fd(file)
        .ok()
        .filter(|path| path.starts_with("/"))?;

    // Linux appends the string " (deleted)" when a file is deleted; avoid
    // treating that as the actual name. Check this after doing the `readlink`
    // above so that we're conservative about concurrent deletions.
    if file.metadata().ok()?.nlink() == 0 {
        return None;
    }

    Some(path)
}
