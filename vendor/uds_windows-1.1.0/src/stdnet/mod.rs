use std::ascii;
use std::fmt;
use std::io;
use std::mem;
use std::os::raw::{c_char, c_int};
use std::path::Path;

use winapi::shared::ws2def::SOCKADDR;
use winapi::um::winsock2::WSAGetLastError;

mod ext;
mod net;
mod socket;

mod c {
    use std::ffi::CStr;
    use std::fmt;
    use winapi::{
        self,
        shared::{ntdef::CHAR, ws2def::ADDRESS_FAMILY},
    };

    pub const AF_UNIX: ADDRESS_FAMILY = winapi::shared::ws2def::AF_UNIX as _;

    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct sockaddr_un {
        pub sun_family: ADDRESS_FAMILY,
        pub sun_path: [CHAR; 108],
    }

    impl fmt::Debug for sockaddr_un {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            let path = unsafe { CStr::from_ptr(&self.sun_path as *const _).to_str() };
            fmt.debug_struct("sockaddr_un")
                .field("sun_family", &self.sun_family)
                .field("sun_path", &path.unwrap_or("???"))
                .finish()
        }
    }
}

fn sun_path_offset(addr: &c::sockaddr_un) -> usize {
    // Work with an actual instance of the type since using a null pointer is UB
    let base = addr as *const _ as usize;
    let path = &addr.sun_path as *const _ as usize;
    path - base
}

pub unsafe fn sockaddr_un(path: &Path) -> io::Result<(c::sockaddr_un, c_int)> {
    let mut addr: c::sockaddr_un = mem::zeroed();
    addr.sun_family = c::AF_UNIX;

    // Winsock2 expects 'sun_path' to be a Win32 UTF-8 file system path
    let bytes = path.to_str().map(|s| s.as_bytes()).ok_or(io::Error::new(
        io::ErrorKind::InvalidInput,
        "path contains invalid characters",
    ))?;

    if bytes.contains(&0) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "paths may not contain interior null bytes",
        ));
    }

    if bytes.len() >= addr.sun_path.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "path must be shorter than SUN_LEN",
        ));
    }
    for (dst, src) in addr.sun_path.iter_mut().zip(bytes.iter()) {
        *dst = *src as c_char;
    }
    // null byte for pathname addresses is already there because we zeroed the
    // struct

    let mut len = sun_path_offset(&addr) + bytes.len();
    match bytes.first() {
        Some(&0) | None => {}
        Some(_) => len += 1,
    }
    Ok((addr, len as c_int))
}

/// Returns the last error from the Windows socket interface.
fn last_error() -> io::Error {
    io::Error::from_raw_os_error(unsafe { WSAGetLastError() })
}

pub trait IsMinusOne {
    fn is_minus_one(&self) -> bool;
}

macro_rules! impl_is_minus_one {
    ($($t:ident)*) => ($(impl IsMinusOne for $t {
        fn is_minus_one(&self) -> bool {
            *self == -1
        }
    })*)
}

impl_is_minus_one! { i8 i16 i32 i64 isize }

/// Checks if the signed integer is the Windows constant `SOCKET_ERROR` (-1)
/// and if so, returns the last error from the Windows socket interface. This
/// function must be called before another call to the socket API is made.
pub fn cvt<T: IsMinusOne>(t: T) -> io::Result<T> {
    if t.is_minus_one() {
        Err(last_error())
    } else {
        Ok(t)
    }
}

