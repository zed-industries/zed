use std::fs::File;
use std::io::{Error, Result};
use std::mem;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::io::{AsRawHandle, FromRawHandle};
use std::path::Path;
use std::ptr;

use winapi::shared::minwindef::{BOOL, DWORD};
use winapi::shared::winerror::ERROR_LOCK_VIOLATION;
use winapi::um::fileapi::{FILE_ALLOCATION_INFO, FILE_STANDARD_INFO, GetDiskFreeSpaceW};
use winapi::um::fileapi::{GetVolumePathNameW, LockFileEx, UnlockFile, SetFileInformationByHandle};
use winapi::um::handleapi::DuplicateHandle;
use winapi::um::minwinbase::{FileAllocationInfo, FileStandardInfo};
use winapi::um::minwinbase::{LOCKFILE_FAIL_IMMEDIATELY, LOCKFILE_EXCLUSIVE_LOCK};
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::winbase::GetFileInformationByHandleEx;
use winapi::um::winnt::DUPLICATE_SAME_ACCESS;

use FsStats;

pub fn duplicate(file: &File) -> Result<File> {
    unsafe {
        let mut handle = ptr::null_mut();
        let current_process = GetCurrentProcess();
        let ret = DuplicateHandle(current_process,
                                  file.as_raw_handle(),
                                  current_process,
                                  &mut handle,
                                  0,
                                  true as BOOL,
                                  DUPLICATE_SAME_ACCESS);
        if ret == 0 {
            Err(Error::last_os_error())
        } else {
            Ok(File::from_raw_handle(handle))
        }
    }
}

pub fn allocated_size(file: &File) -> Result<u64> {
    unsafe {
        let mut info: FILE_STANDARD_INFO = mem::zeroed();

        let ret = GetFileInformationByHandleEx(
            file.as_raw_handle(),
            FileStandardInfo,
            &mut info as *mut _ as *mut _,
            mem::size_of::<FILE_STANDARD_INFO>() as DWORD);

        if ret == 0 {
            Err(Error::last_os_error())
        } else {
            Ok(*info.AllocationSize.QuadPart() as u64)
        }
    }
}

pub fn allocate(file: &File, len: u64) -> Result<()> {
    if try!(allocated_size(file)) < len {
        unsafe {
            let mut info: FILE_ALLOCATION_INFO = mem::zeroed();
            *info.AllocationSize.QuadPart_mut() = len as i64;
            let ret = SetFileInformationByHandle(
                file.as_raw_handle(),
                FileAllocationInfo,
                &mut info as *mut _ as *mut _,
                mem::size_of::<FILE_ALLOCATION_INFO>() as DWORD);
            if ret == 0 {
                return Err(Error::last_os_error());
            }
        }
    }
    if try!(file.metadata()).len() < len {
        file.set_len(len)
    } else {
        Ok(())
    }
}

pub fn lock_shared(file: &File) -> Result<()> {
    lock_file(file, 0)
}

pub fn lock_exclusive(file: &File) -> Result<()> {
    lock_file(file, LOCKFILE_EXCLUSIVE_LOCK)
}

pub fn try_lock_shared(file: &File) -> Result<()> {
    lock_file(file, LOCKFILE_FAIL_IMMEDIATELY)
}

pub fn try_lock_exclusive(file: &File) -> Result<()> {
    lock_file(file, LOCKFILE_EXCLUSIVE_LOCK | LOCKFILE_FAIL_IMMEDIATELY)
}

pub fn unlock(file: &File) -> Result<()> {
    unsafe {
        let ret = UnlockFile(file.as_raw_handle(), 0, 0, !0, !0);
        if ret == 0 { Err(Error::last_os_error()) } else { Ok(()) }
    }
}

pub fn lock_error() -> Error {
    Error::from_raw_os_error(ERROR_LOCK_VIOLATION as i32)
}

fn lock_file(file: &File, flags: DWORD) -> Result<()> {
    unsafe {
        let mut overlapped = mem::zeroed();
        let ret = LockFileEx(file.as_raw_handle(), flags, 0, !0, !0, &mut overlapped);
        if ret == 0 { Err(Error::last_os_error()) } else { Ok(()) }
    }
}

fn volume_path(path: &Path, volume_path: &mut [u16]) -> Result<()> {
    let path_utf8: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
    unsafe {
        let ret = GetVolumePathNameW(path_utf8.as_ptr(),
                                     volume_path.as_mut_ptr(),
                                     volume_path.len() as DWORD);
        if ret == 0 { Err(Error::last_os_error()) } else { Ok(())
        }
    }
}

pub fn statvfs(path: &Path) -> Result<FsStats> {
    let root_path: &mut [u16] = &mut [0; 261];
    try!(volume_path(path, root_path));
    unsafe {

        let mut sectors_per_cluster = 0;
        let mut bytes_per_sector = 0;
        let mut number_of_free_clusters = 0;
        let mut total_number_of_clusters = 0;
        let ret = GetDiskFreeSpaceW(root_path.as_ptr(),
                                    &mut sectors_per_cluster,
                                    &mut bytes_per_sector,
                                    &mut number_of_free_clusters,
                                    &mut total_number_of_clusters);
        if ret == 0 {
            Err(Error::last_os_error())
        } else {
            let bytes_per_cluster = sectors_per_cluster as u64 * bytes_per_sector as u64;
            let free_space = bytes_per_cluster * number_of_free_clusters as u64;
            let total_space = bytes_per_cluster * total_number_of_clusters as u64;
            Ok(FsStats {
                free_space: free_space,
                available_space: free_space,
                total_space: total_space,
                allocation_granularity: bytes_per_cluster,
            })
        }
    }
}

