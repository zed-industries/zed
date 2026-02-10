use crate::{AudioStream, Participant, RemoteTrack, RoomEvent, TrackPublication};

use crate::mock_client::{participant::*, publication::*, track::*};
use anyhow::{Context as _, Result};
use async_trait::async_trait;
use collections::{BTreeMap, HashMap, HashSet, btree_map::Entry as BTreeEntry, hash_map::Entry};
use gpui::{App, AsyncApp, BackgroundExecutor};
use livekit_api::{proto, token};
use parking_lot::Mutex;
use postage::{mpsc, sink::Sink};
use std::sync::{
    Arc, Weak,
    atomic::{AtomicBool, Ordering::SeqCst},
};

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ParticipantIdentity(pub String);

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct TrackSid(pub(crate) String);

impl std::fmt::Display for TrackSid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl TryFrom<String> for TrackSid {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(TrackSid(value))
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum ConnectionState {
    Connected,
    Disconnected,
}

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
            anyhow::bail!("a server with url {url:?} already exists");
        }
    }

    fn get(url: &str) -> Result<Arc<TestServer>> {
        Ok(SERVERS
            .lock()
            .get(url)
            .context("no server found for url")?
            .clone())
    }

    pub fn teardown(&self) -> Result<()> {
        SERVERS
            .lock()
            .remove(&self.url)
            .with_context(|| format!("server with url {:?} does not exist", self.url))?;
        Ok(())
    }

    pub fn create_api_client(&self) -> TestApiClient {
        TestApiClient {
            url: self.url.clone(),
        }
    }

    pub async fn create_room(&self, room: String) -> Result<()> {
        self.simulate_random_delay().await;

        let mut server_rooms = self.rooms.lock();
        if let Entry::Vacant(e) = server_rooms.entry(room.clone()) {
            e.insert(Default::default());
            Ok(())
        } else {
            anyhow::bail!("{room:?} already exists");
        }
    }

    async fn delete_room(&self, room: String) -> Result<()> {
        self.simulate_random_delay().await;

        let mut server_rooms = self.rooms.lock();
        server_rooms
            .remove(&room)
            .with_context(|| format!("room {room:?} does not exist"))?;
        Ok(())
    }

    async fn join_room(&self, token: String, client_room: Room) -> Result<ParticipantIdentity> {
        self.simulate_random_delay().await;

        let claims = livekit_api::token::validate(&token, &self.secret_key)?;
        let identity = ParticipantIdentity(claims.sub.unwrap().to_string());
        let room_name = claims.video.room.unwrap();
        let mut server_rooms = self.rooms.lock();
        let room = (*server_rooms).entry(room_name.to_string()).or_default();

        if let Entry::Vacant(e) = room.client_rooms.entry(identity.clone()) {
            for server_track in &room.video_tracks {
                let track = RemoteTrack::Video(RemoteVideoTrack {
                    server_track: server_track.clone(),
                    _room: client_room.downgrade(),
                });
                client_room
                    .0
                    .lock()
                    .updates_tx
                    .blocking_send(RoomEvent::TrackSubscribed {
                        track: track.clone(),
                        publication: RemoteTrackPublication {
                            sid: server_track.sid.clone(),
                            room: client_room.downgrade(),
                            track,
                        },
                        participant: RemoteParticipant {
                            room: client_room.downgrade(),
                            identity: server_track.publisher_id.clone(),
                        },
                    })
                    .unwrap();
            }
            for server_track in &room.audio_tracks {
                let track = RemoteTrack::Audio(RemoteAudioTrack {
                    server_track: server_track.clone(),
                    room: client_room.downgrade(),
                });
                client_room
                    .0
                    .lock()
                    .updates_tx
                    .blocking_send(RoomEvent::TrackSubscribed {
                        track: track.clone(),
                        publication: RemoteTrackPublication {
                            sid: server_track.sid.clone(),
                            room: client_room.downgrade(),
                            track,
                        },
                        participant: RemoteParticipant {
                            room: client_room.downgrade(),
                            identity: server_track.publisher_id.clone(),
                        },
                    })
                    .unwrap();
            }
            e.insert(client_room);
            Ok(identity)
        } else {
            anyhow::bail!("{identity:?} attempted to join room {room_name:?} twice");
        }
    }

    async fn leave_room(&self, token: String) -> Result<()> {
        self.simulate_random_delay().await;

        let claims = livekit_api::token::validate(&token, &self.secret_key)?;
        let identity = ParticipantIdentity(claims.sub.unwrap().to_string());
        let room_name = claims.video.room.unwrap();
        let mut server_rooms = self.rooms.lock();
        let room = server_rooms
            .get_mut(&*room_name)
            .with_context(|| format!("room {room_name:?} does not exist"))?;
        room.client_rooms.remove(&identity).with_context(|| {
            format!("{identity:?} attempted to leave room {room_name:?} before joining it")
        })?;
        Ok(())
    }

    fn remote_participants(
        &self,
        token: String,
    ) -> Result<HashMap<ParticipantIdentity, RemoteParticipant>> {
        let claims = livekit_api::token::validate(&token, &self.secret_key)?;
        let local_identity = ParticipantIdentity(claims.sub.unwrap().to_string());
        let room_name = claims.video.room.unwrap().to_string();

        if let Some(server_room) = self.rooms.lock().get(&room_name) {
            let room = server_room
                .client_rooms
                .get(&local_identity)
                .unwrap()
                .downgrade();
            Ok(server_room
                .client_rooms
                .iter()
                .filter(|(identity, _)| *identity != &local_identity)
                .map(|(identity, _)| {
                    (
                        identity.clone(),
                        RemoteParticipant {
                            room: room.clone(),
                            identity: identity.clone(),
                        },
                    )
                })
                .collect())
        } else {
            Ok(Default::default())
        }
    }

    async fn remove_participant(
        &self,
        room_name: String,
        identity: ParticipantIdentity,
    ) -> Result<()> {
        self.simulate_random_delay().await;

        let mut server_rooms = self.rooms.lock();
        let room = server_rooms
            .get_mut(&room_name)
            .with_context(|| format!("room {room_name} does not exist"))?;
        room.client_rooms
            .remove(&identity)
            .with_context(|| format!("participant {identity:?} did not join room {room_name:?}"))?;
        Ok(())
    }

    async fn update_participant(
        &self,
        room_name: String,
        identity: String,
        permission: proto::ParticipantPermission,
    ) -> Result<()> {
        self.simulate_random_delay().await;

        let mut server_rooms = self.rooms.lock();
        let room = server_rooms
            .get_mut(&room_name)
            .with_context(|| format!("room {room_name} does not exist"))?;
        room.participant_permissions
            .insert(ParticipantIdentity(identity), permission);
        Ok(())
    }

    pub async fn disconnect_client(&self, client_identity: String) {
        let client_identity = ParticipantIdentity(client_identity);

        self.simulate_random_delay().await;

        let mut server_rooms = self.rooms.lock();
        for room in server_rooms.values_mut() {
            if let Some(room) = room.client_rooms.remove(&client_identity) {
                let mut room = room.0.lock();
                room.connection_state = ConnectionState::Disconnected;
                room.updates_tx
                    .blocking_send(RoomEvent::Disconnected {
                        reason: "SIGNAL_CLOSED",
                    })
                    .ok();
            }
        }
    }

    pub(crate) async fn publish_video_track(
        &self,
        token: String,
        _local_track: LocalVideoTrack,
    ) -> Result<TrackSid> {
        self.simulate_random_delay().await;

        let claims = livekit_api::token::validate(&token, &self.secret_key)?;
        let identity = ParticipantIdentity(claims.sub.unwrap().to_string());
        let room_name = claims.video.room.unwrap();

        let mut server_rooms = self.rooms.lock();
        let room = server_rooms
            .get_mut(&*room_name)
            .with_context(|| format!("room {room_name} does not exist"))?;

        let can_publish = room
            .participant_permissions
            .get(&identity)
            .map(|permission| permission.can_publish)
            .or(claims.video.can_publish)
            .unwrap_or(true);

        anyhow::ensure!(can_publish, "user is not allowed to publish");

        let sid: TrackSid = format!("TR_{}", nanoid::nanoid!(17)).try_into().unwrap();
        let server_track = Arc::new(TestServerVideoTrack {
            sid: sid.clone(),
            publisher_id: identity.clone(),
        });

        room.video_tracks.push(server_track.clone());

        for (room_identity, client_room) in &room.client_rooms {
            if *room_identity != identity {
                let track = RemoteTrack::Video(RemoteVideoTrack {
                    server_track: server_track.clone(),
                    _room: client_room.downgrade(),
                });
                let publication = RemoteTrackPublication {
                    sid: sid.clone(),
                    room: client_room.downgrade(),
                    track: track.clone(),
                };
                let participant = RemoteParticipant {
                    identity: identity.clone(),
                    room: client_room.downgrade(),
                };
                client_room
                    .0
                    .lock()
                    .updates_tx
                    .blocking_send(RoomEvent::TrackSubscribed {
                        track,
                        publication,
                        participant,
                    })
                    .unwrap();
            }
        }

        Ok(sid)
    }

    pub(crate) async fn publish_audio_track(
        &self,
        token: String,
        _local_track: &LocalAudioTrack,
    ) -> Result<TrackSid> {
        self.simulate_random_delay().await;

        let claims = livekit_api::token::validate(&token, &self.secret_key)?;
        let identity = ParticipantIdentity(claims.sub.unwrap().to_string());
        let room_name = claims.video.room.unwrap();

        let mut server_rooms = self.rooms.lock();
        let room = server_rooms
            .get_mut(&*room_name)
            .with_context(|| format!("room {room_name} does not exist"))?;

        let can_publish = room
            .participant_permissions
            .get(&identity)
            .map(|permission| permission.can_publish)
            .or(claims.video.can_publish)
            .unwrap_or(true);

        anyhow::ensure!(can_publish, "user is not allowed to publish");

        let sid: TrackSid = format!("TR_{}", nanoid::nanoid!(17)).try_into().unwrap();
        let server_track = Arc::new(TestServerAudioTrack {
            sid: sid.clone(),
            publisher_id: identity.clone(),
            muted: AtomicBool::new(false),
        });

        room.audio_tracks.push(server_track.clone());

        for (room_identity, client_room) in &room.client_rooms {
            if *room_identity != identity {
                let track = RemoteTrack::Audio(RemoteAudioTrack {
                    server_track: server_track.clone(),
                    room: client_room.downgrade(),
                });
                let publication = RemoteTrackPublication {
                    sid: sid.clone(),
                    room: client_room.downgrade(),
                    track: track.clone(),
                };
                let participant = RemoteParticipant {
                    identity: identity.clone(),
                    room: client_room.downgrade(),
                };
                client_room
                    .0
                    .lock()
                    .updates_tx
                    .blocking_send(RoomEvent::TrackSubscribed {
                        track,
                        publication,
                        participant,
                    })
                    .ok();
            }
        }

        Ok(sid)
    }

    pub(crate) async fn unpublish_track(&self, _token: String, _track: &TrackSid) -> Result<()> {
        Ok(())
    }

    pub(crate) fn set_track_muted(
        &self,
        token: &str,
        track_sid: &TrackSid,
        muted: bool,
    ) -> Result<()> {
        let claims = livekit_api::token::validate(token, &self.secret_key)?;
        let room_name = claims.video.room.unwrap();
        let identity = ParticipantIdentity(claims.sub.unwrap().to_string());
        let mut server_rooms = self.rooms.lock();
        let room = server_rooms
            .get_mut(&*room_name)
            .with_context(|| format!("room {room_name} does not exist"))?;
        if let Some(track) = room
            .audio_tracks
            .iter_mut()
            .find(|track| track.sid == *track_sid)
        {
            track.muted.store(muted, SeqCst);
            for (id, client_room) in room.client_rooms.iter() {
                if *id != identity {
                    let participant = Participant::Remote(RemoteParticipant {
                        identity: identity.clone(),
                        room: client_room.downgrade(),
                    });
                    let track = RemoteTrack::Audio(RemoteAudioTrack {
                        server_track: track.clone(),
                        room: client_room.downgrade(),
                    });
                    let publication = TrackPublication::Remote(RemoteTrackPublication {
                        sid: track_sid.clone(),
                        room: client_room.downgrade(),
                        track,
                    });

                    let event = if muted {
                        RoomEvent::TrackMuted {
                            participant,
                            publication,
                        }
                    } else {
                        RoomEvent::TrackUnmuted {
                            participant,
                            publication,
                        }
                    };

                    client_room
                        .0
                        .lock()
                        .updates_tx
                        .blocking_send(event)
                        .unwrap();
                }
            }
        }
        Ok(())
    }

    pub(crate) fn is_track_muted(&self, token: &str, track_sid: &TrackSid) -> Option<bool> {
        let claims = livekit_api::token::validate(token, &self.secret_key).ok()?;
        let room_name = claims.video.room.unwrap();

        let mut server_rooms = self.rooms.lock();
        let room = server_rooms.get_mut(&*room_name)?;
        room.audio_tracks.iter().find_map(|track| {
            if track.sid == *track_sid {
                Some(track.muted.load(SeqCst))
            } else {
                None
            }
        })
    }

    pub(crate) fn video_tracks(&self, token: String) -> Result<Vec<RemoteVideoTrack>> {
        let claims = livekit_api::token::validate(&token, &self.secret_key)?;
        let room_name = claims.video.room.unwrap();
        let identity = ParticipantIdentity(claims.sub.unwrap().to_string());

        let mut server_rooms = self.rooms.lock();
        let room = server_rooms
            .get_mut(&*room_name)
            .with_context(|| format!("room {room_name} does not exist"))?;
        let client_room = room
            .client_rooms
            .get(&identity)
            .context("not a participant in room")?;
        Ok(room
            .video_tracks
            .iter()
            .map(|track| RemoteVideoTrack {
                server_track: track.clone(),
                _room: client_room.downgrade(),
            })
            .collect())
    }

    pub(crate) fn audio_tracks(&self, token: String) -> Result<Vec<RemoteAudioTrack>> {
        let claims = livekit_api::token::validate(&token, &self.secret_key)?;
        let room_name = claims.video.room.unwrap();
        let identity = ParticipantIdentity(claims.sub.unwrap().to_string());

        let mut server_rooms = self.rooms.lock();
        let room = server_rooms
            .get_mut(&*room_name)
            .with_context(|| format!("room {room_name} does not exist"))?;
        let client_room = room
            .client_rooms
            .get(&identity)
            .context("not a participant in room")?;
        Ok(room
            .audio_tracks
            .iter()
            .map(|track| RemoteAudioTrack {
                server_track: track.clone(),
                room: client_room.downgrade(),
            })
            .collect())
    }

    async fn simulate_random_delay(&self) {
        #[cfg(any(test, feature = "test-support"))]
        self.executor.simulate_random_delay().await;
    }
}

