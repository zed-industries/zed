//! This module is an attempt at rustifying the OSStatus result.

pub use self::audio::Error as AudioError;
pub use self::audio_codec::Error as AudioCodecError;
pub use self::audio_format::Error as AudioFormatError;
pub use self::audio_unit::Error as AudioUnitError;
use sys::OSStatus;

pub mod audio {
    use sys::OSStatus;

    #[derive(Copy, Clone, Debug)]
    pub enum Error {
        Unimplemented = -4,
        FileNotFound = -43,
        FilePermission = -54,
        TooManyFilesOpen = -42,
        BadFilePath = 561017960,
        Param = -50,
        MemFull = -108,
        Unknown,
    }

    impl Error {
        pub fn from_os_status(os_status: OSStatus) -> Result<(), Error> {
            match os_status {
                0 => Ok(()),
                -4 => Err(Error::Unimplemented),
                -43 => Err(Error::FileNotFound),
                -54 => Err(Error::FilePermission),
                -42 => Err(Error::TooManyFilesOpen),
                561017960 => Err(Error::BadFilePath),
                -50 => Err(Error::Param),
                -108 => Err(Error::MemFull),
                _ => Err(Error::Unknown),
            }
        }

        pub fn as_os_status(&self) -> OSStatus {
            *self as OSStatus
        }
    }

    impl std::error::Error for Error {}

    impl ::std::fmt::Display for Error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
            let description = match *self {
                Error::Unimplemented => "Unimplemented",
                Error::FileNotFound => "File not found",
                Error::FilePermission => "File permission",
                Error::TooManyFilesOpen => "Too many files open",
                Error::BadFilePath => "Bad file path",
                Error::Param => "Param",
                Error::MemFull => "Memory full",
                Error::Unknown => "An unknown error occurred",
            };
            write!(f, "{}", description)
        }
    }
}

pub mod audio_codec {
    use sys::OSStatus;

    #[derive(Copy, Clone, Debug)]
    pub enum Error {
        Unspecified = 2003329396,
        UnknownProperty = 2003332927,
        BadPropertySize = 561211770,
        IllegalOperation = 1852797029,
        UnsupportedFormat = 560226676,
        State = 561214580,
        NotEnoughBufferSpace = 560100710,
        Unknown,
    }

    impl Error {
        pub fn from_os_status(os_status: OSStatus) -> Result<(), Error> {
            match os_status {
                0 => Ok(()),
                2003329396 => Err(Error::Unspecified),
                2003332927 => Err(Error::UnknownProperty),
                561211770 => Err(Error::BadPropertySize),
                1852797029 => Err(Error::IllegalOperation),
                560226676 => Err(Error::UnsupportedFormat),
                561214580 => Err(Error::State),
                560100710 => Err(Error::NotEnoughBufferSpace),
                _ => Err(Error::Unknown),
            }
        }

        pub fn as_os_status(&self) -> OSStatus {
            *self as OSStatus
        }
    }

    impl std::error::Error for Error {}

    impl ::std::fmt::Display for Error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
            let description = match *self {
                Error::Unspecified => "Unspecified",
                Error::UnknownProperty => "Unknown property",
                Error::BadPropertySize => "Bad property size",
                Error::IllegalOperation => "Illegal operation",
                Error::UnsupportedFormat => "Unsupported format",
                Error::State => "State",
                Error::NotEnoughBufferSpace => "Not enough buffer space",
                Error::Unknown => "Unknown error occurred",
            };
            write!(f, "{}", description)
        }
    }
}

pub mod audio_format {
    use sys::OSStatus;

    // TODO: Finish implementing these values.
    #[derive(Copy, Clone, Debug)]
    pub enum Error {
        Unspecified,                        // 'what'
        UnsupportedProperty,                // 'prop'
        BadPropertySize,                    // '!siz'
        BadSpecifierSize,                   // '!spc'
        UnsupportedDataFormat = 1718449215, // 'fmt?'
        UnknownFormat,                      // '!fmt'
        Unknown,                            //
    }

