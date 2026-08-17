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

use cxx::SharedPtr;
use parking_lot::Mutex;
use tokio::sync::{mpsc, oneshot};
use webrtc_sys::{
    data_channel as sys_dc, jsep as sys_jsep, peer_connection as sys_pc,
    peer_connection_factory as sys_pcf, rtc_error as sys_err,
};

use crate::{
    data_channel::{DataChannel, DataChannelInit},
    ice_candidate::IceCandidate,
    imp::{
        data_channel as imp_dc, ice_candidate as imp_ic, media_stream as imp_ms,
        media_stream_track as imp_mst, rtp_receiver as imp_rr, rtp_sender as imp_rs,
        rtp_transceiver as imp_rt, session_description as imp_sdp,
    },
    media_stream::MediaStream,
    media_stream_track::MediaStreamTrack,
    peer_connection::{
        AnswerOptions, IceCandidateError, IceConnectionState, IceGatheringState, OfferOptions,
        OnConnectionChange, OnDataChannel, OnIceCandidate, OnIceCandidateError,
        OnIceConnectionChange, OnIceGatheringChange, OnNegotiationNeeded, OnSignalingChange,
        OnTrack, PeerConnectionState, SignalingState, TrackEvent,
    },
    peer_connection_factory::{
        ContinualGatheringPolicy, IceServer, IceTransportsType, RtcConfiguration,
    },
    rtp_receiver::RtpReceiver,
    rtp_sender::RtpSender,
    rtp_transceiver::{RtpTransceiver, RtpTransceiverInit},
    session_description::SessionDescription,
    stats::RtcStats,
    MediaType, RtcError, RtcErrorType,
};

impl From<OfferOptions> for sys_pc::ffi::RtcOfferAnswerOptions {
    fn from(options: OfferOptions) -> Self {
        Self {
            ice_restart: options.ice_restart,
            offer_to_receive_audio: options.offer_to_receive_audio as i32,
            offer_to_receive_video: options.offer_to_receive_video as i32,
            ..Default::default()
        }
    }
}

impl From<AnswerOptions> for sys_pc::ffi::RtcOfferAnswerOptions {
    fn from(_options: AnswerOptions) -> Self {
        Self::default()
    }
}

impl From<sys_pc::ffi::PeerConnectionState> for PeerConnectionState {
    fn from(state: sys_pc::ffi::PeerConnectionState) -> Self {
        match state {
            sys_pc::ffi::PeerConnectionState::New => PeerConnectionState::New,
            sys_pc::ffi::PeerConnectionState::Connecting => PeerConnectionState::Connecting,
            sys_pc::ffi::PeerConnectionState::Connected => PeerConnectionState::Connected,
            sys_pc::ffi::PeerConnectionState::Disconnected => PeerConnectionState::Disconnected,
            sys_pc::ffi::PeerConnectionState::Failed => PeerConnectionState::Failed,
            sys_pc::ffi::PeerConnectionState::Closed => PeerConnectionState::Closed,
            _ => panic!("unknown PeerConnectionState"),
        }
    }
}

impl From<sys_pc::ffi::IceConnectionState> for IceConnectionState {
    fn from(state: sys_pc::ffi::IceConnectionState) -> Self {
        match state {
            sys_pc::ffi::IceConnectionState::IceConnectionNew => IceConnectionState::New,
            sys_pc::ffi::IceConnectionState::IceConnectionChecking => IceConnectionState::Checking,
            sys_pc::ffi::IceConnectionState::IceConnectionConnected => {
                IceConnectionState::Connected
            }
            sys_pc::ffi::IceConnectionState::IceConnectionCompleted => {
                IceConnectionState::Completed
            }
            sys_pc::ffi::IceConnectionState::IceConnectionFailed => IceConnectionState::Failed,
            sys_pc::ffi::IceConnectionState::IceConnectionDisconnected => {
                IceConnectionState::Disconnected
            }
            sys_pc::ffi::IceConnectionState::IceConnectionClosed => IceConnectionState::Closed,
            sys_pc::ffi::IceConnectionState::IceConnectionMax => IceConnectionState::Max,
            _ => panic!("unknown IceConnectionState"),
        }
    }
}

impl From<sys_pc::ffi::IceGatheringState> for IceGatheringState {
    fn from(state: sys_pc::ffi::IceGatheringState) -> Self {
        match state {
            sys_pc::ffi::IceGatheringState::IceGatheringNew => IceGatheringState::New,
            sys_pc::ffi::IceGatheringState::IceGatheringGathering => IceGatheringState::Gathering,
            sys_pc::ffi::IceGatheringState::IceGatheringComplete => IceGatheringState::Complete,
            _ => panic!("unknown IceGatheringState"),
        }
    }
}

