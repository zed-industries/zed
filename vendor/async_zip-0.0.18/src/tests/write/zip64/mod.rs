// Copyright Cognite AS, 2023

use crate::base::write::ZipFileWriter;
use crate::error::{Zip64ErrorCase, ZipError};
use crate::spec::consts::NON_ZIP64_MAX_SIZE;
use crate::tests::init_logger;
use crate::tests::write::AsyncSink;
use crate::{Compression, ZipEntryBuilder};
use std::io::Read;

use crate::spec::header::ExtraField;
use futures_lite::io::AsyncWriteExt;

// Useful constants for writing a large file.
const BATCH_SIZE: usize = 100_000;
const NUM_BATCHES: usize = NON_ZIP64_MAX_SIZE as usize / BATCH_SIZE + 1;
const BATCHED_FILE_SIZE: usize = NUM_BATCHES * BATCH_SIZE;

/// Test writing a small zip64 file.
/// No zip64 extra fields will be emitted for EntryWhole.
/// Z64 end of directory record & locator should be emitted
#[tokio::test]
async fn test_write_zip64_file() {
    init_logger();

    let mut buffer = Vec::new();
    let mut writer = ZipFileWriter::new(&mut buffer).force_zip64();
    let entry = ZipEntryBuilder::new("file1".to_string().into(), Compression::Stored);
    writer.write_entry_whole(entry, &[0, 0, 0, 0]).await.unwrap();
    let entry = ZipEntryBuilder::new("file2".to_string().into(), Compression::Stored);
    let mut entry_writer = writer.write_entry_stream(entry).await.unwrap();
    entry_writer.write_all(&[0, 0, 0, 0]).await.unwrap();
    entry_writer.close().await.unwrap();
    writer.close().await.unwrap();

    let cursor = std::io::Cursor::new(buffer);
    let mut zip = zip::read::ZipArchive::new(cursor).unwrap();
    let mut file1 = zip.by_name("file1").unwrap();
    assert_eq!(file1.extra_data(), Some(&[] as &[u8]));
    let mut buffer = Vec::new();
    file1.read_to_end(&mut buffer).unwrap();
    assert_eq!(buffer.as_slice(), &[0, 0, 0, 0]);
    drop(file1);

    let mut file2 = zip.by_name("file2").unwrap();
    let mut buffer = Vec::new();
    file2.read_to_end(&mut buffer).unwrap();
    assert_eq!(buffer.as_slice(), &[0, 0, 0, 0]);
}

/// Test writing a large zip64 file. This test will use upwards of 4GB of memory.
#[tokio::test]
async fn test_write_large_zip64_file() {
    init_logger();

    // Allocate space with some extra for metadata records
    let mut buffer = Vec::with_capacity(BATCHED_FILE_SIZE + 100_000);
    let mut writer = ZipFileWriter::new(&mut buffer);

    // Stream-written zip files are dubiously spec-conformant. We need to specify a valid file size
    // in order for rs-zip (and unzip) to correctly read these files.
    let entry = ZipEntryBuilder::new("file".to_string().into(), Compression::Stored);
    let entry = entry.uncompressed_size(BATCHED_FILE_SIZE as u64);
    let entry = entry.compressed_size(BATCHED_FILE_SIZE as u64);
    let mut entry_writer = writer.write_entry_stream(entry).await.unwrap();
    for _ in 0..NUM_BATCHES {
        entry_writer.write_all(&[0; BATCH_SIZE]).await.unwrap();
    }
    entry_writer.close().await.unwrap();

    assert!(writer.is_zip64);
    let cd_entry = writer.cd_entries.last().unwrap();
    match &cd_entry.entry.extra_fields.last().unwrap() {
        ExtraField::Zip64ExtendedInformation(zip64) => {
            assert_eq!(zip64.compressed_size.unwrap(), BATCHED_FILE_SIZE as u64);
            assert_eq!(zip64.uncompressed_size.unwrap(), BATCHED_FILE_SIZE as u64);
        }
        e => panic!("Expected a Zip64 extended field, got {:?}", e),
    }
    assert_eq!(cd_entry.header.uncompressed_size, NON_ZIP64_MAX_SIZE);
    assert_eq!(cd_entry.header.compressed_size, NON_ZIP64_MAX_SIZE);
    writer.close().await.unwrap();

    let cursor = std::io::Cursor::new(buffer);
    let mut archive = zip::read::ZipArchive::new(cursor).unwrap();
    let mut file = archive.by_name("file").unwrap();
    assert_eq!(file.compression(), zip::CompressionMethod::Stored);
    assert_eq!(file.size(), BATCHED_FILE_SIZE as u64);
    let mut buffer = [0; 100_000];
    let mut bytes_total = 0;
    loop {
        let read_bytes = file.read(&mut buffer).unwrap();
        if read_bytes == 0 {
            break;
        }
        bytes_total += read_bytes;
    }
    assert_eq!(bytes_total, BATCHED_FILE_SIZE);
}