#[derive(Default, Debug)]
struct TestServerRoom {
    client_rooms: HashMap<ParticipantIdentity, Room>,
    video_tracks: Vec<Arc<TestServerVideoTrack>>,
    audio_tracks: Vec<Arc<TestServerAudioTrack>>,
    participant_permissions: HashMap<ParticipantIdentity, proto::ParticipantPermission>,
}

#[derive(Debug)]
pub(crate) struct TestServerVideoTrack {
    pub(crate) sid: TrackSid,
    pub(crate) publisher_id: ParticipantIdentity,
    // frames_rx: async_broadcast::Receiver<Frame>,
}

#[derive(Debug)]
pub(crate) struct TestServerAudioTrack {
    pub(crate) sid: TrackSid,
    pub(crate) publisher_id: ParticipantIdentity,
    pub(crate) muted: AtomicBool,
}

pub struct TestApiClient {
    url: String,
}

#[async_trait]
impl livekit_api::Client for TestApiClient {
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
        server
            .remove_participant(room, ParticipantIdentity(identity))
            .await?;
        Ok(())
    }

    async fn update_participant(
        &self,
        room: String,
        identity: String,
        permission: livekit_api::proto::ParticipantPermission,
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

pub(crate) struct RoomState {
    pub(crate) url: String,
    pub(crate) token: String,
    pub(crate) local_identity: ParticipantIdentity,
    pub(crate) connection_state: ConnectionState,
    pub(crate) paused_audio_tracks: HashSet<TrackSid>,
    pub(crate) updates_tx: mpsc::Sender<RoomEvent>,
}

#[derive(Clone, Debug)]
pub struct Room(pub(crate) Arc<Mutex<RoomState>>);

#[derive(Clone, Debug)]
pub(crate) struct WeakRoom(Weak<Mutex<RoomState>>);

impl std::fmt::Debug for RoomState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Room")
            .field("url", &self.url)
            .field("token", &self.token)
            .field("local_identity", &self.local_identity)
            .field("connection_state", &self.connection_state)
            .field("paused_audio_tracks", &self.paused_audio_tracks)
            .finish()
    }
}