impl From<sys_pc::ffi::SignalingState> for SignalingState {
    fn from(state: sys_pc::ffi::SignalingState) -> Self {
        match state {
            sys_pc::ffi::SignalingState::Stable => SignalingState::Stable,
            sys_pc::ffi::SignalingState::HaveLocalOffer => SignalingState::HaveLocalOffer,
            sys_pc::ffi::SignalingState::HaveRemoteOffer => SignalingState::HaveRemoteOffer,
            sys_pc::ffi::SignalingState::HaveLocalPrAnswer => SignalingState::HaveLocalPrAnswer,
            sys_pc::ffi::SignalingState::HaveRemotePrAnswer => SignalingState::HaveRemotePrAnswer,
            sys_pc::ffi::SignalingState::Closed => SignalingState::Closed,
            _ => panic!("unknown SignalingState"),
        }
    }
}

impl From<IceServer> for sys_pc::ffi::IceServer {
    fn from(value: IceServer) -> Self {
        sys_pc::ffi::IceServer {
            urls: value.urls,
            username: value.username,
            password: value.password,
        }
    }
}

impl From<ContinualGatheringPolicy> for sys_pc::ffi::ContinualGatheringPolicy {
    fn from(value: ContinualGatheringPolicy) -> Self {
        match value {
            ContinualGatheringPolicy::GatherOnce => {
                sys_pc::ffi::ContinualGatheringPolicy::GatherOnce
            }
            ContinualGatheringPolicy::GatherContinually => {
                sys_pc::ffi::ContinualGatheringPolicy::GatherContinually
            }
        }
    }
}

impl From<IceTransportsType> for sys_pc::ffi::IceTransportsType {
    fn from(value: IceTransportsType) -> Self {
        match value {
            IceTransportsType::Relay => sys_pc::ffi::IceTransportsType::Relay,
            IceTransportsType::NoHost => sys_pc::ffi::IceTransportsType::NoHost,
            IceTransportsType::All => sys_pc::ffi::IceTransportsType::All,
        }
    }
}

impl From<RtcConfiguration> for sys_pc::ffi::RtcConfiguration {
    fn from(value: RtcConfiguration) -> Self {
        Self {
            ice_servers: value.ice_servers.into_iter().map(Into::into).collect(),
            continual_gathering_policy: value.continual_gathering_policy.into(),
            ice_transport_type: value.ice_transport_type.into(),
        }
    }
}

#[derive(Clone)]
pub struct PeerConnection {
    observer: Arc<PeerObserver>,
    pub(crate) sys_handle: SharedPtr<sys_pc::ffi::PeerConnection>,
}

impl PeerConnection {
    pub fn configure(
        sys_handle: SharedPtr<sys_pc::ffi::PeerConnection>,
        observer: Arc<PeerObserver>,
    ) -> Self {
        Self { sys_handle, observer }
    }

