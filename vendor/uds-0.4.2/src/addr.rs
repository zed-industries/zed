use std::fmt::{self, Debug, Display};
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::ffi::{OsStr, CStr};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::net;
use std::{mem, slice};
use std::io::{self, ErrorKind};

use libc::{sockaddr, sa_family_t, AF_UNIX, socklen_t, sockaddr_un, c_char};

/// Offset of `.sun_path` in `sockaddr_un`.
///
/// This is not always identical to `mem::size_of::<sa_family_t>()`,
/// as there can be other fields before or after `.sun_family`.
fn path_offset() -> socklen_t {
    unsafe {
        let total_size = mem::size_of::<sockaddr_un>();
        let name_size = mem::size_of_val(&mem::zeroed::<sockaddr_un>().sun_path);
        (total_size - name_size) as socklen_t
    }
}

const fn as_u8(slice: &[c_char]) -> &[u8] {
    unsafe { &*(slice as *const[c_char] as *const[u8]) }
}

const fn as_char(slice: &[u8]) -> &[c_char] {
    unsafe { &*(slice as *const[u8] as *const[c_char]) }
}

const TOO_LONG_DESC: &str = "address is too long";

/// A unix domain socket address.
///
/// # Differences from `std`'s `unix::net::SocketAddr`
///
/// This type fully supports Linux's abstract socket addresses,
/// and can be created by user code instead of just returned by `accept()`
/// and similar.
///
/// # Examples
///
/// Creating an abstract address (fails if the OS doesn't support them):
///
#[cfg_attr(any(target_os="linux", target_os="android"), doc="```")]
#[cfg_attr(not(any(target_os="linux", target_os="android")), doc="```no_run")]
/// use uds::UnixSocketAddr;
///
/// let addr = UnixSocketAddr::new("@abstract").unwrap();
/// assert!(addr.is_abstract());
/// assert_eq!(addr.to_string(), "@abstract");
/// ```
#[derive(Clone,Copy)]
pub struct UnixSocketAddr {
    addr: sockaddr_un,
    /// How many bytes of addr are in use.
    ///
    /// Must never be greater than `size_of::<sockaddr_un>()`.
    ///
    /// On BSDs and macOS, `sockaddr_un` has a (non-standard) `.sun_len` field
    /// that *could* be used to store the length instead, but doing that is
    /// not a very good idea:  
    /// At least [NetBSD ignores it](http://mail-index.netbsd.org/tech-net/2006/10/11/0008.html)
    /// so we would still need to pass a correctly set `socklen_t`,
    /// in some cases by referece.
    /// Because it's rarely used and some BSDs aren't afraid to break stuff,
    /// it could even dissappear in the future.  
    /// The size this extra field is also rather minor compared to the size of
    /// `sockaddr_un`, so the possible benefit is tiny.
    len: socklen_t,
}

/// An enum representation of an unix socket address.
///
/// Useful for pattern matching an [`UnixSocketAddr`](struct.UnixSocketAddr.html)
/// via [`UnixSocketAddr.name()`](struct.UnixSocketAddr.html#method.name).
///
/// It cannot be used to bind or connect a socket directly as it
/// doesn't contain a `sockaddr_un`, but a `UnixSocketAddr` can be created
/// from it.
///
/// # Examples
///
/// Cleaning up pathname socket files after ourselves:
///
/// ```no_run
/// # use uds::{UnixSocketAddr, AddrName};
/// let addr = UnixSocketAddr::from_path("/var/run/socket.sock").unwrap();
/// if let AddrName::Path(path) = addr.name() {
///     let _ = std::fs::remove_file(path);
/// }
/// ```
#[derive(Clone,Copy, PartialEq,Eq,Hash, Debug)]
pub enum AddrName<'a> {
    /// Unnamed / anonymous address.
    Unnamed,
    /// Regular file path based address.
    ///
    /// Can be both relative and absolute.
    Path(&'a Path),
    /// Address in the abstract namespace.
    Abstract(&'a [u8]),
}
impl<'a> From<&'a UnixSocketAddr> for AddrName<'a> {
    fn from(addr: &'a UnixSocketAddr) -> AddrName<'a> {
        let name_len = addr.len as isize - path_offset() as isize;
        if addr.is_unnamed() {
            AddrName::Unnamed
        } else if addr.is_abstract() {
            let slice = &addr.addr.sun_path[1..name_len as usize];
            AddrName::Abstract(as_u8(slice))
        } else {
            let mut slice = &addr.addr.sun_path[..name_len as usize];
            // remove trailing NUL if present (and multiple NULs on OpenBSD)
            while let Some(&0) = slice.last() {
                slice = &slice[..slice.len()-1];
            }
            AddrName::Path(Path::new(OsStr::from_bytes(as_u8(slice))))
        }
    }
}

pub type UnixSocketAddrRef<'a> = AddrName<'a>;

impl Debug for UnixSocketAddr {
    fn fmt(&self,  fmtr: &mut fmt::Formatter) -> fmt::Result {
        #[derive(Debug)]
        struct Unnamed;
        #[derive(Debug)]
        struct Path<'a>(&'a std::path::Path);
        #[derive(Debug)]
        struct Abstract<'a>(&'a OsStr);

        // doesn't live long enough if created inside match
        let mut path_type = Path("".as_ref());
        let mut abstract_type = Abstract(OsStr::new(""));

        let variant: &dyn Debug = match self.into() {
            UnixSocketAddrRef::Unnamed => &Unnamed,
            UnixSocketAddrRef::Path(path) => {
                path_type.0 = path;
                &path_type
            },
            UnixSocketAddrRef::Abstract(name) => {
                abstract_type.0 = OsStr::from_bytes(name);
                &abstract_type
            },
        };
        fmtr.debug_tuple("UnixSocketAddr").field(variant).finish()
    }
}