enum AddressKind<'a> {
    Unnamed,
    Pathname(&'a Path),
    Abstract(&'a [u8]),
}

/// An address associated with a Unix socket
///
/// # Examples
///
/// ```no_run
/// use uds_windows::UnixListener;
///
/// let l = UnixListener::bind("/tmp/sock").unwrap();
/// let addr = l.local_addr().expect("Couldn't get local address");
/// ```
#[derive(Copy, Clone)]
pub struct SocketAddr {
    addr: c::sockaddr_un,
    len: c_int,
}

impl SocketAddr {
    fn new<F>(f: F) -> io::Result<SocketAddr>
    where
        F: FnOnce(*mut SOCKADDR, *mut c_int) -> c_int,
    {
        unsafe {
            let mut addr: c::sockaddr_un = mem::zeroed();
            let mut len = mem::size_of::<c::sockaddr_un>() as c_int;
            cvt(f(&mut addr as *mut _ as *mut _, &mut len))?;
            SocketAddr::from_parts(addr, len)
        }
    }

    fn from_parts(addr: c::sockaddr_un, mut len: c_int) -> io::Result<SocketAddr> {
        if len == 0 {
            // When there is a datagram from unnamed unix socket
            // linux returns zero bytes of address
            len = sun_path_offset(&addr) as c_int; // i.e. zero-length address
        } else if addr.sun_family != c::AF_UNIX {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "file descriptor did not correspond to a Unix socket",
            ));
        }

        Ok(SocketAddr { addr, len })
    }

    /// Returns true if and only if the address is unnamed.
    ///
    /// # Examples
    ///
    /// A named address:
    ///
    /// ```no_run
    /// use uds_windows::UnixListener;
    ///
    /// let socket = UnixListener::bind("/tmp/sock").unwrap();
    /// let addr = socket.local_addr().expect("Couldn't get local address");
    /// assert_eq!(addr.is_unnamed(), false);
    /// ```

    // TODO: Is this following section relevant on Windows? Removed from the
    //       docs for now...
    // An unnamed address:
    //
    // ```ignore
    // use std::os::windows::net::UnixDatagram;
    //
    // let socket = UnixDatagram::unbound().unwrap();
    // let addr = socket.local_addr().expect("Couldn't get local address");
    // assert_eq!(addr.is_unnamed(), true);
    // ```
    pub fn is_unnamed(&self) -> bool {
        matches!(self.address(), AddressKind::Unnamed)
    }

    /// Returns the contents of this address if it is a `pathname` address.
    ///
    /// # Examples
    ///
    /// With a pathname:
    ///
    /// ```no_run
    /// use uds_windows::UnixListener;
    /// use std::path::Path;
    ///
    /// let socket = UnixListener::bind("/tmp/sock").unwrap();
    /// let addr = socket.local_addr().expect("Couldn't get local address");
    /// assert_eq!(addr.as_pathname(), Some(Path::new("/tmp/sock")));
    /// ```

    // TODO: Is this following section relevant on Windows? Removed from the
    //       docs for now...
    // Without a pathname:
    //
    // ```ignore
    // use std::os::windows::net::UnixDatagram;
    //
    // let socket = UnixDatagram::unbound().unwrap();
    // let addr = socket.local_addr().expect("Couldn't get local address");
    // assert_eq!(addr.as_pathname(), None);
    // ```
    pub fn as_pathname(&self) -> Option<&Path> {
        if let AddressKind::Pathname(path) = self.address() {
            Some(path)
        } else {
            None
        }
    }

    fn address(&self) -> AddressKind {
        let len = self.len as usize - sun_path_offset(&self.addr);
        // sockaddr_un::sun_path on Windows is a Win32 UTF-8 file system path
        let path = unsafe { mem::transmute::<&[c_char], &[u8]>(&self.addr.sun_path) };

        // macOS seems to return a len of 16 and a zeroed sun_path for unnamed addresses
        if len == 0
            || (cfg!(not(any(target_os = "linux", target_os = "android")))
                && self.addr.sun_path[0] == 0)
        {
            AddressKind::Unnamed
        } else if self.addr.sun_path[0] == 0 {
            AddressKind::Abstract(&path[1..len])
        } else {
            use std::ffi::CStr;
            let pathname = unsafe { CStr::from_bytes_with_nul_unchecked(&path[..len]) };
            AddressKind::Pathname(Path::new(pathname.to_str().unwrap()))
        }
    }
}

impl fmt::Debug for SocketAddr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.address() {
            AddressKind::Unnamed => write!(fmt, "(unnamed)"),
            AddressKind::Abstract(name) => write!(fmt, "{} (abstract)", AsciiEscaped(name)),
            AddressKind::Pathname(path) => write!(fmt, "{:?} (pathname)", path),
        }
    }
}

impl PartialEq for SocketAddr {
    fn eq(&self, other: &SocketAddr) -> bool {
        let ita = self.addr.sun_path.iter();
        let itb = other.addr.sun_path.iter();

        self.len == other.len
            && self.addr.sun_family == other.addr.sun_family
            && ita.zip(itb).all(|(a, b)| a == b)
    }
}

pub fn from_sockaddr_un(addr: c::sockaddr_un, len: c_int) -> io::Result<SocketAddr> {
    SocketAddr::from_parts(addr, len)
}

pub fn from_path(path: &Path) -> io::Result<SocketAddr> {
    let (addr, len) = unsafe { sockaddr_un(path)? };
    SocketAddr::from_parts(addr, len)
}

struct AsciiEscaped<'a>(&'a [u8]);

impl<'a> fmt::Display for AsciiEscaped<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "\"")?;
        for byte in self.0.iter().cloned().flat_map(ascii::escape_default) {
            write!(fmt, "{}", byte as char)?;
        }
        write!(fmt, "\"")
    }
}

pub use self::ext::{AcceptAddrs, AcceptAddrsBuf, UnixListenerExt, UnixStreamExt};
pub use self::net::{UnixListener, UnixStream};
