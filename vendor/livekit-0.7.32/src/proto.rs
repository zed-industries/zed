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

use livekit_protocol::*;

use crate::{
    e2ee::EncryptionType, participant, room::ChatMessage as RoomChatMessage, track, DataPacketKind,
};

// Conversions
impl From<ConnectionQuality> for participant::ConnectionQuality {
    fn from(value: ConnectionQuality) -> Self {
        match value {
            ConnectionQuality::Excellent => Self::Excellent,
            ConnectionQuality::Good => Self::Good,
            ConnectionQuality::Poor => Self::Poor,
            ConnectionQuality::Lost => Self::Lost,
        }
    }
}

impl From<DisconnectReason> for participant::DisconnectReason {
    fn from(value: DisconnectReason) -> Self {
        match value {
            DisconnectReason::UnknownReason => Self::UnknownReason,
            DisconnectReason::ClientInitiated => Self::ClientInitiated,
            DisconnectReason::DuplicateIdentity => Self::DuplicateIdentity,
            DisconnectReason::ServerShutdown => Self::ServerShutdown,
            DisconnectReason::ParticipantRemoved => Self::ParticipantRemoved,
            DisconnectReason::RoomDeleted => Self::RoomDeleted,
            DisconnectReason::StateMismatch => Self::StateMismatch,
            DisconnectReason::JoinFailure => Self::JoinFailure,
            DisconnectReason::Migration => Self::Migration,
            DisconnectReason::SignalClose => Self::SignalClose,
            DisconnectReason::RoomClosed => Self::RoomClosed,
            DisconnectReason::UserUnavailable => Self::UserUnavailable,
            DisconnectReason::UserRejected => Self::UserRejected,
            DisconnectReason::SipTrunkFailure => Self::SipTrunkFailure,
            DisconnectReason::ConnectionTimeout => Self::ConnectionTimeout,
            DisconnectReason::MediaFailure => Self::MediaFailure,
        }
    }
}

impl TryFrom<TrackType> for track::TrackKind {
    type Error = &'static str;

    fn try_from(r#type: TrackType) -> Result<Self, Self::Error> {
        match r#type {
            TrackType::Audio => Ok(Self::Audio),
            TrackType::Video => Ok(Self::Video),
            TrackType::Data => Err("data tracks are not implemented yet"),
        }
    }
}

impl From<track::TrackKind> for TrackType {
    fn from(kind: track::TrackKind) -> Self {
        match kind {
            track::TrackKind::Audio => Self::Audio,
            track::TrackKind::Video => Self::Video,
        }
    }
}

impl From<TrackSource> for track::TrackSource {
    fn from(source: TrackSource) -> Self {
        match source {
            TrackSource::Camera => Self::Camera,
            TrackSource::Microphone => Self::Microphone,
            TrackSource::ScreenShare => Self::Screenshare,
            TrackSource::ScreenShareAudio => Self::ScreenshareAudio,
            TrackSource::Unknown => Self::Unknown,
        }
    }
}

impl From<track::TrackSource> for TrackSource {
    fn from(source: track::TrackSource) -> Self {
        match source {
            track::TrackSource::Camera => Self::Camera,
            track::TrackSource::Microphone => Self::Microphone,
            track::TrackSource::Screenshare => Self::ScreenShare,
            track::TrackSource::ScreenshareAudio => Self::ScreenShareAudio,
            track::TrackSource::Unknown => Self::Unknown,
        }
    }
}

impl From<DataPacketKind> for data_packet::Kind {
    fn from(kind: DataPacketKind) -> Self {
        match kind {
            DataPacketKind::Lossy => Self::Lossy,
            DataPacketKind::Reliable => Self::Reliable,
        }
    }
}

impl From<data_packet::Kind> for DataPacketKind {
    fn from(kind: data_packet::Kind) -> Self {
        match kind {
            data_packet::Kind::Lossy => Self::Lossy,
            data_packet::Kind::Reliable => Self::Reliable,
        }
    }
}