impl Display for UnixSocketAddr {
    fn fmt(&self,  fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self.into() {
            UnixSocketAddrRef::Unnamed => fmtr.write_str("unnamed"),
            UnixSocketAddrRef::Path(path) => write!(fmtr, "{}", path.display()), // TODO check that display() doesn't print \n as-is
            UnixSocketAddrRef::Abstract(name) => write!(fmtr, "@{}", OsStr::from_bytes(name).to_string_lossy()), // FIXME escape to sane characters
        }
    }
}

impl UnixSocketAddr {
    /// Allows creating abstract, path or unspecified address based on an
    /// user-supplied string.
    ///
    /// A leading `'@'` or `'\0'` signifies an abstract address,
    /// an empty slice is taken as the unnamed address, and anything else is a
    /// path address.  
    /// If a relative path address starts with `@`, escape it by prepending
    /// `"./"`.
    /// To avoid surprises, abstract addresses will be detected regargsless of
    /// wheither the OS supports them, and result in an error if it doesn't.
    ///
    /// # Errors
    ///
    /// * A path or abstract address is too long.
    /// * A path address contains `'\0'`.
    /// * An abstract name was supplied on an OS that doesn't support them.
    ///
    /// # Examples
    ///
    /// Abstract address:
    ///
    /// ```
    /// # use uds::UnixSocketAddr;
    /// if UnixSocketAddr::has_abstract_addresses() {
    ///     assert!(UnixSocketAddr::new("@abstract").unwrap().is_abstract());
    ///     assert!(UnixSocketAddr::new("\0abstract").unwrap().is_abstract());
    /// } else {
    ///     assert!(UnixSocketAddr::new("@abstract").is_err());
    ///     assert!(UnixSocketAddr::new("\0abstract").is_err());
    /// }
    /// ```
    ///
    /// Escaped path address:
    ///
    /// ```
    /// # use uds::UnixSocketAddr;
    /// assert!(UnixSocketAddr::new("./@path").unwrap().is_relative_path());
    /// ```
    ///
    /// Unnamed address:
    ///
    /// ```
    /// # use uds::UnixSocketAddr;
    /// assert!(UnixSocketAddr::new("").unwrap().is_unnamed());
    /// ```
    pub fn new<A: AsRef<[u8]>+?Sized>(addr: &A) -> Result<Self, io::Error> {
        fn parse(addr: &[u8]) -> Result<UnixSocketAddr, io::Error> {
            match addr.first() {
                Some(&b'@') | Some(&b'\0') => UnixSocketAddr::from_abstract(&addr[1..]),
                Some(_) => UnixSocketAddr::from_path(Path::new(OsStr::from_bytes(addr))),
                None => Ok(UnixSocketAddr::new_unspecified()),
            }
        }
        parse(addr.as_ref())
    }

