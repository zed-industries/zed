//! Timestamps for files in Rust
//!
//! This library provides platform-agnostic inspection of the various timestamps
//! present in the standard `fs::Metadata` structure.
//!
//! # Installation
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! filetime = "0.2"
//! ```
//!
//! # Usage
//!
//! ```no_run
//! use std::fs;
//! use filetime::FileTime;
//!
//! let metadata = fs::metadata("foo.txt").unwrap();
//!
//! let mtime = FileTime::from_last_modification_time(&metadata);
//! println!("{}", mtime);
//!
//! let atime = FileTime::from_last_access_time(&metadata);
//! assert!(mtime < atime);
//!
//! // Inspect values that can be interpreted across platforms
//! println!("{}", mtime.unix_seconds());
//! println!("{}", mtime.nanoseconds());
//!
//! // Print the platform-specific value of seconds
//! println!("{}", mtime.seconds());
//! ```

use std::fmt;
use std::fs;
use std::io;
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

cfg_if::cfg_if! {
    if #[cfg(target_os = "redox")] {
        #[path = "redox.rs"]
        mod imp;
    } else if #[cfg(windows)] {
        #[path = "windows.rs"]
        mod imp;
    } else if #[cfg(all(target_family = "wasm", not(target_os = "emscripten")))] {
        #[path = "wasm.rs"]
        mod imp;
    } else {
        #[path = "unix/mod.rs"]
        mod imp;
    }
}

/// A helper structure to represent a timestamp for a file.
///
/// The actual value contined within is platform-specific and does not have the
/// same meaning across platforms, but comparisons and stringification can be
/// significant among the same platform.
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone, Hash)]
pub struct FileTime {
    seconds: i64,
    nanos: u32,
}

impl FileTime {
    /// Creates a new timestamp representing a 0 time.
    ///
    /// Useful for creating the base of a cmp::max chain of times.
    pub const fn zero() -> FileTime {
        FileTime {
            seconds: 0,
            nanos: 0,
        }
    }

    const fn emulate_second_only_system(self) -> FileTime {
        if cfg!(emulate_second_only_system) {
            FileTime {
                seconds: self.seconds,
                nanos: 0,
            }
        } else {
            self
        }
    }

    /// Creates a new timestamp representing the current system time.
    ///
    /// ```
    /// # use filetime::FileTime;
    /// #
    /// # fn example() -> std::io::Result<()> {
    /// #     let path = "";
    /// #
    /// filetime::set_file_mtime(path, FileTime::now())?;
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// Equivalent to `FileTime::from_system_time(SystemTime::now())`.
    pub fn now() -> FileTime {
        FileTime::from_system_time(SystemTime::now())
    }

    /// Creates a new instance of `FileTime` with a number of seconds and
    /// nanoseconds relative to the Unix epoch, 1970-01-01T00:00:00Z.
    ///
    /// Negative seconds represent times before the Unix epoch, and positive
    /// values represent times after it. Nanos always count forwards in time.
    ///
    /// Note that this is typically the relative point that Unix time stamps are
    /// from, but on Windows the native time stamp is relative to January 1,
    /// 1601 so the return value of `seconds` from the returned `FileTime`
    /// instance may not be the same as that passed in.
    pub const fn from_unix_time(seconds: i64, nanos: u32) -> FileTime {
        FileTime {
            seconds: seconds + if cfg!(windows) { 11644473600 } else { 0 },
            nanos,
        }
        .emulate_second_only_system()
    }

    /// Creates a new timestamp from the last modification time listed in the
    /// specified metadata.
    ///
    /// The returned value corresponds to the `mtime` field of `stat` on Unix
    /// platforms and the `ftLastWriteTime` field on Windows platforms.
    pub fn from_last_modification_time(meta: &fs::Metadata) -> FileTime {
        imp::from_last_modification_time(meta).emulate_second_only_system()
    }

    /// Creates a new timestamp from the last access time listed in the
    /// specified metadata.
    ///
    /// The returned value corresponds to the `atime` field of `stat` on Unix
    /// platforms and the `ftLastAccessTime` field on Windows platforms.
    pub fn from_last_access_time(meta: &fs::Metadata) -> FileTime {
        imp::from_last_access_time(meta).emulate_second_only_system()
    }

