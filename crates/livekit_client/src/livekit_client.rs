mod playback;

use std::sync::Arc;

use anyhow::Result;
use collections::HashMap;
use futures::{channel::mpsc, SinkExt};
use gpui::{App, AsyncApp, Task};
use gpui_tokio::Tokio;
pub use playback::*;

mod remote_video_track_view;
#[cfg(any(
    test,
    feature = "test-support",
    all(target_os = "windows", target_env = "gnu")
))]
pub mod test;

// #[cfg(all(
//     not(any(test, feature = "test-support")),
//     not(all(target_os = "windows", target_env = "gnu"))
// ))]
// pub use livekit::*;
// #[cfg(any(
//     test,
//     feature = "test-support",
//     all(target_os = "windows", target_env = "gnu")
// ))]
// use test::track::RemoteAudioTrack;
// #[cfg(any(
//     test,
//     feature = "test-support",
//     all(target_os = "windows", target_env = "gnu")
// ))]
// pub use test::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ParticipantIdentity(pub String);

#[derive(Clone, Debug)]
pub struct RemoteVideoTrack(livekit::track::RemoteVideoTrack);
#[derive(Clone, Debug)]
pub struct RemoteAudioTrack(livekit::track::RemoteAudioTrack);
#[derive(Clone, Debug)]
pub struct LocalTrackPublication(livekit::publication::LocalTrackPublication);
#[derive(Clone, Debug)]
pub struct RemoteTrackPublication(livekit::publication::RemoteTrackPublication);

#[derive(Clone, Debug)]
pub struct LocalParticipant(livekit::participant::LocalParticipant);
#[derive(Clone, Debug)]
pub struct RemoteParticipant(livekit::participant::RemoteParticipant);

pub type LocalTrack = livekit::track::LocalTrack;
pub type TrackPublishOptions = livekit::options::TrackPublishOptions;
pub type TrackSid = livekit::id::TrackSid;
pub type TrackKind = livekit::track::TrackKind;
pub type TrackSource = livekit::track::TrackSource;
pub type ConnectionState = livekit::ConnectionState;

#[derive(Debug, Clone)]
pub enum Participant {
    Local(LocalParticipant),
    Remote(RemoteParticipant),
}

impl From<livekit::participant::Participant> for Participant {
    fn from(participant: livekit::participant::Participant) -> Self {
        match participant {
            livekit::participant::Participant::Local(local) => {
                Participant::Local(LocalParticipant(local))
            }
            livekit::participant::Participant::Remote(remote) => {
                Participant::Remote(RemoteParticipant(remote))
            }
        }
    }
}
#[derive(Debug, Clone)]
pub enum TrackPublication {
    Local(LocalTrackPublication),
    Remote(RemoteTrackPublication),
}

