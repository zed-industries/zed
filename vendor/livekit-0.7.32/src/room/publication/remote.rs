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
use parking_lot::{Mutex, RwLock};

use super::{PermissionStatus, SubscriptionStatus, TrackPublication, TrackPublicationInner};
use crate::{e2ee::EncryptionType, prelude::*, track::VideoQuality};

type SubscribedHandler = Box<dyn Fn(RemoteTrackPublication, RemoteTrack) + Send>;
type UnsubscribedHandler = Box<dyn Fn(RemoteTrackPublication, RemoteTrack) + Send>;
type SubscriptionStatusChangedHandler =
    Box<dyn Fn(RemoteTrackPublication, SubscriptionStatus, SubscriptionStatus) + Send>; // old_status, new_status
type PermissionStatusChangedHandler =
    Box<dyn Fn(RemoteTrackPublication, PermissionStatus, PermissionStatus) + Send>; // old_status, new_status
type SubscriptionUpdateNeededHandler = Box<dyn Fn(RemoteTrackPublication, bool) + Send>;
type EnabledStatusChangedHandler = Box<dyn Fn(RemoteTrackPublication, bool) + Send>;
type VideoDimensionsChangedHandler = Box<dyn Fn(RemoteTrackPublication, TrackDimension) + Send>;
type VideoQualityChangedHandler = Box<dyn Fn(RemoteTrackPublication, VideoQuality) + Send>;

#[derive(Default)]
struct RemoteEvents {
    subscribed: Mutex<Option<SubscribedHandler>>,
    unsubscribed: Mutex<Option<UnsubscribedHandler>>,
    subscription_status_changed: Mutex<Option<SubscriptionStatusChangedHandler>>,
    permission_status_changed: Mutex<Option<PermissionStatusChangedHandler>>,
    subscription_update_needed: Mutex<Option<SubscriptionUpdateNeededHandler>>,
    enabled_status_changed: Mutex<Option<EnabledStatusChangedHandler>>,
    video_dimensions_changed: Mutex<Option<VideoDimensionsChangedHandler>>,
    video_quality_changed: Mutex<Option<VideoQualityChangedHandler>>,
}

#[derive(Debug)]
struct RemoteInfo {
    subscribed: bool,
    allowed: bool,
}

struct RemoteInner {
    info: RwLock<RemoteInfo>,
    events: RemoteEvents,
}

#[derive(Clone)]
pub struct RemoteTrackPublication {
    inner: Arc<TrackPublicationInner>,
    remote: Arc<RemoteInner>,
}

impl Debug for RemoteTrackPublication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RemoteTrackPublication")
            .field("is_subscribed", &self.is_subscribed())
            .field("is_allowed", &self.is_allowed())
            .finish()
    }
}

impl RemoteTrackPublication {
    pub(crate) fn new(
        info: proto::TrackInfo,
        track: Option<RemoteTrack>,
        auto_subscribe: bool,
    ) -> Self {
        Self {
            inner: super::new_inner(info, track.map(Into::into)),
            remote: Arc::new(RemoteInner {
                info: RwLock::new(RemoteInfo { subscribed: auto_subscribe, allowed: true }),
                events: Default::default(),
            }),
        }
    }

    /// This is called by the RemoteParticipant when it successfully subscribe to the track or when
    /// the track is being unsubscribed.
    /// We register the mute events from the track here so we can forward them.
    pub(crate) fn set_track(&self, track: Option<RemoteTrack>) {
        let old_subscription_state = self.subscription_status();
        let old_permission_state = self.permission_status();

        let prev_track = self.track();

        if let Some(prev_track) = prev_track {
            if let Some(unsubscribed) = self.remote.events.unsubscribed.lock().as_ref() {
                unsubscribed(self.clone(), prev_track);
            }
        }

        super::set_track(
            &self.inner,
            &TrackPublication::Remote(self.clone()),
            track.clone().map(Into::into),
        );

        if let Some(track) = track {
            if let Some(subscribed) = self.remote.events.subscribed.lock().as_ref() {
                subscribed(self.clone(), track);
            }
        }

        self.emit_subscription_update(old_subscription_state);
        self.emit_permission_update(old_permission_state);
    }

