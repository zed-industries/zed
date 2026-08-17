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

use livekit_protocol::{self as proto, AudioTrackFeature};
use parking_lot::Mutex;

use super::TrackPublicationInner;
use crate::{e2ee::EncryptionType, options::TrackPublishOptions, prelude::*};

#[derive(Default)]
struct LocalInfo {
    publish_options: Mutex<TrackPublishOptions>,
}

#[derive(Clone)]
pub struct LocalTrackPublication {
    inner: Arc<TrackPublicationInner>,
    local: Arc<LocalInfo>,
}

impl Debug for LocalTrackPublication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalTrackPublication")
            .field("sid", &self.sid())
            .field("name", &self.name())
            .field("kind", &self.kind())
            .finish()
    }
}

impl LocalTrackPublication {
    pub(crate) fn new(info: proto::TrackInfo, track: LocalTrack) -> Self {
        Self {
            inner: super::new_inner(info, Some(track.into())),
            local: Arc::new(LocalInfo::default()),
        }
    }

    pub(crate) fn on_muted(&self, f: impl Fn(TrackPublication) + Send + 'static) {
        *self.inner.events.muted.lock() = Some(Box::new(f));
    }

    pub(crate) fn on_unmuted(&self, f: impl Fn(TrackPublication) + Send + 'static) {
        *self.inner.events.unmuted.lock() = Some(Box::new(f));
    }

    pub(crate) fn set_track(&self, track: Option<Track>) {
        super::set_track(&self.inner, &TrackPublication::Local(self.clone()), track);
    }

    pub(crate) fn proto_info(&self) -> proto::TrackInfo {
        self.inner.info.read().proto_info.clone()
    }

    #[allow(dead_code)]
    pub(crate) fn update_info(&self, info: proto::TrackInfo) {
        super::update_info(&self.inner, &TrackPublication::Local(self.clone()), info);
    }

    pub(crate) fn update_publish_options(&self, opts: TrackPublishOptions) {
        *self.local.publish_options.lock() = opts;
    }

    pub fn publish_options(&self) -> TrackPublishOptions {
        self.local.publish_options.lock().clone()
    }

    pub fn mute(&self) {
        if let Some(track) = self.track() {
            track.mute();
        }

        if let Some(mute_update_needed) = self.inner.events.muted.lock().as_ref() {
            mute_update_needed(TrackPublication::Local(self.clone()))
        }
    }

    pub fn unmute(&self) {
        if let Some(track) = self.track() {
            track.unmute();
        }

        if let Some(unmute_update_needed) = self.inner.events.unmuted.lock().as_ref() {
            unmute_update_needed(TrackPublication::Local(self.clone()))
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

    pub fn simulcasted(&self) -> bool {
        self.inner.info.read().simulcasted
    }

    pub fn dimension(&self) -> TrackDimension {
        self.inner.info.read().dimension
    }

    pub fn track(&self) -> Option<LocalTrack> {
        self.inner.info.read().track.clone().map(|track| track.try_into().unwrap())
    }

    pub fn mime_type(&self) -> String {
        self.inner.info.read().mime_type.clone()
    }

    pub fn is_muted(&self) -> bool {
        if let Some(track) = self.track() {
            return track.is_muted();
        }

        self.inner.info.read().muted
    }

    pub fn is_remote(&self) -> bool {
        false
    }

    pub fn encryption_type(&self) -> EncryptionType {
        self.inner.info.read().encryption_type
    }

    pub fn audio_features(&self) -> Vec<AudioTrackFeature> {
        self.inner.info.read().audio_features.clone()
    }
}
