/// Error that can occur when creating a [`crate::Client`] or [`crate::Server`],
/// or generating minidumps
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// The provided socket name or path was invalid
    #[error("the socket name is invalid")]
    InvalidName,
    #[cfg(target_os = "macos")]
    /// The provided socket name or path was invalid as a Mach port name
    #[error("the mach port name is invalid")]
    InvalidPortName,
    /// An error occurred while creating or communicating with a Mach port
    #[cfg(target_os = "macos")]
    #[error("the mach port name is invalid")]
    PortError(#[from] crash_context::ipc::Error),
    /// An I/O or other syscall failed
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// A crash request received by the server could not be processed as the
    /// PID for the client process was unknown or invalid
    #[error("client process requesting crash dump has an unknown or invalid pid")]
    UnknownClientPid,
    /// An error occurred during minidump generation
    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[error(transparent)]
    Writer(Box<minidump_writer::minidump_writer::errors::WriterError>),
    /// An error occurred during minidump generation
    #[cfg(target_os = "windows")]
    #[error(transparent)]
    Writer(#[from] minidump_writer::errors::Error),
    /// An error occurred during minidump generation
    #[cfg(target_os = "macos")]
    #[error(transparent)]
    Writer(#[from] minidump_writer::errors::WriterError),
    /// An error occurred reading or writing binary data
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    #[error(transparent)]
    Scroll(#[from] scroll::Error),
    #[error("protocol error occurred: {0}")]
    ProtocolError(&'static str),
}

#[cfg(any(target_os = "linux", target_os = "android"))]
impl From<minidump_writer::minidump_writer::errors::WriterError> for Error {
    fn from(we: minidump_writer::minidump_writer::errors::WriterError) -> Self {
        Self::Writer(Box::new(we))
    }
}
