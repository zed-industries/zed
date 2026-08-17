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

use std::any::Any;

use crate::impl_thread_safety;

#[cxx::bridge(namespace = "livekit_ffi")]
pub mod ffi {

    extern "C++" {
        include!("livekit/webrtc.h");
        include!("livekit/rtp_parameters.h");
        include!("livekit/media_stream.h");

        type MediaType = crate::webrtc::ffi::MediaType;
        type RtpEncodingParameters = crate::rtp_parameters::ffi::RtpEncodingParameters;
        type RtpParameters = crate::rtp_parameters::ffi::RtpParameters;
        type MediaStreamTrack = crate::media_stream::ffi::MediaStreamTrack;
    }

    unsafe extern "C++" {
        include!("livekit/rtp_sender.h");

        type RtpSender;

        fn set_track(self: &RtpSender, track: SharedPtr<MediaStreamTrack>) -> bool;
        fn track(self: &RtpSender) -> SharedPtr<MediaStreamTrack>;
        fn get_stats(
            self: &RtpSender,
            ctx: Box<SenderContext>,
            on_stats: fn(ctx: Box<SenderContext>, json: String),
        );
        fn ssrc(self: &RtpSender) -> u32;
        fn media_type(self: &RtpSender) -> MediaType;
        fn id(self: &RtpSender) -> String;
        fn stream_ids(self: &RtpSender) -> Vec<String>;
        fn set_streams(self: &RtpSender, stream_ids: &Vec<String>);
        fn init_send_encodings(self: &RtpSender) -> Vec<RtpEncodingParameters>;
        fn get_parameters(self: &RtpSender) -> RtpParameters;
        fn set_parameters(self: &RtpSender, parameters: RtpParameters) -> Result<()>;

        fn _shared_rtp_sender() -> SharedPtr<RtpSender>;
    }

    extern "Rust" {
        type SenderContext;
    }
}

#[repr(transparent)]
pub struct SenderContext(pub Box<dyn Any + Send>);

impl_thread_safety!(ffi::RtpSender, Send + Sync);
