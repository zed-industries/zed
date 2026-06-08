use std::path::Path;
#[cfg(unix)]
use std::{
    collections::{HashMap, HashSet, VecDeque},
    ffi::{OsStr, OsString},
    path::{Component, PathBuf},
};

use anyhow::{Context as _, Result};
use async_zip::base::read;
use futures::{AsyncRead, io::BufReader};
#[cfg(not(windows))]
use futures::{AsyncReadExt, AsyncSeek};
#[cfg(unix)]
use unicase::UniCase;
#[cfg(unix)]
use unicode_normalization::UnicodeNormalization as _;

#[cfg(unix)]
const MAX_SYMLINK_TARGET_BYTES: u64 = 4096;

#[cfg(unix)]
enum SymlinkTargetComponent {
    CurDir,
    ParentDir,
    Normal(OsString),
}

#[cfg(any(unix, windows))]
fn archive_path_is_normal(filename: &str) -> bool {
    Path::new(filename).components().all(|c| {
        matches!(
            c,
            std::path::Component::Normal(_) | std::path::Component::CurDir
        )
    })
}

#[cfg(unix)]
fn zip_entry_is_symlink(unix_permissions: Option<u16>) -> bool {
    const S_IFMT: u16 = 0o170000;
    const S_IFLNK: u16 = 0o120000;

    unix_permissions.is_some_and(|permissions| permissions & S_IFMT == S_IFLNK)
}

#[cfg(unix)]
fn normalized_case_folded_component(component: &OsStr) -> String {
    let normalized = component.to_string_lossy().nfc().collect::<String>();
    UniCase::new(normalized).to_folded_case().nfc().collect()
}

#[cfg(unix)]
fn path_with_normalized_case_folded_components(path: impl AsRef<Path>) -> PathBuf {
    let path = crate::normalize_path(path.as_ref());
    let mut normalized_path = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Prefix(prefix) => {
                normalized_path.push(normalized_case_folded_component(prefix.as_os_str()));
            }
            Component::RootDir => normalized_path.push(component.as_os_str()),
            Component::CurDir => {}
            Component::ParentDir => {
                normalized_path.pop();
            }
            Component::Normal(component) => {
                normalized_path.push(normalized_case_folded_component(component));
            }
        }
    }
    normalized_path
}

#[cfg(unix)]
#[derive(Default)]
struct ArchiveSymlinkPaths {
    exact_paths: HashSet<PathBuf>,
    normalized_paths: HashSet<PathBuf>,
}

#[cfg(unix)]
impl ArchiveSymlinkPaths {
    fn insert(&mut self, path: PathBuf) {
        self.normalized_paths
            .insert(path_with_normalized_case_folded_components(&path));
        self.exact_paths.insert(path);
    }

    fn contains(&self, path: &Path) -> bool {
        self.exact_paths.contains(path)
            || self
                .normalized_paths
                .contains(&path_with_normalized_case_folded_components(path))
    }
}

#[cfg(unix)]
fn symlink_target_components(
    target: &Path,
    link_path: &Path,
) -> Result<VecDeque<SymlinkTargetComponent>> {
    let mut components = VecDeque::new();
    for component in target.components() {
        match component {
            Component::CurDir => components.push_back(SymlinkTargetComponent::CurDir),
            Component::ParentDir => components.push_back(SymlinkTargetComponent::ParentDir),
            Component::Normal(component) => {
                components.push_back(SymlinkTargetComponent::Normal(component.to_os_string()));
            }
            Component::Prefix(_) | Component::RootDir => {
                anyhow::bail!(
                    "symlink target contains unsupported component {component:?} for path {link_path:?}: {target:?}"
                );
            }
        }
    }
    Ok(components)
}

