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

use std::collections::HashMap;

use serde::Deserialize;

use crate::data_channel::DataChannelState;

/// Values from https://www.w3.org/TR/webrtc-stats/ (NOTE: Some of the structs are not in the SPEC
/// but inside libwebrtc)
/// serde will handle the magic of correctly deserializing the json into our structs.
/// The enums values are inside encapsulated inside option because we're not sure about their
/// default values (So we default to None instead of an arbitrary value)

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "kebab-case")]
pub enum RtcStats {
    Codec(CodecStats),
    InboundRtp(InboundRtpStats),
    OutboundRtp(OutboundRtpStats),
    RemoteInboundRtp(RemoteInboundRtpStats),
    RemoteOutboundRtp(RemoteOutboundRtpStats),
    MediaSource(MediaSourceStats),
    MediaPlayout(MediaPlayoutStats),
    PeerConnection(PeerConnectionStats),
    DataChannel(DataChannelStats),
    Transport(TransportStats),
    CandidatePair(CandidatePairStats),
    LocalCandidate(LocalCandidateStats),
    RemoteCandidate(RemoteCandidateStats),
    Certificate(CertificateStats),
    Stream(StreamStats),
    Track, // Deprecated
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QualityLimitationReason {
    #[default]
    None,
    Cpu,
    Bandwidth,
    Other,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IceRole {
    #[default]
    Unknown,
    Controlling,
    Controlled,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DtlsTransportState {
    New,
    Connecting,
    Connected,
    Closed,
    Failed,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IceTransportState {
    New,
    Checking,
    Connected,
    Completed,
    Disconnected,
    Failed,
    Closed,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DtlsRole {
    Client,
    Server,
    #[default]
    Unknown,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IceCandidatePairState {
    Frozen,
    Waiting,
    InProgress, // in-progress
    Failed,
    Succeeded,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IceCandidateType {
    Host,
    Srflx,
    Prflx,
    Relay,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IceServerTransportProtocol {
    Udp,
    Tcp,
    Tls,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IceTcpCandidateType {
    Active,
    Passive,
    So,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct CodecStats {
    #[serde(flatten)]
    pub rtc: dictionaries::RtcStats,

    #[serde(flatten)]
    pub codec: dictionaries::CodecStats,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct InboundRtpStats {
    #[serde(flatten)]
    pub rtc: dictionaries::RtcStats,

    #[serde(flatten)]
    pub stream: dictionaries::RtpStreamStats,

    #[serde(flatten)]
    pub received: dictionaries::ReceivedRtpStreamStats,

    #[serde(flatten)]
    pub inbound: dictionaries::InboundRtpStreamStats,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct OutboundRtpStats {
    #[serde(flatten)]
    pub rtc: dictionaries::RtcStats,

    #[serde(flatten)]
    pub stream: dictionaries::RtpStreamStats,

    #[serde(flatten)]
    pub sent: dictionaries::SentRtpStreamStats,

    #[serde(flatten)]
    pub outbound: dictionaries::OutboundRtpStreamStats,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct RemoteInboundRtpStats {
    #[serde(flatten)]
    pub rtc: dictionaries::RtcStats,

    #[serde(flatten)]
    pub stream: dictionaries::RtpStreamStats,

    #[serde(flatten)]
    pub received: dictionaries::ReceivedRtpStreamStats,

    #[serde(flatten)]
    pub remote_inbound: dictionaries::RemoteInboundRtpStreamStats,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct RemoteOutboundRtpStats {
    #[serde(flatten)]
    pub rtc: dictionaries::RtcStats,

    #[serde(flatten)]
    pub stream: dictionaries::RtpStreamStats,

    #[serde(flatten)]
    pub sent: dictionaries::SentRtpStreamStats,

    #[serde(flatten)]
    pub remote_outbound: dictionaries::RemoteOutboundRtpStreamStats,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct MediaSourceStats {
    #[serde(flatten)]
    pub rtc: dictionaries::RtcStats,

    #[serde(flatten)]
    pub source: dictionaries::MediaSourceStats,

    #[serde(flatten)]
    pub audio: dictionaries::AudioSourceStats,

    #[serde(flatten)]
    pub video: dictionaries::VideoSourceStats,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct MediaPlayoutStats {
    #[serde(flatten)]
    pub rtc: dictionaries::RtcStats,

    #[serde(flatten)]
    pub audio_playout: dictionaries::AudioPlayoutStats,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct PeerConnectionStats {
    #[serde(flatten)]
    pub rtc: dictionaries::RtcStats,

    #[serde(flatten)]
    pub pc: dictionaries::PeerConnectionStats,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct DataChannelStats {
    #[serde(flatten)]
    pub rtc: dictionaries::RtcStats,

    #[serde(flatten)]
    pub dc: dictionaries::DataChannelStats,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct TransportStats {
    #[serde(flatten)]
    pub rtc: dictionaries::RtcStats,

    #[serde(flatten)]
    pub transport: dictionaries::TransportStats,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct CandidatePairStats {
    #[serde(flatten)]
    pub rtc: dictionaries::RtcStats,

    #[serde(flatten)]
    pub candidate_pair: dictionaries::CandidatePairStats,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct LocalCandidateStats {
    #[serde(flatten)]
    pub rtc: dictionaries::RtcStats,

    #[serde(flatten)]
    pub local_candidate: dictionaries::IceCandidateStats,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct RemoteCandidateStats {
    #[serde(flatten)]
    pub rtc: dictionaries::RtcStats,

    #[serde(flatten)]
    pub remote_candidate: dictionaries::IceCandidateStats,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct CertificateStats {
    #[serde(flatten)]
    pub rtc: dictionaries::RtcStats,

    #[serde(flatten)]
    pub certificate: dictionaries::CertificateStats,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct StreamStats {
    #[serde(flatten)]
    pub rtc: dictionaries::RtcStats,

    #[serde(flatten)]
    pub stream: dictionaries::StreamStats,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct TrackStats {}

pub mod dictionaries {
    use super::*;

    #[derive(Debug, Default, Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[serde(default)]
    pub struct RtcStats {
        pub id: String,
        pub timestamp: i64,
    }

    #[derive(Debug, Default, Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[serde(default)]
    pub struct CodecStats {
        pub payload_type: u32,
        pub transport_id: String,
        pub mime_type: String,
        pub clock_rate: u32,
        pub channels: u32,
        pub sdp_fmtp_line: String,
    }

    #[derive(Debug, Default, Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[serde(default)]
    pub struct RtpStreamStats {
        pub ssrc: u32,
        pub kind: String,
        pub transport_id: String,
        pub codec_id: String,
    }

    #[derive(Debug, Default, Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[serde(default)]
    pub struct ReceivedRtpStreamStats {
        pub packets_received: u64,
        pub packets_lost: i64,
        pub jitter: f64,
    }

    #[derive(Debug, Default, Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[serde(default)]
    pub struct InboundRtpStreamStats {
        pub track_identifier: String,
        pub mid: String,
        pub remote_id: String,
        pub frames_decoded: u32,
        pub key_frames_decoded: u32,
        pub frames_rendered: u32,
        pub frames_dropped: u32,
        pub frame_width: u32,
        pub frame_height: u32,
        pub frames_per_second: f64,
        pub qp_sum: u64,
        pub total_decode_time: f64,
        pub total_inter_frame_delay: f64,
        pub total_squared_inter_frame_delay: f64,
        pub pause_count: u32,
        pub total_pause_duration: f64,
        pub freeze_count: u32,
        pub total_freeze_duration: f64,
        pub last_packet_received_timestamp: f64,
        pub header_bytes_received: u64,
        pub packets_discarded: u64,
        pub fec_bytes_received: u64,
        pub fec_packets_received: u64,
        pub fec_packets_discarded: u64,
        pub bytes_received: u64,
        pub nack_count: u32,
        pub fir_count: u32,
        pub pli_count: u32,
        pub total_processing_delay: f64,
        pub estimated_playout_timestamp: f64,
        pub jitter_buffer_delay: f64,
        pub jitter_buffer_target_delay: f64,
        pub jitter_buffer_emitted_count: u64,
        pub jitter_buffer_minimum_delay: f64,
        pub total_samples_received: u64,
        pub concealed_samples: u64,
        pub silent_concealed_samples: u64,
        pub concealment_events: u64,
        pub inserted_samples_for_deceleration: u64,
        pub removed_samples_for_acceleration: u64,
        pub audio_level: f64,
        pub total_audio_energy: f64,
        pub total_samples_duration: f64,
        pub frames_received: u64,
        pub decoder_implementation: String,
        pub playout_id: String,
        pub power_efficient_decoder: bool,
        pub frames_assembled_from_multiple_packets: u64,
        pub total_assembly_time: f64,
        pub retransmitted_packets_received: u64,
        pub retransmitted_bytes_received: u64,
        pub rtx_ssrc: u32,
        pub fec_ssrc: u32,
    }

    #[derive(Debug, Default, Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[serde(default)]
    pub struct SentRtpStreamStats {
        pub packets_sent: u64,
        pub bytes_sent: u64,
    }

    #[derive(Debug, Default, Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[serde(default)]
    pub struct OutboundRtpStreamStats {
        pub mid: String,
        pub media_source_id: String,
        pub remote_id: String,
        pub rid: String,
        pub header_bytes_sent: u64,
        pub retransmitted_packets_sent: u64,
        pub retransmitted_bytes_sent: u64,
        pub rtx_ssrc: u32,
        pub target_bitrate: f64,
        pub total_encoded_bytes_target: u64,
        pub frame_width: u32,
        pub frame_height: u32,
        pub frames_per_second: f64,
        pub frames_sent: u32,
        pub huge_frames_sent: u32,
        pub frames_encoded: u32,
        pub key_frames_encoded: u32,
        pub qp_sum: u64,
        pub total_encode_time: f64,
        pub total_packet_send_delay: f64,
        pub quality_limitation_reason: QualityLimitationReason,
        pub quality_limitation_durations: HashMap<String, f64>,
        pub quality_limitation_resolution_changes: u32,
        pub nack_count: u32,
        pub fir_count: u32,
        pub pli_count: u32,
        pub encoder_implementation: String,
        pub power_efficient_encoder: bool,
        pub active: bool,
        pub scalibility_mode: String,
    }

    #[derive(Debug, Default, Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[serde(default)]
    pub struct RemoteInboundRtpStreamStats {
        pub local_id: String,
        pub round_trip_time: f64,
        pub total_round_trip_time: f64,
        pub fraction_lost: f64,
        pub round_trip_time_measurements: u64,
    }

    #[derive(Debug, Default, Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[serde(default)]
    pub struct RemoteOutboundRtpStreamStats {
        pub local_id: String,
        pub remote_timestamp: f64,
        pub reports_sent: u64,
        pub round_trip_time: f64,
        pub total_round_trip_time: f64,
        pub round_trip_time_measurements: u64,
    }

    #[derive(Debug, Default, Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[serde(default)]
    pub struct MediaSourceStats {
        pub track_identifier: String,
        pub kind: String,
    }

    #[derive(Debug, Default, Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[serde(default)]
    pub struct AudioSourceStats {
        pub audio_level: f64,
        pub total_audio_energy: f64,
        pub total_samples_duration: f64,
        pub echo_return_loss: f64,
        pub echo_return_loss_enhancement: f64,
        pub dropped_samples_duration: f64,
        pub dropped_samples_events: u32,
        pub total_capture_delay: f64,
        pub total_samples_captured: u64,
    }

    #[derive(Debug, Default, Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[serde(default)]
    pub struct VideoSourceStats {
        pub width: u32,
        pub height: u32,
        pub frames: u32,
        pub frames_per_second: f64,
    }

    #[derive(Debug, Default, Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[serde(default)]
    pub struct AudioPlayoutStats {
        pub kind: String,
        pub synthesized_samples_duration: f64,
        pub synthesized_samples_events: u32,
        pub total_samples_duration: f64,
        pub total_playout_delay: f64,
        pub total_samples_count: u64,
    }

    #[derive(Debug, Default, Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[serde(default)]
    pub struct PeerConnectionStats {
        pub data_channels_opened: u32,
        pub data_channels_closed: u32,
    }

    #[derive(Debug, Default, Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[serde(default)]
    pub struct DataChannelStats {
        pub label: String,
        pub protocol: String,
        pub data_channel_identifier: i32,
        pub state: Option<DataChannelState>,
        pub messages_sent: u32,
        pub bytes_sent: u64,
        pub messages_received: u32,
        pub bytes_received: u64,
    }

    #[derive(Debug, Default, Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[serde(default)]
    pub struct TransportStats {
        pub packets_sent: u64,
        pub packets_received: u64,
        pub bytes_sent: u64,
        pub bytes_received: u64,
        pub ice_role: IceRole,
        pub ice_local_username_fragment: String,
        pub dtls_state: Option<DtlsTransportState>,
        pub ice_state: Option<IceTransportState>,
        pub selected_candidate_pair_id: String,
        pub local_certificate_id: String,
        pub remote_certificate_id: String,
        pub tls_version: String,
        pub dtls_cipher: String,
        pub dtls_role: DtlsRole,
        pub srtp_cipher: String,
        pub selected_candidate_pair_changes: u32,
    }

    #[derive(Debug, Default, Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[serde(default)]
    pub struct CandidatePairStats {
        pub transport_id: String,
        pub local_candidate_id: String,
        pub remote_candidate_id: String,
        pub state: Option<IceCandidatePairState>,
        pub nominated: bool,
        pub packets_sent: u64,
        pub packets_received: u64,
        pub bytes_sent: u64,
        pub bytes_received: u64,
        pub last_packet_sent_timestamp: f64,
        pub last_packet_received_timestamp: f64,
        pub total_round_trip_time: f64,
        pub current_round_trip_time: f64,
        pub available_outgoing_bitrate: f64,
        pub available_incoming_bitrate: f64,
        pub requests_received: u64,
        pub requests_sent: u64,
        pub responses_received: u64,
        pub responses_sent: u64,
        pub consent_requests_sent: u64,
        pub packets_discarded_on_send: u32,
        pub bytes_discarded_on_send: u64,
    }

    #[derive(Debug, Default, Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[serde(default)]
    pub struct IceCandidateStats {
        pub transport_id: String,
        pub address: String,
        pub port: i32,
        pub protocol: String,
        pub candidate_type: Option<IceCandidateType>,
        pub priority: i32,
        pub url: String,
        pub relay_protocol: Option<IceServerTransportProtocol>,
        pub foundation: String,
        pub related_address: String,
        pub related_port: i32,
        pub username_fragment: String,
        pub tcp_type: Option<IceTcpCandidateType>,
    }

    #[derive(Debug, Default, Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[serde(default)]
    pub struct CertificateStats {
        pub fingerprint: String,
        pub fingerprint_algorithm: String,
        pub base64_certificate: String,
        pub issuer_certificate_id: String,
    }

    #[derive(Debug, Default, Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[serde(default)]
    pub struct StreamStats {
        pub id: String,
        pub stream_identifier: String,
        // pub timestamp: i64,
    }
}
