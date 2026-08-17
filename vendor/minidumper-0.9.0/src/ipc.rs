cfg_if::cfg_if! {
    if #[cfg(any(target_os = "linux", target_os = "android"))] {
        use std::os::{
            unix::{prelude::{RawFd, BorrowedFd}, io::AsRawFd},
            fd::AsFd,
        };

        type Stream = uds::UnixSeqpacketConn;

        struct Connection(uds::nonblocking::UnixSeqpacketConn);

        impl polling::AsRawSource for Connection {
            fn raw(&self) -> RawFd {
                self.0.as_raw_fd()
            }
        }

        impl AsRawFd for Connection {
            fn as_raw_fd(&self) -> RawFd {
                self.0.as_raw_fd()
            }
        }

        impl AsFd for Connection {
            fn as_fd(&self) -> BorrowedFd<'_> {
                #[allow(unsafe_code)]
                unsafe { BorrowedFd::borrow_raw(self.as_raw_fd()) }
            }
        }

        impl Connection {
            #[inline]
            fn send(&self, buf: &[u8]) -> Result<usize, std::io::Error> {
                self.0.send(buf)
            }

            #[inline]
            fn recv(&self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
                self.0.recv(buf)
            }

            #[inline]
            fn recv_vectored(&self, buf: &mut [std::io::IoSliceMut<'_>]) -> Result<(usize, bool), std::io::Error> {
                self.0.recv_vectored(buf)
            }
        }

        struct Listener(uds::nonblocking::UnixSeqpacketListener);

        impl polling::AsRawSource for Listener {
            fn raw(&self) -> RawFd {
                self.0.as_raw_fd()
            }
        }

        impl AsRawFd for Listener {
            fn as_raw_fd(&self) -> RawFd {
                self.0.as_raw_fd()
            }
        }

        impl AsFd for Listener {
            fn as_fd(&self) -> BorrowedFd<'_> {
                #[allow(unsafe_code)]
                unsafe { BorrowedFd::borrow_raw(self.as_raw_fd()) }
            }
        }

        impl Listener {
            fn accept_unix_addr(&self) -> Result<(Connection, uds::UnixSocketAddr), std::io::Error> {
                self.0.accept_unix_addr().map(|(conn, addr)| (Connection(conn), addr))
            }
        }
    } else if #[cfg(target_os = "windows")] {
        mod windows;

        type Stream = windows::UnixStream;

        type Listener = windows::UnixListener;
        type Connection = windows::UnixStream;

        // This will of course break if the client and server are built for different
        // arches, but that is the fault of the user in that case
        cfg_if::cfg_if! {
            if #[cfg(target_pointer_width = "32")] {
                type ProtoPointer = u32;
            } else if #[cfg(target_pointer_width = "64")] {
                type ProtoPointer = u64;
            }
        }

        #[derive(scroll::Pwrite, scroll::Pread, scroll::SizeWith)]
        struct DumpRequest {
            /// The address of an `EXCEPTION_POINTERS` in the client's memory
            exception_pointers: ProtoPointer,
            /// The process id of the client process
            process_id: u32,
            /// The id of the thread in the client process in which the crash originated
            thread_id: u32,
            /// The top level exception code, also found in the `EXCEPTION_POINTERS.ExceptionRecord.ExceptionCode`
            exception_code: i32,
        }
    } else if #[cfg(target_os = "macos")] {
        mod mac;

        type Stream = mac::UnixStream;

        type Listener = mac::UnixListener;
        type Connection = mac::UnixStream;

        #[derive(scroll::Pwrite, scroll::Pread, scroll::SizeWith)]
        struct DumpRequest {
            /// The exception code
            code: i64,
            /// Optional subcode, typically only present for `EXC_BAD_ACCESS` exceptions
            subcode: i64,
            /// The process which crashed
            task: u32,
            /// The thread in the process that crashed
            thread: u32,
            /// The thread that handled the exception. This may be useful to ignore.
            handler_thread: u32,
            /// The exception kind
            kind: i32,
            /// Boolean to indicate if there is exception information or not
            has_exception: u8,
            /// Boolean to indicate if there is a subcode
            has_subcode: u8,
        }

    }
}

mod client;
mod server;

pub use client::Client;
pub use server::Server;

const CRASH: u32 = 0;
#[cfg_attr(target_os = "macos", allow(dead_code))]
const CRASH_ACK: u32 = 1;
const PING: u32 = 2;
const PONG: u32 = 3;
const USER: u32 = 4;

/// A socket name.
///
/// Linux, Windows, and Macos can all use a file path as the name for the socket.
///
/// Additionally, Linux can use a plain string that will be used as an abstract
/// name. See [here](https://man7.org/linux/man-pages/man7/unix.7.html) for
/// more details on abstract namespace sockets.
///
/// Note that on Macos, this name is _also_ used as the name for a mach port.
/// Apple doesn't have good/any documentation for mach port service names, but
/// they are allowed to be longer than the path for a socket name. We also
/// require that the path be utf-8.
#[derive(Copy, Clone)]
pub enum SocketName<'scope> {
    /// The path to the domain socket
    Path(&'scope std::path::Path),
    /// An abstract Linux socket
    #[cfg(any(target_os = "linux", target_os = "android"))]
    Abstract(&'scope str),
}

impl<'scope> SocketName<'scope> {
    /// Create an abstract Linux socket name
    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[inline]
    pub fn abstract_namespace(name: &'scope str) -> Self {
        Self::Abstract(name)
    }

    /// Convenience function to create a [`Self::Path`] from a string
    #[inline]
    pub fn path(path: &'scope str) -> Self {
        Self::Path(std::path::Path::new(path))
    }
}

impl<'scope> From<&'scope std::path::Path> for SocketName<'scope> {
    fn from(s: &'scope std::path::Path) -> Self {
        Self::Path(s)
    }
}

#[derive(Copy, Clone)]
#[cfg_attr(test, derive(PartialEq, Eq, Debug))]
#[repr(C)]
pub struct Header {
    kind: u32,
    size: u32,
}

impl Header {
    fn as_bytes(&self) -> &[u8] {
        #[allow(unsafe_code)]
        unsafe {
            let size = std::mem::size_of::<Self>();
            let ptr = (self as *const Self).cast();
            std::slice::from_raw_parts(ptr, size)
        }
    }

    fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() != std::mem::size_of::<Self>() {
            return None;
        }

        #[allow(unsafe_code)]
        unsafe {
            Some(*buf.as_ptr().cast::<Self>())
        }
    }
}

#[cfg(test)]
mod test {
    use super::Header;

    #[test]
    fn header_bytes() {
        let expected = Header {
            kind: 20,
            size: 8 * 1024,
        };
        let exp_bytes = expected.as_bytes();

        let actual = Header::from_bytes(exp_bytes).unwrap();

        assert_eq!(expected, actual);
    }
}
