//! Extended utilities for working with files and filesystems in Rust.

#![doc(html_root_url = "https://docs.rs/fs2/0.4.3")]

#![cfg_attr(test, feature(test))]

#[cfg(windows)]
extern crate winapi;

#[cfg(unix)]
mod unix;
#[cfg(unix)]
use unix as sys;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
use windows as sys;

use std::fs::File;
use std::io::{Error, Result};
use std::path::Path;

/// Extension trait for `std::fs::File` which provides allocation, duplication and locking methods.
///
/// ## Notes on File Locks
///
/// This library provides whole-file locks in both shared (read) and exclusive
/// (read-write) varieties.
///
/// File locks are a cross-platform hazard since the file lock APIs exposed by
/// operating system kernels vary in subtle and not-so-subtle ways.
///
/// The API exposed by this library can be safely used across platforms as long
/// as the following rules are followed:
///
///   * Multiple locks should not be created on an individual `File` instance
///     concurrently.
///   * Duplicated files should not be locked without great care.
///   * Files to be locked should be opened with at least read or write
///     permissions.
///   * File locks may only be relied upon to be advisory.
///
/// See the tests in `lib.rs` for cross-platform lock behavior that may be
/// relied upon; see the tests in `unix.rs` and `windows.rs` for examples of
/// platform-specific behavior. File locks are implemented with
/// [`flock(2)`](http://man7.org/linux/man-pages/man2/flock.2.html) on Unix and
/// [`LockFile`](https://msdn.microsoft.com/en-us/library/windows/desktop/aa365202(v=vs.85).aspx)
/// on Windows.
pub trait FileExt {

    /// Returns a duplicate instance of the file.
    ///
    /// The returned file will share the same file position as the original
    /// file.
    ///
    /// If using rustc version 1.9 or later, prefer using `File::try_clone` to this.
    ///
    /// # Notes
    ///
    /// This is implemented with
    /// [`dup(2)`](http://man7.org/linux/man-pages/man2/dup.2.html) on Unix and
    /// [`DuplicateHandle`](https://msdn.microsoft.com/en-us/library/windows/desktop/ms724251(v=vs.85).aspx)
    /// on Windows.
    fn duplicate(&self) -> Result<File>;

    /// Returns the amount of physical space allocated for a file.
    fn allocated_size(&self) -> Result<u64>;

    /// Ensures that at least `len` bytes of disk space are allocated for the
    /// file, and the file size is at least `len` bytes. After a successful call
    /// to `allocate`, subsequent writes to the file within the specified length
    /// are guaranteed not to fail because of lack of disk space.
    fn allocate(&self, len: u64) -> Result<()>;

    /// Locks the file for shared usage, blocking if the file is currently
    /// locked exclusively.
    fn lock_shared(&self) -> Result<()>;

    /// Locks the file for exclusive usage, blocking if the file is currently
    /// locked.
    fn lock_exclusive(&self) -> Result<()>;

    /// Locks the file for shared usage, or returns a an error if the file is
    /// currently locked (see `lock_contended_error`).
    fn try_lock_shared(&self) -> Result<()>;

    /// Locks the file for shared usage, or returns a an error if the file is
    /// currently locked (see `lock_contended_error`).
    fn try_lock_exclusive(&self) -> Result<()>;

    /// Unlocks the file.
    fn unlock(&self) -> Result<()>;
}

impl FileExt for File {
    fn duplicate(&self) -> Result<File> {
        sys::duplicate(self)
    }
    fn allocated_size(&self) -> Result<u64> {
        sys::allocated_size(self)
    }
    fn allocate(&self, len: u64) -> Result<()> {
        sys::allocate(self, len)
    }
    fn lock_shared(&self) -> Result<()> {
        sys::lock_shared(self)
    }
    fn lock_exclusive(&self) -> Result<()> {
        sys::lock_exclusive(self)
    }
    fn try_lock_shared(&self) -> Result<()> {
        sys::try_lock_shared(self)
    }
    fn try_lock_exclusive(&self) -> Result<()> {
        sys::try_lock_exclusive(self)
    }
    fn unlock(&self) -> Result<()> {
        sys::unlock(self)
    }
}

