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

use std::{fmt::Debug, sync::Arc};

use libwebrtc::{prelude::*, stats::RtcStats};
use livekit_protocol::{self as proto, AudioTrackFeature};

use super::{remote_track, TrackInner};
use crate::prelude::*;

#[derive(Clone)]
pub struct RemoteAudioTrack {
    inner: Arc<TrackInner>,
}

impl Debug for RemoteAudioTrack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RemoteAudioTrack")
            .field("sid", &self.sid())
            .field("name", &self.name())
            .field("source", &self.source())
            .finish()
    }
}

impl RemoteAudioTrack {
    pub(crate) fn new(sid: TrackSid, name: String, rtc_track: RtcAudioTrack) -> Self {
        Self {
            inner: Arc::new(super::new_inner(
                sid,
                name,
                TrackKind::Audio,
                MediaStreamTrack::Audio(rtc_track),
            )),
        }
    }

    pub fn sid(&self) -> TrackSid {
        self.inner.info.read().sid.clone()
    }

    pub fn name(&self) -> String {
        self.inner.info.read().name.clone()
    }

    pub fn kind(&self) -> TrackKind {
        self.inner.info.read().kind
    }

    pub fn source(&self) -> TrackSource {
        self.inner.info.read().source
    }

    pub fn stream_state(&self) -> StreamState {
        self.inner.info.read().stream_state
    }

    pub fn is_enabled(&self) -> bool {
        self.inner.rtc_track.enabled()
    }

    pub fn enable(&self) {
        self.inner.rtc_track.set_enabled(true);
    }

    pub fn disable(&self) {
        self.inner.rtc_track.set_enabled(false);
    }

    pub fn is_muted(&self) -> bool {
        self.inner.info.read().muted
    }

    pub fn rtc_track(&self) -> RtcAudioTrack {
        if let MediaStreamTrack::Audio(audio) = self.inner.rtc_track.clone() {
            return audio;
        }
        unreachable!();
    }

    pub fn is_remote(&self) -> bool {
        true
    }

    pub async fn get_stats(&self) -> RoomResult<Vec<RtcStats>> {
        super::remote_track::get_stats(&self.inner).await
    }

    pub(crate) fn on_muted(&self, f: impl Fn(Track) + Send + 'static) {
        self.inner.events.lock().muted.replace(Box::new(f));
    }

    pub(crate) fn on_unmuted(&self, f: impl Fn(Track) + Send + 'static) {
        self.inner.events.lock().unmuted.replace(Box::new(f));
    }

    #[allow(dead_code)]
    pub(crate) fn transceiver(&self) -> Option<RtpTransceiver> {
        self.inner.info.read().transceiver.clone()
    }

    #[allow(dead_code)]
    pub(crate) fn set_transceiver(&self, transceiver: Option<RtpTransceiver>) {
        self.inner.info.write().transceiver = transceiver;
    }

    pub(crate) fn update_info(&self, info: proto::TrackInfo) {
        remote_track::update_info(&self.inner, &Track::RemoteAudio(self.clone()), info);
    }
}
