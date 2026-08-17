// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! ApiSet Contract for api-ms-win-core-file-l1
use shared::minwindef::{
    BOOL, DWORD, FILETIME, LPCVOID, LPDWORD, LPFILETIME, LPVOID, PDWORD, PUCHAR, UCHAR, UINT,
    ULONG, WORD,
};
use um::minwinbase::{
    FILE_INFO_BY_HANDLE_CLASS, FINDEX_INFO_LEVELS, FINDEX_SEARCH_OPS, GET_FILEEX_INFO_LEVELS,
    LPOVERLAPPED, LPOVERLAPPED_COMPLETION_ROUTINE, LPSECURITY_ATTRIBUTES, LPWIN32_FIND_DATAA,
    LPWIN32_FIND_DATAW
};
use um::winnt::{
    BOOLEAN, CCHAR, FILE_ID_128, FILE_SEGMENT_ELEMENT, HANDLE, LARGE_INTEGER, LONG, LONGLONG,
    LPCSTR, LPCWSTR, LPSTR, LPWCH, LPWSTR, PLARGE_INTEGER, PLONG, PULARGE_INTEGER, PWSTR,
    ULONGLONG, WCHAR,
};
pub const CREATE_NEW: DWORD = 1;
pub const CREATE_ALWAYS: DWORD = 2;
pub const OPEN_EXISTING: DWORD = 3;
pub const OPEN_ALWAYS: DWORD = 4;
pub const TRUNCATE_EXISTING: DWORD = 5;
pub const INVALID_FILE_SIZE: DWORD = 0xFFFFFFFF;
pub const INVALID_SET_FILE_POINTER: DWORD = 0xFFFFFFFF;
pub const INVALID_FILE_ATTRIBUTES: DWORD = 0xFFFFFFFF;
STRUCT!{struct WIN32_FILE_ATTRIBUTE_DATA {
    dwFileAttributes: DWORD,
    ftCreationTime: FILETIME,
    ftLastAccessTime: FILETIME,
    ftLastWriteTime: FILETIME,
    nFileSizeHigh: DWORD,
    nFileSizeLow: DWORD,
}}
pub type LPWIN32_FILE_ATTRIBUTE_DATA = *mut WIN32_FILE_ATTRIBUTE_DATA;
STRUCT!{struct BY_HANDLE_FILE_INFORMATION {
    dwFileAttributes: DWORD,
    ftCreationTime: FILETIME,
    ftLastAccessTime: FILETIME,
    ftLastWriteTime: FILETIME,
    dwVolumeSerialNumber: DWORD,
    nFileSizeHigh: DWORD,
    nFileSizeLow: DWORD,
    nNumberOfLinks: DWORD,
    nFileIndexHigh: DWORD,
    nFileIndexLow: DWORD,
}}
pub type PBY_HANDLE_FILE_INFORMATION = *mut BY_HANDLE_FILE_INFORMATION;
pub type LPBY_HANDLE_FILE_INFORMATION = *mut BY_HANDLE_FILE_INFORMATION;
STRUCT!{struct CREATEFILE2_EXTENDED_PARAMETERS {
    dwSize: DWORD,
    dwFileAttributes: DWORD,
    dwFileFlags: DWORD,
    dwSecurityQosFlags: DWORD,
    lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
    hTemplateFile: HANDLE,
}}
pub type PCREATEFILE2_EXTENDED_PARAMETERS = *mut CREATEFILE2_EXTENDED_PARAMETERS;
pub type LPCREATEFILE2_EXTENDED_PARAMETERS = *mut CREATEFILE2_EXTENDED_PARAMETERS;
ENUM!{enum PRIORITY_HINT {
    IoPriorityHintVeryLow = 0,
    IoPriorityHintLow = 1,
    IoPriorityHintNormal = 2,
    MaximumIoPriorityHintType = 3,
}}
STRUCT!{struct FILE_BASIC_INFO {
    CreationTime: LARGE_INTEGER,
    LastAccessTime: LARGE_INTEGER,
    LastWriteTime: LARGE_INTEGER,
    ChangeTime: LARGE_INTEGER,
    FileAttributes: DWORD,
}}
STRUCT!{struct FILE_STANDARD_INFO {
    AllocationSize: LARGE_INTEGER,
    EndOfFile: LARGE_INTEGER,
    NumberOfLinks: DWORD,
    DeletePending: BOOLEAN,
    Directory: BOOLEAN,
}}
STRUCT!{struct FILE_NAME_INFO {
    FileNameLength: DWORD,
    FileName: [WCHAR; 1],
}}
STRUCT!{struct FILE_RENAME_INFO {
    ReplaceIfExists: BOOL,
    RootDirectory: HANDLE,
    FileNameLength: DWORD,
    FileName: [WCHAR; 1],
}}
STRUCT!{struct FILE_DISPOSITION_INFO {
    DeleteFile: BOOLEAN,
}}
STRUCT!{struct FILE_ALLOCATION_INFO {
    AllocationSize: LARGE_INTEGER,
}}
STRUCT!{struct FILE_END_OF_FILE_INFO {
    EndOfFile: LARGE_INTEGER,
}}
STRUCT!{struct FILE_STREAM_INFO {
    NextEntryOffset: DWORD,
    StreamNameLength: DWORD,
    StreamSize: LARGE_INTEGER,
    StreamAllocationSize: LARGE_INTEGER,
    StreamName: [WCHAR; 1],
}}
STRUCT!{struct FILE_COMPRESSION_INFO {
    CompressedFileSize: LARGE_INTEGER,
    CompressionFormat: WORD,
    CompressionUnitShift: UCHAR,
    ChunkShift: UCHAR,
    ClusterShift: UCHAR,
    Reserved: [UCHAR; 3],
}}
STRUCT!{struct FILE_ATTRIBUTE_TAG_INFO {
    NextEntryOffset: DWORD,
    ReparseTag: DWORD,
}}
STRUCT!{struct FILE_ID_BOTH_DIR_INFO {
    NextEntryOffset: DWORD,
    FileIndex: DWORD,
    CreationTime: LARGE_INTEGER,
    LastAccessTime: LARGE_INTEGER,
    LastWriteTime: LARGE_INTEGER,
    ChangeTime: LARGE_INTEGER,
    EndOfFile: LARGE_INTEGER,
    AllocationSize: LARGE_INTEGER,
    FileAttributes: DWORD,
    FileNameLength: DWORD,
    EaSize: DWORD,
    ShortNameLength: CCHAR,
    ShortName: [WCHAR; 12],
    FileId: LARGE_INTEGER,
    FileName: [WCHAR; 1],
}}
STRUCT!{struct FILE_IO_PRIORITY_HINT_INFO {
    PriorityHint: PRIORITY_HINT,
}}
STRUCT!{struct FILE_FULL_DIR_INFO {
    NextEntryOffset: ULONG,
    FileIndex: ULONG,
    CreationTime: LARGE_INTEGER,
    LastAccessTime: LARGE_INTEGER,
    LastWriteTime: LARGE_INTEGER,
    ChangeTime: LARGE_INTEGER,
    EndOfFile: LARGE_INTEGER,
    AllocationSize: LARGE_INTEGER,
    FileAttributes: ULONG,
    FileNameLength: ULONG,
    EaSize: ULONG,
    FileName: [WCHAR; 1],
}}
STRUCT!{struct FILE_STORAGE_INFO {
    LogicalBytesPerSector: ULONG,
    PhysicalBytesPerSectorForAtomicity: ULONG,
    PhysicalBytesPerSectorForPerformance: ULONG,
    FileSystemEffectivePhysicalBytesPerSectorForAtomicity: ULONG,
    Flags: ULONG,
    ByteOffsetForSectorAlignment: ULONG,
    ByteOffsetForPartitionAlignment: ULONG,
}}
STRUCT!{struct FILE_ALIGNMENT_INFO {
    AlignmentRequirement: ULONG,
}}
STRUCT!{struct FILE_ID_INFO {
    VolumeSerialNumber: ULONGLONG,
    FileId: FILE_ID_128,
}}
extern "system" {
    pub fn CompareFileTime(
        lpFileTime1: *const FILETIME,
        lpFileTime2: *const FILETIME,
    ) -> LONG;
    pub fn CreateDirectoryA(
        lpPathName: LPCSTR,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
    ) -> BOOL;
    pub fn CreateDirectoryW(
        lpPathName: LPCWSTR,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
    ) -> BOOL;
    pub fn CreateFileA(
        lpFileName: LPCSTR,
        dwDesiredAccess: DWORD,
        dwShareMode: DWORD,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
        dwCreationDisposition: DWORD,
        dwFlagsAndAttributes: DWORD,
        hTemplateFile: HANDLE,
    ) -> HANDLE;
    pub fn CreateFileW(
        lpFileName: LPCWSTR,
        dwDesiredAccess: DWORD,
        dwShareMode: DWORD,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
        dwCreationDisposition: DWORD,
        dwFlagsAndAttributes: DWORD,
        hTemplateFile: HANDLE,
    ) -> HANDLE;
    pub fn DefineDosDeviceW(
        dwFlags: DWORD,
        lpDeviceName: LPCWSTR,
        lpTargetPath: LPCWSTR,
    ) -> BOOL;
    pub fn DeleteFileA(
        lpFileName: LPCSTR,
    ) -> BOOL;
    pub fn DeleteFileW(
        lpFileName: LPCWSTR,
    ) -> BOOL;
    pub fn DeleteVolumeMountPointW(
        lpszVolumeMountPoint: LPCWSTR,
    ) -> BOOL;
    pub fn FileTimeToLocalFileTime(
        lpFileTime: *const FILETIME,
        lpLocalFileTime: LPFILETIME,
    ) -> BOOL;
    pub fn FindClose(
        hFindFile: HANDLE,
    ) -> BOOL;
    pub fn FindCloseChangeNotification(
        hChangeHandle: HANDLE,
    ) -> BOOL;
    pub fn FindFirstChangeNotificationA(
        lpPathName: LPCSTR,
        bWatchSubtree: BOOL,
        dwNotifyFilter: DWORD,
    ) -> HANDLE;
    pub fn FindFirstChangeNotificationW(
        lpPathName: LPCWSTR,
        bWatchSubtree: BOOL,
        dwNotifyFilter: DWORD,
    ) -> HANDLE;
    pub fn FindFirstFileA(
        lpFileName: LPCSTR,
        lpFindFileData: LPWIN32_FIND_DATAA,
    ) -> HANDLE;
    pub fn FindFirstFileW(
        lpFileName: LPCWSTR,
        lpFindFileData: LPWIN32_FIND_DATAW,
    ) -> HANDLE;
    pub fn FindFirstFileExA(
        lpFileName: LPCSTR,
        fInfoLevelId: FINDEX_INFO_LEVELS,
        lpFindFileData: LPVOID,
        fSearchOp: FINDEX_SEARCH_OPS,
        lpSearchFilter: LPVOID,
        dwAdditionalFlags: DWORD,
    ) -> HANDLE;
    pub fn FindFirstFileExW(
        lpFileName: LPCWSTR,
        fInfoLevelId: FINDEX_INFO_LEVELS,
        lpFindFileData: LPVOID,
        fSearchOp: FINDEX_SEARCH_OPS,
        lpSearchFilter: LPVOID,
        dwAdditionalFlags: DWORD,
    ) -> HANDLE;
    pub fn FindFirstVolumeW(
        lpszVolumeName: LPWSTR,
        cchBufferLength: DWORD,
    ) -> HANDLE;
    pub fn FindNextChangeNotification(
        hChangeHandle: HANDLE,
    ) -> BOOL;
    pub fn FindNextFileA(
        hFindFile: HANDLE,
        lpFindFileData: LPWIN32_FIND_DATAA,
    ) -> BOOL;
    pub fn FindNextFileW(
        hFindFile: HANDLE,
        lpFindFileData: LPWIN32_FIND_DATAW,
    ) -> BOOL;
    pub fn FindNextVolumeW(
        hFindVolume: HANDLE,
        lpszVolumeName: LPWSTR,
        cchBufferLength: DWORD,
    ) -> BOOL;
    pub fn FindVolumeClose(
        hFindVolume: HANDLE,
    ) -> BOOL;
    pub fn FlushFileBuffers(
        hFile: HANDLE,
    ) -> BOOL;
    pub fn GetDiskFreeSpaceA(
        lpRootPathName: LPCSTR,
        lpSectorsPerCluster: LPDWORD,
        lpBytesPerSector: LPDWORD,
        lpNumberOfFreeClusters: LPDWORD,
        lpTotalNumberOfClusters: LPDWORD,
    ) -> BOOL;
    pub fn GetDiskFreeSpaceW(
        lpRootPathName: LPCWSTR,
        lpSectorsPerCluster: LPDWORD,
        lpBytesPerSector: LPDWORD,
        lpNumberOfFreeClusters: LPDWORD,
        lpTotalNumberOfClusters: LPDWORD,
    ) -> BOOL;
    pub fn GetDiskFreeSpaceExA(
        lpDirectoryName: LPCSTR,
        lpFreeBytesAvailableToCaller: PULARGE_INTEGER,
        lpTotalNumberOfBytes: PULARGE_INTEGER,
        lpTotalNumberOfFreeBytes: PULARGE_INTEGER,
    ) -> BOOL;
    pub fn GetDiskFreeSpaceExW(
        lpDirectoryName: LPCWSTR,
        lpFreeBytesAvailableToCaller: PULARGE_INTEGER,
        lpTotalNumberOfBytes: PULARGE_INTEGER,
        lpTotalNumberOfFreeBytes: PULARGE_INTEGER,
    ) -> BOOL;
    pub fn GetDriveTypeA(
        lpRootPathName: LPCSTR,
    ) -> UINT;
    pub fn GetDriveTypeW(
        lpRootPathName: LPCWSTR,
    ) -> UINT;
    pub fn GetFileAttributesA(
        lpFileName: LPCSTR,
    ) -> DWORD;
    pub fn GetFileAttributesW(
        lpFileName: LPCWSTR,
    ) -> DWORD;
    pub fn GetFileAttributesExA(
        lpFileName: LPCSTR,
        fInfoLevelId: GET_FILEEX_INFO_LEVELS,
        lpFileInformation: LPVOID,
    ) -> BOOL;
    pub fn GetFileAttributesExW(
        lpFileName: LPCWSTR,
        fInfoLevelId: GET_FILEEX_INFO_LEVELS,
        lpFileInformation: LPVOID,
    ) -> BOOL;
    pub fn GetFileInformationByHandle(
        hFile: HANDLE,
        lpFileInformation: LPBY_HANDLE_FILE_INFORMATION,
    ) -> BOOL;
    pub fn GetFileSize(
        hFile: HANDLE,
        lpFileSizeHigh: LPDWORD,
    ) -> DWORD;
    pub fn GetFileSizeEx(
        hFile: HANDLE,
        lpFileSize: PLARGE_INTEGER,
    ) -> BOOL;
    pub fn GetFileType(
        hFile: HANDLE,
    ) -> DWORD;
    pub fn GetFinalPathNameByHandleA(
        hFile: HANDLE,
        lpszFilePath: LPSTR,
        cchFilePath: DWORD,
        dwFlags: DWORD,
    ) -> DWORD;
    pub fn GetFinalPathNameByHandleW(
        hFile: HANDLE,
        lpszFilePath: LPWSTR,
        cchFilePath: DWORD,
        dwFlags: DWORD,
    ) -> DWORD;
    pub fn GetFileTime(
        hFile: HANDLE,
        lpCreationTime: LPFILETIME,
        lpLastAccessTime: LPFILETIME,
        lpLastWriteTime: LPFILETIME,
    ) -> BOOL;
    pub fn GetFullPathNameW(
        lpFileName: LPCWSTR,
        nBufferLength: DWORD,
        lpBuffer: LPWSTR,
        lpFilePart: *mut LPWSTR,
    ) -> DWORD;
    pub fn GetFullPathNameA(
        lpFileName: LPCSTR,
        nBufferLength: DWORD,
        lpBuffer: LPSTR,
        lpFilePart: *mut LPSTR,
    ) -> DWORD;
    pub fn GetLogicalDrives() -> DWORD;
    pub fn GetLogicalDriveStringsW(
        nBufferLength: DWORD,
        lpBuffer: LPWSTR,
    ) -> DWORD;
    pub fn GetLongPathNameA(
        lpszShortPath: LPCSTR,
        lpszLongPath: LPSTR,
        cchBuffer: DWORD,
    ) -> DWORD;
    pub fn GetLongPathNameW(
        lpszShortPath: LPCWSTR,
        lpszLongPath: LPWSTR,
        cchBuffer: DWORD,
    ) -> DWORD;
    pub fn GetShortPathNameW(
        lpszLongPath: LPCWSTR,
        lpszShortPath: LPWSTR,
        cchBuffer: DWORD,
    ) -> DWORD;
    pub fn GetTempFileNameW(
        lpPathName: LPCWSTR,
        lpPrefixString: LPCWSTR,
        uUnique: UINT,
        lpTempFileName: LPWSTR,
    ) -> UINT;
    pub fn GetVolumeInformationByHandleW(
        hFile: HANDLE,
        lpVolumeNameBuffer: LPWSTR,
        nVolumeNameSize: DWORD,
        lpVolumeSerialNumber: LPDWORD,
        lpMaximumComponentLength: LPDWORD,
        lpFileSystemFlags: LPDWORD,
        lpFileSystemNameBuffer: LPWSTR,
        nFileSystemNameSize: DWORD,
    ) -> BOOL;
    pub fn GetVolumeInformationW(
        lpRootPathName: LPCWSTR,
        lpVolumeNameBuffer: LPWSTR,
        nVolumeNameSize: DWORD,
        lpVolumeSerialNumber: LPDWORD,
        lpMaximumComponentLength: LPDWORD,
        lpFileSystemFlags: LPDWORD,
        lpFileSystemNameBuffer: LPWSTR,
        nFileSystemNameSize: DWORD,
    ) -> BOOL;
    pub fn GetVolumePathNameW(
        lpszFileName: LPCWSTR,
        lpszVolumePathName: LPWSTR,
        cchBufferLength: DWORD,
    ) -> BOOL;
    pub fn LocalFileTimeToFileTime(
        lpLocalFileTime: *const FILETIME,
        lpFileTime: LPFILETIME,
    ) -> BOOL;
    pub fn LockFile(
        hFile: HANDLE,
        dwFileOffsetLow: DWORD,
        dwFileOffsetHigh: DWORD,
        nNumberOfBytesToLockLow: DWORD,
        nNumberOfBytesToLockHigh: DWORD,
    ) -> BOOL;
    pub fn LockFileEx(
        hFile: HANDLE,
        dwFlags: DWORD,
        dwReserved: DWORD,
        nNumberOfBytesToLockLow: DWORD,
        nNumberOfBytesToLockHigh: DWORD,
        lpOverlapped: LPOVERLAPPED,
    ) -> BOOL;
    pub fn QueryDosDeviceW(
        lpDeviceName: LPCWSTR,
        lpTargetPath: LPWSTR,
        ucchMax: DWORD,
    ) -> DWORD;
    pub fn ReadFile(
        hFile: HANDLE,
        lpBuffer: LPVOID,
        nNumberOfBytesToRead: DWORD,
        lpNumberOfBytesRead: LPDWORD,
        lpOverlapped: LPOVERLAPPED,
    ) -> BOOL;
    pub fn ReadFileEx(
        hFile: HANDLE,
        lpBuffer: LPVOID,
        nNumberOfBytesToRead: DWORD,
        lpOverlapped: LPOVERLAPPED,
        lpCompletionRoutine: LPOVERLAPPED_COMPLETION_ROUTINE,
    ) -> BOOL;
    pub fn ReadFileScatter(
        hFile: HANDLE,
        aSegmentArray: *mut FILE_SEGMENT_ELEMENT,
        nNumberOfBytesToRead: DWORD,
        lpReserved: LPDWORD,
        lpOverlapped: LPOVERLAPPED,
    ) -> BOOL;
    pub fn RemoveDirectoryA(
        lpPathName: LPCSTR,
    ) -> BOOL;
    pub fn RemoveDirectoryW(
        lpPathName: LPCWSTR,
    ) -> BOOL;
    pub fn SetEndOfFile(
        hFile: HANDLE,
    ) -> BOOL;
    pub fn SetFileAttributesA(
        lpFileName: LPCSTR,
        dwFileAttributes: DWORD,
    ) -> BOOL;
    pub fn SetFileAttributesW(
        lpFileName: LPCWSTR,
        dwFileAttributes: DWORD,
    ) -> BOOL;
    pub fn SetFileInformationByHandle(
        hFile: HANDLE,
        FileInformationClass: FILE_INFO_BY_HANDLE_CLASS,
        lpFileInformation: LPVOID,
        dwBufferSize: DWORD,
    ) -> BOOL;
    pub fn SetFilePointer(
        hFile: HANDLE,
        lDistanceToMove: LONG,
        lpDistanceToMoveHigh: PLONG,
        dwMoveMethod: DWORD,
    ) -> DWORD;
    pub fn SetFilePointerEx(
        hFile: HANDLE,
        liDistanceToMove: LARGE_INTEGER,
        lpNewFilePointer: PLARGE_INTEGER,
        dwMoveMethod: DWORD,
    ) -> BOOL;
    pub fn SetFileTime(
        hFile: HANDLE,
        lpCreationTime: *const FILETIME,
        lpLastAccessTime: *const FILETIME,
        lpLastWriteTime: *const FILETIME,
    ) -> BOOL;
    pub fn SetFileValidData(
        hFile: HANDLE,
        ValidDataLength: LONGLONG,
    ) -> BOOL;
    pub fn UnlockFile(
        hFile: HANDLE,
        dwFileOffsetLow: DWORD,
        dwFileOffsetHigh: DWORD,
        nNumberOfBytesToUnlockLow: DWORD,
        nNumberOfBytesToUnlockHigh: DWORD,
    ) -> BOOL;
    pub fn UnlockFileEx(
        hFile: HANDLE,
        dwReserved: DWORD,
        nNumberOfBytesToUnlockLow: DWORD,
        nNumberOfBytesToUnlockHigh: DWORD,
        lpOverlapped: LPOVERLAPPED,
    ) -> BOOL;
    pub fn WriteFile(
        hFile: HANDLE,
        lpBuffer: LPCVOID,
        nNumberOfBytesToWrite: DWORD,
        lpNumberOfBytesWritten: LPDWORD,
        lpOverlapped: LPOVERLAPPED,
    ) -> BOOL;
    pub fn WriteFileEx(
        hFile: HANDLE,
        lpBuffer: LPCVOID,
        nNumberOfBytesToWrite: DWORD,
        lpOverlapped: LPOVERLAPPED,
        lpCompletionRoutine: LPOVERLAPPED_COMPLETION_ROUTINE,
    ) -> BOOL;
    pub fn WriteFileGather(
        hFile: HANDLE,
        aSegmentArray: *mut FILE_SEGMENT_ELEMENT,
        nNumberOfBytesToWrite: DWORD,
        lpReserved: LPDWORD,
        lpOverlapped: LPOVERLAPPED,
    ) -> BOOL;
    pub fn GetTempPathW(
        nBufferLength: DWORD,
        lpBuffer: LPWSTR,
    ) -> DWORD;
    pub fn GetVolumeNameForVolumeMountPointW(
        lpszVolumeMountPoint: LPCWSTR,
        lpszVolumeName: LPWSTR,
        cchBufferLength: DWORD,
    ) -> BOOL;
    pub fn GetVolumePathNamesForVolumeNameW(
        lpszVolumeName: LPCWSTR,
        lpszVolumePathNames: LPWCH,
        cchBufferLength: DWORD,
        lpcchReturnLength: PDWORD,
    ) -> BOOL;
    pub fn CreateFile2(
        lpFileName: LPCWSTR,
        dwDesiredAccess: DWORD,
        dwShareMode: DWORD,
        dwCreationDisposition: DWORD,
        pCreateExParams: LPCREATEFILE2_EXTENDED_PARAMETERS,
    ) -> HANDLE;
    pub fn SetFileIoOverlappedRange(
        FileHandle: HANDLE,
        OverlappedRangeStart: PUCHAR,
        Length: ULONG,
    ) -> BOOL;
    pub fn GetCompressedFileSizeA(
        lpFileName: LPCSTR,
        lpFileSizeHigh: LPDWORD,
    ) -> DWORD;
    pub fn GetCompressedFileSizeW(
        lpFileName: LPCWSTR,
        lpFileSizeHigh: LPDWORD,
    ) -> DWORD;
}
ENUM!{enum STREAM_INFO_LEVELS {
    FindStreamInfoStandard,
    FindStreamInfoMaxInfoLevel,
}}
extern "system" {
    pub fn FindFirstStreamW(
        lpFileName: LPCWSTR,
        InfoLevel: STREAM_INFO_LEVELS,
        lpFindStreamData: LPVOID,
        dwFlags: DWORD,
    ) -> HANDLE;
    pub fn FindNextStreamW(
        hFindStream: HANDLE,
        lpFindStreamData: LPVOID,
    ) -> BOOL;
    pub fn AreFileApisANSI() -> BOOL;
    pub fn GetTempPathA(
        nBufferLength: DWORD,
        lpBuffer: LPSTR,
    ) -> DWORD;
    pub fn FindFirstFileNameW(
        lpFileName: LPCWSTR,
        dwFlags: DWORD,
        StringLength: LPDWORD,
        LinkName: PWSTR,
    ) -> HANDLE;
    pub fn FindNextFileNameW(
        hFindStream: HANDLE,
        StringLength: LPDWORD,
        LinkName: PWSTR,
    ) -> BOOL;
    pub fn GetVolumeInformationA(
        lpRootPathName: LPCSTR,
        lpVolumeNameBuffer: LPSTR,
        nVolumeNameSize: DWORD,
        lpVolumeSerialNumber: LPDWORD,
        lpMaximumComponentLength: LPDWORD,
        lpFileSystemFlags: LPDWORD,
        lpFileSystemNameBuffer: LPSTR,
        nFileSystemNameSize: DWORD,
    ) -> BOOL;
    pub fn GetTempFileNameA(
        lpPathName: LPCSTR,
        lpPrefixString: LPCSTR,
        uUnique: UINT,
        lpTempFileName: LPSTR,
    ) -> UINT;
    pub fn SetFileApisToOEM();
    pub fn SetFileApisToANSI();
}