    impl Error {
        pub fn from_os_status(os_status: OSStatus) -> Result<(), Error> {
            match os_status {
                0 => Ok(()),
                1718449215 => Err(Error::UnsupportedDataFormat),
                _ => Err(Error::Unknown),
            }
        }

        pub fn as_os_status(&self) -> OSStatus {
            *self as OSStatus
        }
    }

    impl std::error::Error for Error {}

    impl ::std::fmt::Display for Error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
            let description = match *self {
                Error::Unspecified => "An unspecified error",
                Error::UnsupportedProperty => "The specified property is not supported",
                Error::BadPropertySize => "Bad property size",
                Error::BadSpecifierSize => "Bad specifier size",
                Error::UnsupportedDataFormat => "The specified data format is not supported",
                Error::UnknownFormat => "The specified data format is not a known format",
                Error::Unknown => "Unknown error occurred",
            };
            write!(f, "{}", description)
        }
    }
}

pub mod audio_unit {
    use sys::OSStatus;

    #[derive(Copy, Clone, Debug)]
    pub enum Error {
        InvalidProperty = -10879,
        InvalidParameter = -10878,
        InvalidElement = -10877,
        NoConnection = -10876,
        FailedInitialization = -10875,
        TooManyFramesToProcess = -10874,
        InvalidFile = -10871,
        FormatNotSupported = -10868,
        Uninitialized = -10867,
        InvalidScope = -10866,
        PropertyNotWritable = -10865,
        CannotDoInCurrentContext = -10863,
        InvalidPropertyValue = -10851,
        PropertyNotInUse = -10850,
        Initialized = -10849,
        InvalidOfflineRender = -10848,
        Unauthorized = -10847,
        Unknown,
    }

    impl Error {
        pub fn from_os_status(os_status: OSStatus) -> Result<(), Error> {
            match os_status {
                -10879 => Err(Error::InvalidProperty),
                -10878 => Err(Error::InvalidParameter),
                -10877 => Err(Error::InvalidElement),
                -10876 => Err(Error::NoConnection),
                -10875 => Err(Error::FailedInitialization),
                -10874 => Err(Error::TooManyFramesToProcess),
                -10871 => Err(Error::InvalidFile),
                -10868 => Err(Error::FormatNotSupported),
                -10867 => Err(Error::Uninitialized),
                -10866 => Err(Error::InvalidScope),
                -10865 => Err(Error::PropertyNotWritable),
                -10863 => Err(Error::CannotDoInCurrentContext),
                -10851 => Err(Error::InvalidPropertyValue),
                -10850 => Err(Error::PropertyNotInUse),
                -10849 => Err(Error::Initialized),
                -10848 => Err(Error::InvalidOfflineRender),
                -10847 => Err(Error::Unauthorized),
                _ => Err(Error::Unknown),
            }
        }

        pub fn as_os_status(&self) -> OSStatus {
            *self as OSStatus
        }
    }

    impl std::error::Error for Error {}

    impl ::std::fmt::Display for Error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
            let description = match *self {
                Error::InvalidProperty => "Invalid property",
                Error::InvalidParameter => "Invalid parameter",
                Error::InvalidElement => "Invalid element",
                Error::NoConnection => "No connection",
                Error::FailedInitialization => "Failed initialization",
                Error::TooManyFramesToProcess => "Too many frames to process",
                Error::InvalidFile => "Invalid file",
                Error::FormatNotSupported => "Format not supported",
                Error::Uninitialized => "Uninitialized",
                Error::InvalidScope => "Invalid scope",
                Error::PropertyNotWritable => "Property not writable",
                Error::CannotDoInCurrentContext => "Cannot do in current context",
                Error::InvalidPropertyValue => "Invalid property value",
                Error::PropertyNotInUse => "Property not in use",
                Error::Initialized => "Initialized",
                Error::InvalidOfflineRender => "Invalid offline render",
                Error::Unauthorized => "Unauthorized",
                Error::Unknown => "Unknown error occurred",
            };
            write!(f, "{}", description)
        }
    }
}

