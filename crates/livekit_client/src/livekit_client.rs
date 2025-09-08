use std::sync::Arc;

use anyhow::{Context as _, Result};
use audio::AudioSettings;
use collections::HashMap;
use futures::{SinkExt, channel::mpsc};
use gpui::{App, AsyncApp, ScreenCaptureSource, ScreenCaptureStream, Task};
use gpui_tokio::Tokio;
use log::info;
use playback::capture_local_video_track;
use settings::Settings;

mod playback;

use crate::{LocalTrack, Participant, RemoteTrack, RoomEvent, TrackPublication};
pub use playback::AudioStream;
pub(crate) use playback::{RemoteVideoFrame, play_remote_video_track};

#[derive(Clone, Debug)]
pub struct RemoteVideoTrack(livekit::track::RemoteVideoTrack);
#[derive(Clone, Debug)]
pub struct RemoteAudioTrack(livekit::track::RemoteAudioTrack);
#[derive(Clone, Debug)]
pub struct RemoteTrackPublication(livekit::publication::RemoteTrackPublication);
#[derive(Clone, Debug)]
pub struct RemoteParticipant(livekit::participant::RemoteParticipant);

#[derive(Clone, Debug)]
pub struct LocalVideoTrack(livekit::track::LocalVideoTrack);
#[derive(Clone, Debug)]
pub struct LocalAudioTrack(livekit::track::LocalAudioTrack);
#[derive(Clone, Debug)]
pub struct LocalTrackPublication(livekit::publication::LocalTrackPublication);
#[derive(Clone, Debug)]
pub struct LocalParticipant(livekit::participant::LocalParticipant);

pub struct Room {
    room: livekit::Room,
    _task: Task<()>,
    playback: playback::AudioStack,
}

pub type TrackSid = livekit::id::TrackSid;
pub type ConnectionState = livekit::ConnectionState;
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ParticipantIdentity(pub String);

impl Room {
    pub async fn connect(
        url: String,
        token: String,
        cx: &mut AsyncApp,
    ) -> Result<(Self, mpsc::UnboundedReceiver<RoomEvent>)> {
        let connector =
            tokio_tungstenite::Connector::Rustls(Arc::new(http_client_tls::tls_config()));
        let mut config = livekit::RoomOptions::default();
        config.connector = Some(connector);
        let (room, mut events) = Tokio::spawn(cx, async move {
            livekit::Room::connect(&url, &token, config).await
        })?
        .await??;

        let (mut tx, rx) = mpsc::unbounded();
        let task = cx.background_executor().spawn(async move {
            while let Some(event) = events.recv().await {
                if let Some(event) = room_event_from_livekit(event) {
                    tx.send(event).await.ok();
                }
            }
        });

        Ok((
            Self {
                room,
                _task: task,
                playback: playback::AudioStack::new(cx.background_executor().clone()),
            },
            rx,
        ))
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

    pub fn connection_state(&self) -> ConnectionState {
        self.room.connection_state()
    }

    pub async fn publish_local_microphone_track(
        &self,
        user_name: String,
        is_staff: bool,
        cx: &mut AsyncApp,
    ) -> Result<(LocalTrackPublication, playback::AudioStream)> {
        let (track, stream) = self
            .playback
            .capture_local_microphone_track(user_name, is_staff, &cx)?;
        let publication = self
            .local_participant()
            .publish_track(
                livekit::track::LocalTrack::Audio(track.0),
                livekit::options::TrackPublishOptions {
                    source: livekit::track::TrackSource::Microphone,
                    ..Default::default()
                },
                cx,
            )
            .await?;

        Ok((publication, stream))
    }

    pub async fn unpublish_local_track(
        &self,
        sid: TrackSid,
        cx: &mut AsyncApp,
    ) -> Result<LocalTrackPublication> {
        self.local_participant().unpublish_track(sid, cx).await
    }

    pub fn play_remote_audio_track(
        &self,
        track: &RemoteAudioTrack,
        cx: &mut App,
    ) -> Result<playback::AudioStream> {
        if AudioSettings::get_global(cx).rodio_audio {
            info!("Using experimental.rodio_audio audio pipeline for output");
            playback::play_remote_audio_track(&track.0, cx)
        } else {
            Ok(self.playback.play_remote_audio_track(&track.0))
        }
    }
}

impl LocalParticipant {
    pub async fn publish_screenshare_track(
        &self,
        source: &dyn ScreenCaptureSource,
        cx: &mut AsyncApp,
    ) -> Result<(LocalTrackPublication, Box<dyn ScreenCaptureStream>)> {
        let (track, stream) = capture_local_video_track(source, cx).await?;
        let options = livekit::options::TrackPublishOptions {
            source: livekit::track::TrackSource::Screenshare,
            video_codec: livekit::options::VideoCodec::VP8,
            ..Default::default()
        };
        let publication = self
            .publish_track(livekit::track::LocalTrack::Video(track.0), options, cx)
            .await?;

        Ok((publication, stream))
    }

    async fn publish_track(
        &self,
        track: livekit::track::LocalTrack,
        options: livekit::options::TrackPublishOptions,
        cx: &mut AsyncApp,
    ) -> Result<LocalTrackPublication> {
        let participant = self.0.clone();
        Tokio::spawn(cx, async move {
            participant.publish_track(track, options).await
        })?
        .await?
        .map(LocalTrackPublication)
        .context("publishing a track")
    }

