use crate::{ConnectionState, RoomUpdate, Sid};
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use collections::{btree_map::Entry as BTreeEntry, hash_map::Entry, BTreeMap, HashMap, HashSet};
use futures::Stream;
use gpui::{BackgroundExecutor, SurfaceSource};
use live_kit_server::{proto, token};

use parking_lot::Mutex;
use postage::watch;
use std::{
    future::Future,
    mem,
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc, Weak,
    },
};

static SERVERS: Mutex<BTreeMap<String, Arc<TestServer>>> = Mutex::new(BTreeMap::new());

pub struct TestServer {
    pub url: String,
    pub api_key: String,
    pub secret_key: String,
    rooms: Mutex<HashMap<String, TestServerRoom>>,
    executor: BackgroundExecutor,
}

impl TestServer {
    pub fn create(
        url: String,
        api_key: String,
        secret_key: String,
        executor: BackgroundExecutor,
    ) -> Result<Arc<TestServer>> {
        let mut servers = SERVERS.lock();
        if let BTreeEntry::Vacant(e) = servers.entry(url.clone()) {
            let server = Arc::new(TestServer {
                url,
                api_key,
                secret_key,
                rooms: Default::default(),
                executor,
            });
            e.insert(server.clone());
            Ok(server)
        } else {
            Err(anyhow!("a server with url {:?} already exists", url))
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

    pub async fn create_room(&self, room: String) -> Result<()> {
        // todo(linux): Remove this once the cross-platform LiveKit implementation is merged
        #[cfg(any(test, feature = "test-support"))]
        self.executor.simulate_random_delay().await;
        let mut server_rooms = self.rooms.lock();
        if let Entry::Vacant(e) = server_rooms.entry(room.clone()) {
            e.insert(Default::default());
            Ok(())
        } else {
            Err(anyhow!("room {:?} already exists", room))
        }
    }

    async fn delete_room(&self, room: String) -> Result<()> {
        // TODO: clear state associated with all `Room`s.
        // todo(linux): Remove this once the cross-platform LiveKit implementation is merged
        #[cfg(any(test, feature = "test-support"))]
        self.executor.simulate_random_delay().await;
        let mut server_rooms = self.rooms.lock();
        server_rooms
            .remove(&room)
            .ok_or_else(|| anyhow!("room {:?} does not exist", room))?;
        Ok(())
    }

    async fn join_room(&self, token: String, client_room: Arc<Room>) -> Result<()> {
        // todo(linux): Remove this once the cross-platform LiveKit implementation is merged
        #[cfg(any(test, feature = "test-support"))]
        self.executor.simulate_random_delay().await;

        let claims = live_kit_server::token::validate(&token, &self.secret_key)?;
        let identity = claims.sub.unwrap().to_string();
        let room_name = claims.video.room.unwrap();
        let mut server_rooms = self.rooms.lock();
        let room = (*server_rooms).entry(room_name.to_string()).or_default();

        if let Entry::Vacant(e) = room.client_rooms.entry(identity.clone()) {
            for track in &room.video_tracks {
                client_room
                    .0
                    .lock()
                    .updates_tx
                    .try_broadcast(RoomUpdate::SubscribedToRemoteVideoTrack(Arc::new(
                        RemoteVideoTrack {
                            server_track: track.clone(),
                        },
                    )))
                    .unwrap();
            }
            for track in &room.audio_tracks {
                client_room
                    .0
                    .lock()
                    .updates_tx
                    .try_broadcast(RoomUpdate::SubscribedToRemoteAudioTrack(
                        Arc::new(RemoteAudioTrack {
                            server_track: track.clone(),
                            room: Arc::downgrade(&client_room),
                        }),
                        Arc::new(RemoteTrackPublication),
                    ))
                    .unwrap();
            }
            e.insert(client_room);
            Ok(())
        } else {
            Err(anyhow!(
                "{:?} attempted to join room {:?} twice",
                identity,
                room_name
            ))
        }
    }

    async fn leave_room(&self, token: String) -> Result<()> {
        // todo(linux): Remove this once the cross-platform LiveKit implementation is merged
        #[cfg(any(test, feature = "test-support"))]
        self.executor.simulate_random_delay().await;
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
        // todo(linux): Remove this once the cross-platform LiveKit implementation is merged
        #[cfg(any(test, feature = "test-support"))]
        self.executor.simulate_random_delay().await;

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

    async fn update_participant(
        &self,
        room_name: String,
        identity: String,
        permission: proto::ParticipantPermission,
    ) -> Result<()> {
        // todo(linux): Remove this once the cross-platform LiveKit implementation is merged
        #[cfg(any(test, feature = "test-support"))]
        self.executor.simulate_random_delay().await;
        let mut server_rooms = self.rooms.lock();
        let room = server_rooms
            .get_mut(&room_name)
            .ok_or_else(|| anyhow!("room {} does not exist", room_name))?;
        room.participant_permissions.insert(identity, permission);
        Ok(())
    }

    pub async fn disconnect_client(&self, client_identity: String) {
        // todo(linux): Remove this once the cross-platform LiveKit implementation is merged
        #[cfg(any(test, feature = "test-support"))]
        self.executor.simulate_random_delay().await;
        let mut server_rooms = self.rooms.lock();
        for room in server_rooms.values_mut() {
            if let Some(room) = room.client_rooms.remove(&client_identity) {
                *room.0.lock().connection.0.borrow_mut() = ConnectionState::Disconnected;
            }
        }
    }

    async fn publish_video_track(
        &self,
        token: String,
        local_track: LocalVideoTrack,
    ) -> Result<Sid> {
        // todo(linux): Remove this once the cross-platform LiveKit implementation is merged
        #[cfg(any(test, feature = "test-support"))]
        self.executor.simulate_random_delay().await;
        let claims = live_kit_server::token::validate(&token, &self.secret_key)?;
        let identity = claims.sub.unwrap().to_string();
        let room_name = claims.video.room.unwrap();

        let mut server_rooms = self.rooms.lock();
        let room = server_rooms
            .get_mut(&*room_name)
            .ok_or_else(|| anyhow!("room {} does not exist", room_name))?;

        let can_publish = room
            .participant_permissions
            .get(&identity)
            .map(|permission| permission.can_publish)
            .or(claims.video.can_publish)
            .unwrap_or(true);

        if !can_publish {
            return Err(anyhow!("user is not allowed to publish"));
        }

        let sid = nanoid::nanoid!(17);
        let track = Arc::new(TestServerVideoTrack {
            sid: sid.clone(),
            publisher_id: identity.clone(),
            frames_rx: local_track.frames_rx.clone(),
        });

        room.video_tracks.push(track.clone());

        for (id, client_room) in &room.client_rooms {
            if *id != identity {
                let _ = client_room
                    .0
                    .lock()
                    .updates_tx
                    .try_broadcast(RoomUpdate::SubscribedToRemoteVideoTrack(Arc::new(
                        RemoteVideoTrack {
                            server_track: track.clone(),
                        },
                    )))
                    .unwrap();
            }
        }

        Ok(sid)
    }

    async fn publish_audio_track(
        &self,
        token: String,
        _local_track: &LocalAudioTrack,
    ) -> Result<Sid> {
        // todo(linux): Remove this once the cross-platform LiveKit implementation is merged
        #[cfg(any(test, feature = "test-support"))]
        self.executor.simulate_random_delay().await;

        let claims = live_kit_server::token::validate(&token, &self.secret_key)?;
        let identity = claims.sub.unwrap().to_string();
        let room_name = claims.video.room.unwrap();

        let mut server_rooms = self.rooms.lock();
        let room = server_rooms
            .get_mut(&*room_name)
            .ok_or_else(|| anyhow!("room {} does not exist", room_name))?;

        let can_publish = room
            .participant_permissions
            .get(&identity)
            .map(|permission| permission.can_publish)
            .or(claims.video.can_publish)
            .unwrap_or(true);

        if !can_publish {
            return Err(anyhow!("user is not allowed to publish"));
        }

        let sid = nanoid::nanoid!(17);
        let track = Arc::new(TestServerAudioTrack {
            sid: sid.clone(),
            publisher_id: identity.clone(),
            muted: AtomicBool::new(false),
        });

        let publication = Arc::new(RemoteTrackPublication);

        room.audio_tracks.push(track.clone());

        for (id, client_room) in &room.client_rooms {
            if *id != identity {
                let _ = client_room
                    .0
                    .lock()
                    .updates_tx
                    .try_broadcast(RoomUpdate::SubscribedToRemoteAudioTrack(
                        Arc::new(RemoteAudioTrack {
                            server_track: track.clone(),
                            room: Arc::downgrade(&client_room),
                        }),
                        publication.clone(),
                    ))
                    .unwrap();
            }
        }

        Ok(sid)
    }

    fn set_track_muted(&self, token: &str, track_sid: &str, muted: bool) -> Result<()> {
        let claims = live_kit_server::token::validate(&token, &self.secret_key)?;
        let room_name = claims.video.room.unwrap();
        let identity = claims.sub.unwrap();
        let mut server_rooms = self.rooms.lock();
        let room = server_rooms
            .get_mut(&*room_name)
            .ok_or_else(|| anyhow!("room {} does not exist", room_name))?;
        if let Some(track) = room
            .audio_tracks
            .iter_mut()
            .find(|track| track.sid == track_sid)
        {
            track.muted.store(muted, SeqCst);
            for (id, client_room) in room.client_rooms.iter() {
                if *id != identity {
                    client_room
                        .0
                        .lock()
                        .updates_tx
                        .try_broadcast(RoomUpdate::RemoteAudioTrackMuteChanged {
                            track_id: track_sid.to_string(),
                            muted,
                        })
                        .unwrap();
                }
            }
        }
        Ok(())
    }

    fn is_track_muted(&self, token: &str, track_sid: &str) -> Option<bool> {
        let claims = live_kit_server::token::validate(&token, &self.secret_key).ok()?;
        let room_name = claims.video.room.unwrap();

        let mut server_rooms = self.rooms.lock();
        let room = server_rooms.get_mut(&*room_name)?;
        room.audio_tracks.iter().find_map(|track| {
            if track.sid == track_sid {
                Some(track.muted.load(SeqCst))
            } else {
                None
            }
        })
    }

    fn video_tracks(&self, token: String) -> Result<Vec<Arc<RemoteVideoTrack>>> {
        let claims = live_kit_server::token::validate(&token, &self.secret_key)?;
        let room_name = claims.video.room.unwrap();
        let identity = claims.sub.unwrap();

        let mut server_rooms = self.rooms.lock();
        let room = server_rooms
            .get_mut(&*room_name)
            .ok_or_else(|| anyhow!("room {} does not exist", room_name))?;
        room.client_rooms
            .get(identity.as_ref())
            .ok_or_else(|| anyhow!("not a participant in room"))?;
        Ok(room
            .video_tracks
            .iter()
            .map(|track| {
                Arc::new(RemoteVideoTrack {
                    server_track: track.clone(),
                })
            })
            .collect())
    }

    fn audio_tracks(&self, token: String) -> Result<Vec<Arc<RemoteAudioTrack>>> {
        let claims = live_kit_server::token::validate(&token, &self.secret_key)?;
        let room_name = claims.video.room.unwrap();
        let identity = claims.sub.unwrap();

        let mut server_rooms = self.rooms.lock();
        let room = server_rooms
            .get_mut(&*room_name)
            .ok_or_else(|| anyhow!("room {} does not exist", room_name))?;
        let client_room = room
            .client_rooms
            .get(identity.as_ref())
            .ok_or_else(|| anyhow!("not a participant in room"))?;
        Ok(room
            .audio_tracks
            .iter()
            .map(|track| {
                Arc::new(RemoteAudioTrack {
                    server_track: track.clone(),
                    room: Arc::downgrade(&client_room),
                })
            })
            .collect())
    }
}

#[derive(Default)]
struct TestServerRoom {
    client_rooms: HashMap<Sid, Arc<Room>>,
    video_tracks: Vec<Arc<TestServerVideoTrack>>,
    audio_tracks: Vec<Arc<TestServerAudioTrack>>,
    participant_permissions: HashMap<Sid, proto::ParticipantPermission>,
}

#[derive(Debug)]
struct TestServerVideoTrack {
    sid: Sid,
    publisher_id: Sid,
    frames_rx: async_broadcast::Receiver<Frame>,
}

#[derive(Debug)]
struct TestServerAudioTrack {
    sid: Sid,
    publisher_id: Sid,
    muted: AtomicBool,
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

    async fn update_participant(
        &self,
        room: String,
        identity: String,
        permission: live_kit_server::proto::ParticipantPermission,
    ) -> Result<()> {
        let server = TestServer::get(&self.url)?;
        server
            .update_participant(room, identity, permission)
            .await?;
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

    fn guest_token(&self, room: &str, identity: &str) -> Result<String> {
        let server = TestServer::get(&self.url)?;
        token::create(
            &server.api_key,
            &server.secret_key,
            Some(identity),
            token::VideoGrant::for_guest(room),
        )
    }
}

struct RoomState {
    connection: (
        watch::Sender<ConnectionState>,
        watch::Receiver<ConnectionState>,
    ),
    display_sources: Vec<MacOSDisplay>,
    paused_audio_tracks: HashSet<Sid>,
    updates_tx: async_broadcast::Sender<RoomUpdate>,
    updates_rx: async_broadcast::Receiver<RoomUpdate>,
}

pub struct Room(Mutex<RoomState>);

impl Room {
    pub fn new() -> Arc<Self> {
        let (updates_tx, updates_rx) = async_broadcast::broadcast(128);
        Arc::new(Self(Mutex::new(RoomState {
            connection: watch::channel_with(ConnectionState::Disconnected),
            display_sources: Default::default(),
            paused_audio_tracks: Default::default(),
            updates_tx,
            updates_rx,
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
            server
                .join_room(token.clone(), this.clone())
                .await
                .context("room join")?;
            *this.0.lock().connection.0.borrow_mut() = ConnectionState::Connected { url, token };
            Ok(())
        }
    }

    pub fn display_sources(self: &Arc<Self>) -> impl Future<Output = Result<Vec<MacOSDisplay>>> {
        let this = self.clone();
        async move {
            // todo(linux): Remove this once the cross-platform LiveKit implementation is merged
            #[cfg(any(test, feature = "test-support"))]
            {
                let server = this.test_server();
                server.executor.simulate_random_delay().await;
            }

            Ok(this.0.lock().display_sources.clone())
        }
    }

    pub fn publish_video_track(
        self: &Arc<Self>,
        track: LocalVideoTrack,
    ) -> impl Future<Output = Result<LocalTrackPublication>> {
        let this = self.clone();
        let track = track.clone();
        async move {
            let sid = this
                .test_server()
                .publish_video_track(this.token(), track)
                .await?;
            Ok(LocalTrackPublication {
                room: Arc::downgrade(&this),
                sid,
            })
        }
    }

    pub fn publish_audio_track(
        self: &Arc<Self>,
        track: LocalAudioTrack,
    ) -> impl Future<Output = Result<LocalTrackPublication>> {
        let this = self.clone();
        let track = track.clone();
        async move {
            let sid = this
                .test_server()
                .publish_audio_track(this.token(), &track)
                .await?;
            Ok(LocalTrackPublication {
                room: Arc::downgrade(&this),
                sid,
            })
        }
    }

    pub fn unpublish_track(&self, _publication: LocalTrackPublication) {}

    pub fn remote_audio_tracks(&self, publisher_id: &str) -> Vec<Arc<RemoteAudioTrack>> {
        if !self.is_connected() {
            return Vec::new();
        }

        self.test_server()
            .audio_tracks(self.token())
            .unwrap()
            .into_iter()
            .filter(|track| track.publisher_id() == publisher_id)
            .collect()
    }

    pub fn remote_audio_track_publications(
        &self,
        publisher_id: &str,
    ) -> Vec<Arc<RemoteTrackPublication>> {
        if !self.is_connected() {
            return Vec::new();
        }

        self.test_server()
            .audio_tracks(self.token())
            .unwrap()
            .into_iter()
            .filter(|track| track.publisher_id() == publisher_id)
            .map(|_track| Arc::new(RemoteTrackPublication {}))
            .collect()
    }

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

    pub fn updates(&self) -> impl Stream<Item = RoomUpdate> {
        self.0.lock().updates_rx.clone()
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
                let executor = server.executor.clone();
                executor
                    .spawn(async move { server.leave_room(token).await.unwrap() })
                    .detach();
            }
        }
    }
}

#[derive(Clone)]
pub struct LocalTrackPublication {
    sid: String,
    room: Weak<Room>,
}

impl LocalTrackPublication {
    pub fn set_mute(&self, mute: bool) -> impl Future<Output = Result<()>> {
        let sid = self.sid.clone();
        let room = self.room.clone();
        async move {
            if let Some(room) = room.upgrade() {
                room.test_server()
                    .set_track_muted(&room.token(), &sid, mute)
            } else {
                Err(anyhow!("no such room"))
            }
        }
    }

    pub fn is_muted(&self) -> bool {
        if let Some(room) = self.room.upgrade() {
            room.test_server()
                .is_track_muted(&room.token(), &self.sid)
                .unwrap_or(false)
        } else {
            false
        }
    }

    pub fn sid(&self) -> String {
        self.sid.clone()
    }
}

pub struct RemoteTrackPublication;

impl RemoteTrackPublication {
    pub fn set_enabled(&self, _enabled: bool) -> impl Future<Output = Result<()>> {
        async { Ok(()) }
    }

    pub fn is_muted(&self) -> bool {
        false
    }

    pub fn sid(&self) -> String {
        "".to_string()
    }
}

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

#[derive(Clone)]
pub struct LocalAudioTrack;

impl LocalAudioTrack {
    pub fn create() -> Self {
        Self
    }
}

#[derive(Debug)]
pub struct RemoteVideoTrack {
    server_track: Arc<TestServerVideoTrack>,
}

impl RemoteVideoTrack {
    pub fn sid(&self) -> &str {
        &self.server_track.sid
    }

    pub fn publisher_id(&self) -> &str {
        &self.server_track.publisher_id
    }

    pub fn frames(&self) -> async_broadcast::Receiver<Frame> {
        self.server_track.frames_rx.clone()
    }
}

#[derive(Debug)]
pub struct RemoteAudioTrack {
    server_track: Arc<TestServerAudioTrack>,
    room: Weak<Room>,
}

impl RemoteAudioTrack {
    pub fn sid(&self) -> &str {
        &self.server_track.sid
    }

    pub fn publisher_id(&self) -> &str {
        &self.server_track.publisher_id
    }

    pub fn start(&self) {
        if let Some(room) = self.room.upgrade() {
            room.0
                .lock()
                .paused_audio_tracks
                .remove(&self.server_track.sid);
        }
    }

    pub fn stop(&self) {
        if let Some(room) = self.room.upgrade() {
            room.0
                .lock()
                .paused_audio_tracks
                .insert(self.server_track.sid.clone());
        }
    }

    pub fn is_playing(&self) -> bool {
        !self
            .room
            .upgrade()
            .unwrap()
            .0
            .lock()
            .paused_audio_tracks
            .contains(&self.server_track.sid)
    }
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

    pub fn image(&self) -> SurfaceSource {
        unimplemented!("you can't call this in test mode")
    }
}
