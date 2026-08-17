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

use crate::{
    candidate::ffi::Candidate, data_channel::ffi::DataChannel, impl_thread_safety,
    jsep::ffi::IceCandidate, media_stream::ffi::MediaStream, rtp_receiver::ffi::RtpReceiver,
    rtp_transceiver::ffi::RtpTransceiver,
};

#[cxx::bridge(namespace = "livekit_ffi")]
pub mod ffi {
    pub struct CandidatePair {
        local: SharedPtr<Candidate>,
        remote: SharedPtr<Candidate>,
    }

    pub struct CandidatePairChangeEvent {
        selected_candidate_pair: CandidatePair,
        last_data_received_ms: i64,
        reason: String,
        estimated_disconnected_time_ms: i64,
    }

    extern "C++" {
        include!("livekit/rtp_parameters.h");
        include!("livekit/rtc_error.h");
        include!("livekit/helper.h");
        include!("livekit/candidate.h");
        include!("livekit/media_stream.h");
        include!("livekit/rtp_transceiver.h");
        include!("livekit/rtp_sender.h");
        include!("livekit/rtp_receiver.h");
        include!("livekit/data_channel.h");
        include!("livekit/jsep.h");
        include!("livekit/webrtc.h");
        include!("livekit/peer_connection.h");
        include!("livekit/audio_track.h");

        type RtcConfiguration = crate::peer_connection::ffi::RtcConfiguration;
        type PeerConnectionState = crate::peer_connection::ffi::PeerConnectionState;
        type SignalingState = crate::peer_connection::ffi::SignalingState;
        type IceConnectionState = crate::peer_connection::ffi::IceConnectionState;
        type IceGatheringState = crate::peer_connection::ffi::IceGatheringState;
        type AudioTrackSource = crate::audio_track::ffi::AudioTrackSource;
        type VideoTrackSource = crate::video_track::ffi::VideoTrackSource;
        type RtpCapabilities = crate::rtp_parameters::ffi::RtpCapabilities;
        type AudioTrack = crate::audio_track::ffi::AudioTrack;
        type VideoTrack = crate::video_track::ffi::VideoTrack;
        type MediaStreamPtr = crate::helper::ffi::MediaStreamPtr;
        type CandidatePtr = crate::helper::ffi::CandidatePtr;
        type RtpSenderPtr = crate::helper::ffi::RtpSenderPtr;
        type RtpReceiverPtr = crate::helper::ffi::RtpReceiverPtr;
        type RtpTransceiverPtr = crate::helper::ffi::RtpTransceiverPtr;
        type RtcError = crate::rtc_error::ffi::RtcError;
        type Candidate = crate::candidate::ffi::Candidate;
        type IceCandidate = crate::jsep::ffi::IceCandidate;
        type DataChannel = crate::data_channel::ffi::DataChannel;
        type DataChannelInit = crate::data_channel::ffi::DataChannelInit;
        type RtpSender = crate::rtp_sender::ffi::RtpSender;
        type RtpReceiver = crate::rtp_receiver::ffi::RtpReceiver;
        type RtpTransceiver = crate::rtp_transceiver::ffi::RtpTransceiver;
        type RtpTransceiverInit = crate::rtp_transceiver::ffi::RtpTransceiverInit;
        type MediaStream = crate::media_stream::ffi::MediaStream;
        type MediaStreamTrack = crate::media_stream::ffi::MediaStreamTrack;
        type SessionDescription = crate::jsep::ffi::SessionDescription;
        type MediaType = crate::webrtc::ffi::MediaType;
    }

    unsafe extern "C++" {
        include!("livekit/peer_connection_factory.h");

        type PeerConnection = crate::peer_connection::ffi::PeerConnection;
        type PeerConnectionFactory;

        fn create_peer_connection_factory() -> SharedPtr<PeerConnectionFactory>;

        fn create_peer_connection(
            self: &PeerConnectionFactory,
            config: RtcConfiguration,
            observer: Box<PeerConnectionObserverWrapper>,
        ) -> Result<SharedPtr<PeerConnection>>;

        fn create_video_track(
            self: &PeerConnectionFactory,
            label: String,
            source: SharedPtr<VideoTrackSource>,
        ) -> SharedPtr<VideoTrack>;

        fn create_audio_track(
            self: &PeerConnectionFactory,
            label: String,
            source: SharedPtr<AudioTrackSource>,
        ) -> SharedPtr<AudioTrack>;

        fn rtp_sender_capabilities(
            self: &PeerConnectionFactory,
            kind: MediaType,
        ) -> RtpCapabilities;

        fn rtp_receiver_capabilities(
            self: &PeerConnectionFactory,
            kind: MediaType,
        ) -> RtpCapabilities;
    }

