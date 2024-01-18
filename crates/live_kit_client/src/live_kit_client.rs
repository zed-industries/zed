use std::sync::Arc;

#[cfg(not(any(test, feature = "test-support")))]
pub mod prod;

#[cfg(not(any(test, feature = "test-support")))]
pub use prod::*;

#[cfg(any(test, feature = "test-support"))]
pub mod test;

#[cfg(any(test, feature = "test-support"))]
pub use test::*;

pub type Sid = String;

#[derive(Clone, Eq, PartialEq)]
pub enum ConnectionState {
    Disconnected,
    Connected { url: String, token: String },
}

#[derive(Clone)]
pub enum RoomUpdate {
    ActiveSpeakersChanged { speakers: Vec<Sid> },
    RemoteAudioTrackMuteChanged { track_id: Sid, muted: bool },
    SubscribedToRemoteVideoTrack(Arc<RemoteVideoTrack>),
    SubscribedToRemoteAudioTrack(Arc<RemoteAudioTrack>, Arc<RemoteTrackPublication>),
    UnsubscribedFromRemoteVideoTrack { publisher_id: Sid, track_id: Sid },
    UnsubscribedFromRemoteAudioTrack { publisher_id: Sid, track_id: Sid },
    LocalAudioTrackPublished { publication: LocalTrackPublication },
    LocalAudioTrackUnpublished { publication: LocalTrackPublication },
    LocalVideoTrackPublished { publication: LocalTrackPublication },
    LocalVideoTrackUnpublished { publication: LocalTrackPublication },
}
