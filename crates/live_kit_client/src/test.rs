use anyhow::{anyhow, Result};
use async_trait::async_trait;
use collections::HashMap;
use futures::{channel::mpsc, future};
use gpui::executor::Background;
use lazy_static::lazy_static;
use live_kit_server::token;
use media::core_video::CVImageBuffer;
use parking_lot::Mutex;
use std::{future::Future, sync::Arc};

lazy_static! {
    static ref SERVERS: Mutex<HashMap<String, Arc<TestServer>>> = Default::default();
}

pub struct TestServer {
    pub url: String,
    pub api_key: String,
    pub secret_key: String,
    rooms: Mutex<HashMap<String, TestServerRoom>>,
    background: Arc<Background>,
}

impl TestServer {
    pub fn create(
        url: String,
        api_key: String,
        secret_key: String,
        background: Arc<Background>,
    ) -> Result<Arc<TestServer>> {
        let mut servers = SERVERS.lock();
        if servers.contains_key(&url) {
            Err(anyhow!("a server with url {:?} already exists", url))
        } else {
            let server = Arc::new(TestServer {
                url: url.clone(),
                api_key,
                secret_key,
                rooms: Default::default(),
                background,
            });
            servers.insert(url, server.clone());
            Ok(server)
        }
    }

    fn get(url: &str) -> Result<Arc<TestServer>> {
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

    pub fn create_api_client(&self) -> TestApiClient {
        TestApiClient {
            url: self.url.clone(),
        }
    }

    async fn create_room(&self, room: String) -> Result<()> {
        self.background.simulate_random_delay().await;
        let mut server_rooms = self.rooms.lock();
        if server_rooms.contains_key(&room) {
            Err(anyhow!("room {:?} already exists", room))
        } else {
            server_rooms.insert(room, Default::default());
            Ok(())
        }
    }

    async fn delete_room(&self, room: String) -> Result<()> {
        // TODO: clear state associated with all `Room`s.
        self.background.simulate_random_delay().await;
        let mut server_rooms = self.rooms.lock();
        server_rooms
            .remove(&room)
            .ok_or_else(|| anyhow!("room {:?} does not exist", room))?;
        Ok(())
    }

    async fn join_room(&self, token: String, client_room: Arc<Room>) -> Result<()> {
        self.background.simulate_random_delay().await;
        let claims = live_kit_server::token::validate(&token, &self.secret_key)?;
        let identity = claims.sub.unwrap().to_string();
        let room_name = claims.video.room.unwrap();
        let mut server_rooms = self.rooms.lock();
        let room = server_rooms
            .get_mut(&*room_name)
            .ok_or_else(|| anyhow!("room {:?} does not exist", room_name))?;
        if room.clients.contains_key(&identity) {
            Err(anyhow!(
                "{:?} attempted to join room {:?} twice",
                identity,
                room_name
            ))
        } else {
            room.clients.insert(identity, client_room);
            Ok(())
        }
    }

    async fn leave_room(&self, token: String) -> Result<()> {
        self.background.simulate_random_delay().await;
        let claims = live_kit_server::token::validate(&token, &self.secret_key)?;
        let identity = claims.sub.unwrap().to_string();
        let room_name = claims.video.room.unwrap();
        let mut server_rooms = self.rooms.lock();
        let room = server_rooms
            .get_mut(&*room_name)
            .ok_or_else(|| anyhow!("room {} does not exist", room_name))?;
        room.clients.remove(&identity).ok_or_else(|| {
            anyhow!(
                "{:?} attempted to leave room {:?} before joining it",
                identity,
                room_name
            )
        })?;
        Ok(())
    }

    async fn remove_participant(&self, room_name: String, identity: String) -> Result<()> {
        // TODO: clear state associated with the `Room`.

        self.background.simulate_random_delay().await;
        let mut server_rooms = self.rooms.lock();
        let room = server_rooms
            .get_mut(&room_name)
            .ok_or_else(|| anyhow!("room {} does not exist", room_name))?;
        room.clients.remove(&identity).ok_or_else(|| {
            anyhow!(
                "participant {:?} did not join room {:?}",
                identity,
                room_name
            )
        })?;
        Ok(())
    }
}

#[derive(Default)]
struct TestServerRoom {
    clients: HashMap<Sid, Arc<Room>>,
}

impl TestServerRoom {}

pub struct TestApiClient {
    url: String,
}

#[async_trait]
impl live_kit_server::api::Client for TestApiClient {
    fn url(&self) -> &str {
        &self.url
    }

    async fn create_room(&self, name: String) -> Result<()> {
        let server = TestServer::get(&self.url)?;
        server.create_room(name).await?;
        Ok(())
    }

    async fn delete_room(&self, name: String) -> Result<()> {
        let server = TestServer::get(&self.url)?;
        server.delete_room(name).await?;
        Ok(())
    }

    async fn remove_participant(&self, room: String, identity: String) -> Result<()> {
        let server = TestServer::get(&self.url)?;
        server.remove_participant(room, identity).await?;
        Ok(())
    }

    fn room_token(&self, room: &str, identity: &str) -> Result<String> {
        let server = TestServer::get(&self.url)?;
        token::create(
            &server.api_key,
            &server.secret_key,
            Some(identity),
            token::VideoGrant::to_join(room),
        )
    }
}

pub type Sid = String;

#[derive(Default)]
struct RoomState {
    token: Option<String>,
}

#[derive(Default)]
pub struct Room(Mutex<RoomState>);

impl Room {
    pub fn new() -> Arc<Self> {
        Default::default()
    }

    pub fn connect(self: &Arc<Self>, url: &str, token: &str) -> impl Future<Output = Result<()>> {
        let this = self.clone();
        let url = url.to_string();
        let token = token.to_string();
        async move {
            let server = TestServer::get(&url)?;
            server.join_room(token.clone(), this.clone()).await?;
            this.0.lock().token = Some(token);
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
        if let Some(token) = self.0.lock().token.take() {
            if let Ok(server) = TestServer::get(&token) {
                let background = server.background.clone();
                background
                    .spawn(async move { server.leave_room(token).await.unwrap() })
                    .detach();
            }
        }
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