    /// Creates an unnamed address, which on Linux can be used for auto-bind.
    ///
    /// Binding a socket to the unnamed address is different from not binding
    /// at all:
    ///
    /// On Linux doing so binds the socket to a random abstract address
    /// determined by the OS.
    ///
    /// # Examples
    ///
    #[cfg_attr(any(target_os="linux", target_os="android"), doc="```")]
    #[cfg_attr(not(any(target_os="linux", target_os="android")), doc="```no_run")]
    /// # use uds::{UnixSocketAddr, UnixDatagramExt};
    /// # use std::os::unix::net::UnixDatagram;
    /// let addr = UnixSocketAddr::new_unspecified();
    /// assert!(addr.is_unnamed());
    /// let socket = UnixDatagram::unbound().unwrap();
    /// socket.bind_to_unix_addr(&addr).unwrap();
    /// assert!(socket.local_unix_addr().unwrap().is_abstract());
    /// ```
    pub fn new_unspecified() -> Self {
        let mut addr: sockaddr_un = unsafe { mem::zeroed() };
        addr.sun_family = AF_UNIX as sa_family_t;
        UnixSocketAddr {
            len: path_offset(),
            addr,
        }
    }

    /// Returns the maximum size of pathname addresses supported by `UnixSocketAddr`.
    ///
    /// Is the size of the underlying `sun_path` field, minus 1 if the OS
    /// is known to either require a trailing NUL (`'\0'`) byte,
    /// or supports longer paths that go past the end of `sun_path`.
    ///
    /// These OSes are:
    ///
    /// * OpenBSD: Enforces that `sun_path`` is NUL-terminated.
    /// * macOS / iOS / anything else Apple: I haven't found a manpage,
    ///   but it supports longer paths.
    /// * Illumos: [The manpage](https://illumos.org/man/3SOCKET/sockaddr_un)
    ///   says it must be NUL-terminated (and that it cannot be longer),
    ///   but when I tested on an older version, neither of these constraints seem to be the case.
    /// * Solaris: Assumed to be identical to Illumos.
    ///
    /// OSes that have been tested that they allow using the full `sun_path`
    /// without NUL and no longer paths, and whose manpages don't state the opposite:
    ///
    /// * [Linux](https://www.man7.org/linux/man-pages/man7/unix.7.html)
    /// * [FreeBSD](https://man.freebsd.org/cgi/man.cgi?query=unix&sektion=4)
    /// * [NetBSD](https://man.netbsd.org/unix.4)
    /// * [Dragonfly BSD](https://man.dragonflybsd.org/?command=unix&section=4)
    pub fn max_path_len() -> usize {
        let always_nul_terminate = cfg!(any(
            target_os="openbsd",
            target_vendor="apple",
            target_os="illumos",
            target_os="solaris",
        ));
        mem::size_of_val(&Self::new_unspecified().addr.sun_path)
            - if always_nul_terminate {1} else {0}
    }

    /// Creates a pathname unix socket address.
    ///
    /// # Errors
    ///
    /// This function will return an error if the path is too long for the
    /// underlying `sockaddr_un` type, or contains NUL (`'\0'`) bytes.
    pub fn from_path<P: AsRef<Path>+?Sized>(path: &P) -> Result<Self, io::Error> {
        fn from_path_inner(path: &[u8]) -> Result<UnixSocketAddr, io::Error> {
            let mut addr = UnixSocketAddr::new_unspecified();
            let capacity = UnixSocketAddr::max_path_len();
            if path.is_empty() {
                Err(io::Error::new(ErrorKind::NotFound, "path is empty"))
            } else if path.len() > capacity {
                let message = "path is too long for an unix socket address";
                Err(io::Error::new(ErrorKind::InvalidInput, message))
            } else if path.iter().any(|&b| b == b'\0' ) {
                Err(io::Error::new(ErrorKind::InvalidInput, "path cannot contain nul bytes"))
            } else {
                addr.addr.sun_path[..path.len()].copy_from_slice(as_char(path));
                addr.len = path_offset() + path.len() as socklen_t;
                if path.len() < capacity {
                    addr.len += 1; // for increased portability
                }
                Ok(addr)
            }
        }
        from_path_inner(path.as_ref().as_os_str().as_bytes())
    }

    /// Returns maximum size of abstract addesses supported by `UnixSocketAddr`.
    ///
    /// Is the size of the underlying `sun_path` field minus 1 for the
    /// leading `'\0'` byte.
    ///
    /// This value is also returned on operating systems that doesn't support
    /// abstract addresses.
    pub fn max_abstract_len() -> usize {
        mem::size_of_val(&Self::new_unspecified().addr.sun_path) - 1
    }

