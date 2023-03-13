use anyhow::{anyhow, Result};
use async_trait::async_trait;
use collections::HashMap;
use futures::Stream;
use gpui::executor::Background;
use lazy_static::lazy_static;
use live_kit_server::token;
use media::core_video::CVImageBuffer;
use parking_lot::Mutex;
use postage::watch;
use std::{future::Future, mem, sync::Arc};

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
        if room.client_rooms.contains_key(&identity) {
            Err(anyhow!(
                "{:?} attempted to join room {:?} twice",
                identity,
                room_name
            ))
        } else {
            for track in &room.tracks {
                client_room
                    .0
                    .lock()
                    .video_track_updates
                    .0
                    .try_broadcast(RemoteVideoTrackUpdate::Subscribed(track.clone()))
                    .unwrap();
            }
            room.client_rooms.insert(identity, client_room);
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
        room.client_rooms.remove(&identity).ok_or_else(|| {
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
        room.client_rooms.remove(&identity).ok_or_else(|| {
            anyhow!(
                "participant {:?} did not join room {:?}",
                identity,
                room_name
            )
        })?;
        Ok(())
    }

    pub async fn disconnect_client(&self, client_identity: String) {
        self.background.simulate_random_delay().await;
        let mut server_rooms = self.rooms.lock();
        for room in server_rooms.values_mut() {
            if let Some(room) = room.client_rooms.remove(&client_identity) {
                *room.0.lock().connection.0.borrow_mut() = ConnectionState::Disconnected;
            }
        }
    }

    async fn publish_video_track(&self, token: String, local_track: LocalVideoTrack) -> Result<()> {
        self.background.simulate_random_delay().await;
        let claims = live_kit_server::token::validate(&token, &self.secret_key)?;
        let identity = claims.sub.unwrap().to_string();
        let room_name = claims.video.room.unwrap();

        let mut server_rooms = self.rooms.lock();
        let room = server_rooms
            .get_mut(&*room_name)
            .ok_or_else(|| anyhow!("room {} does not exist", room_name))?;

        let track = Arc::new(RemoteVideoTrack {
            sid: nanoid::nanoid!(17),
            publisher_id: identity.clone(),
            frames_rx: local_track.frames_rx.clone(),
        });

        room.tracks.push(track.clone());

        for (id, client_room) in &room.client_rooms {
            if *id != identity {
                let _ = client_room
                    .0
                    .lock()
                    .video_track_updates
                    .0
                    .try_broadcast(RemoteVideoTrackUpdate::Subscribed(track.clone()))
                    .unwrap();
            }
        }

        Ok(())
    }

    fn video_tracks(&self, token: String) -> Result<Vec<Arc<RemoteVideoTrack>>> {
        let claims = live_kit_server::token::validate(&token, &self.secret_key)?;
        let room_name = claims.video.room.unwrap();

        let mut server_rooms = self.rooms.lock();
        let room = server_rooms
            .get_mut(&*room_name)
            .ok_or_else(|| anyhow!("room {} does not exist", room_name))?;
        Ok(room.tracks.clone())
    }
}

#[derive(Default)]
struct TestServerRoom {
    client_rooms: HashMap<Sid, Arc<Room>>,
    tracks: Vec<Arc<RemoteVideoTrack>>,
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

struct RoomState {
    connection: (
        watch::Sender<ConnectionState>,
        watch::Receiver<ConnectionState>,
    ),
    display_sources: Vec<MacOSDisplay>,
    video_track_updates: (
        async_broadcast::Sender<RemoteVideoTrackUpdate>,
        async_broadcast::Receiver<RemoteVideoTrackUpdate>,
    ),
}

#[derive(Clone, Eq, PartialEq)]
pub enum ConnectionState {
    Disconnected,
    Connected { url: String, token: String },
}

pub struct Room(Mutex<RoomState>);

impl Room {
    pub fn new() -> Arc<Self> {
        Arc::new(Self(Mutex::new(RoomState {
            connection: watch::channel_with(ConnectionState::Disconnected),
            display_sources: Default::default(),
            video_track_updates: async_broadcast::broadcast(128),
        })))
    }

    pub fn status(&self) -> watch::Receiver<ConnectionState> {
        self.0.lock().connection.1.clone()
    }

    pub fn connect(self: &Arc<Self>, url: &str, token: &str) -> impl Future<Output = Result<()>> {
        let this = self.clone();
        let url = url.to_string();
        let token = token.to_string();
        async move {
            let server = TestServer::get(&url)?;
            server.join_room(token.clone(), this.clone()).await?;
            *this.0.lock().connection.0.borrow_mut() = ConnectionState::Connected { url, token };
            Ok(())
        }
    }

    pub fn display_sources(self: &Arc<Self>) -> impl Future<Output = Result<Vec<MacOSDisplay>>> {
        let this = self.clone();
        async move {
            let server = this.test_server();
            server.background.simulate_random_delay().await;
            Ok(this.0.lock().display_sources.clone())
        }
    }

    pub fn publish_video_track(
        self: &Arc<Self>,
        track: &LocalVideoTrack,
    ) -> impl Future<Output = Result<LocalTrackPublication>> {
        let this = self.clone();
        let track = track.clone();
        async move {
            this.test_server()
                .publish_video_track(this.token(), track)
                .await?;
            Ok(LocalTrackPublication)
        }
    }

    pub fn unpublish_track(&self, _: LocalTrackPublication) {}

    pub fn remote_video_tracks(&self, publisher_id: &str) -> Vec<Arc<RemoteVideoTrack>> {
        if !self.is_connected() {
            return Vec::new();
        }

        self.test_server()
            .video_tracks(self.token())
            .unwrap()
            .into_iter()
            .filter(|track| track.publisher_id() == publisher_id)
            .collect()
    }

    pub fn remote_video_track_updates(&self) -> impl Stream<Item = RemoteVideoTrackUpdate> {
        self.0.lock().video_track_updates.1.clone()
    }

    pub fn set_display_sources(&self, sources: Vec<MacOSDisplay>) {
        self.0.lock().display_sources = sources;
    }

    fn test_server(&self) -> Arc<TestServer> {
        match self.0.lock().connection.1.borrow().clone() {
            ConnectionState::Disconnected => panic!("must be connected to call this method"),
            ConnectionState::Connected { url, .. } => TestServer::get(&url).unwrap(),
        }
    }

    fn token(&self) -> String {
        match self.0.lock().connection.1.borrow().clone() {
            ConnectionState::Disconnected => panic!("must be connected to call this method"),
            ConnectionState::Connected { token, .. } => token,
        }
    }

    fn is_connected(&self) -> bool {
        match *self.0.lock().connection.1.borrow() {
            ConnectionState::Disconnected => false,
            ConnectionState::Connected { .. } => true,
        }
    }
}

impl Drop for Room {
    fn drop(&mut self) {
        if let ConnectionState::Connected { token, .. } = mem::replace(
            &mut *self.0.lock().connection.0.borrow_mut(),
            ConnectionState::Disconnected,
        ) {
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

#[derive(Clone)]
pub struct LocalVideoTrack {
    frames_rx: async_broadcast::Receiver<Frame>,
}

impl LocalVideoTrack {
    pub fn screen_share_for_display(display: &MacOSDisplay) -> Self {
        Self {
            frames_rx: display.frames.1.clone(),
        }
    }
}

pub struct RemoteVideoTrack {
    sid: Sid,
    publisher_id: Sid,
    frames_rx: async_broadcast::Receiver<Frame>,
}

impl RemoteVideoTrack {
    pub fn sid(&self) -> &str {
        &self.sid
    }

    pub fn publisher_id(&self) -> &str {
        &self.publisher_id
    }

    pub fn frames(&self) -> async_broadcast::Receiver<Frame> {
        self.frames_rx.clone()
    }
}

#[derive(Clone)]
pub enum RemoteVideoTrackUpdate {
    Subscribed(Arc<RemoteVideoTrack>),
    Unsubscribed { publisher_id: Sid, track_id: Sid },
}

#[derive(Clone)]
pub struct MacOSDisplay {
    frames: (
        async_broadcast::Sender<Frame>,
        async_broadcast::Receiver<Frame>,
    ),
}

impl MacOSDisplay {
    pub fn new() -> Self {
        Self {
            frames: async_broadcast::broadcast(128),
        }
    }

    pub fn send_frame(&self, frame: Frame) {
        self.frames.0.try_broadcast(frame).unwrap();
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Frame {
    pub label: String,
    pub width: usize,
    pub height: usize,
}

impl Frame {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn image(&self) -> CVImageBuffer {
        unimplemented!("you can't call this in test mode")
    }
}