/// Returns the error that a call to a try lock method on a contended file will
/// return.
pub fn lock_contended_error() -> Error {
    sys::lock_error()
}

/// `FsStats` contains some common stats about a file system.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FsStats {
    free_space: u64,
    available_space: u64,
    total_space: u64,
    allocation_granularity: u64,
}

impl FsStats {
    /// Returns the number of free bytes in the file system containing the provided
    /// path.
    pub fn free_space(&self) -> u64 {
        self.free_space
    }

    /// Returns the available space in bytes to non-priveleged users in the file
    /// system containing the provided path.
    pub fn available_space(&self) -> u64 {
        self.available_space
    }

    /// Returns the total space in bytes in the file system containing the provided
    /// path.
    pub fn total_space(&self) -> u64 {
        self.total_space
    }

    /// Returns the filesystem's disk space allocation granularity in bytes.
    /// The provided path may be for any file in the filesystem.
    ///
    /// On Posix, this is equivalent to the filesystem's block size.
    /// On Windows, this is equivalent to the filesystem's cluster size.
    pub fn allocation_granularity(&self) -> u64 {
        self.allocation_granularity
    }
}

/// Get the stats of the file system containing the provided path.
pub fn statvfs<P>(path: P) -> Result<FsStats> where P: AsRef<Path> {
    sys::statvfs(path.as_ref())
}

/// Returns the number of free bytes in the file system containing the provided
/// path.
pub fn free_space<P>(path: P) -> Result<u64> where P: AsRef<Path> {
    statvfs(path).map(|stat| stat.free_space)
}

/// Returns the available space in bytes to non-priveleged users in the file
/// system containing the provided path.
pub fn available_space<P>(path: P) -> Result<u64> where P: AsRef<Path> {
    statvfs(path).map(|stat| stat.available_space)
}

/// Returns the total space in bytes in the file system containing the provided
/// path.
pub fn total_space<P>(path: P) -> Result<u64> where P: AsRef<Path> {
    statvfs(path).map(|stat| stat.total_space)
}

/// Returns the filesystem's disk space allocation granularity in bytes.
/// The provided path may be for any file in the filesystem.
///
/// On Posix, this is equivalent to the filesystem's block size.
/// On Windows, this is equivalent to the filesystem's cluster size.
pub fn allocation_granularity<P>(path: P) -> Result<u64> where P: AsRef<Path> {
    statvfs(path).map(|stat| stat.allocation_granularity)
}

#[cfg(test)]
mod test {

    extern crate tempdir;
    extern crate test;

    use std::fs;
    use super::*;
    use std::io::{Read, Seek, SeekFrom, Write};

    /// Tests file duplication.
    #[test]
    fn duplicate() {
        let tempdir = tempdir::TempDir::new("fs2").unwrap();
        let path = tempdir.path().join("fs2");
        let mut file1 =
            fs::OpenOptions::new().read(true).write(true).create(true).open(&path).unwrap();
        let mut file2 = file1.duplicate().unwrap();

        // Write into the first file and then drop it.
        file1.write_all(b"foo").unwrap();
        drop(file1);

        let mut buf = vec![];

        // Read from the second file; since the position is shared it will already be at EOF.
        file2.read_to_end(&mut buf).unwrap();
        assert_eq!(0, buf.len());

        // Rewind and read.
        file2.seek(SeekFrom::Start(0)).unwrap();
        file2.read_to_end(&mut buf).unwrap();
        assert_eq!(&buf, &b"foo");
    }

