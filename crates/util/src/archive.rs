use std::path::Path;

use anyhow::{Context as _, Result};
use async_zip::base::read;
#[cfg(not(windows))]
use futures::AsyncSeek;
use futures::{AsyncRead, io::BufReader};

#[cfg(any(unix, windows))]
fn archive_path_is_normal(filename: &str) -> bool {
    Path::new(filename).components().all(|c| {
        matches!(
            c,
            std::path::Component::Normal(_) | std::path::Component::CurDir
        )
    })
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
    let destination_dir = ArchiveExtractionRoot::new(destination)?;
    for (i, entry) in reader.file().entries().to_vec().into_iter().enumerate() {
        let filename = entry
            .filename()
            .as_str()
            .context("reading zip entry file name")?;

        if !archive_path_is_normal(filename) {
            continue;
        }

        let path = destination.join(filename);
        let archive_path = Path::new(filename);

        if entry
            .dir()
            .with_context(|| format!("reading zip entry metadata for path {path:?}"))?
        {
            destination_dir
                .open_directory(archive_path)
                .with_context(|| format!("creating directory {path:?}"))?;
        } else {
            let (parent_dir, file_name, parent_depth) =
                destination_dir
                    .open_parent(archive_path)
                    .with_context(|| format!("creating parent directory for {path:?}"))?;
            let mut entry_reader = reader
                .reader_with_entry(i)
                .await
                .with_context(|| format!("reading entry for path {path:?}"))?;

            if entry
                .unix_permissions()
                .is_some_and(|permissions| permissions & 0o170000 == 0o120000)
            {
                use std::os::fd::AsRawFd as _;

                use nix::unistd::symlinkat;

                let mut target = Vec::new();
                futures::AsyncReadExt::read_to_end(&mut entry_reader, &mut target)
                    .await
                    .with_context(|| format!("reading symlink target for path {path:?}"))?;
                let target = std::str::from_utf8(&target)
                    .with_context(|| format!("reading symlink target for path {path:?}"))?;
                let target_path = Path::new(target);
                anyhow::ensure!(
                    symlink_target_is_safe(parent_depth, target_path),
                    "symlink target escapes extraction directory: {path:?} -> {target:?}"
                );
                symlinkat(target_path, Some(parent_dir.as_raw_fd()), file_name)
                    .with_context(|| format!("creating symlink {path:?} -> {target:?}"))?;
            } else {
                let mut file = smol::fs::File::from(
                    open_archive_file(&parent_dir, file_name)
                        .with_context(|| format!("creating file {path:?}"))?,
                );
                futures::io::copy(&mut entry_reader, &mut file)
                    .await
                    .with_context(|| format!("extracting into file {path:?}"))?;

                if let Some(perms) = entry.unix_permissions()
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

#[cfg(unix)]
struct ArchiveExtractionRoot {
    directory: std::fs::File,
}

#[cfg(unix)]
impl ArchiveExtractionRoot {
    fn new(destination: &Path) -> Result<Self> {
        use std::os::fd::FromRawFd as _;

        use nix::{
            fcntl::{OFlag, open},
            sys::stat::Mode,
        };

        std::fs::create_dir_all(destination)
            .with_context(|| format!("creating extraction directory {destination:?}"))?;
        let directory = open(
            destination,
            OFlag::O_RDONLY | OFlag::O_DIRECTORY | OFlag::O_NOFOLLOW | OFlag::O_CLOEXEC,
            Mode::empty(),
        )
        .with_context(|| format!("opening extraction directory {destination:?}"))?;
        let directory = unsafe { std::fs::File::from_raw_fd(directory) };

        Ok(Self { directory })
    }

    fn open_directory(&self, path: &Path) -> Result<std::fs::File> {
        self.open_directory_components(&normal_components(path))
    }

    fn open_parent<'a>(
        &self,
        path: &'a Path,
    ) -> Result<(std::fs::File, &'a std::ffi::OsStr, usize)> {
        let mut components = normal_components(path);
        let file_name = components.pop().context("archive entry has no file name")?;
        let parent_depth = components.len();
        let directory = self.open_directory_components(&components)?;

        Ok((directory, file_name, parent_depth))
    }

    fn open_directory_components(&self, components: &[&std::ffi::OsStr]) -> Result<std::fs::File> {
        let mut directory = self
            .directory
            .try_clone()
            .context("cloning extraction directory")?;

        for component in components {
            directory = open_or_create_directory(&directory, component)?;
        }

        Ok(directory)
    }
}

#[cfg(unix)]
fn normal_components(path: &Path) -> Vec<&std::ffi::OsStr> {
    path.components()
        .filter_map(|component| match component {
            std::path::Component::Normal(component) => Some(component),
            _ => None,
        })
        .collect()
}

#[cfg(unix)]
fn open_or_create_directory(
    parent: &std::fs::File,
    component: &std::ffi::OsStr,
) -> Result<std::fs::File> {
    use std::os::fd::{AsRawFd as _, FromRawFd as _};

    use nix::{
        errno::Errno,
        fcntl::{OFlag, openat},
        sys::stat::{Mode, mkdirat},
    };

    let flags = OFlag::O_RDONLY | OFlag::O_DIRECTORY | OFlag::O_NOFOLLOW | OFlag::O_CLOEXEC;
    let directory = match openat(Some(parent.as_raw_fd()), component, flags, Mode::empty()) {
        Ok(file_descriptor) => file_descriptor,
        Err(Errno::ENOENT) => {
            match mkdirat(
                Some(parent.as_raw_fd()),
                component,
                Mode::from_bits_truncate(0o755),
            ) {
                Ok(()) | Err(Errno::EEXIST) => {}
                Err(error) => return Err(error).context("creating extraction directory"),
            }
            openat(Some(parent.as_raw_fd()), component, flags, Mode::empty())
                .context("opening extraction directory")?
        }
        Err(error) => return Err(error).context("opening extraction directory"),
    };

    Ok(unsafe { std::fs::File::from_raw_fd(directory) })
}

#[cfg(unix)]
fn open_archive_file(parent: &std::fs::File, file_name: &std::ffi::OsStr) -> Result<std::fs::File> {
    use std::os::fd::{AsRawFd as _, FromRawFd as _};

    use nix::{
        fcntl::{OFlag, openat},
        sys::stat::Mode,
    };

    let file_descriptor = openat(
        Some(parent.as_raw_fd()),
        file_name,
        OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_TRUNC | OFlag::O_NOFOLLOW | OFlag::O_CLOEXEC,
        Mode::from_bits_truncate(0o666),
    )
    .context("opening archive file")?;
    Ok(unsafe { std::fs::File::from_raw_fd(file_descriptor) })
}

#[cfg(unix)]
fn symlink_target_is_safe(parent_depth: usize, target: &Path) -> bool {
    if target.is_absolute() {
        return false;
    }

    let mut depth = parent_depth;
    let mut encountered_normal_component = false;
    for component in target.components() {
        match component {
            std::path::Component::Normal(_) => {
                encountered_normal_component = true;
                depth += 1;
            }
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                // A preceding component may be an archive symlink. Traversing upwards
                // after it would make the lexical depth check diverge from filesystem resolution.
                if encountered_normal_component || depth == 0 {
                    return false;
                }
                depth -= 1;
            }
            _ => return false,
        }
    }

    true
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
    fn test_extract_zip_preserves_symlinks() -> Result<()> {
        smol::block_on(async {
            let mut archive = Cursor::new(Vec::new());
            let mut writer = ZipFileWriter::new(&mut archive);
            let target = ZipEntryBuilder::new(
                "Python.framework/Versions/3.12/Python".into(),
                async_zip::Compression::Stored,
            );
            writer.write_entry_whole(target, b"python binary").await?;
            let link = ZipEntryBuilder::new("Python".into(), async_zip::Compression::Stored)
                .unix_permissions(0o120777);
            writer
                .write_entry_whole(link, b"Python.framework/Versions/3.12/Python")
                .await?;
            writer.close().await?;
            archive.set_position(0);

            let extract_dir = tempfile::tempdir()?;
            extract_seekable_zip(extract_dir.path(), archive).await?;

            assert_eq!(
                std::fs::read_link(extract_dir.path().join("Python"))?,
                Path::new("Python.framework/Versions/3.12/Python")
            );
            assert!(
                std::fs::symlink_metadata(extract_dir.path().join("Python"))?
                    .file_type()
                    .is_symlink()
            );
            assert_file_content(&extract_dir.path().join("Python"), "python binary");
            Ok(())
        })
    }

    #[cfg(unix)]
    #[test]
    fn test_extract_zip_allows_valid_symlink_from_root() -> Result<()> {
        smol::block_on(async {
            let mut archive = Cursor::new(Vec::new());
            let mut writer = ZipFileWriter::new(&mut archive);

            let target = ZipEntryBuilder::new("target.txt".into(), async_zip::Compression::Stored);
            writer.write_entry_whole(target, b"target content").await?;

            let link = ZipEntryBuilder::new("link".into(), async_zip::Compression::Stored)
                .unix_permissions(0o120777);
            writer.write_entry_whole(link, b"./target.txt").await?;

            writer.close().await?;
            archive.set_position(0);

            let extract_dir = tempfile::tempdir()?;
            extract_seekable_zip(extract_dir.path(), archive).await?;

            assert_eq!(
                std::fs::read_link(extract_dir.path().join("link"))?,
                Path::new("./target.txt")
            );
            assert_file_content(&extract_dir.path().join("link"), "target content");
            Ok(())
        })
    }

    #[cfg(unix)]
    #[test]
    fn test_extract_zip_allows_valid_nested_symlink_traversal() -> Result<()> {
        smol::block_on(async {
            let mut archive = Cursor::new(Vec::new());
            let mut writer = ZipFileWriter::new(&mut archive);
            writer
                .write_entry_whole(
                    ZipEntryBuilder::new("lib/target".into(), async_zip::Compression::Stored),
                    b"target content",
                )
                .await?;
            writer
                .write_entry_whole(
                    ZipEntryBuilder::new("bin/python".into(), async_zip::Compression::Stored)
                        .unix_permissions(0o120777),
                    b"../lib/target",
                )
                .await?;
            writer.close().await?;
            archive.set_position(0);

            let extract_dir = tempfile::tempdir()?;
            extract_seekable_zip(extract_dir.path(), archive).await?;

            assert_file_content(&extract_dir.path().join("bin/python"), "target content");
            Ok(())
        })
    }

    #[cfg(unix)]
    #[test]
    fn test_extract_zip_rejects_absolute_symlink_target() -> Result<()> {
        smol::block_on(async {
            let mut archive = Cursor::new(Vec::new());
            let mut writer = ZipFileWriter::new(&mut archive);
            writer
                .write_entry_whole(
                    ZipEntryBuilder::new("link".into(), async_zip::Compression::Stored)
                        .unix_permissions(0o120777),
                    b"/etc/passwd",
                )
                .await?;
            writer.close().await?;
            archive.set_position(0);

            let extract_dir = tempfile::tempdir()?;
            assert!(
                extract_seekable_zip(extract_dir.path(), archive)
                    .await
                    .is_err()
            );
            assert!(!extract_dir.path().join("link").exists());
            Ok(())
        })
    }

    #[cfg(unix)]
    #[test]
    fn test_extract_zip_rejects_symlink_target_with_late_traversal() -> Result<()> {
        smol::block_on(async {
            let mut archive = Cursor::new(Vec::new());
            let mut writer = ZipFileWriter::new(&mut archive);
            writer
                .write_entry_whole(
                    ZipEntryBuilder::new("c/target".into(), async_zip::Compression::Stored),
                    b"target content",
                )
                .await?;
            writer
                .write_entry_whole(
                    ZipEntryBuilder::new("d/a".into(), async_zip::Compression::Stored)
                        .unix_permissions(0o120777),
                    b"../c",
                )
                .await?;
            writer
                .write_entry_whole(
                    ZipEntryBuilder::new("b".into(), async_zip::Compression::Stored)
                        .unix_permissions(0o120777),
                    b"d/a/../../outside",
                )
                .await?;
            writer.close().await?;
            archive.set_position(0);

            let extract_dir = tempfile::tempdir()?;
            assert!(
                extract_seekable_zip(extract_dir.path(), archive)
                    .await
                    .is_err()
            );
            assert!(!extract_dir.path().join("b").exists());
            Ok(())
        })
    }

    #[cfg(unix)]
    #[test]
    fn test_extract_zip_rejects_symlink_traversal() -> Result<()> {
        smol::block_on(async {
            let mut archive = Cursor::new(Vec::new());
            let mut writer = ZipFileWriter::new(&mut archive);

            let target =
                ZipEntryBuilder::new("dir/target.txt".into(), async_zip::Compression::Stored);
            writer.write_entry_whole(target, b"target content").await?;

            let link = ZipEntryBuilder::new("link".into(), async_zip::Compression::Stored)
                .unix_permissions(0o120777);
            writer
                .write_entry_whole(link, b"../../../etc/passwd")
                .await?;

            let link2 = ZipEntryBuilder::new("link2".into(), async_zip::Compression::Stored)
                .unix_permissions(0o120777);
            writer
                .write_entry_whole(link2, b"../../../../etc/passwd")
                .await?;

            writer.close().await?;
            archive.set_position(0);

            let extract_dir = tempfile::tempdir()?;
            let result = extract_seekable_zip(extract_dir.path(), archive).await;

            assert!(
                result.is_err(),
                "Expected extraction to fail for escaping symlink"
            );
            let error = match result {
                Ok(()) => anyhow::bail!("expected extraction to reject an escaping symlink"),
                Err(error) => error,
            };
            assert!(error.to_string().contains("escapes extraction directory"));
            Ok(())
        })
    }

    #[cfg(unix)]
    #[test]
    fn test_extract_zip_rejects_symlinked_parent_directories() -> Result<()> {
        smol::block_on(async {
            let mut archive = Cursor::new(Vec::new());
            let mut writer = ZipFileWriter::new(&mut archive);

            let symlink = |path: &str| {
                ZipEntryBuilder::new(path.into(), async_zip::Compression::Stored)
                    .unix_permissions(0o120777)
            };
            writer
                .write_entry_whole(symlink("foo/bar/redirect"), b"../../safe")
                .await?;
            writer
                .write_entry_whole(symlink("foo/bar/redirect/link"), b"../../outside")
                .await?;
            writer
                .write_entry_whole(
                    ZipEntryBuilder::new(
                        "foo/bar/redirect/link/payload".into(),
                        async_zip::Compression::Stored,
                    ),
                    b"must not escape",
                )
                .await?;
            writer.close().await?;
            archive.set_position(0);

            let root = tempfile::tempdir()?;
            let destination = root.path().join("extract");
            let outside = root.path().join("outside");
            std::fs::create_dir(&outside)?;

            assert!(extract_seekable_zip(&destination, archive).await.is_err());
            assert!(!outside.join("payload").exists());
            Ok(())
        })
    }

    #[cfg(unix)]
    #[test]
    fn test_extract_zip_rejects_symlinked_destination() -> Result<()> {
        use std::os::unix::fs::symlink;

        smol::block_on(async {
            let archive = build_zip_with_entries(&[("payload", b"must not extract")]).await;
            let root = tempfile::tempdir()?;
            let redirected_destination = root.path().join("redirected");
            std::fs::create_dir(&redirected_destination)?;
            let destination = root.path().join("extract");
            symlink(&redirected_destination, &destination)?;

            assert!(extract_seekable_zip(&destination, archive).await.is_err());
            assert!(!redirected_destination.join("payload").exists());
            Ok(())
        })
    }

    #[cfg(unix)]
    #[test]
    fn test_extract_zip_rejects_regular_file_over_symlink() -> Result<()> {
        smol::block_on(async {
            let mut archive = Cursor::new(Vec::new());
            let mut writer = ZipFileWriter::new(&mut archive);
            writer
                .write_entry_whole(
                    ZipEntryBuilder::new("target".into(), async_zip::Compression::Stored),
                    b"safe content",
                )
                .await?;
            writer
                .write_entry_whole(
                    ZipEntryBuilder::new("link".into(), async_zip::Compression::Stored)
                        .unix_permissions(0o120777),
                    b"target",
                )
                .await?;
            writer
                .write_entry_whole(
                    ZipEntryBuilder::new("link".into(), async_zip::Compression::Stored),
                    b"must not overwrite the target",
                )
                .await?;
            writer.close().await?;
            archive.set_position(0);

            let extract_dir = tempfile::tempdir()?;
            assert!(
                extract_seekable_zip(extract_dir.path(), archive)
                    .await
                    .is_err()
            );
            assert_eq!(
                std::fs::read_to_string(extract_dir.path().join("target"))?,
                "safe content"
            );
            Ok(())
        })
    }

    #[cfg(unix)]
    #[test]
    fn test_extract_zip_surfaces_corrupt_payload_errors() -> Result<()> {
        smol::block_on(async {
            let mut archive = Cursor::new(Vec::new());
            let mut writer = ZipFileWriter::new(&mut archive);
            writer
                .write_entry_whole(
                    ZipEntryBuilder::new("payload".into(), async_zip::Compression::Deflate),
                    b"payload contents",
                )
                .await?;
            writer.close().await?;

            let mut archive = archive.into_inner();
            let filename_length = u16::from_le_bytes(
                archive
                    .get(26..28)
                    .context("reading ZIP local header filename length")?
                    .try_into()
                    .context("decoding ZIP local header filename length")?,
            ) as usize;
            let extra_field_length = u16::from_le_bytes(
                archive
                    .get(28..30)
                    .context("reading ZIP local header extra field length")?
                    .try_into()
                    .context("decoding ZIP local header extra field length")?,
            ) as usize;
            let compressed_data_offset = 30 + filename_length + extra_field_length;
            let compressed_data = archive
                .get_mut(compressed_data_offset)
                .context("reading ZIP compressed payload")?;
            *compressed_data ^= 0xff;

            let extract_dir = tempfile::tempdir()?;
            assert!(
                extract_seekable_zip(extract_dir.path(), Cursor::new(archive))
                    .await
                    .is_err()
            );
            Ok(())
        })
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
}
