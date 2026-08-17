// Copyright 2025 LiveKit, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{
    error::Error,
    fmt::{Display, Formatter},
};

// cxx doesn't support custom Exception type, so we serialize RtcError inside the cxx::Exception
// "what" string

#[cxx::bridge(namespace = "livekit_ffi")]
pub mod ffi {
    #[derive(Debug)]
    #[repr(i32)]
    pub enum RtcErrorType {
        None,
        UnsupportedOperation,
        UnsupportedParameter,
        InvalidParameter,
        InvalidRange,
        SyntaxError,
        InvalidState,
        InvalidModification,
        NetworkError,
        ResourceExhausted,
        InternalError,
        OperationErrorWithData,
    }

    #[derive(Debug)]
    #[repr(i32)]
    pub enum RtcErrorDetailType {
        None,
        DataChannelFailure,
        DtlsFailure,
        FingerprintFailure,
        SctpFailure,
        SdpSyntaxError,
        HardwareEncoderNotAvailable,
        HardwareEncoderError,
    }

    #[derive(Debug)]
    pub struct RtcError {
        pub error_type: RtcErrorType,
        pub message: String,
        pub error_detail: RtcErrorDetailType,
        // cxx doesn't support the Option trait
        pub has_sctp_cause_code: bool,
        pub sctp_cause_code: u16,
    }
}

impl ffi::RtcError {
    /// Decodes the hex-encoded error produced by the C++ `serialize_error`.
    ///
    /// `value` is the `what()` string of a [`cxx::Exception`], which may carry any
    /// exception that crossed the FFI boundary, not only our serialized errors. When
    /// `value` is not in the expected format it is surfaced as an [`InternalError`]
    /// carrying the raw message instead of panicking.
    ///
    /// [`InternalError`]: ffi::RtcErrorType::InternalError
    pub fn from(value: &str) -> Self {
        Self::parse(value).unwrap_or_else(|| Self {
            error_type: ffi::RtcErrorType::InternalError,
            error_detail: ffi::RtcErrorDetailType::None,
            has_sctp_cause_code: false,
            sctp_cause_code: 0,
            message: value.to_string(),
        })
    }

    fn parse(value: &str) -> Option<Self> {
        let error_type = u32::from_str_radix(value.get(0..8)?, 16).ok()?;
        let error_detail = u32::from_str_radix(value.get(8..16)?, 16).ok()?;
        let has_sctp_cause_code = u8::from_str_radix(value.get(16..18)?, 16).ok()?;
        let sctp_cause_code = u16::from_str_radix(value.get(18..22)?, 16).ok()?;
        let message = value.get(22..)?; // msg isn't encoded

        Some(Self {
            error_type: error_type_from_repr(error_type)?,
            error_detail: error_detail_from_repr(error_detail)?,
            sctp_cause_code,
            has_sctp_cause_code: has_sctp_cause_code == 1,
            message: message.to_string(),
        })
    }

    pub fn ok(&self) -> bool {
        self.error_type == ffi::RtcErrorType::None
    }
}

fn error_type_from_repr(value: u32) -> Option<ffi::RtcErrorType> {
    Some(match value {
        0 => ffi::RtcErrorType::None,
        1 => ffi::RtcErrorType::UnsupportedOperation,
        2 => ffi::RtcErrorType::UnsupportedParameter,
        3 => ffi::RtcErrorType::InvalidParameter,
        4 => ffi::RtcErrorType::InvalidRange,
        5 => ffi::RtcErrorType::SyntaxError,
        6 => ffi::RtcErrorType::InvalidState,
        7 => ffi::RtcErrorType::InvalidModification,
        8 => ffi::RtcErrorType::NetworkError,
        9 => ffi::RtcErrorType::ResourceExhausted,
        10 => ffi::RtcErrorType::InternalError,
        11 => ffi::RtcErrorType::OperationErrorWithData,
        _ => return None,
    })
}

fn error_detail_from_repr(value: u32) -> Option<ffi::RtcErrorDetailType> {
    Some(match value {
        0 => ffi::RtcErrorDetailType::None,
        1 => ffi::RtcErrorDetailType::DataChannelFailure,
        2 => ffi::RtcErrorDetailType::DtlsFailure,
        3 => ffi::RtcErrorDetailType::FingerprintFailure,
        4 => ffi::RtcErrorDetailType::SctpFailure,
        5 => ffi::RtcErrorDetailType::SdpSyntaxError,
        6 => ffi::RtcErrorDetailType::HardwareEncoderNotAvailable,
        7 => ffi::RtcErrorDetailType::HardwareEncoderError,
        _ => return None,
    })
}

impl Error for ffi::RtcError {}

impl Display for ffi::RtcError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "RtcError occurred {:?}: {}", self.error_type, self.message)
    }
}

#[cfg(test)]
mod tests {
    use crate::rtc_error::ffi::{RtcError, RtcErrorDetailType, RtcErrorType};

    #[cxx::bridge(namespace = "livekit_ffi")]
    pub mod ffi {
        unsafe extern "C++" {
            include!("livekit/rtc_error.h");

            fn serialize_deserialize() -> String;
            fn throw_error() -> Result<()>;
        }
    }

    #[test]
    fn serialize_deserialize() {
        let str = ffi::serialize_deserialize();
        let error = RtcError::from(&str);

        assert_eq!(error.error_type, RtcErrorType::InternalError);
        assert_eq!(error.error_detail, RtcErrorDetailType::DataChannelFailure);
        assert!(error.has_sctp_cause_code);
        assert_eq!(error.sctp_cause_code, 24);
        assert_eq!(error.message, "this is not a test, I repeat, this is not a test");
    }

    #[test]
    fn throw_error() {
        let exc: cxx::Exception = ffi::throw_error().err().unwrap();
        let error = RtcError::from(exc.what());

        assert_eq!(error.error_type, RtcErrorType::InvalidModification);
        assert_eq!(error.error_detail, RtcErrorDetailType::None);
        assert!(!error.has_sctp_cause_code);
        assert_eq!(error.sctp_cause_code, 0);
        assert_eq!(error.message, "exception is thrown!");
    }
}
