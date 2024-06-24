pub mod participant;
pub mod publication;
pub mod track;
pub mod webrtc;

use self::{id::*, participant::*, publication::*, track::*};
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use collections::{BTreeMap, HashMap, HashSet};
use gpui::BackgroundExecutor;
use live_kit_server::{proto, token};
use livekit::options::TrackPublishOptions;
use parking_lot::Mutex;
use postage::{mpsc, sink::Sink, watch};
use std::sync::{
    atomic::{AtomicBool, Ordering::SeqCst},
    Arc, Weak,
};

pub use livekit::{id, options, ConnectionState, DisconnectReason, RoomOptions};

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
        if servers.contains_key(&url) {
            Err(anyhow!("a server with url {:?} already exists", url))
        } else {
            let server = Arc::new(TestServer {
                url: url.clone(),
                api_key,
                secret_key,
                rooms: Default::default(),
                executor,
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

    pub async fn create_room(&self, room: String) -> Result<()> {
        // todo(linux): Remove this once the cross-platform LiveKit implementation is merged
        #[cfg(any(test, feature = "test-support"))]
        self.executor.simulate_random_delay().await;
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
        // todo(linux): Remove this once the cross-platform LiveKit implementation is merged
        #[cfg(any(test, feature = "test-support"))]
        self.executor.simulate_random_delay().await;
        let mut server_rooms = self.rooms.lock();
        server_rooms
            .remove(&room)
            .ok_or_else(|| anyhow!("room {:?} does not exist", room))?;
        Ok(())
    }

    async fn join_room(&self, token: String, client_room: Room) -> Result<ParticipantIdentity> {
        // todo(linux): Remove this once the cross-platform LiveKit implementation is merged
        #[cfg(any(test, feature = "test-support"))]
        self.executor.simulate_random_delay().await;

        let claims = live_kit_server::token::validate(&token, &self.secret_key)?;
        let identity = claims
            .sub
            .unwrap()
            .to_string()
            .try_into()
            .expect("invalid room sid");
        let room_name = claims.video.room.unwrap();
        let mut server_rooms = self.rooms.lock();
        let room = (*server_rooms).entry(room_name.to_string()).or_default();

        if room.client_rooms.contains_key(&identity) {
            Err(anyhow!(
                "{:?} attempted to join room {:?} twice",
                identity,
                room_name
            ))
        } else {
            for track in &room.video_tracks {
                client_room
                    .0
                    .lock()
                    .updates_tx
                    .blocking_send(RoomEvent::TrackSubscribed {
                        track: RemoteTrack::Video(RemoteVideoTrack {
                            server_track: track.clone(),
                            _room: client_room.downgrade(),
                        }),
                        publication: RemoteTrackPublication {
                            sid: track.sid.clone(),
                        },
                        participant: RemoteParticipant {
                            room: client_room.downgrade(),
                            identity: track.publisher_id.clone(),
                        },
                    })
                    .unwrap();
            }
            for track in &room.audio_tracks {
                client_room
                    .0
                    .lock()
                    .updates_tx
                    .blocking_send(RoomEvent::TrackSubscribed {
                        track: RemoteTrack::Audio(RemoteAudioTrack {
                            server_track: track.clone(),
                            room: client_room.downgrade(),
                        }),
                        publication: RemoteTrackPublication {
                            sid: track.sid.clone(),
                        },
                        participant: RemoteParticipant {
                            room: client_room.downgrade(),
                            identity: track.publisher_id.clone(),
                        },
                    })
                    .unwrap();
            }
            room.client_rooms.insert(identity.clone(), client_room);
            Ok(identity)
        }
    }

    async fn leave_room(&self, token: String) -> Result<()> {
        // todo(linux): Remove this once the cross-platform LiveKit implementation is merged
        #[cfg(any(test, feature = "test-support"))]
        self.executor.simulate_random_delay().await;
        let claims = live_kit_server::token::validate(&token, &self.secret_key)?;
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

    async fn remove_participant(
        &self,
        room_name: String,
        identity: ParticipantIdentity,
    ) -> Result<()> {
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
        #[cfg(any(test, feature = "test-support"))]
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
        _local_track: LocalVideoTrack,
    ) -> Result<TrackSid> {
        #[cfg(any(test, feature = "test-support"))]
        self.executor.simulate_random_delay().await;

        let claims = live_kit_server::token::validate(&token, &self.secret_key)?;
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
        let track = Arc::new(TestServerVideoTrack {
            sid: sid.clone(),
            publisher_id: identity.clone(),
            // frames_rx: local_track.frames_rx.clone(),
        });

        room.video_tracks.push(track.clone());

        for (id, client_room) in &room.client_rooms {
            if *id != identity {
                let _ = client_room
                    .0
                    .lock()
                    .updates_tx
                    .blocking_send(RoomEvent::TrackSubscribed {
                        track: RemoteTrack::Video(RemoteVideoTrack {
                            server_track: track.clone(),
                            _room: client_room.downgrade(),
                        }),
                        publication: RemoteTrackPublication { sid: sid.clone() },
                        participant: RemoteParticipant {
                            identity: identity.clone(),
                            room: client_room.downgrade(),
                        },
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
        // todo(linux): Remove this once the cross-platform LiveKit implementation is merged
        #[cfg(any(test, feature = "test-support"))]
        self.executor.simulate_random_delay().await;

        let claims = live_kit_server::token::validate(&token, &self.secret_key)?;
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
        let track = Arc::new(TestServerAudioTrack {
            sid: sid.clone(),
            publisher_id: identity.clone(),
            muted: AtomicBool::new(false),
        });

        room.audio_tracks.push(track.clone());

        for (id, client_room) in &room.client_rooms {
            if *id != identity {
                let _ = client_room
                    .0
                    .lock()
                    .updates_tx
                    .blocking_send(RoomEvent::TrackSubscribed {
                        track: RemoteTrack::Audio(RemoteAudioTrack {
                            server_track: track.clone(),
                            room: client_room.downgrade(),
                        }),
                        publication: RemoteTrackPublication { sid: sid.clone() },
                        participant: RemoteParticipant {
                            identity: id.clone(),
                            room: client_room.downgrade(),
                        },
                    })
                    .unwrap();
            }
        }

        Ok(sid)
    }

    async fn unpublish_track(&self, _token: String, _track: &TrackSid) -> Result<()> {
        Ok(())
    }

    fn set_track_muted(&self, token: &str, track_sid: &TrackSid, muted: bool) -> Result<()> {
        let claims = live_kit_server::token::validate(&token, &self.secret_key)?;
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
                    let publication = TrackPublication::Remote(RemoteTrackPublication {
                        sid: track_sid.clone(),
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
        let claims = live_kit_server::token::validate(&token, &self.secret_key).ok()?;
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

    fn video_tracks(&self, token: String) -> Result<Vec<Arc<RemoteVideoTrack>>> {
        let claims = live_kit_server::token::validate(&token, &self.secret_key)?;
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
            .map(|track| {
                Arc::new(RemoteVideoTrack {
                    server_track: track.clone(),
                    _room: client_room.downgrade(),
                })
            })
            .collect())
    }

    fn audio_tracks(&self, token: String) -> Result<Vec<Arc<RemoteAudioTrack>>> {
        let claims = live_kit_server::token::validate(&token, &self.secret_key)?;
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
            .map(|track| {
                Arc::new(RemoteAudioTrack {
                    server_track: track.clone(),
                    room: client_room.downgrade(),
                })
            })
            .collect())
    }
}

#[derive(Default)]
struct TestServerRoom {
    client_rooms: HashMap<ParticipantIdentity, Room>,
    video_tracks: Vec<Arc<TestServerVideoTrack>>,
    audio_tracks: Vec<Arc<TestServerAudioTrack>>,
    participant_permissions: HashMap<ParticipantIdentity, proto::ParticipantPermission>,
}

#[derive(Debug)]
struct TestServerVideoTrack {
    sid: TrackSid,
    publisher_id: ParticipantIdentity,
    // frames_rx: async_broadcast::Receiver<Frame>,
}

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
    ConnectionStateChanged(ConnectionState),
    Connected {
        participants_with_tracks: Vec<(RemoteParticipant, Vec<RemoteTrackPublication>)>,
    },
    Disconnected {
        reason: DisconnectReason,
    },
    Reconnecting,
    Reconnected,
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
        server
            .remove_participant(room, ParticipantIdentity(identity))
            .await?;
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
    url: String,
    token: String,
    local_identity: ParticipantIdentity,
    connection: (
        watch::Sender<ConnectionState>,
        watch::Receiver<ConnectionState>,
    ),
    paused_audio_tracks: HashSet<TrackSid>,
    updates_tx: mpsc::Sender<RoomEvent>,
}

#[derive(Clone, Debug)]
pub struct Room(Arc<Mutex<RoomState>>);

#[derive(Clone, Debug)]
pub(crate) struct WeakRoom(Weak<Mutex<RoomState>>);

impl std::fmt::Debug for RoomState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Room").finish()
    }
}

impl Room {
    pub fn status(&self) -> watch::Receiver<ConnectionState> {
        self.0.lock().connection.1.clone()
    }

    fn downgrade(&self) -> WeakRoom {
        WeakRoom(Arc::downgrade(&self.0))
    }

    pub fn connection_state(&self) -> ConnectionState {
        self.0.lock().connection.1.borrow().clone()
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
            connection: watch::channel_with(ConnectionState::Disconnected),
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
            *state.connection.0.borrow_mut() = ConnectionState::Connected;
        }

        Ok((this, updates_rx))
    }

    pub fn remote_participants(&self) -> HashMap<ParticipantIdentity, RemoteParticipant> {
        if let Some(server_room) = self.test_server().rooms.lock().get(&self.token()) {
            server_room
                .client_rooms
                .iter()
                .map(|(identity, room)| {
                    (
                        identity.clone(),
                        RemoteParticipant {
                            room: room.downgrade(),
                            identity: identity.clone(),
                        },
                    )
                })
                .collect()
        } else {
            Default::default()
        }
    }

    fn test_server(&self) -> Arc<TestServer> {
        TestServer::get(&self.0.lock().url).unwrap()
    }

    fn token(&self) -> String {
        self.0.lock().token.clone()
    }
}

impl Drop for Room {
    fn drop(&mut self) {
        let state = self.0.lock();
        if let Ok(server) = TestServer::get(&state.url) {
            let executor = server.executor.clone();
            let token = state.token.clone();
            executor
                .spawn(async move { server.leave_room(token).await.unwrap() })
                .detach();
        }
    }
}

impl WeakRoom {
    fn upgrade(&self) -> Option<Room> {
        self.0.upgrade().map(Room)
    }
}