impl From<encryption::Type> for EncryptionType {
    fn from(value: livekit_protocol::encryption::Type) -> Self {
        match value {
            livekit_protocol::encryption::Type::None => Self::None,
            livekit_protocol::encryption::Type::Gcm => Self::Gcm,
            livekit_protocol::encryption::Type::Custom => Self::Custom,
        }
    }
}

impl From<EncryptionType> for encryption::Type {
    fn from(value: EncryptionType) -> Self {
        match value {
            EncryptionType::None => Self::None,
            EncryptionType::Gcm => Self::Gcm,
            EncryptionType::Custom => Self::Custom,
        }
    }
}

impl From<EncryptionType> for i32 {
    fn from(value: EncryptionType) -> Self {
        match value {
            EncryptionType::None => 0,
            EncryptionType::Gcm => 1,
            EncryptionType::Custom => 2,
        }
    }
}

impl From<participant_info::Kind> for participant::ParticipantKind {
    fn from(value: participant_info::Kind) -> Self {
        match value {
            participant_info::Kind::Standard => participant::ParticipantKind::Standard,
            participant_info::Kind::Ingress => participant::ParticipantKind::Ingress,
            participant_info::Kind::Egress => participant::ParticipantKind::Egress,
            participant_info::Kind::Sip => participant::ParticipantKind::Sip,
            participant_info::Kind::Agent => participant::ParticipantKind::Agent,
            participant_info::Kind::Connector => participant::ParticipantKind::Connector,
            participant_info::Kind::Bridge => participant::ParticipantKind::Bridge,
        }
    }
}

impl From<participant_info::KindDetail> for participant::ParticipantKindDetail {
    fn from(value: participant_info::KindDetail) -> Self {
        match value {
            participant_info::KindDetail::CloudAgent => {
                participant::ParticipantKindDetail::CloudAgent
            }
            participant_info::KindDetail::Forwarded => {
                participant::ParticipantKindDetail::Forwarded
            }
            participant_info::KindDetail::ConnectorWhatsapp => {
                participant::ParticipantKindDetail::ConnectorWhatsapp
            }
            participant_info::KindDetail::ConnectorTwilio => {
                participant::ParticipantKindDetail::ConnectorTwilio
            }
            participant_info::KindDetail::BridgeRtsp => {
                participant::ParticipantKindDetail::BridgeRtsp
            }
        }
    }
}

impl From<ChatMessage> for RoomChatMessage {
    fn from(proto_msg: ChatMessage) -> Self {
        RoomChatMessage {
            id: proto_msg.id,
            message: proto_msg.message,
            timestamp: proto_msg.timestamp,
            edit_timestamp: proto_msg.edit_timestamp,
            deleted: proto_msg.deleted.into(),
            generated: proto_msg.generated.into(),
        }
    }
}

impl From<RoomChatMessage> for ChatMessage {
    fn from(msg: RoomChatMessage) -> Self {
        ChatMessage {
            id: msg.id,
            message: msg.message,
            timestamp: msg.timestamp,
            edit_timestamp: msg.edit_timestamp,
            deleted: msg.deleted.unwrap_or(false),
            generated: msg.generated.unwrap_or(false),
        }
    }
}

impl From<participant::ParticipantTrackPermission> for TrackPermission {
    fn from(perm: participant::ParticipantTrackPermission) -> Self {
        TrackPermission {
            participant_identity: perm.participant_identity.to_string(),
            participant_sid: String::new(),
            all_tracks: perm.allow_all,
            track_sids: perm.allowed_track_sids.iter().map(|sid| sid.to_string()).collect(),
        }
    }
}

impl From<TrackPermission> for participant::ParticipantTrackPermission {
    fn from(perm: TrackPermission) -> Self {
        participant::ParticipantTrackPermission {
            participant_identity: perm.participant_identity.into(),
            allow_all: perm.all_tracks,
            allowed_track_sids: perm
                .track_sids
                .into_iter()
                .map(|sid| sid.try_into().unwrap())
                .collect(),
        }
    }
}