    pub fn set_configuration(&self, config: RtcConfiguration) -> Result<(), RtcError> {
        let res = self.sys_handle.set_configuration(config.into());

        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(sys_err::ffi::RtcError::from(e.what()).into()),
        }
    }

    pub async fn create_offer(
        &self,
        options: OfferOptions,
    ) -> Result<SessionDescription, RtcError> {
        let (tx, mut rx) = mpsc::channel::<Result<SessionDescription, RtcError>>(1);
        let ctx = Box::new(sys_pc::PeerContext(Box::new(tx)));
        type CtxType = mpsc::Sender<Result<SessionDescription, RtcError>>;

        self.sys_handle.create_offer(
            options.into(),
            ctx,
            |ctx, sdp| {
                let tx = *ctx.0.downcast::<CtxType>().unwrap();
                let _ = tx.blocking_send(Ok(SessionDescription {
                    handle: imp_sdp::SessionDescription { sys_handle: sdp },
                }));
            },
            |ctx, error| {
                let tx = *ctx.0.downcast::<CtxType>().unwrap();
                let _ = tx.blocking_send(Err(error.into()));
            },
        );

        rx.recv().await.unwrap()
    }

    pub async fn create_answer(
        &self,
        options: AnswerOptions,
    ) -> Result<SessionDescription, RtcError> {
        let (tx, mut rx) = mpsc::channel::<Result<SessionDescription, RtcError>>(1);
        let ctx = Box::new(sys_pc::PeerContext(Box::new(tx)));
        type CtxType = mpsc::Sender<Result<SessionDescription, RtcError>>;

        self.sys_handle.create_answer(
            options.into(),
            ctx,
            |ctx, sdp| {
                let tx = *ctx.0.downcast::<CtxType>().unwrap();
                let _ = tx.blocking_send(Ok(SessionDescription {
                    handle: imp_sdp::SessionDescription { sys_handle: sdp },
                }));
            },
            |ctx, error| {
                let tx = *ctx.0.downcast::<CtxType>().unwrap();
                let _ = tx.blocking_send(Err(error.into()));
            },
        );

        rx.recv().await.unwrap()
    }

    pub async fn set_local_description(&self, desc: SessionDescription) -> Result<(), RtcError> {
        let (tx, rx) = oneshot::channel::<Result<(), RtcError>>();
        let ctx = Box::new(sys_pc::PeerContext(Box::new(tx)));

        self.sys_handle.set_local_description(desc.handle.sys_handle, ctx, |ctx, err| {
            let tx = ctx.0.downcast::<oneshot::Sender<Result<(), RtcError>>>().unwrap();

            if err.ok() {
                let _ = tx.send(Ok(()));
            } else {
                let _ = tx.send(Err(err.into()));
            }
        });

        rx.await.unwrap()
    }

    pub async fn set_remote_description(&self, desc: SessionDescription) -> Result<(), RtcError> {
        let (tx, rx) = oneshot::channel::<Result<(), RtcError>>();
        let ctx = Box::new(sys_pc::PeerContext(Box::new(tx)));

        self.sys_handle.set_remote_description(desc.handle.sys_handle, ctx, |ctx, err| {
            let tx = ctx.0.downcast::<oneshot::Sender<Result<(), RtcError>>>().unwrap();

            if err.ok() {
                let _ = tx.send(Ok(()));
            } else {
                let _ = tx.send(Err(err.into()));
            }
        });

        rx.await.map_err(|_| RtcError {
            error_type: RtcErrorType::Internal,
            message: "set_remote_description cancelled".to_owned(),
        })?
    }

    pub async fn add_ice_candidate(&self, candidate: IceCandidate) -> Result<(), RtcError> {
        let (tx, rx) = oneshot::channel::<Result<(), RtcError>>();
        let ctx = Box::new(sys_pc::PeerContext(Box::new(tx)));

        self.sys_handle.add_ice_candidate(candidate.handle.sys_handle, ctx, |ctx, err| {
            let tx = ctx.0.downcast::<oneshot::Sender<Result<(), RtcError>>>().unwrap();

            if err.ok() {
                let _ = tx.send(Ok(()));
            } else {
                let _ = tx.send(Err(err.into()));
            }
        });

        rx.await.map_err(|_| RtcError {
            error_type: RtcErrorType::Internal,
            message: "add_ice_candidate cancelled".to_owned(),
        })?
    }

    pub fn create_data_channel(
        &self,
        label: &str,
        init: DataChannelInit,
    ) -> Result<DataChannel, RtcError> {
        let res = self.sys_handle.create_data_channel(label.to_string(), init.into());

        match res {
            Ok(sys_handle) => {
                Ok(DataChannel { handle: imp_dc::DataChannel::configure(sys_handle) })
            }
            Err(e) => Err(sys_err::ffi::RtcError::from(e.what()).into()),
        }
    }

    pub fn add_track<T: AsRef<str>>(
        &self,
        track: MediaStreamTrack,
        stream_ids: &[T],
    ) -> Result<RtpSender, RtcError> {
        let stream_ids = stream_ids.iter().map(|s| s.as_ref().to_owned()).collect();
        let res = self.sys_handle.add_track(track.sys_handle(), &stream_ids);

        match res {
            Ok(sys_handle) => Ok(RtpSender { handle: imp_rs::RtpSender { sys_handle } }),
            Err(e) => Err(sys_err::ffi::RtcError::from(e.what()).into()),
        }
    }

    pub fn add_transceiver(
        &self,
        track: MediaStreamTrack,
        init: RtpTransceiverInit,
    ) -> Result<RtpTransceiver, RtcError> {
        let res = self.sys_handle.add_transceiver(track.sys_handle(), init.into());

        match res {
            Ok(sys_handle) => Ok(RtpTransceiver { handle: imp_rt::RtpTransceiver { sys_handle } }),
            Err(e) => Err(sys_err::ffi::RtcError::from(e.what()).into()),
        }
    }

    pub fn add_transceiver_for_media(
        &self,
        media_type: MediaType,
        init: RtpTransceiverInit,
    ) -> Result<RtpTransceiver, RtcError> {
        let res = self.sys_handle.add_transceiver_for_media(media_type.into(), init.into());

        match res {
            Ok(cxx_handle) => {
                Ok(RtpTransceiver { handle: imp_rt::RtpTransceiver { sys_handle: cxx_handle } })
            }
            Err(e) => Err(sys_err::ffi::RtcError::from(e.what()).into()),
        }
    }

    pub fn restart_ice(&self) {
        self.sys_handle.restart_ice();
    }

    pub fn close(&self) {
        self.sys_handle.close();
    }

    pub fn connection_state(&self) -> PeerConnectionState {
        self.sys_handle.connection_state().into()
    }

    pub fn ice_connection_state(&self) -> IceConnectionState {
        self.sys_handle.ice_connection_state().into()
    }

    pub fn ice_gathering_state(&self) -> IceGatheringState {
        self.sys_handle.ice_gathering_state().into()
    }

    pub fn signaling_state(&self) -> SignalingState {
        self.sys_handle.signaling_state().into()
    }

    pub fn current_local_description(&self) -> Option<SessionDescription> {
        let sdp = self.sys_handle.current_local_description();
        if sdp.is_null() {
            return None;
        }

        Some(SessionDescription { handle: imp_sdp::SessionDescription { sys_handle: sdp } })
    }

    pub fn current_remote_description(&self) -> Option<SessionDescription> {
        let sdp = self.sys_handle.current_remote_description();
        if sdp.is_null() {
            return None;
        }

        Some(SessionDescription { handle: imp_sdp::SessionDescription { sys_handle: sdp } })
    }

    pub fn remove_track(&self, sender: RtpSender) -> Result<(), RtcError> {
        self.sys_handle
            .remove_track(sender.handle.sys_handle)
            .map_err(|e| sys_err::ffi::RtcError::from(e.what()).into())
    }

    pub async fn get_stats(&self) -> Result<Vec<RtcStats>, RtcError> {
        let (tx, rx) = oneshot::channel::<Result<Vec<RtcStats>, RtcError>>();
        let ctx = Box::new(sys_pc::PeerContext(Box::new(tx)));

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

    pub fn senders(&self) -> Vec<RtpSender> {
        self.sys_handle
            .get_senders()
            .into_iter()
            .map(|sender| RtpSender { handle: imp_rs::RtpSender { sys_handle: sender.ptr } })
            .collect()
    }

    pub fn receivers(&self) -> Vec<RtpReceiver> {
        self.sys_handle
            .get_receivers()
            .into_iter()
            .map(|receiver| RtpReceiver {
                handle: imp_rr::RtpReceiver { sys_handle: receiver.ptr },
            })
            .collect()
    }

    pub fn transceivers(&self) -> Vec<RtpTransceiver> {
        self.sys_handle
            .get_transceivers()
            .into_iter()
            .map(|transceiver| RtpTransceiver {
                handle: imp_rt::RtpTransceiver { sys_handle: transceiver.ptr },
            })
            .collect()
    }

    pub fn on_connection_state_change(&self, f: Option<OnConnectionChange>) {
        *self.observer.connection_change_handler.lock() = f;
    }

    pub fn on_data_channel(&self, f: Option<OnDataChannel>) {
        *self.observer.data_channel_handler.lock() = f;
    }

    pub fn on_ice_candidate(&self, f: Option<OnIceCandidate>) {
        *self.observer.ice_candidate_handler.lock() = f;
    }

    pub fn on_ice_candidate_error(&self, f: Option<OnIceCandidateError>) {
        *self.observer.ice_candidate_error_handler.lock() = f;
    }

    pub fn on_ice_connection_state_change(&self, f: Option<OnIceConnectionChange>) {
        *self.observer.ice_connection_change_handler.lock() = f;
    }

    pub fn on_ice_gathering_state_change(&self, f: Option<OnIceGatheringChange>) {
        *self.observer.ice_gathering_change_handler.lock() = f;
    }

    pub fn on_negotiation_needed(&self, f: Option<OnNegotiationNeeded>) {
        *self.observer.negotiation_needed_handler.lock() = f;
    }

    pub fn on_signaling_state_change(&self, f: Option<OnSignalingChange>) {
        *self.observer.signaling_change_handler.lock() = f;
    }

    pub fn on_track(&self, f: Option<OnTrack>) {
        *self.observer.track_handler.lock() = f;
    }
}

