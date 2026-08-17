use std::error::Error;
use std::fmt::{Display, Formatter};

/// The requested host, although supported on this platform, is unavailable.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct HostUnavailable;

impl Display for HostUnavailable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("the requested host is unavailable")
    }
}

impl Error for HostUnavailable {}

/// Some error has occurred that is specific to the backend from which it was produced.
///
/// This error is often used as a catch-all in cases where:
///
/// - It is unclear exactly what error might be produced by the backend API.
/// - It does not make sense to add a variant to the enclosing error type.
/// - No error was expected to occur at all, but we return an error to avoid the possibility of a
///   `panic!` caused by some unforeseen or unknown reason.
///
/// **Note:** If you notice a `BackendSpecificError` that you believe could be better handled in a
/// cross-platform manner, please create an issue at <https://github.com/RustAudio/cpal/issues>
/// with details about your use case, the backend you're using, and the error message. Or submit
/// a pull request with a patch that adds the necessary error variant to the appropriate error enum.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BackendSpecificError {
    pub description: String,
}

impl Display for BackendSpecificError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A backend-specific error has occurred: {}",
            self.description
        )
    }
}

impl Error for BackendSpecificError {}

/// An error that might occur while attempting to enumerate the available devices on a system.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DevicesError {
    /// See the [`BackendSpecificError`] docs for more information about this error variant.
    BackendSpecific { err: BackendSpecificError },
}

impl Display for DevicesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BackendSpecific { err } => err.fmt(f),
        }
    }
}

impl Error for DevicesError {}

impl From<BackendSpecificError> for DevicesError {
    fn from(err: BackendSpecificError) -> Self {
        Self::BackendSpecific { err }
    }
}

/// An error that may occur while attempting to retrieve a device ID.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeviceIdError {
    /// See the [`BackendSpecificError`] docs for more information about this error variant.
    BackendSpecific {
        err: BackendSpecificError,
    },
    UnsupportedPlatform,
}

impl Display for DeviceIdError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BackendSpecific { err } => err.fmt(f),
            Self::UnsupportedPlatform => f.write_str("Device IDs are unsupported for this OS"),
        }
    }
}

impl Error for DeviceIdError {}

impl From<BackendSpecificError> for DeviceIdError {
    fn from(err: BackendSpecificError) -> Self {
        Self::BackendSpecific { err }
    }
}

/// An error that may occur while attempting to retrieve a device name.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DeviceNameError {
    /// See the [`BackendSpecificError`] docs for more information about this error variant.
    BackendSpecific { err: BackendSpecificError },
}

impl Display for DeviceNameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BackendSpecific { err } => err.fmt(f),
        }
    }
}

impl Error for DeviceNameError {}

impl From<BackendSpecificError> for DeviceNameError {
    fn from(err: BackendSpecificError) -> Self {
        Self::BackendSpecific { err }
    }
}

/// Error that can happen when enumerating the list of supported formats.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SupportedStreamConfigsError {
    /// The device no longer exists. This can happen if the device is disconnected while the
    /// program is running.
    DeviceNotAvailable,
    /// We called something the C-Layer did not understand
    InvalidArgument,
    /// See the [`BackendSpecificError`] docs for more information about this error variant.
    BackendSpecific { err: BackendSpecificError },
}

impl Display for SupportedStreamConfigsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BackendSpecific { err } => err.fmt(f),
            Self::DeviceNotAvailable => f.write_str("The requested device is no longer available. For example, it has been unplugged."),
            Self::InvalidArgument => f.write_str("Invalid argument passed to the backend. For example, this happens when trying to read capture capabilities when the device does not support it.")
        }
    }
}

impl Error for SupportedStreamConfigsError {}

impl From<BackendSpecificError> for SupportedStreamConfigsError {
    fn from(err: BackendSpecificError) -> Self {
        Self::BackendSpecific { err }
    }
}

/// May occur when attempting to request the default input or output stream format from a [`Device`](crate::Device).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DefaultStreamConfigError {
    /// The device no longer exists. This can happen if the device is disconnected while the
    /// program is running.
    DeviceNotAvailable,
    /// Returned if e.g. the default input format was requested on an output-only audio device.
    StreamTypeNotSupported,
    /// See the [`BackendSpecificError`] docs for more information about this error variant.
    BackendSpecific { err: BackendSpecificError },
}

impl Display for DefaultStreamConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BackendSpecific { err } => err.fmt(f),
            Self::DeviceNotAvailable => f.write_str(
                "The requested device is no longer available. For example, it has been unplugged.",
            ),
            Self::StreamTypeNotSupported => {
                f.write_str("The requested stream type is not supported by the device.")
            }
        }
    }
}

