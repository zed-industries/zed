mod socketaddr;
pub use self::socketaddr::SocketAddr;

/// Get the `sun_path` field offset of `sockaddr_un` for the target OS.
///
/// On Linux, this function equates to the same value as
/// `size_of::<sa_family_t>()`, but some other implementations include
/// other fields before `sun_path`, so the expression more portably
/// describes the size of the address structure.
pub(in crate::sys) fn path_offset(sockaddr: &libc::sockaddr_un) -> usize {
    let base = sockaddr as *const _ as usize;
    let path = &sockaddr.sun_path as *const _ as usize;
    path - base
}

cfg_os_poll! {
    use std::cmp::Ordering;
    use std::os::unix::io::{RawFd, FromRawFd};
    use std::{io, mem};

    pub(crate) mod datagram;
    pub(crate) mod listener;
    pub(crate) mod stream;

    pub(in crate::sys) fn socket_addr(bytes: &[u8]) -> io::Result<(libc::sockaddr_un, libc::socklen_t)> {
        let sockaddr = mem::MaybeUninit::<libc::sockaddr_un>::zeroed();

        // This is safe to assume because a `libc::sockaddr_un` filled with `0`
        // bytes is properly initialized.
        //
        // `0` is a valid value for `sockaddr_un::sun_family`; it is
        // `libc::AF_UNSPEC`.
        //
        // `[0; 108]` is a valid value for `sockaddr_un::sun_path`; it begins an
        // abstract path.
        let mut sockaddr = unsafe { sockaddr.assume_init() };

        sockaddr.sun_family = libc::AF_UNIX as libc::sa_family_t;

        match (bytes.first(), bytes.len().cmp(&sockaddr.sun_path.len())) {
            // Abstract paths don't need a null terminator
            (Some(&0), Ordering::Greater) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "path must be no longer than libc::sockaddr_un.sun_path",
                ));
            }
            (_, Ordering::Greater) | (_, Ordering::Equal) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "path must be shorter than libc::sockaddr_un.sun_path",
                ));
            }
            _ => {}
        }

        for (dst, src) in sockaddr.sun_path.iter_mut().zip(bytes.iter()) {
            *dst = *src as libc::c_char;
        }

        let offset = path_offset(&sockaddr);
        let mut socklen = offset + bytes.len();

        match bytes.first() {
            // The struct has already been zeroes so the null byte for pathname
            // addresses is already there.
            Some(&0) | None => {}
            Some(_) => socklen += 1,
        }

        Ok((sockaddr, socklen as libc::socklen_t))
    }

    fn pair<T>(flags: libc::c_int) -> io::Result<(T, T)>
        where T: FromRawFd,
    {
        #[cfg(not(any(
            target_os = "aix",
            target_os = "ios",
            target_os = "macos",
            target_os = "tvos",
            target_os = "watchos",
            target_os = "espidf",
            target_os = "vita",
        )))]
        let flags = flags | libc::SOCK_NONBLOCK | libc::SOCK_CLOEXEC;

        let mut fds = [-1; 2];
        syscall!(socketpair(libc::AF_UNIX, flags, 0, fds.as_mut_ptr()))?;
        let pair = unsafe { (T::from_raw_fd(fds[0]), T::from_raw_fd(fds[1])) };

        // Darwin (and others) doesn't have SOCK_NONBLOCK or SOCK_CLOEXEC.
        //
        // In order to set those flags, additional `fcntl` sys calls must be
        // performed. If a `fnctl` fails after the sockets have been created,
        // the file descriptors will leak. Creating `pair` above ensures that if
        // there is an error, the file descriptors are closed.
        #[cfg(any(
            target_os = "aix",
            target_os = "ios",
            target_os = "macos",
            target_os = "tvos",
            target_os = "watchos",
            target_os = "espidf",
            target_os = "vita",
        ))]
        {
            syscall!(fcntl(fds[0], libc::F_SETFL, libc::O_NONBLOCK))?;
            #[cfg(not(any(target_os = "espidf", target_os = "vita")))]
            syscall!(fcntl(fds[0], libc::F_SETFD, libc::FD_CLOEXEC))?;
            syscall!(fcntl(fds[1], libc::F_SETFL, libc::O_NONBLOCK))?;
            #[cfg(not(any(target_os = "espidf", target_os = "vita")))]
            syscall!(fcntl(fds[1], libc::F_SETFD, libc::FD_CLOEXEC))?;
        }

        Ok(pair)
    }

    // The following functions can't simply be replaced with a call to
    // `net::UnixDatagram` because of our `SocketAddr` type.

    fn local_addr(socket: RawFd) -> io::Result<SocketAddr> {
        SocketAddr::new(|sockaddr, socklen| syscall!(getsockname(socket, sockaddr, socklen)))
    }

    fn peer_addr(socket: RawFd) -> io::Result<SocketAddr> {
        SocketAddr::new(|sockaddr, socklen| syscall!(getpeername(socket, sockaddr, socklen)))
    }

    #[cfg(test)]
    mod tests {
        use super::{path_offset, socket_addr};
        use std::os::unix::ffi::OsStrExt;
        use std::path::Path;
        use std::str;

        #[test]
        fn pathname_address() {
            const PATH: &str = "./foo/bar.txt";
            const PATH_LEN: usize = 13;

            // Pathname addresses do have a null terminator, so `socklen` is
            // expected to be `PATH_LEN` + `offset` + 1.
            let path = Path::new(PATH);
            let (sockaddr, actual) = socket_addr(path.as_os_str().as_bytes()).unwrap();
            let offset = path_offset(&sockaddr);
            let expected = PATH_LEN + offset + 1;
            assert_eq!(expected as libc::socklen_t, actual)
        }

        #[test]
        fn abstract_address() {
            const PATH: &[u8] = &[0, 116, 111, 107, 105, 111];
            const PATH_LEN: usize = 6;

            // Abstract addresses do not have a null terminator, so `socklen` is
            // expected to be `PATH_LEN` + `offset`.
            let (sockaddr, actual) = socket_addr(PATH).unwrap();
            let offset = path_offset(&sockaddr);
            let expected = PATH_LEN + offset;
            assert_eq!(expected as libc::socklen_t, actual)
        }
    }
}