#[cfg(unix)]
fn validate_symlink_target(
    link_path: &Path,
    target: &Path,
    destination: &Path,
    archive_symlink_paths: &ArchiveSymlinkPaths,
    archive_symlinks: &HashMap<PathBuf, PathBuf>,
) -> Result<()> {
    anyhow::ensure!(
        destination.is_absolute(),
        "destination must be absolute when validating symlink target for path {link_path:?}: {destination:?}"
    );
    anyhow::ensure!(
        !target.as_os_str().is_empty(),
        "symlink target cannot be empty for path {link_path:?}"
    );
    anyhow::ensure!(
        !target.is_absolute(),
        "symlink target cannot be absolute for path {link_path:?}: {target:?}"
    );

    let link_parent = link_path
        .parent()
        .with_context(|| format!("no parent directory for symlink {link_path:?}"))?;
    let destination = crate::normalize_path(destination);
    let mut resolved_target = crate::normalize_path(link_parent);
    let mut target_components = symlink_target_components(target, link_path)?;
    let mut target_contains_unresolved_archive_symlink = false;
    let mut symlink_expansion_count = 0;
    anyhow::ensure!(
        resolved_target.starts_with(&destination),
        "symlink target escapes destination for path {link_path:?}: {target:?}"
    );

    while let Some(component) = target_components.pop_front() {
        match component {
            SymlinkTargetComponent::CurDir => {}
            SymlinkTargetComponent::ParentDir => {
                anyhow::ensure!(
                    !target_contains_unresolved_archive_symlink,
                    "symlink target traverses parent after unresolved archive symlink component for path {link_path:?}: {target:?}"
                );
                resolved_target.pop();
                anyhow::ensure!(
                    resolved_target.starts_with(&destination),
                    "symlink target escapes destination for path {link_path:?}: {target:?}"
                );
            }
            SymlinkTargetComponent::Normal(component) => {
                resolved_target.push(component);
                anyhow::ensure!(
                    resolved_target.starts_with(&destination),
                    "symlink target escapes destination for path {link_path:?}: {target:?}"
                );

                if let Some(archive_target) = archive_symlinks.get(&resolved_target) {
                    symlink_expansion_count += 1;
                    anyhow::ensure!(
                        symlink_expansion_count <= archive_symlinks.len(),
                        "symlink target resolves through too many archive symlinks for path {link_path:?}: {target:?}"
                    );
                    let symlink_parent = resolved_target.parent().with_context(|| {
                        format!("no parent directory for symlink target {resolved_target:?}")
                    })?;
                    let mut archive_target_components =
                        symlink_target_components(archive_target, &resolved_target)?;
                    archive_target_components.append(&mut target_components);
                    target_components = archive_target_components;
                    resolved_target = crate::normalize_path(symlink_parent);
                    target_contains_unresolved_archive_symlink = false;
                    continue;
                }

                match std::fs::symlink_metadata(&resolved_target) {
                    Ok(metadata) => {
                        anyhow::ensure!(
                            !metadata.file_type().is_symlink(),
                            "symlink target crosses symlinked component {resolved_target:?} for path {link_path:?}: {target:?}"
                        );
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                        if archive_symlink_paths.contains(&resolved_target) {
                            target_contains_unresolved_archive_symlink = true;
                        }
                    }
                    Err(error) => {
                        return Err(error).with_context(|| {
                            format!(
                                "reading metadata for symlink target component {resolved_target:?}"
                            )
                        });
                    }
                }
            }
        }
    }

    Ok(())
}

