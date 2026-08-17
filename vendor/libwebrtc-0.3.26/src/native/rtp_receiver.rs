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

use cxx::SharedPtr;
use tokio::sync::oneshot;
use webrtc_sys::rtp_receiver as sys_rr;

use crate::{
    imp::media_stream_track::new_media_stream_track, media_stream_track::MediaStreamTrack,
    rtp_parameters::RtpParameters, stats::RtcStats, RtcError, RtcErrorType,
};

#[derive(Clone)]
pub struct RtpReceiver {
    pub(crate) sys_handle: SharedPtr<sys_rr::ffi::RtpReceiver>,
}

impl RtpReceiver {
    pub fn track(&self) -> Option<MediaStreamTrack> {
        let track_handle = self.sys_handle.track();
        if track_handle.is_null() {
            return None;
        }

        Some(new_media_stream_track(track_handle))
    }

    pub async fn get_stats(&self) -> Result<Vec<RtcStats>, RtcError> {
        let (tx, rx) = oneshot::channel::<Result<Vec<RtcStats>, RtcError>>();
        let ctx = Box::new(sys_rr::ReceiverContext(Box::new(tx)));

        self.sys_handle.get_stats(ctx, |ctx, stats| {
            let tx = ctx.0.downcast::<oneshot::Sender<Result<Vec<RtcStats>, RtcError>>>().unwrap();

            if stats.is_empty() {
                let _ = tx.send(Ok(vec![]));
                return;
            }

            // Unwrap because it should not happens
            let vec = serde_json::from_str(&stats).unwrap();
            let _ = tx.send(Ok(vec));
        });

        rx.await.map_err(|_| RtcError {
            error_type: RtcErrorType::Internal,
            message: "get_stats cancelled".to_owned(),
        })?
    }

    pub fn parameters(&self) -> RtpParameters {
        self.sys_handle.get_parameters().into()
    }
}