/// Test writing a file, and reading it with async-zip
#[tokio::test]
async fn test_write_large_zip64_file_self_read() {
    use futures_lite::io::AsyncReadExt;

    init_logger();

    // Allocate space with some extra for metadata records
    let mut buffer = Vec::with_capacity(BATCHED_FILE_SIZE + 100_000);
    let mut writer = ZipFileWriter::new(&mut buffer);

    let entry = ZipEntryBuilder::new("file".into(), Compression::Stored);
    let mut entry_writer = writer.write_entry_stream(entry).await.unwrap();
    for _ in 0..NUM_BATCHES {
        entry_writer.write_all(&[0; BATCH_SIZE]).await.unwrap();
    }
    entry_writer.close().await.unwrap();
    writer.close().await.unwrap();

    let reader = crate::base::read::mem::ZipFileReader::new(buffer).await.unwrap();
    assert!(reader.file().zip64);
    assert_eq!(reader.file().entries[0].entry.filename().as_str().unwrap(), "file");
    assert_eq!(reader.file().entries[0].entry.compressed_size, BATCHED_FILE_SIZE as u64);
    let mut entry = reader.reader_without_entry(0).await.unwrap();

    let mut buffer = [0; 100_000];
    let mut bytes_total = 0;
    loop {
        let read_bytes = entry.read(&mut buffer).await.unwrap();
        if read_bytes == 0 {
            break;
        }
        bytes_total += read_bytes;
    }
    assert_eq!(bytes_total, BATCHED_FILE_SIZE);
}

/// Test writing a zip64 file with more than u16::MAX files.
#[tokio::test]
async fn test_write_zip64_file_many_entries() {
    init_logger();

    // The generated file will likely be ~3MB in size.
    let mut buffer = Vec::with_capacity(3_500_000);

    let mut writer = ZipFileWriter::new(&mut buffer);
    for i in 0..=u16::MAX as u32 + 1 {
        let entry = ZipEntryBuilder::new(i.to_string().into(), Compression::Stored);
        writer.write_entry_whole(entry, &[]).await.unwrap();
    }
    assert!(writer.is_zip64);
    writer.close().await.unwrap();

    let cursor = std::io::Cursor::new(buffer);
    let mut zip = zip::read::ZipArchive::new(cursor).unwrap();
    assert_eq!(zip.len(), u16::MAX as usize + 2);

    for i in 0..=u16::MAX as u32 + 1 {
        let mut file = zip.by_name(&i.to_string()).unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
    }
}

/// Tests that EntryWholeWriter switches to Zip64 mode when writing too many files for a non-Zip64.
#[tokio::test]
async fn test_zip64_when_many_files_whole() {
    let mut sink = AsyncSink;
    let mut writer = ZipFileWriter::new(&mut sink);
    for i in 0..=u16::MAX as u32 + 1 {
        let entry = ZipEntryBuilder::new(format!("{i}").into(), Compression::Stored);
        writer.write_entry_whole(entry, &[]).await.unwrap()
    }
    assert!(writer.is_zip64);
    writer.close().await.unwrap();
}

/// Tests that EntryStreamWriter switches to Zip64 mode when writing too many files for a non-Zip64.
#[tokio::test]
async fn test_zip64_when_many_files_stream() {
    let mut sink = AsyncSink;
    let mut writer = ZipFileWriter::new(&mut sink);
    for i in 0..=u16::MAX as u32 + 1 {
        let entry = ZipEntryBuilder::new(format!("{i}").into(), Compression::Stored);
        let entrywriter = writer.write_entry_stream(entry).await.unwrap();
        entrywriter.close().await.unwrap();
    }

    assert!(writer.is_zip64);
    writer.close().await.unwrap();
}

/// Tests that when force_no_zip64 is true, EntryWholeWriter errors when trying to write more than
/// u16::MAX files to a single archive.
#[tokio::test]
async fn test_force_no_zip64_errors_with_too_many_files_whole() {
    let mut sink = AsyncSink;
    let mut writer = ZipFileWriter::new(&mut sink).force_no_zip64();
    for i in 0..u16::MAX {
        let entry = ZipEntryBuilder::new(format!("{i}").into(), Compression::Stored);
        writer.write_entry_whole(entry, &[]).await.unwrap()
    }
    let entry = ZipEntryBuilder::new("65537".to_string().into(), Compression::Stored);
    let result = writer.write_entry_whole(entry, &[]).await;

    assert!(matches!(result, Err(ZipError::Zip64Needed(Zip64ErrorCase::TooManyFiles))));
}

/// Tests that when force_no_zip64 is true, EntryStreamWriter errors when trying to write more than
/// u16::MAX files to a single archive.
#[tokio::test]
async fn test_force_no_zip64_errors_with_too_many_files_stream() {
    let mut sink = AsyncSink;
    let mut writer = ZipFileWriter::new(&mut sink).force_no_zip64();
    for i in 0..u16::MAX {
        let entry = ZipEntryBuilder::new(format!("{i}").into(), Compression::Stored);
        let entrywriter = writer.write_entry_stream(entry).await.unwrap();
        entrywriter.close().await.unwrap();
    }
    let entry = ZipEntryBuilder::new("65537".to_string().into(), Compression::Stored);
    let entrywriter = writer.write_entry_stream(entry).await.unwrap();
    let result = entrywriter.close().await;

    assert!(matches!(result, Err(ZipError::Zip64Needed(Zip64ErrorCase::TooManyFiles))));
}

/// Tests that when force_no_zip64 is true, EntryStreamWriter errors when trying to write
/// a file larger than ~4 GiB to an archive.
#[tokio::test]
async fn test_force_no_zip64_errors_with_too_large_file_stream() {
    let mut sink = AsyncSink;
    let mut writer = ZipFileWriter::new(&mut sink).force_no_zip64();

    let entry = ZipEntryBuilder::new("-".to_string().into(), Compression::Stored);
    let mut entrywriter = writer.write_entry_stream(entry).await.unwrap();

    // Writing 4GB, 1kb at a time
    for _ in 0..NUM_BATCHES {
        entrywriter.write_all(&[0; BATCH_SIZE]).await.unwrap();
    }
    let result = entrywriter.close().await;

    assert!(matches!(result, Err(ZipError::Zip64Needed(Zip64ErrorCase::LargeFile))));
}