#[derive(Default)]
pub struct PeerObserver {
    pub connection_change_handler: Mutex<Option<OnConnectionChange>>,
    pub data_channel_handler: Mutex<Option<OnDataChannel>>,
    pub ice_candidate_handler: Mutex<Option<OnIceCandidate>>,
    pub ice_candidate_error_handler: Mutex<Option<OnIceCandidateError>>,
    pub ice_connection_change_handler: Mutex<Option<OnIceConnectionChange>>,
    pub ice_gathering_change_handler: Mutex<Option<OnIceGatheringChange>>,
    pub negotiation_needed_handler: Mutex<Option<OnNegotiationNeeded>>,
    pub signaling_change_handler: Mutex<Option<OnSignalingChange>>,
    pub track_handler: Mutex<Option<OnTrack>>,
}

impl sys_pcf::PeerConnectionObserver for PeerObserver {
    fn on_signaling_change(&self, new_state: sys_pc::ffi::SignalingState) {
        if let Some(f) = self.signaling_change_handler.lock().as_mut() {
            f(new_state.into());
        }
    }

    fn on_add_stream(&self, _stream: SharedPtr<webrtc_sys::media_stream::ffi::MediaStream>) {}

    fn on_remove_stream(&self, _stream: SharedPtr<webrtc_sys::media_stream::ffi::MediaStream>) {}