    pub(crate) fn emit_subscription_update(&self, old_subscription_state: SubscriptionStatus) {
        if old_subscription_state != self.subscription_status() {
            if let Some(subscription_status_changed) =
                self.remote.events.subscription_status_changed.lock().as_ref()
            {
                subscription_status_changed(
                    self.clone(),
                    old_subscription_state,
                    self.subscription_status(),
                );
            }
        }
    }

    pub(crate) fn emit_permission_update(&self, old_permission_state: PermissionStatus) {
        if old_permission_state != self.permission_status() {
            if let Some(subscription_permission_changed) =
                self.remote.events.permission_status_changed.lock().as_ref()
            {
                subscription_permission_changed(
                    self.clone(),
                    old_permission_state,
                    self.permission_status(),
                );
            }
        }
    }

    #[allow(dead_code)]
    pub(crate) fn proto_info(&self) -> proto::TrackInfo {
        self.inner.info.read().proto_info.clone()
    }

    pub(crate) fn update_info(&self, new_info: proto::TrackInfo) {
        super::update_info(&self.inner, &TrackPublication::Remote(self.clone()), new_info.clone());

        let mut info = self.inner.info.write();
        let muted = info.muted;
        info.muted = new_info.muted;
        drop(info);

        // For remote tracks, the publication need to manually fire the muted/unmuted events
        // (they are not being fired for the tracks)
        if muted != new_info.muted {
            if new_info.muted {
                if let Some(on_mute) = self.inner.events.muted.lock().as_ref() {
                    on_mute(TrackPublication::Remote(self.clone()));
                }
            } else if let Some(on_unmute) = self.inner.events.unmuted.lock().as_ref() {
                on_unmute(TrackPublication::Remote(self.clone()));
            }
        }
    }

    pub(crate) fn on_muted(&self, f: impl Fn(TrackPublication) + Send + 'static) {
        *self.inner.events.muted.lock() = Some(Box::new(f));
    }

    pub(crate) fn on_unmuted(&self, f: impl Fn(TrackPublication) + Send + 'static) {
        *self.inner.events.unmuted.lock() = Some(Box::new(f));
    }

    pub(crate) fn on_subscribed(
        &self,
        f: impl Fn(RemoteTrackPublication, RemoteTrack) + Send + 'static,
    ) {
        *self.remote.events.subscribed.lock() = Some(Box::new(f));
    }

    pub(crate) fn on_unsubscribed(
        &self,
        f: impl Fn(RemoteTrackPublication, RemoteTrack) + Send + 'static,
    ) {
        *self.remote.events.unsubscribed.lock() = Some(Box::new(f));
    }

    pub(crate) fn on_subscription_status_changed(
        &self,
        f: impl Fn(RemoteTrackPublication, SubscriptionStatus, SubscriptionStatus) + Send + 'static,
    ) {
        *self.remote.events.subscription_status_changed.lock() = Some(Box::new(f));
    }

    pub(crate) fn on_permission_status_changed(
        &self,
        f: impl Fn(RemoteTrackPublication, PermissionStatus, PermissionStatus) + Send + 'static,
    ) {
        *self.remote.events.permission_status_changed.lock() = Some(Box::new(f));
    }

    pub(crate) fn on_subscription_update_needed(
        &self,
        f: impl Fn(RemoteTrackPublication, bool) + Send + 'static,
    ) {
        *self.remote.events.subscription_update_needed.lock() = Some(Box::new(f));
    }

    pub(crate) fn on_enabled_status_changed(
        &self,
        f: impl Fn(RemoteTrackPublication, bool) + Send + 'static,
    ) {
        *self.remote.events.enabled_status_changed.lock() = Some(Box::new(f));
    }

    pub(crate) fn on_video_dimensions_changed(
        &self,
        f: impl Fn(RemoteTrackPublication, TrackDimension) + Send + 'static,
    ) {
        *self.remote.events.video_dimensions_changed.lock() = Some(Box::new(f));
    }