    /// Creates a new timestamp from the creation time listed in the specified
    /// metadata.
    ///
    /// The returned value corresponds to the `birthtime` field of `stat` on
    /// Unix platforms and the `ftCreationTime` field on Windows platforms. Note
    /// that not all Unix platforms have this field available and may return
    /// `None` in some circumstances.
    pub fn from_creation_time(meta: &fs::Metadata) -> Option<FileTime> {
        imp::from_creation_time(meta).map(|x| x.emulate_second_only_system())
    }

    /// Creates a new timestamp from the given SystemTime.
    ///
    /// Windows counts file times since 1601-01-01T00:00:00Z, and cannot
    /// represent times before this, but it's possible to create a SystemTime
    /// that does. This function will error if passed such a SystemTime.
    pub fn from_system_time(time: SystemTime) -> FileTime {
        let epoch = if cfg!(windows) {
            UNIX_EPOCH - Duration::from_secs(11644473600)
        } else {
            UNIX_EPOCH
        };

        time.duration_since(epoch)
            .map(|d| FileTime {
                seconds: d.as_secs() as i64,
                nanos: d.subsec_nanos(),
            })
            .unwrap_or_else(|e| {
                let until_epoch = e.duration();
                let (sec_offset, nanos) = if until_epoch.subsec_nanos() == 0 {
                    (0, 0)
                } else {
                    (-1, 1_000_000_000 - until_epoch.subsec_nanos())
                };

                FileTime {
                    seconds: -1 * until_epoch.as_secs() as i64 + sec_offset,
                    nanos,
                }
            })
            .emulate_second_only_system()
    }

    /// Returns the whole number of seconds represented by this timestamp.
    ///
    /// Note that this value's meaning is **platform specific**. On Unix
    /// platform time stamps are typically relative to January 1, 1970, but on
    /// Windows platforms time stamps are relative to January 1, 1601.
    pub const fn seconds(&self) -> i64 {
        self.seconds
    }

    /// Returns the whole number of seconds represented by this timestamp,
    /// relative to the Unix epoch start of January 1, 1970.
    ///
    /// Note that this does not return the same value as `seconds` for Windows
    /// platforms as seconds are relative to a different date there.
    pub const fn unix_seconds(&self) -> i64 {
        self.seconds - if cfg!(windows) { 11644473600 } else { 0 }
    }

    /// Returns the nanosecond precision of this timestamp.
    ///
    /// The returned value is always less than one billion and represents a
    /// portion of a second forward from the seconds returned by the `seconds`
    /// method.
    pub const fn nanoseconds(&self) -> u32 {
        self.nanos
    }
}

impl fmt::Display for FileTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{:09}s", self.seconds, self.nanos)
    }
}

impl From<SystemTime> for FileTime {
    fn from(time: SystemTime) -> FileTime {
        FileTime::from_system_time(time)
    }
}

impl From<FileTime> for SystemTime {
    fn from(time: FileTime) -> SystemTime {
        let seconds = time.unix_seconds();
        if seconds < 0 {
            SystemTime::UNIX_EPOCH - Duration::from_secs(-seconds as u64)
                + Duration::from_nanos(time.nanoseconds() as u64)
        } else {
            SystemTime::UNIX_EPOCH + Duration::new(seconds as u64, time.nanoseconds())
        }
    }
}

/// Set the last access and modification times for a file on the filesystem.
///
/// This function will set the `atime` and `mtime` metadata fields for a file
/// on the local filesystem, returning any error encountered.
pub fn set_file_times<P>(p: P, atime: FileTime, mtime: FileTime) -> io::Result<()>
where
    P: AsRef<Path>,
{
    imp::open(p.as_ref())?.set_times(
        fs::FileTimes::new()
            .set_accessed(atime.into())
            .set_modified(mtime.into()),
    )
}

/// Set the last access and modification times for a file handle.
///
/// This function will either or both of  the `atime` and `mtime` metadata
/// fields for a file handle , returning any error encountered. If `None` is
/// specified then the time won't be updated. If `None` is specified for both
/// options then no action is taken.
pub fn set_file_handle_times(
    f: &fs::File,
    atime: Option<FileTime>,
    mtime: Option<FileTime>,
) -> io::Result<()> {
    let mut times = fs::FileTimes::new();
    if let Some(t) = atime {
        times = times.set_accessed(t.into());
    }
    if let Some(t) = mtime {
        times = times.set_modified(t.into());
    }
    f.set_times(times)
}

