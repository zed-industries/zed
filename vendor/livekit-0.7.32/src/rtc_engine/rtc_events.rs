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

use libwebrtc::{self as rtc, prelude::*};
use livekit_protocol as proto;
use tokio::sync::mpsc;

use super::peer_transport::PeerTransport;
use crate::{
    rtc_engine::{peer_transport::OnOfferCreated, rtc_session::RELIABLE_DC_LABEL},
    DataPacketKind,
};

pub type RtcEmitter = mpsc::UnboundedSender<RtcEvent>;
pub type RtcEvents = mpsc::UnboundedReceiver<RtcEvent>;

#[derive(Debug)]
pub enum RtcEvent {
    IceCandidate {
        ice_candidate: IceCandidate,
        target: proto::SignalTarget,
    },
    ConnectionChange {
        state: PeerConnectionState,
        target: proto::SignalTarget,
    },
    DataChannel {
        data_channel: DataChannel,
        target: proto::SignalTarget,
    },
    // TODO (theomonnom): Move Offer to PeerTransport
    Offer {
        offer: SessionDescription,
        target: proto::SignalTarget,
    },
    Track {
        streams: Vec<MediaStream>,
        track: MediaStreamTrack,
        transceiver: RtpTransceiver,
        target: proto::SignalTarget,
    },
    Data {
        data: Vec<u8>,
        binary: bool,
        kind: DataPacketKind,
    },
    DataChannelBufferedAmountChange {
        sent: u64,
        amount: u64,
        kind: DataPacketKind,
    },
}

/// Handlers used to forward events to a channel
/// Every callback here is called on the signaling thread

fn on_connection_state_change(
    target: proto::SignalTarget,
    emitter: RtcEmitter,
) -> rtc::peer_connection::OnConnectionChange {
    Box::new(move |state| {
        let _ = emitter.send(RtcEvent::ConnectionChange { state, target });
    })
}

fn on_ice_candidate(
    target: proto::SignalTarget,
    emitter: RtcEmitter,
) -> rtc::peer_connection::OnIceCandidate {
    Box::new(move |ice_candidate| {
        let _ = emitter.send(RtcEvent::IceCandidate { ice_candidate, target });
    })
}

fn on_offer(target: proto::SignalTarget, emitter: RtcEmitter) -> OnOfferCreated {
    Box::new(move |offer| {
        let _ = emitter.send(RtcEvent::Offer { offer, target });
    })
}

fn on_data_channel(
    target: proto::SignalTarget,
    emitter: RtcEmitter,
) -> rtc::peer_connection::OnDataChannel {
    Box::new(move |data_channel| {
        let kind = if data_channel.label() == RELIABLE_DC_LABEL {
            DataPacketKind::Reliable
        } else {
            DataPacketKind::Lossy
        };
        data_channel.on_message(Some(on_message(emitter.clone(), kind)));

        let _ = emitter.send(RtcEvent::DataChannel { data_channel, target });
    })
}

fn on_track(target: proto::SignalTarget, emitter: RtcEmitter) -> rtc::peer_connection::OnTrack {
    Box::new(move |event| {
        let _ = emitter.send(RtcEvent::Track {
            streams: event.streams,
            track: event.track,
            transceiver: event.transceiver,
            target,
        });
    })
}

fn on_ice_candidate_error(
    _target: proto::SignalTarget,
    _emitter: RtcEmitter,
) -> rtc::peer_connection::OnIceCandidateError {
    Box::new(move |ice_error| {
        log::debug!("{:?}", ice_error);
    })
}

pub fn forward_pc_events(transport: &mut PeerTransport, rtc_emitter: RtcEmitter) {
    let signal_target = transport.signal_target();
    transport
        .peer_connection()
        .on_ice_candidate(Some(on_ice_candidate(signal_target, rtc_emitter.clone())));

    transport
        .peer_connection()
        .on_data_channel(Some(on_data_channel(signal_target, rtc_emitter.clone())));

    transport.peer_connection().on_track(Some(on_track(signal_target, rtc_emitter.clone())));

    transport.peer_connection().on_connection_state_change(Some(on_connection_state_change(
        signal_target,
        rtc_emitter.clone(),
    )));

    transport
        .peer_connection()
        .on_ice_candidate_error(Some(on_ice_candidate_error(signal_target, rtc_emitter.clone())));

    transport.on_offer(Some(on_offer(signal_target, rtc_emitter)));
}

fn on_message(emitter: RtcEmitter, kind: DataPacketKind) -> rtc::data_channel::OnMessage {
    Box::new(move |buffer| {
        let _ = emitter.send(RtcEvent::Data {
            data: buffer.data.to_vec(),
            binary: buffer.binary,
            kind,
        });
    })
}

fn on_buffered_amount_change(
    emitter: RtcEmitter,
    dc: DataChannel,
    kind: DataPacketKind,
) -> rtc::data_channel::OnBufferedAmountChange {
    Box::new(move |sent| {
        let amount = dc.buffered_amount();
        let _ = emitter.send(RtcEvent::DataChannelBufferedAmountChange { sent, amount, kind });
    })
}

pub fn forward_dc_events(dc: &mut DataChannel, kind: DataPacketKind, rtc_emitter: RtcEmitter) {
    dc.on_message(Some(on_message(rtc_emitter.clone(), kind)));
    dc.on_buffered_amount_change(Some(on_buffered_amount_change(rtc_emitter, dc.clone(), kind)));
}
