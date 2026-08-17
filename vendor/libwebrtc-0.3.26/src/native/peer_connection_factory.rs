// Copyright 2025 LiveKit, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

use cxx::{SharedPtr, UniquePtr};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use webrtc_sys::{peer_connection_factory as sys_pcf, rtc_error as sys_err, webrtc as sys_rtc};

use crate::{
    audio_source::native::NativeAudioSource,
    audio_track::RtcAudioTrack,
    imp::{audio_track as imp_at, peer_connection as imp_pc, video_track as imp_vt},
    peer_connection::PeerConnection,
    peer_connection_factory::RtcConfiguration,
    rtp_parameters::RtpCapabilities,
    video_source::native::NativeVideoSource,
    video_track::RtcVideoTrack,
    MediaType, RtcError,
};

lazy_static! {
    static ref LOG_SINK: Mutex<Option<UniquePtr<sys_rtc::ffi::LogSink>>> = Default::default();
}

#[derive(Clone)]
pub struct PeerConnectionFactory {
    pub(crate) sys_handle: SharedPtr<sys_pcf::ffi::PeerConnectionFactory>,
}

impl Default for PeerConnectionFactory {
    fn default() -> Self {
        let mut log_sink = LOG_SINK.lock();
        if log_sink.is_none() {
            *log_sink = Some(sys_rtc::ffi::new_log_sink(|msg, _| {
                let msg = msg.strip_suffix("\r\n").or(msg.strip_suffix('\n')).unwrap_or(&msg);

                log::debug!(target: "libwebrtc", "{}", msg);
            }));
        }

        Self { sys_handle: sys_pcf::ffi::create_peer_connection_factory() }
    }
}

impl PeerConnectionFactory {
    pub fn create_peer_connection(
        &self,
        config: RtcConfiguration,
    ) -> Result<PeerConnection, RtcError> {
        let observer = Arc::new(imp_pc::PeerObserver::default());
        let res = self.sys_handle.create_peer_connection(
            config.into(),
            Box::new(sys_pcf::PeerConnectionObserverWrapper::new(observer.clone())),
        );

        match res {
            Ok(sys_handle) => Ok(PeerConnection {
                handle: imp_pc::PeerConnection::configure(sys_handle, observer),
            }),
            Err(e) => Err(sys_err::ffi::RtcError::from(e.what()).into()),
        }
    }

    pub fn create_video_track(&self, label: &str, source: NativeVideoSource) -> RtcVideoTrack {
        RtcVideoTrack {
            handle: imp_vt::RtcVideoTrack {
                sys_handle: self
                    .sys_handle
                    .create_video_track(label.to_string(), source.handle.sys_handle()),
            },
        }
    }

    pub fn create_audio_track(&self, label: &str, source: NativeAudioSource) -> RtcAudioTrack {
        RtcAudioTrack {
            handle: imp_at::RtcAudioTrack {
                sys_handle: self
                    .sys_handle
                    .create_audio_track(label.to_string(), source.handle.sys_handle()),
            },
        }
    }

    pub fn get_rtp_sender_capabilities(&self, media_type: MediaType) -> RtpCapabilities {
        self.sys_handle.rtp_sender_capabilities(media_type.into()).into()
    }

    pub fn get_rtp_receiver_capabilities(&self, media_type: MediaType) -> RtpCapabilities {
        self.sys_handle.rtp_receiver_capabilities(media_type.into()).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_peer_connection_factory() {
        let _ = env_logger::builder().is_test(true).try_init();

        let factory = PeerConnectionFactory::default();
        let source = NativeVideoSource::default();
        let _track = factory.create_video_track("test", source);
        drop(factory);
    }
}
