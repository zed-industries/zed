use std::path::Path;

use anyhow::{Context as _, Result};
use async_zip::base::read;
#[cfg(not(windows))]
use futures::AsyncSeek;
use futures::{AsyncRead, io::BufReader};

#[cfg(windows)]
pub async fn extract_zip<R: AsyncRead + Unpin>(destination: &Path, reader: R) -> Result<()> {
    let mut reader = read::stream::ZipFileReader::new(BufReader::new(reader));

    let destination = &destination
        .canonicalize()
        .unwrap_or_else(|_| destination.to_path_buf());

    while let Some(mut item) = reader.next_with_entry().await? {
        let entry_reader = item.reader_mut();
        let entry = entry_reader.entry();
        let path = destination.join(
            entry
                .filename()
                .as_str()
                .context("reading zip entry file name")?,
        );

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

#[cfg(not(windows))]
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

#[cfg(not(windows))]
pub async fn extract_seekable_zip<R: AsyncRead + AsyncSeek + Unpin>(
    destination: &Path,
    reader: R,
) -> Result<()> {
    let mut reader = read::seek::ZipFileReader::new(BufReader::new(reader))
        .await
        .context("reading the zip archive")?;
    let destination = &destination
        .canonicalize()
        .unwrap_or_else(|_| destination.to_path_buf());
    for (i, entry) in reader.file().entries().to_vec().into_iter().enumerate() {
        let path = destination.join(
            entry
                .filename()
                .as_str()
                .context("reading zip entry file name")?,
        );

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
            let mut entry_reader = reader
                .reader_with_entry(i)
                .await
                .with_context(|| format!("reading entry for path {path:?}"))?;
            futures::io::copy(&mut entry_reader, &mut file)
                .await
                .with_context(|| format!("extracting into file {path:?}"))?;

            if let Some(perms) = entry.unix_permissions() {
                use std::os::unix::fs::PermissionsExt;
                let permissions = std::fs::Permissions::from_mode(u32::from(perms));
                file.set_permissions(permissions)
                    .await
                    .with_context(|| format!("setting permissions for file {path:?}"))?;
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

    async fn compress_zip(src_dir: &Path, dst: &Path) -> Result<()> {
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
                let perms = metadata.permissions().mode() as u16;
                builder = builder.unix_permissions(perms);
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
            compress_zip(test_dir.path(), &zip_file).await.unwrap();
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
            compress_zip(test_dir.path(), &zip_file).await.unwrap();

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
}
