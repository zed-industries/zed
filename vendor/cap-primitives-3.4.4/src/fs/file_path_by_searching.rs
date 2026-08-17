use crate::fs::{
    is_root_dir, open_dir_unchecked, read_dir_unchecked, FollowSymlinks, MaybeOwnedFile, Metadata,
};
use std::fs;
use std::path::{Component, PathBuf};

/// Implementation of `file_path` for directories by opening `..` and searching
/// for a directory among `..`'s children to find its name.
pub(crate) fn file_path_by_searching(file: &fs::File) -> Option<PathBuf> {
    // Use the `_noassert` functions because the asserts depend on `file_path`,
    // which is what we're implementing here.
    let mut base = MaybeOwnedFile::borrowed_noassert(file);
    let mut components = Vec::new();

    // Iterate with `..` until we reach the root directory.
    'next_component: loop {
        // Open `..`.
        let mut iter =
            read_dir_unchecked(&base, Component::ParentDir.as_ref(), FollowSymlinks::No).ok()?;
        let metadata = Metadata::from_file(&*base).ok()?;

        // Search the children until we find one with matching metadata, and
        // then record its name.
        while let Some(child) = iter.next() {
            let child = child.ok()?;
            if child.is_same_file(&metadata).ok()? {
                // Found a match. Record the name and continue to the next component.
                components.push(child.file_name());
                base = MaybeOwnedFile::owned_noassert(
                    open_dir_unchecked(&base, Component::ParentDir.as_ref()).ok()?,
                );
                continue 'next_component;
            }
        }

        // We didn't find the directory among its parent's children. If we're at
        // the root directory, we're done.
        if is_root_dir(&base, &iter).ok()? {
            break;
        }

        // Otherwise, something went wrong and we can't determine the path.
        return None;
    }

    let mut path = PathBuf::new();
    path.push(Component::RootDir);
    for component in components.iter().rev() {
        path.push(component);
    }
    Some(path)
}
