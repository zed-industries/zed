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

use cxx::SharedPtr;
use webrtc_sys::{rtc_error as sys_err, rtp_transceiver as sys_rt, webrtc as sys_webrtc};

use crate::{
    imp::{rtp_receiver::RtpReceiver, rtp_sender::RtpSender},
    rtp_parameters::RtpCodecCapability,
    rtp_receiver, rtp_sender,
    rtp_transceiver::{RtpTransceiverDirection, RtpTransceiverInit},
    RtcError,
};

impl From<sys_webrtc::ffi::RtpTransceiverDirection> for RtpTransceiverDirection {
    fn from(value: sys_webrtc::ffi::RtpTransceiverDirection) -> Self {
        match value {
            sys_webrtc::ffi::RtpTransceiverDirection::SendRecv => Self::SendRecv,
            sys_webrtc::ffi::RtpTransceiverDirection::SendOnly => Self::SendOnly,
            sys_webrtc::ffi::RtpTransceiverDirection::RecvOnly => Self::RecvOnly,
            sys_webrtc::ffi::RtpTransceiverDirection::Inactive => Self::Inactive,
            sys_webrtc::ffi::RtpTransceiverDirection::Stopped => Self::Stopped,
            _ => panic!("unknown RtpTransceiverDirection"),
        }
    }
}

impl From<RtpTransceiverDirection> for sys_webrtc::ffi::RtpTransceiverDirection {
    fn from(value: RtpTransceiverDirection) -> Self {
        match value {
            RtpTransceiverDirection::SendRecv => Self::SendRecv,
            RtpTransceiverDirection::SendOnly => Self::SendOnly,
            RtpTransceiverDirection::RecvOnly => Self::RecvOnly,
            RtpTransceiverDirection::Inactive => Self::Inactive,
            RtpTransceiverDirection::Stopped => Self::Stopped,
        }
    }
}

impl From<RtpTransceiverInit> for sys_rt::ffi::RtpTransceiverInit {
    fn from(value: RtpTransceiverInit) -> Self {
        Self {
            direction: value.direction.into(),
            stream_ids: value.stream_ids,
            send_encodings: value.send_encodings.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Clone)]
pub struct RtpTransceiver {
    pub(crate) sys_handle: SharedPtr<sys_rt::ffi::RtpTransceiver>,
}

impl RtpTransceiver {
    pub fn mid(&self) -> Option<String> {
        self.sys_handle.mid().ok()
    }

    pub fn current_direction(&self) -> Option<RtpTransceiverDirection> {
        self.sys_handle.current_direction().ok().map(Into::into)
    }

    pub fn direction(&self) -> RtpTransceiverDirection {
        self.sys_handle.direction().into()
    }

    pub fn sender(&self) -> rtp_sender::RtpSender {
        rtp_sender::RtpSender { handle: RtpSender { sys_handle: self.sys_handle.sender() } }
    }

    pub fn receiver(&self) -> rtp_receiver::RtpReceiver {
        rtp_receiver::RtpReceiver { handle: RtpReceiver { sys_handle: self.sys_handle.receiver() } }
    }

    pub fn set_codec_preferences(&self, codecs: Vec<RtpCodecCapability>) -> Result<(), RtcError> {
        self.sys_handle
            .set_codec_preferences(codecs.into_iter().map(Into::into).collect())
            .map_err(|e| sys_err::ffi::RtcError::from(e.what()).into())
    }

    pub fn stop(&self) -> Result<(), RtcError> {
        self.sys_handle.stop_standard().map_err(|e| sys_err::ffi::RtcError::from(e.what()).into())
    }
}