    extern "Rust" {
        type PeerConnectionObserverWrapper;

        fn on_signaling_change(self: &PeerConnectionObserverWrapper, new_state: SignalingState);
        fn on_add_stream(self: &PeerConnectionObserverWrapper, stream: SharedPtr<MediaStream>);
        fn on_remove_stream(self: &PeerConnectionObserverWrapper, stream: SharedPtr<MediaStream>);
        fn on_data_channel(
            self: &PeerConnectionObserverWrapper,
            data_channel: SharedPtr<DataChannel>,
        );
        fn on_renegotiation_needed(self: &PeerConnectionObserverWrapper);
        fn on_negotiation_needed_event(self: &PeerConnectionObserverWrapper, event: u32);
        fn on_ice_connection_change(
            self: &PeerConnectionObserverWrapper,
            new_state: IceConnectionState,
        );
        fn on_standardized_ice_connection_change(
            self: &PeerConnectionObserverWrapper,
            new_state: IceConnectionState,
        );
        fn on_connection_change(
            self: &PeerConnectionObserverWrapper,
            new_state: PeerConnectionState,
        );
        fn on_ice_gathering_change(
            self: &PeerConnectionObserverWrapper,
            new_state: IceGatheringState,
        );
        fn on_ice_candidate(
            self: &PeerConnectionObserverWrapper,
            candidate: SharedPtr<IceCandidate>,
        );
        fn on_ice_candidate_error(
            self: &PeerConnectionObserverWrapper,
            address: String,
            port: i32,
            url: String,
            error_code: i32,
            error_text: String,
        );
        fn on_ice_candidates_removed(
            self: &PeerConnectionObserverWrapper,
            removed: Vec<CandidatePtr>,
        );
        fn on_ice_connection_receiving_change(
            self: &PeerConnectionObserverWrapper,
            receiving: bool,
        );
        fn on_ice_selected_candidate_pair_changed(
            self: &PeerConnectionObserverWrapper,
            event: CandidatePairChangeEvent,
        );
        fn on_add_track(
            self: &PeerConnectionObserverWrapper,
            receiver: SharedPtr<RtpReceiver>,
            streams: Vec<MediaStreamPtr>,
        );
        fn on_track(self: &PeerConnectionObserverWrapper, transceiver: SharedPtr<RtpTransceiver>);
        fn on_remove_track(self: &PeerConnectionObserverWrapper, receiver: SharedPtr<RtpReceiver>);
        fn on_interesting_usage(self: &PeerConnectionObserverWrapper, usage_pattern: i32);
    }
}

impl_thread_safety!(ffi::PeerConnectionFactory, Send + Sync);

pub trait PeerConnectionObserver: Send + Sync {
    fn on_signaling_change(&self, new_state: ffi::SignalingState);
    fn on_add_stream(&self, stream: SharedPtr<MediaStream>);
    fn on_remove_stream(&self, stream: SharedPtr<MediaStream>);
    fn on_data_channel(&self, data_channel: SharedPtr<DataChannel>);
    fn on_renegotiation_needed(&self);
    fn on_negotiation_needed_event(&self, event: u32);
    fn on_ice_connection_change(&self, new_state: ffi::IceConnectionState);
    fn on_standardized_ice_connection_change(&self, new_state: ffi::IceConnectionState);
    fn on_connection_change(&self, new_state: ffi::PeerConnectionState);
    fn on_ice_gathering_change(&self, new_state: ffi::IceGatheringState);
    fn on_ice_candidate(&self, candidate: SharedPtr<IceCandidate>);
    fn on_ice_candidate_error(
        &self,
        address: String,
        port: i32,
        url: String,
        error_code: i32,
        error_text: String,
    );
    fn on_ice_candidates_removed(&self, removed: Vec<SharedPtr<Candidate>>);
    fn on_ice_connection_receiving_change(&self, receiving: bool);
    fn on_ice_selected_candidate_pair_changed(&self, event: ffi::CandidatePairChangeEvent);
    fn on_add_track(&self, receiver: SharedPtr<RtpReceiver>, streams: Vec<SharedPtr<MediaStream>>);
    fn on_track(&self, transceiver: SharedPtr<RtpTransceiver>);
    fn on_remove_track(&self, receiver: SharedPtr<RtpReceiver>);
    fn on_interesting_usage(&self, usage_pattern: i32);
}

