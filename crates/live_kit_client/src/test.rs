use anyhow::Result;
use futures::{channel::mpsc, future};
use media::core_video::CVImageBuffer;
use std::{future::Future, sync::Arc};

pub type Sid = String;

pub struct Room;

impl Room {
    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }

    pub fn connect(&self, url: &str, token: &str) -> impl Future<Output = Result<()>> {
        future::pending()
    }

    pub fn publish_video_track(
        &self,
        track: &LocalVideoTrack,
    ) -> impl Future<Output = Result<LocalTrackPublication>> {
        future::pending()
    }

    pub fn unpublish_track(&self, publication: LocalTrackPublication) {}

    pub fn remote_video_tracks(&self, participant_id: &str) -> Vec<Arc<RemoteVideoTrack>> {
        Default::default()
    }

    pub fn remote_video_track_updates(&self) -> mpsc::UnboundedReceiver<RemoteVideoTrackUpdate> {
        mpsc::unbounded().1
    }
}

pub struct LocalTrackPublication;

pub struct LocalVideoTrack;

impl LocalVideoTrack {
    pub fn screen_share_for_display(display: &MacOSDisplay) -> Self {
        Self
    }
}

pub struct RemoteVideoTrack {
    sid: Sid,
    publisher_id: Sid,
}

impl RemoteVideoTrack {
    pub fn sid(&self) -> &str {
        &self.sid
    }

    pub fn publisher_id(&self) -> &str {
        &self.publisher_id
    }

    pub fn add_renderer<F>(&self, callback: F)
    where
        F: 'static + FnMut(CVImageBuffer),
    {
    }
}

pub enum RemoteVideoTrackUpdate {
    Subscribed(Arc<RemoteVideoTrack>),
    Unsubscribed { publisher_id: Sid, track_id: Sid },
}

pub struct MacOSDisplay;

pub fn display_sources() -> impl Future<Output = Result<Vec<MacOSDisplay>>> {
    future::pending()
}
