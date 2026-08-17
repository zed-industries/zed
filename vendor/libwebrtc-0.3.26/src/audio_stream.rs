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

use crate::imp::audio_stream as stream_imp;

#[cfg(not(target_arch = "wasm32"))]
pub mod native {
    use std::{
        fmt::{Debug, Formatter},
        pin::Pin,
        task::{Context, Poll},
    };

    use livekit_runtime::Stream;

    use super::stream_imp;
    use crate::{audio_frame::AudioFrame, audio_track::RtcAudioTrack};

    pub struct NativeAudioStream {
        pub(crate) handle: stream_imp::NativeAudioStream,
    }

    impl Debug for NativeAudioStream {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            f.debug_struct("NativeAudioStream").field("track", &self.track()).finish()
        }
    }

    impl NativeAudioStream {
        pub fn new(audio_track: RtcAudioTrack, sample_rate: i32, num_channels: i32) -> Self {
            Self {
                handle: stream_imp::NativeAudioStream::new(audio_track, sample_rate, num_channels),
            }
        }

        pub fn track(&self) -> RtcAudioTrack {
            self.handle.track()
        }

        pub fn close(&mut self) {
            self.handle.close()
        }
    }

    impl Stream for NativeAudioStream {
        type Item = AudioFrame<'static>;

        fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
            Pin::new(&mut self.get_mut().handle).poll_next(cx)
        }
    }
}
