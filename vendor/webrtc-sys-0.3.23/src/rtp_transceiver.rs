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

use crate::impl_thread_safety;

#[cxx::bridge(namespace = "livekit_ffi")]
pub mod ffi {

    #[derive(Debug)]
    pub struct RtpTransceiverInit {
        pub direction: RtpTransceiverDirection,
        pub stream_ids: Vec<String>,
        pub send_encodings: Vec<RtpEncodingParameters>,
    }

    extern "C++" {
        include!("livekit/webrtc.h");
        include!("livekit/rtp_parameters.h");
        include!("livekit/rtp_sender.h");
        include!("livekit/rtp_receiver.h");

        type MediaType = crate::webrtc::ffi::MediaType;
        type RtpTransceiverDirection = crate::webrtc::ffi::RtpTransceiverDirection;
        type RtpEncodingParameters = crate::rtp_parameters::ffi::RtpEncodingParameters;
        type RtpCodecCapability = crate::rtp_parameters::ffi::RtpCodecCapability;
        type RtpHeaderExtensionCapability =
            crate::rtp_parameters::ffi::RtpHeaderExtensionCapability;
        type RtpSender = crate::rtp_sender::ffi::RtpSender;
        type RtpReceiver = crate::rtp_receiver::ffi::RtpReceiver;
        type RtcError = crate::rtc_error::ffi::RtcError;
    }

    unsafe extern "C++" {
        include!("livekit/rtp_transceiver.h");

        type RtpTransceiver;

        fn media_type(self: &RtpTransceiver) -> MediaType;
        fn mid(self: &RtpTransceiver) -> Result<String>;
        fn sender(self: &RtpTransceiver) -> SharedPtr<RtpSender>;
        fn receiver(self: &RtpTransceiver) -> SharedPtr<RtpReceiver>;
        fn stopped(self: &RtpTransceiver) -> bool;
        fn stopping(self: &RtpTransceiver) -> bool;
        fn direction(self: &RtpTransceiver) -> RtpTransceiverDirection;
        fn set_direction(self: &RtpTransceiver, direction: RtpTransceiverDirection) -> Result<()>;
        fn current_direction(self: &RtpTransceiver) -> Result<RtpTransceiverDirection>;
        fn fired_direction(self: &RtpTransceiver) -> Result<RtpTransceiverDirection>;
        fn stop_standard(self: &RtpTransceiver) -> Result<()>;
        fn set_codec_preferences(
            self: &RtpTransceiver,
            codecs: Vec<RtpCodecCapability>,
        ) -> Result<()>;
        fn codec_preferences(self: &RtpTransceiver) -> Vec<RtpCodecCapability>;
        fn header_extensions_to_negotiate(
            self: &RtpTransceiver,
        ) -> Vec<RtpHeaderExtensionCapability>;
        fn negotiated_header_extensions(self: &RtpTransceiver)
            -> Vec<RtpHeaderExtensionCapability>;
        fn set_header_extensions_to_negotiate(
            self: &RtpTransceiver,
            headers: Vec<RtpHeaderExtensionCapability>,
        ) -> Result<()>;

        fn _shared_rtp_transceiver() -> SharedPtr<RtpTransceiver>;
    }
}

impl_thread_safety!(ffi::RtpTransceiver, Send + Sync);