    /// Returns whether the operating system is known to support
    /// abstract unix domain socket addresses.
    ///
    /// Is `true` for Linux & Android, and `false` for all other OSes.
    pub const fn has_abstract_addresses() -> bool {
        cfg!(any(target_os="linux", target_os="android"))
    }

    /// Creates an abstract unix domain socket address.
    ///
    /// Abstract addresses use a namespace separate from the file system,
    /// that doesn't have directories (ie. is flat) or permissions.
    /// The advandage of it is that the address disappear when the socket bound
    /// to it is closed, which frees one from dealing with removing it when
    /// shutting down cleanly.
    ///
    /// They are a Linux-only feature though, and this function will fail
    /// if abstract addresses are not supported.
    ///
    /// # Errors
    ///
    /// This function will return an error if the name is too long.
    /// Call [`max_abstract_len()`](#method.max_abstract_len)
    /// get the limit.
    ///
    /// It will also fail on operating systems that don't support abstract
    /// addresses. (ie. anything other than Linux and Android)
    pub fn from_abstract<N: AsRef<[u8]>+?Sized>(name: &N) -> Result<Self, io::Error> {
        fn from_abstract_inner(name: &[u8]) -> Result<UnixSocketAddr, io::Error> {
            let mut addr = UnixSocketAddr::new_unspecified();
            if !UnixSocketAddr::has_abstract_addresses() {
                Err(io::Error::new(ErrorKind::AddrNotAvailable, format!(
                            "abstract unix domain socket addresses are not available on {}",
                            std::env::consts::OS
                )))
            } else if name.len() > UnixSocketAddr::max_abstract_len() {
                Err(io::Error::new(ErrorKind::InvalidInput, "abstract name is too long"))
            } else {
                addr.addr.sun_path[1..1+name.len()].copy_from_slice(as_char(name));
                addr.len = path_offset() + 1 + name.len() as socklen_t;
                Ok(addr)
            }
        }
        from_abstract_inner(name.as_ref())
    }

    /// Tries to convert a `std::os::unix::net::SocketAddr` into an `UnixSocketAddr`.
    ///
    /// This can fail (produce `None`) on Linux and Android
    /// if the `std` `SocketAddr` represents an abstract address,
    /// as it provides no method for viewing abstract addresses.
    /// (other than parsing its `Debug` output, anyway.)
    pub fn from_std(addr: net::SocketAddr) -> Option<Self> {
        if let Some(path) = addr.as_pathname() {
            Some(Self::from_path(path).expect("pathname addr cannot be converted"))
        } else if addr.is_unnamed() {
            Some(Self::new_unspecified())
        } else {
            None
        }
    }

    /// Returns unnamed addres for empty strings, and path addresses otherwise.
    ///
    /// # Errors
    ///
    /// Returns ENAMETOOLONG if path (without the trailing `'\0'`) is too long
    /// for `sockaddr_un.sun_path`.
    pub fn from_c_str(path: &CStr) -> Result<Self, io::Error> {
        let path = path.to_bytes();
        let mut addr = Self::new_unspecified();
        if path.is_empty() {
            Ok(addr)
        } else if path.len() > mem::size_of_val(&addr.addr.sun_path) {
            let message = "path is too long for unix socket address";
            Err(io::Error::new(ErrorKind::InvalidInput, message))
        } else {
            addr.addr.sun_path[..path.len()].copy_from_slice(as_char(path));
            addr.len = path_offset() + path.len() as socklen_t;
            if path.len() < mem::size_of_val(&addr.addr.sun_path) {
                addr.len += 1;
            }
            Ok(addr)
        }
    }

    /// Checks whether the address is unnamed.
    pub fn is_unnamed(&self) -> bool {
        if Self::has_abstract_addresses() {
            self.len <= path_offset()
        } else {
            // MacOS can apparently return non-empty addresses but with
            // all-zeroes path for unnamed addresses.
            self.len <= path_offset()  ||  self.addr.sun_path[0] as u8 == b'\0'
        }
    }
    /// Checks whether the address is a name in the abstract namespace.
    ///
    /// Always returns `false` on operating systems that don't support abstract
    /// addresses.
    pub fn is_abstract(&self) -> bool {
        if Self::has_abstract_addresses() {
            self.len > path_offset()  &&  self.addr.sun_path[0] as u8 == b'\0'
        } else {
            false
        }
    }
    /// Checks whether the address is a path that begins with '/'.
    pub fn is_absolute_path(&self) -> bool {
        self.len > path_offset()  &&  self.addr.sun_path[0] as u8 == b'/'
    }
    /// Checks whether the address is a path that doesn't begin with '/'.
    pub fn is_relative_path(&self) -> bool {
        self.len > path_offset()
            &&  self.addr.sun_path[0] as u8 != b'\0'
            &&  self.addr.sun_path[0] as u8 != b'/'
    }
    /// Checks whether the address is a path.
    pub fn is_path(&self) -> bool {
        self.len > path_offset()  &&  self.addr.sun_path[0] as u8 != b'\0'
    }

