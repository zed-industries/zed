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

use crate::imp::video_stream as stream_imp;

// There is no shared sink between native and web platforms.
// Each platform requires different configuration (e.g: WebGlContext, ..)

#[cfg(not(target_arch = "wasm32"))]
pub mod native {
    use std::{
        fmt::Debug,
        pin::Pin,
        task::{Context, Poll},
    };

    use super::stream_imp;
    use crate::{video_frame::BoxVideoFrame, video_track::RtcVideoTrack};
    use livekit_runtime::Stream;

    pub struct NativeVideoStream {
        pub(crate) handle: stream_imp::NativeVideoStream,
    }

    impl Debug for NativeVideoStream {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct("NativeVideoStream").field("track", &self.track()).finish()
        }
    }

    impl NativeVideoStream {
        pub fn new(video_track: RtcVideoTrack) -> Self {
            Self { handle: stream_imp::NativeVideoStream::new(video_track) }
        }

        pub fn track(&self) -> RtcVideoTrack {
            self.handle.track()
        }

        pub fn close(&mut self) {
            self.handle.close();
        }
    }

    impl Stream for NativeVideoStream {
        type Item = BoxVideoFrame;

        fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
            Pin::new(&mut self.get_mut().handle).poll_next(cx)
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub mod web {}
