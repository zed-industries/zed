use std::{io, mem};

use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::Foundation::{GetLastError, FILETIME, NO_ERROR};
use windows_sys::Win32::Storage::FileSystem::{
    GetFileInformationByHandle, GetFileType, BY_HANDLE_FILE_INFORMATION,
    FILE_ATTRIBUTE_HIDDEN,
};

use crate::AsHandleRef;

/// Return various pieces of information about a file.
///
/// This includes information such as a file's size, unique identifier and
/// time related fields.
///
/// This corresponds to calling [`GetFileInformationByHandle`].
///
/// [`GetFileInformationByHandle`]: https://docs.microsoft.com/en-us/windows/desktop/api/fileapi/nf-fileapi-getfileinformationbyhandle
pub fn information<H: AsHandleRef>(h: H) -> io::Result<Information> {
    unsafe {
        let mut info: BY_HANDLE_FILE_INFORMATION = mem::zeroed();
        let rc = GetFileInformationByHandle(h.as_raw() as HANDLE, &mut info);
        if rc == 0 {
            return Err(io::Error::last_os_error());
        };
        Ok(Information(info))
    }
}

/// Returns the file type of the given handle.
///
/// If there was a problem querying the file type, then an error is returned.
///
/// This corresponds to calling [`GetFileType`].
///
/// [`GetFileType`]: https://docs.microsoft.com/en-us/windows/desktop/api/fileapi/nf-fileapi-getfiletype
pub fn typ<H: AsHandleRef>(h: H) -> io::Result<Type> {
    unsafe {
        let rc = GetFileType(h.as_raw() as HANDLE);
        if rc == 0 && GetLastError() != NO_ERROR {
            return Err(io::Error::last_os_error());
        }
        Ok(Type(rc))
    }
}

/// Returns true if and only if the given file attributes contain the
/// `FILE_ATTRIBUTE_HIDDEN` attribute.
pub fn is_hidden(file_attributes: u64) -> bool {
    file_attributes & (FILE_ATTRIBUTE_HIDDEN as u64) > 0
}

/// Represents file information such as creation time, file size, etc.
///
/// This wraps a [`BY_HANDLE_FILE_INFORMATION`].
///
/// [`BY_HANDLE_FILE_INFORMATION`]: https://docs.microsoft.com/en-us/windows/desktop/api/fileapi/ns-fileapi-_by_handle_file_information
#[derive(Clone)]
pub struct Information(BY_HANDLE_FILE_INFORMATION);

impl Information {
    /// Returns file attributes.
    ///
    /// This corresponds to `dwFileAttributes`.
    pub fn file_attributes(&self) -> u64 {
        self.0.dwFileAttributes as u64
    }

    /// Returns true if and only if this file information has the
    /// `FILE_ATTRIBUTE_HIDDEN` attribute.
    pub fn is_hidden(&self) -> bool {
        is_hidden(self.file_attributes())
    }

    /// Return the creation time, if one exists.
    ///
    /// This corresponds to `ftCreationTime`.
    pub fn creation_time(&self) -> Option<u64> {
        filetime_to_u64(self.0.ftCreationTime)
    }

    /// Return the last access time, if one exists.
    ///
    /// This corresponds to `ftLastAccessTime`.
    pub fn last_access_time(&self) -> Option<u64> {
        filetime_to_u64(self.0.ftLastAccessTime)
    }

    /// Return the last write time, if one exists.
    ///
    /// This corresponds to `ftLastWriteTime`.
    pub fn last_write_time(&self) -> Option<u64> {
        filetime_to_u64(self.0.ftLastWriteTime)
    }

    /// Return the serial number of the volume that the file is on.
    ///
    /// This corresponds to `dwVolumeSerialNumber`.
    pub fn volume_serial_number(&self) -> u64 {
        self.0.dwVolumeSerialNumber as u64
    }

    /// Return the file size, in bytes.
    ///
    /// This corresponds to `nFileSizeHigh` and `nFileSizeLow`.
    pub fn file_size(&self) -> u64 {
        ((self.0.nFileSizeHigh as u64) << 32) | (self.0.nFileSizeLow as u64)
    }

    /// Return the number of links to this file.
    ///
    /// This corresponds to `nNumberOfLinks`.
    pub fn number_of_links(&self) -> u64 {
        self.0.nNumberOfLinks as u64
    }

    /// Return the index of this file. The index of a file is a purpotedly
    /// unique identifier for a file within a particular volume.
    pub fn file_index(&self) -> u64 {
        ((self.0.nFileIndexHigh as u64) << 32) | (self.0.nFileIndexLow as u64)
    }
}

/// Represents a Windows file type.
///
/// This wraps the result of [`GetFileType`].
///
/// [`GetFileType`]: https://docs.microsoft.com/en-us/windows/desktop/api/fileapi/nf-fileapi-getfiletype
#[derive(Clone)]
pub struct Type(u32);

impl Type {
    /// Returns true if this type represents a character file, which is
    /// typically an LPT device or a console.
    pub fn is_char(&self) -> bool {
        self.0 == ::windows_sys::Win32::Storage::FileSystem::FILE_TYPE_CHAR
    }

    /// Returns true if this type represents a disk file.
    pub fn is_disk(&self) -> bool {
        self.0 == ::windows_sys::Win32::Storage::FileSystem::FILE_TYPE_DISK
    }

    /// Returns true if this type represents a sock, named pipe or an
    /// anonymous pipe.
    pub fn is_pipe(&self) -> bool {
        self.0 == ::windows_sys::Win32::Storage::FileSystem::FILE_TYPE_PIPE
    }

    /// Returns true if this type is not known.
    ///
    /// Note that this never corresponds to a failure.
    pub fn is_unknown(&self) -> bool {
        self.0 == ::windows_sys::Win32::Storage::FileSystem::FILE_TYPE_UNKNOWN
    }
}

fn filetime_to_u64(t: FILETIME) -> Option<u64> {
    let v = ((t.dwHighDateTime as u64) << 32) | (t.dwLowDateTime as u64);
    if v == 0 {
        None
    } else {
        Some(v)
    }
}
