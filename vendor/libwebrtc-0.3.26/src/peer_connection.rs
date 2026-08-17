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

use std::fmt::Debug;

use crate::{
    data_channel::{DataChannel, DataChannelInit},
    ice_candidate::IceCandidate,
    imp::peer_connection as imp_pc,
    media_stream::MediaStream,
    media_stream_track::MediaStreamTrack,
    peer_connection_factory::RtcConfiguration,
    rtp_receiver::RtpReceiver,
    rtp_sender::RtpSender,
    rtp_transceiver::{RtpTransceiver, RtpTransceiverInit},
    session_description::SessionDescription,
    stats::RtcStats,
    MediaType, RtcError,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PeerConnectionState {
    New,
    Connecting,
    Connected,
    Disconnected,
    Failed,
    Closed,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IceConnectionState {
    New,
    Checking,
    Connected,
    Completed,
    Failed,
    Disconnected,
    Closed,
    Max,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IceGatheringState {
    New,
    Gathering,
    Complete,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SignalingState {
    Stable,
    HaveLocalOffer,
    HaveLocalPrAnswer,
    HaveRemoteOffer,
    HaveRemotePrAnswer,
    Closed,
}

#[derive(Debug, Clone, Default)]
pub struct OfferOptions {
    pub ice_restart: bool,
    pub offer_to_receive_audio: bool,
    pub offer_to_receive_video: bool,
}

#[derive(Debug, Clone, Default)]
pub struct AnswerOptions {}

#[derive(Debug, Clone)]
pub struct IceCandidateError {
    pub address: String,
    pub port: i32,
    pub url: String,
    pub error_code: i32,
    pub error_text: String,
}

#[derive(Debug, Clone)]
pub struct TrackEvent {
    pub receiver: RtpReceiver,
    pub streams: Vec<MediaStream>,
    pub track: MediaStreamTrack,
    pub transceiver: RtpTransceiver,
}

pub type OnConnectionChange = Box<dyn FnMut(PeerConnectionState) + Send + Sync>;
pub type OnDataChannel = Box<dyn FnMut(DataChannel) + Send + Sync>;
pub type OnIceCandidate = Box<dyn FnMut(IceCandidate) + Send + Sync>;
pub type OnIceCandidateError = Box<dyn FnMut(IceCandidateError) + Send + Sync>;
pub type OnIceConnectionChange = Box<dyn FnMut(IceConnectionState) + Send + Sync>;
pub type OnIceGatheringChange = Box<dyn FnMut(IceGatheringState) + Send + Sync>;
pub type OnNegotiationNeeded = Box<dyn FnMut(u32) + Send + Sync>;
pub type OnSignalingChange = Box<dyn FnMut(SignalingState) + Send + Sync>;
pub type OnTrack = Box<dyn FnMut(TrackEvent) + Send + Sync>;

#[derive(Clone)]
pub struct PeerConnection {
    pub(crate) handle: imp_pc::PeerConnection,
}

impl PeerConnection {
    pub fn set_configuration(&self, config: RtcConfiguration) -> Result<(), RtcError> {
        self.handle.set_configuration(config)
    }

    pub async fn create_offer(
        &self,
        options: OfferOptions,
    ) -> Result<SessionDescription, RtcError> {
        self.handle.create_offer(options).await
    }

    pub async fn create_answer(
        &self,
        options: AnswerOptions,
    ) -> Result<SessionDescription, RtcError> {
        self.handle.create_answer(options).await
    }

    pub async fn set_local_description(&self, desc: SessionDescription) -> Result<(), RtcError> {
        self.handle.set_local_description(desc).await
    }

    pub async fn set_remote_description(&self, desc: SessionDescription) -> Result<(), RtcError> {
        self.handle.set_remote_description(desc).await
    }

    pub async fn add_ice_candidate(&self, candidate: IceCandidate) -> Result<(), RtcError> {
        self.handle.add_ice_candidate(candidate).await
    }

    pub fn create_data_channel(
        &self,
        label: &str,
        init: DataChannelInit,
    ) -> Result<DataChannel, RtcError> {
        self.handle.create_data_channel(label, init)
    }

    pub fn add_track<T: AsRef<str>>(
        &self,
        track: MediaStreamTrack,
        streams_ids: &[T],
    ) -> Result<RtpSender, RtcError> {
        self.handle.add_track(track, streams_ids)
    }

    pub fn remove_track(&self, sender: RtpSender) -> Result<(), RtcError> {
        self.handle.remove_track(sender)
    }

    pub async fn get_stats(&self) -> Result<Vec<RtcStats>, RtcError> {
        self.handle.get_stats().await
    }

    pub fn add_transceiver(
        &self,
        track: MediaStreamTrack,
        init: RtpTransceiverInit,
    ) -> Result<RtpTransceiver, RtcError> {
        self.handle.add_transceiver(track, init)
    }

    pub fn add_transceiver_for_media(
        &self,
        media_type: MediaType,
        init: RtpTransceiverInit,
    ) -> Result<RtpTransceiver, RtcError> {
        self.handle.add_transceiver_for_media(media_type, init)
    }

    pub fn close(&self) {
        self.handle.close()
    }

    pub fn restart_ice(&self) {
        self.handle.restart_ice()
    }

    pub fn connection_state(&self) -> PeerConnectionState {
        self.handle.connection_state()
    }

    pub fn ice_connection_state(&self) -> IceConnectionState {
        self.handle.ice_connection_state()
    }

    pub fn ice_gathering_state(&self) -> IceGatheringState {
        self.handle.ice_gathering_state()
    }

    pub fn signaling_state(&self) -> SignalingState {
        self.handle.signaling_state()
    }

    pub fn current_local_description(&self) -> Option<SessionDescription> {
        self.handle.current_local_description()
    }

    pub fn current_remote_description(&self) -> Option<SessionDescription> {
        self.handle.current_remote_description()
    }

    pub fn senders(&self) -> Vec<RtpSender> {
        self.handle.senders()
    }

    pub fn receivers(&self) -> Vec<RtpReceiver> {
        self.handle.receivers()
    }

    pub fn transceivers(&self) -> Vec<RtpTransceiver> {
        self.handle.transceivers()
    }

    pub fn on_connection_state_change(&self, f: Option<OnConnectionChange>) {
        self.handle.on_connection_state_change(f)
    }

    pub fn on_data_channel(&self, f: Option<OnDataChannel>) {
        self.handle.on_data_channel(f)
    }

    pub fn on_ice_candidate(&self, f: Option<OnIceCandidate>) {
        self.handle.on_ice_candidate(f)
    }

    pub fn on_ice_candidate_error(&self, f: Option<OnIceCandidateError>) {
        self.handle.on_ice_candidate_error(f)
    }

    pub fn on_ice_connection_state_change(&self, f: Option<OnIceConnectionChange>) {
        self.handle.on_ice_connection_state_change(f)
    }

    pub fn on_ice_gathering_state_change(&self, f: Option<OnIceGatheringChange>) {
        self.handle.on_ice_gathering_state_change(f)
    }

    pub fn on_negotiation_needed(&self, f: Option<OnNegotiationNeeded>) {
        self.handle.on_negotiation_needed(f)
    }

    pub fn on_signaling_state_change(&self, f: Option<OnSignalingChange>) {
        self.handle.on_signaling_state_change(f)
    }

    pub fn on_track(&self, f: Option<OnTrack>) {
        self.handle.on_track(f)
    }
}

impl Debug for PeerConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PeerConnection")
            .field("state", &self.connection_state())
            .field("ice_state", &self.ice_connection_state())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use log::trace;
    use tokio::sync::mpsc;

    use crate::{peer_connection::*, peer_connection_factory::*};

    #[tokio::test]
    async fn create_pc() {
        let _ = env_logger::builder().is_test(true).try_init();

        let factory = PeerConnectionFactory::default();
        let config = RtcConfiguration {
            ice_servers: vec![IceServer {
                urls: vec!["stun:stun1.l.google.com:19302".to_string()],
                username: "".into(),
                password: "".into(),
            }],
            continual_gathering_policy: ContinualGatheringPolicy::GatherOnce,
            ice_transport_type: IceTransportsType::All,
        };

        let bob = factory.create_peer_connection(config.clone()).unwrap();
        let alice = factory.create_peer_connection(config.clone()).unwrap();

        let (bob_ice_tx, mut bob_ice_rx) = mpsc::unbounded_channel::<IceCandidate>();
        let (alice_ice_tx, mut alice_ice_rx) = mpsc::unbounded_channel::<IceCandidate>();
        let (alice_dc_tx, mut alice_dc_rx) = mpsc::unbounded_channel::<DataChannel>();

        bob.on_ice_candidate(Some(Box::new(move |candidate| {
            bob_ice_tx.send(candidate).unwrap();
        })));

        alice.on_ice_candidate(Some(Box::new(move |candidate| {
            alice_ice_tx.send(candidate).unwrap();
        })));

        alice.on_data_channel(Some(Box::new(move |dc| {
            alice_dc_tx.send(dc).unwrap();
        })));

        let bob_dc = bob.create_data_channel("test_dc", DataChannelInit::default()).unwrap();

        let offer = bob.create_offer(OfferOptions::default()).await.unwrap();
        trace!("Bob offer: {:?}", offer);
        bob.set_local_description(offer.clone()).await.unwrap();
        alice.set_remote_description(offer).await.unwrap();

        let answer = alice.create_answer(AnswerOptions::default()).await.unwrap();
        trace!("Alice answer: {:?}", answer);
        alice.set_local_description(answer.clone()).await.unwrap();
        bob.set_remote_description(answer).await.unwrap();

        let bob_ice = bob_ice_rx.recv().await.unwrap();
        let alice_ice = alice_ice_rx.recv().await.unwrap();

        bob.add_ice_candidate(alice_ice).await.unwrap();
        alice.add_ice_candidate(bob_ice).await.unwrap();

        let (data_tx, mut data_rx) = mpsc::unbounded_channel::<String>();
        let alice_dc = alice_dc_rx.recv().await.unwrap();
        alice_dc.on_message(Some(Box::new(move |buffer| {
            data_tx.send(String::from_utf8_lossy(buffer.data).to_string()).unwrap();
        })));

        bob_dc.send(b"This is a test", true).unwrap();
        assert_eq!(data_rx.recv().await.unwrap(), "This is a test");

        alice.close();
        bob.close();
    }
}