    pub async fn unpublish_track(
        &self,
        sid: TrackSid,
        cx: &mut AsyncApp,
    ) -> Result<LocalTrackPublication> {
        let participant = self.0.clone();
        Tokio::spawn(cx, async move { participant.unpublish_track(&sid).await })?
            .await?
            .map(LocalTrackPublication)
            .context("unpublishing a track")
    }
}

impl LocalTrackPublication {
    pub fn mute(&self, cx: &App) {
        let track = self.0.clone();
        Tokio::spawn(cx, async move {
            track.mute();
        })
        .detach();
    }

    pub fn unmute(&self, cx: &App) {
        let track = self.0.clone();
        Tokio::spawn(cx, async move {
            track.unmute();
        })
        .detach();
    }

    pub fn sid(&self) -> TrackSid {
        self.0.sid()
    }

    pub fn is_muted(&self) -> bool {
        self.0.is_muted()
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

    pub fn is_enabled(&self) -> bool {
        self.0.is_enabled()
    }

    pub fn track(&self) -> Option<RemoteTrack> {
        self.0.track().map(remote_track_from_livekit)
    }

    pub fn is_audio(&self) -> bool {
        self.0.kind() == livekit::track::TrackKind::Audio
    }

    pub fn set_enabled(&self, enabled: bool, cx: &App) {
        let track = self.0.clone();
        Tokio::spawn(cx, async move { track.set_enabled(enabled) }).detach();
    }

    pub fn sid(&self) -> TrackSid {
        self.0.sid()
    }
}

impl Participant {
    pub fn identity(&self) -> ParticipantIdentity {
        match self {
            Participant::Local(local_participant) => {
                ParticipantIdentity(local_participant.0.identity().0)
            }
            Participant::Remote(remote_participant) => {
                ParticipantIdentity(remote_participant.0.identity().0)
            }
        }
    }
}

fn participant_from_livekit(participant: livekit::participant::Participant) -> Participant {
    match participant {
        livekit::participant::Participant::Local(local) => {
            Participant::Local(LocalParticipant(local))
        }
        livekit::participant::Participant::Remote(remote) => {
            Participant::Remote(RemoteParticipant(remote))
        }
    }
}

fn publication_from_livekit(
    publication: livekit::publication::TrackPublication,
) -> TrackPublication {
    match publication {
        livekit::publication::TrackPublication::Local(local) => {
            TrackPublication::Local(LocalTrackPublication(local))
        }
        livekit::publication::TrackPublication::Remote(remote) => {
            TrackPublication::Remote(RemoteTrackPublication(remote))
        }
    }
}

fn remote_track_from_livekit(track: livekit::track::RemoteTrack) -> RemoteTrack {
    match track {
        livekit::track::RemoteTrack::Audio(audio) => RemoteTrack::Audio(RemoteAudioTrack(audio)),
        livekit::track::RemoteTrack::Video(video) => RemoteTrack::Video(RemoteVideoTrack(video)),
    }
}

fn local_track_from_livekit(track: livekit::track::LocalTrack) -> LocalTrack {
    match track {
        livekit::track::LocalTrack::Audio(audio) => LocalTrack::Audio(LocalAudioTrack(audio)),
        livekit::track::LocalTrack::Video(video) => LocalTrack::Video(LocalVideoTrack(video)),
    }
}
fn room_event_from_livekit(event: livekit::RoomEvent) -> Option<RoomEvent> {
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
            track: local_track_from_livekit(track),
            participant: LocalParticipant(participant),
        },
        livekit::RoomEvent::LocalTrackUnpublished {
            publication,
            participant,
        } => RoomEvent::LocalTrackUnpublished {
            publication: LocalTrackPublication(publication),
            participant: LocalParticipant(participant),
        },
        livekit::RoomEvent::LocalTrackSubscribed { track } => RoomEvent::LocalTrackSubscribed {
            track: local_track_from_livekit(track),
        },
        livekit::RoomEvent::TrackSubscribed {
            track,
            publication,
            participant,
        } => RoomEvent::TrackSubscribed {
            track: remote_track_from_livekit(track),
            publication: RemoteTrackPublication(publication),
            participant: RemoteParticipant(participant),
        },
        livekit::RoomEvent::TrackUnsubscribed {
            track,
            publication,
            participant,
        } => RoomEvent::TrackUnsubscribed {
            track: remote_track_from_livekit(track),
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
            publication: publication_from_livekit(publication),
            participant: participant_from_livekit(participant),
        },
        livekit::RoomEvent::TrackUnmuted {
            participant,
            publication,
        } => RoomEvent::TrackUnmuted {
            publication: publication_from_livekit(publication),
            participant: participant_from_livekit(participant),
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
            participant: participant_from_livekit(participant),
            old_metadata,
            metadata,
        },
        livekit::RoomEvent::ParticipantNameChanged {
            participant,
            old_name,
            name,
        } => RoomEvent::ParticipantNameChanged {
            participant: participant_from_livekit(participant),
            old_name,
            name,
        },
        livekit::RoomEvent::ParticipantAttributesChanged {
            participant,
            changed_attributes,
        } => RoomEvent::ParticipantAttributesChanged {
            participant: participant_from_livekit(participant),
            changed_attributes: changed_attributes.into_iter().collect(),
        },
        livekit::RoomEvent::ActiveSpeakersChanged { speakers } => {
            RoomEvent::ActiveSpeakersChanged {
                speakers: speakers.into_iter().map(participant_from_livekit).collect(),
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
                            t.into_iter().map(RemoteTrackPublication).collect(),
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