/// A wrapper around all possible Core Audio errors.
#[derive(Copy, Clone, Debug)]
pub enum Error {
    Unspecified,
    SystemSoundClientMessageTimedOut,
    NoMatchingDefaultAudioUnitFound,
    RenderCallbackBufferFormatDoesNotMatchAudioUnitStreamFormat,
    NoKnownSubtype,
    NonInterleavedInputOnlySupportsMono,
    UnsupportedSampleRate,
    UnsupportedStreamFormat,
    Audio(AudioError),
    AudioCodec(AudioCodecError),
    AudioFormat(AudioFormatError),
    AudioUnit(AudioUnitError),
    Unknown(OSStatus),
}

impl Error {
    /// Convert an OSStatus to a std Rust Result.
    pub fn from_os_status(os_status: OSStatus) -> Result<(), Error> {
        match os_status {
            0 => Ok(()),
            -1500 => Err(Error::Unspecified),
            -1501 => Err(Error::SystemSoundClientMessageTimedOut),
            _ => {
                match AudioError::from_os_status(os_status) {
                    Ok(()) => return Ok(()),
                    Err(AudioError::Unknown) => (),
                    Err(err) => return Err(Error::Audio(err)),
                }
                match AudioCodecError::from_os_status(os_status) {
                    Ok(()) => return Ok(()),
                    Err(AudioCodecError::Unknown) => (),
                    Err(err) => return Err(Error::AudioCodec(err)),
                }
                match AudioFormatError::from_os_status(os_status) {
                    Ok(()) => return Ok(()),
                    Err(AudioFormatError::Unknown) => (),
                    Err(err) => return Err(Error::AudioFormat(err)),
                }
                match AudioUnitError::from_os_status(os_status) {
                    Ok(()) => return Ok(()),
                    Err(AudioUnitError::Unknown) => (),
                    Err(err) => return Err(Error::AudioUnit(err)),
                }
                Err(Error::Unknown(os_status))
            }
        }
    }

    /// Convert an Error to an OSStatus.
    pub fn as_os_status(&self) -> OSStatus {
        match *self {
            Error::Unspecified => -1500,
            Error::NoMatchingDefaultAudioUnitFound => -1500,
            Error::RenderCallbackBufferFormatDoesNotMatchAudioUnitStreamFormat => -1500,
            Error::SystemSoundClientMessageTimedOut => -1501,
            Error::Audio(err) => err as OSStatus,
            Error::AudioCodec(err) => err as OSStatus,
            Error::AudioUnit(err) => err as OSStatus,
            _ => -1500,
        }
    }
}

impl std::error::Error for Error {}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            Error::Unspecified => write!(f, "An unspecified error has occurred"),
            Error::NoMatchingDefaultAudioUnitFound => write!(f, "No matching default audio unit found"),
            Error::RenderCallbackBufferFormatDoesNotMatchAudioUnitStreamFormat =>
                write!(f, "The given render callback buffer format does not match the `AudioUnit` `StreamFormat`"),
            Error::SystemSoundClientMessageTimedOut => write!(f, "The system sound client message timed out"),
            Error::NoKnownSubtype => write!(f, "The type has no known subtypes"),
            Error::NonInterleavedInputOnlySupportsMono => write!(f, "In non-interleaved mode input only supports one channel"),
            Error::UnsupportedSampleRate => write!(f, "The requested sample rate is not available"),
            Error::UnsupportedStreamFormat => write!(f, "The requested stream format is not available"),
            Error::Audio(ref err) => write!(f, "{}", err),
            Error::AudioCodec(ref err) => write!(f, "{}", err),
            Error::AudioFormat(ref err) => write!(f, "{}", err),
            Error::AudioUnit(ref err) => write!(f, "{}", err),
            Error::Unknown(_) => write!(f, "An unknown error unknown to the coreaudio-rs API occurred"),
        }
    }
}