/// Set the last access and modification times for a file on the filesystem.
/// This function does not follow symlink.
///
/// This function will set the `atime` and `mtime` metadata fields for a file
/// on the local filesystem, returning any error encountered.
pub fn set_symlink_file_times<P>(p: P, atime: FileTime, mtime: FileTime) -> io::Result<()>
where
    P: AsRef<Path>,
{
    imp::set_symlink_file_times(p.as_ref(), atime, mtime)
}

/// Set the last modification time for a file on the filesystem.
///
/// This function will set the `mtime` metadata field for a file on the local
/// filesystem, returning any error encountered.
///
/// # Platform support
///
/// Where supported this will attempt to issue just one syscall to update only
/// the `mtime`, but where not supported this may issue one syscall to learn the
/// existing `atime` so only the `mtime` can be configured.
pub fn set_file_mtime<P>(p: P, mtime: FileTime) -> io::Result<()>
where
    P: AsRef<Path>,
{
    imp::open(p.as_ref())?.set_times(fs::FileTimes::new().set_modified(mtime.into()))
}

/// Set the last access time for a file on the filesystem.
///
/// This function will set the `atime` metadata field for a file on the local
/// filesystem, returning any error encountered.
///
/// # Platform support
///
/// Where supported this will attempt to issue just one syscall to update only
/// the `atime`, but where not supported this may issue one syscall to learn the
/// existing `mtime` so only the `atime` can be configured.
pub fn set_file_atime<P>(p: P, atime: FileTime) -> io::Result<()>
where
    P: AsRef<Path>,
{
    imp::open(p.as_ref())?.set_times(fs::FileTimes::new().set_accessed(atime.into()))
}

#[cfg(test)]
mod tests {
    use super::{
        set_file_atime, set_file_handle_times, set_file_mtime, set_file_times,
        set_symlink_file_times, FileTime,
    };
    use std::fs::{self, File};
    use std::io;
    use std::path::Path;
    use std::time::{Duration, UNIX_EPOCH};
    use tempfile::Builder;

