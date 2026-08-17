#![allow(unsafe_code)]

use std::mem;
use std::os::windows::io::HandleOrInvalid;
use std::ptr::null_mut;
use windows_sys::Wdk::Foundation::OBJECT_ATTRIBUTES;
use windows_sys::Wdk::Storage::FileSystem::{
    NtCreateFile, FILE_CREATE, FILE_DELETE_ON_CLOSE, FILE_NON_DIRECTORY_FILE,
    FILE_NO_INTERMEDIATE_BUFFERING, FILE_OPEN, FILE_OPEN_FOR_BACKUP_INTENT, FILE_OPEN_IF,
    FILE_OPEN_REPARSE_POINT, FILE_OVERWRITE, FILE_OVERWRITE_IF, FILE_RANDOM_ACCESS,
    FILE_SEQUENTIAL_ONLY, FILE_SYNCHRONOUS_IO_NONALERT, FILE_WRITE_THROUGH,
};
use windows_sys::Win32::Foundation::{
    RtlNtStatusToDosError, SetLastError, ERROR_ALREADY_EXISTS, ERROR_FILE_EXISTS,
    ERROR_INVALID_NAME, ERROR_INVALID_PARAMETER, ERROR_NOT_SUPPORTED, GENERIC_ALL, GENERIC_READ,
    GENERIC_WRITE, HANDLE, INVALID_HANDLE_VALUE, STATUS_OBJECT_NAME_COLLISION, STATUS_PENDING,
    STATUS_SUCCESS, SUCCESS, UNICODE_STRING,
};
use windows_sys::Win32::Security::{
    SECURITY_ATTRIBUTES, SECURITY_DYNAMIC_TRACKING, SECURITY_QUALITY_OF_SERVICE,
    SECURITY_STATIC_TRACKING,
};
use windows_sys::Win32::Storage::FileSystem::{
    CREATE_ALWAYS, CREATE_NEW, DELETE, FILE_ATTRIBUTE_ARCHIVE, FILE_ATTRIBUTE_COMPRESSED,
    FILE_ATTRIBUTE_DEVICE, FILE_ATTRIBUTE_DIRECTORY, FILE_ATTRIBUTE_EA, FILE_ATTRIBUTE_ENCRYPTED,
    FILE_ATTRIBUTE_HIDDEN, FILE_ATTRIBUTE_INTEGRITY_STREAM, FILE_ATTRIBUTE_NORMAL,
    FILE_ATTRIBUTE_NOT_CONTENT_INDEXED, FILE_ATTRIBUTE_NO_SCRUB_DATA, FILE_ATTRIBUTE_OFFLINE,
    FILE_ATTRIBUTE_PINNED, FILE_ATTRIBUTE_READONLY, FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS,
    FILE_ATTRIBUTE_RECALL_ON_OPEN, FILE_ATTRIBUTE_REPARSE_POINT, FILE_ATTRIBUTE_SPARSE_FILE,
    FILE_ATTRIBUTE_SYSTEM, FILE_ATTRIBUTE_TEMPORARY, FILE_ATTRIBUTE_UNPINNED,
    FILE_ATTRIBUTE_VIRTUAL, FILE_CREATION_DISPOSITION, FILE_FLAGS_AND_ATTRIBUTES,
    FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_DELETE_ON_CLOSE, FILE_FLAG_NO_BUFFERING,
    FILE_FLAG_OPEN_NO_RECALL, FILE_FLAG_OPEN_REPARSE_POINT, FILE_FLAG_OVERLAPPED,
    FILE_FLAG_POSIX_SEMANTICS, FILE_FLAG_RANDOM_ACCESS, FILE_FLAG_SEQUENTIAL_SCAN,
    FILE_FLAG_SESSION_AWARE, FILE_FLAG_WRITE_THROUGH, FILE_READ_ATTRIBUTES, FILE_SHARE_MODE,
    OPEN_ALWAYS, OPEN_EXISTING, SECURITY_CONTEXT_TRACKING, SECURITY_EFFECTIVE_ONLY,
    SECURITY_SQOS_PRESENT, SYNCHRONIZE, TRUNCATE_EXISTING,
};
use windows_sys::Win32::System::Kernel::{OBJ_CASE_INSENSITIVE, OBJ_INHERIT};
use windows_sys::Win32::System::WindowsProgramming::{
    FILE_OPENED, FILE_OPEN_NO_RECALL, FILE_OPEN_REMOTE_INSTANCE, FILE_OVERWRITTEN,
};
use windows_sys::Win32::System::IO::IO_STATUS_BLOCK;