    /// Returns a view of the address that can be pattern matched
    /// to the differnt types of addresses.
    ///
    /// # Examples
    ///
    /// ```
    /// use uds::{UnixSocketAddr, AddrName};
    /// use std::path::Path;
    ///
    /// assert_eq!(
    ///     UnixSocketAddr::new_unspecified().name(),
    ///     AddrName::Unnamed
    /// );
    /// assert_eq!(
    ///     UnixSocketAddr::from_path("/var/run/socket.sock").unwrap().name(),
    ///     AddrName::Path(Path::new("/var/run/socket.sock"))
    /// );
    /// if UnixSocketAddr::has_abstract_addresses() {
    ///     assert_eq!(
    ///         UnixSocketAddr::from_abstract("tcartsba").unwrap().name(),
    ///         AddrName::Abstract(b"tcartsba")
    ///     );
    /// }
    /// ```
    pub fn name(&self) -> AddrName {
        AddrName::from(self)
    }

    /// Returns the path of a path-based address.
    pub fn as_pathname(&self) -> Option<&Path> {
        match UnixSocketAddrRef::from(self) {
            UnixSocketAddrRef::Path(path) => Some(path),
            _ => None
        }
    }

    /// Returns the name of an address which is in the abstract namespace.
    pub fn as_abstract(&self) -> Option<&[u8]> {
        match UnixSocketAddrRef::from(self) {
            UnixSocketAddrRef::Abstract(name) => Some(name),
            _ => None
        }
    }

    /// Returns a view that can be pattern matched to the differnt types of
    /// addresses.
    ///
    /// # Examples
    ///
    /// ```
    /// use uds::{UnixDatagramExt, UnixSocketAddr, UnixSocketAddrRef};
    /// use std::os::unix::net::UnixDatagram;
    /// use std::path::Path;
    ///
    /// # let _ = std::fs::remove_file("dgram.socket");
    /// let receiver = UnixDatagram::bind("dgram.socket").expect("create datagram socket");
    /// assert_eq!(
    ///     receiver.local_unix_addr().unwrap().as_ref(),
    ///     UnixSocketAddrRef::Path(Path::new("dgram.socket"))
    /// );
    ///
    /// let sender = UnixDatagram::unbound().expect("create unbound datagram socket");
    /// sender.send_to(b"I can't hear you", "dgram.socket").expect("send");
    ///
    /// let mut buf = [0; 100];
    /// let (len, addr) = receiver.recv_from_unix_addr(&mut buf).unwrap();
    /// assert_eq!(addr.as_ref(), UnixSocketAddrRef::Unnamed);
    /// # std::fs::remove_file("dgram.socket").expect("clean up socket file");
    /// ```
    pub fn as_ref(&self) -> UnixSocketAddrRef {
        UnixSocketAddrRef::from(self)
    }