// Wrapper for PeerConnectionObserver because cxx doesn't support dyn Trait on c++
// https://github.com/dtolnay/cxx/issues/665
pub struct PeerConnectionObserverWrapper {
    observer: Arc<dyn PeerConnectionObserver>,
}

impl PeerConnectionObserverWrapper {
    pub fn new(observer: Arc<dyn PeerConnectionObserver>) -> Self {
        Self { observer }
    }

    fn on_signaling_change(&self, new_state: ffi::SignalingState) {
        self.observer.on_signaling_change(new_state);
    }

    fn on_add_stream(&self, stream: SharedPtr<MediaStream>) {
        self.observer.on_add_stream(stream);
    }

    fn on_remove_stream(&self, stream: SharedPtr<MediaStream>) {
        self.observer.on_remove_stream(stream);
    }

    fn on_data_channel(&self, data_channel: SharedPtr<DataChannel>) {
        self.observer.on_data_channel(data_channel);
    }

    fn on_renegotiation_needed(&self) {
        self.observer.on_renegotiation_needed();
    }

    fn on_negotiation_needed_event(&self, event: u32) {
        self.observer.on_negotiation_needed_event(event);
    }

    fn on_ice_connection_change(&self, new_state: ffi::IceConnectionState) {
        self.observer.on_ice_connection_change(new_state);
    }

    fn on_standardized_ice_connection_change(&self, new_state: ffi::IceConnectionState) {
        self.observer.on_standardized_ice_connection_change(new_state);
    }

    fn on_connection_change(&self, new_state: ffi::PeerConnectionState) {
        self.observer.on_connection_change(new_state);
    }

    fn on_ice_gathering_change(&self, new_state: ffi::IceGatheringState) {
        self.observer.on_ice_gathering_change(new_state);
    }

    fn on_ice_candidate(&self, candidate: SharedPtr<IceCandidate>) {
        self.observer.on_ice_candidate(candidate);
    }

    fn on_ice_candidate_error(
        &self,
        address: String,
        port: i32,
        url: String,
        error_code: i32,
        error_text: String,
    ) {
        self.observer.on_ice_candidate_error(address, port, url, error_code, error_text);
    }

    fn on_ice_candidates_removed(&self, candidates: Vec<ffi::CandidatePtr>) {
        self.observer.on_ice_candidates_removed(candidates.into_iter().map(|v| v.ptr).collect());
    }

    fn on_ice_connection_receiving_change(&self, receiving: bool) {
        self.observer.on_ice_connection_receiving_change(receiving);
    }

    fn on_ice_selected_candidate_pair_changed(&self, event: ffi::CandidatePairChangeEvent) {
        self.observer.on_ice_selected_candidate_pair_changed(event);
    }

    fn on_add_track(&self, receiver: SharedPtr<RtpReceiver>, streams: Vec<ffi::MediaStreamPtr>) {
        self.observer.on_add_track(receiver, streams.into_iter().map(|v| v.ptr).collect());
    }

    fn on_track(&self, transceiver: SharedPtr<RtpTransceiver>) {
        self.observer.on_track(transceiver);
    }

    fn on_remove_track(&self, receiver: SharedPtr<RtpReceiver>) {
        self.observer.on_remove_track(receiver);
    }

    fn on_interesting_usage(&self, usage_pattern: i32) {
        self.observer.on_interesting_usage(usage_pattern);
    }
}