#[cfg(test)]
mod test {

    extern crate tempdir;

    use std::fs;
    use std::os::windows::io::AsRawHandle;

    use {FileExt, lock_contended_error};

    /// The duplicate method returns a file with a new file handle.
    #[test]
    fn duplicate_new_handle() {
        let tempdir = tempdir::TempDir::new("fs2").unwrap();
        let path = tempdir.path().join("fs2");
        let file1 = fs::OpenOptions::new().write(true).create(true).open(&path).unwrap();
        let file2 = file1.duplicate().unwrap();
        assert!(file1.as_raw_handle() != file2.as_raw_handle());
    }

    /// A duplicated file handle does not have access to the original handle's locks.
    #[test]
    fn lock_duplicate_handle_independence() {
        let tempdir = tempdir::TempDir::new("fs2").unwrap();
        let path = tempdir.path().join("fs2");
        let file1 = fs::OpenOptions::new().read(true).write(true).create(true).open(&path).unwrap();
        let file2 = file1.duplicate().unwrap();

        // Locking the original file handle will block the duplicate file handle from opening a lock.
        file1.lock_shared().unwrap();
        assert_eq!(file2.try_lock_exclusive().unwrap_err().raw_os_error(),
                   lock_contended_error().raw_os_error());

        // Once the original file handle is unlocked, the duplicate handle can proceed with a lock.
        file1.unlock().unwrap();
        file2.lock_exclusive().unwrap();
    }

    /// A file handle may not be exclusively locked multiple times, or exclusively locked and then
    /// shared locked.
    #[test]
    fn lock_non_reentrant() {
        let tempdir = tempdir::TempDir::new("fs2").unwrap();
        let path = tempdir.path().join("fs2");
        let file = fs::OpenOptions::new().read(true).write(true).create(true).open(&path).unwrap();

        // Multiple exclusive locks fails.
        file.lock_exclusive().unwrap();
        assert_eq!(file.try_lock_exclusive().unwrap_err().raw_os_error(),
                   lock_contended_error().raw_os_error());
        file.unlock().unwrap();

        // Shared then Exclusive locks fails.
        file.lock_shared().unwrap();
        assert_eq!(file.try_lock_exclusive().unwrap_err().raw_os_error(),
                   lock_contended_error().raw_os_error());
    }

    /// A file handle can hold an exclusive lock and any number of shared locks, all of which must
    /// be unlocked independently.
    #[test]
    fn lock_layering() {
        let tempdir = tempdir::TempDir::new("fs2").unwrap();
        let path = tempdir.path().join("fs2");
        let file = fs::OpenOptions::new().read(true).write(true).create(true).open(&path).unwrap();

        // Open two shared locks on the file, and then try and fail to open an exclusive lock.
        file.lock_exclusive().unwrap();
        file.lock_shared().unwrap();
        file.lock_shared().unwrap();
        assert_eq!(file.try_lock_exclusive().unwrap_err().raw_os_error(),
                   lock_contended_error().raw_os_error());

        // Pop one of the shared locks and try again.
        file.unlock().unwrap();
        assert_eq!(file.try_lock_exclusive().unwrap_err().raw_os_error(),
                   lock_contended_error().raw_os_error());

        // Pop the second shared lock and try again.
        file.unlock().unwrap();
        assert_eq!(file.try_lock_exclusive().unwrap_err().raw_os_error(),
                   lock_contended_error().raw_os_error());

        // Pop the exclusive lock and finally succeed.
        file.unlock().unwrap();
        file.lock_exclusive().unwrap();
    }

    /// A file handle with multiple open locks will have all locks closed on drop.
    #[test]
    fn lock_layering_cleanup() {
        let tempdir = tempdir::TempDir::new("fs2").unwrap();
        let path = tempdir.path().join("fs2");
        let file1 = fs::OpenOptions::new().read(true).write(true).create(true).open(&path).unwrap();
        let file2 = fs::OpenOptions::new().read(true).write(true).create(true).open(&path).unwrap();

        // Open two shared locks on the file, and then try and fail to open an exclusive lock.
        file1.lock_shared().unwrap();
        assert_eq!(file2.try_lock_exclusive().unwrap_err().raw_os_error(),
                   lock_contended_error().raw_os_error());

        drop(file1);
        file2.lock_exclusive().unwrap();
    }

    /// A file handle's locks will not be released until the original handle and all of its
    /// duplicates have been closed. This on really smells like a bug in Windows.
    #[test]
    fn lock_duplicate_cleanup() {
        let tempdir = tempdir::TempDir::new("fs2").unwrap();
        let path = tempdir.path().join("fs2");
        let file1 = fs::OpenOptions::new().read(true).write(true).create(true).open(&path).unwrap();
        let file2 = file1.duplicate().unwrap();

        // Open a lock on the original handle, then close it.
        file1.lock_shared().unwrap();
        drop(file1);

        // Attempting to create a lock on the file with the duplicate handle will fail.
        assert_eq!(file2.try_lock_exclusive().unwrap_err().raw_os_error(),
                   lock_contended_error().raw_os_error());
    }
}