    /// Creates an address from a slice of bytes to place in `sun_path`.
    ///
    /// This is a low-level but simple interface for creating addresses by
    /// other unix socket wrappers without exposing any libc types.  
    /// The meaning of a slice can vary between operating systems.
    ///
    /// `addr` should point to thes start of the "path" part of a socket
    /// address, with length being the number of valid bytes of the path.
    /// (Trailing NULs are not stripped by this function.)
    ///
    /// # Errors
    ///
    /// If the slice is longer than `sun_path`, an error of kind `Other` is
    /// returned. No other validation of the bytes is performed.
    ///
    /// # Examples
    ///
    /// A normal path-based address
    ///
    /// ```
    /// # use std::path::Path;
    /// # use uds::UnixSocketAddr;
    /// let addr = UnixSocketAddr::from_raw_bytes(b"/var/run/a.sock\0").unwrap();
    /// assert_eq!(addr.as_pathname(), Some(Path::new("/var/run/a.sock")));
    /// assert_eq!(addr.as_raw_bytes(), b"/var/run/a.sock\0");
    /// ```
    ///
    /// On Linux:
    ///
    #[cfg_attr(any(target_os="linux", target_os="android"), doc="```")]
    #[cfg_attr(not(any(target_os="linux", target_os="android")), doc="```no_run")]
    /// # use uds::UnixSocketAddr;
    /// let addr = UnixSocketAddr::from_raw_bytes(b"\0a").unwrap();
    /// assert_eq!(addr.as_abstract(), Some(&b"a"[..]));
    /// assert_eq!(addr.as_raw_bytes(), b"\0a");
    /// ```
    ///
    /// Elsewhere:
    ///
    #[cfg_attr(not(any(target_os="linux", target_os="android")), doc="```")]
    #[cfg_attr(any(target_os="linux", target_os="android"), doc="```no_run")]
    /// # use uds::UnixSocketAddr;
    /// let addr = UnixSocketAddr::from_raw_bytes(b"\0a").unwrap();
    /// assert!(addr.is_unnamed());
    /// assert_eq!(addr.as_raw_bytes().len(), 2);
    /// ```
    ///
    /// A portable unnamed address:
    ///
    /// ```
    /// # use uds::UnixSocketAddr;
    /// let addr = UnixSocketAddr::from_raw_bytes(&[]).expect("not too long");
    /// assert!(addr.is_unnamed());
    /// assert!(addr.as_raw_bytes().is_empty());
    /// ```
    pub fn from_raw_bytes(addr: &[u8]) -> Result<Self, io::Error> {
        if addr.len() <= Self::max_path_len() {
            let name = addr;
            let mut addr = Self::default();
            addr.addr.sun_path[..name.len()].copy_from_slice(as_char(name));
            addr.len = path_offset() + name.len() as socklen_t;
            Ok(addr)
        } else {
            Err(io::Error::new(ErrorKind::InvalidInput, TOO_LONG_DESC))
        }
    }
    /// Returns a low-level view of the address without using any libc types.
    ///
    /// The returned slice points to the start of `sun_addr` of the contained
    /// `sockaddr_un`, and the length is the number of bytes of `sun_addr`
    /// that were filled out by the OS. Any trailing NUL(s) will be preserved.
    ///
    /// # Examples
    ///
    /// A normal path-based address:
    ///
    /// ```
    /// # use std::path::Path;
    /// # use std::os::unix::net::UnixDatagram;
    /// # use uds::{UnixSocketAddr, UnixDatagramExt};
    /// let pathname = "a file";
    /// let socket = UnixDatagram::bind(pathname).expect("create datagram socket");
    /// # let _ = std::fs::remove_file(pathname);
    /// let addr = socket.local_unix_addr().expect("get its address");
    /// assert!(addr.as_raw_bytes().starts_with(pathname.as_bytes()));
    /// assert!(addr.as_raw_bytes()[pathname.len()..].iter().all(|&b| b == b'\0' ));
    /// assert_eq!(addr.as_pathname(), Some(Path::new(pathname)));
    /// ```
    ///
    /// Abstract address:
    ///
    #[cfg_attr(any(target_os="linux", target_os="android"), doc="```")]
    #[cfg_attr(not(any(target_os="linux", target_os="android")), doc="```no_run")]
    /// # use uds::UnixSocketAddr;
    /// let addr = UnixSocketAddr::new("@someone@").unwrap();
    /// assert_eq!(addr.as_raw_bytes(), b"\0someone@");
    /// ```
    ///
    /// Unnamed address on macOS, OpenBSD and maybe others:
    ///
    #[cfg_attr(any(target_vendor="apple", target_os="openbsd"), doc="```")]
    #[cfg_attr(not(any(target_vendor="apple", target_os="openbsd")), doc="```no_run")]
    /// # use std::os::unix::net::UnixDatagram;
    /// # use uds::{UnixSocketAddr, UnixDatagramExt};
    /// let socket = UnixDatagram::unbound().expect("create datagram socket");
    /// let addr = socket.local_unix_addr().expect("get its unbound address");
    /// let bytes = addr.as_raw_bytes();
    /// assert!(bytes.len() > 0);
    /// assert!(bytes.iter().all(|&b| b == b'\0' ));
    /// assert!(addr.is_unnamed());
    /// ```
    pub fn as_raw_bytes(&self) -> &[u8] {
        as_u8(&self.addr.sun_path[..(self.len-path_offset()) as usize])
    }

