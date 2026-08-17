use crate::errors::*;
use log::{debug, trace};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Component, Path, PathBuf};

/// Naively replaces any path separator with a forward-slash '/'
pub fn normalize_path(path: &str) -> String {
    use std::path::is_separator;
    path.chars()
        .map(|ch| if is_separator(ch) { '/' } else { ch })
        .collect::<String>()
}

/// Write the given data to a file, creating it first if necessary
pub fn write_file<P: AsRef<Path>>(build_dir: &Path, filename: P, content: &[u8]) -> Result<()> {
    let path = build_dir.join(filename);

    create_file(&path)?.write_all(content).map_err(Into::into)
}

/// Takes a path and returns a path containing just enough `../` to point to
/// the root of the given path.
///
/// This is mostly interesting for a relative path to point back to the
/// directory from where the path starts.
///
/// ```rust
/// # use std::path::Path;
/// # use mdbook::utils::fs::path_to_root;
/// let path = Path::new("some/relative/path");
/// assert_eq!(path_to_root(path), "../../");
/// ```
///
/// **note:** it's not very fool-proof, if you find a situation where
/// it doesn't return the correct path.
/// Consider [submitting a new issue](https://github.com/rust-lang/mdBook/issues)
/// or a [pull-request](https://github.com/rust-lang/mdBook/pulls) to improve it.
pub fn path_to_root<P: Into<PathBuf>>(path: P) -> String {
    // Remove filename and add "../" for every directory

    path.into()
        .parent()
        .expect("")
        .components()
        .fold(String::new(), |mut s, c| {
            match c {
                Component::Normal(_) => s.push_str("../"),
                _ => {
                    debug!("Other path component... {:?}", c);
                }
            }
            s
        })
}

/// This function creates a file and returns it. But before creating the file
/// it checks every directory in the path to see if it exists,
/// and if it does not it will be created.
pub fn create_file(path: &Path) -> Result<File> {
    debug!("Creating {}", path.display());

    // Construct path
    if let Some(p) = path.parent() {
        trace!("Parent directory is: {:?}", p);

        fs::create_dir_all(p)?;
    }

    File::create(path).map_err(Into::into)
}

/// Removes all the content of a directory but not the directory itself
pub fn remove_dir_content(dir: &Path) -> Result<()> {
    for item in fs::read_dir(dir)?.flatten() {
        let item = item.path();
        if item.is_dir() {
            fs::remove_dir_all(item)?;
        } else {
            fs::remove_file(item)?;
        }
    }
    Ok(())
}

/// Copies all files of a directory to another one except the files
/// with the extensions given in the `ext_blacklist` array
pub fn copy_files_except_ext(
    from: &Path,
    to: &Path,
    recursive: bool,
    avoid_dir: Option<&PathBuf>,
    ext_blacklist: &[&str],
) -> Result<()> {
    debug!(
        "Copying all files from {} to {} (blacklist: {:?}), avoiding {:?}",
        from.display(),
        to.display(),
        ext_blacklist,
        avoid_dir
    );

    // Check that from and to are different
    if from == to {
        return Ok(());
    }

    for entry in fs::read_dir(from)? {
        let entry = entry?.path();
        let metadata = entry
            .metadata()
            .with_context(|| format!("Failed to read {entry:?}"))?;

        let entry_file_name = entry.file_name().unwrap();
        let target_file_path = to.join(entry_file_name);

        // If the entry is a dir and the recursive option is enabled, call itself
        if metadata.is_dir() && recursive {
            if entry == to.as_os_str() {
                continue;
            }

            if let Some(avoid) = avoid_dir {
                if entry == *avoid {
                    continue;
                }
            }

            // check if output dir already exists
            if !target_file_path.exists() {
                fs::create_dir(&target_file_path)?;
            }

            copy_files_except_ext(&entry, &target_file_path, true, avoid_dir, ext_blacklist)?;
        } else if metadata.is_file() {
            // Check if it is in the blacklist
            if let Some(ext) = entry.extension() {
                if ext_blacklist.contains(&ext.to_str().unwrap()) {
                    continue;
                }
            }
            debug!("Copying {entry:?} to {target_file_path:?}");
            copy(&entry, &target_file_path)?;
        }
    }
    Ok(())
}

