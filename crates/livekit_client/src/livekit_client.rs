use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::time::Duration;

use crate::ScreenCaptureStreamHandle;

use anyhow::{Context as _, Result, anyhow};
use audio::AudioSettings;
use collections::HashMap;
use futures::{SinkExt, channel::mpsc, stream::StreamExt};
use gpui::{App, AsyncApp, Task};
use gpui_tokio::Tokio;
use livekit::options::{TrackPublishOptions, VideoCodec};
use livekit::track::TrackSource;
use livekit::webrtc::desktop_capturer::{
    CaptureError, CaptureSource, DesktopCaptureSourceType, DesktopCapturer, DesktopCapturerOptions,
    DesktopFrame,
};
use livekit::webrtc::native::yuv_helper;
use livekit::webrtc::prelude::{
    I420Buffer, RtcVideoSource, VideoFrame, VideoResolution, VideoRotation,
};
use livekit::webrtc::video_source::native::NativeVideoSource;
use log::info;
use settings::Settings;

mod playback;

use crate::{
    LocalTrack, Participant, RemoteTrack, RoomEvent, TrackPublication,
    livekit_client::playback::Speaker,
};
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
#[allow(unused)]
pub struct LocalVideoTrack(livekit::track::LocalVideoTrack);
#[derive(Clone, Debug)]
pub struct LocalAudioTrack(livekit::track::LocalAudioTrack);
#[derive(Clone, Debug)]
pub struct LocalTrackPublication(livekit::publication::LocalTrackPublication);
#[derive(Clone, Debug)]
pub struct LocalParticipant(livekit::participant::LocalParticipant);

fn desktop_capturer_options() -> DesktopCapturerOptions {
    // Picking either a screen or a window with one DesktopCapturer is only implemented
    // in libwebrtc on Wayland and macOS.
    #[allow(unused_variables)]
    let source_type = DesktopCaptureSourceType::Screen;
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    let source_type = match gpui::guess_compositor() {
        gpui::LinuxCompositor::Wayland => DesktopCaptureSourceType::Generic,
        _ => DesktopCaptureSourceType::Screen,
    };
    #[cfg(target_os = "macos")]
    let source_type = DesktopCaptureSourceType::Generic;

    let mut options = DesktopCapturerOptions::new(source_type);
    options.set_include_cursor(true);
    options
}

pub fn screen_capture_sources() -> Vec<CaptureSource> {
    let capturer = DesktopCapturer::new(desktop_capturer_options()).unwrap();
    capturer.get_source_list()
}

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
        })
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

    pub fn name(&self) -> String {
        self.room.name()
    }

    pub async fn sid(&self) -> String {
        self.room.sid().await.to_string()
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
        let speaker: Speaker =
            serde_urlencoded::from_str(&track.0.name()).unwrap_or_else(|_| Speaker {
                name: track.0.name(),
                is_staff: false,
                sends_legacy_audio: true,
            });

        if AudioSettings::get_global(cx).rodio_audio {
            info!("Using experimental.rodio_audio audio pipeline for output");
            playback::play_remote_audio_track(&track.0, speaker, cx)
        } else if speaker.sends_legacy_audio {
            Ok(self.playback.play_remote_audio_track(&track.0))
        } else {
            Err(anyhow!("Client version too old to play audio in call"))
        }
    }
}

impl LocalParticipant {
    pub async fn publish_screenshare_track(
        &self,
        source: Option<CaptureSource>,
        cx: &mut AsyncApp,
    ) -> Result<(LocalTrackPublication, ScreenCaptureStreamHandle)> {
        let stop_capture = Arc::new(AtomicBool::new(false));
        let (mut video_source_sender, mut video_source_receiver) = mpsc::channel(0);
        let callback = {
            // These dimensions are arbitrary initial values.
            // libwebrtc only exposes the resolution of the source in the DesktopFrame
            // passed to the callback, so wait to publish the video track until
            // the callback is called the first time.
            let mut stream_width = 1920;
            let mut stream_height = 1080;

            let mut video_frame = VideoFrame {
                rotation: VideoRotation::VideoRotation0,
                buffer: I420Buffer::new(stream_width, stream_height),
                timestamp_us: 0,
            };
            let mut video_source: Option<NativeVideoSource> = None;
            let stop_capture = stop_capture.clone();
            move |result: Result<DesktopFrame, CaptureError>| {
                let frame = match result {
                    Ok(frame) => frame,
                    // This error is expected on Wayland while waiting for the user
                    // to pick a screen with the XDG Desktop Portal.
                    Err(CaptureError::Temporary) => {
                        log::debug!("Temporary error capturing screen");
                        return;
                    }
                    Err(CaptureError::Permanent) => {
                        log::error!("Error capturing screen");
                        stop_capture.store(true, Ordering::Release);
                        return;
                    }
                };
                let height = frame.height().try_into().unwrap();
                let width = frame.width().try_into().unwrap();

                if width != stream_width || height != stream_height {
                    stream_width = width;
                    stream_height = height;
                    video_frame.buffer = I420Buffer::new(width, height);
                }

                let stride = frame.stride();
                let data = frame.data();

                let (s_y, s_u, s_v) = video_frame.buffer.strides();
                let (y, u, v) = video_frame.buffer.data_mut();
                yuv_helper::argb_to_i420(
                    data,
                    stride,
                    y,
                    s_y,
                    u,
                    s_u,
                    v,
                    s_v,
                    frame.width(),
                    frame.height(),
                );

                if let Some(video_source) = &video_source {
                    video_source.capture_frame(&video_frame);
                } else {
                    // This is the first time the callback has been called.
                    // Use the resolution from the DesktopFrame to create a video source
                    // and push it over a channel to be published from the async context.
                    let video_source_inner = NativeVideoSource::new(VideoResolution {
                        width: stream_width,
                        height: stream_height,
                    });

                    video_source_sender
                        .try_send(video_source_inner.clone())
                        .unwrap();

                    video_source = Some(video_source_inner);
                }
            }
        };

        // source should only be None in tests which have a different implementation
        // of this function.
        let source = source.unwrap();
        let screen_id = source.id();

        let mut capturer = DesktopCapturer::new(desktop_capturer_options())
            .ok_or(anyhow!("Failed to create DesktopCapturer"))?;
        capturer.start_capture(Some(source), callback);
        log::debug!("Starting screen capture");

        let spawn_handle = gpui_tokio::Tokio::spawn(cx, {
            let stop_capture = stop_capture.clone();
            async move {
                loop {
                    if stop_capture.load(Ordering::Acquire) {
                        log::debug!("Stopping screen capture");
                        break;
                    }
                    capturer.capture_frame();
                    tokio::time::sleep(Duration::from_secs_f32(1.0 / 60.0)).await;
                }
            }
        });

        let video_source = video_source_receiver.next().await.ok_or(anyhow!(
            "No NativeVideoSource received from DesktopCapturer"
        ))?;
        let track = livekit::track::LocalVideoTrack::create_video_track(
            "screen_share",
            RtcVideoSource::Native(video_source),
        );

        let publication = self
            .publish_track(
                livekit::track::LocalTrack::Video(track),
                TrackPublishOptions {
                    source: TrackSource::Screenshare,
                    video_codec: VideoCodec::VP8,
                    ..Default::default()
                },
                cx,
            )
            .await?;
        let handle = ScreenCaptureStreamHandle {
            screen_id,
            stop_capture,
            _spawn_handle: spawn_handle,
        };
        Ok((publication, handle))
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
        })
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
        Tokio::spawn(cx, async move { participant.unpublish_track(&sid).await })
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
