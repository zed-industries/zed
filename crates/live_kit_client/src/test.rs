use anyhow::{anyhow, Result};
use collections::HashMap;
use futures::{channel::mpsc, future};
use gpui::executor::Background;
use lazy_static::lazy_static;
use media::core_video::CVImageBuffer;
use parking_lot::Mutex;
use std::{future::Future, sync::Arc};

lazy_static! {
    static ref SERVERS: Mutex<HashMap<String, Arc<FakeServer>>> = Default::default();
}

pub struct FakeServer {
    url: String,
    secret_key: String,
    rooms: Mutex<HashMap<String, FakeServerRoom>>,
    background: Arc<Background>,
}

impl FakeServer {
    pub fn create(
        url: String,
        secret_key: String,
        background: Arc<Background>,
    ) -> Result<Arc<FakeServer>> {
        let mut servers = SERVERS.lock();
        if servers.contains_key(&url) {
            Err(anyhow!("a server with url {:?} already exists", url))
        } else {
            let server = Arc::new(FakeServer {
                url: url.clone(),
                secret_key,
                rooms: Default::default(),
                background,
            });
            servers.insert(url, server.clone());
            Ok(server)
        }
    }

    fn get(url: &str) -> Result<Arc<FakeServer>> {
        Ok(SERVERS
            .lock()
            .get(url)
            .ok_or_else(|| anyhow!("no server found for url"))?
            .clone())
    }

    pub fn teardown(&self) -> Result<()> {
        SERVERS
            .lock()
            .remove(&self.url)
            .ok_or_else(|| anyhow!("server with url {:?} does not exist", self.url))?;
        Ok(())
    }

    async fn join_room(&self, token: String, client_room: Arc<Room>) -> Result<()> {
        self.background.simulate_random_delay().await;
        let claims = live_kit_server::token::validate(&token, &self.secret_key)?;
        let identity = claims.sub.unwrap().to_string();
        let room = claims.video.room.unwrap();
        let mut server_rooms = self.rooms.lock();
        let room = server_rooms
            .get_mut(&*room)
            .ok_or_else(|| anyhow!("room {} does not exist", room))?;
        room.clients.insert(identity, client_room);
        Ok(())
    }
}

struct FakeServerRoom {
    clients: HashMap<Sid, Arc<Room>>,
}

impl FakeServerRoom {}

pub type Sid = String;

pub struct Room;

impl Room {
    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }

    pub fn connect(self: &Arc<Self>, url: &str, token: &str) -> impl Future<Output = Result<()>> {
        let this = self.clone();
        let url = url.to_string();
        let token = token.to_string();
        async move {
            let server = FakeServer::get(&url)?;
            server.join_room(token, this).await?;
            Ok(())
        }
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

impl Drop for Room {
    fn drop(&mut self) {
        todo!()
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