impl Room {
    pub(crate) fn downgrade(&self) -> WeakRoom {
        WeakRoom(Arc::downgrade(&self.0))
    }

    pub fn connection_state(&self) -> ConnectionState {
        self.0.lock().connection_state
    }

    pub fn local_participant(&self) -> LocalParticipant {
        let identity = self.0.lock().local_identity.clone();
        LocalParticipant {
            identity,
            room: self.clone(),
        }
    }

    pub async fn connect(
        url: String,
        token: String,
        _cx: &mut AsyncApp,
    ) -> Result<(Self, mpsc::Receiver<RoomEvent>)> {
        let server = TestServer::get(&url)?;
        let (updates_tx, updates_rx) = mpsc::channel(1024);
        let this = Self(Arc::new(Mutex::new(RoomState {
            local_identity: ParticipantIdentity(String::new()),
            url: url.to_string(),
            token: token.to_string(),
            connection_state: ConnectionState::Disconnected,
            paused_audio_tracks: Default::default(),
            updates_tx,
        })));

        let identity = server
            .join_room(token.to_string(), this.clone())
            .await
            .context("room join")?;
        {
            let mut state = this.0.lock();
            state.local_identity = identity;
            state.connection_state = ConnectionState::Connected;
        }

        Ok((this, updates_rx))
    }

