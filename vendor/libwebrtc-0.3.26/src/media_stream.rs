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

use std::fmt::Debug;

use crate::{audio_track::RtcAudioTrack, imp::media_stream as imp_ms, video_track::RtcVideoTrack};

#[derive(Clone)]
pub struct MediaStream {
    pub(crate) handle: imp_ms::MediaStream,
}

impl MediaStream {
    pub fn id(&self) -> String {
        self.handle.id()
    }

    pub fn audio_tracks(&self) -> Vec<RtcAudioTrack> {
        self.handle.audio_tracks()
    }

    pub fn video_tracks(&self) -> Vec<RtcVideoTrack> {
        self.handle.video_tracks()
    }
}

impl Debug for MediaStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MediaStream")
            .field("id", &self.id())
            .field("audio_tracks", &self.audio_tracks())
            .field("video_tracks", &self.video_tracks())
            .finish()
    }
}
