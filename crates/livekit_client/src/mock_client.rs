use gpui::BackgroundExecutor;

use crate::test;

pub type RemoteVideoTrack = test::track::RemoteVideoTrack;
pub type RemoteAudioTrack = test::track::RemoteAudioTrack;
pub type RemoteTrackPublication = test::publication::RemoteTrackPublication;
pub type RemoteParticipant = test::participant::RemoteParticipant;

pub type LocalVideoTrack = test::track::LocalVideoTrack;
pub type LocalAudioTrack = test::track::LocalAudioTrack;
pub type LocalTrackPublication = test::publication::LocalTrackPublication;
pub type LocalParticipant = test::participant::LocalParticipant;

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