    /// Prepares a `struct sockaddr*` and `socklen_t*` for passing to FFI
    /// (such as `getsockname()`, `getpeername()`, or `accept()`),
    /// and validate and normalize the produced address afterwards.
    ///
    /// Validation:
    ///
    /// * Check that the address family is `AF_UNIX`.
    /// * Check that the address wasn't truncated (the `socklen_t` is too big).
    ///
    /// Normalization:
    ///
    /// * Ensure path addresses have a trailing NUL byte if there is space.
    pub fn new_from_ffi<R, F>(call: F) -> Result<(R, Self), io::Error>
    where F: FnOnce(&mut sockaddr, &mut socklen_t) -> Result<R, io::Error> {
        let mut addr = Self::new_unspecified();
        let capacity = mem::size_of_val(&addr.addr) as socklen_t;
        addr.len = capacity;
        unsafe {
            let (addr_ptr, addr_len_ptr) = addr.as_raw_mut_general();
            let ret = call(addr_ptr, addr_len_ptr)?;
            if addr.addr.sun_family != AF_UNIX as sa_family_t {
                return Err(io::Error::new(
                    ErrorKind::InvalidData,
                    "file descriptor did not correspond to a Unix socket" // identical to std's
                ));
            }
            if addr.is_abstract() {
                if addr.len > capacity {
                    return Err(io::Error::new(
                        ErrorKind::InvalidData,
                        "abstract name was too long"
                    ));
                }
            } else if addr.is_path() {
                if addr.len > capacity+1 {
                    return Err(io::Error::new(ErrorKind::InvalidData, "path was too long"));
                    // accept lengths one too big; assume the truncated byte was NUL
                } else {
                    // normalize addr.len to include terminating NUL byte if possible
                    // and not be greater than capacity
                    if addr.len >= capacity {
                        addr.len = capacity;
                    } else if addr.addr.sun_path[(addr.len-1-path_offset()) as usize] != 0 {
                        addr.len += 1;
                        addr.addr.sun_path[(addr.len-1-path_offset()) as usize] = 0;
                    }
                }
            }
            Ok((ret, addr))
        }
    }

    /// Creates an `UnixSocketAddr` from a pointer to a generic `sockaddr` and
    /// a length.
    ///
    /// # Safety
    ///
    /// * `len` must not be greater than the size of the memory `addr` points to.
    /// * `addr` must point to valid memory if `len` is greater than zero, or be NULL.
    pub unsafe fn from_raw(addr: *const sockaddr,  len: socklen_t) -> Result<Self, io::Error> {
        let mut copy = Self::new_unspecified();
        if addr.is_null() && len == 0 {
            Ok(Self::new_unspecified())
        } else if addr.is_null() {
            Err(io::Error::new(ErrorKind::InvalidInput, "addr is NULL"))
        } else if len < path_offset() {
            Err(io::Error::new(ErrorKind::InvalidInput, "address length is too short"))
        } else if len > mem::size_of::<sockaddr_un>() as socklen_t {
            Err(io::Error::new(ErrorKind::InvalidInput, TOO_LONG_DESC))
        } else if (&*addr).sa_family != AF_UNIX as sa_family_t {
            Err(io::Error::new(ErrorKind::InvalidData, "not an unix socket address"))
        } else {
            let addr = addr as *const sockaddr_un;
            let sun_path_ptr = (&*addr).sun_path.as_ptr();
            let path_len = (len - path_offset()) as usize;
            let sun_path = slice::from_raw_parts(sun_path_ptr, path_len);
            copy.addr.sun_path[..path_len].copy_from_slice(sun_path);
            copy.len = len;
            Ok(copy)
        }
    }

    /// Creates an `UnixSocketAddr` without any validation.
    ///
    /// # Safety
    ///
    /// * `len` must be `<= size_of::<sockaddr_un>()`.
    /// * `addr.sun_family` should be `AF_UNIX` or strange things might happen.
    /// * `addr.sun_len`, if it exists, should be zero (but is probably ignored).
    pub unsafe fn from_raw_unchecked(addr: sockaddr_un,  len: socklen_t) -> Self {
        Self{addr, len}
    }

    /// Splits the address into its inner, raw parts.
    pub fn into_raw(self) -> (sockaddr_un, socklen_t) {
        (self.addr, self.len)
    }