// All currently known `FILE_ATTRIBUTE_*` constants, according to
// windows-sys' documentation.
const FILE_ATTRIBUTE_VALID_FLAGS: FILE_FLAGS_AND_ATTRIBUTES = FILE_ATTRIBUTE_EA
    | FILE_ATTRIBUTE_DEVICE
    | FILE_ATTRIBUTE_HIDDEN
    | FILE_ATTRIBUTE_NORMAL
    | FILE_ATTRIBUTE_PINNED
    | FILE_ATTRIBUTE_SYSTEM
    | FILE_ATTRIBUTE_ARCHIVE
    | FILE_ATTRIBUTE_OFFLINE
    | FILE_ATTRIBUTE_VIRTUAL
    | FILE_ATTRIBUTE_READONLY
    | FILE_ATTRIBUTE_UNPINNED
    | FILE_ATTRIBUTE_DIRECTORY
    | FILE_ATTRIBUTE_ENCRYPTED
    | FILE_ATTRIBUTE_TEMPORARY
    | FILE_ATTRIBUTE_COMPRESSED
    | FILE_ATTRIBUTE_SPARSE_FILE
    | FILE_ATTRIBUTE_NO_SCRUB_DATA
    | FILE_ATTRIBUTE_REPARSE_POINT
    | FILE_ATTRIBUTE_RECALL_ON_OPEN
    | FILE_ATTRIBUTE_INTEGRITY_STREAM
    | FILE_ATTRIBUTE_NOT_CONTENT_INDEXED
    | FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS;

