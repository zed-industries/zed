#![macro_use]

use libc::{c_char, c_int, c_void, free};
use std::error::Error as StdError;
use std::ffi::CStr;
use std::{fmt, str};

/// ALSA error
///
/// Most ALSA functions can return a negative error code.
/// If so, then that error code is wrapped into this `Error` struct.
/// An Error is also returned in case ALSA returns a string that
/// cannot be translated into Rust's UTF-8 strings.
#[derive(Clone, PartialEq, Copy)]
pub struct Error(&'static str, i32);

pub type Result<T> = ::std::result::Result<T, Error>;

macro_rules! acheck {
    ($f: ident ( $($x: expr),* ) ) => {{
        let r = unsafe { alsa::$f( $($x),* ) };
        if r < 0 { Err(Error::new(stringify!($f), -r as ::libc::c_int)) }
        else { Ok(r) }
    }}
}

pub fn from_const<'a>(func: &'static str, s: *const c_char) -> Result<&'a str> {
    if s.is_null() {
        return Err(invalid_str(func));
    };
    let cc = unsafe { CStr::from_ptr(s) };
    str::from_utf8(cc.to_bytes()).map_err(|_| invalid_str(func))
}

pub fn from_alloc(func: &'static str, s: *mut c_char) -> Result<String> {
    if s.is_null() {
        return Err(invalid_str(func));
    };
    let c = unsafe { CStr::from_ptr(s) };
    let ss = str::from_utf8(c.to_bytes())
        .map_err(|_| {
            unsafe {
                free(s as *mut c_void);
            }
            invalid_str(func)
        })?
        .to_string();
    unsafe {
        free(s as *mut c_void);
    }
    Ok(ss)
}

pub fn from_code(func: &'static str, r: c_int) -> Result<c_int> {
    if r < 0 {
        Err(Error::new(func, r))
    } else {
        Ok(r)
    }
}

impl Error {
    pub fn new(func: &'static str, res: c_int) -> Error {
        Self(func, res)
    }

    pub fn last(func: &'static str) -> Error {
        Self(
            func,
            std::io::Error::last_os_error()
                .raw_os_error()
                .unwrap_or_default(),
        )
    }

    pub fn unsupported(func: &'static str) -> Error {
        Error(func, libc::ENOTSUP)
    }

    /// The function which failed.
    pub fn func(&self) -> &'static str {
        self.0
    }

    /// Underlying error
    ///
    /// Match this against the re-export of `nix::Error` in this crate, not against a specific version
    /// of the nix crate. The nix crate version might be updated with minor updates of this library.
    pub fn errno(&self) -> i32 {
        self.1
    }
}

pub fn invalid_str(func: &'static str) -> Error {
    Error(func, libc::EILSEQ)
}

impl StdError for Error {
    fn description(&self) -> &str {
        "ALSA error"
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ALSA function '{}' failed with error '{} ({})'",
            self.0,
            desc(self.1),
            self.1,
        )
    }
}

/// See <https://github.com/nix-rust/nix/blob/197f55b3ccbce3273bf6ce119d1a8541b5df5d66/src/errno.rs#L198>
///
/// Note this doesn't include the total set of possible errno variants, but they
/// can easily be added in the future for better error messages
fn desc(errno: i32) -> &'static str {
    match errno {
        libc::EPERM => "Operation not permitted",
        libc::ENOENT => "No such file or directory",
        libc::ESRCH => "No such process",
        libc::EINTR => "Interrupted system call",
        libc::EIO => "I/O error",
        libc::ENXIO => "No such device or address",
        libc::E2BIG => "Argument list too long",
        libc::ENOEXEC => "Exec format error",
        libc::EBADF => "Bad file number",
        libc::ECHILD => "No child processes",
        libc::EAGAIN => "Try again",
        libc::ENOMEM => "Out of memory",
        libc::EACCES => "Permission denied",
        libc::EFAULT => "Bad address",
        libc::ENOTBLK => "Block device required",
        libc::EBUSY => "Device or resource busy",
        libc::EEXIST => "File exists",
        libc::EXDEV => "Cross-device link",
        libc::ENODEV => "No such device",
        libc::ENOTDIR => "Not a directory",
        libc::EISDIR => "Is a directory",
        libc::EINVAL => "Invalid argument",
        libc::ENFILE => "File table overflow",
        libc::EMFILE => "Too many open files",
        libc::ENOTTY => "Not a typewriter",
        libc::ETXTBSY => "Text file busy",
        libc::EFBIG => "File too large",
        libc::ENOSPC => "No space left on device",
        libc::ESPIPE => "Illegal seek",
        libc::EROFS => "Read-only file system",
        libc::EMLINK => "Too many links",
        libc::EPIPE => "Broken pipe",
        libc::EDOM => "Math argument out of domain of func",
        libc::ERANGE => "Math result not representable",
        libc::EDEADLK => "Resource deadlock would occur",
        libc::ENAMETOOLONG => "File name too long",
        libc::ENOLCK => "No record locks available",
        libc::ENOSYS => "Function not implemented",
        libc::ENOTEMPTY => "Directory not empty",
        libc::ELOOP => "Too many symbolic links encountered",
        libc::ENOMSG => "No message of desired type",
        libc::EIDRM => "Identifier removed",
        libc::EINPROGRESS => "Operation now in progress",
        libc::EALREADY => "Operation already in progress",
        libc::ENOTSOCK => "Socket operation on non-socket",
        libc::EDESTADDRREQ => "Destination address required",
        libc::EMSGSIZE => "Message too long",
        libc::EPROTOTYPE => "Protocol wrong type for socket",
        libc::ENOPROTOOPT => "Protocol not available",
        libc::EPROTONOSUPPORT => "Protocol not supported",
        libc::ESOCKTNOSUPPORT => "Socket type not supported",
        libc::EPFNOSUPPORT => "Protocol family not supported",
        libc::EAFNOSUPPORT => "Address family not supported by protocol",
        libc::EADDRINUSE => "Address already in use",
        libc::EADDRNOTAVAIL => "Cannot assign requested address",
        libc::ENETDOWN => "Network is down",
        libc::ENETUNREACH => "Network is unreachable",
        libc::ENETRESET => "Network dropped connection because of reset",
        libc::ECONNABORTED => "Software caused connection abort",
        libc::ECONNRESET => "Connection reset by peer",
        libc::ENOBUFS => "No buffer space available",
        libc::EISCONN => "Transport endpoint is already connected",
        libc::ENOTCONN => "Transport endpoint is not connected",
        libc::ESHUTDOWN => "Cannot send after transport endpoint shutdown",
        libc::ETOOMANYREFS => "Too many references: cannot splice",
        libc::ETIMEDOUT => "Connection timed out",
        libc::ECONNREFUSED => "Connection refused",
        libc::EHOSTDOWN => "Host is down",
        libc::EHOSTUNREACH => "No route to host",
        libc::ENOTSUP => "Operation not supported",
        _ => "Unknown errno",
    }
}

impl From<Error> for fmt::Error {
    fn from(_: Error) -> fmt::Error {
        fmt::Error
    }
}

#[test]
fn broken_pcm_name() {
    use std::ffi::CString;
    let e = crate::PCM::open(
        &*CString::new("this_PCM_does_not_exist").unwrap(),
        crate::Direction::Playback,
        false,
    )
    .err()
    .unwrap();
    assert_eq!(e.func(), "snd_pcm_open");
    assert_eq!(e.errno(), libc::ENOENT);
}
