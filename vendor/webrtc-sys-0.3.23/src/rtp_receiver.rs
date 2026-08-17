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
        include!("livekit/helper.h");
        include!("livekit/media_stream.h");

        type MediaType = crate::webrtc::ffi::MediaType;
        type RtpParameters = crate::rtp_parameters::ffi::RtpParameters;
        type MediaStreamPtr = crate::helper::ffi::MediaStreamPtr;
        type MediaStreamTrack = crate::media_stream::ffi::MediaStreamTrack;
        type MediaStream = crate::media_stream::ffi::MediaStream;
    }

    unsafe extern "C++" {
        include!("livekit/rtp_receiver.h");

        type RtpReceiver;

        fn track(self: &RtpReceiver) -> SharedPtr<MediaStreamTrack>;
        fn get_stats(
            self: &RtpReceiver,
            ctx: Box<ReceiverContext>,
            on_stats: fn(ctx: Box<ReceiverContext>, json: String),
        );
        fn stream_ids(self: &RtpReceiver) -> Vec<String>;
        fn streams(self: &RtpReceiver) -> Vec<MediaStreamPtr>;
        fn media_type(self: &RtpReceiver) -> MediaType;
        fn id(self: &RtpReceiver) -> String;
        fn get_parameters(self: &RtpReceiver) -> RtpParameters;
        fn set_jitter_buffer_minimum_delay(self: &RtpReceiver, is_some: bool, delay_seconds: f64);

        fn _shared_rtp_receiver() -> SharedPtr<RtpReceiver>;
    }

    extern "Rust" {
        type ReceiverContext;
    }
}

pub struct ReceiverContext(pub Box<dyn Any + Send>);

impl_thread_safety!(ffi::RtpReceiver, Send + Sync);