    /// Tests shared file lock operations.
    #[test]
    fn lock_shared() {
        let tempdir = tempdir::TempDir::new("fs2").unwrap();
        let path = tempdir.path().join("fs2");
        let file1 = fs::OpenOptions::new().read(true).write(true).create(true).open(&path).unwrap();
        let file2 = fs::OpenOptions::new().read(true).write(true).create(true).open(&path).unwrap();
        let file3 = fs::OpenOptions::new().read(true).write(true).create(true).open(&path).unwrap();

        // Concurrent shared access is OK, but not shared and exclusive.
        file1.lock_shared().unwrap();
        file2.lock_shared().unwrap();
        assert_eq!(file3.try_lock_exclusive().unwrap_err().kind(),
                   lock_contended_error().kind());
        file1.unlock().unwrap();
        assert_eq!(file3.try_lock_exclusive().unwrap_err().kind(),
                   lock_contended_error().kind());

        // Once all shared file locks are dropped, an exclusive lock may be created;
        file2.unlock().unwrap();
        file3.lock_exclusive().unwrap();
    }

    /// Tests exclusive file lock operations.
    #[test]
    fn lock_exclusive() {
        let tempdir = tempdir::TempDir::new("fs2").unwrap();
        let path = tempdir.path().join("fs2");
        let file1 = fs::OpenOptions::new().read(true).write(true).create(true).open(&path).unwrap();
        let file2 = fs::OpenOptions::new().read(true).write(true).create(true).open(&path).unwrap();

        // No other access is possible once an exclusive lock is created.
        file1.lock_exclusive().unwrap();
        assert_eq!(file2.try_lock_exclusive().unwrap_err().kind(),
                   lock_contended_error().kind());
        assert_eq!(file2.try_lock_shared().unwrap_err().kind(),
                   lock_contended_error().kind());

        // Once the exclusive lock is dropped, the second file is able to create a lock.
        file1.unlock().unwrap();
        file2.lock_exclusive().unwrap();
    }

    /// Tests that a lock is released after the file that owns it is dropped.
    #[test]
    fn lock_cleanup() {
        let tempdir = tempdir::TempDir::new("fs2").unwrap();
        let path = tempdir.path().join("fs2");
        let file1 = fs::OpenOptions::new().read(true).write(true).create(true).open(&path).unwrap();
        let file2 = fs::OpenOptions::new().read(true).write(true).create(true).open(&path).unwrap();

        file1.lock_exclusive().unwrap();
        assert_eq!(file2.try_lock_shared().unwrap_err().kind(),
                   lock_contended_error().kind());

        // Drop file1; the lock should be released.
        drop(file1);
        file2.lock_shared().unwrap();
    }

    /// Tests file allocation.
    #[test]
    fn allocate() {
        let tempdir = tempdir::TempDir::new("fs2").unwrap();
        let path = tempdir.path().join("fs2");
        let file = fs::OpenOptions::new().write(true).create(true).open(&path).unwrap();
        let blksize = allocation_granularity(&path).unwrap();

        // New files are created with no allocated size.
        assert_eq!(0, file.allocated_size().unwrap());
        assert_eq!(0, file.metadata().unwrap().len());

        // Allocate space for the file, checking that the allocated size steps
        // up by block size, and the file length matches the allocated size.

        file.allocate(2 * blksize - 1).unwrap();
        assert_eq!(2 * blksize, file.allocated_size().unwrap());
        assert_eq!(2 * blksize - 1, file.metadata().unwrap().len());

        // Truncate the file, checking that the allocated size steps down by
        // block size.

        file.set_len(blksize + 1).unwrap();
        assert_eq!(2 * blksize, file.allocated_size().unwrap());
        assert_eq!(blksize + 1, file.metadata().unwrap().len());
    }

    /// Checks filesystem space methods.
    #[test]
    fn filesystem_space() {
        let tempdir = tempdir::TempDir::new("fs2").unwrap();
        let total_space = total_space(&tempdir.path()).unwrap();
        let free_space = free_space(&tempdir.path()).unwrap();
        let available_space = available_space(&tempdir.path()).unwrap();

        assert!(total_space > free_space);
        assert!(total_space > available_space);
        assert!(available_space <= free_space);
    }