impl Error for DefaultStreamConfigError {}

impl From<BackendSpecificError> for DefaultStreamConfigError {
    fn from(err: BackendSpecificError) -> Self {
        Self::BackendSpecific { err }
    }
}
/// Error that can happen when creating a [`Stream`](crate::Stream).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BuildStreamError {
    /// The device no longer exists. This can happen if the device is disconnected while the
    /// program is running.
    DeviceNotAvailable,
    /// The specified stream configuration is not supported.
    StreamConfigNotSupported,
    /// We called something the C-Layer did not understand
    ///
    /// On ALSA device functions called with a feature they do not support will yield this. E.g.
    /// Trying to use capture capabilities on an output only format yields this.
    InvalidArgument,
    /// Occurs if adding a new Stream ID would cause an integer overflow.
    StreamIdOverflow,
    /// See the [`BackendSpecificError`] docs for more information about this error variant.
    BackendSpecific { err: BackendSpecificError },
}

impl Display for BuildStreamError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BackendSpecific { err } => err.fmt(f),
            Self::DeviceNotAvailable => f.write_str(
                "The requested device is no longer available. For example, it has been unplugged.",
            ),
            Self::StreamConfigNotSupported => {
                f.write_str("The requested stream configuration is not supported by the device.")
            }
            Self::InvalidArgument => f.write_str(
                "The requested device does not support this capability (invalid argument)",
            ),
            Self::StreamIdOverflow => f.write_str("Adding a new stream ID would cause an overflow"),
        }
    }
}

impl Error for BuildStreamError {}

impl From<BackendSpecificError> for BuildStreamError {
    fn from(err: BackendSpecificError) -> Self {
        Self::BackendSpecific { err }
    }
}

/// Errors that might occur when calling [`Stream::play()`](crate::traits::StreamTrait::play).
///
/// As of writing this, only macOS may immediately return an error while calling this method. This
/// is because both the alsa and wasapi backends only enqueue these commands and do not process
/// them immediately.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PlayStreamError {
    /// The device associated with the stream is no longer available.
    DeviceNotAvailable,
    /// See the [`BackendSpecificError`] docs for more information about this error variant.
    BackendSpecific { err: BackendSpecificError },
}

impl Display for PlayStreamError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BackendSpecific { err } => err.fmt(f),
            Self::DeviceNotAvailable => {
                f.write_str("the device associated with the stream is no longer available")
            }
        }
    }
}

impl Error for PlayStreamError {}

impl From<BackendSpecificError> for PlayStreamError {
    fn from(err: BackendSpecificError) -> Self {
        Self::BackendSpecific { err }
    }
}

/// Errors that might occur when calling [`Stream::pause()`](crate::traits::StreamTrait::pause).
///
/// As of writing this, only macOS may immediately return an error while calling this method. This
/// is because both the alsa and wasapi backends only enqueue these commands and do not process
/// them immediately.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PauseStreamError {
    /// The device associated with the stream is no longer available.
    DeviceNotAvailable,
    /// See the [`BackendSpecificError`] docs for more information about this error variant.
    BackendSpecific { err: BackendSpecificError },
}

impl Display for PauseStreamError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BackendSpecific { err } => err.fmt(f),
            Self::DeviceNotAvailable => {
                f.write_str("the device associated with the stream is no longer available")
            }
        }
    }
}

impl Error for PauseStreamError {}

impl From<BackendSpecificError> for PauseStreamError {
    fn from(err: BackendSpecificError) -> Self {
        Self::BackendSpecific { err }
    }
}

/// Errors that might occur while a stream is running.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StreamError {
    /// The device no longer exists. This can happen if the device is disconnected while the
    /// program is running.
    DeviceNotAvailable,

    /// The stream configuration is no longer valid and must be rebuilt.
    StreamInvalidated,

    /// Buffer underrun or overrun occurred, causing a potential audio glitch.
    BufferUnderrun,

    /// See the [`BackendSpecificError`] docs for more information about this error variant.
    BackendSpecific { err: BackendSpecificError },
}

impl Display for StreamError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BackendSpecific { err } => err.fmt(f),
            Self::StreamInvalidated => {
                f.write_str("The stream configuration is no longer valid and must be rebuilt.")
            }
            Self::BufferUnderrun => f.write_str("Buffer underrun/overrun occurred."),
            Self::DeviceNotAvailable => f.write_str(
                "The requested device is no longer available. For example, it has been unplugged.",
            ),
        }
    }
}

impl Error for StreamError {}

impl From<BackendSpecificError> for StreamError {
    fn from(err: BackendSpecificError) -> Self {
        Self::BackendSpecific { err }
    }
}