    pub fn remote_participants(&self) -> HashMap<ParticipantIdentity, RemoteParticipant> {
        self.test_server()
            .remote_participants(self.0.lock().token.clone())
            .unwrap()
    }

    pub(crate) fn test_server(&self) -> Arc<TestServer> {
        TestServer::get(&self.0.lock().url).unwrap()
    }

    pub(crate) fn token(&self) -> String {
        self.0.lock().token.clone()
    }

    pub fn name(&self) -> String {
        "test_room".to_string()
    }

    pub async fn sid(&self) -> String {
        "RM_test_session".to_string()
    }

    pub fn play_remote_audio_track(
        &self,
        _track: &RemoteAudioTrack,
        _cx: &App,
    ) -> anyhow::Result<AudioStream> {
        Ok(AudioStream {})
    }

    pub async fn unpublish_local_track(&self, sid: TrackSid, cx: &mut AsyncApp) -> Result<()> {
        self.local_participant().unpublish_track(sid, cx).await
    }

    pub async fn publish_local_microphone_track(
        &self,
        _track_name: String,
        _is_staff: bool,
        cx: &mut AsyncApp,
    ) -> Result<(LocalTrackPublication, AudioStream)> {
        self.local_participant().publish_microphone_track(cx).await
    }
}

impl Drop for RoomState {
    fn drop(&mut self) {
        if self.connection_state == ConnectionState::Connected
            && let Ok(server) = TestServer::get(&self.url)
        {
            let executor = server.executor.clone();
            let token = self.token.clone();
            executor
                .spawn(async move { server.leave_room(token).await.ok() })
                .detach();
        }
    }
}

impl WeakRoom {
    pub(crate) fn upgrade(&self) -> Option<Room> {
        self.0.upgrade().map(Room)
    }
}
