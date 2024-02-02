use std::sync::Arc;

use anyhow::Result;
use futures::{channel::mpsc as futures_mpsc, Future};
use gpui::AppContext;
use livekit::{
    webrtc::video_source::{native::NativeVideoSource, RtcVideoSource},
    RoomEvent,
};
use parking_lot::Mutex;
use postage::watch;
use tokio::sync::mpsc as tokio_mpsc;

use crate::{ConnectionState, PlatformDisplayAbstractor, RoomUpdate};
use async_compat::{Compat, CompatExt};

struct RoomConnection {
    room: livekit::Room,
    events: Compat<tokio_mpsc::UnboundedReceiver<RoomEvent>>,
}

pub struct Room {
    livekit_connection: Mutex<Option<RoomConnection>>,
    update_subscribers: Mutex<Vec<futures_mpsc::UnboundedSender<RoomUpdate>>>,
    connection: Mutex<(
        watch::Sender<ConnectionState>,
        watch::Receiver<ConnectionState>,
    )>,
}

impl Room {
    pub fn new() -> Arc<Self> {
        Arc::new(Room {
            livekit_connection: Mutex::new(None),
            update_subscribers: Default::default(),
            connection: Mutex::new(watch::channel_with(ConnectionState::Disconnected)),
        })
    }

    pub fn status(&self) -> watch::Receiver<ConnectionState> {
        self.connection.lock().1.clone()
    }

    pub async fn connect(self: &Arc<Self>, url: &str, token: &str) -> Result<()> {
        let (room, events) = livekit::Room::connect(url, token, Default::default())
            .compat()
            .await?;

        *self.livekit_connection.lock() = Some(RoomConnection {
            room,
            events: events.compat(),
        });

        *self.connection.lock().0.borrow_mut() = ConnectionState::Connected {
            url: url.to_string(),
            token: token.to_string(),
        };

        Ok(())
    }

    pub fn updates(&self) -> futures_mpsc::UnboundedReceiver<RoomUpdate> {
        let (tx, rx) = futures_mpsc::unbounded();
        self.update_subscribers.lock().push(tx);
        rx
    }

    fn did_disconnect(&self) {
        *self.connection.lock().0.borrow_mut() = ConnectionState::Disconnected;
    }

    pub async fn display_sources(
        self: &Arc<Self>,
        cx: &mut AppContext,
    ) -> Result<Vec<Box<dyn PlatformDisplayAbstractor>>> {
        Ok(vec![])
    }

    pub fn publish_audio_track(
        self: &Arc<Self>,
        track: LocalAudioTrack,
    ) -> impl Future<Output = Result<LocalTrackPublication>> {
        async { todo!() }
    }

    pub fn remote_audio_tracks(&self, participant_id: &str) -> Vec<Arc<RemoteAudioTrack>> {
        todo!()
    }

    pub fn unpublish_track(&self, publication: LocalTrackPublication) {
        todo!()
    }

    pub fn publish_video_track(
        self: &Arc<Self>,
        track: LocalVideoTrack,
    ) -> impl Future<Output = Result<LocalTrackPublication>> {
        async { todo!() }
    }

    pub fn remote_video_tracks(&self, participant_id: &str) -> Vec<Arc<RemoteVideoTrack>> {
        todo!()
    }
}

pub struct LocalAudioTrack {}

impl LocalAudioTrack {
    pub fn create() -> Self {
        todo!()
    }
}

pub struct LocalVideoTrack {}

impl LocalVideoTrack {
    pub fn screen_share_for_display(display: &dyn PlatformDisplayAbstractor) -> Self {
        todo!()
    }
}

#[derive(Clone)]
pub struct LocalTrackPublication {}

impl LocalTrackPublication {
    pub fn set_mute(&self, muted: bool) -> impl Future<Output = Result<()>> {
        async { todo!() }
    }
}

pub struct RemoteTrackPublication {}

pub struct RemoteAudioTrack {}

impl RemoteAudioTrack {
    pub fn publisher_id(&self) -> &str {
        todo!()
    }

    pub fn sid(&self) -> &str {
        todo!()
    }
}

pub struct RemoteVideoTrack {}

impl RemoteVideoTrack {
    pub fn publisher_id(&self) -> &str {
        todo!()
    }

    pub fn sid(&self) -> &str {
        todo!()
    }
}

pub struct MacOSDisplay {}

pub struct Frame {}
