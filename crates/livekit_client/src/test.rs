pub mod participant;
pub mod publication;
pub mod track;

#[cfg(not(windows))]
pub mod webrtc;

#[cfg(not(windows))]
use self::id::*;
use self::{participant::*, publication::*, track::*};
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use collections::{btree_map::Entry as BTreeEntry, hash_map::Entry, BTreeMap, HashMap, HashSet};
use gpui::BackgroundExecutor;
#[cfg(not(windows))]
use livekit::options::TrackPublishOptions;
use livekit_server::{proto, token};
use parking_lot::Mutex;
use postage::{mpsc, sink::Sink};
use std::sync::{
    atomic::{AtomicBool, Ordering::SeqCst},
    Arc, Weak,
};

#[cfg(not(windows))]
pub use livekit::{id, options, ConnectionState, DisconnectReason, RoomOptions};

static SERVERS: Mutex<BTreeMap<String, Arc<TestServer>>> = Mutex::new(BTreeMap::new());

pub struct TestServer {
    pub url: String,
    pub api_key: String,
    pub secret_key: String,
    #[cfg(not(target_os = "windows"))]
    rooms: Mutex<HashMap<String, TestServerRoom>>,
    executor: BackgroundExecutor,
}

#[cfg(not(target_os = "windows"))]
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
        self.executor.simulate_random_delay().await;

        let mut server_rooms = self.rooms.lock();
        server_rooms
            .remove(&room)
            .ok_or_else(|| anyhow!("room {:?} does not exist", room))?;
        Ok(())
    }

    async fn join_room(&self, token: String, client_room: Room) -> Result<ParticipantIdentity> {
        self.executor.simulate_random_delay().await;

        let claims = livekit_server::token::validate(&token, &self.secret_key)?;
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
            Err(anyhow!(
                "{:?} attempted to join room {:?} twice",
                identity,
                room_name
            ))
        }
    }

    async fn leave_room(&self, token: String) -> Result<()> {
        self.executor.simulate_random_delay().await;

        let claims = livekit_server::token::validate(&token, &self.secret_key)?;
        let identity = ParticipantIdentity(claims.sub.unwrap().to_string());
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

    fn remote_participants(
        &self,
        token: String,
    ) -> Result<HashMap<ParticipantIdentity, RemoteParticipant>> {
        let claims = livekit_server::token::validate(&token, &self.secret_key)?;
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
        self.executor.simulate_random_delay().await;

        let mut server_rooms = self.rooms.lock();
        let room = server_rooms
            .get_mut(&room_name)
            .ok_or_else(|| anyhow!("room {} does not exist", room_name))?;
        room.participant_permissions
            .insert(ParticipantIdentity(identity), permission);
        Ok(())
    }

    pub async fn disconnect_client(&self, client_identity: String) {
        let client_identity = ParticipantIdentity(client_identity);

        self.executor.simulate_random_delay().await;

        let mut server_rooms = self.rooms.lock();
        for room in server_rooms.values_mut() {
            if let Some(room) = room.client_rooms.remove(&client_identity) {
                let mut room = room.0.lock();
                room.connection_state = ConnectionState::Disconnected;
                room.updates_tx
                    .blocking_send(RoomEvent::Disconnected {
                        reason: DisconnectReason::SignalClose,
                    })
                    .ok();
            }
        }
    }

    async fn publish_video_track(
        &self,
        token: String,
        _local_track: LocalVideoTrack,
    ) -> Result<TrackSid> {
        self.executor.simulate_random_delay().await;

        let claims = livekit_server::token::validate(&token, &self.secret_key)?;
        let identity = ParticipantIdentity(claims.sub.unwrap().to_string());
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

    async fn publish_audio_track(
        &self,
        token: String,
        _local_track: &LocalAudioTrack,
    ) -> Result<TrackSid> {
        self.executor.simulate_random_delay().await;

        let claims = livekit_server::token::validate(&token, &self.secret_key)?;
        let identity = ParticipantIdentity(claims.sub.unwrap().to_string());
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

    async fn unpublish_track(&self, _token: String, _track: &TrackSid) -> Result<()> {
        Ok(())
    }

    fn set_track_muted(&self, token: &str, track_sid: &TrackSid, muted: bool) -> Result<()> {
        let claims = livekit_server::token::validate(&token, &self.secret_key)?;
        let room_name = claims.video.room.unwrap();
        let identity = ParticipantIdentity(claims.sub.unwrap().to_string());
        let mut server_rooms = self.rooms.lock();
        let room = server_rooms
            .get_mut(&*room_name)
            .ok_or_else(|| anyhow!("room {} does not exist", room_name))?;
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

    fn is_track_muted(&self, token: &str, track_sid: &TrackSid) -> Option<bool> {
        let claims = livekit_server::token::validate(&token, &self.secret_key).ok()?;
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

    fn video_tracks(&self, token: String) -> Result<Vec<RemoteVideoTrack>> {
        let claims = livekit_server::token::validate(&token, &self.secret_key)?;
        let room_name = claims.video.room.unwrap();
        let identity = ParticipantIdentity(claims.sub.unwrap().to_string());

        let mut server_rooms = self.rooms.lock();
        let room = server_rooms
            .get_mut(&*room_name)
            .ok_or_else(|| anyhow!("room {} does not exist", room_name))?;
        let client_room = room
            .client_rooms
            .get(&identity)
            .ok_or_else(|| anyhow!("not a participant in room"))?;
        Ok(room
            .video_tracks
            .iter()
            .map(|track| RemoteVideoTrack {
                server_track: track.clone(),
                _room: client_room.downgrade(),
            })
            .collect())
    }

    fn audio_tracks(&self, token: String) -> Result<Vec<RemoteAudioTrack>> {
        let claims = livekit_server::token::validate(&token, &self.secret_key)?;
        let room_name = claims.video.room.unwrap();
        let identity = ParticipantIdentity(claims.sub.unwrap().to_string());

        let mut server_rooms = self.rooms.lock();
        let room = server_rooms
            .get_mut(&*room_name)
            .ok_or_else(|| anyhow!("room {} does not exist", room_name))?;
        let client_room = room
            .client_rooms
            .get(&identity)
            .ok_or_else(|| anyhow!("not a participant in room"))?;
        Ok(room
            .audio_tracks
            .iter()
            .map(|track| RemoteAudioTrack {
                server_track: track.clone(),
                room: client_room.downgrade(),
            })
            .collect())
    }
}

#[cfg(not(target_os = "windows"))]
#[derive(Default, Debug)]
struct TestServerRoom {
    client_rooms: HashMap<ParticipantIdentity, Room>,
    video_tracks: Vec<Arc<TestServerVideoTrack>>,
    audio_tracks: Vec<Arc<TestServerAudioTrack>>,
    participant_permissions: HashMap<ParticipantIdentity, proto::ParticipantPermission>,
}

#[cfg(not(target_os = "windows"))]
#[derive(Debug)]
struct TestServerVideoTrack {
    sid: TrackSid,
    publisher_id: ParticipantIdentity,
    // frames_rx: async_broadcast::Receiver<Frame>,
}

#[cfg(not(target_os = "windows"))]
#[derive(Debug)]
struct TestServerAudioTrack {
    sid: TrackSid,
    publisher_id: ParticipantIdentity,
    muted: AtomicBool,
}

pub struct TestApiClient {
    url: String,
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum RoomEvent {
    ParticipantConnected(RemoteParticipant),
    ParticipantDisconnected(RemoteParticipant),
    LocalTrackPublished {
        publication: LocalTrackPublication,
        track: LocalTrack,
        participant: LocalParticipant,
    },
    LocalTrackUnpublished {
        publication: LocalTrackPublication,
        participant: LocalParticipant,
    },
    TrackSubscribed {
        track: RemoteTrack,
        publication: RemoteTrackPublication,
        participant: RemoteParticipant,
    },
    TrackUnsubscribed {
        track: RemoteTrack,
        publication: RemoteTrackPublication,
        participant: RemoteParticipant,
    },
    TrackSubscriptionFailed {
        participant: RemoteParticipant,
        error: String,
        #[cfg(not(target_os = "windows"))]
        track_sid: TrackSid,
    },
    TrackPublished {
        publication: RemoteTrackPublication,
        participant: RemoteParticipant,
    },
    TrackUnpublished {
        publication: RemoteTrackPublication,
        participant: RemoteParticipant,
    },
    TrackMuted {
        participant: Participant,
        publication: TrackPublication,
    },
    TrackUnmuted {
        participant: Participant,
        publication: TrackPublication,
    },
    RoomMetadataChanged {
        old_metadata: String,
        metadata: String,
    },
    ParticipantMetadataChanged {
        participant: Participant,
        old_metadata: String,
        metadata: String,
    },
    ParticipantNameChanged {
        participant: Participant,
        old_name: String,
        name: String,
    },
    ActiveSpeakersChanged {
        speakers: Vec<Participant>,
    },
    #[cfg(not(target_os = "windows"))]
    ConnectionStateChanged(ConnectionState),
    Connected {
        participants_with_tracks: Vec<(RemoteParticipant, Vec<RemoteTrackPublication>)>,
    },
    #[cfg(not(target_os = "windows"))]
    Disconnected {
        reason: DisconnectReason,
    },
    Reconnecting,
    Reconnected,
}

#[cfg(not(target_os = "windows"))]
#[async_trait]
impl livekit_server::api::Client for TestApiClient {
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
        permission: livekit_server::proto::ParticipantPermission,
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
    url: String,
    token: String,
    #[cfg(not(target_os = "windows"))]
    local_identity: ParticipantIdentity,
    #[cfg(not(target_os = "windows"))]
    connection_state: ConnectionState,
    #[cfg(not(target_os = "windows"))]
    paused_audio_tracks: HashSet<TrackSid>,
    updates_tx: mpsc::Sender<RoomEvent>,
}

#[derive(Clone, Debug)]
pub struct Room(Arc<Mutex<RoomState>>);

#[derive(Clone, Debug)]
pub(crate) struct WeakRoom(Weak<Mutex<RoomState>>);

#[cfg(not(target_os = "windows"))]
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

#[cfg(target_os = "windows")]
impl std::fmt::Debug for RoomState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Room")
            .field("url", &self.url)
            .field("token", &self.token)
            .finish()
    }
}

#[cfg(not(target_os = "windows"))]
impl Room {
    fn downgrade(&self) -> WeakRoom {
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
        url: &str,
        token: &str,
        _options: RoomOptions,
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

    fn test_server(&self) -> Arc<TestServer> {
        TestServer::get(&self.0.lock().url).unwrap()
    }

    fn token(&self) -> String {
        self.0.lock().token.clone()
    }
}

#[cfg(not(target_os = "windows"))]
impl Drop for RoomState {
    fn drop(&mut self) {
        if self.connection_state == ConnectionState::Connected {
            if let Ok(server) = TestServer::get(&self.url) {
                let executor = server.executor.clone();
                let token = self.token.clone();
                executor
                    .spawn(async move { server.leave_room(token).await.ok() })
                    .detach();
            }
        }
    }
}

impl WeakRoom {
    fn upgrade(&self) -> Option<Room> {
        self.0.upgrade().map(Room)
    }
}