    pub(crate) fn on_video_quality_changed(
        &self,
        f: impl Fn(RemoteTrackPublication, VideoQuality) + Send + 'static,
    ) {
        *self.remote.events.video_quality_changed.lock() = Some(Box::new(f));
    }

    pub fn set_subscribed(&self, subscribed: bool) {
        let old_subscription_state = self.subscription_status();
        let old_permission_state = self.permission_status();

        {
            let mut info = self.remote.info.write();
            info.subscribed = subscribed;

            if subscribed {
                info.allowed = true;
            }
        }

        if !subscribed {
            // TODO(theomonnom): Wait for the PC onRemoveTrack event instead?
            self.set_track(None);
        }

        // Request to send an update to the SFU
        if let Some(subscription_update_needed) =
            self.remote.events.subscription_update_needed.lock().as_ref()
        {
            subscription_update_needed(self.clone(), subscribed);
        }

        self.emit_subscription_update(old_subscription_state);
        self.emit_permission_update(old_permission_state);
    }

    /// For tracks that support simulcasting, adjust subscribed quality.
    ///
    /// This indicates the highest quality the client can accept. if network
    /// bandwidth does not allow, server will automatically reduce quality to
    /// optimize for uninterrupted video.
    ///
    pub fn set_video_quality(&self, quality: VideoQuality) {
        if !self.simulcasted() {
            log::warn!("Cannot set video quality for a track that is not simulcasted");
            return;
        }
        if let Some(video_quality_changed) =
            self.remote.events.video_quality_changed.lock().as_ref()
        {
            video_quality_changed(self.clone(), quality)
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        if self.is_subscribed() && enabled != self.is_enabled() {
            let track = self.track().unwrap();
            if self.is_enabled() {
                track.disable();
            } else {
                track.enable();
            }

            // Request to send an update to the SFU
            if let Some(enabled_status_changed) =
                self.remote.events.enabled_status_changed.lock().as_ref()
            {
                enabled_status_changed(self.clone(), enabled)
            }
        }
    }

    pub fn update_video_dimensions(&self, dimension: TrackDimension) {
        if self.is_subscribed() {
            if dimension != self.dimension() {
                let TrackDimension(width, height) = dimension;
                let mut new_info = self.proto_info();
                new_info.width = width;
                new_info.height = height;
                self.update_info(new_info);
            }
            // Request to send an update to the SFU
            if let Some(video_dimensions_changed) =
                self.remote.events.video_dimensions_changed.lock().as_ref()
            {
                video_dimensions_changed(self.clone(), dimension)
            }
        }
    }

    pub fn subscription_status(&self) -> SubscriptionStatus {
        if !self.remote.info.read().subscribed {
            return SubscriptionStatus::Unsubscribed;
        }

        if self.track().is_none() {
            return SubscriptionStatus::Desired;
        }

        SubscriptionStatus::Subscribed
    }

    pub fn permission_status(&self) -> PermissionStatus {
        if self.is_allowed() {
            PermissionStatus::Allowed
        } else {
            PermissionStatus::NotAllowed
        }
    }

    pub fn is_subscribed(&self) -> bool {
        self.track().is_some()
    }

    pub fn is_desired(&self) -> bool {
        self.remote.info.read().subscribed
    }

    pub fn is_allowed(&self) -> bool {
        self.remote.info.read().allowed
    }

    pub fn is_enabled(&self) -> bool {
        self.track().is_some_and(|x| x.is_enabled())
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

    pub fn track(&self) -> Option<RemoteTrack> {
        self.inner.info.read().track.clone().map(|track| track.try_into().unwrap())
    }

    pub fn mime_type(&self) -> String {
        self.inner.info.read().mime_type.clone()
    }

    pub fn is_muted(&self) -> bool {
        self.inner.info.read().muted
    }

    pub fn is_remote(&self) -> bool {
        true
    }

    pub fn encryption_type(&self) -> EncryptionType {
        self.inner.info.read().encryption_type
    }

    pub fn audio_features(&self) -> Vec<AudioTrackFeature> {
        self.inner.info.read().audio_features.clone()
    }
}