    #[cfg(unix)]
    fn make_symlink_file<P, Q>(src: P, dst: Q) -> io::Result<()>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        use std::os::unix::fs::symlink;
        symlink(src, dst)
    }

    #[cfg(windows)]
    fn make_symlink_file<P, Q>(src: P, dst: Q) -> io::Result<()>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        use std::os::windows::fs::symlink_file;
        symlink_file(src, dst)
    }

    #[cfg(unix)]
    fn make_symlink_dir<P, Q>(src: P, dst: Q) -> io::Result<()>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        use std::os::unix::fs::symlink;
        symlink(src, dst)
    }

    #[cfg(windows)]
    fn make_symlink_dir<P, Q>(src: P, dst: Q) -> io::Result<()>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        use std::os::windows::fs::symlink_dir;
        symlink_dir(src, dst)
    }

    #[test]
    #[cfg(windows)]
    fn from_unix_time_test() {
        let time = FileTime::from_unix_time(10, 100_000_000);
        assert_eq!(11644473610, time.seconds);
        assert_eq!(100_000_000, time.nanos);

        let time = FileTime::from_unix_time(-10, 100_000_000);
        assert_eq!(11644473590, time.seconds);
        assert_eq!(100_000_000, time.nanos);

        let time = FileTime::from_unix_time(-12_000_000_000, 0);
        assert_eq!(-355526400, time.seconds);
        assert_eq!(0, time.nanos);
    }

    #[test]
    #[cfg(not(windows))]
    fn from_unix_time_test() {
        let time = FileTime::from_unix_time(10, 100_000_000);
        assert_eq!(10, time.seconds);
        assert_eq!(100_000_000, time.nanos);

        let time = FileTime::from_unix_time(-10, 100_000_000);
        assert_eq!(-10, time.seconds);
        assert_eq!(100_000_000, time.nanos);

        let time = FileTime::from_unix_time(-12_000_000_000, 0);
        assert_eq!(-12_000_000_000, time.seconds);
        assert_eq!(0, time.nanos);
    }

    #[test]
    #[cfg(windows)]
    fn to_from_system_time_test() {
        let systime = UNIX_EPOCH + Duration::from_secs(10);
        let time = FileTime::from_system_time(systime);
        assert_eq!(11644473610, time.seconds);
        assert_eq!(0, time.nanos);
        assert_eq!(systime, time.into());

        let systime = UNIX_EPOCH - Duration::from_secs(10);
        let time = FileTime::from_system_time(systime);
        assert_eq!(11644473590, time.seconds);
        assert_eq!(0, time.nanos);
        assert_eq!(systime, time.into());

        let systime = UNIX_EPOCH - Duration::from_millis(1100);
        let time = FileTime::from_system_time(systime);
        assert_eq!(11644473598, time.seconds);
        assert_eq!(900_000_000, time.nanos);
        assert_eq!(systime, time.into());
    }

    #[test]
    #[cfg(not(windows))]
    fn to_from_system_time_test() {
        let systime = UNIX_EPOCH + Duration::from_secs(10);
        let time = FileTime::from_system_time(systime);
        assert_eq!(10, time.seconds);
        assert_eq!(0, time.nanos);
        assert_eq!(systime, time.into());

        let systime = UNIX_EPOCH - Duration::from_secs(10);
        let time = FileTime::from_system_time(systime);
        assert_eq!(-10, time.seconds);
        assert_eq!(0, time.nanos);
        assert_eq!(systime, time.into());

        let systime = UNIX_EPOCH - Duration::from_millis(1100);
        let time = FileTime::from_system_time(systime);
        assert_eq!(-2, time.seconds);
        assert_eq!(900_000_000, time.nanos);
        assert_eq!(systime, time.into());

        let systime = UNIX_EPOCH - Duration::from_secs(12_000_000);
        let time = FileTime::from_system_time(systime);
        assert_eq!(-12_000_000, time.seconds);
        assert_eq!(0, time.nanos);
        assert_eq!(systime, time.into());
    }

    #[test]
    fn set_file_times_test() -> io::Result<()> {
        let td = Builder::new().prefix("filetime").tempdir()?;
        let path = td.path().join("foo.txt");
        let mut f = File::create(&path)?;

        let metadata = fs::metadata(&path)?;
        let mtime = FileTime::from_last_modification_time(&metadata);
        let atime = FileTime::from_last_access_time(&metadata);
        set_file_times(&path, atime, mtime)?;

        let new_mtime = FileTime::from_unix_time(10_000, 0);
        set_file_times(&path, atime, new_mtime)?;

        let metadata = fs::metadata(&path)?;
        let mtime = FileTime::from_last_modification_time(&metadata);
        assert_eq!(mtime, new_mtime, "modification should be updated");

        // Update just mtime
        let new_mtime = FileTime::from_unix_time(20_000, 0);
        set_file_handle_times(&mut f, None, Some(new_mtime))?;
        let metadata = f.metadata()?;
        let mtime = FileTime::from_last_modification_time(&metadata);
        assert_eq!(mtime, new_mtime, "modification time should be updated");
        let new_atime = FileTime::from_last_access_time(&metadata);
        assert_eq!(atime, new_atime, "accessed time should not be updated");

        // Update just atime
        let new_atime = FileTime::from_unix_time(30_000, 0);
        set_file_handle_times(&mut f, Some(new_atime), None)?;
        let metadata = f.metadata()?;
        let mtime = FileTime::from_last_modification_time(&metadata);
        assert_eq!(mtime, new_mtime, "modification time should not be updated");
        let atime = FileTime::from_last_access_time(&metadata);
        assert_eq!(atime, new_atime, "accessed time should be updated");

        let spath = td.path().join("bar.txt");
        make_symlink_file(&path, &spath)?;
        let metadata = fs::symlink_metadata(&spath)?;
        let smtime = FileTime::from_last_modification_time(&metadata);

        set_file_times(&spath, atime, mtime)?;

        let metadata = fs::metadata(&path)?;
        let cur_mtime = FileTime::from_last_modification_time(&metadata);
        assert_eq!(mtime, cur_mtime);

        let metadata = fs::symlink_metadata(&spath)?;
        let cur_mtime = FileTime::from_last_modification_time(&metadata);
        assert_eq!(smtime, cur_mtime);

        set_file_times(&spath, atime, new_mtime)?;

        let metadata = fs::metadata(&path)?;
        let mtime = FileTime::from_last_modification_time(&metadata);
        assert_eq!(mtime, new_mtime);

        let metadata = fs::symlink_metadata(&spath)?;
        let mtime = FileTime::from_last_modification_time(&metadata);
        assert_eq!(mtime, smtime);
        Ok(())
    }

    #[test]
    fn set_dir_times_test() -> io::Result<()> {
        let td = Builder::new().prefix("filetime").tempdir()?;
        let path = td.path().join("foo");
        fs::create_dir(&path)?;

        let metadata = fs::metadata(&path)?;
        let mtime = FileTime::from_last_modification_time(&metadata);
        let atime = FileTime::from_last_access_time(&metadata);
        set_file_times(&path, atime, mtime)?;

        let new_mtime = FileTime::from_unix_time(10_000, 0);
        set_file_times(&path, atime, new_mtime)?;

        let metadata = fs::metadata(&path)?;
        let mtime = FileTime::from_last_modification_time(&metadata);
        assert_eq!(mtime, new_mtime, "modification should be updated");

        // Update just mtime
        let new_mtime = FileTime::from_unix_time(20_000, 0);
        set_file_mtime(&path, new_mtime)?;
        let metadata = fs::metadata(&path)?;
        let mtime = FileTime::from_last_modification_time(&metadata);
        assert_eq!(mtime, new_mtime, "modification time should be updated");
        let new_atime = FileTime::from_last_access_time(&metadata);
        assert_eq!(atime, new_atime, "accessed time should not be updated");

        // Update just atime
        let new_atime = FileTime::from_unix_time(30_000, 0);
        set_file_atime(&path, new_atime)?;
        let metadata = fs::metadata(&path)?;
        let mtime = FileTime::from_last_modification_time(&metadata);
        assert_eq!(mtime, new_mtime, "modification time should not be updated");
        let atime = FileTime::from_last_access_time(&metadata);
        assert_eq!(atime, new_atime, "accessed time should be updated");

        let spath = td.path().join("bar");
        make_symlink_dir(&path, &spath)?;
        let metadata = fs::symlink_metadata(&spath)?;
        let smtime = FileTime::from_last_modification_time(&metadata);

        set_file_times(&spath, atime, mtime)?;

        let metadata = fs::metadata(&path)?;
        let cur_mtime = FileTime::from_last_modification_time(&metadata);
        assert_eq!(mtime, cur_mtime);

        let metadata = fs::symlink_metadata(&spath)?;
        let cur_mtime = FileTime::from_last_modification_time(&metadata);
        assert_eq!(smtime, cur_mtime);

        set_file_times(&spath, atime, new_mtime)?;

        let metadata = fs::metadata(&path)?;
        let mtime = FileTime::from_last_modification_time(&metadata);
        assert_eq!(mtime, new_mtime);

        let metadata = fs::symlink_metadata(&spath)?;
        let mtime = FileTime::from_last_modification_time(&metadata);
        assert_eq!(mtime, smtime);
        Ok(())
    }

    #[test]
    fn set_file_times_pre_unix_epoch_test() {
        let td = Builder::new().prefix("filetime").tempdir().unwrap();
        let path = td.path().join("foo.txt");
        File::create(&path).unwrap();

        let metadata = fs::metadata(&path).unwrap();
        let mtime = FileTime::from_last_modification_time(&metadata);
        let atime = FileTime::from_last_access_time(&metadata);
        set_file_times(&path, atime, mtime).unwrap();

        let new_mtime = FileTime::from_unix_time(-10_000, 0);
        if cfg!(target_os = "aix") {
            // On AIX, os checks if the unix timestamp is valid.
            let result = set_file_times(&path, atime, new_mtime);
            assert!(result.is_err());
            assert!(result.err().unwrap().kind() == std::io::ErrorKind::InvalidInput);
        } else {
            set_file_times(&path, atime, new_mtime).unwrap();

            let metadata = fs::metadata(&path).unwrap();
            let mtime = FileTime::from_last_modification_time(&metadata);
            assert_eq!(mtime, new_mtime);
        }
    }

    #[test]
    #[cfg(windows)]
    fn set_file_times_pre_windows_epoch_test() {
        let td = Builder::new().prefix("filetime").tempdir().unwrap();
        let path = td.path().join("foo.txt");
        File::create(&path).unwrap();

        let metadata = fs::metadata(&path).unwrap();
        let mtime = FileTime::from_last_modification_time(&metadata);
        let atime = FileTime::from_last_access_time(&metadata);
        set_file_times(&path, atime, mtime).unwrap();
    }

    #[test]
    fn set_symlink_file_times_test() {
        let td = Builder::new().prefix("filetime").tempdir().unwrap();
        let path = td.path().join("foo.txt");
        File::create(&path).unwrap();

        let metadata = fs::metadata(&path).unwrap();
        let mtime = FileTime::from_last_modification_time(&metadata);
        let atime = FileTime::from_last_access_time(&metadata);
        set_symlink_file_times(&path, atime, mtime).unwrap();

        let new_mtime = FileTime::from_unix_time(10_000, 0);
        set_symlink_file_times(&path, atime, new_mtime).unwrap();

        let metadata = fs::metadata(&path).unwrap();
        let mtime = FileTime::from_last_modification_time(&metadata);
        assert_eq!(mtime, new_mtime);

        let spath = td.path().join("bar.txt");
        make_symlink_file(&path, &spath).unwrap();

        let metadata = fs::symlink_metadata(&spath).unwrap();
        let smtime = FileTime::from_last_modification_time(&metadata);
        let satime = FileTime::from_last_access_time(&metadata);
        set_symlink_file_times(&spath, smtime, satime).unwrap();

        let metadata = fs::metadata(&path).unwrap();
        let mtime = FileTime::from_last_modification_time(&metadata);
        assert_eq!(mtime, new_mtime);

        let new_smtime = FileTime::from_unix_time(20_000, 0);
        set_symlink_file_times(&spath, atime, new_smtime).unwrap();

        let metadata = fs::metadata(&spath).unwrap();
        let mtime = FileTime::from_last_modification_time(&metadata);
        assert_eq!(mtime, new_mtime);

        let metadata = fs::symlink_metadata(&spath).unwrap();
        let mtime = FileTime::from_last_modification_time(&metadata);
        assert_eq!(mtime, new_smtime);
    }

    #[test]
    fn set_symlink_dir_times_test() {
        let td = Builder::new().prefix("filetime").tempdir().unwrap();
        let path = td.path().join("foo");
        fs::create_dir(&path).unwrap();

        let metadata = fs::metadata(&path).unwrap();
        let mtime = FileTime::from_last_modification_time(&metadata);
        let atime = FileTime::from_last_access_time(&metadata);
        set_symlink_file_times(&path, atime, mtime).unwrap();

        let new_mtime = FileTime::from_unix_time(10_000, 0);
        set_symlink_file_times(&path, atime, new_mtime).unwrap();

        let metadata = fs::metadata(&path).unwrap();
        let mtime = FileTime::from_last_modification_time(&metadata);
        assert_eq!(mtime, new_mtime);

        let spath = td.path().join("bar");
        make_symlink_dir(&path, &spath).unwrap();

        let metadata = fs::symlink_metadata(&spath).unwrap();
        let smtime = FileTime::from_last_modification_time(&metadata);
        let satime = FileTime::from_last_access_time(&metadata);
        set_symlink_file_times(&spath, smtime, satime).unwrap();

        let metadata = fs::metadata(&path).unwrap();
        let mtime = FileTime::from_last_modification_time(&metadata);
        assert_eq!(mtime, new_mtime);

        let new_smtime = FileTime::from_unix_time(20_000, 0);
        set_symlink_file_times(&spath, atime, new_smtime).unwrap();

        let metadata = fs::metadata(&spath).unwrap();
        let mtime = FileTime::from_last_modification_time(&metadata);
        assert_eq!(mtime, new_mtime);

        let metadata = fs::symlink_metadata(&spath).unwrap();
        let mtime = FileTime::from_last_modification_time(&metadata);
        assert_eq!(mtime, new_smtime);
    }

    #[test]
    fn set_single_time_test() {
        use super::{set_file_atime, set_file_mtime};

        let td = Builder::new().prefix("filetime").tempdir().unwrap();
        let path = td.path().join("foo.txt");
        File::create(&path).unwrap();

        let metadata = fs::metadata(&path).unwrap();
        let mtime = FileTime::from_last_modification_time(&metadata);
        let atime = FileTime::from_last_access_time(&metadata);
        set_file_times(&path, atime, mtime).unwrap();

        let new_mtime = FileTime::from_unix_time(10_000, 0);
        set_file_mtime(&path, new_mtime).unwrap();

        let metadata = fs::metadata(&path).unwrap();
        let mtime = FileTime::from_last_modification_time(&metadata);
        assert_eq!(mtime, new_mtime, "modification time should be updated");
        assert_eq!(
            atime,
            FileTime::from_last_access_time(&metadata),
            "access time should not be updated",
        );

        let new_atime = FileTime::from_unix_time(20_000, 0);
        set_file_atime(&path, new_atime).unwrap();

        let metadata = fs::metadata(&path).unwrap();
        let atime = FileTime::from_last_access_time(&metadata);
        assert_eq!(atime, new_atime, "access time should be updated");
        assert_eq!(
            mtime,
            FileTime::from_last_modification_time(&metadata),
            "modification time should not be updated"
        );
    }
}