    fn on_data_channel(&self, data_channel: SharedPtr<sys_dc::ffi::DataChannel>) {
        if let Some(f) = self.data_channel_handler.lock().as_mut() {
            f(DataChannel { handle: imp_dc::DataChannel::configure(data_channel) });
        }
    }

    fn on_renegotiation_needed(&self) {}

    fn on_negotiation_needed_event(&self, event: u32) {
        if let Some(f) = self.negotiation_needed_handler.lock().as_mut() {
            f(event);
        }
    }

    fn on_ice_connection_change(&self, _new_state: sys_pc::ffi::IceConnectionState) {}

    fn on_standardized_ice_connection_change(&self, new_state: sys_pc::ffi::IceConnectionState) {
        if let Some(f) = self.ice_connection_change_handler.lock().as_mut() {
            f(new_state.into());
        }
    }

    fn on_connection_change(&self, new_state: sys_pc::ffi::PeerConnectionState) {
        if let Some(f) = self.connection_change_handler.lock().as_mut() {
            f(new_state.into());
        }
    }

    fn on_ice_gathering_change(&self, new_state: sys_pc::ffi::IceGatheringState) {
        if let Some(f) = self.ice_gathering_change_handler.lock().as_mut() {
            f(new_state.into());
        }
    }

    fn on_ice_candidate(&self, candidate: SharedPtr<sys_jsep::ffi::IceCandidate>) {
        if let Some(f) = self.ice_candidate_handler.lock().as_mut() {
            f(IceCandidate { handle: imp_ic::IceCandidate { sys_handle: candidate } });
        }
    }

    fn on_ice_candidate_error(
        &self,
        address: String,
        port: i32,
        url: String,
        error_code: i32,
        error_text: String,
    ) {
        if let Some(f) = self.ice_candidate_error_handler.lock().as_mut() {
            f(IceCandidateError { address, port, url, error_code, error_text });
        }
    }

    fn on_ice_candidates_removed(
        &self,
        _removed: Vec<SharedPtr<webrtc_sys::candidate::ffi::Candidate>>,
    ) {
    }

    fn on_ice_connection_receiving_change(&self, _receiving: bool) {}

    fn on_ice_selected_candidate_pair_changed(
        &self,
        _event: sys_pcf::ffi::CandidatePairChangeEvent,
    ) {
    }

    fn on_add_track(
        &self,
        _receiver: SharedPtr<webrtc_sys::rtp_receiver::ffi::RtpReceiver>,
        _streams: Vec<SharedPtr<webrtc_sys::media_stream::ffi::MediaStream>>,
    ) {
    }

    fn on_track(&self, transceiver: SharedPtr<webrtc_sys::rtp_transceiver::ffi::RtpTransceiver>) {
        if let Some(f) = self.track_handler.lock().as_mut() {
            let receiver = transceiver.receiver();
            let streams = receiver.streams();
            let track = receiver.track();

            f(TrackEvent {
                receiver: RtpReceiver { handle: imp_rr::RtpReceiver { sys_handle: receiver } },
                streams: streams
                    .into_iter()
                    .map(|s| MediaStream { handle: imp_ms::MediaStream { sys_handle: s.ptr } })
                    .collect(),
                track: imp_mst::new_media_stream_track(track),
                transceiver: RtpTransceiver {
                    handle: imp_rt::RtpTransceiver { sys_handle: transceiver },
                },
            });
        }
    }

    fn on_remove_track(&self, _receiver: SharedPtr<webrtc_sys::rtp_receiver::ffi::RtpReceiver>) {}

    fn on_interesting_usage(&self, _usage_pattern: i32) {}
}
