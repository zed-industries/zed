use crate::ResultExt;
use anyhow::{Result, bail};
use async_fs as fs;
use collections::HashMap;
use futures_lite::StreamExt;
use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
    time::{Duration, SystemTime},
};

/// Removes all files and directories matching the given predicate
pub async fn remove_matching<F>(dir: &Path, predicate: F)
where
    F: Fn(&Path) -> bool,
{
    if let Some(mut entries) = fs::read_dir(dir).await.log_err() {
        while let Some(entry) = entries.next().await {
            if let Some(entry) = entry.log_err() {
                let entry_path = entry.path();
                if predicate(entry_path.as_path())
                    && let Ok(metadata) = fs::metadata(&entry_path).await
                {
                    if metadata.is_file() {
                        fs::remove_file(&entry_path).await.log_err();
                    } else {
                        fs::remove_dir_all(&entry_path).await.log_err();
                    }
                }
            }
        }
    }
}

/// Groups the files in `dir` by `group_key`, ignoring files for which it returns `None`, and
/// removes all but the `max_groups` most recently modified groups, as well as any group whose
/// newest file is older than `max_age`.
pub fn remove_stale_file_groups(
    dir: &Path,
    max_groups: usize,
    max_age: Duration,
    group_key: impl Fn(&str) -> Option<&str>,
) {
    let Some(entries) = std::fs::read_dir(dir).log_err() else {
        return;
    };

    let mut groups = HashMap::<String, (SystemTime, Vec<PathBuf>)>::default();
    for entry in entries.filter_map(|entry| entry.log_err()) {
        let file_name = entry.file_name();
        let Some(key) = file_name.to_str().and_then(&group_key) else {
            continue;
        };
        let Some(metadata) = entry.metadata().log_err() else {
            continue;
        };
        if !metadata.is_file() {
            continue;
        }
        let Some(modified) = metadata.modified().log_err() else {
            continue;
        };
        let (newest, paths) = groups
            .entry(key.to_owned())
            .or_insert_with(|| (modified, Vec::new()));
        *newest = (*newest).max(modified);
        paths.push(entry.path());
    }

    let mut groups = groups.into_values().collect::<Vec<_>>();
    groups.sort_by(|(newest_a, _), (newest_b, _)| newest_b.cmp(newest_a));
    let now = SystemTime::now();
    for (index, (newest, paths)) in groups.into_iter().enumerate() {
        let expired = now.duration_since(newest).is_ok_and(|age| age > max_age);
        if index < max_groups && !expired {
            continue;
        }
        for path in paths {
            // Several processes may clean up the same directory at once.
            if let Err(error) = std::fs::remove_file(&path)
                && error.kind() != ErrorKind::NotFound
            {
                log::warn!("failed to remove {path:?}: {error}");
            }
        }
    }
}

pub async fn collect_matching<F>(dir: &Path, predicate: F) -> Vec<PathBuf>
where
    F: Fn(&Path) -> bool,
{
    let mut matching = vec![];

    if let Some(mut entries) = fs::read_dir(dir).await.log_err() {
        while let Some(entry) = entries.next().await {
            if let Some(entry) = entry.log_err()
                && predicate(entry.path().as_path())
            {
                matching.push(entry.path());
            }
        }
    }

    matching
}

pub async fn find_file_name_in_dir<F>(dir: &Path, predicate: F) -> Option<PathBuf>
where
    F: Fn(&str) -> bool,
{
    if let Some(mut entries) = fs::read_dir(dir).await.log_err() {
        while let Some(entry) = entries.next().await {
            if let Some(entry) = entry.log_err() {
                let entry_path = entry.path();

                if let Some(file_name) = entry_path
                    .file_name()
                    .map(|file_name| file_name.to_string_lossy())
                    && predicate(&file_name)
                {
                    return Some(entry_path);
                }
            }
        }
    }

    None
}

pub async fn move_folder_files_to_folder<P: AsRef<Path>>(
    source_path: P,
    target_path: P,
) -> Result<()> {
    if !target_path.as_ref().is_dir() {
        bail!("Folder not found or is not a directory");
    }

    let mut entries = fs::read_dir(source_path.as_ref()).await?;
    while let Some(entry) = entries.next().await {
        let entry = entry?;
        let old_path = entry.path();
        let new_path = target_path.as_ref().join(entry.file_name());

        fs::rename(&old_path, &new_path).await?;
    }

    fs::remove_dir(source_path).await?;

    Ok(())
}

#[cfg(unix)]
/// Set the permissions for the given path so that the file becomes executable.
/// This is a noop for non-unix platforms.
pub async fn make_file_executable(path: &Path) -> std::io::Result<()> {
    fs::set_permissions(
        path,
        <fs::Permissions as fs::unix::PermissionsExt>::from_mode(0o755),
    )
    .await
}

#[cfg(not(unix))]
#[allow(clippy::unused_async)]
/// Set the permissions for the given path so that the file becomes executable.
/// This is a noop for non-unix platforms.
pub async fn make_file_executable(_path: &Path) -> std::io::Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_file(dir: &Path, name: &str, age: Duration) {
        let path = dir.join(name);
        std::fs::write(&path, name).unwrap();
        std::fs::File::options()
            .write(true)
            .open(&path)
            .unwrap()
            .set_modified(SystemTime::now() - age)
            .unwrap();
    }

    fn file_names(dir: &Path) -> Vec<String> {
        let mut names = std::fs::read_dir(dir)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().into_string().unwrap())
            .collect::<Vec<_>>();
        names.sort();
        names
    }

    #[test]
    fn test_remove_stale_file_groups() {
        let dir = tempfile::tempdir().unwrap();
        let hour = Duration::from_secs(60 * 60);
        write_file(dir.path(), "a.data", hour);
        write_file(dir.path(), "a.meta", 50 * hour);
        write_file(dir.path(), "b.data", 2 * hour);
        write_file(dir.path(), "c.data", 3 * hour);
        write_file(dir.path(), "c.meta", 3 * hour);
        write_file(dir.path(), "expired.data", 100 * hour);
        write_file(dir.path(), "unrelated.txt", 1000 * hour);

        fn group_key(file_name: &str) -> Option<&str> {
            file_name
                .strip_suffix(".data")
                .or_else(|| file_name.strip_suffix(".meta"))
        }

        remove_stale_file_groups(dir.path(), 10, 10 * hour, group_key);
        assert_eq!(
            file_names(dir.path()),
            [
                "a.data",
                "a.meta",
                "b.data",
                "c.data",
                "c.meta",
                "unrelated.txt"
            ],
            "groups are aged by their newest file, and unmatched files are kept"
        );

        remove_stale_file_groups(dir.path(), 2, 10 * hour, group_key);
        assert_eq!(
            file_names(dir.path()),
            ["a.data", "a.meta", "b.data", "unrelated.txt"],
            "only the most recently modified groups are kept"
        );
    }
}
