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
use livekit_protocol::enum_dispatch;
use livekit_protocol::{self as proto};
use parking_lot::{Mutex, RwLock};
use thiserror::Error;

use crate::prelude::*;

mod audio_track;
mod local_audio_track;
mod local_track;
mod local_video_track;
mod remote_audio_track;
mod remote_track;
mod remote_video_track;
mod video_track;

pub use audio_track::*;
pub use local_audio_track::*;
pub use local_track::*;
pub use local_video_track::*;
pub use remote_audio_track::*;
pub use remote_track::*;
pub use remote_video_track::*;
pub use video_track::*;

#[derive(Error, Debug, Clone)]
pub enum TrackError {
    #[error("could not find published track with sid: {0:?}")]
    TrackNotFound(TrackSid),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackKind {
    Audio,
    Video,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamState {
    Active,
    Paused,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackSource {
    Unknown,
    Camera,
    Microphone,
    Screenshare,
    ScreenshareAudio,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrackDimension(pub u32, pub u32);

/// Video quality for simulcasted tracks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VideoQuality {
    Low,
    Medium,
    High,
}

macro_rules! track_dispatch {
    ([$($variant:ident),+]) => {
        enum_dispatch!(
            [$($variant),+];
            pub fn sid(self: &Self) -> TrackSid;
            pub fn name(self: &Self) -> String;
            pub fn kind(self: &Self) -> TrackKind;
            pub fn source(self: &Self) -> TrackSource;
            pub fn stream_state(self: &Self) -> StreamState;
            pub fn is_enabled(self: &Self) -> bool;
            pub fn enable(self: &Self) -> ();
            pub fn disable(self: &Self) -> ();
            pub fn is_muted(self: &Self) -> bool;
            pub fn is_remote(self: &Self) -> bool;
            pub fn on_muted(self: &Self, on_mute: impl Fn(Track) + Send + 'static) -> ();
            pub fn on_unmuted(self: &Self, on_unmute: impl Fn(Track) + Send + 'static) -> ();

            pub(crate) fn transceiver(self: &Self) -> Option<RtpTransceiver>;
            pub(crate) fn set_transceiver(self: &Self, transceiver: Option<RtpTransceiver>) -> ();
            pub(crate) fn update_info(self: &Self, info: proto::TrackInfo) -> ();
        );
    };
}

#[derive(Clone, Debug)]
pub enum Track {
    LocalAudio(LocalAudioTrack),
    LocalVideo(LocalVideoTrack),
    RemoteAudio(RemoteAudioTrack),
    RemoteVideo(RemoteVideoTrack),
}

impl Track {
    track_dispatch!([LocalAudio, LocalVideo, RemoteAudio, RemoteVideo]);

    pub fn rtc_track(&self) -> MediaStreamTrack {
        match self {
            Self::LocalAudio(track) => track.rtc_track().into(),
            Self::LocalVideo(track) => track.rtc_track().into(),
            Self::RemoteAudio(track) => track.rtc_track().into(),
            Self::RemoteVideo(track) => track.rtc_track().into(),
        }
    }

    pub async fn get_stats(&self) -> RoomResult<Vec<RtcStats>> {
        match self {
            Self::LocalAudio(track) => track.get_stats().await,
            Self::LocalVideo(track) => track.get_stats().await,
            Self::RemoteAudio(track) => track.get_stats().await,
            Self::RemoteVideo(track) => track.get_stats().await,
        }
    }
}

pub(super) use track_dispatch;

type MutedHandler = Box<dyn Fn(Track) + Send>;
type UnmutedHandler = Box<dyn Fn(Track) + Send>;

#[derive(Default)]
struct TrackEvents {
    // These mute handlers are only called for local tracks
    pub muted: Option<MutedHandler>,
    pub unmuted: Option<UnmutedHandler>,
}

#[derive(Debug)]
struct TrackInfo {
    pub sid: TrackSid,
    pub name: String,
    pub kind: TrackKind,
    pub source: TrackSource,
    pub stream_state: StreamState,
    pub muted: bool,
    pub transceiver: Option<RtpTransceiver>,
    pub audio_features: Vec<proto::AudioTrackFeature>,
}

pub(super) struct TrackInner {
    info: RwLock<TrackInfo>,
    rtc_track: MediaStreamTrack,
    events: Mutex<TrackEvents>,
}

pub(super) fn new_inner(
    sid: TrackSid,
    name: String,
    kind: TrackKind,
    rtc_track: MediaStreamTrack,
) -> TrackInner {
    TrackInner {
        info: RwLock::new(TrackInfo {
            sid,
            name,
            kind,
            source: TrackSource::Unknown,
            stream_state: StreamState::Active,
            muted: false,
            transceiver: None,
            audio_features: Vec::new(),
        }),
        rtc_track,
        events: Default::default(),
    }
}

/// This is only called for local tracks
pub(super) fn set_muted(inner: &Arc<TrackInner>, track: &Track, muted: bool) {
    let info = inner.info.read();
    if info.muted == muted {
        return;
    }
    drop(info);

    if muted {
        inner.rtc_track.set_enabled(false);
    } else {
        inner.rtc_track.set_enabled(true);
    }

    inner.info.write().muted = muted;

    if muted {
        if let Some(on_mute) = inner.events.lock().muted.as_ref() {
            on_mute(track.clone());
        }
    } else if let Some(on_unmute) = inner.events.lock().unmuted.as_ref() {
        on_unmute(track.clone());
    }
}

pub(super) fn update_info(inner: &Arc<TrackInner>, _track: &Track, new_info: proto::TrackInfo) {
    let mut info = inner.info.write();
    info.kind = TrackKind::try_from(new_info.r#type()).unwrap();
    info.source = TrackSource::from(new_info.source());
    info.name = new_info.name.clone();
    info.sid = new_info.sid.clone().try_into().unwrap();
    info.audio_features =
        new_info.audio_features().into_iter().map(|item| item.try_into().unwrap()).collect();
}