    /// Benchmarks creating and removing a file. This is a baseline benchmark
    /// for comparing against the truncate and allocate benchmarks.
    #[bench]
    fn bench_file_create(b: &mut test::Bencher) {
        let tempdir = tempdir::TempDir::new("fs2").unwrap();
        let path = tempdir.path().join("file");

        b.iter(|| {
            fs::OpenOptions::new()
                            .read(true)
                            .write(true)
                            .create(true)
                            .open(&path)
                            .unwrap();
            fs::remove_file(&path).unwrap();
        });
    }

    /// Benchmarks creating a file, truncating it to 32MiB, and deleting it.
    #[bench]
    fn bench_file_truncate(b: &mut test::Bencher) {
        let size = 32 * 1024 * 1024;
        let tempdir = tempdir::TempDir::new("fs2").unwrap();
        let path = tempdir.path().join("file");

        b.iter(|| {
            let file = fs::OpenOptions::new()
                                       .read(true)
                                       .write(true)
                                       .create(true)
                                       .open(&path)
                                       .unwrap();
            file.set_len(size).unwrap();
            fs::remove_file(&path).unwrap();
        });
    }

    /// Benchmarks creating a file, allocating 32MiB for it, and deleting it.
    #[bench]
    fn bench_file_allocate(b: &mut test::Bencher) {
        let size = 32 * 1024 * 1024;
        let tempdir = tempdir::TempDir::new("fs2").unwrap();
        let path = tempdir.path().join("file");

        b.iter(|| {
            let file = fs::OpenOptions::new()
                                       .read(true)
                                       .write(true)
                                       .create(true)
                                       .open(&path)
                                       .unwrap();
            file.allocate(size).unwrap();
            fs::remove_file(&path).unwrap();
        });
    }

    /// Benchmarks creating a file, allocating 32MiB for it, and deleting it.
    #[bench]
    fn bench_allocated_size(b: &mut test::Bencher) {
        let size = 32 * 1024 * 1024;
        let tempdir = tempdir::TempDir::new("fs2").unwrap();
        let path = tempdir.path().join("file");
        let file = fs::OpenOptions::new()
                                   .read(true)
                                   .write(true)
                                   .create(true)
                                   .open(&path)
                                   .unwrap();
        file.allocate(size).unwrap();

        b.iter(|| {
            file.allocated_size().unwrap();
        });
    }

    /// Benchmarks duplicating a file descriptor or handle.
    #[bench]
    fn bench_duplicate(b: &mut test::Bencher) {
        let tempdir = tempdir::TempDir::new("fs2").unwrap();
        let path = tempdir.path().join("fs2");
        let file = fs::OpenOptions::new().read(true).write(true).create(true).open(&path).unwrap();

        b.iter(|| test::black_box(file.duplicate().unwrap()));
    }

    /// Benchmarks locking and unlocking a file lock.
    #[bench]
    fn bench_lock_unlock(b: &mut test::Bencher) {
        let tempdir = tempdir::TempDir::new("fs2").unwrap();
        let path = tempdir.path().join("fs2");
        let file = fs::OpenOptions::new().read(true).write(true).create(true).open(&path).unwrap();

        b.iter(|| {
            file.lock_exclusive().unwrap();
            file.unlock().unwrap();
        });
    }

    /// Benchmarks the free space method.
    #[bench]
    fn bench_free_space(b: &mut test::Bencher) {
        let tempdir = tempdir::TempDir::new("fs2").unwrap();
        b.iter(|| {
            test::black_box(free_space(&tempdir.path()).unwrap());
        });
    }

    /// Benchmarks the available space method.
    #[bench]
    fn bench_available_space(b: &mut test::Bencher) {
        let tempdir = tempdir::TempDir::new("fs2").unwrap();
        b.iter(|| {
            test::black_box(available_space(&tempdir.path()).unwrap());
        });
    }

    /// Benchmarks the total space method.
    #[bench]
    fn bench_total_space(b: &mut test::Bencher) {
        let tempdir = tempdir::TempDir::new("fs2").unwrap();
        b.iter(|| {
            test::black_box(total_space(&tempdir.path()).unwrap());
        });
    }
}
