use gpui::BackgroundExecutor;

use crate::test;

pub(crate) mod participant;
pub(crate) mod publication;
pub(crate) mod track;

pub type RemoteVideoTrack = track::RemoteVideoTrack;
pub type RemoteAudioTrack = track::RemoteAudioTrack;
pub type RemoteTrackPublication = publication::RemoteTrackPublication;
pub type RemoteParticipant = participant::RemoteParticipant;

pub type LocalVideoTrack = track::LocalVideoTrack;
pub type LocalAudioTrack = track::LocalAudioTrack;
pub type LocalTrackPublication = publication::LocalTrackPublication;
pub type LocalParticipant = participant::LocalParticipant;

pub type Room = test::Room;
pub use test::{ConnectionState, ParticipantIdentity, TrackSid};

pub struct AudioStream {}

pub fn play_remote_audio_track(
    _track: &RemoteAudioTrack,
    _background_executor: &BackgroundExecutor,
) -> anyhow::Result<AudioStream> {
    Ok(AudioStream {})
}

#[derive(Clone)]
pub(crate) struct RemoteVideoFrame {}
impl Into<gpui::SurfaceSource> for RemoteVideoFrame {
    fn into(self) -> gpui::SurfaceSource {
        unimplemented!()
    }
}
pub(crate) fn play_remote_video_track(
    _track: &crate::RemoteVideoTrack,
) -> impl futures::Stream<Item = RemoteVideoFrame> {
    futures::stream::empty()
}
