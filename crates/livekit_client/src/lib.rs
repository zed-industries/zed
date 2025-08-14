use anyhow::Context as _;
use collections::HashMap;

mod remote_video_track_view;
use cpal::traits::HostTrait as _;
pub use remote_video_track_view::{RemoteVideoTrackView, RemoteVideoTrackViewEvent};
use rodio::DeviceTrait as _;

mod record;
pub use record::CaptureInput;

#[cfg(not(any(
    test,
    feature = "test-support",
    all(target_os = "windows", target_env = "gnu"),
    target_os = "freebsd"
)))]
mod livekit_client;
#[cfg(not(any(
    test,
    feature = "test-support",
    all(target_os = "windows", target_env = "gnu"),
    target_os = "freebsd"
)))]
pub use livekit_client::*;

// If you need proper LSP in livekit_client you've got to comment out
// the mocks and test
#[cfg(any(
    test,
    feature = "test-support",
    all(target_os = "windows", target_env = "gnu"),
    target_os = "freebsd"
))]
mod mock_client;
#[cfg(any(
    test,
    feature = "test-support",
    all(target_os = "windows", target_env = "gnu"),
    target_os = "freebsd"
))]
pub mod test;
#[cfg(any(
    test,
    feature = "test-support",
    all(target_os = "windows", target_env = "gnu"),
    target_os = "freebsd"
))]
pub use mock_client::*;

#[derive(Debug, Clone)]
pub enum Participant {
    Local(LocalParticipant),
    Remote(RemoteParticipant),
}

#[derive(Debug, Clone)]
pub enum TrackPublication {
    Local(LocalTrackPublication),
    Remote(RemoteTrackPublication),
}

impl TrackPublication {
    pub fn sid(&self) -> TrackSid {
        match self {
            TrackPublication::Local(local) => local.sid(),
            TrackPublication::Remote(remote) => remote.sid(),
        }
    }

    pub fn is_muted(&self) -> bool {
        match self {
            TrackPublication::Local(local) => local.is_muted(),
            TrackPublication::Remote(remote) => remote.is_muted(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum RemoteTrack {
    Audio(RemoteAudioTrack),
    Video(RemoteVideoTrack),
}

impl RemoteTrack {
    pub fn sid(&self) -> TrackSid {
        match self {
            RemoteTrack::Audio(remote_audio_track) => remote_audio_track.sid(),
            RemoteTrack::Video(remote_video_track) => remote_video_track.sid(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum LocalTrack {
    Audio(LocalAudioTrack),
    Video(LocalVideoTrack),
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
    ConnectionStateChanged(ConnectionState),
    Connected {
        participants_with_tracks: Vec<(RemoteParticipant, Vec<RemoteTrackPublication>)>,
    },
    Disconnected {
        reason: &'static str,
    },
    Reconnecting,
    Reconnected,
}

pub(crate) fn default_device(
    input: bool,
) -> anyhow::Result<(cpal::Device, cpal::SupportedStreamConfig)> {
    let device;
    let config;
    if input {
        device = cpal::default_host()
            .default_input_device()
            .context("no audio input device available")?;
        config = device
            .default_input_config()
            .context("failed to get default input config")?;
    } else {
        device = cpal::default_host()
            .default_output_device()
            .context("no audio output device available")?;
        config = device
            .default_output_config()
            .context("failed to get default output config")?;
    }
    Ok((device, config))
}

pub(crate) fn get_sample_data(
    sample_format: cpal::SampleFormat,
    data: &cpal::Data,
) -> anyhow::Result<Vec<i16>> {
    match sample_format {
        cpal::SampleFormat::I8 => Ok(convert_sample_data::<i8, i16>(data)),
        cpal::SampleFormat::I16 => Ok(data.as_slice::<i16>().unwrap().to_vec()),
        cpal::SampleFormat::I24 => Ok(convert_sample_data::<cpal::I24, i16>(data)),
        cpal::SampleFormat::I32 => Ok(convert_sample_data::<i32, i16>(data)),
        cpal::SampleFormat::I64 => Ok(convert_sample_data::<i64, i16>(data)),
        cpal::SampleFormat::U8 => Ok(convert_sample_data::<u8, i16>(data)),
        cpal::SampleFormat::U16 => Ok(convert_sample_data::<u16, i16>(data)),
        cpal::SampleFormat::U32 => Ok(convert_sample_data::<u32, i16>(data)),
        cpal::SampleFormat::U64 => Ok(convert_sample_data::<u64, i16>(data)),
        cpal::SampleFormat::F32 => Ok(convert_sample_data::<f32, i16>(data)),
        cpal::SampleFormat::F64 => Ok(convert_sample_data::<f64, i16>(data)),
        _ => anyhow::bail!("Unsupported sample format"),
    }
}

pub(crate) fn convert_sample_data<
    TSource: cpal::SizedSample,
    TDest: cpal::SizedSample + cpal::FromSample<TSource>,
>(
    data: &cpal::Data,
) -> Vec<TDest> {
    data.as_slice::<TSource>()
        .unwrap()
        .iter()
        .map(|e| e.to_sample::<TDest>())
        .collect()
}