    /// Returns a general `sockaddr` reference to the address and its length.
    ///
    /// Useful for passing to `bind()`, `connect()`, `sendto()` or other FFI.
    ///
    /// Pathname addresses are not guaranteed to be NUL-terminated on most OSes:
    /// Most paths will be NUL-terminated, but paths that just fit within `sockaddr_un.sun_len`
    /// (iow their length is equal to `addr.sun_len[..].len()`) will not have one.  
    /// Therefore do not call `SUN_LEN()` on unknown addresses.  
    /// See [`max_path_len()`](#tymethod.max_path_len) for which OSes this affects.
    pub fn as_raw_general(&self) -> (&sockaddr, socklen_t) {
        // SAFETY: sockaddr is a super-type of sockaddr_un.
        (unsafe { &*(&self.addr as *const sockaddr_un as *const sockaddr) }, self.len)
    }

    /// Returns a reference to the inner `struct sockaddr_un`, and length.
    ///
    /// Pathname addresses are not guaranteed to be NUL-terminated on most OSes:
    /// Most paths will be NUL-terminated, but paths that just fit within `sockaddr_un.sun_len`
    /// (iow their length is equal to `addr.sun_len[..].len()`) will not have one.  
    /// Therefore do not call `SUN_LEN()` on unknown addresses.  
    /// See [`max_path_len()`](#tymethod.max_path_len) for which OSes this affects.
    pub fn as_raw(&self) -> (&sockaddr_un, socklen_t) {
        (&self.addr, self.len)
    }

    /// Returns mutable references to a general `struct sockaddr` and `socklen_t`.
    ///
    /// If passing to `getpeername()`, `accept()` or similar, remember to set
    /// the length to the capacity,
    /// and consider using [`new_from_ffi()`](#method.new_from_ffi) instead.
    ///
    /// Pathname addresses are not guaranteed to be NUL-terminated on most OSes:
    /// Most paths will be NUL-terminated, but paths that just fit within `sockaddr_un.sun_len`
    /// (iow their length is equal to `addr.sun_len[..].len()`) will not have one.  
    /// Therefore do not call `SUN_LEN()` on unknown addresses.  
    /// See [`max_path_len()`](#tymethod.max_path_len) for which OSes this affects.
    ///
    /// # Safety
    ///
    /// Assigning a value > `sizeof(struct sockaddr_un)` to the `socklen_t`
    /// reference might lead to out-of-bounds reads later.
    pub unsafe fn as_raw_mut_general(&mut self) -> (&mut sockaddr, &mut socklen_t) {
        // SAFETY: sockaddr is a super-type of sockaddr_un.
        (&mut*(&mut self.addr as *mut sockaddr_un as *mut sockaddr), &mut self.len)
    }

    /// Returns mutable references to the inner `struct sockaddr_un` and length.
    ///
    /// Pathname addresses are not guaranteed to be NUL-terminated on most OSes:
    /// Most paths will be NUL-terminated, but paths that just fit within `sockaddr_un.sun_len`
    /// (iow their length is equal to `addr.sun_len[..].len()`) will not have one.  
    /// Therefore do not call `SUN_LEN()` on unknown addresses.  
    /// See [`max_path_len()`](#tymethod.max_path_len) for which OSes this affects
    ///
    /// # Safety
    ///
    /// Assigning a value > `sizeof(struct sockaddr_un)` to the `socklen_t`
    /// reference might lead to out-of-bounds reads later.
    pub unsafe fn as_raw_mut(&mut self) -> (&mut sockaddr_un, &mut socklen_t) {
        (&mut self.addr, &mut self.len)
    }
}

impl Default for UnixSocketAddr {
    fn default() -> Self {
        Self::new_unspecified()
    }
}

impl PartialEq for UnixSocketAddr {
    fn eq(&self,  other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}
impl Eq for UnixSocketAddr {}
impl Hash for UnixSocketAddr {
    fn hash<H: Hasher>(&self,  hasher: &mut H) {
        self.as_ref().hash(hasher)
    }
}

impl PartialEq<[u8]> for UnixSocketAddr {
    fn eq(&self,  unescaped: &[u8]) -> bool {
        match (self.as_ref(), unescaped.first()) {
            (UnixSocketAddrRef::Path(path), Some(_)) => path.as_os_str().as_bytes() == unescaped,
            (UnixSocketAddrRef::Abstract(name), Some(b'\0')) => name == &unescaped[1..],
            (UnixSocketAddrRef::Unnamed, None) => true,
            (_, _) => false,
        }
    }
}
impl PartialEq<UnixSocketAddr> for [u8]  {
    fn eq(&self,  addr: &UnixSocketAddr) -> bool {
        addr == self
    }
}