/// Copies a file.
fn copy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()> {
    let from = from.as_ref();
    let to = to.as_ref();
    return copy_inner(from, to)
        .with_context(|| format!("failed to copy `{}` to `{}`", from.display(), to.display()));

    // This is a workaround for an issue with the macOS file watcher.
    // Rust's `std::fs::copy` function uses `fclonefileat`, which creates
    // clones on APFS. Unfortunately fs events seem to trigger on both
    // sides of the clone, and there doesn't seem to be a way to differentiate
    // which side it is.
    // https://github.com/notify-rs/notify/issues/465#issuecomment-1657261035
    // contains more information.
    //
    // This is essentially a copy of the simple copy code path in Rust's
    // standard library.
    #[cfg(target_os = "macos")]
    fn copy_inner(from: &Path, to: &Path) -> Result<()> {
        use std::fs::OpenOptions;
        use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};

        let mut reader = File::open(from)?;
        let metadata = reader.metadata()?;
        if !metadata.is_file() {
            anyhow::bail!(
                "expected a file, `{}` appears to be {:?}",
                from.display(),
                metadata.file_type()
            );
        }
        let perm = metadata.permissions();
        let mut writer = OpenOptions::new()
            .mode(perm.mode())
            .write(true)
            .create(true)
            .truncate(true)
            .open(to)?;
        let writer_metadata = writer.metadata()?;
        if writer_metadata.is_file() {
            // Set the correct file permissions, in case the file already existed.
            // Don't set the permissions on already existing non-files like
            // pipes/FIFOs or device nodes.
            writer.set_permissions(perm)?;
        }
        std::io::copy(&mut reader, &mut writer)?;
        Ok(())
    }

    #[cfg(not(target_os = "macos"))]
    fn copy_inner(from: &Path, to: &Path) -> Result<()> {
        fs::copy(from, to)?;
        Ok(())
    }
}

pub fn get_404_output_file(input_404: &Option<String>) -> String {
    input_404
        .as_ref()
        .unwrap_or(&"404.md".to_string())
        .replace(".md", ".html")
}

#[cfg(test)]
mod tests {
    use super::copy_files_except_ext;
    use std::{fs, io::Result, path::Path};

    #[cfg(target_os = "windows")]
    fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<()> {
        std::os::windows::fs::symlink_file(src, dst)
    }

    #[cfg(not(target_os = "windows"))]
    fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<()> {
        std::os::unix::fs::symlink(src, dst)
    }

    #[test]
    fn copy_files_except_ext_test() {
        let tmp = match tempfile::TempDir::new() {
            Ok(t) => t,
            Err(e) => panic!("Could not create a temp dir: {}", e),
        };

        // Create a couple of files
        if let Err(err) = fs::File::create(tmp.path().join("file.txt")) {
            panic!("Could not create file.txt: {}", err);
        }
        if let Err(err) = fs::File::create(tmp.path().join("file.md")) {
            panic!("Could not create file.md: {}", err);
        }
        if let Err(err) = fs::File::create(tmp.path().join("file.png")) {
            panic!("Could not create file.png: {}", err);
        }
        if let Err(err) = fs::create_dir(tmp.path().join("sub_dir")) {
            panic!("Could not create sub_dir: {}", err);
        }
        if let Err(err) = fs::File::create(tmp.path().join("sub_dir/file.png")) {
            panic!("Could not create sub_dir/file.png: {}", err);
        }
        if let Err(err) = fs::create_dir(tmp.path().join("sub_dir_exists")) {
            panic!("Could not create sub_dir_exists: {}", err);
        }
        if let Err(err) = fs::File::create(tmp.path().join("sub_dir_exists/file.txt")) {
            panic!("Could not create sub_dir_exists/file.txt: {}", err);
        }
        if let Err(err) = symlink(tmp.path().join("file.png"), tmp.path().join("symlink.png")) {
            panic!("Could not symlink file.png: {}", err);
        }

        // Create output dir
        if let Err(err) = fs::create_dir(tmp.path().join("output")) {
            panic!("Could not create output: {}", err);
        }
        if let Err(err) = fs::create_dir(tmp.path().join("output/sub_dir_exists")) {
            panic!("Could not create output/sub_dir_exists: {}", err);
        }

        if let Err(e) =
            copy_files_except_ext(tmp.path(), &tmp.path().join("output"), true, None, &["md"])
        {
            panic!("Error while executing the function:\n{:?}", e);
        }

        // Check if the correct files where created
        if !tmp.path().join("output/file.txt").exists() {
            panic!("output/file.txt should exist")
        }
        if tmp.path().join("output/file.md").exists() {
            panic!("output/file.md should not exist")
        }
        if !tmp.path().join("output/file.png").exists() {
            panic!("output/file.png should exist")
        }
        if !tmp.path().join("output/sub_dir/file.png").exists() {
            panic!("output/sub_dir/file.png should exist")
        }
        if !tmp.path().join("output/sub_dir_exists/file.txt").exists() {
            panic!("output/sub_dir/file.png should exist")
        }
        if !tmp.path().join("output/symlink.png").exists() {
            panic!("output/symlink.png should exist")
        }
    }
}