#[cfg(unix)]
fn ensure_no_symlinked_path_components(path: &Path, destination: &Path) -> Result<()> {
    let path = crate::normalize_path(path);
    let destination = crate::normalize_path(destination);
    anyhow::ensure!(
        path.starts_with(&destination),
        "archive path escapes destination: {path:?}"
    );

    let relative_path = path
        .strip_prefix(&destination)
        .with_context(|| format!("checking archive path {path:?} under {destination:?}"))?;
    let mut component_path = destination;
    for component in relative_path.components() {
        match component {
            Component::CurDir => {}
            Component::Normal(component) => {
                component_path.push(component);
                match std::fs::symlink_metadata(&component_path) {
                    Ok(metadata) => {
                        anyhow::ensure!(
                            !metadata.file_type().is_symlink(),
                            "archive path contains symlinked component {component_path:?} for path {path:?}"
                        );
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                    Err(error) => {
                        return Err(error).with_context(|| {
                            format!(
                                "reading metadata for archive path component {component_path:?}"
                            )
                        });
                    }
                }
            }
            Component::Prefix(_) | Component::RootDir | Component::ParentDir => {
                anyhow::bail!("archive path contains unsupported component {component:?}: {path:?}")
            }
        }
    }

    Ok(())
}

#[cfg(windows)]
pub async fn extract_zip<R: AsyncRead + Unpin>(destination: &Path, reader: R) -> Result<()> {
    let mut reader = read::stream::ZipFileReader::new(BufReader::new(reader));

    let destination = &destination
        .canonicalize()
        .unwrap_or_else(|_| destination.to_path_buf());

    while let Some(mut item) = reader.next_with_entry().await? {
        let entry_reader = item.reader_mut();
        let entry = entry_reader.entry();
        let filename = entry
            .filename()
            .as_str()
            .context("reading zip entry file name")?;

        if !archive_path_is_normal(filename) {
            reader = item.skip().await.context("reading next zip entry")?;
            continue;
        }

        let path = destination.join(filename);

        if entry
            .dir()
            .with_context(|| format!("reading zip entry metadata for path {path:?}"))?
        {
            std::fs::create_dir_all(&path)
                .with_context(|| format!("creating directory {path:?}"))?;
        } else {
            let parent_dir = path
                .parent()
                .with_context(|| format!("no parent directory for {path:?}"))?;
            std::fs::create_dir_all(parent_dir)
                .with_context(|| format!("creating parent directory {parent_dir:?}"))?;
            let mut file = smol::fs::File::create(&path)
                .await
                .with_context(|| format!("creating file {path:?}"))?;
            futures::io::copy(entry_reader, &mut file)
                .await
                .with_context(|| format!("extracting into file {path:?}"))?;
        }

        reader = item.skip().await.context("reading next zip entry")?;
    }

    Ok(())
}

#[cfg(unix)]
pub async fn extract_zip<R: AsyncRead + Unpin>(destination: &Path, reader: R) -> Result<()> {
    // Unix needs file permissions copied when extracting.
    // This is only possible to do when a reader impls `AsyncSeek` and `seek::ZipFileReader` is used.
    // `stream::ZipFileReader` also has the `unix_permissions` method, but it will always return `Some(0)`.
    //
    // A typical `reader` comes from a streaming network response, so cannot be sought right away,
    // and reading the entire archive into the memory seems wasteful.
    //
    // So, save the stream into a temporary file first and then get it read with a seeking reader.
    let mut file = async_fs::File::from(tempfile::tempfile().context("creating a temporary file")?);
    futures::io::copy(&mut BufReader::new(reader), &mut file)
        .await
        .context("saving archive contents into the temporary file")?;
    extract_seekable_zip(destination, file).await
}

#[cfg(unix)]
pub async fn extract_seekable_zip<R: AsyncRead + AsyncSeek + Unpin>(
    destination: &Path,
    reader: R,
) -> Result<()> {
    let mut reader = read::seek::ZipFileReader::new(BufReader::new(reader))
        .await
        .context("reading the zip archive")?;
    std::fs::create_dir_all(destination)
        .with_context(|| format!("creating extraction destination {destination:?}"))?;
    let destination = &destination
        .canonicalize()
        .with_context(|| format!("canonicalizing extraction destination {destination:?}"))?;
    let mut archive_symlinks = HashMap::new();
    let entries = reader.file().entries().to_vec();
    let mut archive_symlink_paths = ArchiveSymlinkPaths::default();
    for entry in &entries {
        let Ok(filename) = entry.filename().as_str() else {
            continue;
        };

        if archive_path_is_normal(filename) && zip_entry_is_symlink(entry.unix_permissions()) {
            archive_symlink_paths.insert(crate::normalize_path(&destination.join(filename)));
        }
    }

    for (i, entry) in entries.into_iter().enumerate() {
        let filename = entry
            .filename()
            .as_str()
            .context("reading zip entry file name")?;

        if !archive_path_is_normal(filename) {
            continue;
        }

        let path = destination.join(filename);

        if entry
            .dir()
            .with_context(|| format!("reading zip entry metadata for path {path:?}"))?
        {
            ensure_no_symlinked_path_components(&path, destination)?;
            std::fs::create_dir_all(&path)
                .with_context(|| format!("creating directory {path:?}"))?;
        } else {
            let parent_dir = path
                .parent()
                .with_context(|| format!("no parent directory for {path:?}"))?;
            ensure_no_symlinked_path_components(parent_dir, destination)?;
            std::fs::create_dir_all(parent_dir)
                .with_context(|| format!("creating parent directory {parent_dir:?}"))?;
            ensure_no_symlinked_path_components(parent_dir, destination)?;
            let mut entry_reader = reader
                .reader_with_entry(i)
                .await
                .with_context(|| format!("reading entry for path {path:?}"))?;
            let unix_permissions = entry.unix_permissions();

            if zip_entry_is_symlink(unix_permissions) {
                use std::os::unix::ffi::OsStringExt as _;

                anyhow::ensure!(
                    entry.uncompressed_size() <= MAX_SYMLINK_TARGET_BYTES,
                    "symlink target is too large for path {path:?}: {} bytes",
                    entry.uncompressed_size()
                );
                let mut target_bytes = Vec::new();
                let mut limited_reader = entry_reader.take(MAX_SYMLINK_TARGET_BYTES + 1);
                limited_reader
                    .read_to_end(&mut target_bytes)
                    .await
                    .with_context(|| format!("reading symlink target for path {path:?}"))?;
                anyhow::ensure!(
                    target_bytes.len() as u64 <= MAX_SYMLINK_TARGET_BYTES,
                    "symlink target is too large for path {path:?}: {} bytes",
                    target_bytes.len()
                );
                let target = PathBuf::from(std::ffi::OsString::from_vec(target_bytes));
                ensure_no_symlinked_path_components(&path, destination)?;
                validate_symlink_target(
                    &path,
                    &target,
                    destination,
                    &archive_symlink_paths,
                    &archive_symlinks,
                )?;
                std::os::unix::fs::symlink(&target, &path)
                    .with_context(|| format!("creating symlink {path:?} -> {target:?}"))?;
                archive_symlinks.insert(crate::normalize_path(&path), target);
            } else {
                ensure_no_symlinked_path_components(&path, destination)?;
                let mut file = smol::fs::File::create(&path)
                    .await
                    .with_context(|| format!("creating file {path:?}"))?;
                futures::io::copy(&mut entry_reader, &mut file)
                    .await
                    .with_context(|| format!("extracting into file {path:?}"))?;

                if let Some(perms) = unix_permissions
                    && perms != 0o000
                {
                    use std::os::unix::fs::PermissionsExt;
                    let permissions = std::fs::Permissions::from_mode(u32::from(perms));
                    file.set_permissions(permissions)
                        .await
                        .with_context(|| format!("setting permissions for file {path:?}"))?;
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use async_zip::ZipEntryBuilder;
    use async_zip::base::write::ZipFileWriter;
    use futures::{AsyncSeek, AsyncWriteExt};
    use smol::io::Cursor;
    use tempfile::TempDir;

    use super::*;

    #[allow(unused_variables)]
    async fn compress_zip(src_dir: &Path, dst: &Path, keep_file_permissions: bool) -> Result<()> {
        let mut out = smol::fs::File::create(dst).await?;
        let mut writer = ZipFileWriter::new(&mut out);

        for entry in walkdir::WalkDir::new(src_dir) {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                continue;
            }

            let relative_path = path.strip_prefix(src_dir)?;
            let data = smol::fs::read(&path).await?;

            let filename = relative_path.display().to_string();

            #[cfg(unix)]
            {
                let mut builder =
                    ZipEntryBuilder::new(filename.into(), async_zip::Compression::Deflate);
                use std::os::unix::fs::PermissionsExt;
                let metadata = std::fs::metadata(path)?;
                let perms = keep_file_permissions.then(|| metadata.permissions().mode() as u16);
                builder = builder.unix_permissions(perms.unwrap_or_default());
                writer.write_entry_whole(builder, &data).await?;
            }
            #[cfg(not(unix))]
            {
                let builder =
                    ZipEntryBuilder::new(filename.into(), async_zip::Compression::Deflate);
                writer.write_entry_whole(builder, &data).await?;
            }
        }

        writer.close().await?;
        out.flush().await?;
        out.sync_all().await?;

        Ok(())
    }

    #[track_caller]
    fn assert_file_content(path: &Path, content: &str) {
        assert!(path.exists(), "file not found: {:?}", path);
        let actual = std::fs::read_to_string(path).unwrap();
        assert_eq!(actual, content);
    }

    #[track_caller]
    fn make_test_data() -> TempDir {
        let dir = tempfile::tempdir().unwrap();
        let dst = dir.path();

        std::fs::write(dst.join("test"), "Hello world.").unwrap();
        std::fs::create_dir_all(dst.join("foo/bar")).unwrap();
        std::fs::write(dst.join("foo/bar.txt"), "Foo bar.").unwrap();
        std::fs::write(dst.join("foo/dar.md"), "Bar dar.").unwrap();
        std::fs::write(dst.join("foo/bar/dar你好.txt"), "你好世界").unwrap();

        dir
    }

    async fn read_archive(path: &Path) -> impl AsyncRead + AsyncSeek + Unpin {
        let data = smol::fs::read(&path).await.unwrap();
        Cursor::new(data)
    }

    #[test]
    fn test_extract_zip() {
        let test_dir = make_test_data();
        let zip_file = test_dir.path().join("test.zip");

        smol::block_on(async {
            compress_zip(test_dir.path(), &zip_file, true)
                .await
                .unwrap();
            let reader = read_archive(&zip_file).await;

            let dir = tempfile::tempdir().unwrap();
            let dst = dir.path();
            extract_zip(dst, reader).await.unwrap();

            assert_file_content(&dst.join("test"), "Hello world.");
            assert_file_content(&dst.join("foo/bar.txt"), "Foo bar.");
            assert_file_content(&dst.join("foo/dar.md"), "Bar dar.");
            assert_file_content(&dst.join("foo/bar/dar你好.txt"), "你好世界");
        });
    }

    #[cfg(unix)]
    #[test]
    fn test_extract_zip_preserves_executable_permissions() {
        use std::os::unix::fs::PermissionsExt;

        smol::block_on(async {
            let test_dir = tempfile::tempdir().unwrap();
            let executable_path = test_dir.path().join("my_script");

            // Create an executable file
            std::fs::write(&executable_path, "#!/bin/bash\necho 'Hello'").unwrap();
            let mut perms = std::fs::metadata(&executable_path).unwrap().permissions();
            perms.set_mode(0o755); // rwxr-xr-x
            std::fs::set_permissions(&executable_path, perms).unwrap();

            // Create zip
            let zip_file = test_dir.path().join("test.zip");
            compress_zip(test_dir.path(), &zip_file, true)
                .await
                .unwrap();

            // Extract to new location
            let extract_dir = tempfile::tempdir().unwrap();
            let reader = read_archive(&zip_file).await;
            extract_zip(extract_dir.path(), reader).await.unwrap();

            // Check permissions are preserved
            let extracted_path = extract_dir.path().join("my_script");
            assert!(extracted_path.exists());
            let extracted_perms = std::fs::metadata(&extracted_path).unwrap().permissions();
            assert_eq!(extracted_perms.mode() & 0o777, 0o755);
        });
    }

    #[cfg(unix)]
    #[test]
    fn test_extract_zip_sets_default_permissions() {
        use std::os::unix::fs::PermissionsExt;

        smol::block_on(async {
            let test_dir = tempfile::tempdir().unwrap();
            let file_path = test_dir.path().join("my_script");

            std::fs::write(&file_path, "#!/bin/bash\necho 'Hello'").unwrap();
            // The permissions will be shaped by the umask in the test environment
            let original_perms = std::fs::metadata(&file_path).unwrap().permissions();

            // Create zip
            let zip_file = test_dir.path().join("test.zip");
            compress_zip(test_dir.path(), &zip_file, false)
                .await
                .unwrap();

            // Extract to new location
            let extract_dir = tempfile::tempdir().unwrap();
            let reader = read_archive(&zip_file).await;
            extract_zip(extract_dir.path(), reader).await.unwrap();

            // Permissions were not stored, so will be whatever the umask generates
            // by default for new files. This should match what we saw when we previously wrote
            // the file.
            let extracted_path = extract_dir.path().join("my_script");
            assert!(extracted_path.exists());
            let extracted_perms = std::fs::metadata(&extracted_path).unwrap().permissions();
            assert_eq!(
                extracted_perms.mode(),
                original_perms.mode(),
                "Expected matching Unix file mode for unzipped file without keep_file_permissions"
            );
            assert_eq!(
                extracted_perms, original_perms,
                "Expected default set of permissions for unzipped file without keep_file_permissions"
            );
        });
    }

    #[test]
    fn test_archive_path_is_normal_rejects_traversal() {
        assert!(!archive_path_is_normal("../parent.txt"));
        assert!(!archive_path_is_normal("foo/../../grandparent.txt"));
        assert!(!archive_path_is_normal("/tmp/absolute.txt"));

        assert!(archive_path_is_normal("foo/bar.txt"));
        assert!(archive_path_is_normal("foo/bar/baz.txt"));
        assert!(archive_path_is_normal("./foo/bar.txt"));
        assert!(archive_path_is_normal("normal.txt"));
    }

    async fn build_zip_with_entries(entries: &[(&str, &[u8])]) -> Cursor<Vec<u8>> {
        let mut buf = Cursor::new(Vec::new());
        let mut writer = ZipFileWriter::new(&mut buf);
        for (name, data) in entries {
            let builder = ZipEntryBuilder::new((*name).into(), async_zip::Compression::Stored);
            writer.write_entry_whole(builder, data).await.unwrap();
        }
        writer.close().await.unwrap();
        buf.set_position(0);
        buf
    }

    #[cfg(unix)]
    async fn build_zip_with_unix_entries(entries: &[(&str, &[u8], u16)]) -> Cursor<Vec<u8>> {
        let entries = entries
            .iter()
            .map(|(name, data, permissions)| ((*name).into(), *data, *permissions))
            .collect::<Vec<(async_zip::ZipString, &[u8], u16)>>();
        build_zip_with_unix_zip_string_entries(&entries).await
    }

    #[cfg(unix)]
    async fn build_zip_with_unix_zip_string_entries(
        entries: &[(async_zip::ZipString, &[u8], u16)],
    ) -> Cursor<Vec<u8>> {
        let mut buf = Cursor::new(Vec::new());
        let mut writer = ZipFileWriter::new(&mut buf);
        for (name, data, permissions) in entries {
            let builder = ZipEntryBuilder::new(name.clone(), async_zip::Compression::Stored)
                .unix_permissions(*permissions);
            writer.write_entry_whole(builder, data).await.unwrap();
        }
        writer.close().await.unwrap();
        buf.set_position(0);
        buf
    }

    #[test]
    fn test_extract_zip_skips_path_traversal_entries() {
        smol::block_on(async {
            let base_dir = tempfile::tempdir().unwrap();
            let extract_dir = base_dir.path().join("subdir");
            std::fs::create_dir_all(&extract_dir).unwrap();

            let absolute_target = base_dir.path().join("absolute.txt");
            let reader = build_zip_with_entries(&[
                ("normal.txt", b"normal file"),
                ("subdir/nested.txt", b"nested file"),
                ("../parent.txt", b"parent file"),
                ("foo/../../grandparent.txt", b"grandparent file"),
                (absolute_target.to_str().unwrap(), b"absolute file"),
            ])
            .await;

            extract_zip(&extract_dir, reader).await.unwrap();

            assert_file_content(&extract_dir.join("normal.txt"), "normal file");
            assert_file_content(&extract_dir.join("subdir/nested.txt"), "nested file");

            assert!(
                !base_dir.path().join("parent.txt").exists(),
                "parent traversal entry should have been skipped"
            );
            assert!(
                !base_dir.path().join("grandparent.txt").exists(),
                "nested traversal entry should have been skipped"
            );
            assert!(
                !absolute_target.exists(),
                "absolute path entry should have been skipped"
            );
        });
    }

    #[cfg(unix)]
    #[test]
    fn test_extract_zip_extracts_entries_before_invalid_raw_filename() {
        smol::block_on(async {
            let reader = build_zip_with_unix_zip_string_entries(&[
                ("valid.txt".into(), b"valid file", 0o100644),
                (
                    async_zip::ZipString::new(
                        b"invalid-\xff.txt".to_vec(),
                        async_zip::StringEncoding::Raw,
                    ),
                    b"invalid file",
                    0o100644,
                ),
            ])
            .await;

            let extract_dir = tempfile::tempdir().unwrap();
            let error = extract_zip(extract_dir.path(), reader).await.unwrap_err();
            assert!(
                format!("{error:#}").contains("reading zip entry file name"),
                "unexpected error: {error:#}"
            );
            assert_file_content(&extract_dir.path().join("valid.txt"), "valid file");
        });
    }

    #[cfg(unix)]
    #[test]
    fn test_extract_zip_preserves_symlinks() {
        smol::block_on(async {
            let reader = build_zip_with_unix_entries(&[
                ("java.base/LICENSE", b"license", 0o100644),
                ("java.compiler/LICENSE", b"../java.base/LICENSE", 0o120755),
            ])
            .await;

            let extract_dir = tempfile::tempdir().unwrap();
            extract_zip(extract_dir.path(), reader).await.unwrap();

            let link_path = extract_dir.path().join("java.compiler/LICENSE");
            let metadata = std::fs::symlink_metadata(&link_path).unwrap();
            assert!(
                metadata.file_type().is_symlink(),
                "expected {link_path:?} to be extracted as a symlink"
            );
            assert_eq!(
                std::fs::read_link(&link_path).unwrap(),
                PathBuf::from("../java.base/LICENSE")
            );
            assert_eq!(std::fs::read_to_string(&link_path).unwrap(), "license");
        });
    }

    #[cfg(unix)]
    #[test]
    fn test_extract_zip_allows_archive_owned_symlink_chains() {
        smol::block_on(async {
            let reader = build_zip_with_unix_entries(&[
                ("lib.so.1", b"lib.so.1.2", 0o120755),
                ("lib.so", b"lib.so.1", 0o120755),
                ("lib.so.1.2", b"library", 0o100644),
            ])
            .await;

            let extract_dir = tempfile::tempdir().unwrap();
            extract_zip(extract_dir.path(), reader).await.unwrap();

            assert_eq!(
                std::fs::read_link(extract_dir.path().join("lib.so.1")).unwrap(),
                PathBuf::from("lib.so.1.2")
            );
            assert_eq!(
                std::fs::read_link(extract_dir.path().join("lib.so")).unwrap(),
                PathBuf::from("lib.so.1")
            );
            assert_file_content(&extract_dir.path().join("lib.so"), "library");
        });
    }

    #[cfg(unix)]
    #[test]
    fn test_extract_zip_rejects_symlink_targets_outside_destination() {
        smol::block_on(async {
            let reader =
                build_zip_with_unix_entries(&[("links/outside", b"../../outside", 0o120755)]).await;

            let base_dir = tempfile::tempdir().unwrap();
            let extract_dir = base_dir.path().join("extract");
            std::fs::create_dir_all(&extract_dir).unwrap();

            let error = extract_zip(&extract_dir, reader).await.unwrap_err();
            assert!(
                format!("{error:#}").contains("symlink target escapes destination"),
                "unexpected error: {error:#}"
            );
            assert!(
                std::fs::symlink_metadata(extract_dir.join("links/outside")).is_err(),
                "escaping symlink should not be created"
            );
        });
    }

    #[cfg(unix)]
    #[test]
    fn test_extract_zip_rejects_symlink_target_through_existing_symlink() {
        smol::block_on(async {
            let reader = build_zip_with_unix_entries(&[("link", b"escape/file", 0o120755)]).await;

            let extract_dir = tempfile::tempdir().unwrap();
            let outside_dir = tempfile::tempdir().unwrap();
            std::os::unix::fs::symlink(outside_dir.path(), extract_dir.path().join("escape"))
                .unwrap();

            let error = extract_zip(extract_dir.path(), reader).await.unwrap_err();
            assert!(
                format!("{error:#}").contains("symlink target crosses symlinked component"),
                "unexpected error: {error:#}"
            );
            assert!(
                std::fs::symlink_metadata(extract_dir.path().join("link")).is_err(),
                "symlink target through existing symlink should not be created"
            );
        });
    }

    #[cfg(unix)]
    #[test]
    fn test_extract_zip_rejects_symlink_target_through_existing_symlink_before_parent() {
        smol::block_on(async {
            let reader =
                build_zip_with_unix_entries(&[("link", b"escape/../safe", 0o120755)]).await;

            let extract_dir = tempfile::tempdir().unwrap();
            let outside_dir = tempfile::tempdir().unwrap();
            std::os::unix::fs::symlink(outside_dir.path(), extract_dir.path().join("escape"))
                .unwrap();

            let error = extract_zip(extract_dir.path(), reader).await.unwrap_err();
            assert!(
                format!("{error:#}").contains("symlink target crosses symlinked component"),
                "unexpected error: {error:#}"
            );
            assert!(
                std::fs::symlink_metadata(extract_dir.path().join("link")).is_err(),
                "symlink target through existing symlink should not be created"
            );
        });
    }

    #[cfg(unix)]
    #[test]
    fn test_extract_zip_rejects_symlink_target_parent_after_unresolved_component() {
        smol::block_on(async {
            let reader = build_zip_with_unix_entries(&[
                ("link", b"escape/../outside", 0o120755),
                ("escape", b".", 0o120755),
            ])
            .await;

            let extract_dir = tempfile::tempdir().unwrap();
            let error = extract_zip(extract_dir.path(), reader).await.unwrap_err();
            assert!(
                format!("{error:#}").contains(
                    "symlink target traverses parent after unresolved archive symlink component"
                ),
                "unexpected error: {error:#}"
            );
            assert!(
                std::fs::symlink_metadata(extract_dir.path().join("link")).is_err(),
                "symlink target with unresolved parent traversal should not be created"
            );
        });
    }

    #[cfg(unix)]
    #[test]
    fn test_extract_zip_rejects_symlink_target_parent_after_case_aliased_unresolved_component() {
        smol::block_on(async {
            let reader = build_zip_with_unix_entries(&[
                ("link", b"a/../outside", 0o120755),
                ("A", b".", 0o120755),
            ])
            .await;

            let extract_dir = tempfile::tempdir().unwrap();
            let error = extract_zip(extract_dir.path(), reader).await.unwrap_err();
            assert!(
                format!("{error:#}").contains(
                    "symlink target traverses parent after unresolved archive symlink component"
                ),
                "unexpected error: {error:#}"
            );
            assert!(
                std::fs::symlink_metadata(extract_dir.path().join("link")).is_err(),
                "symlink target with case-aliased parent traversal should not be created"
            );
        });
    }

    #[cfg(unix)]
    #[test]
    fn test_extract_zip_rejects_symlink_target_parent_after_normalized_unresolved_component() {
        smol::block_on(async {
            let decomposed_target = "dir/e\u{301}/../outside";
            let reader = build_zip_with_unix_entries(&[
                ("link", decomposed_target.as_bytes(), 0o120755),
                ("dir/\u{e9}", b"..", 0o120755),
            ])
            .await;

            let extract_dir = tempfile::tempdir().unwrap();
            let error = extract_zip(extract_dir.path(), reader).await.unwrap_err();
            assert!(
                format!("{error:#}").contains(
                    "symlink target traverses parent after unresolved archive symlink component"
                ),
                "unexpected error: {error:#}"
            );
            assert!(
                std::fs::symlink_metadata(extract_dir.path().join("link")).is_err(),
                "symlink target with normalized parent traversal should not be created"
            );
        });
    }

    #[cfg(unix)]
    #[test]
    fn test_extract_zip_allows_symlink_target_parent_after_missing_non_archive_component() {
        smol::block_on(async {
            let reader =
                build_zip_with_unix_entries(&[("link", b"missing/../safe", 0o120755)]).await;

            let extract_dir = tempfile::tempdir().unwrap();
            extract_zip(extract_dir.path(), reader).await.unwrap();
            assert_eq!(
                std::fs::read_link(extract_dir.path().join("link")).unwrap(),
                PathBuf::from("missing/../safe")
            );
        });
    }

    #[cfg(unix)]
    #[test]
    fn test_extract_zip_rejects_symlinked_ancestor_for_symlink_entry() {
        smol::block_on(async {
            let reader = build_zip_with_unix_entries(&[
                ("a", b".", 0o120755),
                ("a/b/c/link", b"../../../outside", 0o120755),
            ])
            .await;

            let extract_dir = tempfile::tempdir().unwrap();
            let error = extract_zip(extract_dir.path(), reader).await.unwrap_err();
            assert!(
                format!("{error:#}").contains("archive path contains symlinked component"),
                "unexpected error: {error:#}"
            );
            assert!(
                std::fs::symlink_metadata(extract_dir.path().join("a"))
                    .unwrap()
                    .file_type()
                    .is_symlink(),
                "first symlink should have been created"
            );
            assert!(
                std::fs::symlink_metadata(extract_dir.path().join("b/c/link")).is_err(),
                "symlinked ancestor should not be followed while creating the link"
            );
        });
    }

    #[cfg(unix)]
    #[test]
    fn test_extract_zip_rejects_symlinked_ancestor_for_file_entry() {
        smol::block_on(async {
            let reader = build_zip_with_unix_entries(&[
                ("a", b".", 0o120755),
                ("a/file", b"payload", 0o100644),
            ])
            .await;

            let extract_dir = tempfile::tempdir().unwrap();
            let error = extract_zip(extract_dir.path(), reader).await.unwrap_err();
            assert!(
                format!("{error:#}").contains("archive path contains symlinked component"),
                "unexpected error: {error:#}"
            );
            assert!(
                std::fs::symlink_metadata(extract_dir.path().join("file")).is_err(),
                "symlinked ancestor should not be followed while creating the file"
            );
        });
    }

    #[cfg(unix)]
    #[test]
    fn test_extract_zip_rejects_regular_file_over_existing_symlink() {
        smol::block_on(async {
            let reader = build_zip_with_unix_entries(&[
                ("target", b"original", 0o100644),
                ("link", b"target", 0o120755),
                ("link", b"replacement", 0o100644),
            ])
            .await;

            let extract_dir = tempfile::tempdir().unwrap();
            let error = extract_zip(extract_dir.path(), reader).await.unwrap_err();
            assert!(
                format!("{error:#}").contains("archive path contains symlinked component"),
                "unexpected error: {error:#}"
            );
            assert_file_content(&extract_dir.path().join("target"), "original");
            assert_eq!(
                std::fs::read_link(extract_dir.path().join("link")).unwrap(),
                PathBuf::from("target")
            );
        });
    }

    #[cfg(unix)]
    #[test]
    fn test_extract_zip_rejects_relative_destination_symlink_escape() {
        smol::block_on(async {
            let base_dir = tempfile::Builder::new()
                .prefix("archive-symlink-test-")
                .tempdir_in(".")
                .unwrap();
            let base_dir_name = base_dir.path().file_name().unwrap().to_os_string();
            let relative_extract_dir = PathBuf::from(&base_dir_name).join("extract");
            let target = format!("../../../{}/extract/file", base_dir_name.to_string_lossy());
            let reader =
                build_zip_with_unix_entries(&[("link", target.as_bytes(), 0o120755)]).await;

            let error = extract_zip(&relative_extract_dir, reader)
                .await
                .unwrap_err();
            assert!(
                format!("{error:#}").contains("symlink target escapes destination"),
                "unexpected error: {error:#}"
            );
            assert!(
                std::fs::symlink_metadata(relative_extract_dir.join("link")).is_err(),
                "escaping symlink should not be created"
            );
        });
    }

    #[cfg(unix)]
    #[test]
    fn test_extract_zip_rejects_oversized_symlink_target() {
        smol::block_on(async {
            let oversized_target = vec![b'a'; MAX_SYMLINK_TARGET_BYTES as usize + 1];
            let reader =
                build_zip_with_unix_entries(&[("link", oversized_target.as_slice(), 0o120755)])
                    .await;

            let extract_dir = tempfile::tempdir().unwrap();
            let error = extract_zip(extract_dir.path(), reader).await.unwrap_err();
            assert!(
                format!("{error:#}").contains("symlink target is too large"),
                "unexpected error: {error:#}"
            );
            assert!(
                std::fs::symlink_metadata(extract_dir.path().join("link")).is_err(),
                "oversized symlink should not be created"
            );
        });
    }
}
