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

use libwebrtc::prelude::*;
use livekit_protocol as proto;
use livekit_protocol::enum_dispatch;

use super::track_dispatch;
use crate::prelude::*;

#[derive(Clone, Debug)]
pub enum AudioTrack {
    Local(LocalAudioTrack),
    Remote(RemoteAudioTrack),
}

impl AudioTrack {
    track_dispatch!([Local, Remote]);

    pub fn rtc_track(&self) -> RtcAudioTrack {
        match self {
            Self::Local(track) => track.rtc_track(),
            Self::Remote(track) => track.rtc_track(),
        }
    }
}

impl From<AudioTrack> for Track {
    fn from(track: AudioTrack) -> Self {
        match track {
            AudioTrack::Local(track) => Self::LocalAudio(track),
            AudioTrack::Remote(track) => Self::RemoteAudio(track),
        }
    }
}

impl TryFrom<Track> for AudioTrack {
    type Error = &'static str;

    fn try_from(track: Track) -> Result<Self, Self::Error> {
        match track {
            Track::LocalAudio(track) => Ok(Self::Local(track)),
            Track::RemoteAudio(track) => Ok(Self::Remote(track)),
            _ => Err("not an audio track"),
        }
    }
}
