use std::sync::Arc;

// #[cfg(all(
//     target_os = "macos",
//     not(any(test, feature = "test-support")),
//     not(feature = "livekit-rust-sdk")
// ))]
// pub mod prod;

// #[cfg(all(
//     target_os = "macos",
//     not(any(test, feature = "test-support")),
//     not(feature = "livekit-rust-sdk")
// ))]
// pub use prod::*;

// #[cfg(any(test, feature = "test-support", not(target_os = "macos")))]
// pub mod test;

// #[cfg(any(test, feature = "test-support", not(target_os = "macos")))]
// pub use test::*;

#[cfg(all(
    target_os = "macos",
    not(any(test, feature = "test-support")),
    feature = "livekit-rust-sdk"
))]
pub mod rust;

#[cfg(all(
    target_os = "macos",
    not(any(test, feature = "test-support")),
    feature = "livekit-rust-sdk"
))]
pub use rust::*;

pub type Sid = String;

// TEMPORARY TYPES FOR COMPILATION PURPOSES
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct MacOsDisplayPointer(pub *const std::ffi::c_void);
unsafe impl Send for MacOsDisplayPointer {}

pub trait PlatformDisplayAbstractor {
    fn get_pointer(&self) -> MacOsDisplayPointer;
}

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
