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

use std::sync::Arc;

use livekit_protocol::enum_dispatch;
use livekit_protocol::{self as proto, AudioTrackFeature};
use parking_lot::{Mutex, RwLock};

use super::track::TrackDimension;
use crate::{e2ee::EncryptionType, prelude::*, track::Track};

mod local;
mod remote;

pub use local::*;
pub use remote::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubscriptionStatus {
    Desired,
    Subscribed,
    Unsubscribed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionStatus {
    Allowed,
    NotAllowed,
}

#[derive(Clone, Debug)]
pub enum TrackPublication {
    Local(LocalTrackPublication),
    Remote(RemoteTrackPublication),
}

impl TrackPublication {
    enum_dispatch!(
        [Local, Remote];
        pub fn sid(self: &Self) -> TrackSid;
        pub fn name(self: &Self) -> String;
        pub fn kind(self: &Self) -> TrackKind;
        pub fn source(self: &Self) -> TrackSource;
        pub fn simulcasted(self: &Self) -> bool;
        pub fn dimension(self: &Self) -> TrackDimension;
        pub fn mime_type(self: &Self) -> String;
        pub fn is_muted(self: &Self) -> bool;
        pub fn is_remote(self: &Self) -> bool;
        pub fn encryption_type(self: &Self) -> EncryptionType;
        pub fn audio_features(self: &Self) -> Vec<AudioTrackFeature>;

        pub(crate) fn on_muted(self: &Self, on_mute: impl Fn(TrackPublication) + Send + 'static) -> ();
        pub(crate) fn on_unmuted(self: &Self, on_unmute: impl Fn(TrackPublication) + Send + 'static) -> ();
        pub(crate) fn proto_info(self: &Self) -> proto::TrackInfo;
        pub(crate) fn update_info(self: &Self, info: proto::TrackInfo) -> ();
    );

    #[allow(dead_code)]
    pub(crate) fn set_track(&self, track: Option<Track>) {
        match self {
            TrackPublication::Local(p) => p.set_track(track),
            TrackPublication::Remote(p) => p.set_track(track.map(|t| t.try_into().unwrap())),
        }
    }

    pub fn track(&self) -> Option<Track> {
        match self {
            TrackPublication::Local(p) => p.track().map(Into::into),
            TrackPublication::Remote(p) => p.track().map(Into::into),
        }
    }
}

struct PublicationInfo {
    pub track: Option<Track>,
    pub name: String,
    pub sid: TrackSid,
    pub kind: TrackKind,
    pub source: TrackSource,
    pub simulcasted: bool,
    pub dimension: TrackDimension,
    pub mime_type: String,
    pub muted: bool,
    pub proto_info: proto::TrackInfo,
    pub encryption_type: EncryptionType,
    pub audio_features: Vec<AudioTrackFeature>,
}

pub(crate) type MutedHandler = Box<dyn Fn(TrackPublication) + Send>;
pub(crate) type UnmutedHandler = Box<dyn Fn(TrackPublication) + Send>;

#[derive(Default)]
struct PublicationEvents {
    muted: Mutex<Option<MutedHandler>>,
    unmuted: Mutex<Option<UnmutedHandler>>,
}

pub(super) struct TrackPublicationInner {
    info: RwLock<PublicationInfo>,
    events: Arc<PublicationEvents>,
}

pub(super) fn new_inner(
    info: proto::TrackInfo,
    track: Option<Track>,
) -> Arc<TrackPublicationInner> {
    let info = PublicationInfo {
        track,
        proto_info: info.clone(),
        source: info.source().into(),
        kind: info.r#type().try_into().unwrap(),
        encryption_type: info.encryption().into(),
        name: info.clone().name,
        sid: info.sid.clone().try_into().unwrap(),
        simulcasted: info.simulcast,
        dimension: TrackDimension(info.width, info.height),
        mime_type: info.mime_type.clone(),
        muted: info.muted,
        audio_features: info
            .audio_features()
            .into_iter()
            .map(|item| item.try_into().unwrap())
            .collect(),
    };

    Arc::new(TrackPublicationInner { info: RwLock::new(info), events: Default::default() })
}

pub(super) fn update_info(
    inner: &TrackPublicationInner,
    _publication: &TrackPublication,
    new_info: proto::TrackInfo,
) {
    let mut info = inner.info.write();
    info.kind = TrackKind::try_from(new_info.r#type()).unwrap();
    info.source = TrackSource::from(new_info.source());
    info.encryption_type = new_info.encryption().into();
    info.proto_info = new_info.clone();
    info.name = new_info.name.clone();
    info.sid = new_info.sid.clone().try_into().unwrap();
    info.dimension = TrackDimension(new_info.width, new_info.height);
    info.mime_type = new_info.mime_type.clone();
    info.simulcasted = new_info.simulcast;
    info.audio_features = new_info.audio_features().collect();
}

pub(super) fn set_track(
    inner: &TrackPublicationInner,
    publication: &TrackPublication,
    track: Option<Track>,
) {
    let mut info = inner.info.write();
    if let Some(prev_track) = info.track.as_ref() {
        prev_track.on_muted(|_| {});
        prev_track.on_unmuted(|_| {});
    }

    info.track = track.clone();

    if let Some(track) = track.as_ref() {
        info.sid = track.sid();

        track.on_muted({
            let events = inner.events.clone();
            let publication = publication.clone();
            move |_| {
                if let Some(on_muted) = events.muted.lock().as_ref() {
                    on_muted(publication.clone());
                }
            }
        });

        track.on_unmuted({
            let events = inner.events.clone();
            let publication = publication.clone();
            move |_| {
                if let Some(on_unmuted) = events.unmuted.lock().as_ref() {
                    on_unmuted(publication.clone());
                }
            }
        });
    }
}