/// Like Windows' `CreateFileW`, but takes a `dir` argument to use as the
/// root directory.
///
/// Also, the `lpfilename` is a Rust slice instead of a C-style NUL-terminated
/// array, because that's what our callers have and it's closer to what
/// `NtCreatePath` takes.
#[allow(non_snake_case)]
pub unsafe fn CreateFileAtW(
    dir: HANDLE,
    lpfilename: &[u16],
    dwdesiredaccess: u32,
    dwsharemode: FILE_SHARE_MODE,
    lpsecurityattributes: *const SECURITY_ATTRIBUTES,
    dwcreationdisposition: FILE_CREATION_DISPOSITION,
    dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES,
    htemplatefile: HANDLE,
) -> HandleOrInvalid {
    // Absolute paths are not yet implemented here.
    //
    // It seems like `NtCreatePath` needs the apparently NT-internal `\??\`
    // prefix prepended to absolute paths. It's possible it needs other
    // path transforms as well. `RtlDosPathNameToNtPathName_U` may be a
    // function that does these things, though it's not available in
    // windows-sys and not documented, though one can find
    // [unofficial blog posts], though even they say things like "I`m
    // sorry that I cannot give more details on these functions".
    //
    // [unofficial blog posts]: https://mecanik.dev/en/posts/convert-dos-and-nt-paths-using-rtl-functions/
    assert!(dir != 0 as HANDLE);

    // Extended attributes are not implemented yet.
    if htemplatefile != 0 as HANDLE {
        SetLastError(ERROR_NOT_SUPPORTED);
        return HandleOrInvalid::from_raw_handle(INVALID_HANDLE_VALUE as _);
    }

    // Convert `dwcreationdisposition` to the `createdisposition` argument
    // to `NtCreateFile`. Do this before converting `lpfilename` so that
    // we can return early on failure.
    let createdisposition = match dwcreationdisposition {
        CREATE_NEW => FILE_CREATE,
        CREATE_ALWAYS => FILE_OVERWRITE_IF,
        OPEN_EXISTING => FILE_OPEN,
        OPEN_ALWAYS => FILE_OPEN_IF,
        TRUNCATE_EXISTING => FILE_OVERWRITE,
        _ => {
            SetLastError(ERROR_INVALID_PARAMETER);
            return HandleOrInvalid::from_raw_handle(INVALID_HANDLE_VALUE as _);
        }
    };

    // Convert `lpfilename` to a `UNICODE_STRING`.
    let byte_length = lpfilename.len() * mem::size_of::<u16>();
    let length: u16 = match byte_length.try_into() {
        Ok(length) => length,
        Err(_) => {
            SetLastError(ERROR_INVALID_NAME);
            return HandleOrInvalid::from_raw_handle(INVALID_HANDLE_VALUE as _);
        }
    };
    let mut unicode_string = UNICODE_STRING {
        Buffer: lpfilename.as_ptr() as *mut u16,
        Length: length,
        MaximumLength: length,
    };

    let mut handle = INVALID_HANDLE_VALUE;

    // Convert `dwdesiredaccess` and `dwflagsandattributes` to the
    // `desiredaccess` argument to `NtCreateFile`.
    let mut desiredaccess = dwdesiredaccess | SYNCHRONIZE | FILE_READ_ATTRIBUTES;
    if dwflagsandattributes & FILE_FLAG_DELETE_ON_CLOSE != 0 {
        desiredaccess |= DELETE;
    }

    // Compute `objectattributes`' `Attributes` field. Case-insensitive is
    // the expected behavior on Windows.
    let mut attributes = 0;
    if dwflagsandattributes & FILE_FLAG_POSIX_SEMANTICS != 0 {
        attributes |= OBJ_CASE_INSENSITIVE as u32;
    };
    if !lpsecurityattributes.is_null() && (*lpsecurityattributes).bInheritHandle != 0 {
        attributes |= OBJ_INHERIT as u32;
    }

    // Compute the `objectattributes` argument to `NtCreateFile`.
    let mut objectattributes = mem::zeroed::<OBJECT_ATTRIBUTES>();
    objectattributes.Length = mem::size_of::<OBJECT_ATTRIBUTES>() as _;
    objectattributes.RootDirectory = dir;
    objectattributes.ObjectName = &mut unicode_string;
    objectattributes.Attributes = attributes;
    if !lpsecurityattributes.is_null() {
        objectattributes.SecurityDescriptor = (*lpsecurityattributes).lpSecurityDescriptor;
    }

    // If needed, set `objectattributes`' `SecurityQualityOfService` field.
    let mut qos;
    if dwflagsandattributes & SECURITY_SQOS_PRESENT != 0 {
        qos = mem::zeroed::<SECURITY_QUALITY_OF_SERVICE>();
        qos.Length = mem::size_of::<SECURITY_QUALITY_OF_SERVICE>() as _;
        qos.ImpersonationLevel = ((dwflagsandattributes >> 16) & 0x3) as _;
        qos.ContextTrackingMode = if dwflagsandattributes & SECURITY_CONTEXT_TRACKING != 0 {
            SECURITY_DYNAMIC_TRACKING
        } else {
            SECURITY_STATIC_TRACKING
        };
        qos.EffectiveOnly = ((dwflagsandattributes & SECURITY_EFFECTIVE_ONLY) != 0) as _;

        objectattributes.SecurityQualityOfService =
            (&mut qos as *mut SECURITY_QUALITY_OF_SERVICE).cast();
    }

    let mut iostatusblock = mem::zeroed::<IO_STATUS_BLOCK>();
    iostatusblock.Anonymous.Status = STATUS_PENDING;

    // Compute the `fileattributes` argument to `NtCreateFile`. Mask off
    // unrecognized flags.
    let mut fileattributes = dwflagsandattributes & FILE_ATTRIBUTE_VALID_FLAGS;
    if fileattributes == 0 {
        fileattributes = FILE_ATTRIBUTE_NORMAL;
    }

    // Compute the `createoptions` argument to `NtCreateFile`.
    let mut createoptions = 0;
    if dwflagsandattributes & FILE_FLAG_BACKUP_SEMANTICS == 0 {
        createoptions |= FILE_NON_DIRECTORY_FILE;
    } else {
        if dwdesiredaccess & GENERIC_ALL != 0 {
            createoptions |= FILE_OPEN_FOR_BACKUP_INTENT | FILE_OPEN_REMOTE_INSTANCE;
        } else {
            if dwdesiredaccess & GENERIC_READ != 0 {
                createoptions |= FILE_OPEN_FOR_BACKUP_INTENT;
            }
            if dwdesiredaccess & GENERIC_WRITE != 0 {
                createoptions |= FILE_OPEN_REMOTE_INSTANCE;
            }
        }
    }
    if dwflagsandattributes & FILE_FLAG_DELETE_ON_CLOSE != 0 {
        createoptions |= FILE_DELETE_ON_CLOSE;
    }
    if dwflagsandattributes & FILE_FLAG_NO_BUFFERING != 0 {
        createoptions |= FILE_NO_INTERMEDIATE_BUFFERING;
    }
    if dwflagsandattributes & FILE_FLAG_OPEN_NO_RECALL != 0 {
        createoptions |= FILE_OPEN_NO_RECALL;
    }
    if dwflagsandattributes & FILE_FLAG_OPEN_REPARSE_POINT != 0 {
        createoptions |= FILE_OPEN_REPARSE_POINT;
    }
    if dwflagsandattributes & FILE_FLAG_OVERLAPPED == 0 {
        createoptions |= FILE_SYNCHRONOUS_IO_NONALERT;
    }
    // FILE_FLAG_POSIX_SEMANTICS is handled above.
    if dwflagsandattributes & FILE_FLAG_RANDOM_ACCESS != 0 {
        createoptions |= FILE_RANDOM_ACCESS;
    }
    if dwflagsandattributes & FILE_FLAG_SESSION_AWARE != 0 {
        // TODO: How should we handle FILE_FLAG_SESSION_AWARE?
        SetLastError(ERROR_NOT_SUPPORTED);
        return HandleOrInvalid::from_raw_handle(INVALID_HANDLE_VALUE as _);
    }
    if dwflagsandattributes & FILE_FLAG_SEQUENTIAL_SCAN != 0 {
        createoptions |= FILE_SEQUENTIAL_ONLY;
    }
    if dwflagsandattributes & FILE_FLAG_WRITE_THROUGH != 0 {
        createoptions |= FILE_WRITE_THROUGH;
    }

    // Ok, we have what we need to call `NtCreateFile` now!
    let status = NtCreateFile(
        &mut handle,
        desiredaccess,
        &mut objectattributes,
        &mut iostatusblock,
        null_mut(),
        fileattributes,
        dwsharemode,
        createdisposition,
        createoptions,
        null_mut(),
        0,
    );

    // Check for errors.
    if status != STATUS_SUCCESS {
        handle = INVALID_HANDLE_VALUE;
        if status == STATUS_OBJECT_NAME_COLLISION {
            SetLastError(ERROR_FILE_EXISTS);
        } else {
            SetLastError(RtlNtStatusToDosError(status));
        }
    } else if (dwcreationdisposition == CREATE_ALWAYS
        && iostatusblock.Information == FILE_OVERWRITTEN as _)
        || (dwcreationdisposition == OPEN_ALWAYS && iostatusblock.Information == FILE_OPENED as _)
    {
        // Set `ERROR_ALREADY_EXISTS` according to the table for
        // `dwCreationDisposition` in the [`CreateFileW` docs].
        //
        // [`CreateFileW` docs]: https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew
        SetLastError(ERROR_ALREADY_EXISTS);
    } else {
        // Otherwise indicate that we succeeded.
        SetLastError(SUCCESS);
    }

    HandleOrInvalid::from_raw_handle(handle as _)
}