impl From<livekit::publication::TrackPublication> for TrackPublication {
    fn from(publication: livekit::publication::TrackPublication) -> Self {
        match publication {
            livekit::publication::TrackPublication::Local(local) => {
                TrackPublication::Local(LocalTrackPublication(local))
            }
            livekit::publication::TrackPublication::Remote(remote) => {
                TrackPublication::Remote(RemoteTrackPublication(remote))
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum RemoteTrack {
    Audio(RemoteAudioTrack),
    Video(RemoteVideoTrack),
}

impl From<livekit::track::RemoteTrack> for RemoteTrack {
    fn from(track: livekit::track::RemoteTrack) -> Self {
        match track {
            livekit::track::RemoteTrack::Audio(audio) => {
                RemoteTrack::Audio(RemoteAudioTrack(audio))
            }
            livekit::track::RemoteTrack::Video(video) => {
                RemoteTrack::Video(RemoteVideoTrack(video))
            }
        }
    }
}

pub struct Room {
    room: livekit::Room,
    _task: Task<()>,
}

impl Room {
    pub async fn connect(
        url: String,
        token: String,
        cx: &mut App,
    ) -> Result<(Self, mpsc::UnboundedReceiver<RoomEvent>)> {
        let _guard = Tokio::handle(cx);
        let connector =
            tokio_tungstenite::Connector::Rustls(Arc::new(http_client_tls::tls_config()));
        let mut config = livekit::RoomOptions::default();
        config.connector = Some(connector);
        let (room, mut events) = livekit::Room::connect(&url, &token, config).await?;

        let (mut tx, rx) = mpsc::unbounded();
        let task = cx.background_executor().spawn(async move {
            while let Some(event) = events.recv().await {
                if let Some(event) = RoomEvent::from_livekit(event) {
                    tx.send(event.into()).await;
                }
            }
        });

        Ok((Self { room, _task: task }, rx))
    }

    pub fn local_participant(&self) -> LocalParticipant {
        LocalParticipant(self.room.local_participant())
    }

    pub fn remote_participants(&self) -> HashMap<ParticipantIdentity, RemoteParticipant> {
        self.room
            .remote_participants()
            .into_iter()
            .map(|(k, v)| (ParticipantIdentity(k.0), RemoteParticipant(v)))
            .collect()
    }

    pub fn connection_state(&self, cx: &App) -> ConnectionState {
        let _guard = Tokio::handle(cx);
        self.room.connection_state()
    }
}

impl LocalParticipant {
    pub async fn publish_track(
        &self,
        track: LocalTrack,
        options: TrackPublishOptions,
        cx: &mut AsyncApp,
    ) -> Result<LocalTrackPublication> {
        let participant = self.0.clone();
        Tokio::spawn(cx, async move {
            participant.publish_track(track, options).await
        })?
        .await?
        .map(|p| LocalTrackPublication(p))
        .map_err(|error| anyhow::anyhow!("failed to publish track: {error}"))
    }

    pub async fn unpublish_track(
        &self,
        sid: TrackSid,
        cx: &mut AsyncApp,
    ) -> Result<LocalTrackPublication> {
        let participant = self.0.clone();
        Tokio::spawn(cx, async move { participant.unpublish_track(&sid).await })?
            .await?
            .map(|p| LocalTrackPublication(p))
            .map_err(|error| anyhow::anyhow!("failed to unpublish track: {error}"))
    }
}

impl LocalTrackPublication {
    pub fn mute(&self, cx: &App) {
        let _guard = Tokio::handle(cx);
        self.0.mute()
    }

    pub fn unmute(&self, cx: &App) {
        let _guard = Tokio::handle(cx);
        self.0.unmute()
    }
}

impl RemoteParticipant {
    pub fn identity(&self) -> ParticipantIdentity {
        ParticipantIdentity(self.0.identity().0)
    }

    pub fn track_publications(&self) -> HashMap<TrackSid, RemoteTrackPublication> {
        self.0
            .track_publications()
            .into_iter()
            .map(|(sid, publication)| (sid, RemoteTrackPublication(publication)))
            .collect()
    }
}

impl RemoteAudioTrack {
    pub fn sid(&self) -> TrackSid {
        self.0.sid()
    }
}

impl RemoteVideoTrack {
    pub fn sid(&self) -> TrackSid {
        self.0.sid()
    }
}

impl RemoteTrackPublication {
    pub fn is_muted(&self) -> bool {
        self.0.is_muted()
    }

    pub fn track(&self) -> Option<RemoteTrack> {
        Some(self.0.track()?.into())
    }

    pub fn kind(&self) -> TrackKind {
        self.0.kind()
    }

    pub fn set_enabled(&self, enabled: bool, cx: &App) {
        let _guard = Tokio::handle(cx);
        self.0.set_enabled(enabled)
    }
}

impl RemoteTrack {
    pub fn sid(&self) -> TrackSid {
        match self {
            RemoteTrack::Audio(remote_audio_track) => remote_audio_track.sid(),
            RemoteTrack::Video(remote_video_track) => remote_video_track.sid(),
        }
    }

    pub fn set_enabled(&self, enabled: bool, cx: &App) -> bool {
        let _guard = Tokio::handle(cx);
        match self {
            RemoteTrack::Audio(remote_audio_track) => {
                remote_audio_track.0.rtc_track().set_enabled(enabled)
            }
            RemoteTrack::Video(remote_video_track) => {
                remote_video_track.0.rtc_track().set_enabled(enabled)
            }
        }
    }
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
    LocalTrackSubscribed {
        track: LocalTrack,
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
        // error: livekit::track::TrackError,
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
    ParticipantAttributesChanged {
        participant: Participant,
        changed_attributes: HashMap<String, String>,
    },
    ActiveSpeakersChanged {
        speakers: Vec<Participant>,
    },
    // ConnectionQualityChanged {
    //     quality: ConnectionQuality,
    //     participant: Participant,
    // },
    // DataReceived {
    //     payload: Arc<Vec<u8>>,
    //     topic: Option<String>,
    //     kind: DataPacketKind,
    //     participant: Option<RemoteParticipant>,
    // },
    // TranscriptionReceived {
    //     participant: Option<Participant>,
    //     track_publication: Option<TrackPublication>,
    //     segments: Vec<TranscriptionSegment>,
    // },
    // SipDTMFReceived {
    //     code: u32,
    //     digit: Option<String>,
    //     participant: Option<RemoteParticipant>,
    // },
    // ChatMessage {
    //     message: ChatMessage,
    //     participant: Option<RemoteParticipant>,
    // },
    // StreamHeaderReceived {
    //     header: proto::data_stream::Header,
    //     participant_identity: String,
    // },
    // StreamChunkReceived {
    //     chunk: proto::data_stream::Chunk,
    //     participant_identity: String,
    // },
    // StreamTrailerReceived {
    //     trailer: proto::data_stream::Trailer,
    //     participant_identity: String,
    // },
    // E2eeStateChanged {
    //     participant: Participant,
    //     state: EncryptionState,
    // },
    ConnectionStateChanged(ConnectionState),
    Connected {
        /// Initial participants & their tracks prior to joining the room
        /// We're not returning this directly inside Room::connect because it is unlikely to be
        /// used
        participants_with_tracks: Vec<(RemoteParticipant, Vec<RemoteTrackPublication>)>,
    },
    Disconnected {
        reason: &'static str,
    },
    Reconnecting,
    Reconnected,
    // DataChannelBufferedAmountLowThresholdChanged {
    //     kind: DataPacketKind,
    //     threshold: u64,
    // },
}

impl RoomEvent {
    fn from_livekit(event: livekit::RoomEvent) -> Option<Self> {
        let event = match event {
            livekit::RoomEvent::ParticipantConnected(remote_participant) => {
                RoomEvent::ParticipantConnected(RemoteParticipant(remote_participant))
            }
            livekit::RoomEvent::ParticipantDisconnected(remote_participant) => {
                RoomEvent::ParticipantDisconnected(RemoteParticipant(remote_participant))
            }
            livekit::RoomEvent::LocalTrackPublished {
                publication,
                track,
                participant,
            } => RoomEvent::LocalTrackPublished {
                publication: LocalTrackPublication(publication),
                track,
                participant: LocalParticipant(participant),
            },
            livekit::RoomEvent::LocalTrackUnpublished {
                publication,
                participant,
            } => RoomEvent::LocalTrackUnpublished {
                publication: LocalTrackPublication(publication),
                participant: LocalParticipant(participant),
            },
            livekit::RoomEvent::LocalTrackSubscribed { track } => {
                RoomEvent::LocalTrackSubscribed { track }
            }
            livekit::RoomEvent::TrackSubscribed {
                track,
                publication,
                participant,
            } => RoomEvent::TrackSubscribed {
                track: track.into(),
                publication: RemoteTrackPublication(publication),
                participant: RemoteParticipant(participant),
            },
            livekit::RoomEvent::TrackUnsubscribed {
                track,
                publication,
                participant,
            } => RoomEvent::TrackUnsubscribed {
                track: track.into(),
                publication: RemoteTrackPublication(publication),
                participant: RemoteParticipant(participant),
            },
            livekit::RoomEvent::TrackSubscriptionFailed {
                participant,
                error: _,
                track_sid,
            } => RoomEvent::TrackSubscriptionFailed {
                participant: RemoteParticipant(participant),
                track_sid,
            },
            livekit::RoomEvent::TrackPublished {
                publication,
                participant,
            } => RoomEvent::TrackPublished {
                publication: RemoteTrackPublication(publication),
                participant: RemoteParticipant(participant),
            },
            livekit::RoomEvent::TrackUnpublished {
                publication,
                participant,
            } => RoomEvent::TrackUnpublished {
                publication: RemoteTrackPublication(publication),
                participant: RemoteParticipant(participant),
            },
            livekit::RoomEvent::TrackMuted {
                participant,
                publication,
            } => RoomEvent::TrackMuted {
                publication: publication.into(),
                participant: participant.into(),
            },
            livekit::RoomEvent::TrackUnmuted {
                participant,
                publication,
            } => RoomEvent::TrackUnmuted {
                publication: publication.into(),
                participant: participant.into(),
            },
            livekit::RoomEvent::RoomMetadataChanged {
                old_metadata,
                metadata,
            } => RoomEvent::RoomMetadataChanged {
                old_metadata,
                metadata,
            },
            livekit::RoomEvent::ParticipantMetadataChanged {
                participant,
                old_metadata,
                metadata,
            } => RoomEvent::ParticipantMetadataChanged {
                participant: participant.into(),
                old_metadata,
                metadata,
            },
            livekit::RoomEvent::ParticipantNameChanged {
                participant,
                old_name,
                name,
            } => RoomEvent::ParticipantNameChanged {
                participant: participant.into(),
                old_name,
                name,
            },
            livekit::RoomEvent::ParticipantAttributesChanged {
                participant,
                changed_attributes,
            } => RoomEvent::ParticipantAttributesChanged {
                participant: participant.into(),
                changed_attributes: changed_attributes.into_iter().collect(),
            },
            livekit::RoomEvent::ActiveSpeakersChanged { speakers } => {
                RoomEvent::ActiveSpeakersChanged {
                    speakers: speakers.into_iter().map(|s| s.into()).collect(),
                }
            }
            livekit::RoomEvent::Connected {
                participants_with_tracks,
            } => RoomEvent::Connected {
                participants_with_tracks: participants_with_tracks
                    .into_iter()
                    .map({
                        |(p, t)| {
                            (
                                RemoteParticipant(p),
                                t.into_iter().map(|t| RemoteTrackPublication(t)).collect(),
                            )
                        }
                    })
                    .collect(),
            },
            livekit::RoomEvent::Disconnected { reason } => RoomEvent::Disconnected {
                reason: reason.as_str_name(),
            },
            livekit::RoomEvent::Reconnecting => RoomEvent::Reconnecting,
            livekit::RoomEvent::Reconnected => RoomEvent::Reconnected,
            _ => {
                log::trace!("dropping livekit event: {:?}", event);
                return None;
            }
        };

        Some(event)
    }
}
