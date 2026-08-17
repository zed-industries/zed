use super::{Header, SocketName, Stream};
use crate::Error;
use std::io::IoSlice;

/// Client side of the connection, which runs in the process that may (or has)
/// crashed to communicate with an external monitor process.
pub struct Client {
    socket: Stream,
    /// On Macos we need this additional mach port based client to send crash
    /// contexts, as, unfortunately, it's the best (though hopefully not only?)
    /// way to get the real info needed by the minidump writer to write the
    /// minidump
    #[cfg(target_os = "macos")]
    port: crash_context::ipc::Client,
}

impl Client {
    /// Creates a new client with the given name.
    ///
    /// # Errors
    ///
    /// The specified socket name is invalid, or a connection cannot be made
    /// with a server
    pub fn with_name<'scope>(sn: SocketName<'scope>) -> Result<Self, Error> {
        cfg_if::cfg_if! {
            if #[cfg(any(target_os = "linux", target_os = "android"))] {
                let socket_addr = match sn {
                    SocketName::Path(path) => {
                        uds::UnixSocketAddr::from_path(path).map_err(|_err| Error::InvalidName)?
                    }
                    SocketName::Abstract(name) => {
                        uds::UnixSocketAddr::from_abstract(name).map_err(|_err| Error::InvalidName)?
                    }
                };

                let socket = Stream::connect_unix_addr(&socket_addr)?;
            } else if #[cfg(target_os = "windows")] {
                let SocketName::Path(path) = sn;
                let socket = Stream::connect(path)?;
            } else if #[cfg(target_os = "macos")] {
                let SocketName::Path(path) = sn;
                let socket = Stream::connect(path)?;

                // Note that sun_path is limited to 108 characters including null,
                // while a mach port name is limited to 128 including null, so
                // the length is already effectively checked here
                let port_name = std::ffi::CString::new(path.to_str().ok_or(Error::InvalidPortName)?).map_err(|_err| Error::InvalidPortName)?;
                let port = crash_context::ipc::Client::create(&port_name)?;
            } else {
                compile_error!("unimplemented target platform");
            }
        }

        let s = Self {
            socket,
            #[cfg(target_os = "macos")]
            port,
        };

        #[cfg(target_os = "macos")]
        {
            // Since we aren't sending crash requests as id 0 like for other
            // platforms, we instead abuse it to send the pid of this process
            // so that the server can pair the port and the socket together
            let id_buf = std::process::id().to_ne_bytes();
            s.send_message_impl(0, &id_buf)?;
            let mut ack = [0u8; 1];
            s.socket.recv(&mut ack)?;
        }

        Ok(s)
    }

    /// Requests that the server generate a minidump for the specified crash
    /// context. This blocks until the server has finished writing the minidump.
    ///
    /// # Linux
    ///
    /// This uses a [`crash_context::CrashContext`] by reference as the size of
    /// it can be larger than one would want in an alternate stack handler, the
    /// use of a reference allows the context to be stored outside of the stack
    /// and heap to avoid that complication, though you may of course generate
    /// one however you like.
    ///
    /// # Windows
    ///
    /// This uses a [`crash_context::CrashContext`] by reference, as
    /// the crash context internally contains pointers into this process'
    /// memory that need to stay valid for the duration of the mindump creation.
    ///
    /// # Macos
    ///
    /// It is _highly_ recommended that you suspend all threads in the current
    /// process (other than the thread that executes this method) via
    /// [`thread_suspend`](https://developer.apple.com/documentation/kernel/1418833-thread_suspend)
    /// (apologies for the terrible documentation, blame Apple) before calling
    /// this method
    pub fn request_dump(&self, crash_context: &crash_context::CrashContext) -> Result<(), Error> {
        cfg_if::cfg_if! {
            if #[cfg(any(target_os = "linux", target_os = "android"))] {
                let crash_ctx_buffer = crash_context.as_bytes();
            } else if #[cfg(target_os = "windows")] {
                use scroll::Pwrite;
                let mut buf = [0u8; 24];
                let written = buf.pwrite(
                    super::DumpRequest {
                        exception_pointers: crash_context.exception_pointers as _,
                        process_id: crash_context.process_id,
                        thread_id: crash_context.thread_id,
                        exception_code: crash_context.exception_code,
                    },
                    0,
                )?;

                let crash_ctx_buffer = &buf[..written];
            } else if #[cfg(target_os = "macos")] {
                self.port.send_crash_context(
                    crash_context,
                    Some(std::time::Duration::from_secs(2)),
                    Some(std::time::Duration::from_secs(5))
                )?;
                Ok(())
            }
        }

        #[cfg(not(target_os = "macos"))]
        {
            self.send_message_impl(0, crash_ctx_buffer)?;

            // Wait for the server to send back an ack that it has finished
            // with the crash context
            let mut ack = [0u8; std::mem::size_of::<Header>()];
            self.socket.recv(&mut ack)?;

            let header = Header::from_bytes(&ack);

            if header.filter(|hdr| hdr.kind == super::CRASH_ACK).is_none() {
                return Err(Error::ProtocolError("received invalid response to crash"));
            }

            Ok(())
        }
    }

    /// Sends a message to the server.
    ///
    /// This method is provided so that users can send their own application
    /// specific messages to the monitor process.
    ///
    /// There are no limits imposed by this method itself, but it is recommended
    /// to keep the message reasonably sized, eg. below 64KiB, as different
    /// targets will have different limits for the maximum payload that can be
    /// delivered.
    ///
    /// It is also important to note that this method can be called from multiple
    /// threads if you so choose. Each message is sent vectored and thus won't
    /// be split, but if you care about ordering you will need to handle that
    /// yourself.
    ///
    /// # Errors
    ///
    /// The send to the server fails
    #[inline]
    pub fn send_message(&self, kind: u32, buf: impl AsRef<[u8]>) -> Result<(), Error> {
        debug_assert!(kind < u32::MAX - super::USER);

        self.send_message_impl(kind + super::USER, buf.as_ref())

        // TODO: should we have an ACK? IPC is a (relatively) reliable communication
        // method, and reserving receives from the server for the exclusive
        // use of crash dumping, the main thing that users will care about, means
        // we reduce complication
        // let mut ack = [0u8; 1];
        // self.socket.recv(&mut ack)?;
    }

    /// Sends a ping to the server, to keep it from reaping connections that haven't
    /// sent a message within its keep alive window
    ///
    /// # Errors
    ///
    /// The send to the server fails
    #[inline]
    pub fn ping(&self) -> Result<(), Error> {
        self.send_message_impl(super::PING, &[])?;

        let mut pong = [0u8; std::mem::size_of::<Header>()];
        self.socket.recv(&mut pong)?;

        let header = Header::from_bytes(&pong);

        if header.filter(|hdr| hdr.kind == super::PONG).is_none() {
            Err(Error::ProtocolError("received invalid response to ping"))
        } else {
            Ok(())
        }
    }

    fn send_message_impl(&self, kind: u32, buf: &[u8]) -> Result<(), Error> {
        let header = Header {
            kind,
            size: buf.len() as u32,
        };

        let io_bufs = [IoSlice::new(header.as_bytes()), IoSlice::new(buf)];

        self.socket.send_vectored(&io_bufs)?;
        Ok(())
    }
}
